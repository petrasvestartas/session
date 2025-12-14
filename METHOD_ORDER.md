# Method & Test Ordering Convention

This repository contains equivalent implementations of a geometry kernel in **Python**, **C++**, and **Rust**.

The goal is to keep the **public API surface**, **method ordering inside files**, and **test ordering** consistent across languages.

This convention applies to the following classes:

- Color
- Point
- Line
- Polyline
- Tolerance

## 1. Class Implementation File Ordering

### 1.1 High-level ordering (all languages)

Within each class implementation, keep the following blocks in this order:

1. **Imports / includes / module prelude**
2. **Type / struct / class declaration**
3. **Constructors & named constructors**
   - Python: `__init__`, classmethods like `with_name`, etc.
   - C++: constructors + static factories
   - Rust: `new()`, `with_name()`, etc.
4. **Copy / duplicate semantics (core API)**
   - Must create **new guid**
5. **Core getters / setters / indexing operators**
   - The most frequently used “basic access” APIs
6. **Transform** (must come directly after constructors + duplicate + basic access)
   - `transform()` (in-place; must reset xform to identity)
   - `transformed()` (returns a copy; implemented via clone/copy + `transform()`)
7. **Serialization**
   - JSON (string-based + file-based)
   - Protobuf (if present; behind feature flags where needed)
8. **String representations** (must come directly after serialization)
   - Minimal string: `str()` / `__str__`
   - Full string: `repr()` / `__repr__`
9. **Equality / comparison**
   - Must ignore guid unless explicitly required
10. **Math / algorithmic methods**
    - e.g. subdivide, length, intersections, etc.
11. **Operators**
    - arithmetic, indexing, etc.
12. **Private helpers**

### 1.2 Transform contract

For all classes that have an `xform` field:

- **`transform()`**
  - Applies `xform` to the object in-place
  - Resets `xform` to identity afterwards
- **`transformed()`**
  - Returns a copy (clone) with the transform applied
  - Must not mutate the original

## 2. Test File Ordering

Test files must have the same suite/test ordering across languages.

Recommended order of test cases (where applicable):

1. `constructor`
2. `transformation`
3. `json_roundtrip`
4. `protobuf_roundtrip` (feature-gated)
5. Algorithmic tests (class-specific)

### 2.1 Per-test internal ordering

Inside a test function, keep this order consistent:

1. **Constructor**
2. **Setters**
3. **Getters**
4. **Minimal + full string** (`str` then `repr`)
5. **Copy / duplicate**
6. **Assertions**

## 3. Cross-language consistency rules

- Keep **test names identical** across languages.
- Keep **variable names identical** across languages.
- Keep **comment text identical** across languages.
- Prefer `TOLERANCE.is_close()` for floats.

## 4. Notes

This document defines ordering only. It should not require functional changes.
