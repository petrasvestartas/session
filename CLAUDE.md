# CLAUDE.md

Multi-language geometry kernel (Python, C++, Rust) with shared protobuf schemas and Vue test viewer.
--dangerously-skip-permissions

## Structure
```
session_cpp/     # C++ (submodule)
session_py/      # Python (submodule)
session_rust/    # Rust (submodule)
session_proto/   # Protobuf schemas (submodule)
session_tests/   # Vue 3 test viewer
```

## Build

```bash
./build.sh                    # All languages
```

**C++:**
```bash
cd session_cpp && mkdir -p build && cd build && cmake .. && make tests -j$(nproc)
```

**Python:**
```bash
uv venv uvsession --python 3.11 && source uvsession/bin/activate
cd session_py && uv pip install -e . && pytest -v
```

**Rust:**
```bash
cd session_rust && cargo build --release && cargo test
```

## Test Viewer

```bash
./minitest.sh                 # Run all tests + launch viewer at localhost:8769
```

## Add New Test Suite (e.g., bbox)

### 1. minitest.sh - Add to these arrays:
- `cleanup_json()` - add JSON paths
- Python test execution section
- Rust test execution section
- `SOURCES` array in `generate_test_data_js()`
- `ARTIFACTS` array (optional)

### 2. Create test files:

| Language | File | Template |
|----------|------|----------|
| Python | `session_py/src/session_py/bbox_test.py` | Use `@MINI_TEST` decorator |
| C++ | `session_cpp/src/bbox_test.cpp` | Use `MINI_TEST` macro |
| Rust | `session_rust/src/bbox_test.rs` | Use `MINI_TEST!` macro |

### 3. Wire up:
- **C++**: Add `bbox_test.cpp` to `MINITEST_SOURCES` in `CMakeLists.txt`
- **Rust**: Add `pub mod bbox_test;` to `lib.rs`, create `src/bin/bbox_minitest.rs`

### 4. MCP/Browser index - Add files to `SOURCE_FILES` in:
- `session_tests/mcp/session_api_server.py`
- `session_tests/mcp/generate_browser_index.py`

### 5. Run
```bash
./minitest.sh
cd session_tests/mcp && python3 generate_browser_index.py  # Regenerate search index
```

## MCP Server (Claude Desktop)

Config: `~/.config/Claude/claude_desktop_config.json`
```json
{
  "mcpServers": {
    "session-api": {
      "command": "python3",
      "args": ["/path/to/session/session_tests/mcp/session_api_server.py"]
    }
  }
}
```

Tools available to Claude:
- `search_api(query)` - Search methods across all languages
- `get_method(name, language)` - Get method implementation
- `list_classes()` - List all classes

## Git

```bash
git clone --recurse-submodules <url>            # Clone
git submodule update --init --recursive         # Update submodules
./git_push.sh "message"                         # Push all
```

## Notes

- Python venv: `uvsession/` at repo root
- Viewer port: 8769
- C++ requires C++23
- Protobuf bindings auto-generated during build

## Test Consistency Patterns

Tests across Python, C++, and Rust must be **EXACTLY identical** in structure, line count, and variable names.

### Import Style Rule

**IMPORTANT:** All imports must be on separate lines. Never combine multiple imports on one line.

**Wrong:**
```python
from session_py import Line, Point, Vector, Color  # BAD - multiple imports on one line
```

**Correct:**
```python
from session_py import Line
from session_py import Point
from session_py import Vector
from session_py import Color
```

This applies to:
- Test files inside test functions (after `@MINI_TEST` decorator)
- Python module imports at file top

### Test File Structure

**Python** (`session_py/src/session_py/<class>_test.py`):
```python
from .mini_test import MINI_TEST, MINI_CHECK, run_all
from .tolerance import TOLERANCE

@MINI_TEST("ClassName", "test_name")
def test_class_test_name():
    from session_py import ClassName
    # test body...
    MINI_CHECK(condition)

if __name__ == "__main__":
    run_all("python")
```

**C++** (`session_cpp/src/<class>_test.cpp`):
```cpp
#include "mini_test.h"
#include "classname.h"
#include "tolerance.h"
#include <cmath>

using namespace session_cpp::mini_test;

namespace session_cpp {

MINI_TEST("ClassName", "test_name") {
    // uncomment #include "classname.h"
    // test body...
    MINI_CHECK(condition);
}

} // namespace session_cpp
```

**Rust** (`session_rust/src/<class>_test.rs`):
```rust
use crate::{MINI_CHECK, MINI_TEST, REGISTER_MINI_TEST};
use crate::mini_test::TestResult;
use crate::tolerance::TOLERANCE;

pub fn run_classname_test_name() -> TestResult {
    MINI_TEST!("test_name", {
        use crate::ClassName;
        // test body...
        MINI_CHECK!(condition);
    })
}

REGISTER_MINI_TEST!("ClassName", "test_name", crate::classname_test::run_classname_test_name);
```

### C++ Include Comments (`// uncomment`)

C++ tests use `// uncomment #include "..."` comments to document which headers are required for each individual test. These comments serve as documentation for the test viewer, showing users what includes they need.

