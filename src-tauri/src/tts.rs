//! Offline neural narration with Piper.
//!
//! The browser's built-in speech only exposes the machine's old SAPI voices,
//! which sound robotic. Piper is a small neural text-to-speech engine that
//! runs on the CPU, offline, with no data leaving the machine. We bundle the
//! engine (~40 MB) as an app resource; the voices (~60 MB each) are fetched
//! on first use into the app's data folder, so the installer stays small and
//! nobody downloads voices they never pick. `narrate` turns a block of
//! already-cleaned lesson prose into a WAV and hands it back as base64 for the
//! web layer to play. Speed is applied on the audio element, so the same
//! synthesis serves any playback rate.

use base64::Engine;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

static SEQ: AtomicU64 = AtomicU64::new(0);
/// Voice ids with a download in flight, so two clicks don't fetch twice.
static DOWNLOADING: Mutex<Option<HashSet<String>>> = Mutex::new(None);

/// The voice catalogue: (id used for the `.onnx` filename, label for the
/// picker, path under `en/en_GB` on Hugging Face). The first entry is the
/// default. `scripts/fetch-tts.sh --voices` downloads the same files for
/// development.
pub const VOICES: &[(&str, &str, &str)] = &[
    ("en_GB-jenny_dioco-medium", "Jenny — British, warm", "jenny_dioco/medium"),
    ("en_GB-alba-medium", "Alba — British, bright", "alba/medium"),
    ("en_GB-alan-medium", "Alan — British, male", "alan/medium"),
    ("en_GB-northern_english_male-medium", "Northern English — male", "northern_english_male/medium"),
];
const DEFAULT_VOICE: &str = "en_GB-jenny_dioco-medium";
const VOICES_ROOT: &str = "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB";
/// Every medium en_GB voice is this size, give or take a few hundred bytes;
/// used for the picker label and as the progress total until the server says.
const VOICE_BYTES: u64 = 63_201_294;

/// Only ever synthesise with a voice we know, defaulting to Jenny.
fn catalogue(id: &str) -> Option<&'static (&'static str, &'static str, &'static str)> {
    VOICES.iter().find(|(vid, _, _)| *vid == id)
}

#[derive(serde::Serialize)]
pub struct VoiceOption {
    id: String,
    label: String,
    installed: bool,
    size_mb: u32,
}

#[derive(serde::Serialize, Clone)]
struct Progress {
    id: String,
    done: u64,
    total: u64,
}

/// Where downloaded voices live: `<app data>/tts/voices`.
fn voices_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("tts")
        .join("voices");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

/// The `.onnx` for a voice, if it is on disk anywhere we look: the download
/// folder first, then the bundled resources (older installs shipped voices
/// inside the app) and the source tree in development.
fn voice_model(app: &AppHandle, id: &str) -> Option<PathBuf> {
    let name = format!("{id}.onnx");
    let mut candidates = Vec::new();
    if let Ok(d) = voices_dir(app) {
        candidates.push(d.join(&name));
    }
    if let Ok(p) = app
        .path()
        .resolve("resources/tts/voices", tauri::path::BaseDirectory::Resource)
    {
        candidates.push(p.join(&name));
    }
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/tts/voices")
            .join(&name),
    );
    candidates
        .into_iter()
        .find(|p| p.exists() && p.with_extension("onnx.json").exists())
}

/// The whole catalogue, flagged with whether each voice is on disk yet.
#[tauri::command]
pub fn voices(app: AppHandle) -> Vec<VoiceOption> {
    VOICES
        .iter()
        .map(|(id, label, _)| VoiceOption {
            id: id.to_string(),
            label: label.to_string(),
            installed: voice_model(&app, id).is_some(),
            size_mb: (VOICE_BYTES / 1_000_000) as u32,
        })
        .collect()
}

/// Fetch one voice (model + config) into the data folder, reporting progress
/// as `tts-progress` events. Safe to call for a voice that is already there.
#[tauri::command]
pub async fn download_voice(app: AppHandle, id: String) -> Result<(), String> {
    let (vid, _, path) = *catalogue(&id).ok_or("unknown voice")?;
    if voice_model(&app, vid).is_some() {
        return Ok(());
    }
    {
        let mut guard = DOWNLOADING.lock().map_err(|e| e.to_string())?;
        let set = guard.get_or_insert_with(HashSet::new);
        if !set.insert(vid.to_string()) {
            return Err("that voice is already downloading".into());
        }
    }
    let result = fetch_voice(&app, vid, path).await;
    if let Ok(mut guard) = DOWNLOADING.lock() {
        if let Some(set) = guard.as_mut() {
            set.remove(vid);
        }
    }
    result
}

