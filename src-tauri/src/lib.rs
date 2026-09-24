//! Grade 9 Tracker — Tauri backend.
//! Builds the study plan from an editable configuration and saves progress to
//! JSON files in the app's data folder. The app finds and orders the work; the
//! questions themselves live on Save My Exams, which each person opens with
//! their own account.

mod ai;
mod config;
mod course;
mod draft;
mod elevation;
mod lessons;
mod papers;
mod outlook;
mod plan;
mod profiles;
mod statements;
mod teams;
mod teams_server;
mod today;
mod tts;
mod update;
mod videos;

use config::PlanConfig;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}


/// The profile store, rooted at the app data folder.
fn store(app: &AppHandle) -> Result<profiles::Store, String> {
    Ok(profiles::Store::new(data_dir(app)?))
}

/// Where the signed-in person's progress, plan and backups live.
fn profile_dir(app: &AppHandle) -> Result<PathBuf, String> {
    store(app)?.current_dir()
}
fn read_json(path: &PathBuf) -> Option<Value> {
    fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok())
}

/// Write atomically: to a temp file first, then rename over the real one, so a
/// crash mid-write can never leave a half-written progress file.
fn write_json(path: &PathBuf, value: &Value) -> Result<(), String> {
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(value).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

/// The user's plan configuration, falling back to the Edexcel defaults the
/// first time the app runs (or if the file has been corrupted).
fn load_config(app: &AppHandle) -> PlanConfig {
    let mut cfg = data_dir(app)
        .ok()
        .and_then(|d| read_json(&d.join("config.json")))
        .and_then(|v| serde_json::from_value::<PlanConfig>(v).ok())
        .unwrap_or_default();
    cfg.sanitise();
    cfg
}

// ---------- Commands ----------

/// The plan. Pass a catch-up snapshot to re-pour unfinished work from a given
/// week; without one you get the plan as originally laid out.
#[tauri::command]
fn get_plan(app: AppHandle, catch_up: Option<plan::CatchUp>) -> plan::Plan {
    plan::build_with(&load_config(&app), catch_up.as_ref())
}

#[tauri::command]
fn get_config(app: AppHandle) -> PlanConfig {
    load_config(&app)
}

/// Save an edited configuration and hand back the plan it produces, so the UI
/// can redraw from one round trip.
#[tauri::command]
fn save_config(app: AppHandle, config: PlanConfig) -> Result<plan::Plan, String> {
    let mut cfg = config;
    cfg.sanitise();
    let value = serde_json::to_value(&cfg).map_err(|e| e.to_string())?;
    write_json(&profile_dir(&app)?.join("config.json"), &value)?;
    Ok(plan::build(&cfg))
}

/// Throw away the edits and go back to the built-in Edexcel plan.
#[tauri::command]
fn reset_config(app: AppHandle) -> Result<plan::Plan, String> {
    let cfg = PlanConfig::default();
    let value = serde_json::to_value(&cfg).map_err(|e| e.to_string())?;
    write_json(&profile_dir(&app)?.join("config.json"), &value)?;
    Ok(plan::build(&cfg))
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<draft::Settings, String> {
    Ok(read_json(&data_dir(&app)?.join("settings.json")).and_then(|v| serde_json::from_value(v).ok()).unwrap_or_default())
}

#[tauri::command]
fn set_settings(app: AppHandle, settings: draft::Settings) -> Result<(), String> {
    write_json(&data_dir(&app)?.join("settings.json"), &serde_json::to_value(settings).map_err(|e| e.to_string())?)
}

/// Draft a new subject from a public spec page or pasted syllabus text. Returns
/// the draft for review — nothing is saved until the plan itself is saved.
#[tauri::command]
async fn draft_subject(app: AppHandle, name: String, url: String, text: String) -> Result<Value, String> {
    let key = get_settings(app)?.api_key;
    draft::draft(&key, &name, &url, &text).await
}

/// The day as Outlook sees it: events, deadlines pulled out of mail, requests
/// with no date. Cached for a while because asking Outlook takes a second or
/// two; `refresh` forces a fresh read. If the read fails and there is a cached
/// copy, that comes back with `error` set so the UI can show stale data and
/// say why.
#[tauri::command]
async fn day_context(app: AppHandle, refresh: bool) -> Result<today::Context, String> {
    const FRESH_MINUTES: i64 = 20;
    let settings = get_settings(app.clone())?;
    if !settings.outlook {
        return Err("Outlook is switched off in the Guide tab.".into());
    }
    let path = data_dir(&app)?.join("outlook.json");
    let cached: Option<today::Context> = read_json(&path).and_then(|v| serde_json::from_value(v).ok());
    if !refresh {
        if let Some(c) = &cached {
            let age = chrono::NaiveDateTime::parse_from_str(&c.fetched_at, "%Y-%m-%dT%H:%M:%S")
                .map(|t| chrono::Local::now().naive_local() - t)
                .unwrap_or(chrono::Duration::days(1));
            if age < chrono::Duration::minutes(FRESH_MINUTES) && c.error.is_none() {
                return Ok(c.clone());
            }
        }
    }
    match tauri::async_runtime::spawn_blocking(outlook::fetch).await.map_err(|e| e.to_string())? {
        Ok(dump) => {
            let now = chrono::Local::now().naive_local();
            let mut ctx = today::build(&dump, now.date(), now.format("%Y-%m-%dT%H:%M:%S").to_string());
            // Triage the day into now/later straight away (pure date logic, no
            // model, instant). The UI then calls `organise_day` in the
            // background to reword each line with the local model — the split
            // is already right, so nothing waits on that.
            if settings.ai_organise {
                ctx.organised = Some(ai::split(&ctx, now.date()));
            }
            write_json(&path, &serde_json::to_value(&ctx).map_err(|e| e.to_string())?)?;
            Ok(ctx)
        }
        Err(e) => match cached {
            Some(mut c) => {
                c.error = Some(e);
                Ok(c)
            }
            None => Err(e),
        },
    }
}

/// Open the email an item in Your day came from, in Outlook's own window.
/// `id` is the bare Outlook EntryID (hex); anything else is refused.
#[tauri::command]
async fn open_email(id: String) -> Result<(), String> {
    if id.is_empty() || id.len() > 1024 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("That doesn't look like an Outlook email id.".into());
    }
    tauri::async_runtime::spawn_blocking(move || outlook::open_email(id)).await.map_err(|e| e.to_string())?
}

/// Reword the cached day with the local model, in the background. `day_context`
/// has already returned the correct now/later split with plain titles; this
/// upgrades each line to a short summary and hands back the tidied plan for the
/// UI to swap in. Slow and best-effort by nature (a small local model), so it
/// is deliberately off the critical path — if it errors, the titles stand.
#[tauri::command]
async fn organise_day(app: AppHandle) -> Result<Option<ai::AiPlan>, String> {
    let settings = get_settings(app.clone())?;
    if !settings.ai_organise {
        return Ok(None);
    }
    let path = data_dir(&app)?.join("outlook.json");
    let ctx: today::Context = match read_json(&path).and_then(|v| serde_json::from_value(v).ok()) {
        Some(c) => c,
        None => return Ok(None),
    };
    let today = chrono::Local::now().date_naive();
    ai::organise(&ctx, today, &settings.ai_url, &settings.ai_model).await.map(Some)
}

/// The built-in lesson for a topic, as Markdown with maths. The UI renders it.
#[tauri::command]
fn get_lesson(topic_id: String) -> Result<String, String> {
    lessons::lesson(&topic_id).map(str::to_string).ok_or_else(|| format!("No lesson written for {topic_id} yet"))
}

/// The built-in mock papers, one entry per paper, for the subject pages.
#[tauri::command]
fn list_papers() -> Vec<papers::PaperInfo> {
    papers::all()
}

/// One mock paper's text (header plus questions). The UI renders and times it.
#[tauri::command]
fn get_paper(id: String) -> Result<String, String> {
    papers::paper(&id).map(str::to_string).ok_or_else(|| format!("No paper called {id}"))
}

#[tauri::command]
fn load_state(app: AppHandle) -> Result<Option<Value>, String> {
    Ok(read_json(&profile_dir(&app)?.join("state.json")))
}

#[tauri::command]
fn save_state(app: AppHandle, state: Value) -> Result<(), String> {
    write_json(&profile_dir(&app)?.join("state.json"), &state)
}

#[tauri::command]
fn state_path(app: AppHandle) -> Result<String, String> {
    Ok(profile_dir(&app)?.join("state.json").to_string_lossy().into_owned())
}

/// A timestamped copy of progress and configuration, so a bad edit or a
/// reinstall is never the end of two years of records.
#[tauri::command]
fn backup(app: AppHandle) -> Result<String, String> {
    let dir = data_dir(&app)?;
    let stamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S").to_string();
    let backups = dir.join("backups");
    fs::create_dir_all(&backups).map_err(|e| e.to_string())?;
    let target = backups.join(format!("backup-{stamp}.json"));
    let bundle = serde_json::json!({
        "savedAt": stamp,
        "state": read_json(&dir.join("state.json")),
        "config": read_json(&dir.join("config.json")),
    });
    write_json(&target, &bundle)?;
    Ok(target.to_string_lossy().into_owned())
}

// ---------- Profiles ----------

#[tauri::command]
fn list_profiles(app: AppHandle) -> Result<profiles::RegistryView, String> {
    store(&app)?.view()
}

#[tauri::command]
fn create_profile(app: AppHandle, name: String, pin: Option<String>) -> Result<String, String> {
    store(&app)?.create(&name, pin.as_deref())
}

#[tauri::command]
fn switch_profile(app: AppHandle, id: String, pin: Option<String>) -> Result<(), String> {
    store(&app)?.switch(&id, pin.as_deref())
}

#[tauri::command]
fn rename_profile(app: AppHandle, id: String, name: String) -> Result<(), String> {
    store(&app)?.rename(&id, &name)
}

#[tauri::command]
fn set_pin(app: AppHandle, id: String, old: Option<String>, new: Option<String>) -> Result<(), String> {
    store(&app)?.set_pin(&id, old.as_deref(), new.as_deref())
}

#[tauri::command]
fn delete_profile(app: AppHandle, id: String, pin: Option<String>) -> Result<(), String> {
    store(&app)?.delete(&id, pin.as_deref())
}

/// Writes the profile to a file the person can copy anywhere, and returns
/// where it went.
#[tauri::command]
fn export_profile(app: AppHandle, id: String) -> Result<String, String> {
    let s = store(&app)?;
    let bundle = s.export(&id)?;
    let name = bundle.get("name").and_then(Value::as_str).unwrap_or("profile").to_string();
    let safe: String = name.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    let dir = data_dir(&app)?.join("exports");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}-{}.g9profile.json", safe, chrono::Local::now().format("%Y-%m-%d")));
    write_json(&path, &bundle)?;
    Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
fn import_profile(app: AppHandle, bundle: Value) -> Result<String, String> {
    store(&app)?.import(&bundle)
}

/// The Microsoft Teams assignments the browser extension has synced, if any.
/// Shape: `{ "items": [...], "syncedAt": "..." }`.
#[tauri::command]
fn teams_assignments(app: AppHandle) -> Result<Value, String> {
    let path = data_dir(&app)?.join("teams.json");
    Ok(read_json(&path).unwrap_or_else(|| serde_json::json!({ "items": [] })))
}

/// True when this copy was started with an administrator token. Nothing here
/// wants one; it gets inherited from whatever launched us, and it breaks the
/// Outlook panel - see elevation.rs.
#[tauri::command]
fn running_elevated() -> bool {
    elevation::is_elevated()
}

/// Start an ordinary, unelevated copy and quit this one.
#[tauri::command]
fn restart_normally() -> Result<(), String> {
    elevation::restart_unelevated()?;
    // The replacement is Explorer's child and outlives us.
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(200));
        std::process::exit(0);
    });
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Receive Teams assignments pushed by the browser extension.
            if let Ok(dir) = data_dir(app.handle()) {
                teams_server::start(dir.join("teams.json"));
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_plan, get_config, save_config, reset_config, get_lesson, list_papers, get_paper,
            load_state, save_state, state_path, backup,
            get_settings, set_settings, draft_subject, day_context, organise_day, open_email,
            list_profiles, create_profile, switch_profile, rename_profile, set_pin, delete_profile, export_profile, import_profile,
            teams_assignments,
            update::check_update, update::install_update,
            tts::narrate, tts::voices, tts::download_voice,
            teams::open_teams,
            running_elevated, restart_normally
        ])
        .run(tauri::generate_context!())
        .expect("error while running Grade 9 Tracker");
}

#[cfg(test)]
mod command_registration {
    /// Every `#[tauri::command]` in this file must also be listed in
    /// `generate_handler!`, or the UI gets "Command X not found" at runtime -
    /// which is exactly how the profile picker once shipped dead.
    #[test]
    fn every_command_is_registered() {
        let src = include_str!("lib.rs");
        let handler = src.split("generate_handler![").nth(1).and_then(|s| s.split("])").next()).expect("handler list");
        let mut missing = Vec::new();
        for (i, line) in src.lines().enumerate() {
            if line.trim_start().starts_with("#[tauri::command]") {
                let sig = src.lines().nth(i + 1).unwrap_or("");
                let name = sig.split("fn ").nth(1).and_then(|s| s.split(['(', '<']).next()).unwrap_or("").trim();
                if !name.is_empty() && !handler.split(|c: char| !c.is_alphanumeric() && c != '_').any(|t| t == name) {
                    missing.push(name.to_string());
                }
            }
        }
        assert!(missing.is_empty(), "commands not registered in generate_handler!: {missing:?}");
    }
}
