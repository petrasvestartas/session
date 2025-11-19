#!/bin/bash

echo "🔄 Updating main repo..."
git pull

echo "🔄 Updating submodules (init + recursive)..."
git submodule update --init --recursive

echo "✅ Done. Main repo and submodules are up to date."
