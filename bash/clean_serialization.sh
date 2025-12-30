#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

rm -f "$ROOT_DIR/session_rust/serialization/"*
rm -f "$ROOT_DIR/session_py/serialization/"*
rm -f "$ROOT_DIR/session_cpp/serialization/"*
