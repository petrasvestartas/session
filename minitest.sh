#!/usr/bin/env bash
# Run mini tests for both Python and C++ Point implementations.
# Usage:
#   bash minitest.sh          # Full build
#   ./minitest.sh --fast      # Skip protobuf regen, pip install, npm install
#   ./minitest.sh --py        # Python only
#   ./minitest.sh --cpp       # C++ only
#   ./minitest.sh --rust      # Rust only

# Parse arguments
FAST_MODE=false
RUN_PYTHON=true
RUN_CPP=true
RUN_RUST=true

for arg in "$@"; do
  case $arg in
    --fast|-f)
      FAST_MODE=true
      ;;
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
  esac
done

# Resolve repository root as the directory containing this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="${SCRIPT_DIR}"
TESTS_DIR="${REPO_ROOT}/session_tests"

VENV_DIR="${REPO_ROOT}/uvsession"
PYTHON="${VENV_DIR}/bin/python"

###############################################################################
# SINGLE DEFINITION: Add new class names here to include them in tests
###############################################################################
CLASS_NAMES=("color" "line" "plane" "point" "pointcloud" "polyline" "tolerance" "vector" "xform")
# Sort CLASS_NAMES alphabetically
readarray -t CLASS_NAMES < <(printf '%s\n' "${CLASS_NAMES[@]}" | sort)
LANGUAGES=("python" "cpp" "rust")
LANG_DIRS=("session_py" "session_cpp" "session_rust")

# Remove old JSON test result files so each run produces fresh results
cleanup_json() {
  for class_name in "${CLASS_NAMES[@]}"; do
    rm -f "${TESTS_DIR}/session_py/${class_name}_test.json"
    rm -f "${TESTS_DIR}/session_cpp/${class_name}_test.json"
    rm -f "${TESTS_DIR}/session_rust/${class_name}_test.json"
  done
}

cleanup_json

