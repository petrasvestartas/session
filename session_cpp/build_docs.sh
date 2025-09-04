#!/bin/bash

# Session C++ Documentation Builder
# Builds the documentation using Doxygen with doxygen-awesome theme

echo "Building Session C++ documentation with doxygen-awesome theme..."

# Navigate to docs directory
cd docs

# Run doxygen
doxygen Doxyfile

if [ $? -eq 0 ]; then
    echo "✅ Documentation built successfully!"
    echo "📖 Open: docs_output/html/index.html"
    
    # Optionally open the documentation
    if [[ "$1" == "--open" ]]; then
        open ../docs_output/html/index.html
    fi
else
    echo "❌ Documentation build failed!"
    exit 1
fi
