# Session

![Python](https://img.shields.io/badge/Python-3670A0?logo=python&logoColor=ffdd54) 
![C++](https://img.shields.io/badge/C++-00599C?logo=cplusplus&logoColor=white) 
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

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

## Goals

The aim is to display serialized geometry for short time sessions, mostly code development, in a web browser via a Rust‑written wgpu viewer.
I am learning engineering and math problems, so I need something that I know very well and can debug.

## Documentation

Instead of typical API documentation (it is often better to look at the source code itself), I decided to write a custom test framework to document the code by (a) profiling, (b) tests, and (c) examples.

See the [Session documentation](https://petrasvestartas.github.io/session/).

## Code structure

The repository is split between 5 submodule, each contains build instructions:

- [`session_py`](https://github.com/petrasvestartas/session_py.git) → Python Kernel
- [`session_rust`](https://github.com/petrasvestartas/session_rust.git) → Rust Kernel
- [`session_cpp`](https://github.com/petrasvestartas/session_cpp.git) → C++ Kernel
- [`session_data`](https://github.com/petrasvestartas/session_data.git) → Geometry Dataset
- [`session_proto`](https://github.com/petrasvestartas/session_proto.git) → Schemas

## Prerequisites

| Tool | macOS | Ubuntu | Windows |
|------|-------|--------|---------|
| **CMake** | `brew install cmake` | `sudo apt install cmake` | [cmake.org](https://cmake.org/download/) |
| **Python 3.11+** | `brew install python` | `sudo apt install python3` | [python.org](https://python.org) |
| **Rust** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | same | [rustup.rs](https://rustup.rs) |
| **Node.js 20+** | `brew install node` | `sudo apt install nodejs npm` | [nodejs.org](https://nodejs.org) |
| **C++ compiler** | `xcode-select --install` | `sudo apt install build-essential` | [Visual Studio](https://visualstudio.microsoft.com/) |

## Getting Started

Clone with all submodules:

```bash
git clone --recurse-submodules https://github.com/petrasvestartas/session.git
cd session
```

If you already cloned without submodules:

```bash
git submodule update --init --recursive
```

## New PC Setup

### 1. Install uv (Python package manager)

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### 2. Create Python virtual environment

```bash
uv venv uvsession --python 3.11
```

### 3. Activate and install dependencies

```bash
# Windows (Git Bash)
source uvsession/Scripts/activate

# macOS/Linux: source uvsession/bin/activate

cd session_py && uv pip install -e . && cd ..
```

### 4. Run tests

```bash
# Python only (fastest)
./bash/minitest.sh --py --no-web

# All languages with web viewer
./bash/minitest.sh
```

### Quick Reference

| Command | Description |
|---------|-------------|
| `./bash/minitest.sh --py --no-web` | Python tests only |
| `./bash/minitest.sh --rust --no-web` | Rust tests only |
| `./bash/minitest.sh --cpp --no-web` | C++ tests only |
| `./bash/minitest.sh --fast` | Skip dependency installs |
| `./bash/minitest.sh` | Full test + web viewer at localhost:8769 |
