#!/bin/bash
# Generate Python protobuf files from all .proto files in session_proto/
# This script uses the protoc binary built by the C++ project

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROTO_DIR="$SCRIPT_DIR/session_proto"
PYTHON_PROTO_DIR="$SCRIPT_DIR/session_py/src/session_py/proto"

# Find protoc - first try system, then C++ build
if command -v protoc &> /dev/null; then
    PROTOC="protoc"
elif [ -f "$SCRIPT_DIR/session_cpp/build/protobuf_external-prefix/src/protobuf_external-build/protoc" ]; then
    PROTOC="$SCRIPT_DIR/session_cpp/build/protobuf_external-prefix/src/protobuf_external-build/protoc"
elif [ -f "$HOME/.local/install/protobuf/bin/protoc" ]; then
    PROTOC="$HOME/.local/install/protobuf/bin/protoc"
else
    echo "Error: protoc not found. Build the C++ project first or install protobuf-compiler."
    exit 1
fi

echo "Using protoc: $PROTOC"
echo "Proto source: $PROTO_DIR"
echo "Python output: $PYTHON_PROTO_DIR"

# Create output directory if needed
mkdir -p "$PYTHON_PROTO_DIR"

# Generate Python files for all .proto files
for proto_file in "$PROTO_DIR"/*.proto; do
    if [ -f "$proto_file" ]; then
        filename=$(basename "$proto_file" .proto)
        echo "Generating ${filename}_pb2.py..."
        "$PROTOC" --python_out="$PYTHON_PROTO_DIR" -I"$PROTO_DIR" "$proto_file"
    fi
done

# Fix imports: convert absolute imports to relative imports for package use
echo "Fixing imports for package use..."
for py_file in "$PYTHON_PROTO_DIR"/*_pb2.py; do
    if [ -f "$py_file" ]; then
        # Replace "import xxx_pb2" with "from . import xxx_pb2"
        sed -i 's/^import \([a-z_]*_pb2\) as/from . import \1 as/' "$py_file"
    fi
done

echo "Done! Generated Python protobuf files in $PYTHON_PROTO_DIR"