**Pattern:** Each test function has `// uncomment` comments listing ALL headers used in that test:
```cpp
MINI_TEST("Vector", "constructor") {
    // uncomment #include "vector.h"
    // uncomment #include "point.h"

    Vector v(1.0, 2.0, 3.0);
    Point p0(1.0, 2.0, 3.0);
    // ...
}

MINI_TEST("Vector", "normalize") {
    // uncomment #include "vector.h"
    // uncomment #include "tolerance.h"

    Vector v0(3.0, 4.0, 0.0);
    v0.normalize_self();
    MINI_CHECK(TOLERANCE.is_close(v0.magnitude(), 1.0));
}

MINI_TEST("Vector", "sum_of_vectors") {
    // uncomment #include "vector.h"
    // uncomment #include <vector>

    std::vector<Vector> vecs = { ... };
}
```

**Rules:**
1. List ALL headers needed for that specific test function
2. Include both project headers (`"vector.h"`) and standard library headers (`<vector>`, `<cmath>`)
3. The actual `#include` directives are at the top of the file - these comments are for documentation
4. Order: project headers first, then standard library headers

### Variable Consistency
If C++ requires a variable (e.g., reference parameter), **all languages must have it**:
```cpp
// C++: get_leveled_vector takes double& reference
double vertical_height = 1.0;
Vector v_leveled = v.get_leveled_vector(vertical_height);
```
```python
# Python: same variable for consistency
vertical_height = 1.0
v_leveled = v.get_leveled_vector(vertical_height)
```
```rust
// Rust: same variable for consistency
let vertical_height = 1.0;
let v_leveled = v.get_leveled_vector(vertical_height);
```

### Comment Formatting
Use **ASCII characters** in comments (not Unicode):
- Use `.` not `*` for dot product
- Use `^2` not `^2` for squared
- Use `deg` not `deg` for degrees
- Use `alpha, beta, gamma` not `a, ss, ?`

### Blank Line Consistency
Each test block should have the same number of blank lines in all languages.

### Serialization File Paths
- **Python**: Use `Path(__file__).resolve().parents[2] / "filename.json"` to save to `session_py/`
- **C++/Rust**: Use simple filename `"filename.json"` (runs from their directories)

### MINI_CHECK Assertions
Keep assertions on separate lines with same logic:
```python
MINI_CHECK(x == 1.0 and y == 2.0 and z == 3.0)
```
```cpp
MINI_CHECK(x == 1.0 && y == 2.0 && z == 3.0);
```
```rust
MINI_CHECK!(x == 1.0 && y == 2.0 && z == 3.0);
```

### Floating Point Comparisons
Use `TOLERANCE.is_close()` instead of rounding:
```python
# GOOD - use is_close
MINI_CHECK(TOLERANCE.is_close(d, 3.741657))

# BAD - don't use rounding
d = round(value, Tolerance.ROUNDING)
MINI_CHECK(d == 3.741657)
```
```cpp
// GOOD - use is_close
MINI_CHECK(TOLERANCE.is_close(d, 3.741657));

// BAD - don't use rounding
double d = Tolerance::round_to(value, Tolerance::ROUNDING);
MINI_CHECK(d == 3.741657);
```
```rust
// GOOD - use is_close
use crate::tolerance::TOLERANCE;
MINI_CHECK!(TOLERANCE.is_close(d, 3.741657));

// BAD - don't use rounding
let d = Tolerance::round_to(value, Tolerance::ROUNDING);
MINI_CHECK!(d == 3.741657);
```

### Test Function Content Must Match Across Languages

Every test function across Python, C++, and Rust must have:
1. **Same test names** - `@MINI_TEST("Vector", "constructor")` in all languages
2. **Same variable names** - `v`, `vcopy`, `vother`, `result_mul`, etc.
3. **Same comments** - identical comment text for each section
4. **Same operations in same order** - constructor, setters, getters, etc.
5. **Same MINI_CHECK assertions** - identical logic in each check

### Language-Specific Syntax Mapping

| Concept | Python | C++ | Rust |
|---------|--------|-----|------|
| Logical AND | `and` | `&&` | `&&` |
| Logical OR | `or` | `\|\|` | `\|\|` |
| Logical NOT | `not` | `!` | `!` |
| Index access | `v[0]` | `v[0]` | `v[0]` |
| Method call | `v.method()` | `v.method()` | `v.method()` |
| Static method | `Class.method()` | `Class::method()` | `Class::method()` |
| Constructor | `Vector(1.0, 2.0, 3.0)` | `Vector(1.0, 2.0, 3.0)` | `Vector::new(1.0, 2.0, 3.0)` |
| sqrt | `math.sqrt(x)` | `std::sqrt(x)` | `x.sqrt()` or `(x).sqrt()` |
| pow | `x**2` | `x*x` | `x.powi(2)` |
| String concat | `"text"` | `"text"` | `"text"` |
| Tuple unpack | `a, b, c = func()` | `auto [a, b, c] = func()` | `let (a, b, c) = func()` |
| List/Vector | `[v1, v2, v3]` | `std::vector<T>{v1, v2, v3}` | `vec![v1, v2, v3]` |

### Rust Reference-Based Operators

In Rust, operators that consume `self` prevent reusing the variable. To match Python/C++ "copy operators" pattern, implement operators for `&Type`:

```rust
// Implement for owned value (consumes self)
impl Mul<f64> for Line {
    type Output = Line;
    fn mul(self, factor: f64) -> Line { ... }
}

// Implement for reference (creates copy, keeps original)
impl Mul<f64> for &Line {
    type Output = Line;
    fn mul(self, factor: f64) -> Line {
        self.clone() * factor
    }
}
```

