# What this is built on

Two kinds of references belong here: the pinned crates and packages the viewer workspace links against (verifiable in `Cargo.toml`, `viewer-core/Cargo.toml`, `Cargo.lock`, and `session_mcp/pyproject.toml`), and the authoritative specifications and rendering algorithms implemented across [Tutorials 1–17](README.md).

## Workspace crates and pinned versions

Every Rust version below is pinned in `session_viewer/Cargo.toml` / `viewer-core/Cargo.toml` and locked in `Cargo.lock` (Rust `1.97.1`, `resolver = "3"`):

| Crate / package | Version | Role and owning tutorial |
|---|---|---|
| `viewer-core` | workspace path | UI- and GPU-independent core (`Scene`, `walk::*`, `CommandHost`, `HeadlessHost`, `SelectionState`, `op::Op`, `session_io`); links zero `wgpu`/`winit`/`egui`/`web-sys` crates ([Tutorial 1](journey/tutorial-one-startup.md), [Tutorial 14](journey/tutorial-fourteen-fastmcp.md)). |
| `wgpu` | `29.0.4` | WebGPU implementation in the browser and Vulkan / Metal / DX12 natively under `cargo xtest` and `session-viewer-cli` ([Tutorial 2](journey/tutorial-two-wgpu.md)). |
| `naga` | `=29.0.4` (`wgsl-in`) | Offline WGSL parser and layout validator run by `cargo xtest` without needing a GPU ([Tutorial 3](journey/tutorial-three-rendering.md)). |
| `winit` | `0.30.13` | Cross-platform window and canvas pointer, wheel, keyboard, and touch event loop ([Tutorial 1](journey/tutorial-one-startup.md), [Tutorial 5](journey/tutorial-five-camera.md)). |
| `egui`, `egui-wgpu` | `=0.34.3` | Immediate-mode 2D UI tessellation and overlay renderer for the mathematics stage, virtualized layer/properties panels, and command dock ([Tutorial 3.1](journey/tutorial-three-painting.md), [Tutorial 9](journey/tutorial-nine-layers.md), [Tutorial 13](journey/tutorial-thirteen-commands.md)). |
| `glyphon` | `=0.11.0` | `cosmic-text` shaping and GPU glyph atlas for 3D world-plane, screen-plate, and outlined scene labels ([Tutorial 10](journey/tutorial-ten-display.md)). |
| `swash`, `ttf-parser` | `0.2`, `0.25` | OpenType `MATH` table metrics, glyph rasterization, and Euclidean alpha-halo dilation for `1600×900` stage slides ([Tutorial 3.2](journey/tutorial-three-mathematics.md)). |
| `wasm-bindgen` `0.2`, `wasm-bindgen-futures` `0.4`, `web-sys` `0.3`, `js-sys` `0.3` | pinned in `Cargo.lock` | Browser DOM, `fetch`, HTTP `Range` streaming, `EventSource`, `Performance`, and `execute_op` / `postMessage` automation ([Tutorial 1](journey/tutorial-one-startup.md), [Tutorial 14](journey/tutorial-fourteen-fastmcp.md), [Tutorial 15](journey/tutorial-fifteen-formats.md)). |
| `bytemuck` | `1` | Zero-copy `Pod`/`Zeroable` views over `#[repr(C)]` GPU structs (`PackedVertex` `20 B`, `Instance` `112 B`) ([Tutorial 4](journey/tutorial-four-geometry.md)). |
| `prost` | `0.14` | Protobuf wire encoding/decoding for `.pb` geometry documents and `.session` archives ([Tutorial 15](journey/tutorial-fifteen-formats.md)). |
| `serde` `1.0`, `serde_json` `1.0`, `serde_yaml_ng` `0.10`, `toml` `=0.8.23` | pinned in `Cargo.lock` | Manifest parsing across YAML/JSON/TOML, stage deck parsing, and typed JSON `Op` serialization ([Tutorial 3.2](journey/tutorial-three-mathematics.md), [Tutorial 14](journey/tutorial-fourteen-fastmcp.md), [Tutorial 15](journey/tutorial-fifteen-formats.md)). |
| `pollster` | `0.4` (native) | Synchronous executor driving `Gpu::new_headless` in `cargo xtest` and `session-viewer-cli` ([Tutorial 2](journey/tutorial-two-wgpu.md), [Tutorial 13](journey/tutorial-thirteen-commands.md)). |
| `session_rust` | path (`../../session_rust`) | Shared CAD/structural geometry kernel (`Mesh`, `BRep`, `NurbsCurve`, `NurbsSurface`, `PointCloud`, `Element`, `Session`) ([Tutorial 4](journey/tutorial-four-geometry.md), [Tutorial 6](journey/tutorial-six-modeling.md)). |
| `session_mcp` | `0.1.0` (`../session_mcp`) | Python `FastMCP` server (`31` tools, `3` `scene://` resources) driving `session-viewer-cli serve --stdio` ([Tutorial 14](journey/tutorial-fourteen-fastmcp.md)). |

## Authoritative external specifications and guides

Check API signatures and validation rules against these primary sources for the pinned versions above:

