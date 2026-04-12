# Testing

## Standard Test Groups

| Test | Content |
|------|---------|
| `constructor` | Default/param constructors, [], str, repr, ==, !=, arithmetic |
| `json_roundtrip` | json_dump → json_load verification |
| `protobuf_roundtrip` | protobuf_dump → protobuf_load verification |
| `transformation` | transform(), transformed() (visual classes) |

## Alignment between C++, Rust, and Python

- Compare all test files and make sure exactly same tests are implemented in all three languages.
- When we have collections such as a vector of points we write each object in separate line for example, this pattern is valid for all languages: 
Polyline pl({
    Point(0.0, 0.0, 0.0),
    Point(1.0, 0.0, 0.0),
    Point(1.0, 1.0, 0.0),
    Point(0.0, 1.0, 0.0),
});

## Multi-language alignment
- Each test, as much as possible, must have: same comments, same empty lines, same tests, same api. They must be essentially the same, because our aim to have same api across all languages.

## Test coverage

- All functions must be covered by following the order of cpp header file. In the same order python and rust function tests must be defined. All tests must be the same and the same amount of tests within all languages following the style mention in testing.md.

## Specific cases

- Cpp requires to declare import using uncomment flag: such as  "// uncomment #include "quaternion.h""

## Naming

- Test names must start from a capital letter e.g. "MINI_TEST("Quaternion", "Normalized")". This rule must be applied to all languages. Here there two strings "Quaternion" and "Normalized" starts from the capital letters.
- Test names must use spaces between words. Never write CamelCase test names. Wrong: "Hull2dCircle", "LuDecompose". Correct: "Hull 2d Circle", "Lu Decompose".

## Import Style

Never import multiple symbols on one line. Each import must be on its own line.

**Python — wrong:**
```python
from session_py import ConvexHull, Point
```
**Python — correct:**
```python
from session_py import ConvexHull
from session_py import Point
```

**Rust — wrong:**
```rust
use crate::{ConvexHull, Point};
```
**Rust — correct:**
```rust
use crate::ConvexHull;
use crate::Point;
```

## Rules

1. Same test names across all languages
2. Same variable names
3. Same assertion order
4. Similar line count

## Constructor Test Includes

- Default constructor
- Parameterized constructor
- Index operator []
- String representation (str, repr)
- Equality operators (==, !=)
- In-place operators (+=, -=, *=, /=)
- Copy operators (+, -, *, /)
- duplicate() with new GUID check

## Output

Tests write JSON to `session_tests/session_{lang}/classname_test.json`

## See Language-Specific

- `cpp.md` - C++ minitest
- `py.md` - Python minitest
- `rust.md` - Rust minitest
