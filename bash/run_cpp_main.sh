#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
CPP_DIR="${REPO_ROOT}/session_cpp"

cd "$CPP_DIR"

PLATFORM=$(detect_platform)

if [[ "$PLATFORM" == "windows" ]]; then
    EXE="./build/Release/session_cpp.exe"
else
    EXE="./build/session_cpp"
fi

if [[ ! -f "$EXE" ]]; then
    log_lang "cpp" "Building first..."
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
    cmake --build build --config Release
fi

log_lang "cpp" "Running main.cpp example..."
"$EXE"
