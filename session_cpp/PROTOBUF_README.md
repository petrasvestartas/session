# Protocol Buffers Integration

This project includes **optional** Protocol Buffers (protobuf) support for efficient serialization of data structures.

## Overview

The protobuf definition file `src/session.proto` mirrors the structure of the C++ classes (Point, Color, Vector, Xform) and provides efficient binary serialization.

**Protobuf is OPTIONAL** - you can enable or disable it at build time.

## Enabling/Disabling Protobuf

```bash
# Enable protobuf (default)
cmake -DENABLE_PROTOBUF=ON ..

# Disable protobuf
cmake -DENABLE_PROTOBUF=OFF ..
```

When disabled:
- No external dependencies are downloaded or built
- Protobuf test file (`session_proto_test.cpp`) is excluded
- Your project builds faster without protobuf overhead

## Build Process

When you run CMake with `ENABLE_PROTOBUF=ON`, the following happens **automatically**:

1. **Abseil C++ library** is downloaded and built (required by protobuf v29)
2. **Protocol Buffers library** and `protoc` compiler are downloaded and built
3. **C++ code is generated** from `session.proto`:
   - `build/generated/session.pb.h` (header file)
   - `build/generated/session.pb.cc` (implementation)
4. **Your project is compiled** with the generated protobuf code linked

All of this happens **before** your main project is built, so you can use protobuf immediately.

### Build Caching (Important!)

**Abseil and Protobuf are built ONLY ONCE** and cached in `build/external/`:
- First build: ~2-3 minutes (downloads + builds external deps)
- Subsequent builds: ~seconds (uses cached libraries)
- Only `.proto` file changes trigger regeneration of C++ code

To force rebuild of external dependencies:
```bash
rm -rf build/external/
cmake --build build
```

## File Structure

```
session_cpp/
├── src/
│   ├── session.proto          # Protobuf definition (your data structures)
│   ├── session_proto_test.cpp # Example usage and tests
│   ├── point.h/cpp            # Original C++ Point class
│   └── ...
├── build/
│   ├── external/              # External dependencies (Abseil, Protobuf)
│   │   ├── abseil/
│   │   └── protobuf/
│   └── generated/             # Generated protobuf C++ code
│       ├── session.pb.h
│       └── session.pb.cc
└── CMakeLists.txt             # Build configuration
```

## Usage Example

### Include the generated header

```cpp
#include "session.pb.h"
```

### Create and populate a message

```cpp
session_proto::Point pb_point;
pb_point.set_guid("my-unique-id");
pb_point.set_name("my_point");
pb_point.set_x(10.5);
pb_point.set_y(20.5);
pb_point.set_z(30.5);
pb_point.set_width(2.0);

// Set color
auto* color = pb_point.mutable_pointcolor();
color->set_r(255);
color->set_g(128);
color->set_b(64);
color->set_a(255);
```

### Serialize to binary

```cpp
// Serialize to string
std::string binary_data;
pb_point.SerializeToString(&binary_data);

// Write to file
std::ofstream ofs("point.bin", std::ios::binary);
pb_point.SerializeToOstream(&ofs);
```

### Deserialize from binary

```cpp
// Deserialize from string
session_proto::Point loaded_point;
loaded_point.ParseFromString(binary_data);

// Read from file
std::ifstream ifs("point.bin", std::ios::binary);
loaded_point.ParseFromIstream(&ifs);

// Access values
double x = loaded_point.x();
std::string name = loaded_point.name();
```

## Available Message Types

All defined in `session.proto`:

- **`session_proto::Point`** - 3D point with metadata
  - `guid`, `name`, `x`, `y`, `z`, `width`
  - Nested: `pointcolor` (Color), `xform` (Xform)

- **`session_proto::Color`** - RGBA color
  - `r`, `g`, `b`, `a` (0-255)

- **`session_proto::Vector`** - 3D vector
  - `x`, `y`, `z`

- **`session_proto::Xform`** - 4x4 transformation matrix
  - `matrix` (repeated double, 16 values)

- **`session_proto::Line`** - Line segment
  - `start`, `end` (Point)

- **`session_proto::Polyline`** - Connected points
  - `points` (repeated Point)
  - `guid`, `name`

## Testing

Run the protobuf tests:

```bash
cd build
./tests "[protobuf]"
```

Tests are in `src/session_proto_test.cpp` and demonstrate:
- Creating messages
- Serialization/deserialization
- File I/O
- All message types

## Benefits of Protobuf

1. **Efficient**: Compact binary format, smaller than JSON/XML
2. **Fast**: Optimized parsing and serialization
3. **Cross-language**: Same .proto file works in Python, Rust, etc.
4. **Versioning**: Forward/backward compatible with schema evolution
5. **Type-safe**: Strongly typed with validation

## Adding New Messages

To add new protobuf types:

1. Edit `src/session.proto` and add your message definition
2. Rebuild with `cmake --build build`
3. Generated code appears in `build/generated/session.pb.h`
4. Use the new message types in your code

Example:

```protobuf
message Mesh {
  string guid = 1;
  string name = 2;
  repeated Point vertices = 3;
  repeated int32 faces = 4;
}
```

## CMakeLists.txt Configuration

The CMakeLists.txt handles everything automatically:

```cmake
# Register proto files
append_proto_file(${CMAKE_CURRENT_SOURCE_DIR}/src/session.proto)

# Generate C++ code
CREATE_CPP_PROTO()
```

All targets (main executable, tests, examples) are automatically configured to:
- Include protobuf headers
- Link protobuf libraries
- Wait for code generation to complete

## Documentation

- [Protocol Buffers C++ Tutorial](https://protobuf.dev/getting-started/cpptutorial/)
- [Proto3 Language Guide](https://protobuf.dev/programming-guides/proto3/)
- [C++ API Reference](https://protobuf.dev/reference/cpp/api-docs/)

---

## Quick Reference

```bash
# Enable protobuf (default)
cmake -DENABLE_PROTOBUF=ON -B build
cmake --build build

# Disable protobuf
cmake -DENABLE_PROTOBUF=OFF -B build
cmake --build build

# Force rebuild external dependencies (Abseil + Protobuf)
rm -rf build/external/
cmake --build build

# Run protobuf tests
./build/tests "[protobuf]"
```

**Key Points:**
- ✅ Protobuf is **optional** - controlled by `ENABLE_PROTOBUF` flag
- ✅ External dependencies build **once** and are cached in `build/external/`
- ✅ Generated code in `build/generated/session.pb.{h,cc}`
- ✅ Only `.proto` changes trigger C++ regeneration
- ✅ All warnings from Abseil are non-fatal (removed `-Werror`)
