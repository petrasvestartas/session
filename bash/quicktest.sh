#!/usr/bin/env bash
# Quick single-class test for rapid development
# Usage:
#   ./quicktest.sh point          # Test Point class in all languages
#   ./quicktest.sh point --py     # Test Point class in Python only
#   ./quicktest.sh point --cpp    # Test Point class in C++ only
#   ./quicktest.sh point --rust   # Test Point class in Rust only

CLASS_NAME="${1:-point}"
shift

LANG_PY=true
LANG_CPP=true
LANG_RUST=true

for arg in "$@"; do
  case $arg in
    --py|--python) LANG_CPP=false; LANG_RUST=false ;;
    --cpp|--c++) LANG_PY=false; LANG_RUST=false ;;
    --rust|--rs) LANG_PY=false; LANG_CPP=false ;;
  esac
done

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="${SCRIPT_DIR}/.."

# Detect Windows
IS_WINDOWS=false
[[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ -n "$MSYSTEM" ]] && IS_WINDOWS=true

if [ "$IS_WINDOWS" = true ]; then
  PYTHON="${REPO_ROOT}/uvsession/Scripts/python.exe"
else
  PYTHON="${REPO_ROOT}/uvsession/bin/python"
fi

echo "=== Quick Test: ${CLASS_NAME} ==="

# Python (instant)
if [ "$LANG_PY" = true ]; then
  echo "[py] Running ${CLASS_NAME}_test..."
  "$PYTHON" -m "session_py.${CLASS_NAME}_test" 2>&1 | tail -5
fi

# Rust (fast incremental)
if [ "$LANG_RUST" = true ]; then
  echo "[rust] Running ${CLASS_NAME} minitest..."
  cd "${REPO_ROOT}/session_rust"
  cargo run --release --features protobuf --bin minitest 2>&1 | grep -i "${CLASS_NAME}\|error\|passed\|failed" | head -20
  cd "${REPO_ROOT}"
fi

# C++ (slower, skip if not needed)
if [ "$LANG_CPP" = true ]; then
  echo "[cpp] Building and running ${CLASS_NAME} minitest..."
  cd "${REPO_ROOT}/session_cpp"
  cmake --build build --config Release --parallel 4 2>&1 | tail -3
  if [ "$IS_WINDOWS" = true ]; then
    ./build/Release/point_minitest.exe 2>&1 | grep -i "${CLASS_NAME}\|error\|passed\|failed" | head -20
  else
    ./build/point_minitest 2>&1 | grep -i "${CLASS_NAME}\|error\|passed\|failed" | head -20
  fi
  cd "${REPO_ROOT}"
fi

echo "=== Done ==="
