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

# protoc is handled by session_rust/build.rs via the protoc-bin-vendored crate;
# no external PROTOC discovery needed anymore.

cd "$RUST_DIR"
JOBS=$(get_jobs)

# The PDF importer is a Rust-only extra behind --features pdf. On by default locally so the Pdf
# class shows up in the viewer. In CI it is driven by SESSION_PDF, which the workflow sets on
# every OS: mupdf-sys builds MuPDF with make (macOS) or MSBuild (Windows MSVC x64) and bindgen
# finds libclang on the stock runners (Ubuntu installs libclang-dev in the workflow). Where it
# is off, the Pdf class is SKIPped.
PDF_FEATURE=()
if [[ "${SESSION_PDF:-}" == "1" ]] || [[ -z "${CI:-}" && -z "${GITHUB_ACTIONS:-}" ]]; then
    PDF_FEATURE=(--features pdf)

    # bindgen needs the host C standard headers on some Linux setups where libclang
    # does not discover the GCC include tree automatically.
    if [[ -z "${BINDGEN_EXTRA_CLANG_ARGS:-}" ]] && [[ "$(detect_platform)" != "windows" ]]; then
        GCC_INCLUDE_DIR="$(cc -print-file-name=include 2>/dev/null || true)"
        if [[ -n "$GCC_INCLUDE_DIR" && -f "${GCC_INCLUDE_DIR}/stddef.h" ]]; then
            export BINDGEN_EXTRA_CLANG_ARGS="-I${GCC_INCLUDE_DIR}"
        fi
    fi
fi

if [[ "$DEV_MODE" == "true" ]]; then
    log_lang "rust" "Building and running minitest (dev mode)..."
    cargo run --bin minitest "${PDF_FEATURE[@]}" -j "$JOBS"
else
    log_lang "rust" "Building and running minitest (release)..."
    cargo run --release --bin minitest "${PDF_FEATURE[@]}" -j "$JOBS"
fi

if [[ $? -ne 0 ]]; then
    log_lang "rust" "Minitest failed"
    exit 1
fi

cd "$REPO_ROOT"
PYTHON=$(get_python_path "$REPO_ROOT")
print_class_summary "rust" "${REPO_ROOT}/session_tests/session_rust" "$PYTHON" || {
    log_lang "rust" "class-summary reported stale/missing/failing classes (see above)"
    exit 1
}
log_lang "rust" "Tests complete"

# Update viewer if requested
if [[ "$UPDATE_VIEWER" == "true" ]]; then
    source "${SCRIPT_DIR}/lib/consolidate.sh"
    consolidate_test_data "$REPO_ROOT"
fi
