#!/usr/bin/env bash
# Build the session_viewer (wasm, release) and copy it into the docs site so the Viewer tab can
# embed it. This static copy is what ships to production — git_push.sh runs it before committing.
set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "→ trunk build --release session_viewer"
( cd "$ROOT/session_viewer" && trunk build --release )

echo "→ copy dist → session_tests/public/viewer"
rm -rf "$ROOT/session_tests/public/viewer"
mkdir -p "$ROOT/session_tests/public/viewer"
cp -r "$ROOT/session_viewer/dist/"* "$ROOT/session_tests/public/viewer/"

echo "done → session_tests/public/viewer (production-embedded viewer)"
