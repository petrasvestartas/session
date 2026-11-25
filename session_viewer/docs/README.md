# Build and understand the Session Viewer

Reconstruct the complete Session Viewer from an empty Rust + WebAssembly workspace (`viewer-core` and `session_viewer`) to the full `388`-file production CAD, presentation-stage, CLI, and FastMCP system (`32` WGSL shaders). Every production file appears **in full once** via a live `--8<--` source include, and every tutorial diagram is rendered directly from a Session Viewer stage deck (`examples/stage/tutorials-*/`).

## Prepare your computer

Install Rust `1.97.1`, `wasm32-unknown-unknown` and Trunk `0.21.14`:

```sh
rustup toolchain install 1.97.1 && rustup default 1.97.1
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
```

## 17-Tutorial Implementation Manual

1. **[Tutorial 1 · Project architecture and browser initialization](journey/tutorial-one-startup.md)** — `Cargo.toml`, `viewer-core` split, `.cargo/config.toml`, and `winit` browser startup (`14` files).
2. **[Tutorial 2 · WebGPU and `wgpu` initialization](journey/tutorial-two-wgpu.md)** — adapter, `256 MiB` device cap, first-error latch, reverse-Z/MSAA/ID targets, and frame pacing (`9` files).
3. **[Tutorial 3 · Rendering engine and shared shaders](journey/tutorial-three-rendering.md)** — `build.rs` `#include` assembly, `PRELUDE`, bind groups `0..=3`, lazy pipelines, and pass list (`19` files), with **[3.1 Compositing `egui` shapes and text](journey/tutorial-three-painting.md)** (`2` files) and **[3.2 Laying out and animating 2D mathematics](journey/tutorial-three-mathematics.md)** (`23` files).
4. **[Tutorial 4 · Scene rows, `112 B` instances and `f64` anchor rebasing](journey/tutorial-four-geometry.md)** (`36` files), with **[4.1 Mesh and BRep solid walkers](journey/tutorial-four-meshes.md)** (`25` files) and **[4.2 Curve ribbons, arrows, vectors and point glyphs](journey/tutorial-four-strokes.md)** (`14` files).
5. **[Tutorial 5 · Camera and navigation](journey/tutorial-five-camera.md)** — reverse-Z `Camera` in metres, `f64` camera-anchor rebasing, orbit/pan/zoom/touch gestures, `State`, and `runtime.rs` (`10` files).
6. **[Tutorial 6 · Construction planes (`CPlane`), `OSnap` and interactive `Tool`s](journey/tutorial-six-modeling.md)** (`23` files), with **[6.1 Parametric CAD primitives, transforms and mass queries](journey/tutorial-six-primitives.md)** (`36` files) and **[6.2 NURBS surfacing and `Split` / `Trim` / `Extend`](journey/tutorial-six-surfacing.md)** (`20` files).
7. **[Tutorial 7 · Picking and selection](journey/tutorial-seven-picking.md)** — rate-limited hover picks, `Rg32Uint` ID window readbacks, and unified `SelectionState` (`14` files).
8. **[Tutorial 8 · Gumball widget and interactive handles](journey/tutorial-eight-gumball.md)** — screen-constant 13-handle gizmo (`World`/`Object`/`CPlane`), CPU aperture hits, and single-commit drags (`8` files).
9. **[Tutorial 9 · Layers, hierarchy, panels and instancing](journey/tutorial-nine-layers.md)** — document trees, virtualized `Layers`/`Properties`/`Graph` panels, `APPEARANCE` undo, and `InstanceRef` sharing (`25` files).
10. **[Tutorial 10 · Display modes: outlines, Arctic SSAO, attributes and scene text](journey/tutorial-ten-display.md)** — coverage-mask silhouettes, GTAO `ssao.wgsl`, element attributes, and `glyphon` text lanes (`30` files).
11. **[Tutorial 11 · Point clouds, octree LOD and sheets](journey/tutorial-eleven-clouds.md)** — clip-frustum octree LOD (`lod_px = 2.0`), EDL splats, `250k` streamed chunks, and cancellable queries (`17` files).
12. **[Tutorial 12 · Transparency, finite-triangle tile visibility and section caps](journey/tutorial-twelve-visibility.md)** — `atomicMax` tile depth bounds, prefix-sum triangle lists, per-object `Alpha`, and stencil section caps (`14` files).
13. **[Tutorial 13 · Command dock, completion, history and native CLI](journey/tutorial-thirteen-commands.md)** — trait-decoupled `command_dock`, mobile IME `agent.rs`, and native `session-viewer-cli` (`17` files).
14. **[Tutorial 14 · Shared `Op` API, browser `execute_op` and FastMCP server](journey/tutorial-fourteen-fastmcp.md)** — `49` serde `Op`s over `CommandHost`, WASM `execute_op`/`postMessage`, and Python `session_mcp` (`10` files).
15. **[Tutorial 15 · File formats, bounded protobuf decode, `.session` archives and live reload](journey/tutorial-fifteen-formats.md)** — YAML/JSON/TOML manifests, `<= 512 MiB` wire validation, canonical `.session` archives, `.obj` export, and ETag/SSE reload (`23` files).
16. **[Tutorial 16 · Performance tiers, GPU timing, diagnostics and headless self-tests](journey/tutorial-sixteen-performance.md)** — `[p50, p95, p99]` `FrameStats`, `NavigationQuality` tiers, `PassTimer`, `?inspect=perf`, and `selftest::oracle` (`11` files).
17. **[Tutorial 17 · Complete application reconstruction and real-scene stage](journey/tutorial-seventeen-application.md)** — `StageApp + spatial::Scene3` 3D kernel bridge, camera TeX lens, `Trunk.toml`, and `index.html` (`8` files).
