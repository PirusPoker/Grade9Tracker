#!/usr/bin/env bash
# Fetch the Piper neural TTS engine and the Jenny (en_GB) narrator voice into
# src-tauri/resources/tts/, where tauri.conf.json bundles them and tts.rs finds
# them at runtime. These files are large (~98 MB) so they are NOT committed:
# run this once locally before `cargo tauri dev`, and CI runs it before each
# release build.
set -euo pipefail

PIPER_TAG="2023.11.14-2"
VOICE="en_GB-jenny_dioco-medium"
VOICE_URL="https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB/jenny_dioco/medium"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BASE="$ROOT/src-tauri/resources/tts"
mkdir -p "$BASE/piper" "$BASE/voices"

# Extract a .zip into a directory. The Actions bash shell is Git Bash, whose
# GNU tar cannot read zips, so prefer unzip (present on windows-latest) and fall
# back to PowerShell's Expand-Archive with Windows-style paths.
extract_zip() {
  local zip="$1" dest="$2"
  if command -v unzip >/dev/null 2>&1; then
    unzip -q -o "$zip" -d "$dest"
  elif command -v powershell >/dev/null 2>&1; then
    local wzip wdest
    wzip="$(cygpath -w "$zip" 2>/dev/null || echo "$zip")"
    wdest="$(cygpath -w "$dest" 2>/dev/null || echo "$dest")"
    powershell -NoProfile -Command "Expand-Archive -Path '$wzip' -DestinationPath '$wdest' -Force"
  else
    tar -xf "$zip" -C "$dest"   # last resort: bsdtar (Windows tar.exe) reads zips
  fi
}

if [ ! -f "$BASE/piper/piper.exe" ]; then
  echo "Downloading Piper $PIPER_TAG ..."
  curl -L --fail -o "$BASE/piper.zip" \
    "https://github.com/rhasspy/piper/releases/download/$PIPER_TAG/piper_windows_amd64.zip"
  extract_zip "$BASE/piper.zip" "$BASE"   # the archive contains a top-level piper/ folder
  rm -f "$BASE/piper.zip"
fi

if [ ! -f "$BASE/voices/$VOICE.onnx" ]; then
  echo "Downloading voice $VOICE ..."
  curl -L --fail -o "$BASE/voices/$VOICE.onnx"      "$VOICE_URL/$VOICE.onnx"
  curl -L --fail -o "$BASE/voices/$VOICE.onnx.json" "$VOICE_URL/$VOICE.onnx.json"
fi

echo "TTS assets ready in $BASE"