async fn fetch_voice(app: &AppHandle, vid: &str, path: &str) -> Result<(), String> {
    let dir = voices_dir(app)?;
    let client = reqwest::Client::builder()
        .user_agent("Grade9Tracker")
        .build()
        .map_err(|e| e.to_string())?;
    // The small config first: if the network is down we find out in a second
    // rather than after 60 MB.
    let cfg = client
        .get(format!("{VOICES_ROOT}/{path}/{vid}.onnx.json"))
        .send()
        .await
        .map_err(|e| format!("could not reach the voice server: {e}"))?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let model_tmp = dir.join(format!("{vid}.onnx.part"));
    let mut resp = client
        .get(format!("{VOICES_ROOT}/{path}/{vid}.onnx"))
        .send()
        .await
        .map_err(|e| format!("could not reach the voice server: {e}"))?
        .error_for_status()
        .map_err(|e| e.to_string())?;
    let total = resp.content_length().unwrap_or(VOICE_BYTES);
    let mut file = std::fs::File::create(&model_tmp).map_err(|e| e.to_string())?;
    let mut done: u64 = 0;
    let mut last_report: u64 = 0;
    let _ = app.emit("tts-progress", Progress { id: vid.into(), done, total });
    loop {
        let chunk = match resp.chunk().await {
            Ok(Some(c)) => c,
            Ok(None) => break,
            Err(e) => {
                let _ = std::fs::remove_file(&model_tmp);
                return Err(format!("download interrupted: {e}"));
            }
        };
        if let Err(e) = file.write_all(&chunk) {
            let _ = std::fs::remove_file(&model_tmp);
            return Err(format!("could not save the voice: {e}"));
        }
        done += chunk.len() as u64;
        if done - last_report >= 1_000_000 {
            last_report = done;
            let _ = app.emit("tts-progress", Progress { id: vid.into(), done, total });
        }
    }
    drop(file);
    if done < 1_000_000 {
        let _ = std::fs::remove_file(&model_tmp);
        return Err("the voice file came back empty".into());
    }
    std::fs::write(dir.join(format!("{vid}.onnx.json")), &cfg).map_err(|e| e.to_string())?;
    std::fs::rename(&model_tmp, dir.join(format!("{vid}.onnx"))).map_err(|e| e.to_string())?;
    let _ = app.emit("tts-progress", Progress { id: vid.into(), done: total, total });
    Ok(())
}

/// The folder holding the bundled `piper/` engine, in a release build or the
/// source tree.
fn engine_dir(app: &AppHandle) -> Result<PathBuf, String> {
    if let Ok(p) = app
        .path()
        .resolve("resources/tts", tauri::path::BaseDirectory::Resource)
    {
        if p.join("piper/piper.exe").exists() {
            return Ok(p);
        }
    }
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/tts");
    if dev.join("piper/piper.exe").exists() {
        return Ok(dev);
    }
    Err("Narrator engine is not installed".into())
}

/// Synthesise `text` (plain prose, already stripped of markup and maths by the
/// UI) with the chosen voice and return the WAV as base64. A voice that has
/// not been downloaded yet gives `voice-missing:<id>` so the UI can offer the
/// download; if the default is on disk it is used instead.
#[tauri::command]
pub fn narrate(app: AppHandle, text: String, voice: Option<String>) -> Result<String, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("nothing to read".into());
    }
    let base = engine_dir(&app)?;
    let exe = base.join("piper/piper.exe");
    let espeak = base.join("piper/espeak-ng-data");
    let wanted = voice
        .as_deref()
        .filter(|v| catalogue(v).is_some())
        .unwrap_or(DEFAULT_VOICE);
    let model = match voice_model(&app, wanted).or_else(|| voice_model(&app, DEFAULT_VOICE)) {
        Some(m) => m,
        None => return Err(format!("voice-missing:{wanted}")),
    };

    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let out = std::env::temp_dir().join(format!("g9-narrate-{}-{}.wav", std::process::id(), seq));

    let mut cmd = Command::new(&exe);
    cmd.arg("-m")
        .arg(&model)
        .arg("-f")
        .arg(&out)
        .arg("--espeak_data")
        .arg(&espeak)
        .current_dir(base.join("piper"))
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd.spawn().map_err(|e| format!("could not start narrator: {e}"))?;
    child
        .stdin
        .take()
        .ok_or("no stdin for narrator")?
        .write_all(text.as_bytes())
        .map_err(|e| e.to_string())?;
    let status = child.wait().map_err(|e| e.to_string())?;
    if !status.success() {
        let _ = std::fs::remove_file(&out);
        return Err("narrator could not read that".into());
    }
    let bytes = std::fs::read(&out).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&out);
    if bytes.is_empty() {
        return Err("narrator produced no audio".into());
    }
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_voice_is_in_the_catalogue_and_first() {
        assert_eq!(VOICES[0].0, DEFAULT_VOICE);
        assert!(catalogue(DEFAULT_VOICE).is_some());
        assert!(catalogue("en_GB-nobody").is_none());
    }

    #[test]
    fn every_catalogue_entry_has_a_download_path() {
        for (id, label, path) in VOICES {
            assert!(id.starts_with("en_GB-"), "{id}");
            assert!(!label.is_empty());
            assert!(path.ends_with("/medium"), "{id}: {path}");
        }
    }
}
