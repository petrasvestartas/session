# Session Protocol Buffers Definitions

This directory contains Protocol Buffers (protobuf) schema definitions for all geometry types and data structures in the Session library.

## Overview

Protocol Buffers provide:
- **Language-agnostic serialization** - Use the same data format across C++, Python, Rust, etc.
- **Efficient binary format** - Smaller and faster than JSON
- **Schema evolution** - Add fields without breaking compatibility
- **Type safety** - Strongly-typed messages with validation

## Available Proto Files

### Basic Types
- `color.proto` - RGBA color representation
- `vector.proto` - 3D vector
- `point.proto` - 3D point with metadata
- `xform.proto` - 4x4 transformation matrix
- `quaternion.proto` - Rotation quaternion

### Geometric Primitives
- `plane.proto` - 3D plane with origin and axes
- `line.proto` - Line segment
- `polyline.proto` - Connected sequence of points
- `mesh.proto` - Halfedge mesh structure
- `boundingbox.proto` - Oriented bounding box (OBB)

### Complex Geometry
- `arrow.proto` - Arrow with cylinder body and cone head
- `cylinder.proto` - Cylindrical geometry
- `pointcloud.proto` - Collection of points

### Graph & Hierarchy
- `vertex.proto` - Graph vertex
- `edge.proto` - Graph edge
- `graph.proto` - Graph data structure
- `treenode.proto` - Hierarchical tree node
- `tree.proto` - Tree structure

### Spatial Structures
- `bvh.proto` - Bounding Volume Hierarchy for collision detection

### Collections & Sessions
- `objects.proto` - Collection of all geometry types
- `session.proto` - Complete session with geometry, graph, and tree

### Utilities
- `tolerance.proto` - Geometric tolerance settings
- `encoders.proto` - Serialization options and metadata

## Usage

### Python Example
```python
import session_proto.point_pb2 as point_pb2

# Create a point
point = point_pb2.Point()
point.x = 1.0
point.y = 2.0
point.z = 3.0
point.name = "my_point"

# Serialize to binary
data = point.SerializeToString()

# Deserialize
point2 = point_pb2.Point()
point2.ParseFromString(data)
```

### C++ Example
```cpp
#include "point.pb.h"

// Create a point
session_proto::Point point;
point.set_x(1.0);
point.set_y(2.0);
point.set_z(3.0);
point.set_name("my_point");

// Serialize to binary
std::string data;
point.SerializeToString(&data);

// Deserialize
session_proto::Point point2;
point2.ParseFromString(data);
```

### Rust Example
```rust
use session_proto::Point;

// Create a point
let mut point = Point::default();
point.x = 1.0;
point.y = 2.0;
point.z = 3.0;
point.name = "my_point".to_string();

// Serialize to binary
let data = point.encode_to_vec();

// Deserialize
let point2 = Point::decode(&data[..]).unwrap();
```

## Building for Each Language

### C++
Already integrated in `session_cpp/CMakeLists.txt`. The proto files are compiled automatically.

### Python
```bash
# Install protobuf compiler
pip install protobuf

# Generate Python code
cd session_proto
protoc --python_out=../session_py/session_proto *.proto
```

### Rust
```bash
# Add to Cargo.toml
[dependencies]
prost = "0.12"

[build-dependencies]
prost-build = "0.12"

# Create build.rs
fn main() {
    prost_build::compile_protos(&["session_proto/*.proto"], &["session_proto/"]).unwrap();
}
```

## Import Dependencies

The proto files have dependencies on each other. The import hierarchy is:

```
Base types (no dependencies):
  ├─ color.proto
  ├─ vector.proto
  └─ xform.proto

Geometry primitives:
  ├─ point.proto (imports: color, xform)
  ├─ plane.proto (imports: point, vector, xform)
  ├─ line.proto (imports: point, xform)
  └─ quaternion.proto

Collections:
  ├─ polyline.proto (imports: point, xform)
  ├─ pointcloud.proto (imports: point, xform)
  ├─ mesh.proto (imports: point, color, xform)
  └─ boundingbox.proto (imports: point, vector, xform)

Complex geometry:
  ├─ arrow.proto (imports: line, mesh, xform)
  └─ cylinder.proto (imports: line, mesh, xform)

Graph:
  ├─ vertex.proto
  ├─ edge.proto
  └─ graph.proto (imports: vertex, edge)

Tree:
  ├─ treenode.proto
  └─ tree.proto (imports: treenode)

Spatial:
  └─ bvh.proto (imports: boundingbox)

Top-level:
  ├─ objects.proto (imports: all geometry types)
  ├─ session.proto (imports: objects, tree, graph, boundingbox)
  ├─ tolerance.proto
  └─ encoders.proto
```

## File Organization

Each proto file follows this structure:
1. **Syntax declaration** - `syntax = "proto3"`
2. **Package declaration** - `package session_proto`
3. **Imports** - Dependencies on other proto files
4. **Message definitions** - Data structures

## Notes

- All messages use `proto3` syntax (latest stable version)
- Package name is `session_proto` across all files
- GUIDs are stored as strings
- Floating-point values use `double` for precision
- Collections use `repeated` keyword for arrays
- Maps use `map<key, value>` syntax

## Integration Status

- ✅ **C++** - Integrated in session_cpp with CMake build
- ⏳ **Python** - Ready to generate
- ⏳ **Rust** - Ready to generate

## Next Steps

1. Generate language-specific code for Python and Rust
2. Implement converters between native types and protobuf messages
3. Add serialization/deserialization tests for each language
4. Create interop tests between languages

## Documentation

- [Protocol Buffers Documentation](https://protobuf.dev/)
- [Proto3 Language Guide](https://protobuf.dev/programming-guides/proto3/)
- [C++ Generated Code Guide](https://protobuf.dev/reference/cpp/cpp-generated/)
- [Python Generated Code Guide](https://protobuf.dev/reference/python/python-generated/)