**Test code comparison:**
```python
# Python - variable reusable
rmul = l * 2.0
rdiv = l / 2.0
```
```cpp
// C++ - variable reusable
Line rmul = l * 2.0;
Line rdiv = l / 2.0;
```
```rust
// Rust - use & to keep variable usable
let rmul = &l * 2.0;
let rdiv = &l / 2.0;
```

### Required Methods for Geometry Classes

Every geometry class (Point, Vector, Line, Color, etc.) must implement these methods consistently:

| Method | Python | C++ | Rust |
|--------|--------|-----|------|
| Constructor | `__init__` | Constructor | `new()` |
| Named constructor | `with_name()` class method | `with_name()` static | `with_name()` |
| Duplicate | `duplicate()` | Copy constructor | `duplicate()` (new guid) |
| Short string | `__str__` | `str()` | `str()` |
| Full string | `__repr__` | `repr()` | `repr()` |
| Equality | `__eq__` | `operator==` | `impl PartialEq` |
| JSON dump | `json_dump(path)` | `json_dump(path)` | `json_dump(path)` |
| JSON load | `json_load(path)` classmethod | `json_load(path)` static | `json_load(path)` |
| Protobuf dump | `protobuf_dump(path)` | `protobuf_dump(path)` | `protobuf_dump(path)` |
| Protobuf load | `protobuf_load(path)` classmethod | `protobuf_load(path)` static | `protobuf_load(path)` |
| Transform in-place | `transform()` | `transform()` | `transform()` |
| Transform copy | `transformed()` | `transformed()` | `transformed()` |

### Rust PartialEq Pattern

Implement `PartialEq` manually for floating-point tolerance comparison:

```rust
impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && (self._x0 * 1000000.0).round() == (other._x0 * 1000000.0).round()
            && (self._y0 * 1000000.0).round() == (other._y0 * 1000000.0).round()
            // ... all coordinate fields
            && self.linecolor == other.linecolor
    }
}
```

### Example: Tolerance Test (Minimal)

**Python:**
```python
@MINI_TEST("Tolerance", "is_zero")
def test_tolerance_is_zero():
    result = TOLERANCE.is_zero(1e-10)
    MINI_CHECK(result == True)
```

**C++:**
```cpp
MINI_TEST("Tolerance", "is_zero") {
    bool result = TOLERANCE.is_zero(1e-10);
    MINI_CHECK(result == true);
}
```

**Rust:**
```rust
pub fn run_tolerance_is_zero() -> TestResult {
    MINI_TEST!("is_zero", {
        let result = TOLERANCE.is_zero(1e-10);
        MINI_CHECK!(result == true);
    })
}
```

### Example: Color Test (Constructor Pattern)

**Python:**
```python
@MINI_TEST("Color", "constructor")
def test_color_constructor():
    from session_py import Color

    # Constructor
    red = Color(255, 0, 0, 255, "red")

    # Setters
    red[0] = 255
    red[1] = 0
    red[2] = 0
    red[3] = 255

    # Getters
    r = red[0]
    g = red[1]
    b = red[2]
    a = red[3]

    # Minimal and Full String Representation
    cstr = str(red)
    crepr = repr(red)

    # Copy (duplicates everything except guid)
    ccopy = red.duplicate()
    cother = Color(255, 0, 0, 255, "red")

    MINI_CHECK(red.name == "red" and red.guid != "" and red[0] == 255 and red[1] == 0 and red[2] == 0 and red[3] == 255 and red.guid)
    MINI_CHECK(r == 255 and g == 0 and b == 0 and a == 255)
    MINI_CHECK(cstr == "255, 0, 0, 255")
    MINI_CHECK(crepr == "Color(red, 255, 0, 0, 255)")
    MINI_CHECK(ccopy == cother)
    MINI_CHECK(ccopy.guid != red.guid)
```

**C++:**
```cpp
MINI_TEST("Color", "constructor"){
    // uncomment #include "color.h"

    // Constructor
    Color c(255, 0, 0, 255, "red");

    // Setters
    c[0] = 255;
    c[1] = 0;
    c[2] = 0;
    c[3] = 255;

    // Getters
    int r = c[0];
    int g = c[1];
    int b = c[2];
    int a = c[3];

    // Minimal and Full String Representation
    std::string cstr = c.str();
    std::string crepr = c.repr();

    // Copy (duplicates everything except guid)
    Color ccopy = c;
    Color cother(255, 0, 0, 255, "red");

    MINI_CHECK(c.name == "red" && c.guid != "" && c[0] == 255 && c[1] == 0 && c[2] == 0 && c[3] == 255);
    MINI_CHECK(r == 255 && g == 0 && b == 0 && a == 255);
    MINI_CHECK(cstr == "255, 0, 0, 255");
    MINI_CHECK(crepr == "Color(red, 255, 0, 0, 255)");
    MINI_CHECK(ccopy == cother);
    MINI_CHECK(ccopy.guid != c.guid);
}
```

**Rust:**
```rust
pub fn run_color_constructor() -> TestResult {
    MINI_TEST!("constructor", {
        use crate::Color;

        // Constructor
        let mut red = Color::new(255, 0, 0, 255);
        red.name = "red".to_string();

        // Setters
        red.r = 255;
        red.g = 0;
        red.b = 0;
        red.a = 255;

        // Getters
        let r = red.r;
        let g = red.g;
        let b = red.b;
        let a = red.a;

        // Minimal and Full String Representation
        let cstr = red.str();
        let crepr = red.repr();

        // Copy (duplicates everything except guid)
        let ccopy = red.duplicate();
        let mut cother = Color::new(255, 0, 0, 255);
        cother.name = "red".to_string();

        MINI_CHECK!(red.name == "red" && !red.guid.is_empty() && red.r == 255 && red.g == 0 && red.b == 0 && red.a == 255);
        MINI_CHECK!(r == 255 && g == 0 && b == 0 && a == 255);
        MINI_CHECK!(cstr == "255, 0, 0, 255");
        MINI_CHECK!(crepr == "Color(red, 255, 0, 0, 255)");
        MINI_CHECK!(ccopy == cother);
        MINI_CHECK!(ccopy.guid != red.guid);
    })
}
```

