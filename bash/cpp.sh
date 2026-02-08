#!/usr/bin/env bash
# Build and run session_cpp main_1, main_2, main_3
# Usage:
#   ./bash/cpp.sh              # Build all 3 + run all
#   ./bash/cpp.sh 1            # Build all + run main_1 only
#   ./bash/cpp.sh 1 2          # Build all + run main_1 and main_2
#   ./bash/cpp.sh --clean      # Force cmake reconfigure
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
CPP_DIR="${REPO_ROOT}/session_cpp"
FORCE_CLEAN=false
TARGETS=()

for arg in "$@"; do
    case $arg in
        --clean|-c) FORCE_CLEAN=true ;;
        1|2|3) TARGETS+=("$arg") ;;
    esac
done

# Default: run all 3
if [[ ${#TARGETS[@]} -eq 0 ]]; then
    TARGETS=(1 2 3)
fi

cd "$CPP_DIR"

PLATFORM=$(detect_platform)
JOBS=$(get_jobs)

# Skip configure if build exists (use --clean to force)
if [[ ! -d "build" ]] || [[ "$FORCE_CLEAN" == "true" ]]; then
    log_lang "cpp" "Configuring CMake..."
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release 2>&1 | grep -vE "^-- |^MSBuild|Completed '|Performing|No .* step|absl|abseil"
fi

log_lang "cpp" "Building..."
if [[ "$PLATFORM" == "windows" ]]; then
    cmake --build build --config Release --parallel "${JOBS}" 2>&1 | grep -vE "\.vcxproj ->|\.lib$|\.exe$|absl|abseil"
else
    cmake --build build --config Release -- -j"${JOBS}"
fi

for t in "${TARGETS[@]}"; do
    if [[ "$PLATFORM" == "windows" ]]; then
        EXE="./build/Release/main_${t}.exe"
    else
        EXE="./build/main_${t}"
    fi
    log_lang "cpp" "Running main_${t}..."
    "$EXE"
done
