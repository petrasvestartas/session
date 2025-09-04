#!/bin/bash

# Simple script to generate C++ API documentation

echo "🔄 Generating C++ API documentation..."

# Change to docs folder and generate documentation
cd docs && doxygen Doxyfile && cd ..

if [ $? -eq 0 ]; then
    echo "✅ Documentation generated successfully!"
    echo "📚 Location: docs_output/html/"
    echo "🌐 Open docs_output/html/index.html in your browser"
    echo ""
    echo "Quick view (if available):"
    
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
else
    echo "❌ Documentation generation failed!"
    exit 1
fi
