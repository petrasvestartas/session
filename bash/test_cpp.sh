#!/usr/bin/env bash
# Run C++ minitest only - does NOT touch other languages' JSON
# Usage:
#   ./test_cpp.sh               # Build point_minitest only and run it
#   ./test_cpp.sh --clean       # Force cmake reconfigure
#   ./test_cpp.sh --all-targets # Also build main_* / main_wood_* executables
#   ./test_cpp.sh --no-viewer   # Don't update testData.js

set -e
set -o pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
CPP_DIR="${REPO_ROOT}/session_cpp"
UPDATE_VIEWER=true
FORCE_CLEAN=false
ALL_TARGETS=false
if [[ "${MINITEST_CPP_ALL:-0}" == "1" ]]; then
    ALL_TARGETS=true
fi

# Parse args
for arg in "$@"; do
    case $arg in
        --clean|-c) FORCE_CLEAN=true ;;
        --no-viewer) UPDATE_VIEWER=false ;;
        --all-targets) ALL_TARGETS=true ;;
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

TARGET_ARGS=()
if [[ "$ALL_TARGETS" != "true" ]]; then
    TARGET_ARGS=(--target point_minitest)
fi

log_lang "cpp" "Compiling..."
if [[ "$PLATFORM" == "windows" ]]; then
    cmake --build build --config Release --parallel "${JOBS}" "${TARGET_ARGS[@]}" 2>&1 | grep -vE "\.vcxproj ->|\.lib$|\.exe$"
else
    cmake --build build --config Release "${TARGET_ARGS[@]}" -- -j"${JOBS}"
fi

if [[ $? -ne 0 ]]; then
    log_lang "cpp" "Build failed"
    exit 1
fi

# Run tests
log_lang "cpp" "Running minitest..."
MINITEST_LOG="$(mktemp)"
if [[ "$PLATFORM" == "windows" ]]; then
    if [[ -f "./build/Release/point_minitest.exe" ]]; then
        ./build/Release/point_minitest.exe | tee "$MINITEST_LOG"
    elif [[ -f "./build/point_minitest.exe" ]]; then
        ./build/point_minitest.exe | tee "$MINITEST_LOG"
    else
        log_lang "cpp" "ERROR: point_minitest.exe not found"
        exit 1
    fi
else
    if [[ -x "./build/point_minitest" ]]; then
        ./build/point_minitest | tee "$MINITEST_LOG"
    elif [[ -x "./build/Release/point_minitest" ]]; then
        ./build/Release/point_minitest | tee "$MINITEST_LOG"
    else
        log_lang "cpp" "ERROR: point_minitest not found"
        exit 1
    fi
fi

cd "$REPO_ROOT"
PYTHON=$(get_python_path "$REPO_ROOT")
# The binary prints its own aggregate ("[cpp-minitest] N/N passed"). Pass it in so the
# summary RECONCILES the two counts instead of anyone adding them together (the old
# "1531 minitests" was 760 + 771 -- the same tests counted twice).
CPP_AGG=$(grep -oE "\[cpp-minitest\] [0-9]+/[0-9]+" "$MINITEST_LOG" 2>/dev/null | tail -1 | grep -oE "[0-9]+" | tail -1)
rm -f "$MINITEST_LOG"
print_class_summary "cpp" "${REPO_ROOT}/session_tests/session_cpp" "$PYTHON" "$CPP_AGG" || {
    log_lang "cpp" "class-summary reported stale/missing/failing classes (see above)"
    exit 1
}
log_lang "cpp" "Tests complete"

# Update viewer if requested
if [[ "$UPDATE_VIEWER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/consolidate.sh"
    consolidate_test_data "$REPO_ROOT"
fi
