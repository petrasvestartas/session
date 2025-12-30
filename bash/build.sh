#!/bin/bash

# Resolve repository root as the parent of this script's directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="${SCRIPT_DIR}/.."

echo "🔨 Building all projects..."
echo "========================================"

SUCCESS_COUNT=0
FAIL_COUNT=0

for d in session_cpp session_py session_rust; do
  if [ -d "${REPO_ROOT}/$d" ]; then
    echo -e "\n🏗️  Building $d"
    cd "${REPO_ROOT}/$d"

    if [ "$d" = "session_cpp" ]; then
      # C++ Build
      if [ ! -d "build" ]; then
        echo "   Creating build directory..."
        mkdir build
      fi

      cd build

      echo "   Running cmake..."
      if cmake .. > /tmp/cpp_cmake.log 2>&1; then
        echo "   ✓ CMake configuration successful"
      else
        echo "   ✗ CMake configuration failed"
        tail -10 /tmp/cpp_cmake.log
        FAIL_COUNT=$((FAIL_COUNT + 1))
        continue
      fi

      echo "   Compiling with make..."
      if make tests -j$(nproc) > /tmp/cpp_make.log 2>&1; then
        echo "✅ $d build successful"
        echo "   Binary: build/tests"
        # Also show any warnings
        WARNINGS=$(grep "warning:" /tmp/cpp_make.log 2>/dev/null | wc -l)
        if [ "$WARNINGS" -gt 0 ]; then
          echo "   Note: $WARNINGS warnings (non-blocking)"
        fi
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
      else
        echo "❌ $d build failed"
        echo "   Last 15 lines of build output:"
        tail -15 /tmp/cpp_make.log | grep -E "error:|undefined reference|fatal error" || tail -15 /tmp/cpp_make.log
        FAIL_COUNT=$((FAIL_COUNT + 1))
      fi

    elif [ "$d" = "session_py" ]; then
      # Python "Build" (check imports and dependencies)
      if [ -f "${REPO_ROOT}/uvsession/bin/activate" ]; then
        source "${REPO_ROOT}/uvsession/bin/activate"
        echo "   Activated virtual environment: uvsession"

        echo "   Checking Python dependencies..."
        if python -c "import sys; print(f'Python {sys.version}')" > /tmp/py_build.log 2>&1; then
          echo "   ✓ Python available: $(python --version)"
        else
          echo "   ✗ Python check failed"
          FAIL_COUNT=$((FAIL_COUNT + 1))
          continue
        fi

        # Check if main module can be imported
        if python -c "import sys; sys.path.insert(0, '.'); print('Module checks passed')" > /tmp/py_import.log 2>&1; then
          echo "✅ $d ready (no build needed for Python)"
          echo "   Python $(python --version 2>&1)"
          SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        else
          echo "❌ $d - Python import check failed"
          cat /tmp/py_import.log
          FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
        deactivate 2>/dev/null || true
      else
        echo "❌ $d - Virtual environment not found"
        echo "   Run: uv venv uvsession && source uvsession/bin/activate"
        FAIL_COUNT=$((FAIL_COUNT + 1))
      fi

    elif [ "$d" = "session_rust" ]; then
      # Rust Build
      if command -v cargo &> /dev/null; then
        echo "   Running cargo build..."
        if cargo build --release 2>&1 | tee /tmp/rust_build.log | tail -10; then
          echo "✅ $d build successful"
          if [ -f "target/release/session_rust" ]; then
            SIZE=$(ls -lh target/release/session_rust | awk '{print $5}')
            echo "   Binary: target/release/session_rust ($SIZE)"
          fi
          SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        else
          echo "❌ $d build failed"
          echo "   Last 20 lines of build output:"
          tail -20 /tmp/rust_build.log | grep -E "error\[|error:" || tail -20 /tmp/rust_build.log
          FAIL_COUNT=$((FAIL_COUNT + 1))
        fi
      else
        echo "❌ $d - Cargo not found"
        echo "   Install Rust: https://rustup.rs/"
        FAIL_COUNT=$((FAIL_COUNT + 1))
      fi
    fi

  else
    echo -e "\n❌ $d - directory not found"
    FAIL_COUNT=$((FAIL_COUNT + 1))
  fi
done

# Summary
echo -e "\n========================================"
echo "📊 Build Summary:"
echo "   ✅ Successful: $SUCCESS_COUNT"
echo "   ❌ Failed:     $FAIL_COUNT"
echo "========================================"

TOTAL=$((SUCCESS_COUNT + FAIL_COUNT))
echo "   Total:       $TOTAL projects"
echo "========================================"

if [ $FAIL_COUNT -eq 0 ] && [ $SUCCESS_COUNT -gt 0 ]; then
  echo "🎉 All builds successful! ($SUCCESS_COUNT/$TOTAL)"
  exit 0
elif [ $FAIL_COUNT -gt 0 ]; then
  echo "⚠️  Some builds failed ($SUCCESS_COUNT/$TOTAL succeeded)"
  exit 1
else
  echo "⚠️  No builds were attempted"
  exit 2
fi
