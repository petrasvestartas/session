# Session 

![Python](https://img.shields.io/badge/Python-3670A0?logo=python&logoColor=ffdd54) ![C++](https://img.shields.io/badge/C++-00599C?logo=cplusplus&logoColor=white) ![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white) 

Session is a geometry kernel for datastructures:
 
1. `arrow`
2. `boundingbox`
3. `bvh`
4. `color`
5. `cylinder`
6. `edge`
7. `graph`
8. `intersection`
9. `line`
10. `mesh`
11. `nurbscurve`
12. `nurbssurface`
13. `obj`
14. `objects`
15. `plane`
16. `point`
17. `pointcloud`
18. `polyline`
19. `quaternion`
20. `tolerance`
21. `tree`
22. `treenode`
23. `vector`
24. `vertex`
25. `xform`

## Goal

The aim is to display serialized geometry for short time sessions, mostly code development, in a web browser via a Rust‑written wgpu viewer.
I am learning engineering and math problems, so I need something that I know very well and can debug.

## Documentation

Instead of typical API documentation (it is often better to look at the source code itself), I decided to write a custom test framework to document the code by (a) profiling, (b) tests, and (c) examples. 
 
See the [Session documentation](https://petrasvestartas.github.io/session/).

## Code structure

The repository is split between 5 submodules:

- [`session_py`](https://github.com/petrasvestartas/session_py.git) → Python Kernel
- [`session_rust`](https://github.com/petrasvestartas/session_rust.git) → Rust Kernel
- [`session_cpp`](https://github.com/petrasvestartas/session_cpp.git) → C++ Kernel
- [`session_data`](https://github.com/petrasvestartas/session_data.git) → Geometry Dataset
- [`session_proto`](https://github.com/petrasvestartas/session_proto.git) → Schemas

