#!/bin/bash

# Simple script to generate C++ API documentation
# Now uses doxygen-awesome theme for modern, beautiful docs

echo "🔄 Generating C++ API documentation with doxygen-awesome theme..."

# Check if Doxygen is available
if ! command -v doxygen &> /dev/null; then
    echo "❌ Doxygen command not found!"
    echo "💡 Please ensure Doxygen is installed and available in PATH."
    if [ -n "$CI" ]; then
        echo "🔍 This is expected on some CI environments where PATH may not be updated after package installation."
        echo "🛠️  The workflow is configured to continue-on-error for documentation builds."
    fi
    echo "🔍 Current PATH: $PATH"
    exit 1
fi

echo "✅ Doxygen found: $(which doxygen)"
echo "📋 Doxygen version: $(doxygen --version)"

# Change to docs folder and generate documentation
cd docs && doxygen Doxyfile && cd ..

if [ $? -eq 0 ]; then
    echo "✅ Documentation generated successfully with doxygen-awesome theme!"
    echo "📚 Location: docs_output/html/"
    echo "🌐 Open docs_output/html/index.html in your browser"
    echo "✨ Features: Dark mode toggle, modern design, copy buttons, interactive TOC"
    echo ""
    echo "Quick view (if available):"
    
    # Skip auto-opening in CI environments; attempt locally only
    if [ -n "$CI" ]; then
        echo "CI environment detected; skipping auto-opening the docs."
    else
        # Try to open in browser (Linux)
        if command -v xdg-open &> /dev/null; then
            echo "🚀 Opening in default browser..."
            xdg-open docs_output/html/index.html
        elif command -v open &> /dev/null; then
            # macOS
            echo "🚀 Opening in default browser..."
            open docs_output/html/index.html
        else
            echo "💡 Manually open docs_output/html/index.html in your browser"
        fi
    fi
else
    echo "❌ Documentation generation failed!"
    exit 1
fi
