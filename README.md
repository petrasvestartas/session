# Session

![Python](https://img.shields.io/badge/Python-3670A0?logo=python&logoColor=ffdd54)
![C++](https://img.shields.io/badge/C++-00599C?logo=cplusplus&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

Session is a multi-language geometry kernel (Python, C++, Rust) with shared protobuf schemas and a Vue test viewer. Implements 35+ geometry classes including points, curves, surfaces, meshes, and spatial data structures.

See the [Session documentation](https://petrasvestartas.github.io/session/).

## Code structure

| Submodule | Description |
|-----------|-------------|
| [`session_py`](https://github.com/petrasvestartas/session_py) | Python kernel |
| [`session_rust`](https://github.com/petrasvestartas/session_rust) | Rust kernel |
| [`session_cpp`](https://github.com/petrasvestartas/session_cpp) | C++ kernel |
| [`session_rhino`](https://github.com/petrasvestartas/session_rhino) | RhinoCommon converters |
| [`session_data`](https://github.com/petrasvestartas/session_data) | Geometry dataset |
| [`session_proto`](https://github.com/petrasvestartas/session_proto) | Protobuf schemas |

## Key API files

One file = one class (or one tightly-coupled group like `graph` which contains `Graph` + `Vertex` + `Edge`). Status reflects manual cross-language parity review (Python / Rust / C++).

- [ ] `aabb`
- [ ] `boolean_polyline`
- [ ] `brep`
- [ ] `bvh`
- [ ] `closest`
- [x] `color`
- [ ] `convex_hull`
- [ ] `element`
- [ ] `element_beam`
- [ ] `element_column`
- [ ] `element_plate`
- [ ] `file_encoders`
- [ ] `graph`
- [ ] `intersection`
- [ ] `kdtree`
- [x] `nurbsknot`
- [ ] `line`
- [ ] `matrix`
- [x] `mesh`
- [x] `nurbscurve`
- [x] `nurbssurface`
- [ ] `obb`
- [ ] `file_obj`
- [ ] `objects`
- [ ] `plane`
- [ ] `point`
- [ ] `pointcloud`
- [ ] `polyline`
- [ ] `primitives`
- [x] `quaternion`
- [x] `remesh_cdt`
- [x] `remesh_nurbssurface_adaptive`
- [x] `remesh_nurbssurface_grid`
- [ ] `rtree`
- [ ] `session`
- [ ] `session_config`
- [ ] `tolerance`
- [ ] `tree`
- [ ] `treenode`
- [ ] `nurbssurface_trimmed`
- [ ] `vector`
- [x] `xform`

## Prerequisites

| Tool | macOS | Ubuntu | Windows |
|------|-------|--------|---------|
| **CMake** | `brew install cmake` | `sudo apt install cmake` | [cmake.org](https://cmake.org/download/) |
| **Python 3.11+** | `brew install python` | `sudo apt install python3` | [python.org](https://python.org) |
| **Rust** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` | same | [rustup.rs](https://rustup.rs) |
| **Bun** | `brew install bun` | `curl -fsSL https://bun.sh/install \| bash` | [bun.sh](https://bun.sh) |
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

### Working with submodules

The language kernels and shared data/proto definitions are Git submodules:

| Submodule | Remote |
|-----------|--------|
| `session_cpp` | https://github.com/petrasvestartas/session_cpp.git |
| `session_py` | https://github.com/petrasvestartas/session_py.git |
| `session_rust` | https://github.com/petrasvestartas/session_rust.git |
| `session_rhino` | https://github.com/petrasvestartas/session_rhino.git |
| `session_data` | https://github.com/petrasvestartas/session_data.git |
| `session_proto` | https://github.com/petrasvestartas/session_proto.git |

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

## Cross-Repo Dependency Testing

Breaking changes in `session-py` or protobuf schemas are caught at two layers.

### Layer 1 — Before a breaking change ships

**Python API diff (`griffe`)** — runs in CI on every push to `main`. Compares the current branch against the last published PyPI version. Reports exactly which class, method, or parameter was removed or renamed with file path and line number.

**Protobuf schema diff (`buf`)** — runs in `session_proto` CI on every push/PR. Catches removed fields, renamed messages, and changed field numbers before they break serialization silently. Config: `session_proto/buf.yaml`.

### Layer 2 — After publish, verify downstream still works

**`dependents.json`** (session root) — registry of downstream repos. Add one entry per dependent:

```json
{
  "repos": [
    { "owner": "petrasvestartas", "repo": "PolygonEngineering", "workflow": "test.yml" }
  ]
}
```

After `deploy` succeeds on `main`, the **`dispatch` CI job** fires a `repository_dispatch` event (`session-py-updated`) to every repo listed. Each dependent runs its own test suite against the freshly published `session-py`. On failure it opens a GitHub issue in this repo with a direct link to the failed run.

**Template workflow** for dependent repos: `templates/session-compat.yml` — copy to `.github/workflows/session-compat.yml` in the dependent repo.

### Required secrets

| Secret | Repo | Purpose |
|--------|------|---------|
| `DOWNSTREAM_DISPATCH_TOKEN` | session | PAT with `repo` scope to trigger dispatches |
| `SESSION_ISSUE_TOKEN` | each dependent | PAT with `issues:write` on the session repo |

### Adding a new dependent repo

1. Add an entry to `dependents.json` via PR
2. Copy `templates/session-compat.yml` into the dependent repo as `.github/workflows/session-compat.yml`
3. Add the `SESSION_ISSUE_TOKEN` secret to the dependent repo
