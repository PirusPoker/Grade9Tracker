//! Signing in to Microsoft Teams inside the app.
//!
//! The school tenant blocks the Graph API, and a separate browser means logging
//! in every time. Instead we open Teams web in its own window that belongs to
//! this app and uses the app's own WebView2 profile, so the sign-in (MFA and
//! all) persists between launches. Once the person is signed in here, a later
//! step can read their assignments straight off the page.

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const TEAMS_URL: &str = "https://teams.microsoft.com/";

/// Open the in-app Teams window, or focus it if it is already open.
#[tauri::command]
pub fn open_teams(app: AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("teams") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
        return Ok(());
    }
    let url = TEAMS_URL.parse().map_err(|_| "bad Teams URL".to_string())?;
    WebviewWindowBuilder::new(&app, "teams", WebviewUrl::External(url))
        .title("Microsoft Teams — Grade 9 Tracker")
        .inner_size(1180.0, 840.0)
        .min_inner_size(720.0, 560.0)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}
