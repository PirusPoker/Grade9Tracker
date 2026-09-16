//! Keeping the installed app current. On launch the UI asks whether a newer
//! release exists; if so it offers one button that downloads the signed
//! installer, verifies it against the public key in tauri.conf.json, installs
//! it and restarts. Releases are published by the GitHub Action in
//! .github/workflows/release.yml, which writes the latest.json the check reads.
//! Nothing is downloaded until the person presses the button.

use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Available {
    pub version: String,
    pub current: String,
    /// The release notes, as written on the GitHub release.
    pub notes: String,
}

/// The newer version waiting, if any. Offline or unreachable is an error the
/// UI ignores - the app must not nag about a check it could not make.
#[tauri::command]
pub async fn check_update(app: AppHandle) -> Result<Option<Available>, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await.map_err(|e| e.to_string())? {
        Some(u) => Ok(Some(Available {
            version: u.version.clone(),
            current: u.current_version.clone(),
            notes: u.body.clone().unwrap_or_default(),
        })),
        None => Ok(None),
    }
}

/// Download, verify, install and restart. Only returns on failure.
#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    let update = updater.check().await.map_err(|e| e.to_string())?.ok_or("No update is available")?;
    update.download_and_install(|_, _| {}, || {}).await.map_err(|e| e.to_string())?;
    app.restart();
}
