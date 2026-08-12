#!/usr/bin/env bash
# Run Python minitest only - does NOT touch other languages' JSON
# Usage:
#   ./test_py.sh              # Run all Python tests (fast by default)
#   ./test_py.sh point        # Run only Point tests
#   ./test_py.sh --fast       # Skip pip install if already installed
#   ./test_py.sh --full       # Force pip reinstall
#   ./test_py.sh --no-viewer  # Don't update testData.js

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
FAST_MODE=true
UPDATE_VIEWER=true
CLASS_FILTER=""

# Parse args
for arg in "$@"; do
    case $arg in
        --fast|-f) FAST_MODE=true ;;
        --full) FAST_MODE=false ;;
        --no-viewer) UPDATE_VIEWER=false ;;
        -*) ;; # ignore unknown flags
        *) CLASS_FILTER="$arg" ;;
    esac
done

PYTHON=$(get_python_path "$REPO_ROOT")

# Tests write file-roundtrip artifacts to CWD-relative "serialization/..." paths
# (same convention as the C++/Rust runners, which cd into their language dirs).
# Anchor at the repo root so the script works no matter where it is invoked from.
cd "$REPO_ROOT"

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

    # Fast mode: skip install if source unchanged and module importable
    if [[ "$FAST_MODE" == "true" ]]; then
        local hash_file="${REPO_ROOT}/.py_install_hash"
        local src_hash=$(find "${REPO_ROOT}/session_py/src" -name '*.py' 2>/dev/null | sort | xargs md5sum 2>/dev/null | md5sum | cut -d' ' -f1)
        if [[ -f "$hash_file" && "$(cat "$hash_file")" == "$src_hash" ]]; then
            if "$PYTHON" -c "import session_py" 2>/dev/null; then
                return 0
            fi
        fi
    fi

    log_lang "py" "Installing session_py..."
    local py_path=$(to_windows_path "${REPO_ROOT}/session_py")
    if has_uv; then
        uv pip install --python "$PYTHON" -e "$py_path" pytest
    else
        "$PYTHON" -m pip install -e "$py_path" pytest
    fi

    # Cache source hash after successful install
    local src_hash=$(find "${REPO_ROOT}/session_py/src" -name '*.py' 2>/dev/null | sort | xargs md5sum 2>/dev/null | md5sum | cut -d' ' -f1)
    echo "$src_hash" > "${REPO_ROOT}/.py_install_hash"
}

ensure_python_env

# Run tests — single process batch (avoids 43 interpreter startups)
if [[ -n "$CLASS_FILTER" ]]; then
    log_lang "py" "Running ${CLASS_FILTER} tests..."
    output=$("$PYTHON" -m "session_py.${CLASS_FILTER}_test" 2>&1)
    exit_code=$?
    printf "%s\n" "$output" | sed "s/\[py-minitest\]/[py-${CLASS_FILTER}]/g"
    [[ $exit_code -ne 0 ]] && exit $exit_code
else
    CLASSES_STR="${CLASS_NAMES[*]}"
    "$PYTHON" -c "
import sys, importlib, io
from contextlib import redirect_stdout, redirect_stderr
import session_py.mini_test as mt

classes = '''${CLASSES_STR}'''.split()
failed = []
for cls in classes:
    mt._REGISTERED_TESTS.clear()
    mod_name = f'session_py.{cls}_test'
    try:
        if mod_name in sys.modules:
            del sys.modules[mod_name]
        buf = io.StringIO()
        ebuf = io.StringIO()
        with redirect_stdout(buf), redirect_stderr(ebuf):
            try:
                importlib.import_module(mod_name)
                mt.run_all(language='python')
            except SystemExit as e:
                if e.code:
                    failed.append(cls)
        out = buf.getvalue().replace('[py-minitest]', f'[py-{cls}]')
        err = ebuf.getvalue().replace('[py-minitest]', f'[py-{cls}]')
        if out.strip():
            print(out, end='')
        if err.strip():
            print(err, end='', file=sys.stderr)
    except ModuleNotFoundError as e:
        # class not implemented in this language -- the class summary below flags it
        if e.name != mod_name:
            failed.append(cls)
            print(f'[py-{cls}] FAILED: {e}', file=sys.stderr)
    except Exception as e:
        failed.append(cls)
        print(f'[py-{cls}] FAILED: {e}', file=sys.stderr)
if failed:
    print(f'[py] FAILED: {\" \".join(failed)}', file=sys.stderr)
    sys.exit(1)
"
fi

# Same honesty check the other languages get: iterate CLASS_NAMES, flag stale json and
# classes that produced none, and print a reconciled total (set MINITEST_LENIENT=1 to
# downgrade the anomalies to warnings).
if [[ -z "$CLASS_FILTER" ]]; then
    print_class_summary "py" "${REPO_ROOT}/session_tests/session_py" "$PYTHON" || {
        log_lang "py" "class-summary reported stale/missing/failing classes (see above)"
        exit 1
    }
fi

log_lang "py" "Tests complete (${#CLASS_NAMES[@]} modules)"

# Update viewer if requested
if [[ "$UPDATE_VIEWER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/consolidate.sh"
    consolidate_test_data "$REPO_ROOT"
fi
