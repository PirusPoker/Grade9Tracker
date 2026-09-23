//! Opening Microsoft Teams for the person to sign in and view assignments.
//!
//! Embedding Teams inside the app's own window doesn't work — the school tenant
//! blocks signing in from an embedded webview. Instead we open Teams in the
//! person's normal browser, where they're already signed in and where the
//! companion browser extension reads their assignments and syncs them back into
//! the app (see teams_server.rs). One click both opens Teams and triggers a sync.
//!
//! The extension is a Chrome extension, so Teams has to open in Chrome — the
//! Windows default browser may well be Edge (it is on the machine this was built
//! for), where there is no extension and nothing would sync. Handing the link to
//! the shell also misfired there: it came back as Teams' "Unsupported Browser"
//! page saved to the IE cache and opened as a local file. So we start Chrome
//! ourselves with the URL, and only fall back to the default browser when Chrome
//! isn't installed.

use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

const TEAMS_URL: &str = "https://teams.microsoft.com/";

/// Where Chrome installs itself: machine-wide (64- or 32-bit) or per-user.
fn chrome_exe() -> Option<PathBuf> {
    ["ProgramFiles", "ProgramFiles(x86)", "LOCALAPPDATA"]
        .iter()
        .filter_map(|var| std::env::var_os(var))
        .map(|base| PathBuf::from(base).join(r"Google\Chrome\Application\chrome.exe"))
        .find(|p| p.is_file())
}

/// Open Teams in Chrome, or the default browser if Chrome isn't installed.
#[tauri::command]
pub fn open_teams(app: AppHandle) -> Result<(), String> {
    if let Some(chrome) = chrome_exe() {
        if std::process::Command::new(chrome).arg(TEAMS_URL).spawn().is_ok() {
            return Ok(());
        }
    }
    app.opener()
        .open_url(TEAMS_URL, None::<&str>)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_path_if_found_is_chrome_exe() {
        if let Some(p) = chrome_exe() {
            assert!(p.is_file());
            assert!(p.ends_with(r"Application\chrome.exe"));
        }
    }
}
