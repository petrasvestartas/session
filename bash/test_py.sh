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
    if has_uv; then
        uv pip install --python "$PYTHON" -e "${REPO_ROOT}/session_py" pytest
    else
        "$PYTHON" -m pip install -e "${REPO_ROOT}/session_py" pytest
    fi
}

ensure_python_env

# Run tests
if [[ -n "$CLASS_FILTER" ]]; then
    log_lang "py" "Running ${CLASS_FILTER} tests..."
    "$PYTHON" -m "session_py.${CLASS_FILTER}_test"
else
    for class_name in "${CLASS_NAMES[@]}"; do
        log_lang "py" "Running ${class_name} tests..."
        "$PYTHON" -m "session_py.${class_name}_test"
    done
fi

log_lang "py" "Tests complete"

# Update viewer if requested
if [[ "$UPDATE_VIEWER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/consolidate.sh"
    consolidate_test_data "$REPO_ROOT"
fi
