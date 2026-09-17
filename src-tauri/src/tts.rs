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
/// UI) with the bundled Jenny voice and return the WAV as base64.
#[tauri::command]
pub fn narrate(app: AppHandle, text: String) -> Result<String, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("nothing to read".into());
    }
    let base = tts_dir(&app)?;
    let exe = base.join("piper/piper.exe");
    let espeak = base.join("piper/espeak-ng-data");
    let model = base.join("voices/en_GB-jenny_dioco-medium.onnx");
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
