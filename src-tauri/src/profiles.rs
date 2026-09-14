//! Profiles: several people sharing one PC, each with their own progress, plan,
//! known topics and cards.
//!
//! Layout inside the app data folder:
//!
//! ```text
//! profiles.json            which profiles exist and which is current
//! profiles/<id>/state.json   that person's progress
//! profiles/<id>/config.json  that person's plan
//! profiles/<id>/backups/     their dated backups
//! settings.json            global: API key, Outlook switch
//! ```
//!
//! The first run after this was added moves the old top-level state.json and
//! config.json into a profile called "Me", so nothing is lost.
//!
//! A PIN is a courtesy lock against a sibling opening the wrong profile, not
//! security: the files are readable on disk by anyone with the login. It is
//! stored salted and hashed all the same, so it is never sitting in plain text.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    /// Salted SHA-256 of the PIN, or empty for none.
    #[serde(default)]
    pub pin_hash: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub last_used: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    pub current: String,
    pub profiles: Vec<Profile>,
}

/// What the UI sees: never the hash, only whether a PIN exists.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileView {
    pub id: String,
    pub name: String,
    pub has_pin: bool,
    pub created_at: String,
    pub last_used: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryView {
    pub current: String,
    pub profiles: Vec<ProfileView>,
}

fn now() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn read_json(path: &Path) -> Option<Value> {
    fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok())
}

