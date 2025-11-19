#!/bin/bash


for d in session_cpp session_py session_rust; do
  if [ -d "$d" ]; then
    echo -e "\n📘 $d"
    cd "$d"
    
    if [ "$d" = "session_cpp" ]; then
      bash doc.sh 2>/dev/null && echo "✅ $d docs built" || echo "❌ $d build failed"
    elif [ "$d" = "session_py" ]; then
      if [ -f "../uvsession/bin/activate" ]; then
        source ../uvsession/bin/activate && bash doc.sh 2>/dev/null && echo "✅ $d docs built" || echo "❌ $d build failed"
      else
        echo "⚠️ Virtual environment not found. Run: uv venv uvsession && source uvsession/bin/activate && uv pip install sphinx sphinxawesome-theme"
        echo "❌ $d build failed"
      fi
    elif [ "$d" = "session_rust" ]; then
      cargo doc --no-deps 2>/dev/null && echo "✅ $d docs built" || echo "❌ $d build failed"
    fi
    
    cd ..
  else
    echo "⚠️ $d not found"
  fi
done

if command -v xdg-open &> /dev/null; then
  [ -f "session_cpp/docs_output/html/index.html" ] && xdg-open session_cpp/docs_output/html/index.html
  [ -f "session_py/docs_output/html/index.html" ] && xdg-open session_py/docs_output/html/index.html
  [ -f "session_rust/target/doc/session_rust/index.html" ] && xdg-open session_rust/target/doc/session_rust/index.html
elif command -v open &> /dev/null; then
  [ -f "session_cpp/docs_output/html/index.html" ] && open session_cpp/docs_output/html/index.html
  [ -f "session_py/docs_output/html/index.html" ] && open session_py/docs_output/html/index.html
  [ -f "session_rust/target/doc/session_rust/index.html" ] && open session_rust/target/doc/session_rust/index.html
fi
