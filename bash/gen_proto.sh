#!/usr/bin/env bash
# Regenerate committed protobuf bindings for Python, C++, and Rust.
#
# Option C: the generated artefacts live inside each language's submodule:
#   session_py/src/session_py/proto/*_pb2.py
#   session_cpp/generated/*.pb.{cc,h}
#   session_rust/src/proto/session_proto.rs
#
# Run this after editing any ../session_proto/*.proto file, review the diff,
# and commit the updated generated files together with the .proto change.
#
# Usage:
#   ./bash/gen_proto.sh           # regenerate all three languages
#   ./bash/gen_proto.sh --py      # Python only
#   ./bash/gen_proto.sh --cpp     # C++ only
#   ./bash/gen_proto.sh --rust    # Rust only
#   ./bash/gen_proto.sh --check   # regenerate + fail if git diff is non-empty
#                                 # (used by CI to catch stale committed output)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
PROTO_DIR="${REPO_ROOT}/session_proto"

# Prefer the uvsession venv's python when it exists — that is where
# minitest.sh installs grpcio-tools (needed for the Python codegen path).
# Fall back to whatever `python` is on PATH.
pick_python() {
    local venv_py=""
    if [[ -x "${REPO_ROOT}/uvsession/Scripts/python.exe" ]]; then
        venv_py="${REPO_ROOT}/uvsession/Scripts/python.exe"
    elif [[ -x "${REPO_ROOT}/uvsession/bin/python" ]]; then
        venv_py="${REPO_ROOT}/uvsession/bin/python"
    fi
    if [[ -n "$venv_py" ]] && "$venv_py" -c "import grpc_tools.protoc" 2>/dev/null; then
        echo "$venv_py"
        return
    fi
    if command -v python >/dev/null 2>&1 && python -c "import grpc_tools.protoc" 2>/dev/null; then
        echo "python"
        return
    fi
    echo ""
}
PYTHON_BIN="$(pick_python)"

# The C++ protoc version is PINNED by session_cpp/CMakeLists.txt and must match the
# committed gencode exactly — see the C++ section below for why a mismatch is fatal.
PROTOC_VERSION="$(sed -n 's/^[[:space:]]*set(PROTOC_VERSION[[:space:]]*"\([0-9.]*\)").*/\1/p' \
    "${REPO_ROOT}/session_cpp/CMakeLists.txt" 2>/dev/null | head -1)"
PROTOC_CACHE="${REPO_ROOT}/session_cpp/.protoc/${PROTOC_VERSION}"
SESSION_PROTOC="${SESSION_PROTOC:-}"   # explicit override, checked first

# Prints just the version number, e.g. "33.6" from "libprotoc 33.6".
protoc_version_of() {
    "$1" --version 2>/dev/null | awk '{print $2}'
}

# Fetches the pinned protoc release into a gitignored cache next to session_cpp.
download_pinned_protoc() {
    local os arch asset url tmp
    case "$(uname -s)" in
        Linux)  os="linux" ;;
        Darwin) os="osx" ;;
        *)      os="win" ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="aarch_64" ;;
        *) arch="x86_64" ;;
    esac
    if [[ "$os" == "win" ]]; then
        asset="protoc-${PROTOC_VERSION}-win64.zip"
    else
        asset="protoc-${PROTOC_VERSION}-${os}-${arch}.zip"
    fi
    url="https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/${asset}"

    command -v curl >/dev/null 2>&1 || return 1
    command -v unzip >/dev/null 2>&1 || return 1

    echo "[gen_proto] caching protoc ${PROTOC_VERSION} -> ${PROTOC_CACHE#${REPO_ROOT}/}"
    tmp="$(mktemp -d)"
    if ! curl -fsSL "$url" -o "${tmp}/protoc.zip"; then
        echo "[gen_proto] download failed: ${url}" >&2
        rm -rf "$tmp"
        return 1
    fi
    mkdir -p "$PROTOC_CACHE"
    unzip -q -o "${tmp}/protoc.zip" -d "$PROTOC_CACHE"
    rm -rf "$tmp"
    chmod +x "${PROTOC_CACHE}/bin/protoc" 2>/dev/null || true
    [[ "$(protoc_version_of "${PROTOC_CACHE}/bin/protoc")" == "$PROTOC_VERSION" ]]
}

DO_PY=true
DO_CPP=true
DO_RUST=true
DO_CHECK=false

for arg in "$@"; do
    case "$arg" in
        --py|--python)  DO_CPP=false; DO_RUST=false ;;
        --cpp|--c++)    DO_PY=false;  DO_RUST=false ;;
        --rust|--rs)    DO_PY=false;  DO_CPP=false ;;
        --check)        DO_CHECK=true ;;
        -h|--help)
            sed -n '2,20p' "$0"
            exit 0
            ;;
        *) echo "unknown arg: $arg" >&2; exit 2 ;;
    esac
done

if [[ ! -d "$PROTO_DIR" ]]; then
    echo "ERROR: session_proto/ not found at $PROTO_DIR" >&2
    exit 1
fi