fn write_json(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(value).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

fn hash_pin(id: &str, pin: &str) -> String {
    let mut h = Sha256::new();
    h.update(b"grade9-tracker:");
    h.update(id.as_bytes());
    h.update(b":");
    h.update(pin.trim().as_bytes());
    format!("{:x}", h.finalize())
}

/// A file-safe id from a display name, unique within the registry.
fn new_id(reg: &Registry, name: &str) -> String {
    let base: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .chars()
        .take(24)
        .collect();
    let base = if base.is_empty() { "profile".to_string() } else { base };
    let mut id = base.clone();
    let mut n = 2;
    while reg.profiles.iter().any(|p| p.id == id) || id.starts_with('_') {
        id = format!("{base}-{n}");
        n += 1;
    }
    id
}

pub struct Store {
    pub root: PathBuf,
}

impl Store {
    pub fn new(root: PathBuf) -> Self {
        Store { root }
    }

    fn registry_path(&self) -> PathBuf {
        self.root.join("profiles.json")
    }

    pub fn dir_of(&self, id: &str) -> PathBuf {
        self.root.join("profiles").join(id)
    }

    /// The registry, creating it - and migrating any pre-profile files into a
    /// first profile called "Me" - on first use.
    pub fn registry(&self) -> Result<Registry, String> {
        if let Some(v) = read_json(&self.registry_path()) {
            if let Ok(reg) = serde_json::from_value::<Registry>(v) {
                if !reg.profiles.is_empty() {
                    return Ok(reg);
                }
            }
        }
        let mut reg = Registry::default();
        let id = "me".to_string();
        let dir = self.dir_of(&id);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for name in ["state.json", "config.json"] {
            let old = self.root.join(name);
            if old.exists() {
                fs::rename(&old, dir.join(name)).map_err(|e| e.to_string())?;
            }
        }
        let old_backups = self.root.join("backups");
        if old_backups.is_dir() && !dir.join("backups").exists() {
            let _ = fs::rename(&old_backups, dir.join("backups"));
        }
        reg.profiles.push(Profile { id: id.clone(), name: "Me".into(), pin_hash: String::new(), created_at: now(), last_used: now() });
        reg.current = id;
        self.save(&reg)?;
        Ok(reg)
    }

    fn save(&self, reg: &Registry) -> Result<(), String> {
        write_json(&self.registry_path(), &serde_json::to_value(reg).map_err(|e| e.to_string())?)
    }

    pub fn view(&self) -> Result<RegistryView, String> {
        let reg = self.registry()?;
        Ok(RegistryView {
            current: reg.current.clone(),
            profiles: reg.profiles.iter().map(|p| ProfileView {
                id: p.id.clone(), name: p.name.clone(), has_pin: !p.pin_hash.is_empty(),
                created_at: p.created_at.clone(), last_used: p.last_used.clone(),
            }).collect(),
        })
    }

    /// The directory of whoever is signed in.
    pub fn current_dir(&self) -> Result<PathBuf, String> {
        let reg = self.registry()?;
        let dir = self.dir_of(&reg.current);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        Ok(dir)
    }

    pub fn create(&self, name: &str, pin: Option<&str>) -> Result<String, String> {
        let name = name.trim();
        if name.is_empty() { return Err("Give the profile a name.".into()); }
        if name.len() > 40 { return Err("That name is too long.".into()); }
        let mut reg = self.registry()?;
        if reg.profiles.iter().any(|p| p.name.eq_ignore_ascii_case(name)) {
            return Err(format!("There is already a profile called {name}."));
        }
        let id = new_id(&reg, name);
        fs::create_dir_all(self.dir_of(&id)).map_err(|e| e.to_string())?;
        let pin_hash = match pin.map(str::trim).filter(|p| !p.is_empty()) {
            Some(p) => { check_pin_shape(p)?; hash_pin(&id, p) }
            None => String::new(),
        };
        reg.profiles.push(Profile { id: id.clone(), name: name.to_string(), pin_hash, created_at: now(), last_used: now() });
        reg.current = id.clone();
        self.save(&reg)?;
        Ok(id)
    }

    /// Switch to a profile, checking its PIN if it has one.
    pub fn switch(&self, id: &str, pin: Option<&str>) -> Result<(), String> {
        let mut reg = self.registry()?;
        let p = reg.profiles.iter_mut().find(|p| p.id == id).ok_or("No such profile.")?;
        if !p.pin_hash.is_empty() {
            let given = pin.map(str::trim).unwrap_or("");
            if given.is_empty() { return Err("PIN needed.".into()); }
            if hash_pin(id, given) != p.pin_hash { return Err("Wrong PIN.".into()); }
        }
        p.last_used = now();
        reg.current = id.to_string();
        self.save(&reg)
    }

    pub fn rename(&self, id: &str, name: &str) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() { return Err("Give the profile a name.".into()); }
        let mut reg = self.registry()?;
        if reg.profiles.iter().any(|p| p.id != id && p.name.eq_ignore_ascii_case(name)) {
            return Err(format!("There is already a profile called {name}."));
        }
        let p = reg.profiles.iter_mut().find(|p| p.id == id).ok_or("No such profile.")?;
        p.name = name.to_string();
        self.save(&reg)
    }

    /// Set, change or clear a PIN. The old one must be given if there is one.
    pub fn set_pin(&self, id: &str, old: Option<&str>, new: Option<&str>) -> Result<(), String> {
        let mut reg = self.registry()?;
        let p = reg.profiles.iter_mut().find(|p| p.id == id).ok_or("No such profile.")?;
        if !p.pin_hash.is_empty() && hash_pin(id, old.map(str::trim).unwrap_or("")) != p.pin_hash {
            return Err("Wrong PIN.".into());
        }
        p.pin_hash = match new.map(str::trim).filter(|n| !n.is_empty()) {
            Some(n) => { check_pin_shape(n)?; hash_pin(id, n) }
            None => String::new(),
        };
        self.save(&reg)
    }

    /// Deleting never destroys the files: the folder is moved aside with a
    /// timestamp, so a mistake is recoverable by hand.
    pub fn delete(&self, id: &str, pin: Option<&str>) -> Result<(), String> {
        let mut reg = self.registry()?;
        if reg.profiles.len() <= 1 { return Err("You cannot delete the only profile.".into()); }
        let p = reg.profiles.iter().find(|p| p.id == id).ok_or("No such profile.")?.clone();
        if !p.pin_hash.is_empty() && hash_pin(id, pin.map(str::trim).unwrap_or("")) != p.pin_hash {
            return Err("Wrong PIN.".into());
        }
        let dir = self.dir_of(id);
        if dir.exists() {
            let aside = self.root.join("profiles").join(format!("_deleted-{}-{}", id, chrono::Local::now().format("%Y%m%d-%H%M%S")));
            fs::rename(&dir, &aside).map_err(|e| e.to_string())?;
        }
        reg.profiles.retain(|p| p.id != id);
        if reg.current == id {
            reg.current = reg.profiles[0].id.clone();
        }
        self.save(&reg)
    }

    /// Everything a profile is, in one file that can be moved to another PC.
    pub fn export(&self, id: &str) -> Result<Value, String> {
        let reg = self.registry()?;
        let p = reg.profiles.iter().find(|p| p.id == id).ok_or("No such profile.")?;
        let dir = self.dir_of(id);
        Ok(serde_json::json!({
            "grade9Profile": 1,
            "name": p.name,
            "exportedAt": now(),
            "state": read_json(&dir.join("state.json")),
            "config": read_json(&dir.join("config.json")),
        }))
    }

    /// Bring in an exported profile as a new profile. Never overwrites.
    pub fn import(&self, bundle: &Value) -> Result<String, String> {
        if bundle.get("grade9Profile").and_then(Value::as_i64) != Some(1) {
            return Err("That is not a Grade 9 Tracker profile file.".into());
        }
        let name = bundle.get("name").and_then(Value::as_str).unwrap_or("Imported").trim().to_string();
        let reg = self.registry()?;
        let mut candidate = name.clone();
        let mut n = 2;
        while reg.profiles.iter().any(|p| p.name.eq_ignore_ascii_case(&candidate)) {
            candidate = format!("{name} {n}");
            n += 1;
        }
        let id = self.create(&candidate, None)?;
        let dir = self.dir_of(&id);
        if let Some(s) = bundle.get("state").filter(|v| !v.is_null()) {
            write_json(&dir.join("state.json"), s)?;
        }
        if let Some(c) = bundle.get("config").filter(|v| !v.is_null()) {
            write_json(&dir.join("config.json"), c)?;
        }
        Ok(id)
    }
}

