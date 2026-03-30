#!/bin/bash
# Deterministic autoformatter for consistent code style.
# Usage: ./bash/format.sh [--py] [--cpp] [--rust] [--dry-run] [file]
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
python3 "$ROOT/bash/format.py" "$@"
