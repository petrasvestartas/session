# Session 
 
Python, C++ and Rust geometry kernel.

## Goal

The aim is to display serialized geometry in a web browser via a Rust‑written wgpu viewer.
I am learning engineering and math problems, so I need something that I know very well and can debug.

## Documentation

Instead of typical API documentation (it is often better to look at the source code itself), I decided to write a custom test framework to document the code by (a) profiling, (b) tests, and (c) examples. I mostly learn how to use a library from examples rather than by scrolling through API documentation.

<https://petrasvestartas.github.io/session/>

## Code structure

The repository is split between 5 submodules:

- `session_py` → Python Kernel <https://github.com/petrasvestartas/session_py.git>
- `session_rust` → Rust Kernel <https://github.com/petrasvestartas/session_rust.git>
- `session_cpp` → C++ Kernel <https://github.com/petrasvestartas/session_cpp.git>
- `session_data` → Geometry Dataset <https://github.com/petrasvestartas/session_data.git>
- `session_proto` → Schemas <https://github.com/petrasvestartas/session_proto.git>