fn check_pin_shape(pin: &str) -> Result<(), String> {
    if pin.len() < 4 || pin.len() > 8 || !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err("A PIN is 4 to 8 digits.".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> Store {
        let root = std::env::temp_dir().join(format!("g9-profiles-test-{}-{}", std::process::id(), chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)));
        fs::create_dir_all(&root).unwrap();
        Store::new(root)
    }

    #[test]
    fn first_run_migrates_old_files_into_a_profile_called_me() {
        let s = store();
        fs::write(s.root.join("state.json"), r#"{"topics":{"maths:1.1":{"done":true}}}"#).unwrap();
        fs::write(s.root.join("config.json"), r#"{"subjects":[],"blocks":[],"reviewGaps":[7]}"#).unwrap();
        let reg = s.registry().unwrap();
        assert_eq!(reg.current, "me");
        assert_eq!(reg.profiles[0].name, "Me");
        assert!(!s.root.join("state.json").exists(), "old state should have moved");
        assert!(s.dir_of("me").join("state.json").exists());
        assert!(s.dir_of("me").join("config.json").exists());
        assert_eq!(s.current_dir().unwrap(), s.dir_of("me"));
    }

    #[test]
    fn profiles_are_separate_and_pins_are_checked() {
        let s = store();
        s.registry().unwrap();
        let id = s.create("Alex", Some("1234")).unwrap();
        assert_eq!(s.registry().unwrap().current, id);
        assert!(s.view().unwrap().profiles.iter().find(|p| p.id == id).unwrap().has_pin);
        assert!(s.switch("me", None).is_ok(), "Me has no PIN");
        assert_eq!(s.switch(&id, None).unwrap_err(), "PIN needed.");
        assert_eq!(s.switch(&id, Some("0000")).unwrap_err(), "Wrong PIN.");
        assert!(s.switch(&id, Some("1234")).is_ok());
        assert_ne!(s.dir_of("me"), s.dir_of(&id));
        // The hash is never exposed and never plain.
        let raw = fs::read_to_string(s.registry_path()).unwrap();
        assert!(!raw.contains("1234"));
    }

    #[test]
    fn names_must_be_unique_and_ids_are_file_safe() {
        let s = store();
        s.registry().unwrap();
        let a = s.create("Sam O'Neil!", None).unwrap();
        assert_eq!(a, "sam-o-neil");
        assert!(s.create("sam o'neil!", None).is_err(), "case-insensitive clash");
        assert!(s.create("   ", None).is_err());
        assert_eq!(s.create("Me", Some("12")).unwrap_err(), "There is already a profile called Me.");
        assert_eq!(s.create("Kit", Some("12")).unwrap_err(), "A PIN is 4 to 8 digits.");
    }

    #[test]
    fn export_then_import_round_trips_as_a_new_profile() {
        let s = store();
        s.registry().unwrap();
        fs::write(s.dir_of("me").join("state.json"), r#"{"days":{"2026-09-14":true}}"#).unwrap();
        let bundle = s.export("me").unwrap();
        assert_eq!(bundle["name"], "Me");
        let id = s.import(&bundle).unwrap();
        assert_eq!(id, "me-2");
        let copied = read_json(&s.dir_of(&id).join("state.json")).unwrap();
        assert_eq!(copied["days"]["2026-09-14"], true);
        assert_eq!(s.view().unwrap().profiles.len(), 2);
        assert!(s.import(&serde_json::json!({"hello": 1})).is_err());
    }

    #[test]
    fn deleting_moves_the_folder_aside_and_never_the_last_profile() {
        let s = store();
        s.registry().unwrap();
        assert!(s.delete("me", None).is_err());
        let id = s.create("Kit", None).unwrap();
        fs::write(s.dir_of(&id).join("state.json"), "{}").unwrap();
        s.delete(&id, None).unwrap();
        assert!(!s.dir_of(&id).exists());
        let aside = fs::read_dir(s.root.join("profiles")).unwrap()
            .filter_map(|e| e.ok()).any(|e| e.file_name().to_string_lossy().starts_with("_deleted-kit"));
        assert!(aside, "deleted profile should be kept aside, not destroyed");
        assert_eq!(s.registry().unwrap().current, "me");
    }
}
