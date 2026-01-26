# New Basic Class

Use this for classes WITHOUT visual properties (no width/color/xform).

Examples: Color, Xform, Tolerance, Knot

## Checklist

1. Create implementation files (see cpp.md, py.md, rust.md)
2. Create test files
3. Register in build system
4. Run minitest

## Files to Create

| Language | Files |
|----------|-------|
| C++ | `src/name.h`, `src/name.cpp`, `src/name_test.cpp` |
| Python | `src/session_py/name.py`, `src/session_py/name_test.py` |
| Rust | `src/name.rs`, `src/name_test.rs` |

## Required Methods

- Constructor(s)
- str(), repr()
- is_valid()
- duplicate() / copy
- operator[], ==, !=
- json_dump/load, protobuf_dump/load

## See Templates

- `cpp.md` - Complete C++ template
- `py.md` - Complete Python template
- `rust.md` - Complete Rust template