# Regenerate Python protobuf bindings from shared session_proto folder
regenerate_python_protos() {
  local PROTO_DIR="${REPO_ROOT}/session_proto"
  local PY_PROTO_OUT="${REPO_ROOT}/session_py/src/session_py/proto"
  
  # Try to find protoc: system, C++ build, or local install
  local PROTOC=""
  if command -v protoc >/dev/null 2>&1; then
    PROTOC="protoc"
  elif [ -f "${REPO_ROOT}/session_cpp/build/protobuf_external-prefix/src/protobuf_external-build/protoc" ]; then
    PROTOC="${REPO_ROOT}/session_cpp/build/protobuf_external-prefix/src/protobuf_external-build/protoc"
  elif [ -f "$HOME/.local/install/protobuf/bin/protoc" ]; then
    PROTOC="$HOME/.local/install/protobuf/bin/protoc"
  else
    echo "[mini] Warning: protoc not found, skipping Python protobuf regeneration"
    return 0
  fi
  
  # Check if proto files exist
  if [ ! -d "${PROTO_DIR}" ]; then
    echo "[mini] Warning: ${PROTO_DIR} not found, skipping Python protobuf regeneration"
    return 0
  fi
  
  # Create output directory if needed
  mkdir -p "${PY_PROTO_OUT}"
  
  # Auto-discover and regenerate Python bindings for ALL proto files
  echo "[mini] Regenerating Python protobuf bindings from session_proto/..."
  for proto_file in "${PROTO_DIR}"/*.proto; do
    if [ -f "$proto_file" ]; then
      "$PROTOC" --python_out="${PY_PROTO_OUT}" -I "${PROTO_DIR}" "$proto_file" || {
        echo "[mini] Warning: Failed to regenerate $(basename "$proto_file")"
      }
    fi
  done
  
  # Fix imports: protoc generates absolute imports but we need relative imports
  # since the files are in a package (session_py.proto)
  echo "[mini] Fixing Python protobuf imports to use relative imports..."
  for pb_file in "${PY_PROTO_OUT}"/*_pb2.py; do
    if [ -f "$pb_file" ]; then
      # Replace "import xxx_pb2" with "from . import xxx_pb2"
      sed -i 's/^import \([a-z_]*_pb2\) as/from . import \1 as/g' "$pb_file"
    fi
  done
  
  return 0
}

# Skip protobuf regeneration in fast mode
if [ "${FAST_MODE}" = false ]; then
  regenerate_python_protos
fi

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

    # Skip pip install in fast mode if already installed
    if [ "${FAST_MODE}" = true ] && "${PYTHON}" -c "import session_py" 2>/dev/null; then
      echo "[mini] Fast mode: Skipping pip install (session_py already installed)"
    else
      echo "[mini] Installing Python dependencies from pyproject.toml with uv..."
      # Use uv to install session_py (and its dependencies) into this environment
      ( cd "${REPO_ROOT}/session_py" && uv pip install --python "${PYTHON}" -e . pytest ) || {
        echo "[mini] Failed to install Python dependencies with uv."
        return 1
      }
    fi

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

if [ "${RUN_PYTHON}" = true ]; then
  if ! ensure_python_env; then
    echo "[mini] Skipping Python mini tests due to environment setup failure."
  else
    for class_name in "${CLASS_NAMES[@]}"; do
      echo "[mini] Running Python ${class_name} mini tests..."
      "${PYTHON}" -m "session_py.${class_name}_test"
    done
  fi
fi

CPP_DIR="${REPO_ROOT}/session_cpp"
# Use all available cores by default for faster builds
JOBS="${MINITEST_JOBS:-$(nproc 2>/dev/null || echo 4)}"

if [ "${RUN_CPP}" = true ]; then
  echo "[mini] Building C++ project (session_cpp) with ${JOBS} jobs..."
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

  # Run all C++ mini tests (one combined executable)
  echo "[mini] Running C++ mini tests (point_minitest)..."
  if [ -x "${CPP_DIR}/build/point_minitest" ]; then
    CPP_EXE="./build/point_minitest"
  elif [ -x "${CPP_DIR}/build/Release/point_minitest" ]; then
    CPP_EXE="./build/Release/point_minitest"
  else
    echo "[mini] C++ executable 'point_minitest' not found even after build."
    exit 1
  fi

  ( cd "${CPP_DIR}" && "${CPP_EXE}" )
fi

RUST_DIR="${REPO_ROOT}/session_rust"

if [ "${RUN_RUST}" = true ]; then
  echo "[mini] Building and running Rust mini tests..."
  if [ -d "${RUST_DIR}" ]; then
    # Find protoc from cargo vendored package or system
    PROTOC_PATH=$(find ~/.cargo/registry/src -name "protoc" -path "*linux-x86_64*" 2>/dev/null | head -1)
    if [ -n "${PROTOC_PATH}" ]; then
      export PROTOC="${PROTOC_PATH}"
    fi
    ( cd "${RUST_DIR}" && cargo run --release --features protobuf --bin minitest )
    RUST_STATUS=$?
    if [ $RUST_STATUS -ne 0 ]; then
      echo "[mini] Rust minitest failed."
      exit $RUST_STATUS
    fi
  else
    echo "[mini] session_rust directory not found at ${RUST_DIR}"
    exit 1
  fi
fi

echo "[mini] Done. JSON results:"
echo "  Point     Python: ${TESTS_DIR}/session_py/point_test.json"
echo "            C++   : ${TESTS_DIR}/session_cpp/point_test.json"
echo "            Rust  : ${TESTS_DIR}/session_rust/point_test.json"
echo "  Color     Python: ${TESTS_DIR}/session_py/color_test.json"
echo "            C++   : ${TESTS_DIR}/session_cpp/color_test.json"
echo "            Rust  : ${TESTS_DIR}/session_rust/color_test.json"
echo "  Vector    Python: ${TESTS_DIR}/session_py/vector_test.json"
echo "            C++   : ${TESTS_DIR}/session_cpp/vector_test.json"
echo "            Rust  : ${TESTS_DIR}/session_rust/vector_test.json"
echo "  Tolerance Python: ${TESTS_DIR}/session_py/tolerance_test.json"
echo "            C++   : ${TESTS_DIR}/session_cpp/tolerance_test.json"
echo "            Rust  : ${TESTS_DIR}/session_rust/tolerance_test.json"

echo "[mini] Generating consolidated testData.js..."
generate_test_data_js() {
  # Create public directory if it doesn't exist
  mkdir -p "${TESTS_DIR}/public"
  
  local OUTPUT="${TESTS_DIR}/public/testData.js"
  
  echo "// Auto-generated test data - Do not edit manually" > "${OUTPUT}"
  echo "// Generated at: $(date)" >> "${OUTPUT}"
  echo "window.TEST_DATA = {" >> "${OUTPUT}"
  
  # Build sources dynamically from CLASS_NAMES
  local FIRST=true
  for class_name in "${CLASS_NAMES[@]}"; do
    for i in "${!LANGUAGES[@]}"; do
      local lang="${LANGUAGES[$i]}"
      local lang_dir="${LANG_DIRS[$i]}"
      local FULL_PATH="${TESTS_DIR}/${lang_dir}/${class_name}_test.json"
      
      if [ -f "${FULL_PATH}" ]; then
        local KEY="${class_name}_test_${lang}"
        
        if [ "${FIRST}" = false ]; then
          echo "," >> "${OUTPUT}"
        fi
        FIRST=false
        
        echo -n "  \"${KEY}\": " >> "${OUTPUT}"
        cat "${FULL_PATH}" >> "${OUTPUT}"
      fi
    done
  done
  
  # Add JSON artifact files (test_<class>.json) from each language - built from CLASS_NAMES
  for class_name in "${CLASS_NAMES[@]}"; do
    for i in "${!LANGUAGES[@]}"; do
      local lang="${LANGUAGES[$i]}"
      local lang_dir="${LANG_DIRS[$i]}"
      local FULL_PATH="${REPO_ROOT}/${lang_dir}/test_${class_name}.json"
      
      if [ -f "${FULL_PATH}" ]; then
        local KEY="artifact_test_${class_name}_${lang}"
        
        echo "," >> "${OUTPUT}"
        echo -n "  \"${KEY}\": " >> "${OUTPUT}"
        cat "${FULL_PATH}" >> "${OUTPUT}"
      fi
    done
  done
  
  # Auto-discover and add ALL proto schema files from session_proto/
  local PROTO_DIR="${REPO_ROOT}/session_proto"
  for proto_file in "${PROTO_DIR}"/*.proto; do
    if [ -f "$proto_file" ]; then
      local proto_name=$(basename "$proto_file" .proto)
      local KEY="proto_${proto_name}"
      echo "," >> "${OUTPUT}"
      # Escape the proto content for JSON string
      local CONTENT=$(cat "$proto_file" | sed 's/\\/\\\\/g' | sed 's/"/\\"/g' | sed ':a;N;$!ba;s/\n/\\n/g')
      echo -n "  \"${KEY}\": \"${CONTENT}\"" >> "${OUTPUT}"
    fi
  done
  
  echo "" >> "${OUTPUT}"
  echo "};" >> "${OUTPUT}"
  
  echo "[mini] testData.js generated at ${OUTPUT}"
  
  # Also copy to root for backward compatibility
  cp "${OUTPUT}" "${TESTS_DIR}/testData.js"
}

generate_test_data_js

# Optional: Generate static search index (silently skip if not present)
if [ -f "${TESTS_DIR}/rag/generate_search_index.py" ]; then
  echo "[mini] Generating static search index for client-side search..."
  python3 "${TESTS_DIR}/rag/generate_search_index.py" 2>/dev/null || true
fi

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

  # Skip npm install in fast mode if node_modules exists
  if [ "${FAST_MODE}" = true ] && [ -d "${TESTS_DIR}/node_modules" ]; then
    echo "[mini] Fast mode: Skipping npm install (node_modules exists)"
  else
    echo "[mini] Ensuring npm dependencies are installed..."
    ( cd "${TESTS_DIR}" && npm install --silent ) || {
      echo "[mini] npm install failed. Trying fresh install..."
      rm -rf "${TESTS_DIR}/node_modules" "${TESTS_DIR}/package-lock.json"
      ( cd "${TESTS_DIR}" && npm install ) || {
        echo "[mini] Failed to install npm dependencies."
        return 1
      }
    }
  fi

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

# Check if running in CI environment (GitHub Actions, etc.)
if [ -n "${CI:-}" ] || [ -n "${GITHUB_ACTIONS:-}" ]; then
  echo "[mini] CI environment detected - skipping dev server and browser."
  echo "[mini] Build complete. Dist folder ready for deployment."
  echo "[mini] Done."
  exit 0
fi

# === LOCAL DEVELOPMENT ONLY (below this line) ===

# Optional: Start RAG API server (silently skip if not present)
RAG_PORT=8770
RAG_DIR="${TESTS_DIR}/rag"
if [ -x "/home/pv/anaconda3/bin/python3" ] && [ -f "${RAG_DIR}/rag_api.py" ]; then
  echo "[mini] Starting RAG API server..."
  pkill -f "rag_api.py" >/dev/null 2>&1 || true
  sleep 1
  cd "${RAG_DIR}" && nohup /home/pv/anaconda3/bin/python3 rag_api.py > /tmp/rag_api.log 2>&1 &
  RAG_PID=$!
  cd "${REPO_ROOT}"
  sleep 2
  if lsof -i :${RAG_PORT} >/dev/null 2>&1; then
    echo "[mini] RAG API server started on http://localhost:${RAG_PORT} (PID: ${RAG_PID})"
  fi
fi

PORT=8769
WEBSITE_URL="http://localhost:${PORT}/session/tests?suite=point_test"

# Check if dev server is already running
SERVER_RUNNING=false
if lsof -i :${PORT} >/dev/null 2>&1; then
  SERVER_RUNNING=true
  echo "[mini] Development server already running on port ${PORT}."
  echo "[mini] Browser will auto-refresh with new test data (Vite HMR)."
else
  echo "[mini] Starting development server..."
  # Start Vite dev server in background
  ( cd "${TESTS_DIR}" && npm run dev >/dev/null 2>&1 & )
  sleep 2

  # Only open browser on first run (server wasn't running before)
  echo "[mini] Opening results website at ${WEBSITE_URL}..."
  if command -v xdg-open >/dev/null 2>&1; then
    xdg-open "${WEBSITE_URL}" >/dev/null 2>&1 || true
  else
    echo "[mini] Please open ${WEBSITE_URL} in a browser."
  fi
fi

echo "[mini] Development server is running at ${WEBSITE_URL}"
echo "[mini] Re-run ./minitest.sh to update test results (browser auto-refreshes)."
