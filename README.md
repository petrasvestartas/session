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
- 1. Arrow
- 2. BoundingBox, for all from method add a plane attribute, that will orient geometry from that plane to xy plane create a box and orient back to the plane. In this way we will create tight bounding box.

Is it possible to compute boundingbox using Principal Component Analysis without external libraries like eigen it should be standalone: https://github.com/petrasvestartas/opennest_2/blob/782381d5fc549acf88f33022d05a0d2fed8a1eb7/src/minkowski/src/boundingbox.cc
- 3. Color
- 4. Cylinder
- 5. Edge
- 6. Graph
- 7. Line
- 8. Mesh
- 9. Objects
- 10. Plane
- 11. Point
- 12. Pointcloud
- 13. Polyline
- 14. Quaternion
- 15. Session
- 16. Tolerance
- 17. Tree
- 18. TreeNode
- 19. Vector
- 20. Vertex
- 21. Xform
- 22. Encoders
- 23. BVH - speed up bvh by using oriented bounding box

**Planned:**
- Intesections - Create ray (point and vector) intersection with a mesh, 
line-line
plane-plane
line-plane
plane plane plane
https://github.com/petrasvestartas/wood/blob/main/cmake/src/wood/include/cgal_intersection_util.cpp
ray box
https://github.com/libigl/libigl/blob/main/include/igl/ray_box_intersect.h
https://github.com/libigl/libigl/blob/main/include/igl/ray_box_intersect.cpp
ray sphere
https://github.com/libigl/libigl/blob/main/include/igl/ray_sphere_intersect.h
https://github.com/libigl/libigl/blob/main/include/igl/ray_sphere_intersect.cpp
ray triangle
https://github.com/libigl/libigl/blob/main/include/igl/ray_triangle_intersect.h
https://github.com/libigl/libigl/blob/main/include/igl/ray_triangle_intersect.cpp
https://github.com/libigl/libigl/blob/main/include/igl/raytri.c
ray mesh
https://github.com/libigl/libigl/blob/main/include/igl/ray_mesh_intersect.h
https://github.com/libigl/libigl/blob/main/include/igl/ray_mesh_intersect.cpp

- Protobuf
- Beam, Plate elements
- Curve
- NURBS
- BREP

## Development Setup

### Build & Run Aliases

**Linux:**
```bash
alias c='(cd ~/code/code_rust/session/session_cpp && ./build.sh)'
alias r='(cd /code/code_rust/session/session_rust && cargo run)'
alias p='(cd /code/code_rust/session/session_py && conda activate session && python main.py)'
```

**macOS:**
```bash
alias c='(cd /Users/petras/brg/code_rust/session/session_cpp && ./build.sh)'
alias r='(cd /Users/petras/brg/code_rust/session/session_rust && cargo run)'
alias p='(cd /Users/petras/brg/code_rust/session/session_py && conda activate session && python main.py)'
```

**Windows (Command Prompt only):**
```cmd
doskey c=cd /d "c:\pc\3_code\code_rust\session\session_cpp" ^& build.bat
doskey r=cd /d "c:\pc\3_code\code_rust\session\session_rust" ^& cargo run
doskey p=cd /d "c:\pc\3_code\code_rust\session\session_py" ^& conda activate session ^& python main.py
```
*Note: Run these commands in each new Command Prompt session. They don't work in PowerShell.*

### Test Aliases

**Linux/macOS:**
```bash
alias ct='(cd ~/code/code_rust/session/session_cpp && ./test.sh)'  # or /Users/petras/...
alias rt='(cd /code/code_rust/session/session_rust && ./test.sh)'
alias pt='(cd /code/code_rust/session/session_py && ./test.sh)'
```

**Windows (Command Prompt only):**
```cmd
doskey ct=cd /d "c:\pc\3_code\code_rust\session\session_cpp" ^& test.bat
doskey rt=cd /d "c:\pc\3_code\code_rust\session\session_rust" ^& test.bat
doskey pt=cd /d "c:\pc\3_code\code_rust\session\session_py" ^& test.bat
```

### Documentation Aliases

```bash
alias cdoc='(cd /home/pv/brg/code_rust/session/session_cpp && ./doc.sh)'
alias rdoc='(cd /home/pv/brg/code_rust/session/session_rust && cargo doc)'
alias pdoc='(cd /home/pv/brg/code_rust/session/session_py && ./doc.sh)'
```

## Adding New Classes to Documentation

### Python
Add module to `session_py/docs/api.rst`:
```rst
NewClass Module
---------------

.. automodule:: session_py.newclass
   :members:
   :undoc-members:
   :show-inheritance:
```

### C++
Add Doxygen comment to class in header file:
```cpp
/**
 * @brief Brief description of the class
 */
class NewClass {
```

### Rust
Rust docs auto-generate from `///` comments - no config needed.

## References

- [Wood Library](https://github.com/petrasvestartas/wood/tree/main/cmake/src/wood/include)
- [Wink Geometry](https://github.com/petrasvestartas/wink/tree/5d0a53e68cef2f4ea3671fef5fccbe009124369a/src/openmodel/src/geometry)

