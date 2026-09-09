#!/usr/bin/env bash
# Copy the browser evidence of a verified reconstruction into the course as checkpoint screenshots.
#   docs/reconstruction/collect_shots.sh ~/.cache/session-viewer-course/shots
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
evidence="${1:?evidence root of a replay.py --verify run}/evidence"
for png in "$evidence"/*/checkpoint-*-dpr1.png; do
    step=$(basename "$(dirname "$png")")
    cp "$png" "$root/docs/screenshots/$step.png"
done
ls "$root/docs/screenshots"
