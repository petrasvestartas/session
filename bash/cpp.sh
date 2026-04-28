#!/usr/bin/env bash
# Build and run every executable in session_cpp.
# Usage:
#   ./bash/cpp.sh              # Build all + run every main_* / example_*
#   ./bash/cpp.sh main_5       # Build all + run only the listed exes
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
        *)          TARGETS+=("$arg") ;;
    esac
done

cd "$CPP_DIR"

PLATFORM=$(detect_platform)
JOBS=$(get_jobs)

if [[ ! -d "build" ]] || [[ "$FORCE_CLEAN" == "true" ]]; then
    log_lang "cpp" "Configuring CMake..."
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Release 2>&1 | grep -vE "^-- |^MSBuild|Completed '|Performing|No .* step|absl|abseil" || true
fi

log_lang "cpp" "Building..."
if [[ "$PLATFORM" == "windows" ]]; then
    cmake --build build --config Release --parallel "${JOBS}" 2>&1 | grep -vE "\.vcxproj ->|\.lib$|\.exe$|absl|abseil" || true
    BIN_DIR="build/Release"
    EXE_EXT=".exe"
else
    cmake --build build --config Release -- -j"${JOBS}"
    BIN_DIR="build"
    EXE_EXT=""
fi

# Default: every main_* / example_* in BIN_DIR (skip the test runner).
if [[ ${#TARGETS[@]} -eq 0 ]]; then
    while IFS= read -r f; do
        name=$(basename "$f" "$EXE_EXT")
        [[ "$name" == *minitest* ]] && continue
        TARGETS+=("$name")
    done < <(ls "$BIN_DIR"/main_*"$EXE_EXT" "$BIN_DIR"/example_*"$EXE_EXT" 2>/dev/null | sort)
fi

for t in "${TARGETS[@]}"; do
    EXE="$BIN_DIR/${t}${EXE_EXT}"
    if [[ ! -x "$EXE" && ! -f "$EXE" ]]; then
        log_lang "cpp" "Skipping ${t}: ${EXE} not found"
        continue
    fi
    log_lang "cpp" "Running ${t}..."
    if ! "$EXE"; then
        log_lang "cpp" "${t} failed (exit=$?), continuing"
    fi
done
