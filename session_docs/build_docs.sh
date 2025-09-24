#!/bin/bash

# Session Library - Combined Documentation Builder
# Builds documentation for all languages (Rust, C++, Python) and creates unified docs

set -e  # Exit on any error

echo "🚀 Building Session Library Documentation..."
echo "=============================================="

# Get the absolute path of the session root directory
SESSION_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCS_DIR="$SESSION_ROOT/session_docs"
OUTPUT_DIR="$DOCS_DIR/combined_docs"

echo "📁 Session root: $SESSION_ROOT"
echo "📁 Docs directory: $DOCS_DIR"
echo "📁 Output directory: $OUTPUT_DIR"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Copy the main index.html
echo "📋 Copying main documentation index..."
cp "$DOCS_DIR/index.html" "$OUTPUT_DIR/"

# Build and copy Rust documentation
echo ""
echo "🦀 Building Rust Documentation..."
echo "================================="
if [ -d "$SESSION_ROOT/session_rust" ]; then
    cd "$SESSION_ROOT/session_rust"
    
    # Check if Rust is installed
    if command -v cargo &> /dev/null; then
        echo "✅ Cargo found: $(which cargo)"
        
        # Build Rust docs
        cargo doc --no-deps --document-private-items
        
        if [ $? -eq 0 ]; then
            echo "✅ Rust documentation built successfully!"
            mkdir -p "$OUTPUT_DIR/rust"
            cp -r target/doc "$OUTPUT_DIR/rust/html"
        else
            echo "❌ Rust documentation build failed!"
        fi
    else
        echo "⚠️  Cargo not found - skipping Rust documentation"
    fi
else
    echo "⚠️  session_rust directory not found - skipping"
fi

# Build and copy C++ documentation
echo ""
echo "⚡ Building C++ Documentation..."
echo "==============================="
if [ -d "$SESSION_ROOT/session_cpp" ]; then
    cd "$SESSION_ROOT/session_cpp"
    
    # Check if Doxygen is installed
    if command -v doxygen &> /dev/null; then
        echo "✅ Doxygen found: $(which doxygen)"
        echo "📋 Doxygen version: $(doxygen --version)"
        
        # Build C++ docs using the doc.sh script
        if [ -f "doc.sh" ]; then
            chmod +x doc.sh
            ./doc.sh
        else
            # Fallback to direct doxygen call
            cd docs && doxygen Doxyfile && cd ..
        fi
        
        if [ $? -eq 0 ]; then
            echo "✅ C++ documentation built successfully!"
            mkdir -p "$OUTPUT_DIR/cpp"
            cp -r docs_output/* "$OUTPUT_DIR/cpp/" 2>/dev/null || echo "No C++ docs output found"
        else
            echo "❌ C++ documentation build failed!"
        fi
    else
        echo "⚠️  Doxygen not found - skipping C++ documentation"
    fi
else
    echo "⚠️  session_cpp directory not found - skipping"
fi

# Build and copy Python documentation
echo ""
echo "🐍 Building Python Documentation..."
echo "=================================="
if [ -d "$SESSION_ROOT/session_py" ]; then
    cd "$SESSION_ROOT/session_py"
    
    # Check if Python and pip are installed
    if command -v python3 &> /dev/null && command -v pip3 &> /dev/null; then
        echo "✅ Python found: $(which python3)"
        
        # Install documentation dependencies
        echo "📦 Installing Python documentation dependencies..."
        pip3 install sphinx sphinxawesome-theme myst-parser || echo "Warning: Could not install some dependencies"
        
        # Install the package itself for autodoc
        pip3 install -e . || echo "Warning: Could not install package in development mode"
        
        # Build Python docs using the doc.sh script
        if [ -f "doc.sh" ]; then
            chmod +x doc.sh
            ./doc.sh
        else
            # Fallback for manual Sphinx build
            if [ -d "docs" ]; then
                cd docs
                if command -v sphinx-build &> /dev/null; then
                    sphinx-build -b html . _build
                fi
                cd ..
            fi
        fi
        
        if [ $? -eq 0 ]; then
            echo "✅ Python documentation built successfully!"
            mkdir -p "$OUTPUT_DIR/python"
            cp -r docs_output/* "$OUTPUT_DIR/python/" 2>/dev/null || echo "No Python docs output found"
        else
            echo "❌ Python documentation build failed!"
        fi
    else
        echo "⚠️  Python3 or pip3 not found - skipping Python documentation"
    fi
else
    echo "⚠️  session_py directory not found - skipping"
fi

# Final summary
echo ""
echo "🎉 Documentation Build Complete!"
echo "================================"
echo "📁 Combined documentation location: $OUTPUT_DIR"
echo "🌐 Open: $OUTPUT_DIR/index.html"
echo ""

# List what was actually built
echo "📋 Built documentation:"
if [ -d "$OUTPUT_DIR/rust" ]; then
    echo "  ✅ Rust documentation"
else
    echo "  ❌ Rust documentation (failed or skipped)"
fi

if [ -d "$OUTPUT_DIR/cpp" ]; then
    echo "  ✅ C++ documentation"
else
    echo "  ❌ C++ documentation (failed or skipped)"
fi

if [ -d "$OUTPUT_DIR/python" ]; then
    echo "  ✅ Python documentation"
else
    echo "  ❌ Python documentation (failed or skipped)"
fi

# Option to open documentation
if [[ "$1" == "--open" ]]; then
    echo ""
    echo "🚀 Opening documentation in browser..."
    if command -v open &> /dev/null; then
        # macOS
        open "$OUTPUT_DIR/index.html"
    elif command -v xdg-open &> /dev/null; then
        # Linux
        xdg-open "$OUTPUT_DIR/index.html"
    else
        echo "💡 Please manually open: $OUTPUT_DIR/index.html"
    fi
fi

echo ""
echo "✨ Done!"
