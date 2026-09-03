# 05 Vertex Buffer — data on the GPU

A standalone browser viewer that draws the same triangle, but its 3 corners now live
in a **GPU vertex buffer** uploaded from Rust instead of being hard-coded in the
shader. Same picture, real data pipeline.
This is the starting point for the next chapter (06 Uniforms & bind groups).

## Prerequisites (once)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Run

```bash
trunk serve            # builds wasm + serves at http://localhost:8770
```

Open http://localhost:8770 in a real browser (Chrome/Edge — the VSCode Simple
Browser has no WebGPU). You should see the same rainbow triangle. Proof it's
data-driven: edit a `position` or `color` in the `TRIANGLE` array in `gpu.rs`, save,
and it moves/recolours without touching the shader.

## What's inside

```
index.html                       # the web page — holds <canvas id="canvas">
Cargo.toml                       # crate + dependencies (bytemuck used here)
Trunk.toml                       # how `trunk` compiles Rust → wasm and serves it
.cargo/config.toml               # default build target = wasm32 (the browser)
src/
├── lib.rs                       # winit/browser shell + canvas sizing
├── state.rs                     # State — driven each frame
├── shaders/
│   └── triangle.wgsl            # now reads @location(0)/(1) vertex inputs
└── engine/
    ├── mod.rs                   # engine module index (gpu + pipelines)
    ├── gpu.rs                   # Gpu — owns the vertex_buffer; binds & draws it
    └── pipelines/
        ├── mod.rs               # Pipelines struct (holds the recipe)
        └── build.rs             # Vertex type + layout(); build_triangle_pipeline
```

Flow: `Rust [Vertex;3] → bytemuck → GPU buffer → shader @location inputs → pixels`

## What changed vs 04 Resize

- `build.rs` — new `Vertex` struct (`#[repr(C)]`, `Pod`) + `layout()`; the pipeline's
  `VertexState` now takes `buffers: &[Vertex::layout()]` instead of `&[]`.
- `triangle.wgsl` — `vs_main` receives `@location(0) position` / `@location(1) color`
  instead of indexing a hard-coded array by `vertex_index`.
- `gpu.rs` — creates a vertex buffer with `create_buffer_init` in `new()`, stores it on
  `Gpu`, and the render pass does `set_vertex_buffer(0, …)` before `draw`.

> In the archive these vertex types live in `engine/gpu/types.rs` (`MeshVertex`,
> `LineVertex`, `PointVertex`) and the buffer grows via a `GpuArena`. Same core
> (`create_buffer_init` / `cast_slice`), scaled up.