### Example: Vector Test (Math Operations)

**Python:**
```python
@MINI_TEST("Vector", "dot_product")
def test_vector_dot_product():
    from session_py import Vector

    # Orthogonality and parallelism via dot product
    # Perpendicular vectors are close to 0.0
    # Parallel vectors are close to 1.0
    v1 = Vector(1.0, 0.0, 0.0)
    v2 = Vector(0.0, 1.0, 0.0)
    v3 = Vector(1.0, 0.0, 0.0)
    dot_perp = v1.dot(v2)
    dot_paral = v1.dot(v3)

    # Projection of a onto b
    # Scalar projection:
    # (a . b) / ||b|| (here ||b||=1, so just a_x = 3.0)
    # Projection coefficient:
    # (a . b) / ||b||^2 = 6/4 = 1.5 (how many b2's fit in projection)
    a = Vector(3.0, 4.0, 0.0)
    b = Vector(1.0, 0.0, 0.0)
    b2 = Vector(2.0, 0.0, 0.0)
    proj_scalar = a.dot(b) / math.sqrt(b[0]**2 + b[1]**2 + b[2]**2)
    proj_coeff = a.dot(b2) / (b2[0]**2 + b2[1]**2 + b2[2]**2)

    MINI_CHECK(TOLERANCE.is_close(dot_perp, 0.0))
    MINI_CHECK(TOLERANCE.is_close(dot_paral, 1.0))
    MINI_CHECK(TOLERANCE.is_close(proj_scalar, 3.0))
    MINI_CHECK(TOLERANCE.is_close(proj_coeff, 1.5))
```

**C++:**
```cpp
MINI_TEST("Vector", "dot_product") {
    // uncomment #include "vector.h"

    // Orthogonality and parallelism via dot product
    // Perpendicular vectors are close to 0.0
    // Parallel vectors are close to 1.0
    Vector v1(1.0, 0.0, 0.0);
    Vector v2(0.0, 1.0, 0.0);
    Vector v3(1.0, 0.0, 0.0);
    double dot_perp = v1.dot(v2);
    double dot_paral = v1.dot(v3);

    // Projection of a onto b
    // Scalar projection:
    // (a . b) / ||b|| (here ||b||=1, so just a_x = 3.0)
    // Projection coefficient:
    // (a . b) / ||b||^2 = 6/4 = 1.5 (how many b2's fit in projection)
    Vector a(3.0, 4.0, 0.0);
    Vector b(1.0, 0.0, 0.0);
    Vector b2(2.0, 0.0, 0.0);
    double proj_scalar = a.dot(b) / std::sqrt(b[0]*b[0] + b[1]*b[1] + b[2]*b[2]);
    double proj_coeff = a.dot(b2) / (b2[0]*b2[0] + b2[1]*b2[1] + b2[2]*b2[2]);

    MINI_CHECK(TOLERANCE.is_close(dot_perp, 0.0));
    MINI_CHECK(TOLERANCE.is_close(dot_paral, 1.0));
    MINI_CHECK(TOLERANCE.is_close(proj_scalar, 3.0));
    MINI_CHECK(TOLERANCE.is_close(proj_coeff, 1.5));
}
```

**Rust:**
```rust
pub fn run_vector_dot_product() -> TestResult {
    MINI_TEST!("dot_product", {
        use crate::Vector;

        // Orthogonality and parallelism via dot product
        // Perpendicular vectors are close to 0.0
        // Parallel vectors are close to 1.0
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let v3 = Vector::new(1.0, 0.0, 0.0);
        let dot_perp = v1.dot(&v2);
        let dot_paral = v1.dot(&v3);

        // Projection of a onto b
        // Scalar projection:
        // (a . b) / ||b|| (here ||b||=1, so just a_x = 3.0)
        // Projection coefficient:
        // (a . b) / ||b||^2 = 6/4 = 1.5 (how many b2's fit in projection)
        let a = Vector::new(3.0, 4.0, 0.0);
        let b = Vector::new(1.0, 0.0, 0.0);
        let b2 = Vector::new(2.0, 0.0, 0.0);
        let proj_scalar = a.dot(&b) / (b[0].powi(2) + b[1].powi(2) + b[2].powi(2)).sqrt();
        let proj_coeff = a.dot(&b2) / (b2[0].powi(2) + b2[1].powi(2) + b2[2].powi(2));

        MINI_CHECK!(TOLERANCE.is_close(dot_perp, 0.0));
        MINI_CHECK!(TOLERANCE.is_close(dot_paral, 1.0));
        MINI_CHECK!(TOLERANCE.is_close(proj_scalar, 3.0));
        MINI_CHECK!(TOLERANCE.is_close(proj_coeff, 1.5));
    })
}
```

### JSON Serialization Test Pattern

**IMPORTANT:** Use class methods `json_dump()` and `json_load()`, NOT encoder imports.

