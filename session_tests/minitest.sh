#!/usr/bin/env bash
# Run mini tests for both Python and C++ Point implementations.
# Usage:
#   bash minitest.sh
# or
#   ./minitest.sh

# Resolve repository root as the parent directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="${SCRIPT_DIR}/.."

echo "[mini] Running Python Point mini tests..."
"${REPO_ROOT}/uvsession/bin/python" -m session_py.point_test

echo "[mini] Running Python Color mini tests..."
"${REPO_ROOT}/uvsession/bin/python" -m session_py.color_test

echo "[mini] Building C++ project (session_cpp) including tests (no test run)..."
CPP_DIR="${REPO_ROOT}/session_cpp"

if [ -d "${CPP_DIR}" ]; then
  ( cd "${CPP_DIR}" && cmake -S . -B build >/dev/null && cmake --build build --parallel )
  BUILD_STATUS=$?
  if [ $BUILD_STATUS -ne 0 ]; then
    echo "[mini] C++ build failed (session_cpp)." 
    exit $BUILD_STATUS
  fi
else
  echo "[mini] session_cpp directory not found at ${CPP_DIR}"
  exit 1
fi

echo "[mini] Running C++ Point mini tests (point_minitest)..."
if [ -x "${CPP_DIR}/build/point_minitest" ]; then
  CPP_EXE="${CPP_DIR}/build/point_minitest"
elif [ -x "${CPP_DIR}/build/Release/point_minitest" ]; then
  CPP_EXE="${CPP_DIR}/build/Release/point_minitest"
else
  echo "[mini] C++ executable 'point_minitest' not found even after build."
  exit 1
fi

"${CPP_EXE}"

echo "[mini] Building and running Rust Point mini tests (point_minitest)..."
RUST_DIR="${REPO_ROOT}/session_rust"
if [ -d "${RUST_DIR}" ]; then
  ( cd "${RUST_DIR}" && cargo run --release --bin point_minitest )
  RUST_STATUS=$?
  if [ $RUST_STATUS -ne 0 ]; then
    echo "[mini] Rust point_minitest failed."
    exit $RUST_STATUS
  fi
else
  echo "[mini] session_rust directory not found at ${RUST_DIR}"
  exit 1
fi

echo "[mini] Done. JSON results:"
echo "  Point  Python: ${REPO_ROOT}/session_tests/session_py/point_test.json"
echo "         C++   : ${REPO_ROOT}/session_tests/session_cpp/point_test.json"
echo "         Rust  : ${REPO_ROOT}/session_tests/session_rust/point_test.json"
echo "  Color  Python: ${REPO_ROOT}/session_tests/session_py/color_test.json"
echo "         C++   : ${REPO_ROOT}/session_tests/session_cpp/color_test.json"
echo "         Rust  : ${REPO_ROOT}/session_tests/session_rust/color_test.json"

echo "[mini] Opening results website (if possible)..."
# Start a simple HTTP server in session_tests on a dedicated port (best-effort)
PORT=8765

start_http_server() {
  # Prefer python3, fall back to python
  if command -v python3 >/dev/null 2>&1; then
    ( cd "${SCRIPT_DIR}" && python3 -m http.server "${PORT}" >/dev/null 2>&1 & )
    return 0
  elif command -v python >/dev/null 2>&1; then
    ( cd "${SCRIPT_DIR}" && python -m http.server "${PORT}" >/dev/null 2>&1 & )
    return 0
  else
    echo "[mini] Could not start HTTP server: neither python3 nor python was found in PATH."
    return 1
  fi
}

start_http_server
sleep 1

# When served from session_tests/, the site is at /website/
WEBSITE_URL="http://localhost:${PORT}/website/"
WEBSITE_FILE="${SCRIPT_DIR}/website/index.html"

if command -v xdg-open >/dev/null 2>&1; then
  xdg-open "${WEBSITE_URL}" >/dev/null 2>&1 || \
  xdg-open "${WEBSITE_FILE}" >/dev/null 2>&1 || true
else
  echo "[mini] Please open ${WEBSITE_FILE} in a browser (preferably via an HTTP server in session_tests)."
fi
