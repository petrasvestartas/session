# Session Multi-Language Geometry Library

Cross-platform 3D geometry library implemented in C++, Rust, and Python with complete API parity.

## Project Structure

```
session/
├── session_cpp/    # C++ implementation
├── session_rust/   # Rust implementation  
└── session_py/     # Python implementation
```

## Implementation Status

**Completed:**
- ✅ Point, Vector, Plane
- ✅ Xform, Quaternion, Line
- ✅ Tolerance, Polyline

**Planned:**
- PointCloud, Mesh
- Arrow, Pipe, BoundingBox
- BVH, Protobuf
- Beam, Plate elements
- BREP, NURBS surfaces

## Code Organization Pattern

All classes follow this structure across C++, Rust, and Python:

1. **Constructors / Static Factory Methods** - `xy_plane()`, `from_points()`, etc.
2. **Operators** - `__str__`, `__eq__`, `to_string()`, equality checks
3. **JSON** - `to_json_data()`, `from_json_data()`, `to_json()`, `from_json()`
4. **No-copy Operators** - `__iadd__`, `__isub__`, `__getitem__`, `operator[]`
5. **Copy Operators** - `__add__`, `__sub__`, `__mul__`, `operator+`, `operator*`
6. **Static Methods** - `x_axis()`, `cosine_law()`, utility functions
7. **Details** - `reverse()`, `rotate()`, geometric/transformation methods

**Section Headers:**
```python
# Python/C++
###########################################################################################
# JSON
###########################################################################################
```

**Rust:** Use `impl` blocks for organization (no comment headers).

## Documentation Style

| Language | Format | Sections |
|----------|--------|----------|
| **Python** | `"""docstring"""` | NumPy-style (Parameters, Returns) |
| **C++** | `/// comment` | NumPy-style OR Doxygen (`@brief`, `@param`) |
| **Rust** | `/// comment` | Brief description only |

## Development Setup

### Build & Run Aliases

**Linux:**
```bash
alias c='(cd /home/pv/brg/code_rust/session/session_cpp && ./build.sh)'
alias r='(cd /home/pv/brg/code_rust/session/session_rust && cargo run)'
alias p='(cd /home/pv/brg/code_rust/session/session_py && conda activate session && python main.py)'
```

**macOS:**
```bash
alias c='(cd /Users/petras/brg/code_rust/session/session_cpp && ./build.sh)'
alias r='(cd /Users/petras/brg/code_rust/session/session_rust && cargo run)'
alias p='(cd /Users/petras/brg/code_rust/session/session_py && conda activate session && python main.py)'
```

**Windows:**
```cmd
doskey c=cd /d "c:\brg\code_rust\session\session_cpp" ^& build.bat
doskey r=cd /d "c:\brg\code_rust\session\session_rust" ^& cargo run
doskey p=cd /d "c:\brg\code_rust\session\session_py" ^& conda activate session ^& python main.py
```

### Test Aliases

**Linux/macOS:**
```bash
alias ct='(cd /home/pv/brg/code_rust/session/session_cpp && ./test.sh)'    # or /Users/petras/...
alias rt='(cd /home/pv/brg/code_rust/session/session_rust && ./test.sh)'
alias pt='(cd /home/pv/brg/code_rust/session/session_py && ./test.sh)'
```

**Windows:**
```cmd
doskey ct=cd /d "c:\brg\code_rust\session\session_cpp" ^& test.bat
doskey rt=cd /d "c:\brg\code_rust\session\session_rust" ^& test.bat
doskey pt=cd /d "c:\brg\code_rust\session\session_py" ^& test.bat
```

### Documentation Aliases

```bash
alias cdoc='(cd /home/pv/brg/code_rust/session/session_cpp && ./doc.sh)'
alias rdoc='(cd /home/pv/brg/code_rust/session/session_rust && cargo doc)'
alias pdoc='(cd /home/pv/brg/code_rust/session/session_py && ./doc.sh)'
```

## References

- [Wood Library](https://github.com/petrasvestartas/wood/tree/main/cmake/src/wood/include)
- [Wink Geometry](https://github.com/petrasvestartas/wink/tree/main/src/openmodel/src/geometry)

