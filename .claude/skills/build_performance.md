# Build Performance Guide

Reference for diagnosing and fixing slow builds in this multi-language geometry kernel.

## Bottleneck Rankings (worst first)

| # | Bottleneck | Lang | Time Cost | File:Line |
|---|-----------|------|-----------|-----------|
| 1 | Rust always `--release` (codegen-units=1, thin LTO) | Rust | 30-120s per build | `test_rust.sh:77`, `Cargo.toml:32-37` |
| 2 | C++ protobuf built from source (v33.2 + abseil) | C++ | 2-5 min first build | `CMakeLists.txt:87-110` |
| 3 | Rust opt-level=3 for ALL deps in dev mode | Rust | +30-50% dev builds | `Cargo.toml:29` |
| 4 | Rust build.rs watches entire proto dir (not per-file) | Rust | Full rebuild on any proto change | `build.rs:7` |
| 5 | Monolithic 51K-line Rust crate, 96 modules, no workspace | Rust | No parallel module compilation | `src/lib.rs` |
| 6 | 12 C++ files excluded from unity build | C++ | Slower incremental for those files | `CMakeLists.txt:48-61` |
| 7 | No sccache/ccache on Windows | C++ | MSVC recompiles from scratch | `CMakeLists.txt:66-76` |
| 8 | JSON consolidation + API index on every run | All | 2-5s overhead | `minitest.sh:184-207` |

## What IS Already Cached (does NOT rebuild)

- C++ protobuf: cached after first `cmake -S . -B build` (skip configure if `build/` exists)
- C++ cmake configure: skipped if `build/` dir exists (use `--clean` to force)
- Python proto: conditional on `.proto` timestamps vs `_pb2.py` timestamps
- Rust protoc binary: cached in `target/release/build/*/out/protoc/`
- Python venv: cached in `uvsession/`, `--fast` skips pip install
- C++ unity build: stable files batched into ~6 compilation units

## Quick Wins (config changes, no architecture refactor)

### 1. Rust: Add `--dev` flag to skip release mode
In `test_rust.sh:77`, the command is always `cargo run --release`. For iteration, debug builds with default `codegen-units=16` are 3-5x faster.

**Fix**: Add `--dev` flag to `test_rust.sh`:
```bash
if [[ "$DEV_MODE" == "true" ]]; then
    cargo run --bin minitest -j "$JOBS"
else
    cargo run --release --bin minitest -j "$JOBS"
fi
```

### 2. Rust: Remove opt-level=3 for deps in dev mode
`Cargo.toml:29` has `[profile.dev.package."*"] opt-level = 3` which recompiles all 137 dependencies with full optimization even in debug builds.

**Fix**: Remove or reduce to `opt-level = 1`:
```toml
[profile.dev.package."*"]
opt-level = 1  # was 3, saves 30-50% on dep compilation
```

### 3. Rust: Fix build.rs to watch individual .proto files
`build.rs:7` watches `../session_proto` (the directory), so ANY file change in that dir triggers a full Rust rebuild including all proto regeneration.

**Fix**: Replace directory watch with per-file watches:
```rust
// Remove: println!("cargo:rerun-if-changed={}", proto_dir);
// Add per-file:
for proto_file in &proto_files {
    println!("cargo:rerun-if-changed={}", proto_file);
}
```

### 4. C++: Install sccache on Windows
The CMakeLists.txt already auto-detects sccache/ccache (lines 66-76) but neither is typically installed on Windows.

**Fix**: `cargo install sccache` (uses Rust toolchain already present)

### 5. C++: Reduce unity build exclusions
`CMakeLists.txt:48-61` excludes 12 files from unity build. Files that are no longer under active development should be moved back into unity batches.

Current exclusions: nurbscurve, nurbssurface, trimmedsurface, knot, mesh, brep, mesh_iso, graph, tree, treenode, session, element

## Medium-Term Improvements

### Rust workspace split
Split monolithic `session_rust` into workspace crates:
- `session_core` (point, vector, plane, line, tolerance) — rarely changes
- `session_shapes` (mesh, nurbs, brep) — changes often
- `session_algorithms` (intersection, closest, bvh, rtree)
- `session_tests` (all test modules)

This enables parallel crate compilation and smaller incremental rebuilds.

### C++ pre-built protobuf
Replace FetchContent source build with pre-built protobuf binaries or vcpkg/conan package. Saves 2-5 minutes on first build. The source build exists for MSVC version compatibility but a pre-built package matched to the toolchain would work.

### Rust custom dev profile
Add a fast iteration profile to `Cargo.toml`:
```toml
[profile.dev-fast]
inherits = "dev"
opt-level = 0
debug = 1
# No [profile.dev-fast.package."*"] — deps stay unoptimized
```
Use with: `cargo run --profile dev-fast --bin minitest`

## Iteration Cheat Sheet

| Scenario | Command | Approx Time |
|----------|---------|-------------|
| Single Python class | `./bash/quicktest.sh point --py` | ~1s |
| All Python (fast) | `./bash/minitest.sh --py --fast --no-web` | ~5s |
| All Python | `./bash/minitest.sh --py --no-web` | ~10s |
| C++ incremental (1 file changed) | `./bash/minitest.sh --cpp --no-web` | ~5-15s |
| C++ clean build | `./bash/test_cpp.sh --clean --no-viewer` | 3-8 min |
| Rust incremental (release) | `./bash/minitest.sh --rust --no-web` | 30-120s |
| Rust incremental (with --dev fix) | `./bash/minitest.sh --rust --no-web` | ~10-30s |
| All languages parallel | `./bash/minitest.sh --no-web` | max(py, cpp, rust) |
| All + viewer | `./bash/minitest.sh` | max(py, cpp, rust) + 5s |

## Why Rust Is the Bottleneck

The Rust build is almost always the slowest step because:
1. **`--release` mode** with `codegen-units=1` forces single-threaded codegen and thin LTO
2. **51K lines in one crate** means any change recompiles the entire crate
3. **137 dependencies** all optimized at opt-level=3 even in dev mode
4. **Directory-level rerun-if-changed** triggers unnecessary full rebuilds

Fixing items 1, 3, and 4 (quick wins above) should cut Rust iteration time by ~60-70%.
