#!/usr/bin/env bash
# Build and run session_cpp/main.cpp
# Usage:
#   ./bash/run_cpp_main.sh          # Fast build + run (~8 sec if no changes)
#   ./bash/run_cpp_main.sh --clean  # Force cmake reconfigure
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
CPP_DIR="${REPO_ROOT}/session_cpp"
FORCE_CLEAN=false

for arg in "$@"; do
    case $arg in
        --clean|-c) FORCE_CLEAN=true ;;
    esac
done

cd "$CPP_DIR"

PLATFORM=$(detect_platform)
JOBS=$(get_jobs)

if [[ "$PLATFORM" == "windows" ]]; then
    EXE="./build/Release/MyProject.exe"
else
    EXE="./build/MyProject"
fi

# Skip configure if build exists (use --clean to force)
if [[ ! -d "build" ]] || [[ "$FORCE_CLEAN" == "true" ]]; then
    log_lang "cpp" "Configuring CMake..."
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release 2>&1 | grep -vE "^-- |^MSBuild|Completed '|Performing|No .* step"
fi

log_lang "cpp" "Building..."
if [[ "$PLATFORM" == "windows" ]]; then
    cmake --build build --config Release --parallel "${JOBS}" 2>&1 | grep -vE "\.vcxproj ->|\.lib$|\.exe$"
else
    cmake --build build --config Release -- -j"${JOBS}"
fi

log_lang "cpp" "Running main.cpp..."
"$EXE"
