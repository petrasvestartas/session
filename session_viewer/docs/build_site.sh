#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
bash "$root/../session_tests/scripts/build-viewer-docs.sh"
mkdir -p "$root/target/docs/site"
cp -a "$root/target/docs/vue/." "$root/target/docs/site/"
