#!/usr/bin/env bash
# Run mini tests for both Python and C++ Point implementations.
# Usage:
#   bash minitest.sh
# or
#   ./minitest.sh

# Resolve repository root as the directory containing this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="${SCRIPT_DIR}"
TESTS_DIR="${REPO_ROOT}/session_tests"

VENV_DIR="${REPO_ROOT}/uvsession"
PYTHON="${VENV_DIR}/bin/python"

# Remove old JSON test result files so each run produces fresh results
cleanup_json() {
  rm -f \
    "${TESTS_DIR}/session_py/point_test.json" \
    "${TESTS_DIR}/session_cpp/point_test.json" \
    "${TESTS_DIR}/session_rust/point_test.json" \
    "${TESTS_DIR}/session_py/color_test.json" \
    "${TESTS_DIR}/session_cpp/color_test.json" \
    "${TESTS_DIR}/session_rust/color_test.json"
}

cleanup_json

ensure_python_env() {
  # Prefer uv: create/manage environment from pyproject.toml manifest
  if command -v uv >/dev/null 2>&1; then
    # Create uv-managed virtual environment if it does not exist yet
    if [ ! -x "${PYTHON}" ]; then
      echo "[mini] Creating Python environment with uv at ${VENV_DIR}..."
      ( cd "${REPO_ROOT}" && uv venv uvsession ) || {
        echo "[mini] uv failed to create virtual environment."
        return 1
      }
    fi

    if [ ! -x "${PYTHON}" ]; then
      echo "[mini] Failed to create Python environment at ${VENV_DIR}"
      return 1
    fi

    echo "[mini] Installing Python dependencies from pyproject.toml with uv..."
    # Use uv to install session_py (and its dependencies) into this environment
    ( cd "${REPO_ROOT}/session_py" && uv pip install --python "${PYTHON}" -e . pytest ) || {
      echo "[mini] Failed to install Python dependencies with uv."
      return 1
    }

    return 0
  fi

  # Fallback: use system python3 + venv + pip if uv is not available
  if [ ! -x "${PYTHON}" ]; then
    echo "[mini] Creating Python environment at ${VENV_DIR} with python3 -m venv..."
    if command -v python3 >/dev/null 2>&1; then
      ( cd "${REPO_ROOT}" && python3 -m venv uvsession ) || {
        echo "[mini] python3 -m venv failed."
        return 1
      }
    else
      echo "[mini] Neither 'uv' nor 'python3' is available; cannot create Python environment."
      return 1
    fi
  fi

  if [ ! -x "${PYTHON}" ]; then
    echo "[mini] Failed to create Python environment at ${VENV_DIR}"
    return 1
  fi

  echo "[mini] Installing session_py and pytest into Python environment (pip)..."
  "${PYTHON}" -m pip install -e "${REPO_ROOT}/session_py" pytest >/dev/null 2>&1 || {
    echo "[mini] Failed to install session_py/pytest into Python environment."
    return 1
  }

  return 0
}

if ! ensure_python_env; then
  echo "[mini] Skipping Python mini tests due to environment setup failure."
else
  echo "[mini] Running Python Point mini tests..."
  "${PYTHON}" -m session_py.point_test

  echo "[mini] Running Python Color mini tests..."
  "${PYTHON}" -m session_py.color_test
fi

echo "[mini] Building C++ project (session_cpp) including tests (no test run)..."
CPP_DIR="${REPO_ROOT}/session_cpp"
JOBS="${MINITEST_JOBS:-2}"

if [ -d "${CPP_DIR}" ]; then
  ( cd "${CPP_DIR}" && cmake -S . -B build -DCMAKE_BUILD_TYPE=Release >/dev/null && cmake --build build --config Release -- -j"${JOBS}" )
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
  CPP_EXE="./build/point_minitest"
elif [ -x "${CPP_DIR}/build/Release/point_minitest" ]; then
  CPP_EXE="./build/Release/point_minitest"
else
  echo "[mini] C++ executable 'point_minitest' not found even after build."
  exit 1
fi

( cd "${CPP_DIR}" && "${CPP_EXE}" )

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
echo "  Point  Python: ${TESTS_DIR}/session_py/point_test.json"
echo "         C++   : ${TESTS_DIR}/session_cpp/point_test.json"
echo "         Rust  : ${TESTS_DIR}/session_rust/point_test.json"
echo "  Color  Python: ${TESTS_DIR}/session_py/color_test.json"
echo "         C++   : ${TESTS_DIR}/session_cpp/color_test.json"
echo "         Rust  : ${TESTS_DIR}/session_rust/color_test.json"

