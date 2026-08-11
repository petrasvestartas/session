#!/usr/bin/env bash
# Minitest - Run tests for Python, C++, and Rust implementations
# Usage:
#   ./minitest.sh              # Run all languages (fast+dev defaults, C++ builds point_minitest only)
#   ./minitest.sh --py         # Python only
#   ./minitest.sh --cpp        # C++ only
#   ./minitest.sh --rust       # Rust only
#   ./minitest.sh --full       # Force reinstall + release build
#   ./minitest.sh --release    # Rust release build (optimized)
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
FAST_MODE=true
START_WEB=true
KILL_SERVER=false
DEV_MODE=true

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
        --full)
            FAST_MODE=false
            DEV_MODE=false
            ;;
        --release)
            DEV_MODE=false
            ;;
        --dev)
            DEV_MODE=true
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
FULL_ARG=""
[[ "$FAST_MODE" == "false" ]] && FULL_ARG="--full"
DEV_ARG=""
if [[ "$DEV_MODE" == "true" ]]; then
    DEV_ARG="--dev"
else
    DEV_ARG="--release"
fi

# Regenerate committed protobuf bindings that are stale.
#
# Option C: Python (_pb2.py) and C++ (.pb.cc/.pb.h) generated files are
# committed alongside the .proto sources. This helper refreshes them when
# any .proto is newer than its corresponding output. Rust auto-regens via
# session_rust/build.rs on every `cargo build` so it is NOT handled here.
regenerate_protos() {
    local proto_dir="${REPO_ROOT}/session_proto"
    [[ -d "$proto_dir" ]] || return 0

    # Skip on CI: git checkout does not preserve mtimes, so -nt comparisons
    # trip false-positives. CI has a dedicated ./bash/gen_proto.sh --check
    # step that uses git diff instead of mtime.
    if [[ -n "${CI:-}" ]]; then
        return 0
    fi

    # ---- Python ----
    local py_out="${REPO_ROOT}/session_py/src/session_py/proto"
    local py_stale=false
    for proto_file in "${proto_dir}"/*.proto; do
        [[ -f "$proto_file" ]] || continue
        local base=$(basename "$proto_file" .proto)
        if [[ ! -f "${py_out}/${base}_pb2.py" ]] || [[ "$proto_file" -nt "${py_out}/${base}_pb2.py" ]]; then
            py_stale=true
            break
        fi
    done

    # ---- C++ ----
    local cpp_out="${REPO_ROOT}/session_cpp/generated"
    local cpp_stale=false
    for proto_file in "${proto_dir}"/*.proto; do
        [[ -f "$proto_file" ]] || continue
        local base=$(basename "$proto_file" .proto)
        if [[ ! -f "${cpp_out}/${base}.pb.cc" ]] || [[ "$proto_file" -nt "${cpp_out}/${base}.pb.cc" ]]; then
            cpp_stale=true
            break
        fi
    done

    if [[ "$py_stale" == "false" && "$cpp_stale" == "false" ]]; then
        log "Protos up-to-date, skipping regeneration"
        return 0
    fi

    local args=()
    [[ "$py_stale" == "true" ]] && args+=(--py)
    [[ "$cpp_stale" == "true" ]] && args+=(--cpp)
    log "Regenerating protobuf bindings: ${args[*]}"
    "${SCRIPT_DIR}/gen_proto.sh" "${args[@]}"
}

# Back-compat alias for earlier callers.
regenerate_python_protos() { regenerate_protos; }

if [[ "$RUN_PYTHON" == "true" || "$RUN_CPP" == "true" ]]; then
    regenerate_protos
fi

# Count enabled languages
LANG_COUNT=0
[[ "$RUN_PYTHON" == "true" ]] && LANG_COUNT=$((LANG_COUNT + 1))
[[ "$RUN_CPP" == "true" ]] && LANG_COUNT=$((LANG_COUNT + 1))
[[ "$RUN_RUST" == "true" ]] && LANG_COUNT=$((LANG_COUNT + 1))

# Run language tests — parallel when 2+ languages, sequential for single
if [[ $LANG_COUNT -ge 2 ]]; then
    TMPDIR_LANG=$(mktemp -d)
    trap "rm -rf $TMPDIR_LANG" EXIT
    LANG_PIDS=()
    LANG_NAMES=()

    if [[ "$RUN_PYTHON" == "true" ]]; then
        "${SCRIPT_DIR}/test_py.sh" $FAST_ARG $FULL_ARG --no-viewer >"${TMPDIR_LANG}/py.log" 2>&1 &
        LANG_PIDS+=($!)
        LANG_NAMES+=("Python")
    fi
    if [[ "$RUN_CPP" == "true" ]]; then
        "${SCRIPT_DIR}/test_cpp.sh" $FAST_ARG --no-viewer >"${TMPDIR_LANG}/cpp.log" 2>&1 &
        LANG_PIDS+=($!)
        LANG_NAMES+=("C++")
    fi
    if [[ "$RUN_RUST" == "true" ]]; then
        "${SCRIPT_DIR}/test_rust.sh" $DEV_ARG --no-viewer >"${TMPDIR_LANG}/rust.log" 2>&1 &
        LANG_PIDS+=($!)
        LANG_NAMES+=("Rust")
    fi

    log "Running ${LANG_NAMES[*]} in parallel..."
    LANG_FAILED=()
    for i in "${!LANG_PIDS[@]}"; do
        if ! wait "${LANG_PIDS[$i]}"; then
            LANG_FAILED+=("${LANG_NAMES[$i]}")
        fi
    done

    # Print buffered output sequentially
    for f in "${TMPDIR_LANG}"/*.log; do
        [[ -f "$f" ]] && cat "$f"
    done

    if [[ ${#LANG_FAILED[@]} -gt 0 ]]; then
        # Extract and display failure lines from logs
        echo ""
        log "============ FAILURES ============"
        for f in "${TMPDIR_LANG}"/*.log; do
            [[ -f "$f" ]] && grep -E "FAIL |FAILURES:|failed$" "$f" 2>/dev/null || true
        done
        log "=================================="
        log "FAILED: ${LANG_FAILED[*]}"
        exit 1
    fi
else
    if [[ "$RUN_PYTHON" == "true" ]]; then
        log "=== Python Tests ==="
        "${SCRIPT_DIR}/test_py.sh" $FAST_ARG $FULL_ARG --no-viewer
    fi
    if [[ "$RUN_CPP" == "true" ]]; then
        log "=== C++ Tests ==="
        "${SCRIPT_DIR}/test_cpp.sh" $FAST_ARG --no-viewer
    fi
    if [[ "$RUN_RUST" == "true" ]]; then
        log "=== Rust Tests ==="
        "${SCRIPT_DIR}/test_rust.sh" $DEV_ARG --no-viewer
    fi
fi

# Consolidate ALL JSON (reads existing files from ALL languages)
log "=== Consolidating Test Data ==="
source "${SCRIPT_DIR}/lib/consolidate.sh"
consolidate_test_data "$REPO_ROOT"

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