# ---- Python ---------------------------------------------------------------
if $DO_PY; then
    PY_OUT="${REPO_ROOT}/session_py/src/session_py/proto"
    mkdir -p "$PY_OUT"
    if [[ -z "$PYTHON_BIN" ]]; then
        echo "ERROR: no Python with grpcio-tools found. Install session_py" >&2
        echo "  (which pulls in grpcio-tools) or pip install grpcio-tools." >&2
        exit 1
    fi
    echo "[gen_proto] Python -> ${PY_OUT#${REPO_ROOT}/}  (using ${PYTHON_BIN})"
    "$PYTHON_BIN" -m grpc_tools.protoc \
        --python_out="$PY_OUT" \
        -I "$PROTO_DIR" \
        $(printf '%s\n' "$PROTO_DIR"/*.proto | LC_ALL=C sort | tr '\n' ' ')
    # Rewrite cross-module imports to be package-relative so _pb2 modules
    # can import each other without polluting sys.path.
    for pb in "$PY_OUT"/*_pb2.py; do
        case "$(uname)" in
            Darwin*) sed -i '' 's/^import \([a-z_]*_pb2\) as/from . import \1 as/g' "$pb" ;;
            *)       sed -i    's/^import \([a-z_]*_pb2\) as/from . import \1 as/g' "$pb" ;;
        esac
    done
fi

# ---- C++ ------------------------------------------------------------------
if $DO_CPP; then
    CPP_OUT="${REPO_ROOT}/session_cpp/generated"
    mkdir -p "$CPP_OUT"
    echo "[gen_proto] C++    -> ${CPP_OUT#${REPO_ROOT}/}"

    # The generated C++ carries an EXACT runtime assertion
    # (`#if PROTOBUF_VERSION != 6033006`), so it must be produced by exactly the
    # protoc that session_cpp/CMakeLists.txt fetches — nothing else will link.
    # Any other protoc on PATH (Ubuntu's snap ships 3.14.0) silently rewrites all
    # 27 .pb.{h,cc} with incompatible gencode and breaks the build, so every
    # candidate is version-checked and a mismatch is fatal, never a fallback.
    if [[ -z "$PROTOC_VERSION" ]]; then
        echo "ERROR: could not read PROTOC_VERSION from session_cpp/CMakeLists.txt" >&2
        exit 1
    fi

    PROTOC=""
    for candidate in \
        "$SESSION_PROTOC" \
        "${PROTOC_CACHE}/bin/protoc" \
        "$(command -v protoc 2>/dev/null)" \
        "${REPO_ROOT}/session_cpp/build/_deps/protobuf-build/Release/protoc.exe" \
        "${REPO_ROOT}/session_cpp/build/_deps/protobuf-build/protoc"; do
        [[ -n "$candidate" && -x "$candidate" ]] || continue
        found_version="$(protoc_version_of "$candidate")"
        if [[ "$found_version" == "$PROTOC_VERSION" ]]; then
            PROTOC="$candidate"
            break
        fi
        echo "[gen_proto] skipping ${candidate} (libprotoc ${found_version:-unknown}, need ${PROTOC_VERSION})"
    done

    if [[ -z "$PROTOC" ]]; then
        download_pinned_protoc && PROTOC="${PROTOC_CACHE}/bin/protoc"
    fi

    if [[ -z "$PROTOC" ]]; then
        echo "ERROR: no protoc ${PROTOC_VERSION} available for C++ codegen." >&2
        echo "  The committed bindings only link against exactly this version." >&2
        echo "  Fix by either:" >&2
        echo "    - re-running with network access so this script can cache it, or" >&2
        echo "    - SESSION_PROTOC=/path/to/protoc-${PROTOC_VERSION} ./bash/gen_proto.sh --cpp" >&2
        echo "  Do NOT substitute a different protoc: mismatched gencode does not compile." >&2
        exit 1
    fi
    echo "[gen_proto] using protoc ${PROTOC_VERSION} at ${PROTOC}"
    "$PROTOC" --cpp_out="$CPP_OUT" --proto_path="$PROTO_DIR" $(printf '%s\n' "$PROTO_DIR"/*.proto | LC_ALL=C sort | tr '\n' ' ')
fi

# ---- Rust -----------------------------------------------------------------
if $DO_RUST; then
    echo "[gen_proto] Rust   -> session_rust/src/proto/"
    # session_rust/build.rs regenerates into src/proto/ on every build using
    # the protoc-bin-vendored crate. A no-op compile is enough.
    (cd "${REPO_ROOT}/session_rust" && cargo build --lib --quiet)
fi

# ---- Freshness check (CI) -------------------------------------------------
# Only inspect the generated proto output paths — minitest also dirties
# other files (serialization/test_*.json, etc.) which are unrelated.
if $DO_CHECK; then
    echo "[gen_proto] running freshness check…"
    declare -A CHECK_PATHS=(
        [session_py]="src/session_py/proto"
        [session_cpp]="generated"
        [session_rust]="src/proto"
    )
    bad=0
    for sub in "${!CHECK_PATHS[@]}"; do
        local_path="${CHECK_PATHS[$sub]}"
        pushd "${REPO_ROOT}/${sub}" >/dev/null
        if ! git diff --quiet -- "$local_path"; then
            echo "ERROR: ${sub}/${local_path} has stale generated output:" >&2
            git --no-pager diff --stat -- "$local_path" >&2
            bad=1
        fi
        popd >/dev/null
    done
    if [[ $bad -ne 0 ]]; then
        echo "" >&2
        echo "Run ./bash/gen_proto.sh locally and commit the refreshed files." >&2
        exit 1
    fi
    echo "[gen_proto] all generated proto paths are up-to-date."
fi