- **WebGPU Specification** (`https://www.w3.org/TR/webgpu/` / `https://gpuweb.github.io/gpuweb/`) — adapter/device limits, texture usage flags, buffer alignment (`256 B` row pitch), and render/compute pass rules ([Tutorial 2](journey/tutorial-two-wgpu.md), [Tutorial 12](journey/tutorial-twelve-visibility.md)).
- **WGSL Specification** (`https://www.w3.org/TR/WGSL/` / `https://gpuweb.github.io/gpuweb/wgsl/`) — uniform/storage address-space layout, `atomicMax`, subgroup-free prefix scans, and derivative uniformity ([Tutorial 3](journey/tutorial-three-rendering.md), [Tutorial 12](journey/tutorial-twelve-visibility.md)).
- **MDN WebGPU API Reference** (`https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API`) — browser `GPUCanvasContext`, device-loss handling, and `devicePixelRatio` sizing ([Tutorial 1](journey/tutorial-one-startup.md), [Tutorial 2](journey/tutorial-two-wgpu.md)).
- **`wgpu` `29.x` Rust Documentation** (`https://docs.rs/wgpu/29.0.4/wgpu/`) — Rust types for `wgpu 29.0.4` (`PollType::Wait`, `TexelCopyBufferInfo`, `RenderPass::forget_lifetime`) ([Tutorial 2](journey/tutorial-two-wgpu.md), [Tutorial 3.1](journey/tutorial-three-painting.md)).
- **WebGPU Fundamentals & WebGPU Samples** (`https://webgpufundamentals.org/`, `https://webgpu.github.io/webgpu-samples/`) — practical patterns for bind-group organization, reversed-Z depth, and compute prefix sums ([Tutorial 3](journey/tutorial-three-rendering.md), [Tutorial 12](journey/tutorial-twelve-visibility.md)).
- **Force Density Method (Linkwitz & Schek, 1971 / Schek, 1974)** — branch-node equilibrium `D_N x_N = p - D_F x_F` solved and animated in `stage/net.rs` ([Tutorial 3.2](journey/tutorial-three-mathematics.md), [Tutorial 17](journey/tutorial-seventeen-application.md)).

## Rendering and geometry techniques in this codebase

| Technique | Problem solved | Tutorial | Production module(s) |
|---|---|---|---|
| **Reverse-Z depth (`[1, 0]`)** | Cancels perspective `1/z` clustering against IEEE-754 `f32` exponent spacing near `0.0`. | [Tutorial 2](journey/tutorial-two-wgpu.md), [Tutorial 5](journey/tutorial-five-camera.md) | `viewer-core/src/camera.rs`, `src/engine/gpu/targets.rs` |
| **Camera-anchored rebasing** | Subtracts an `f64` anchor near `camera.target` before `f32` conversion so kilometre-scale survey coordinates keep sub-mm precision. | [Tutorial 4](journey/tutorial-four-geometry.md), [Tutorial 5](journey/tutorial-five-camera.md) | `src/engine/gpu/objects/table.rs`, `src/shaders/scene.wgsl` |
| **Octahedral unit normals** | Packs unit normals into two `snorm8` bytes inside the `20 B` `PackedVertex`. | [Tutorial 3](journey/tutorial-three-rendering.md), [Tutorial 4](journey/tutorial-four-geometry.md) | `viewer-core/src/rows/encode.rs`, `src/shaders/normals.wgsl` |
| **Newell polygon normal** | Computes robust face orientation across concave and reflex polygon loops without inverting on collinear corners. | [Tutorial 4](journey/tutorial-four-geometry.md) | `viewer-core/src/walk/mesh_topology.rs` |
| **Screen-space ribbons & `24×0.30` arrows** | Expands constant-pixel-width antialiased strokes and Force-Density arrowheads with `1.0 px` tuck overlap and zero base seam. | [Tutorial 4](journey/tutorial-four-geometry.md) | `src/shaders/ribbon.wgsl`, `src/shaders/arrow.wgsl`, `src/engine/gpu/vectors.rs` |
| **Prefix-sum tile binning + `atomicMax` depth** | Bins projected triangles into screen tiles with frontmost depth early-out (`-59.4%` ink time on `view_local.yaml`). | [Tutorial 12](journey/tutorial-twelve-visibility.md) | `src/engine/gpu/triangle_tiles.rs`, `src/shaders/ink_visibility.wgsl` |
| **Coverage-mask silhouettes & GTAO** | Dilates binary solid coverage for gap-free selection/surface outlines and integrates horizon slices for `Arctic` shading. | [Tutorial 10](journey/tutorial-ten-display.md) | `src/engine/gpu/surface_outline.rs`, `src/engine/gpu/ssao.rs` |
| **Octree LOD & Eye-Dome Lighting** | Frustum-culls point-cloud octants, halts splitting below `2.0 px` spacing, and shades normal-free splats from depth jumps. | [Tutorial 11](journey/tutorial-eleven-clouds.md) | `src/engine/gpu/lod.rs`, `src/shaders/splat_resolve.wgsl` |
| **Generation-stamped `Rg32Uint` ID picking** | Re-renders exact `(row + 1, sub)` IDs in a cursor window with non-cancelling hover pre-selection. | [Tutorial 7](journey/tutorial-seven-picking.md) | `src/engine/gpu/pick.rs`, `viewer-core/src/selection.rs` |
| **Single `Op` seam over `CommandHost`** | Executes identical geometry/layer/view operations from the dock, native CLI, browser `execute_op`, and Python FastMCP. | [Tutorial 14](journey/tutorial-fourteen-fastmcp.md) | `viewer-core/src/op.rs`, `session_mcp/src/session_mcp/server.py` |

## CAD boundary-representation comparison

[The CAD design record](cad-design.md) documents the commit-pinned Open CASCADE (`OCCT`) comparison behind the BRep trim, crease, and silhouette rules in `viewer-core/src/walk/brep*.rs`, alongside [`ARCHITECTURE.md`](../ARCHITECTURE.md) and [`operations.md`](operations.md).
