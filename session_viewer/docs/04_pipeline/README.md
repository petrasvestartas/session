# 04 Pipeline — first triangle

A standalone browser viewer that draws one hard-coded triangle (red/green/blue
corners, smoothly blended) on the grey background.
This is the starting point for the next chapter (05 Resize — stop the stretch).

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
Browser has no WebGPU). You should see a rainbow triangle on grey.

## What's inside

```
index.html                       # the web page — holds <canvas id="canvas">
Cargo.toml                       # crate + dependencies (trimmed to this chapter)
Trunk.toml                       # how `trunk` compiles Rust → wasm and serves it
.cargo/config.toml               # default build target = wasm32 (the browser)
src/
├── lib.rs                       # entry point: winit event loop + browser shell
├── state.rs                     # State — driven each frame; calls gpu.clear()
├── shaders/
│   └── triangle.wgsl            # the GPU program: 3 corners + colour
└── engine/
    ├── mod.rs                   # engine module index (gpu + pipelines)
    ├── gpu.rs                   # Gpu — device/queue/surface; clears + draws
    └── pipelines/
        ├── mod.rs               # Pipelines struct (holds the recipe)
        └── build.rs             # build_triangle_pipeline (the recipe)
```

Flow: `browser → lib.rs → state.rs → engine/gpu.rs → pipelines (triangle)`

## What changed vs 03 Window

- `src/shaders/triangle.wgsl` + `src/engine/pipelines/` are new.
- `engine/mod.rs` gained `pub mod pipelines;`.
- `gpu.rs` holds a `Pipelines` and the render pass now does
  `set_pipeline` + `draw(0..3, 0..1)` instead of only clearing.
