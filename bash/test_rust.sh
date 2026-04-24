#!/usr/bin/env bash
# Run Rust minitest only - does NOT touch other languages' JSON
# Usage:
#   ./test_rust.sh              # Build and run all Rust tests
#   ./test_rust.sh --no-viewer  # Don't update testData.js

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/common.sh"

REPO_ROOT=$(resolve_repo_root "${BASH_SOURCE[0]}")
RUST_DIR="${REPO_ROOT}/session_rust"
UPDATE_VIEWER=true
DEV_MODE=true

# Parse args
for arg in "$@"; do
    case $arg in
        --fast|-f) ;; # ignored for Rust, kept for consistency
        --no-viewer) UPDATE_VIEWER=false ;;
        --dev) DEV_MODE=true ;;
        --release) DEV_MODE=false ;;
    esac
done

if [[ ! -d "$RUST_DIR" ]]; then
    log_lang "rust" "ERROR: session_rust not found at ${RUST_DIR}"
    exit 1
fi

# Find protoc from various locations
find_protoc() {
    # System protoc
    if command -v protoc >/dev/null 2>&1; then
        echo "protoc"
        return
    fi
    # From C++ build
    local cpp_protoc="${REPO_ROOT}/session_cpp/build/protobuf_external-prefix/src/protobuf_external-build/protoc"
    if [[ -x "$cpp_protoc" ]]; then
        echo "$cpp_protoc"
        return
    fi
    # Windows C++ build
    local cpp_protoc_win="${REPO_ROOT}/session_cpp/build/protobuf_external-prefix/src/protobuf_external-build/Release/protoc.exe"
    if [[ -x "$cpp_protoc_win" ]]; then
        echo "$cpp_protoc_win"
        return
    fi
    # Rust build cached (downloaded by build.rs)
    local rust_protoc=$(ls "${REPO_ROOT}"/session_rust/target/release/build/*/out/protoc/bin/protoc.exe 2>/dev/null | head -1)
    if [[ -z "$rust_protoc" ]]; then
        rust_protoc=$(ls "${REPO_ROOT}"/session_rust/target/release/build/*/out/protoc/bin/protoc 2>/dev/null | head -1)
    fi
    if [[ -n "$rust_protoc" && -f "$rust_protoc" ]]; then
        echo "$rust_protoc"
        return
    fi
    # Cargo vendored
    local cargo_protoc=$(find ~/.cargo/registry/src -name "protoc" -path "*linux-x86_64*" 2>/dev/null | head -1)
    if [[ -n "$cargo_protoc" && -x "$cargo_protoc" ]]; then
        echo "$cargo_protoc"
        return
    fi
}

PROTOC_PATH=$(find_protoc)
if [[ -n "$PROTOC_PATH" ]]; then
    export PROTOC="$PROTOC_PATH"
    log_lang "rust" "Using protoc: $PROTOC"
else
    log_lang "rust" "Warning: protoc not found, build may fail"
fi

cd "$RUST_DIR"
JOBS=$(get_jobs)

if [[ "$DEV_MODE" == "true" ]]; then
    log_lang "rust" "Building and running minitest (dev mode)..."
    cargo run --bin minitest -j "$JOBS"
else
    log_lang "rust" "Building and running minitest (release)..."
    cargo run --release --bin minitest -j "$JOBS"
fi

if [[ $? -ne 0 ]]; then
    log_lang "rust" "Minitest failed"
    exit 1
fi

cd "$REPO_ROOT"
PYTHON=$(get_python_path "$REPO_ROOT")
print_class_summary "rust" "${REPO_ROOT}/session_tests/session_rust" "$PYTHON"
log_lang "rust" "Tests complete"

# Update viewer if requested
if [[ "$UPDATE_VIEWER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/consolidate.sh"
    consolidate_test_data "$REPO_ROOT"
fi
