#!/bin/bash

# Script to build session_py documentation using Sphinx

echo "Building session_py documentation..."

# Change to docs directory
cd docs

# Clean previous build
echo "Cleaning previous build..."
make clean

# Build HTML documentation
echo "Building HTML documentation..."
make html

if [ $? -eq 0 ]; then
    echo ""
    echo "Documentation built successfully!"
    echo "Open docs_output/html/index.html in your browser to view the documentation."
    echo ""
    echo "To serve locally, run:"
    echo "  cd docs_output/html && python -m http.server 8000"
else
    echo "Build failed!"
    exit 1
fi
