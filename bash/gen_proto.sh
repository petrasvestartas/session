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
    echo "[gen_proto] Python -> ${PY_OUT#${REPO_ROOT}/}"
    python -m grpc_tools.protoc \
        --python_out="$PY_OUT" \
        -I "$PROTO_DIR" \
        "$PROTO_DIR"/*.proto
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

    # Protoc resolution order (all Option-C friendly — none build protoc
    # from source):
    #   1. System `protoc` (apt / brew / pre-installed)
    #   2. An existing protoc under session_cpp/build/_deps (if user ran
    #      cmake with SESSION_REGEN_PROTO=ON earlier)
    #   3. `python -m grpc_tools.protoc` — grpcio-tools bundles protoc and
    #      is already a Python dep of session_py; works on CI without any
    #      C++ toolchain
    PROTOC=""
    for candidate in \
        "$(command -v protoc 2>/dev/null)" \
        "${REPO_ROOT}/session_cpp/build/_deps/protobuf-build/Release/protoc.exe" \
        "${REPO_ROOT}/session_cpp/build/_deps/protobuf-build/protoc"; do
        if [[ -n "$candidate" && -x "$candidate" ]]; then
            PROTOC="$candidate"
            break
        fi
    done

    if [[ -z "$PROTOC" ]]; then
        echo "ERROR: no protoc available for C++ codegen." >&2
        echo "Install one of: protobuf-compiler (apt), protobuf (brew)," >&2
        echo "  or let session_cpp's CMake build it once with" >&2
        echo "  SESSION_REGEN_PROTO=ON." >&2
        exit 1
    fi
    "$PROTOC" --cpp_out="$CPP_OUT" --proto_path="$PROTO_DIR" "$PROTO_DIR"/*.proto
fi

# ---- Rust -----------------------------------------------------------------
if $DO_RUST; then
    echo "[gen_proto] Rust   -> session_rust/src/proto/"
    # session_rust/build.rs regenerates into src/proto/ on every build using
    # the protoc-bin-vendored crate. A no-op compile is enough.
    (cd "${REPO_ROOT}/session_rust" && cargo build --lib --quiet)
fi

# ---- Freshness check (CI) -------------------------------------------------
if $DO_CHECK; then
    echo "[gen_proto] running freshness check…"
    for sub in session_py session_cpp session_rust; do
        pushd "${REPO_ROOT}/${sub}" >/dev/null
        if ! git diff --quiet -- '*'; then
            echo "ERROR: ${sub} has stale generated protobuf output." >&2
            echo "Run ./bash/gen_proto.sh and commit the result." >&2
            git --no-pager diff --stat -- '*' >&2
            popd >/dev/null
            exit 1
        fi
        popd >/dev/null
    done
    echo "[gen_proto] all submodules clean."
fi
