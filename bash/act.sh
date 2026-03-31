#!/usr/bin/env bash
set -euo pipefail
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
act -j minitest --matrix os:ubuntu-22.04 "$@"
