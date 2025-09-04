#!/bin/bash

# Session Library - Top-level Documentation Builder
# Convenience script to build all documentation from project root

echo "🚀 Session Library - Building All Documentation"
echo "==============================================="

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to session_docs directory and run the build script
cd "$SCRIPT_DIR/session_docs"

echo "📁 Building from: $(pwd)"
echo ""

# Run the documentation build script
if [ "$1" == "--open" ]; then
    ./build_docs.sh --open
else
    ./build_docs.sh
fi
