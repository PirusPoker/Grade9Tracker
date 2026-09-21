#!/usr/bin/env bash
# Build the browser edition into site/:
#   site/            the download page (docs/)
#   site/app/        the app itself - ui/index.html on top of the Rust core
#                    compiled to WebAssembly, plus the bridge, manifest,
#                    service worker and icons from web/
# Needs the wasm32 target and wasm-bindgen-cli (see .github/workflows/pages.yml).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/site}"
VERSION="$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || date +%s)"

echo "Compiling the core to WebAssembly..."
( cd "$ROOT/web" && cargo build --release --target wasm32-unknown-unknown )
TARGET="${CARGO_TARGET_DIR:-$ROOT/web/target}"
WASM="$(ls "$TARGET/wasm32-unknown-unknown/release/"*.wasm | head -1)"

rm -rf "$OUT"
mkdir -p "$OUT/app"
cp -r "$ROOT/docs/." "$OUT/"
wasm-bindgen --target web --no-typescript --out-dir "$OUT/app" --out-name g9 "$WASM"
cp -r "$ROOT/ui/katex" "$OUT/app/katex"
cp "$ROOT/web/web.js" "$ROOT/web/manifest.webmanifest" "$ROOT/web/icon-192.png" "$ROOT/web/icon-512.png" "$OUT/app/"
sed "s/__VERSION__/$VERSION/" "$ROOT/web/sw.js" > "$OUT/app/sw.js"

# The interface, with the bridge loaded before the app script and the
# home-screen metadata in the head.
"${PYTHON:-python3}" - "$ROOT/ui/index.html" "$OUT/app/index.html" "$VERSION" <<'EOF'
import sys, re
src, dst, ver = sys.argv[1:4]
html = open(src, encoding="utf-8").read()
head = ('<link rel="manifest" href="manifest.webmanifest">\n'
        '<meta name="theme-color" content="#131314">\n'
        '<meta name="apple-mobile-web-app-capable" content="yes">\n'
        '<meta name="mobile-web-app-capable" content="yes">\n'
        '<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">\n'
        '<meta name="apple-mobile-web-app-title" content="Grade 9">\n'
        '<link rel="apple-touch-icon" href="icon-192.png">\n'
        f'<meta name="g9-build" content="{ver}">\n')
html = html.replace('<link rel="stylesheet" href="katex/katex.min.css">', head + '<link rel="stylesheet" href="katex/katex.min.css">', 1)
m = re.search(r'<script src="katex/katex.min.js"></script>', html)
assert m, "katex script tag not found"
html = html[:m.start()] + '<script src="web.js"></script>\n' + html[m.start():]
# the viewport: allow the safe-area insets on notched phones
html = html.replace('content="width=device-width, initial-scale=1"', 'content="width=device-width, initial-scale=1, viewport-fit=cover"', 1)
open(dst, "w", encoding="utf-8", newline="\n").write(html)
print("wrote", dst)
EOF

echo "Site built in $OUT ($(du -sh "$OUT" | cut -f1))"
