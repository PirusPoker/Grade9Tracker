//! Grade 9 Tracker — Tauri backend.
//! Builds the study plan from an editable configuration and saves progress to
//! JSON files in the app's data folder. The app finds and orders the work; the
//! questions themselves live on Save My Exams, which each person opens with
//! their own account.

mod config;
mod course;
mod draft;
mod outlook;
mod plan;
mod profiles;
mod statements;
mod today;

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
    if !get_settings(app.clone())?.outlook {
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
            let ctx = today::build(&dump, now.date(), now.format("%Y-%m-%dT%H:%M:%S").to_string());
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_plan, get_config, save_config, reset_config,
            load_state, save_state, state_path, backup,
            get_settings, set_settings, draft_subject, day_context
        ])
        .run(tauri::generate_context!())
        .expect("error while running Grade 9 Tracker");
}