**Wrong:**
```python
from session_py.encoders import json_dump, json_load  # BAD - don't use encoder imports
json_dump(obj, path)
loaded = json_load(path)
```

**Correct:**
```python
obj.json_dump(path)                # Instance method
loaded = ClassName.json_load(path)  # Class method
```

**Python:**
```python
@MINI_TEST("Line", "json_roundtrip")
def test_line_json_roundtrip():
    from session_py import Line
    from pathlib import Path

    l = Line(42.1, 84.2, 126.3, 168.4, 210.5, 252.6)
    l.name = "test_line"

    # json_dump(fname) / json_load(fname) - file-based serialization
    fname = Path(__file__).resolve().parents[2] / "test_line.json"
    l.json_dump(fname)
    loaded = Line.json_load(fname)

    MINI_CHECK(loaded.name == "test_line")
    MINI_CHECK(TOLERANCE.is_close(loaded[0], 42.1))
    MINI_CHECK(TOLERANCE.is_close(loaded[1], 84.2))
    MINI_CHECK(TOLERANCE.is_close(loaded[2], 126.3))
    MINI_CHECK(TOLERANCE.is_close(loaded[3], 168.4))
    MINI_CHECK(TOLERANCE.is_close(loaded[4], 210.5))
    MINI_CHECK(TOLERANCE.is_close(loaded[5], 252.6))
```

**C++:**
```cpp
MINI_TEST("Line", "json_roundtrip") {
    // uncomment #include "line.h"

    Line line(42.1, 84.2, 126.3, 168.4, 210.5, 252.6);
    line.name = "test_line";

    // json_dump(filename) / json_load(filename) - file-based serialization
    std::string filename = "test_line.json";
    line.json_dump(filename);
    Line loaded = Line::json_load(filename);

    MINI_CHECK(loaded.name == "test_line");
    MINI_CHECK(TOLERANCE.is_close(loaded[0], 42.1));
    MINI_CHECK(TOLERANCE.is_close(loaded[1], 84.2));
    MINI_CHECK(TOLERANCE.is_close(loaded[2], 126.3));
    MINI_CHECK(TOLERANCE.is_close(loaded[3], 168.4));
    MINI_CHECK(TOLERANCE.is_close(loaded[4], 210.5));
    MINI_CHECK(TOLERANCE.is_close(loaded[5], 252.6));
}
```

**Rust:**
```rust
pub fn run_line_json_roundtrip() -> TestResult {
    MINI_TEST!("json_roundtrip", {
        use crate::Line;

        let mut line = Line::new(42.1, 84.2, 126.3, 168.4, 210.5, 252.6);
        line.name = "test_line".to_string();

        // json_dump(filename) / json_load(filename) - file-based serialization
        let filename = "test_line.json";
        line.json_dump(filename).unwrap();
        let loaded = Line::json_load(filename).unwrap();

        MINI_CHECK!(loaded.name == "test_line");
        MINI_CHECK!(TOLERANCE.is_close(loaded[0], 42.1));
        MINI_CHECK!(TOLERANCE.is_close(loaded[1], 84.2));
        MINI_CHECK!(TOLERANCE.is_close(loaded[2], 126.3));
        MINI_CHECK!(TOLERANCE.is_close(loaded[3], 168.4));
        MINI_CHECK!(TOLERANCE.is_close(loaded[4], 210.5));
        MINI_CHECK!(TOLERANCE.is_close(loaded[5], 252.6));
    })
}
```

### Protobuf Serialization Test Pattern

Protobuf tests require feature flags. The test structure is identical across languages but wrapped in conditional compilation.

**Python:**
```python
@MINI_TEST("Line", "protobuf_roundtrip")
def test_line_protobuf_roundtrip():
    from session_py import Line
    from pathlib import Path

    line = Line(42.1, 84.2, 126.3, 168.4, 210.5, 252.6)
    line.name = "test_line"

    # protobuf_dump(filename) / protobuf_load(filename) - file-based serialization
    path = Path(__file__).resolve().parents[2] / "test_line.bin"
    line.protobuf_dump(path)
    loaded = Line.protobuf_load(path)

    MINI_CHECK(loaded.name == "test_line")
    MINI_CHECK(TOLERANCE.is_close(loaded[0], 42.1))
    MINI_CHECK(TOLERANCE.is_close(loaded[1], 84.2))
    MINI_CHECK(TOLERANCE.is_close(loaded[2], 126.3))
    MINI_CHECK(TOLERANCE.is_close(loaded[3], 168.4))
    MINI_CHECK(TOLERANCE.is_close(loaded[4], 210.5))
    MINI_CHECK(TOLERANCE.is_close(loaded[5], 252.6))
```

**C++:**
```cpp
#ifdef ENABLE_PROTOBUF
MINI_TEST("Line", "protobuf_roundtrip") {
    // uncomment #include "line.h"

    Line line(42.1, 84.2, 126.3, 168.4, 210.5, 252.6);
    line.name = "test_line";

    // protobuf_dump(filename) / protobuf_load(filename) - file-based serialization
    std::string filename = "test_line.bin";
    line.protobuf_dump(filename);
    Line loaded = Line::protobuf_load(filename);

    MINI_CHECK(loaded.name == "test_line");
    MINI_CHECK(TOLERANCE.is_close(loaded[0], 42.1));
    MINI_CHECK(TOLERANCE.is_close(loaded[1], 84.2));
    MINI_CHECK(TOLERANCE.is_close(loaded[2], 126.3));
    MINI_CHECK(TOLERANCE.is_close(loaded[3], 168.4));
    MINI_CHECK(TOLERANCE.is_close(loaded[4], 210.5));
    MINI_CHECK(TOLERANCE.is_close(loaded[5], 252.6));
}
#endif
```

