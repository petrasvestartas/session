#!/bin/bash

# Resolve repository root as the parent of this script's directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="${SCRIPT_DIR}/.."
cd "${REPO_ROOT}"

echo "🔄 Updating main repo..."
git pull

echo "🔄 Updating submodules (init + recursive)..."
git submodule update --init --recursive

echo "🔄 Checking out main branch in all submodules..."
git submodule foreach 'git checkout main 2>/dev/null || git checkout -b main'

echo "✅ Done. Main repo and submodules are up to date on main branch."
