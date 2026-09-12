---
name: new-class
description: Create a new geometry class in all three kernels (C++, Python, Rust) with identical API and minitests. Use when adding a class, when asked to "add X to session", or when a class exists in one language but not the others. Carries the basic/visual templates and the field, method, operator, serialization and transformation references.
argument-hint: <classname>
---

Create a new geometry class in all 3 languages (C++, Python, Rust) with full minitest coverage.

## Implementation files
- `session_rust/src/$ARGUMENTS.rs` + `session_py/src/session_py/$ARGUMENTS.py` + `session_cpp/src/$ARGUMENTS.h` + `$ARGUMENTS.cpp`

## Minitest files
- `session_rust/src/$ARGUMENTS_minitest.rs` (MINI_TEST!, MINI_CHECK!, REGISTER_MINI_TEST!)
- `session_py/src/session_py/$ARGUMENTS_minitest.py` (@MINI_TEST, MINI_CHECK)
- `session_cpp/src/$ARGUMENTS_minitest.cpp` (MINI_TEST, MINI_CHECK)

## Register
- Rust: `pub mod $ARGUMENTS_minitest;` in `session_rust/src/lib.rs`
- C++: add `src/$ARGUMENTS_minitest.cpp` to MINITEST_SOURCES in `session_cpp/CMakeLists.txt`
- Shell: add `"$ARGUMENTS"` to CLASS_NAMES in `bash/minitest.sh`

## Required API (all classes)
constructor, guid/name fields, duplicate(), clone(), index [], ==, !=, str/repr, __jsondump__/__jsonload__, json_dump/json_load, to_proto/from_proto

## Visual classes also need: width, color, xform, transform()/transformed()
## Arithmetic classes also need: +=, -=, *=, /=, +, -, *, /

## Test structure
- Constructor test groups: default ctor, overloads, [], ==, !=, str(), repr()
- One test per API method
- Tests must be identical across all 3 languages
- JSON fields alphabetically ordered

## Reference: templates in `basic/` and `visual/` (README + cpp/py/rust); field, method, operator, serialization and transformation notes in `reference/`

Verify: run `./bash/minitest.sh` — all tests must pass in all 3 languages.