**Rust:**
```rust
#[cfg(feature = "protobuf")]
pub fn run_line_protobuf_roundtrip() -> TestResult {
    MINI_TEST!("protobuf_roundtrip", {
        use crate::Line;

        let mut line = Line::new(42.1, 84.2, 126.3, 168.4, 210.5, 252.6);
        line.name = "test_line".to_string();

        // protobuf_dump(filename) / protobuf_load(filename) - file-based serialization
        let filename = "test_line.bin";
        line.protobuf_dump(filename);
        let loaded = Line::protobuf_load(filename);

        MINI_CHECK!(loaded.name == "test_line");
        MINI_CHECK!(TOLERANCE.is_close(loaded[0], 42.1));
        MINI_CHECK!(TOLERANCE.is_close(loaded[1], 84.2));
        MINI_CHECK!(TOLERANCE.is_close(loaded[2], 126.3));
        MINI_CHECK!(TOLERANCE.is_close(loaded[3], 168.4));
        MINI_CHECK!(TOLERANCE.is_close(loaded[4], 210.5));
        MINI_CHECK!(TOLERANCE.is_close(loaded[5], 252.6));
    })
}

// Registration also needs feature flag
#[cfg(feature = "protobuf")]
REGISTER_MINI_TEST!("Line", "protobuf_roundtrip", crate::line_test::run_line_protobuf_roundtrip);
```

### Protobuf Feature Flags Summary

| Language | Compile Flag | Code Guard |
|----------|-------------|------------|
| Python | N/A (always available) | None |
| C++ | `-DENABLE_PROTOBUF=ON` | `#ifdef ENABLE_PROTOBUF ... #endif` |
| Rust | `--features protobuf` | `#[cfg(feature = "protobuf")]` |

## Serialization API

All geometry objects support JSON and Protobuf serialization with consistent APIs across languages.

### JSON Serialization

| Operation | Python | C++ | Rust |
|-----------|--------|-----|------|
| Dump to file | `json_dump(obj, path)` | `obj.json_dump(filename)` | `obj.json_dump(filename).unwrap()` |
| Load from file | `json_load(path)` | `Class::json_load(filename)` | `Class::json_load(filename).unwrap()` |
| Dump to string | `json_dumps(obj)` | `obj.jsondump().dump()` | `obj.jsondump().unwrap()` |
| Load from string | `json_loads(s)` | `Class::jsonload(json)` | `Class::jsonload(&s).unwrap()` |

**Python:**
```python
from session_py.encoders import json_dump, json_load
from pathlib import Path

# File-based
path = Path(__file__).resolve().parents[2] / "test_vector.json"
json_dump(v, path)
loaded = json_load(path)

# String-based (internal methods)
json_str = v.__jsondump__()  # Returns dict, use json.dumps() for string
loaded = Vector.__jsonload__(data, guid, name)
```

**C++:**
```cpp
// File-based
std::string filename = "test_vector.json";
v.json_dump(filename);
Vector loaded = Vector::json_load(filename);

// String-based
nlohmann::json j = v.jsondump();
Vector loaded = Vector::jsonload(j);
```

**Rust:**
```rust
// File-based
let filename = "test_vector.json";
v.json_dump(filename).unwrap();
let loaded = Vector::json_load(filename).unwrap();

// String-based
let json_str = v.jsondump().unwrap();
let loaded = Vector::jsonload(&json_str).unwrap();
```

### Protobuf Serialization

| Operation | Python | C++ | Rust |
|-----------|--------|-----|------|
| Dump to file | `obj.protobuf_dump(path)` | `obj.protobuf_dump(filename)` | `obj.protobuf_dump(filename)` |
| Load from file | `Class.protobuf_load(path)` | `Class::protobuf_load(filename)` | `Class::protobuf_load(filename)` |
| To bytes | `obj.to_protobuf()` | `obj.to_protobuf()` | `obj.to_protobuf()` |
| From bytes | `Class.from_protobuf(data)` | `Class::from_protobuf(data)` | `Class::from_protobuf(&data)` |

**Python:**
```python
from pathlib import Path

path = Path(__file__).resolve().parents[2] / "test_vector.bin"
v.protobuf_dump(path)
loaded = Vector.protobuf_load(path)
```

**C++:**
```cpp
std::string filename = "test_vector.bin";
v.protobuf_dump(filename);
Vector loaded = Vector::protobuf_load(filename);
```

**Rust:**
```rust
let filename = "test_vector.bin";
v.protobuf_dump(filename);
let loaded = Vector::protobuf_load(filename);
```

### Protobuf Feature Flags

Protobuf is optional and requires feature flags:

- **C++**: Compile with `-DENABLE_PROTOBUF=ON`, tests wrapped in `#ifdef ENABLE_PROTOBUF`
- **Rust**: Enable `protobuf` feature, tests annotated with `#[cfg(feature = "protobuf")]`
- **Python**: Always available (protobuf package required)

### JSON Format

All objects serialize to JSON with a `type` field for polymorphic deserialization:
```json
{
    "type": "Vector",
    "guid": "550e8400-e29b-41d4-a716-446655440000",
    "name": "my_vector",
    "x": 1.0,
    "y": 2.0,
    "z": 3.0
}
```

The `type` field enables automatic class detection during deserialization.

