#!/usr/bin/env bash
# Quick single-class test — no proto regen, no API index, no web server
# Usage:
#   ./bash/quicktest.sh point --py       # Python only (~1s)
#   ./bash/quicktest.sh mesh --rust      # Rust only
#   ./bash/quicktest.sh point            # All languages

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")

# Parse args
CLASS_FILTER=""
RUN_PYTHON=true
RUN_CPP=true
RUN_RUST=true

for arg in "$@"; do
    case $arg in
        --py|--python) RUN_CPP=false; RUN_RUST=false ;;
        --cpp|--c++) RUN_PYTHON=false; RUN_RUST=false ;;
        --rust|--rs) RUN_PYTHON=false; RUN_CPP=false ;;
        -*) ;;
        *) CLASS_FILTER="$arg" ;;
    esac
done

if [[ -z "$CLASS_FILTER" ]]; then
    echo "Usage: quicktest.sh <class> [--py|--cpp|--rust]"
    echo "Classes: ${CLASS_NAMES[*]}"
    exit 1
fi

# Validate class name
VALID=false
for name in "${CLASS_NAMES[@]}"; do
    [[ "$name" == "$CLASS_FILTER" ]] && VALID=true && break
done
if [[ "$VALID" == "false" ]]; then
    echo "Unknown class: $CLASS_FILTER"
    echo "Valid: ${CLASS_NAMES[*]}"
    exit 1
fi

if [[ "$RUN_PYTHON" == "true" ]]; then
    PYTHON=$(get_python_path "$REPO_ROOT")
    log_lang "py" "Running ${CLASS_FILTER} tests..."
    "$PYTHON" -m "session_py.${CLASS_FILTER}_test"
fi

if [[ "$RUN_CPP" == "true" ]]; then
    "${SCRIPT_DIR}/test_cpp.sh" --no-viewer
fi

if [[ "$RUN_RUST" == "true" ]]; then
    "${SCRIPT_DIR}/test_rust.sh" --no-viewer
fi

# Consolidate
source "${SCRIPT_DIR}/lib/consolidate.sh"
consolidate_test_data "$REPO_ROOT"

log "Done"
