//! Offline neural narration with Piper.
//!
//! The browser's built-in speech only exposes the machine's old SAPI voices,
//! which sound robotic. Piper is a small neural text-to-speech engine that
//! runs on the CPU, offline, with no data leaving the machine. We bundle the
//! engine and one British voice (Jenny) as app resources; this command turns a
//! block of already-cleaned lesson prose into a WAV and hands it back as base64
//! for the web layer to play. Speed is applied on the audio element, so the
//! same synthesis serves any playback rate.

use base64::Engine;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use tauri::{AppHandle, Manager};

static SEQ: AtomicU64 = AtomicU64::new(0);

/// Bundled voices: (id used for the `.onnx` filename, label for the picker).
/// The first entry is the default. `scripts/fetch-tts.sh` must download every
/// id here, or a listed voice simply won't appear in the picker.
pub const VOICES: &[(&str, &str)] = &[
    ("en_GB-jenny_dioco-medium", "Jenny — British, warm"),
    ("en_GB-alba-medium", "Alba — British, bright"),
    ("en_GB-alan-medium", "Alan — British, male"),
    ("en_GB-northern_english_male-medium", "Northern English — male"),
];
const DEFAULT_VOICE: &str = "en_GB-jenny_dioco-medium";

/// Only ever synthesise with a voice we ship, defaulting to Jenny.
fn resolve_voice(requested: Option<&str>) -> &'static str {
    match requested {
        Some(r) => VOICES
            .iter()
            .map(|(id, _)| *id)
            .find(|id| *id == r)
            .unwrap_or(DEFAULT_VOICE),
        None => DEFAULT_VOICE,
    }
}

#[derive(serde::Serialize)]
pub struct VoiceOption {
    id: String,
    label: String,
}

/// The voices actually present on disk, in catalogue order, for the picker.
#[tauri::command]
pub fn voices(app: AppHandle) -> Vec<VoiceOption> {
    let base = match tts_dir(&app) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };
    VOICES
        .iter()
        .filter(|(id, _)| base.join(format!("voices/{id}.onnx")).exists())
        .map(|(id, label)| VoiceOption {
            id: id.to_string(),
            label: label.to_string(),
        })
        .collect()
}

/// The folder holding `piper/` and `voices/`, whether bundled (release) or
/// sitting in the source tree (dev).
fn tts_dir(app: &AppHandle) -> Result<PathBuf, String> {
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
    Err("Narrator voice files are not installed".into())
}

/// Synthesise `text` (plain prose, already stripped of markup and maths by the
/// UI) with the chosen bundled voice (default Jenny) and return the WAV as
/// base64.
#[tauri::command]
pub fn narrate(app: AppHandle, text: String, voice: Option<String>) -> Result<String, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("nothing to read".into());
    }
    let base = tts_dir(&app)?;
    let exe = base.join("piper/piper.exe");
    let espeak = base.join("piper/espeak-ng-data");
    let vid = resolve_voice(voice.as_deref());
    let mut model = base.join(format!("voices/{vid}.onnx"));
    if !model.exists() {
        // Fall back to the default voice if the requested one isn't on disk.
        model = base.join(format!("voices/{DEFAULT_VOICE}.onnx"));
    }
    if !model.exists() {
        return Err("Narrator voice file is missing".into());
    }

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
