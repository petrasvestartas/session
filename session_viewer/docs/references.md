# What this is built on

Two kinds of thing belong on this page: the libraries the viewer actually links against, which you can check in `Cargo.toml`, and the ideas it borrows, which are explained here rather than linked away.

Where this viewer uses a known technique, the course draws it rather than linking to someone else's explanation of it.

## The libraries

Every version below is pinned in `session_viewer/Cargo.toml` and locked by `Cargo.lock`, which the course installs unmodified at lesson 00.

| Crate | Version | What it does here |
|---|---|---|
| `wgpu` | 29.0 | The WebGPU implementation. In the browser it is a thin layer over the browser's own WebGPU; natively it reaches Vulkan, Metal or DX12, which is how the same code runs under `cargo xtest`. |
| `winit` | 0.30 | The window and event loop. On the web it is the canvas and its pointer, keyboard and touch events. |
| `glyphon` | =0.11.0 | Glyph atlas and text renderer on top of wgpu. Pinned exactly: it is the release that matches wgpu 29. |
| `wasm-bindgen` 0.2, `wasm-bindgen-futures` 0.4, `web-sys` 0.3, `js-sys` 0.3 | The bridge to the browser: the DOM, `fetch`, `EventSource`, `performance.now`. |
| `bytemuck` | 1 | `Pod`/`Zeroable`, which is what lets a `#[repr(C)]` struct be viewed as bytes for the GPU without a copy. |
| `prost` | 0.14 | Protobuf decoding for the `.pb` documents. |
| `serde` 1.0, `serde_json` 1.0, `serde_yaml_ng` 0.10, `toml` =0.8.23 | Manifest parsing in three formats with one set of semantics — a test asserts the three agree — plus session JSON validation, sheet side tables and the live source. `toml` is pinned exactly. |
| `anyhow` | 1.0 | One error type at the boundaries, so `?` composes. |
| `log` 0.4, `console_log` 1.0, `console_error_panic_hook` 0.1.6 | Diagnostics that survive the wasm boundary. Without the panic hook a Rust panic reaches the console as `unreachable executed` and nothing else. |
| `getrandom` | 0.2 (`js`) | Randomness in the browser, where the usual system source does not exist. |
| `naga` | =29.0.4 (`wgsl-in`) | Parses every shader in the mirror tests, which is how `cargo xtest` catches a WGSL mistake without a GPU. |
| `pollster` | 0.4 (native only) | Blocks on the async device setup in the native harness, so the same code path serves the browser and the tests. |
| `session_rust` | path | The geometry kernel, shared with the C++ and Python implementations. It links wgpu for the shared display type `RenderVertex` and for the GPU buffers a `Mesh` caches. |

Trunk builds the page; `wasm-bindgen` generates the JavaScript that instantiates the module. Lesson 00 draws that chain.

![Four tools and four artefacts: cargo produces a .wasm a browser cannot load on its own, wasm-bindgen writes the JavaScript that can, Trunk assembles the page, and the browser runs start().](illustrations/toolchain.svg)

## Where the authority lives

Three documents decide what is legal, and no tutorial — including this one — overrides them:

- **The WebGPU specification** (`gpuweb.github.io/gpuweb/`) — what an adapter, device, pipeline and bind group are, and every validation rule your errors come from.
- **The WGSL specification** (`gpuweb.github.io/gpuweb/wgsl/`) — the shading language: types, alignment, entry points, builtins.
- **The `wgpu` API documentation** (`docs.rs/wgpu`) — the Rust shape of all of the above, version by version.

These URLs were not fetched while this page was written, so treat them as the place to look, not as a citation. Which of the three to open: an error at pipeline creation is a WebGPU rule, an error inside a shader is a WGSL rule, and a signature that does not match is an API question.

## The ideas, and where the course draws them


| Idea | The problem it solves | Drawn in | Built in |
|---|---|---|---|
| **Reverse-Z depth** | Float depth crowds its precision at the far plane, which is where you need it least. Swapping near and far puts the precision near the eye. | [frustum](02-camera.md), [ink-visibility](05-visibility.md) | `camera.rs` swaps the planes, every depth-testing `DepthMode` compares `Greater` or `GreaterEqual` |
| **Depth-gradient carried ink** | A thick stroke's fragments sit beside its axis and read the wrong surface's depth. The surface slope carries the depth from fragment to axis. | [ink-visibility](05-visibility.md) | `shaders/ink_visibility.wgsl` |
| **Finite-triangle visibility** | A depth plane is infinite; a triangle is not. Binning projected triangles into screen tiles lets an edge be hidden only by geometry that really covers it. | [finite-triangle](18-finite-visibility.md), [tiles](18-finite-visibility.md) | `engine/gpu/triangle_tiles.rs` |
| **Octahedral normal encoding** | A unit vector has two degrees of freedom, so it does not need three floats. Two 8-bit numbers are enough for shading and culling. | [normals](09-normals.md) | `app/walk/encode.rs`, `shaders/normals.wgsl` |
| **Newell's normal** | The cross product of the first two edges inverts on a reflex corner; a sum over all edges cannot. | [cad-contract](06-cad-contract.md) | `app/walk/mesh_topology.rs` |
| **Screen-space ribbons** | A line with a constant pixel width is not geometry with a width; it is a quad expanded in screen space, with exact coverage rather than a distance ramp. | [ribbon](04b-strokes.md), [joins](17-source-presentation.md) | `shaders/ribbon.wgsl` |
| **Eye-Dome Lighting** | A point cloud with no normals reads flat. Comparing a pixel's depth with its neighbours' gives shape for free. | [splat-resolve](04d-clouds.md) | `shaders/splat_resolve.wgsl` |
| **Octree level of detail** | Drawing every point of a large cloud is wasted work when their spacing projects below a pixel. | [lod](04d-clouds.md) | `engine/gpu/lod.rs` |
| **Prefix-sum allocation** | Per-tile lists with a fixed cap either waste memory or overflow. Counting first, then scanning into offsets, lets dense tiles borrow from sparse ones. | [tiles](18-finite-visibility.md) | `shaders/scan_triangle_tiles.wgsl` |
| **Coverage mask silhouettes** | An outline drawn as geometry fights the geometry it outlines. A coverage mask plus a dilation is one full-screen pass and always closes. | [masks](17-source-presentation.md) | `engine/gpu/surface_outline.rs` |
| **ID-buffer picking** | A CPU ray-caster is a second answer to "what is visible here" and will eventually disagree with the picture. Re-rendering ids cannot. | [picking](12-picking.md) | `engine/gpu/pick.rs` |
| **Camera rebasing** | f32 has about seven digits; CAD coordinates do not fit. Subtracting an anchor near the camera before converting keeps the digits that matter. | [spaces](02-camera.md) | `camera.rs`, `engine/gpu/objects.rs` |

## The CAD comparison

The one place this project does cite external source code line by line is the boundary-representation contract, because "what should a CAD viewer do with a trimmed face" is a question somebody else answered first and answered well. [The CAD design record](cad-design.md) lists the exact OCCT files and revisions the contract was compared against, with commit-pinned links, and says plainly what was taken and what was decided differently.

## Reading this repository instead

The most reliable reference for this viewer is the viewer. Three places answer most questions faster than a search:

- [The map](map.md) — where any file sits, and what it is allowed to know.
- `ARCHITECTURE.md` — the module graph, the owners table, one frame's passes in order, and the Rust ↔ WGSL interface tables.
- `cargo xtest` — the shader mirror tests. If you want to know whether Rust and WGSL still agree about a struct, run the test that asserts it rather than reading both.
