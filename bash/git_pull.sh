#!/bin/bash
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "${SCRIPT_DIR}/.."

echo "=== Main repo ==="
git pull

echo -e "\n=== Clean stale submodule dirs ==="
git submodule status | while read -r line; do
  path=$(echo "$line" | awk '{print $2}')
  if [ -d "$path" ] && [ ! -e "$path/.git" ]; then
    echo "Cleaning stale dir: $path"
    rm -rf "$path"
  fi
done

echo -e "\n=== Init submodules ==="
git submodule update --init --recursive

echo -e "\n=== Pull latest in each submodule ==="
git submodule foreach 'git checkout main && git pull origin main'

echo -e "\n=== Status ==="
git submodule status
