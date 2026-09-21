//! Opening Microsoft Teams for the person to sign in and view assignments.
//!
//! Embedding Teams inside the app's own window doesn't work — the school tenant
//! blocks signing in from an embedded webview. Instead we open Teams in the
//! person's normal browser, where they're already signed in and where the
//! companion browser extension reads their assignments and syncs them back into
//! the app (see teams_server.rs). One click both opens Teams and triggers a sync.

use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

const TEAMS_URL: &str = "https://teams.microsoft.com/";

/// Open Teams in the default browser.
#[tauri::command]
pub fn open_teams(app: AppHandle) -> Result<(), String> {
    app.opener()
        .open_url(TEAMS_URL, None::<&str>)
        .map_err(|e| e.to_string())
}
