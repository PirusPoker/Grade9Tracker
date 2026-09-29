//! Automatic weekly backups of progress, into the person's own Documents folder.
//!
//! Progress lives in the app's data folder, which an uninstall, a reinstall or a
//! bad file can take with it. So once a week the UI hands over the progress it
//! has open (the live copy, not whatever last reached disk) and a dated copy
//! lands in `Documents\Zelinx backups`, where it survives all of those and is
//! easy to find. The newest few per profile are kept; older ones are removed,
//! and only files this module wrote (`backup-<profile>-<stamp>.json`) are ever
//! touched in that folder.

use chrono::NaiveDateTime;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub const FOLDER: &str = "Zelinx backups";
/// Where backups went before the app was renamed; moved across on first use.
pub const OLD_FOLDER: &str = "Gradient backups";
/// How long a backup counts as recent enough that a routine check skips.
pub const EVERY_DAYS: i64 = 7;
/// Backups kept per profile; anything older is removed.
pub const KEEP: usize = 8;
const STAMP: &str = "%Y-%m-%d-%H%M%S";

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The folder backups go to.
    pub folder: String,
    /// When the newest backup for this profile was made, `YYYY-MM-DDTHH:MM:SS`.
    pub last: Option<String>,
    /// The file written by this call, if one was.
    pub made: Option<String>,
    /// How many backups this profile has now.
    pub count: usize,
}

/// A profile id made safe for a file name: letters, digits and dashes only.
fn slug(profile: &str) -> String {
    let s: String = profile.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
    if s.is_empty() { "profile".into() } else { s }
}

/// This profile's backups, newest first, with the time each was made.
fn existing(folder: &Path, profile: &str) -> Vec<(NaiveDateTime, PathBuf)> {
    let prefix = format!("backup-{}-", slug(profile));
    let mut found: Vec<(NaiveDateTime, PathBuf)> = fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let stamp = name.strip_prefix(&prefix)?.strip_suffix(".json")?;
            let when = NaiveDateTime::parse_from_str(stamp, STAMP).ok()?;
            Some((when, e.path()))
        })
        .collect();
    found.sort_by(|a, b| b.0.cmp(&a.0));
    found
}

fn info(folder: &Path, profile: &str, made: Option<String>) -> Info {
    let all = existing(folder, profile);
    Info {
        folder: folder.to_string_lossy().into_owned(),
        last: all.first().map(|(t, _)| t.format("%Y-%m-%dT%H:%M:%S").to_string()),
        made,
        count: all.len(),
    }
}

/// Where things stand, without writing anything.
pub fn status(folder: &Path, profile: &str) -> Info {
    info(folder, profile, None)
}

/// Back up `bundle` for `profile` into `folder` - unless a backup from the last
/// week already exists and this isn't `force`d (the "Back up now" button). Then
/// keep only the newest [`KEEP`].
pub fn run(folder: &Path, profile: &str, bundle: &Value, force: bool, now: NaiveDateTime) -> Result<Info, String> {
    if !force {
        if let Some((last, _)) = existing(folder, profile).first() {
            if now.signed_duration_since(*last) < chrono::Duration::days(EVERY_DAYS) {
                return Ok(info(folder, profile, None));
            }
        }
    }
    fs::create_dir_all(folder).map_err(|e| format!("Couldn't make the backup folder: {e}"))?;
    let target = folder.join(format!("backup-{}-{}.json", slug(profile), now.format(STAMP)));
    let text = serde_json::to_string_pretty(bundle).map_err(|e| e.to_string())?;
    // Write beside it and rename over, so a crash can't leave half a backup.
    let tmp = target.with_extension("json.tmp");
    fs::write(&tmp, text).map_err(|e| format!("Couldn't write the backup: {e}"))?;
    fs::rename(&tmp, &target).map_err(|e| format!("Couldn't save the backup: {e}"))?;
    for (_, old) in existing(folder, profile).into_iter().skip(KEEP) {
        let _ = fs::remove_file(old);
    }
    Ok(info(folder, profile, Some(target.to_string_lossy().into_owned())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn scratch() -> PathBuf {
        std::env::temp_dir().join(format!("g9-backup-test-{}-{}", std::process::id(), chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)))
    }
    fn at(d: u32, h: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 9, d).unwrap().and_hms_opt(h, 0, 0).unwrap()
    }

    #[test]
    fn a_week_apart_and_forced_ones_are_made_others_skipped() {
        let dir = scratch();
        let b = serde_json::json!({"state": {"days": {"2026-09-01": true}}});
        let first = run(&dir, "me", &b, false, at(1, 9)).unwrap();
        assert!(first.made.is_some() && first.count == 1);
        assert!(run(&dir, "me", &b, false, at(5, 9)).unwrap().made.is_none(), "4 days later: skip");
        assert!(run(&dir, "me", &b, true, at(5, 10)).unwrap().made.is_some(), "Back up now always writes");
        let later = run(&dir, "me", &b, false, at(12, 11)).unwrap();
        assert!(later.made.is_some(), "a week after the newest");
        assert_eq!(later.count, 3);
        assert_eq!(later.last.as_deref(), Some("2026-09-12T11:00:00"));
        let saved: Value = serde_json::from_str(&fs::read_to_string(later.made.unwrap()).unwrap()).unwrap();
        assert_eq!(saved, b);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn keeps_the_newest_eight_per_profile_and_leaves_other_files_alone() {
        let dir = scratch();
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("my notes.txt"), "not ours").unwrap();
        let b = serde_json::json!({});
        run(&dir, "tohir", &b, true, at(1, 8)).unwrap();
        for d in 1..=12 {
            run(&dir, "me", &b, true, at(d, 9)).unwrap();
        }
        let me = existing(&dir, "me");
        assert_eq!(me.len(), KEEP);
        assert_eq!(me.first().unwrap().0, at(12, 9));
        assert_eq!(me.last().unwrap().0, at(5, 9), "the oldest four went");
        assert_eq!(existing(&dir, "tohir").len(), 1, "another profile's backups are untouched");
        assert!(dir.join("my notes.txt").exists(), "files we didn't write are never removed");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn odd_profile_ids_make_safe_file_names() {
        assert_eq!(slug("me"), "me");
        assert_eq!(slug("../x y"), "---x-y");
        assert_eq!(slug(""), "profile");
    }
}
