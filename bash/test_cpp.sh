#!/usr/bin/env bash
# Run C++ minitest only - does NOT touch other languages' JSON
# Usage:
#   ./test_cpp.sh              # Build and run (skips cmake if build exists)
#   ./test_cpp.sh --clean      # Force cmake reconfigure
#   ./test_cpp.sh --no-viewer  # Don't update testData.js

set -e
set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
CPP_DIR="${REPO_ROOT}/session_cpp"
UPDATE_VIEWER=true
FORCE_CLEAN=false

# Parse args
for arg in "$@"; do
    case $arg in
        --clean|-c) FORCE_CLEAN=true ;;
        --no-viewer) UPDATE_VIEWER=false ;;
    esac
done

if [[ ! -d "$CPP_DIR" ]]; then
    log_lang "cpp" "ERROR: session_cpp not found at ${CPP_DIR}"
    exit 1
fi

JOBS=$(get_jobs)
PLATFORM=$(detect_platform)

# Build
log_lang "cpp" "Building with ${JOBS} jobs..."

cd "$CPP_DIR"

# Skip configure if build exists (use --clean to force reconfigure)
if [[ ! -d "build" ]] || [[ "$FORCE_CLEAN" == "true" ]]; then
    log_lang "cpp" "Configuring CMake..."
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release 2>&1 | grep -vE "^-- |^MSBuild|Completed '|Performing|No .* step"
fi

log_lang "cpp" "Compiling..."
if [[ "$PLATFORM" == "windows" ]]; then
    cmake --build build --config Release --parallel "${JOBS}" 2>&1 | grep -vE "\.vcxproj ->|\.lib$|\.exe$"
else
    cmake --build build --config Release -- -j"${JOBS}"
fi

if [[ $? -ne 0 ]]; then
    log_lang "cpp" "Build failed"
    exit 1
fi

# Run tests
log_lang "cpp" "Running minitest..."
if [[ "$PLATFORM" == "windows" ]]; then
    if [[ -f "./build/Release/point_minitest.exe" ]]; then
        ./build/Release/point_minitest.exe
    elif [[ -f "./build/point_minitest.exe" ]]; then
        ./build/point_minitest.exe
    else
        log_lang "cpp" "ERROR: point_minitest.exe not found"
        exit 1
    fi
else
    if [[ -x "./build/point_minitest" ]]; then
        ./build/point_minitest
    elif [[ -x "./build/Release/point_minitest" ]]; then
        ./build/Release/point_minitest
    else
        log_lang "cpp" "ERROR: point_minitest not found"
        exit 1
    fi
fi

cd "$REPO_ROOT"
PYTHON=$(get_python_path "$REPO_ROOT")
print_class_summary "cpp" "${REPO_ROOT}/session_tests/session_cpp" "$PYTHON"
log_lang "cpp" "Tests complete"

# Update viewer if requested
if [[ "$UPDATE_VIEWER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/consolidate.sh"
    consolidate_test_data "$REPO_ROOT"
fi
