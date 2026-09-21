#!/usr/bin/env bash
# Fetch the Piper neural TTS engine into src-tauri/resources/tts/, where
# tauri.conf.json bundles it and tts.rs finds it at runtime. The engine is
# ~40 MB so it is NOT committed: run this once locally before `cargo tauri dev`,
# and CI runs it before each release build.
#
# Voices are NOT bundled: the app downloads them on first use into its data
# folder (see tts.rs). Pass --voices to fetch them here too, which lets a dev
# build narrate without a download.
set -euo pipefail

WANT_VOICES=0
[ "${1:-}" = "--voices" ] && WANT_VOICES=1

PIPER_TAG="2023.11.14-2"
VOICES_ROOT="https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_GB"
# id -> path under en_GB (must match VOICES in src-tauri/src/tts.rs)
VOICES="
en_GB-jenny_dioco-medium jenny_dioco/medium
en_GB-alba-medium alba/medium
en_GB-alan-medium alan/medium
en_GB-northern_english_male-medium northern_english_male/medium
"

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

[ "$WANT_VOICES" = 1 ] && echo "$VOICES" | while read -r id path; do
  [ -z "$id" ] && continue
  if [ ! -f "$BASE/voices/$id.onnx" ]; then
    echo "Downloading voice $id ..."
    curl -L --fail -o "$BASE/voices/$id.onnx"      "$VOICES_ROOT/$path/$id.onnx"
    curl -L --fail -o "$BASE/voices/$id.onnx.json" "$VOICES_ROOT/$path/$id.onnx.json"
  fi
done

echo "TTS engine ready in $BASE$([ "$WANT_VOICES" = 1 ] && echo " (with voices)")"