## Polyline Optimization Plan

### Problem: Bloated Serialization

Current Polyline stores points as full `Point` objects, each with:
- guid, name, x, y, z, width, pointcolor (with its own guid, name, r, g, b, a), xform (with its own guid, name, m[16])

This creates extremely verbose JSON:
```json
{
  "type": "Polyline",
  "points": [
    {
      "type": "Point",
      "guid": "...",
      "name": "my_point",
      "x": 1, "y": 2, "z": 3,
      "width": 1,
      "pointcolor": { "type": "Color", "guid": "...", "r": 0, "g": 0, "b": 255, "a": 255 },
      "xform": { "type": "Xform", "guid": "...", "m": [1,0,0,0,...] }
    },
    // ... repeated for each point
  ]
}
```

### Solution: Store Coordinates as Flat Array

Polyline should store coordinates as a flat `Vec<f64>` / `std::vector<double>` / `list[float]`:

```json
{
  "type": "Polyline",
  "guid": "...",
  "name": "test_polyline",
  "coords": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
  "width": 1.0,
  "linecolor": { "type": "Color", "guid": "...", "r": 255, "g": 255, "b": 255, "a": 255 },
  "xform": { "type": "Xform", "guid": "...", "m": [1,0,0,0,...] }
}
```

### Implementation Steps

1. **Change internal storage** in all languages:
   - **Python**: `self._coords: list[float]` instead of `self.points: list[Point]`
   - **C++**: `std::vector<double> coords` instead of `std::vector<Point> points`
   - **Rust**: `pub coords: Vec<f64>` instead of `pub points: Vec<Point>`

2. **Add helper methods** for point access:
   - `point_count() -> int` - returns `len(coords) / 3`
   - `get_point(index) -> Point` - creates Point from coords[i*3:i*3+3]
   - `set_point(index, point)` - updates coords from point
   - `add_point(point)` - appends x, y, z to coords
   - `points() -> list[Point]` - returns list of Point objects (for iteration)

3. **Update serialization**:
   - `__jsondump__` outputs `"coords": [x0, y0, z0, x1, y1, z1, ...]`
   - `__jsonload__` reads coords array

4. **Keep Plane separately** - Plane is computed from first 3 points, not stored per-point

### Benefits
- 90%+ reduction in JSON size for polylines with many points
- Faster serialization/deserialization
- Lower memory footprint
- Matches industry-standard geometry formats (OBJ, PLY, etc.)

## Required Methods for All Geometry Classes

Every geometry class (Point, Vector, Line, Color, Polyline, etc.) **MUST** implement these methods consistently:

### Core Methods

| Method | Python | C++ | Rust | Description |
|--------|--------|-----|------|-------------|
| Constructor | `__init__` | Constructor | `new()` | Create instance with parameters |
| Named constructor | `with_name()` classmethod | `with_name()` static | `with_name()` | Create with custom name |
| **Duplicate** | `duplicate()` | `duplicate()` | `duplicate()` | Deep copy with **new guid** |
| **Short string** | `__str__` | `str()` | `str()` | Minimal representation |
| **Full string** | `__repr__` | `repr()` | `repr()` | Complete representation with name |
| Equality | `__eq__` | `operator==` | `impl PartialEq` | Compare by value (ignore guid) |

### String Representation Pattern

**str()** - Minimal, just the data:
```python
# Color: "255, 0, 0, 255"
# Point: "1.0, 2.0, 3.0"
# Vector: "1.0, 2.0, 3.0"
# Line: "0.0, 0.0, 0.0 -> 1.0, 1.0, 1.0"
# Polyline: "[(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0)]"
```

**repr()** - Full, includes class name and name field:
```python
# Color: "Color(red, 255, 0, 0, 255)"
# Point: "Point(my_point, 1.0, 2.0, 3.0, Color(blue, 0, 0, 255, 255), 1.0)"
# Vector: "Vector(my_vector, 1.0, 2.0, 3.0)"
# Line: "Line(my_line, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0)"
# Polyline: "Polyline(my_polyline, 3 points)"
```

### Duplicate Pattern

The `duplicate()` method creates a deep copy with a **new GUID**:

**Python:**
```python
def duplicate(self):
    """Create a deep copy with a new GUID."""
    result = copy.deepcopy(self)
    result.guid = str(uuid.uuid4())
    return result
```

**C++:**
```cpp
Polyline duplicate() const {
    Polyline result = *this;  // Copy all fields
    result.guid = ::guid();   // Generate new GUID
    return result;
}
```

**Rust:**
```rust
pub fn duplicate(&self) -> Self {
    let mut result = self.clone();
    result.guid = Uuid::new_v4().to_string();
    result
}
```

## Polyline Test Alignment

### Current Issue

The Rust polyline_test.rs uses standard `#[test]` attributes instead of `MINI_TEST!` macro, making tests incompatible with the MINI_TEST framework used by Python and C++.

### Required Test Structure for Polyline

All three languages must have **EXACTLY the same tests** with identical:
- Test names
- Variable names
- Comments
- MINI_CHECK assertions
- Line count per test

### Example: Polyline Constructor Test (Aligned)

