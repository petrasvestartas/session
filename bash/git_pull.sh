#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "${SCRIPT_DIR}/.."

TARGET_BRANCH="main"

echo "=== Preflight ==="
if [ -n "$(git status --porcelain)" ]; then
  echo "Working tree is not clean. Commit or stash changes first."
  git status --short
  exit 1
fi

echo -e "\n=== Main repo (${TARGET_BRANCH}) ==="
git fetch origin
git checkout "${TARGET_BRANCH}"
git pull --ff-only origin "${TARGET_BRANCH}"

echo -e "\n=== Clean stale submodule dirs ==="
git submodule status | while read -r line; do
  path=$(echo "$line" | awk '{print $2}')
  if [ -d "$path" ] && [ ! -e "$path/.git" ]; then
    echo "Cleaning stale dir: $path"
    rm -rf "$path"
  fi
done

echo -e "\n=== Init submodules ==="
git submodule sync --recursive
git submodule update --init --recursive

echo -e "\n=== Pull latest in each submodule (${TARGET_BRANCH}) ==="
git submodule foreach --recursive '
  set -e
  if [ -n "$(git status --porcelain)" ]; then
    echo "Submodule dirty: $name"
    git status --short
    exit 1
  fi
  git fetch origin
  git checkout "${TARGET_BRANCH}"
  git pull --ff-only origin "${TARGET_BRANCH}"
'

echo -e "\n=== Final submodule status ==="
git submodule status
