# Session 

![Python](https://img.shields.io/badge/Python-3670A0?logo=python&logoColor=ffdd54) ![C++](https://img.shields.io/badge/C++-00599C?logo=cplusplus&logoColor=white) ![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

Python, C++ and Rust geometry kernel.

## Goal

The aim is to display serialized geometry in a web browser via a Rust‑written wgpu viewer.
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

