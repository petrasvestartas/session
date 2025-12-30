# Implementation plan for NurbsCurve

## References

- **Ground truth:**
    - `/Users/petras/brg/code_rust/session/opennurbs-8.x/opennurbs_nurbscurve.h`
    - `/Users/petras/brg/code_rust/session/opennurbs-8.x/opennurbs_nurbscurve.cpp`

- **Current implementation:**
    - CPP:
        - `/Users/petras/brg/code_rust/session/session_cpp/src/nurbscurve.h`
        - `/Users/petras/brg/code_rust/session/session_cpp/src/nurbscurve.cpp`
        - `/Users/petras/brg/code_rust/session/session_cpp/src/nurbscurve_test.cpp`
    - PYTHON:
        - `/Users/petras/brg/code_rust/session/session_py/src/session_py/nurbscurve.py`
        - `/Users/petras/brg/code_rust/session/session_py/src/session_py/nurbscurve_test.py`
    - RUST:
        - `/Users/petras/brg/code_rust/session/session_rust/src/nurbscurve.rs`
        - `/Users/petras/brg/code_rust/session/session_rust/src/nurbscurve_test.rs`

- **Template (patterns for JSON/protobuf serialization, str/repr):**
    - CPP:
        - `/Users/petras/brg/code_rust/session/session_cpp/src/point.h`
        - `/Users/petras/brg/code_rust/session/session_cpp/src/point.cpp`
        - `/Users/petras/brg/code_rust/session/session_cpp/src/point_test.cpp`
    - PYTHON:
        - `/Users/petras/brg/code_rust/session/session_py/src/session_py/point.py`
        - `/Users/petras/brg/code_rust/session/session_py/src/session_py/point_test.py`
    - RUST:
        - `/Users/petras/brg/code_rust/session/session_rust/src/point.rs`
        - `/Users/petras/brg/code_rust/session/session_rust/src/point_test.rs`

## Current Status

| Feature | C++ | Python | Rust |
|---------|-----|--------|------|
| Core NURBS (dimension, order, knots, CVs) | ✅ | ✅ | ✅ |
| Visual props (guid, name, width, color, xform) | ✅ | ✅ | ✅ |
| point_at, tangent_at, evaluate | ✅ | ✅ | ✅ |
| intersect_plane | ✅ | ✅ | ✅ |
| JSON serialization | ✅ | ✅ | ✅ |
| Protobuf serialization | ✅ | ✅ | ⚠️ stub |
| to_string/str/repr | ✅ | ✅ | ✅ |

## TODO (Completed 2024-12-30)

- [x] JSON serialization must be equivalent across all 3 languages
    - [x] Ensure identical JSON structure/field names in C++, Python, Rust
    - [x] Use alphabetical field ordering (per CLAUDE.md conventions)
    - [x] Rust uses serde - verify output matches C++/Python exactly

- [x] Protobuf serialization aligned (Python/C++ full, Rust stub)
    - [x] Ensure all 3 languages use the same .proto schema
    - [x] Python and C++ have full protobuf support
    - [ ] Rust protobuf still uses JSON fallback (future work)

- [x] Add visual properties to Rust NurbsCurve:
    - [x] `guid: String`
    - [x] `name: String`
    - [x] `width: f64`
    - [x] `linecolor: Color`
    - [x] `xform: Xform`

- [x] Implement `to_string()` / Display trait for Rust NurbsCurve

- [x] Verify Python has all visual properties matching C++
- [x] Ensure Python `__str__`/`__repr__` output matches C++ `to_string()`

## Test Results

All 33 tests pass in all 3 languages:
- C++: 33/33 passed
- Python: 33/33 passed
- Rust: 33/33 passed