echo "[mini] Generating consolidated testData.js..."
generate_test_data_js() {
  # Create public directory if it doesn't exist
  mkdir -p "${TESTS_DIR}/public"
  
  local OUTPUT="${TESTS_DIR}/public/testData.js"
  
  echo "// Auto-generated test data - Do not edit manually" > "${OUTPUT}"
  echo "// Generated at: $(date)" >> "${OUTPUT}"
  echo "window.TEST_DATA = {" >> "${OUTPUT}"
  
  # Array to track which files we process
  local SOURCES=(
    "session_py/point_test.json:python"
    "session_cpp/point_test.json:cpp"
    "session_rust/point_test.json:rust"
    "session_py/color_test.json:python"
    "session_cpp/color_test.json:cpp"
    "session_rust/color_test.json:rust"
  )
  
  local FIRST=true
  for SOURCE in "${SOURCES[@]}"; do
    local FILE_PATH="${SOURCE%%:*}"
    local LANG="${SOURCE##*:}"
    local FULL_PATH="${TESTS_DIR}/${FILE_PATH}"
    
    if [ -f "${FULL_PATH}" ]; then
      local FILE_NAME=$(basename "${FILE_PATH}")
      local KEY="${FILE_NAME%.json}_${LANG}"
      
      if [ "${FIRST}" = false ]; then
        echo "," >> "${OUTPUT}"
      fi
      FIRST=false
      
      echo -n "  \"${KEY}\": " >> "${OUTPUT}"
      cat "${FULL_PATH}" >> "${OUTPUT}"
    fi
  done
  
  # Add JSON artifact files (test_point.json, etc.) from each language
  local ARTIFACTS=(
    "session_py/test_point.json:python"
    "session_cpp/test_point.json:cpp"
    "session_rust/test_point.json:rust"
  )
  
  for ARTIFACT in "${ARTIFACTS[@]}"; do
    local FILE_PATH="${ARTIFACT%%:*}"
    local LANG="${ARTIFACT##*:}"
    local FULL_PATH="${REPO_ROOT}/${FILE_PATH}"
    
    if [ -f "${FULL_PATH}" ]; then
      local FILE_NAME=$(basename "${FILE_PATH}")
      local KEY="artifact_${FILE_NAME%.json}_${LANG}"
      
      echo "," >> "${OUTPUT}"
      echo -n "  \"${KEY}\": " >> "${OUTPUT}"
      cat "${FULL_PATH}" >> "${OUTPUT}"
    fi
  done
  
  echo "" >> "${OUTPUT}"
  echo "};" >> "${OUTPUT}"
  
  echo "[mini] testData.js generated at ${OUTPUT}"
  
  # Also copy to root for backward compatibility
  cp "${OUTPUT}" "${TESTS_DIR}/testData.js"
}

generate_test_data_js

echo "[mini] Setting up Vue.js application..."

ensure_node_env() {
  # Check if npm is available
  if ! command -v npm >/dev/null 2>&1; then
    echo "[mini] npm not found. Please install Node.js first:"
    echo "       - Ubuntu/Debian: sudo apt install nodejs npm"
    echo "       - Fedora: sudo dnf install nodejs npm"
    echo "       - macOS: brew install node"
    echo "       - Or use nvm: https://github.com/nvm-sh/nvm"
    return 1
  fi

  # Check if node is available (npm requires node)
  if ! command -v node >/dev/null 2>&1; then
    echo "[mini] node not found. npm is installed but node runtime is missing."
    echo "       Please reinstall Node.js properly."
    return 1
  fi

  echo "[mini] Found Node.js $(node --version) with npm $(npm --version)"

  # Check if package.json exists
  if [ ! -f "${TESTS_DIR}/package.json" ]; then
    echo "[mini] No package.json found. Initializing Vue project..."
    ( cd "${TESTS_DIR}" && npm create vue@latest . -- --template minimal --typescript false --jsx false --router false --pinia false --vitest false --e2e false --eslint false --prettier false ) 2>/dev/null || {
      # Fallback: create minimal package.json for Vite
      cat > "${TESTS_DIR}/package.json" << 'EOF'
{
  "name": "session-tests",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "vue": "^3.4.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.0.0",
    "vite": "^5.0.0"
  }
}
EOF
    }
  fi

  # Always ensure dependencies are installed/up-to-date
  echo "[mini] Ensuring npm dependencies are installed..."
  ( cd "${TESTS_DIR}" && npm install --silent ) || {
    echo "[mini] npm install failed. Trying fresh install..."
    rm -rf "${TESTS_DIR}/node_modules" "${TESTS_DIR}/package-lock.json"
    ( cd "${TESTS_DIR}" && npm install ) || {
      echo "[mini] Failed to install npm dependencies."
      return 1
    }
  }

  return 0
}

if ! ensure_node_env; then
  echo "[mini] Skipping Vue.js application due to missing Node.js/npm."
  echo "[mini] Mini tests completed (without web UI)."
  exit 0
fi

# Build the Vue application
echo "[mini] Building Vue application..."
BUILD_OUTPUT=$( cd "${TESTS_DIR}" && npm run build 2>&1 )
BUILD_STATUS=$?
if [ $BUILD_STATUS -ne 0 ]; then
  echo "[mini] Vue build failed. Error output:"
  echo "${BUILD_OUTPUT}" | tail -30
  echo ""
  echo "[mini] Attempting to fix by reinstalling dependencies..."
  rm -rf "${TESTS_DIR}/node_modules" "${TESTS_DIR}/package-lock.json"
  ( cd "${TESTS_DIR}" && npm install ) && ( cd "${TESTS_DIR}" && npm run build )
  if [ $? -ne 0 ]; then
    echo "[mini] Failed to build Vue application after reinstall."
    exit 1
  fi
fi

echo "[mini] Starting development server..."
PORT=8769

# Kill any existing dev server on this port
if command -v pkill >/dev/null 2>&1; then
  pkill -f "vite.*${PORT}" >/dev/null 2>&1 || true
fi

# Start Vite dev server in background (uses port from vite.config: 8769)
( cd "${TESTS_DIR}" && npm run dev >/dev/null 2>&1 & )
sleep 2

# Use the configured Vite base path so refresh works without warnings
WEBSITE_URL="http://localhost:${PORT}/session/tests?suite=point_test"

echo "[mini] Opening results website at ${WEBSITE_URL}..."
if command -v xdg-open >/dev/null 2>&1; then
  xdg-open "${WEBSITE_URL}" >/dev/null 2>&1 || true
else
  echo "[mini] Please open ${WEBSITE_URL} in a browser."
fi

echo "[mini] Development server is running. Press Ctrl+C to stop."
