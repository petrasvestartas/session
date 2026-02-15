---
name: add_class
description: Add new geometry class to all three languages (C++, Python, Rust)
version: 1.0.0
author: session
type: skill
category: development
tags:
  - geometry
  - cpp
  - python
  - rust
  - implementation
---

# Add Geometry Class Skill

> **Purpose**: Add a new geometry class to all three language implementations with consistent API and tests.

---

## What I Do

- Create implementation files in C++, Python, and Rust
- Set up MINI_TEST tests for all three languages
- Register classes in build systems
- Verify tests pass in all languages

---

## Steps Overview

1. Create C++ implementation (ground truth)
2. Port to Python
3. Port to Rust
4. Create tests in all languages
5. Register in build systems
6. Run tests and verify

---

## Step 1: Create C++ Implementation

Create two files:

**`session_cpp/src/<name>.h`**
```cpp
#pragma once
#include <string>
#include "json.h"

namespace session_cpp {

class <Name> {
public:
    <Name>();
    // API methods...
    
    // JSON serialization
    nlohmann::ordered_json __jsondump__() const;
    void __jsonload__(const nlohmann::ordered_json& json);
};

} // namespace session_cpp
```

**`session_cpp/src/<name>.cpp`**
```cpp
#include "<name>.h"

namespace session_cpp {

// Implementation...

} // namespace session_cpp
```

---

## Step 2: Create Python Implementation

**`session_py/src/session_py/<name>.py`**
```python
class <Name>:
    def __init__(self, ...):
        # Match C++ API exactly
        pass
    
    def __jsondump__(self) -> dict:
        # Return dict with alphabetical keys
        return {...}
    
    def __jsonload__(self, data: dict):
        # Load from dict
        pass
```

---

## Step 3: Create Rust Implementation

**`session_rust/src/<name>.rs`**
```rust
use serde::{Serialize, Deserialize};

pub struct <Name> {
    // Match C++ API exactly
}

impl Serialize for <Name> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        // serde_json::json! outputs alphabetically
        serializer.serialize_some(&serde_json::json!({...}))
    }
}
```

---

## Step 4: Create Tests (All Languages)

### C++: `session_cpp/src/<name>_test.cpp`

```cpp
#include "mini_test.h"

MINI_TEST("<Name>", "Constructor") {
    <Name> obj;
    MINI_CHECK(obj.IsValid());
}

MINI_TEST("<Name>", "Json_roundtrip") {
    <Name> obj;
    auto json = obj.__jsondump__();
    <Name> obj2;
    obj2.__jsonload__(json);
    MINI_CHECK(obj == obj2);
}
```

### Python: `session_py/src/session_py/<name>_test.py`

```python
from session_py import <Name>
from session_py.mini_test import MINI_TEST, MINI_CHECK

@MINI_TEST("<Name>", "Constructor")
def test_<name>_constructor():
    obj = <Name>()
    MINI_CHECK(obj.is_valid())

@MINI_TEST("<Name>", "Json_roundtrip")
def test_<name>_json():
    obj = <Name>()
    data = obj.__jsondump__()
    obj2 = <Name>()
    obj2.__jsonload__(data)
    MINI_CHECK(obj == obj2)
```

### Rust: `session_rust/src/<name>_test.rs`

```rust
use crate::{MINI_CHECK, MINI_TEST, REGISTER_MINI_TEST};

MINI_TEST!("Constructor", {
    let obj = <Name>::new();
    MINI_CHECK!(obj.is_valid());
});

REGISTER_MINI_TEST!("<Name>", "Constructor", crate::<name>_test::<name>_constructor);
```

---

## Step 5: Register in Build Systems

### C++: Update `session_cpp/CMakeLists.txt`

Add to `MINITEST_SOURCES`:
```
src/<name>_test.cpp
```

### Rust: Update `session_rust/src/lib.rs`

```rust
pub mod <name>;
pub mod <name>_test;
```

### Shell: Update `bash/lib/common.sh`

Add to `CLASS_NAMES` array:
```bash
CLASS_NAMES=(... "<name>")
```

---

## Step 6: Run Tests

```bash
# Full suite
./bash/minitest.sh

# Or single language during development
./bash/test_py.sh <name>
./bash/test_rust.sh
./bash/test_cpp.sh
```

---

## Required API Methods

All geometry classes must implement:

| Method | Description |
|--------|-------------|
| Constructor | With default parameters |
| GUID/Name | Metadata fields |
| duplicate() / clone() | Copy with new GUID |
| `[]` operator | Component access |
| `==` / `!=` | Equality |
| `__str__` / to_string | String representation |
| `__jsondump__` / json_dump | JSON dict conversion |
| json_load | JSON file I/O |
| to_proto / from_proto | Protobuf serialization |

### Visual Classes (Point, Line, Plane, Polyline)

Additional methods:
- width, color, xform fields
- transform() / transformed()

---

## Code Style Checklist

- [ ] Alphabetical JSON field ordering
- [ ] Each Python import on separate line
- [ ] C++ uses `std::cout << obj`
- [ ] Rust uses `println!("{}", obj)`
- [ ] C++ tolerance.h in tests only
- [ ] All tests in all 3 languages

---

## File Locations

| Component | Path |
|-----------|------|
| C++ Source | `session_cpp/src/<name>.h`, `.cpp` |
| Python Source | `session_py/src/session_py/<name>.py` |
| Rust Source | `session_rust/src/<name>.rs` |
| C++ Test | `session_cpp/src/<name>_test.cpp` |
| Python Test | `session_py/src/session_py/<name>_test.py` |
| Rust Test | `session_rust/src/<name>_test.rs` |
| CMakeLists | `session_cpp/CMakeLists.txt` |
| lib.rs | `session_rust/src/lib.rs` |
| common.sh | `bash/lib/common.sh` |

---

**Add Class Skill** - Implement geometry classes across C++, Python, and Rust
