# Mini test overview

This repo has small cross-language mini tests for comparing implementations.

To add a new class (for example `Color`) to the mini-test + website pipeline:

1. **Python**  
   Add `@MINI_TEST("ClassName", "test_name")` functions in `session_py/<class>_test.py` and call `run_all(language="python")` from `if __name__ == "__main__"`.

2. **C++**  
   Add `MINI_TEST(ClassName, test_name)` blocks in `session_cpp/src/<class>_test.cpp`, exclude that file from Catch2 in `CMakeLists.txt`, and add it to the `point_minitest` (or similar) executable.

3. **Rust**  
   Extend `session_rust/src/bin/point_minitest.rs` (or a new bin) with functions that build `TestResult` values for the new class and write `<class>_test.json` into `session_tests/session_rust/`.

4. **session_tests**  
   Ensure `session_tests/minitest.sh` runs the new Python module and Rust/C++ binaries so the JSON files are produced.

5. **Website**  
   In `session_tests/website/index.html`, append the new JSON paths to the `sources` array and, if needed, add a new suite name so you can switch between `<class>_test` groups in the UI.

---

## Adding JSON Serialization

### Python
```python
def __jsondump__(self) -> dict:
    return {"type": "ClassName", "field": self.field, ...}

@classmethod
def __jsonload__(cls, data: dict) -> "ClassName":
    return cls(data["field"], ...)

def jsondump(self, filepath): json.dump(self.__jsondump__(), open(filepath, 'w'))
@classmethod
def jsonload(cls, filepath): return cls.__jsonload__(json.load(open(filepath)))
```

### C++
```cpp
// In header: declare methods
ordered_json jsondump() const;
static ClassName jsonload(const ordered_json& j);
void jsondump(const std::string& filename) const;
static ClassName jsonload(const std::string& filename);

// In cpp: implement serialization
ordered_json ClassName::jsondump() const {
    return {{"type", "ClassName"}, {"field", field}, ...};
}
```

### Rust
```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename = "ClassName")]
pub struct ClassName { ... }

pub fn jsondump(&self) -> Result<String, ...> { serde_json::to_string_pretty(self) }
pub fn jsonload(s: &str) -> Result<Self, ...> { serde_json::from_str(s) }
pub fn to_json(&self, path: &str) { std::fs::write(path, self.jsondump()?) }
pub fn from_json(path: &str) -> Self { Self::jsonload(&std::fs::read_to_string(path)?) }
```

---

## Adding Protobuf Serialization

### 1. Define Schema
Add `session_proto/<class>.proto`:
```proto
syntax = "proto3";
package session_proto;
message ClassName {
    string guid = 1;
    double field = 2;
}
```

### 2. Python
Generate bindings: `protoc --python_out=session_py/src/session_py/proto/ <class>.proto`  
Fix imports in generated file: `import x_pb2` → `from . import x_pb2`
```python
def to_protobuf(self):
    from .proto import class_pb2
    proto = class_pb2.ClassName()
    proto.field = self.field
    return proto.SerializeToString()

@classmethod
def from_protobuf(cls, data):
    proto = class_pb2.ClassName()
    proto.ParseFromString(data)
    return cls(proto.field)

def protobuf_dump(self, path): open(path, 'wb').write(self.to_protobuf())
@classmethod
def protobuf_load(cls, path): return cls.from_protobuf(open(path, 'rb').read())
```

### 3. C++
Generated `.pb.h/.pb.cc` must exist. Wrap with `#ifdef ENABLE_PROTOBUF`:
```cpp
#ifdef ENABLE_PROTOBUF
#include "class.pb.h"

std::string ClassName::to_protobuf() const {
    session_proto::ClassName proto;
    proto.set_field(field);
    return proto.SerializeAsString();
}

ClassName ClassName::from_protobuf(const std::string& data) {
    session_proto::ClassName proto;
    proto.ParseFromString(data);
    return ClassName(proto.field());
}
#endif
```

### 4. Rust
Proto files go in `session_rust/proto/`. Build script compiles them:
```rust
// build.rs
#[cfg(feature = "protobuf")]
prost_build::compile_protos(&["proto/class.proto"], &["proto/"]).unwrap();

// lib.rs
#[cfg(feature = "protobuf")]
pub mod proto { include!(concat!(env!("OUT_DIR"), "/session_proto.rs")); }

// class.rs
#[cfg(feature = "protobuf")]
pub fn to_protobuf(&self) -> Vec<u8> {
    use prost::Message;
    let proto = crate::proto::ClassName { field: self.field };
    proto.encode_to_vec()
}

#[cfg(feature = "protobuf")]
pub fn from_protobuf(data: &[u8]) -> Result<Self, ...> {
    use prost::Message;
    let proto = crate::proto::ClassName::decode(data)?;
    Ok(Self { field: proto.field })
}
```
