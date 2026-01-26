# Testing

## Standard Test Groups

| Test | Content |
|------|---------|
| `constructor` | Default/param constructors, [], str, repr, ==, !=, arithmetic |
| `json_roundtrip` | json_dump → json_load verification |
| `protobuf_roundtrip` | protobuf_dump → protobuf_load verification |
| `transformation` | transform(), transformed() (visual classes) |

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
