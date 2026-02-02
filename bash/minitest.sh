#!/usr/bin/env bash
# Minitest - Run tests for Python, C++, and Rust implementations
# Usage:
#   ./minitest.sh              # Run all languages
#   ./minitest.sh --py         # Python only
#   ./minitest.sh --cpp        # C++ only
#   ./minitest.sh --rust       # Rust only
#   ./minitest.sh --fast       # Skip dependency installs
#   ./minitest.sh --no-web     # Skip Vue server
#   ./minitest.sh --kill       # Stop dev server only

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")

# Default: run all languages
RUN_PYTHON=true
RUN_CPP=true
RUN_RUST=true
FAST_MODE=false
START_WEB=true
KILL_SERVER=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --py|--python)
            RUN_CPP=false
            RUN_RUST=false
            ;;
        --cpp|--c++)
            RUN_PYTHON=false
            RUN_RUST=false
            ;;
        --rust|--rs)
            RUN_PYTHON=false
            RUN_CPP=false
            ;;
        --fast|-f)
            FAST_MODE=true
            ;;
        --no-web)
            START_WEB=false
            ;;
        --kill|-k)
            KILL_SERVER=true
            ;;
    esac
done

# Handle --kill immediately (no other actions)
if [[ "$KILL_SERVER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/server.sh"
    stop_server
    exit 0
fi

# Build flags for sub-scripts
FAST_ARG=""
[[ "$FAST_MODE" == "true" ]] && FAST_ARG="--fast"

# Regenerate Python protos (skip in fast mode)
regenerate_python_protos() {
    local proto_dir="${REPO_ROOT}/session_proto"
    local py_proto_out="${REPO_ROOT}/session_py/src/session_py/proto"

    if [[ ! -d "$proto_dir" ]]; then
        log "Warning: ${proto_dir} not found, skipping Python protobuf regeneration"
        return 0
    fi

    mkdir -p "$py_proto_out"

    log "Regenerating Python protobuf bindings..."
    for proto_file in "${proto_dir}"/*.proto; do
        if [[ -f "$proto_file" ]]; then
            python -m grpc_tools.protoc --python_out="$py_proto_out" -I "$proto_dir" "$proto_file" 2>/dev/null || true
        fi
    done

    # Fix imports for relative imports
    for pb_file in "${py_proto_out}"/*_pb2.py; do
        if [[ -f "$pb_file" ]]; then
            if [[ "$OSTYPE" == "darwin"* ]]; then
                sed -i '' 's/^import \([a-z_]*_pb2\) as/from . import \1 as/g' "$pb_file"
            else
                sed -i 's/^import \([a-z_]*_pb2\) as/from . import \1 as/g' "$pb_file"
            fi
        fi
    done
}

if [[ "$FAST_MODE" == "false" && "$RUN_PYTHON" == "true" ]]; then
    regenerate_python_protos
fi

# Run language tests (each script handles its own JSON, no cleanup needed)
if [[ "$RUN_PYTHON" == "true" ]]; then
    log "=== Python Tests ==="
    "${SCRIPT_DIR}/test_py.sh" $FAST_ARG --no-viewer
fi

if [[ "$RUN_CPP" == "true" ]]; then
    log "=== C++ Tests ==="
    "${SCRIPT_DIR}/test_cpp.sh" $FAST_ARG --no-viewer
fi

if [[ "$RUN_RUST" == "true" ]]; then
    log "=== Rust Tests ==="
    "${SCRIPT_DIR}/test_rust.sh" --no-viewer
fi

# Consolidate ALL JSON (reads existing files from ALL languages)
log "=== Consolidating Test Data ==="
source "${SCRIPT_DIR}/lib/consolidate.sh"
consolidate_test_data "$REPO_ROOT"

# Regenerate browser API index for Vue chatbot
log "=== Regenerating Browser API Index ==="
cd "$REPO_ROOT"
python -m session_mcp.generate_browser_index || log "Warning: Failed to generate browser index"

# Check for CI environment
if [[ -n "${CI:-}" || -n "${GITHUB_ACTIONS:-}" ]]; then
    log "CI environment detected - building dist for artifact upload"
    cd "${REPO_ROOT}/session_tests"
    npm run build
    log "Done"
    exit 0
fi

# Handle web server
if [[ "$START_WEB" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/server.sh"
    ensure_node_deps "$REPO_ROOT" "$FAST_MODE"
    start_server "$REPO_ROOT"
fi

log "Done"
log "Re-run to update (browser auto-refreshes via Vite HMR)"
