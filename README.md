# Session

![Python](https://img.shields.io/badge/Python-3670A0?logo=python&logoColor=ffdd54)
![C++](https://img.shields.io/badge/C++-00599C?logo=cplusplus&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![WebGPU](https://img.shields.io/badge/WebGPU-005A9C?logo=webgpu&logoColor=white)

Session is a multi-language geometry kernel implemented three times over — in Python, C++ and
Rust — with identical APIs, shared protobuf schemas, and a test suite that runs the same
assertions in every language. It covers 47 tested classes: points, curves, surfaces, meshes,
BReps (OCCT-style topology), and spatial indices.

C++ is the ground truth; Python and Rust are ported from it with matching APIs, variable names
and test logic.

See the [Session documentation](https://petrasvestartas.github.io/session/).

## Repository layout

The three kernels and the shared schema/data live in Git submodules:

| Submodule | Description |
|-----------|-------------|
| [`session_cpp`](https://github.com/petrasvestartas/session_cpp) | C++ kernel — ground truth |
| [`session_py`](https://github.com/petrasvestartas/session_py) | Python kernel |
| [`session_rust`](https://github.com/petrasvestartas/session_rust) | Rust kernel |
| [`session_proto`](https://github.com/petrasvestartas/session_proto) | Protobuf schemas shared by all three |
| [`session_data`](https://github.com/petrasvestartas/session_data) | Geometry datasets used by tests and demos |
| [`session_rhino`](https://github.com/petrasvestartas/session_rhino) | RhinoCommon converters |

Everything else lives directly in this repository:

| Directory | Description |
|-----------|-------------|
| `session_viewer` | Browser-only WebGPU CAD viewer (Rust → WASM via Trunk). Camera-relative f64, reverse-Z depth, CPU ray + BVH picking. `docs/` holds 100+ numbered lessons that build it from scratch. |
| `session_tests` | Vue 3 test viewer — renders the per-class JSON results from all three languages side by side |
| `bash` | Build, test and git automation — `minitest.sh` is the main entry point |
| `serialization` | Round-trip protobuf/JSON fixtures |
| `session_compas` | COMPAS framework interop |
| `session_viewer_archive` | Previous viewer generation, kept for reference |

`uvsession/` (Python virtualenv) and build directories (`target/`, `build/`, `dist*/`) are local
only and never committed.

## Key API files

One file = one class, or one tightly-coupled group (`tree` contains `Tree` + `TreeNode`; `graph`
contains `Graph` + `Vertex` + `Edge`).

Status is computed from the latest test run: a class is ticked when all three languages
emit the **same set of test names**. Currently **31 of 47** classes are at full parity.

- [x] `aabb`
- [x] `boolean_polyline`
- [x] `brep`
- [x] `closest`
- [x] `color`
- [x] `convex_hull`
- [x] `element`
- [x] `file_encoders`
- [x] `file_obj`
- [ ] `file_step` — only C++
- [x] `graph`
- [x] `instance_ref`
- [ ] `intersection` — Python missing 1; C++ missing 1
- [ ] `io` — Python missing 1; C++ missing 1 ("Read Colors"); "Import Minimal" (PDF) is Rust-only behind `--features pdf`
- [x] `line`
- [x] `matrix`
- [ ] `mesh` — Python missing 5; Rust missing 31; C++ missing 1
- [x] `mesh_offset`
- [ ] `nurbscurve` — Python missing 1; Rust missing 3
- [x] `nurbsknot`
- [ ] `nurbssurface` — Python missing 1; Rust missing 1
- [x] `nurbssurface_trimmed`
- [x] `obb`
- [ ] `objects` — Python missing 2; Rust missing 2; C++ missing 2
- [x] `plane`
- [x] `point`
- [x] `pointcloud`
- [x] `polyline`
- [ ] `primitives` — Rust missing 5
- [x] `quaternion`
- [ ] `remesh_cdt` — Python missing 1; Rust missing 1
- [ ] `remesh_nurbssurface_adaptive` — Rust missing 1
- [x] `remesh_nurbssurface_grid`
- [ ] `session` — Python missing 3; C++ missing 3
- [x] `session_config`
- [x] `spatial_aabbtree`
- [x] `spatial_bvh`
- [x] `spatial_kdtree`
- [x] `spatial_rtree`
- [x] `tolerance`
- [x] `tree`
- [ ] `vector` — Python/Rust carry a duplicate `interpolate_points` (see note below)
- [x] `xform`

Regenerate this status with `./bash/minitest.sh` — it rewrites the per-class JSON under
`session_tests/<language>/` that the table above is derived from.

Modules with no cross-language test set: `mesh_boolean`, `render_mesh` and `guid_serde` are Rust
only.

`vector`'s divergence is not a C++ gap. `Polyline::interpolate_points` exists and is tested under
`Polyline` in all three languages. Python and Rust additionally keep a *second* copy of it as a
free function in the `vector` module (`vector.py:1274`, `vector.rs:1338`) with its own
`Vector / Interpolate Points` test. That copy is exported from neither `__init__.py` nor `lib.rs`
and is referenced only by its own test — dead duplicate code that C++ never had.

## Prerequisites

| Tool | macOS | Ubuntu | Windows |
|------|-------|--------|---------|
| **CMake** | `brew install cmake` | `sudo apt install cmake` | [cmake.org](https://cmake.org/download/) |
| **Python 3.11+** | `brew install python` | `sudo apt install python3` | [python.org](https://python.org) |
| **Rust** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | same | [rustup.rs](https://rustup.rs) |
| **Node 20+** | `brew install node` | `sudo apt install nodejs npm` | [nodejs.org](https://nodejs.org) |
| **C++ compiler** | `xcode-select --install` | `sudo apt install build-essential` | [Visual Studio](https://visualstudio.microsoft.com/) |

For `session_viewer` only:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

plus a WebGPU-capable browser (Chrome, Edge, Firefox, or Safari 18+).

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

# macOS/Linux
source uvsession/bin/activate

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
| `./bash/quicktest.sh <class> --py` | Single class test |

Tests are meant to be identical across all three languages — same names, same logic, same line
count — so any divergence shows up as a mismatched cell in the viewer. The
[Key API files](#key-api-files) table tracks where that goal currently holds.

## Working with submodules

Pull the main repo and all submodules:

```bash
./bash/git_pull.sh          # or: git pull && git submodule update --init --recursive
```

Commit and push changes across all submodules **and** the main repo in one step:

```bash
./bash/git_push.sh "your commit message"
```

Add a new submodule:

```bash
git submodule add <repo-url> <folder-name>
git submodule update --init --recursive
git commit -am "Add submodule <folder-name>"
git push
```

## Breaking-Change Detection

Breaking changes in `session-py` or protobuf schemas are caught before they ship:

**Python API diff (`griffe`)** — runs in CI on every push to `main`. Compares the current branch
against the last published PyPI version. Reports exactly which class, method, or parameter was
removed or renamed with file path and line number.

**Protobuf schema diff (`buf`)** — runs in `session_proto` CI on every push/PR. Catches removed
fields, renamed messages, and changed field numbers before they break serialization silently.
Config: `session_proto/buf.yaml`.

CI runs the full minitest matrix on Ubuntu 22.04, macOS 15 (ARM64 and Intel) and Windows.
