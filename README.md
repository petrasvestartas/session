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
- Color
- Graph
- Line
- Mesh
- Objects
- Plane
- Point
- Pointcloud
- Polyline
- Quaternion
- Session
- Tree
- Tolerance
- Vector
- Xform
- Pipe

**Planned:**
- Arrow
- BoundingBox
- BVH
- Protobuf
- Beam, Plate elements
- Curve
- NURBS
- BREP

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
- [Wink Geometry](https://github.com/petrasvestartas/wink/tree/5d0a53e68cef2f4ea3671fef5fccbe009124369a/src/openmodel/src/geometry)

