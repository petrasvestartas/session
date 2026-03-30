#!/usr/bin/env bash
# Run Python minitest only - does NOT touch other languages' JSON
# Usage:
#   ./test_py.sh              # Run all Python tests
#   ./test_py.sh point        # Run only Point tests
#   ./test_py.sh --fast       # Skip pip install if already installed
#   ./test_py.sh --no-viewer  # Don't update testData.js

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
FAST_MODE=false
UPDATE_VIEWER=true
CLASS_FILTER=""

# Parse args
for arg in "$@"; do
    case $arg in
        --fast|-f) FAST_MODE=true ;;
        --no-viewer) UPDATE_VIEWER=false ;;
        -*) ;; # ignore unknown flags
        *) CLASS_FILTER="$arg" ;;
    esac
done

PYTHON=$(get_python_path "$REPO_ROOT")

# Ensure Python environment exists and session_py is installed
ensure_python_env() {
    if [[ ! -f "$PYTHON" ]]; then
        log_lang "py" "Creating Python environment..."
        if has_uv; then
            (cd "$REPO_ROOT" && uv venv uvsession)
        else
            local py_cmd="python3"
            [[ "$(detect_platform)" == "windows" ]] && py_cmd="python"
            (cd "$REPO_ROOT" && $py_cmd -m venv uvsession)
        fi
    fi

    if [[ ! -f "$PYTHON" ]]; then
        log_lang "py" "ERROR: Failed to create Python environment"
        exit 1
    fi

    # Fast mode: skip install if already working
    if [[ "$FAST_MODE" == "true" ]]; then
        if "$PYTHON" -c "import session_py" 2>/dev/null; then
            log_lang "py" "Fast mode: session_py already installed"
            return 0
        fi
    fi

    log_lang "py" "Installing session_py..."
    local py_path=$(to_windows_path "${REPO_ROOT}/session_py")
    if has_uv; then
        uv pip install --python "$PYTHON" -e "$py_path" pytest
    else
        "$PYTHON" -m pip install -e "$py_path" pytest
    fi
}

ensure_python_env

# Run a single class test and prefix output with class name
run_py_test() {
    local class_name="$1"
    local output exit_code
    output=$("$PYTHON" -m "session_py.${class_name}_test" 2>&1)
    exit_code=$?
    printf "%s\n" "$output" | sed "s/\[py-minitest\]/[py-${class_name}]/g"
    return $exit_code
}

# Run tests
if [[ -n "$CLASS_FILTER" ]]; then
    log_lang "py" "Running ${CLASS_FILTER} tests..."
    run_py_test "$CLASS_FILTER"
else
    MAX_JOBS="${MINITEST_PY_JOBS:-$(get_jobs)}"
    PIDS=()
    CLASSES=()
    FAILED=()

    for class_name in "${CLASS_NAMES[@]}"; do
        # Throttle: wait until a slot opens
        while [[ ${#PIDS[@]} -ge $MAX_JOBS ]]; do
            NEW_PIDS=()
            NEW_CLASSES=()
            for i in "${!PIDS[@]}"; do
                if kill -0 "${PIDS[$i]}" 2>/dev/null; then
                    NEW_PIDS+=("${PIDS[$i]}")
                    NEW_CLASSES+=("${CLASSES[$i]}")
                else
                    wait "${PIDS[$i]}" 2>/dev/null || FAILED+=("${CLASSES[$i]}")
                fi
            done
            PIDS=("${NEW_PIDS[@]}")
            CLASSES=("${NEW_CLASSES[@]}")
            [[ ${#PIDS[@]} -ge $MAX_JOBS ]] && sleep 0.1
        done

        run_py_test "$class_name" &
        PIDS+=($!)
        CLASSES+=("$class_name")
    done

    # Wait for remaining
    for i in "${!PIDS[@]}"; do
        wait "${PIDS[$i]}" 2>/dev/null || FAILED+=("${CLASSES[$i]}")
    done

    if [[ ${#FAILED[@]} -gt 0 ]]; then
        log_lang "py" "FAILED: ${FAILED[*]}"
        exit 1
    fi
fi

log_lang "py" "Tests complete (${#CLASS_NAMES[@]} modules)"

# Update viewer if requested
if [[ "$UPDATE_VIEWER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/consolidate.sh"
    consolidate_test_data "$REPO_ROOT"
fi
