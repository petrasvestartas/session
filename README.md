# Session

![Python](https://img.shields.io/badge/Python-3670A0?logo=python&logoColor=ffdd54)
![C++](https://img.shields.io/badge/C++-00599C?logo=cplusplus&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![WebGPU](https://img.shields.io/badge/WebGPU-005A9C?logo=webgpu&logoColor=white)

A geometry kernel written three times, in C++, Python and Rust, with the same API, the same
protobuf schemas and the same tests in every language. C++ is the ground truth.

Documentation: <https://petrasvestartas.github.io/session/>

## Classes

| Group | Classes |
|-------|---------|
| Basics | `tolerance` `color` `matrix` `xform` `quaternion` `session_config` |
| Points and vectors | `point` `vector` `plane` `line` `pointcloud` |
| Curves | `polyline` `nurbsknot` `nurbscurve` |
| Surfaces | `nurbssurface` `nurbssurface_trimmed` `remesh_nurbssurface_grid` `remesh_nurbssurface_adaptive` |
| Meshes and solids | `mesh` `mesh_offset` `remesh_cdt` `convex_hull` `brep` `primitives` `simple_split` |
| Algorithms | `intersection` `closest` `boolean_polyline` |
| Bounding volumes and indices | `aabb` `obb` `spatial_aabbtree` `spatial_bvh` `spatial_kdtree` `spatial_octree` `spatial_rtree` |
| Scene | `session` `objects` `element` `instance_ref` `graph` `tree` `history` |
| Files | `file_encoders` `file_obj` `file_step` `io_xyz` |

Each class is `session_cpp/src/<class>.h|.cpp`, `session_py/src/session_py/<class>.py` and
`session_rust/src/<class>.rs`, each with a `<class>_test` file holding the same tests.

## Repository

| Path | Contents |
|------|----------|
| [`session_cpp`](https://github.com/petrasvestartas/session_cpp) | C++ kernel |
| [`session_py`](https://github.com/petrasvestartas/session_py) | Python kernel |
| [`session_rust`](https://github.com/petrasvestartas/session_rust) | Rust kernel |
| [`session_proto`](https://github.com/petrasvestartas/session_proto) | Protobuf schemas shared by all three |
| [`session_data`](https://github.com/petrasvestartas/session_data) | Test and demo datasets |
| `session_viewer` | WebGPU viewer (Rust to WASM) and its course in `docs/` |
| `session_tests` | Web page showing the three languages' test results side by side |
| `bash` | Build, test and git scripts |

## New PC

### 1. Tools

| Tool | Windows | macOS | Ubuntu |
|------|---------|-------|--------|
| Git + bash | [Git for Windows](https://git-scm.com/download/win) (use Git Bash for all commands below) | `xcode-select --install` | `sudo apt install git` |
| C++ compiler | [Visual Studio 2022](https://visualstudio.microsoft.com/), "Desktop development with C++" | `xcode-select --install` | `sudo apt install build-essential` |
| CMake 3.20+ | [cmake.org](https://cmake.org/download/) | `brew install cmake` | `sudo apt install cmake` |
| Python 3.9+ and uv | [python.org](https://www.python.org/downloads/), then `powershell -c "irm https://astral.sh/uv/install.ps1 \| iex"` | `brew install python uv` | `sudo apt install python3 && curl -LsSf https://astral.sh/uv/install.sh \| sh` |
| Rust | [rustup.rs](https://rustup.rs) | `curl https://sh.rustup.rs -sSf \| sh` | `curl https://sh.rustup.rs -sSf \| sh` |
| Node 20+ (test page only) | [nodejs.org](https://nodejs.org) | `brew install node` | `sudo apt install nodejs npm` |

### 2. Clone

```bash
git clone --recurse-submodules https://github.com/petrasvestartas/session.git
cd session
```

Already cloned without submodules: `git submodule update --init --recursive`.

### 3. Build and test each language

The same commands work on Windows (Git Bash), macOS and Ubuntu.

**Python**

```bash
uv venv uvsession --python 3.9
source uvsession/bin/activate            # Windows: source uvsession/Scripts/activate
uv pip install -e "session_py[dev]"
./bash/minitest.sh --py --no-web
```

**C++** (the first configure builds protobuf from source, which takes a few minutes)

```bash
cmake -S session_cpp -B session_cpp/build -DCMAKE_BUILD_TYPE=Release
cmake --build session_cpp/build --config Release --parallel 4
./bash/minitest.sh --cpp --no-web
```

**Rust**

```bash
cd session_rust && cargo build --lib --bin minitest && cd ..
./bash/minitest.sh --rust --no-web
```

**All three, with the results page at <http://localhost:8769>**

```bash
./bash/minitest.sh
```

One class only: `./bash/quicktest.sh <class> --py|--cpp|--rust`.

### 4. Viewer (optional)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cd session_viewer && trunk serve
```

Needs a WebGPU browser: Chrome, Edge, Firefox or Safari 18+.

## Git

Pull everything: `./bash/git_pull.sh`. Commit and push every submodule and this repo:
`./bash/git_push.sh "message"`. Never add an AI as author or co-author.
