#!/usr/bin/env bash
# Rhino 9 embeds CPython 3.13; the folder name is a guess until checked on a Rhino 9 machine
RHINOCODE_ENV="${RHINOCODE_ENV:-py313-rh9}"
RHINO_PY="$HOME/.rhinocode/$RHINOCODE_ENV"
PIP="$RHINO_PY/python.exe -m pip"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Rhino uses site-envs/ with hashed folder names
TARGET=$(find "$RHINO_PY/site-envs" -maxdepth 1 -name "session_py-*" -type d | head -1)

if [ -z "$TARGET" ]; then
    echo "ERROR: No session_py venv found in $RHINO_PY/site-envs/"
    echo "Create the venv in Rhino first by running a script with '# venv: session_py'"
    exit 1
fi

echo "Installing to: $TARGET"
$PIP install "$ROOT/session_py" --target "$TARGET"
$PIP install "$ROOT/session_rhino" --target "$TARGET"
