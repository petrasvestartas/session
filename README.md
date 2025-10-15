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
- 1. Arrow - xform
- 2. BoundingBox - xform
- 3. Color
- 4. Cylinder
- 5. Edge
- 6. Graph
- 7. Line/
- 8. Mesh
- 9. Objects
- 10. Plane
- 11. Point
- 12. Pointcloud
- 13. Polyline
- 14. Quaternion
- 15. Session - 1) Implement BVH as attribute of the model, when you add geometry in add method you compute the boundingbox inflated by tolerance.PRECISION value 2) Session must have get_collision method that returns all collisions between objects via BVH and return pairs are added as graph edges of object guids to the session graph 3) Implement a method in session get_geometry, that gets all the geometry from the session but the geometry must be consequently transformed following the tree hierarchy xform attribute, the child of the tree must recursively be transformed by parent tree nodes
- 16. Tolerance - no need to serialize
- 17. Tree
- 18. TreeNode
- 19. Vector
- 20. Vertex
- 21. Xform
- 22. Encoders
- 23. BVH

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