**Python** (`session_py/src/session_py/polyline_test.py`):
```python
@MINI_TEST("Polyline", "constructor")
def test_polyline_constructor():
    from session_py import Polyline
    from session_py import Point
    from session_py import Vector
    from session_py import Color

    # Constructor with points
    p0 = Point(0.0, 0.0, 0.0)
    p1 = Point(1.0, 0.0, 0.0)
    p2 = Point(1.0, 1.0, 0.0)
    pl = Polyline([p0, p1, p2])

    # Basic properties
    point_count = len(pl)
    segment_count = pl.segment_count()
    is_empty = pl.is_empty()

    # Minimal and Full String Representation
    plstr = str(pl)
    plrepr = repr(pl)

    # Copy (duplicates everything except guid)
    plcopy = pl.duplicate()
    plother = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)])

    MINI_CHECK(pl.name == "my_polyline" and pl.guid != "" and point_count == 3)
    MINI_CHECK(segment_count == 2 and is_empty == False)
    MINI_CHECK("0.0, 0.0, 0.0" in plstr)
    MINI_CHECK("Polyline(my_polyline" in plrepr)
    MINI_CHECK(plcopy == plother)
    MINI_CHECK(plcopy.guid != pl.guid)
```

**C++** (`session_cpp/src/polyline_test.cpp`):
```cpp
MINI_TEST("Polyline", "constructor") {
    // uncomment #include "polyline.h"
    // uncomment #include "point.h"
    // uncomment #include "vector.h"
    // uncomment #include "color.h"

    // Constructor with points
    Point p0(0.0, 0.0, 0.0);
    Point p1(1.0, 0.0, 0.0);
    Point p2(1.0, 1.0, 0.0);
    Polyline pl({p0, p1, p2});

    // Basic properties
    size_t point_count = pl.len();
    size_t segment_count = pl.segment_count();
    bool is_empty = pl.is_empty();

    // Minimal and Full String Representation
    std::string plstr = pl.str();
    std::string plrepr = pl.repr();

    // Copy (duplicates everything except guid)
    Polyline plcopy = pl.duplicate();
    Polyline plother({Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)});

    MINI_CHECK(pl.name == "my_polyline" && !pl.guid.empty() && point_count == 3);
    MINI_CHECK(segment_count == 2 && is_empty == false);
    MINI_CHECK(plstr.find("0.0, 0.0, 0.0") != std::string::npos);
    MINI_CHECK(plrepr.find("Polyline(my_polyline") != std::string::npos);
    MINI_CHECK(plcopy == plother);
    MINI_CHECK(plcopy.guid != pl.guid);
}
```

**Rust** (`session_rust/src/polyline_test.rs`):
```rust
pub fn run_polyline_constructor() -> TestResult {
    MINI_TEST!("constructor", {
        use crate::Polyline;
        use crate::Point;
        use crate::Vector;
        use crate::Color;

        // Constructor with points
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 0.0);
        let pl = Polyline::new(vec![p0, p1, p2]);

        // Basic properties
        let point_count = pl.len();
        let segment_count = pl.segment_count();
        let is_empty = pl.is_empty();

        // Minimal and Full String Representation
        let plstr = pl.str();
        let plrepr = pl.repr();

        // Copy (duplicates everything except guid)
        let plcopy = pl.duplicate();
        let plother = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0)]);

        MINI_CHECK!(pl.name == "my_polyline" && !pl.guid.is_empty() && point_count == 3);
        MINI_CHECK!(segment_count == 2 && is_empty == false);
        MINI_CHECK!(plstr.contains("0.0, 0.0, 0.0"));
        MINI_CHECK!(plrepr.contains("Polyline(my_polyline"));
        MINI_CHECK!(plcopy == plother);
        MINI_CHECK!(plcopy.guid != pl.guid);
    })
}

REGISTER_MINI_TEST!("Polyline", "constructor", crate::polyline_test::run_polyline_constructor);
```

### Required Polyline Tests

All three languages must implement these tests identically:

| Test Name | Description |
|-----------|-------------|
| `constructor` | Create polyline, test basic properties, str/repr, duplicate |
| `length` | Test `length()` and `magnitude_squared()` |
| `center` | Test `center()` and `center_vec()` |
| `is_closed` | Test open vs closed polylines |
| `reverse` | Test `reverse()` and `reversed()` |
| `closest_point` | Test `closest_distance_and_point()` |
| `json_roundtrip` | Test JSON serialization/deserialization |

### Rust MINI_TEST Migration

Convert existing Rust `#[test]` functions to MINI_TEST! macro format:

**Before (incompatible):**
```rust
#[test]
fn test_polyline_new() {
    let polyline = Polyline::new(vec![...]);
    assert_eq!(polyline.len(), 3);
}
```

**After (compatible):**
```rust
pub fn run_polyline_constructor() -> TestResult {
    MINI_TEST!("constructor", {
        use crate::Polyline;
        let pl = Polyline::new(vec![...]);
        MINI_CHECK!(pl.len() == 3);
    })
}

REGISTER_MINI_TEST!("Polyline", "constructor", crate::polyline_test::run_polyline_constructor);
```

## Coordinate Access Convention

**IMPORTANT:** Always use index operators `[0]`, `[1]`, `[2]` for coordinate access, NOT `.x`, `.y`, `.z` properties.

| Access | Python | C++ | Rust |
|--------|--------|-----|------|
| X coord | `v[0]` | `v[0]` | `v[0]` |
| Y coord | `v[1]` | `v[1]` | `v[1]` |
| Z coord | `v[2]` | `v[2]` | `v[2]` |

This applies to `Point`, `Vector`, `Line`, and all geometry classes.

**Wrong:**
```python
self._x0 += other.x  # Vector has no .x property
```

**Correct:**
```python
self._x0 += other[0]  # Use index operator
```
