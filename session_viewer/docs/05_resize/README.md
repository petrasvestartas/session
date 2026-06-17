# 04 Resize — crisp, no stretch

A standalone browser viewer that draws the triangle and keeps it correctly shaped at
any window size (no stretch) and sharp on high-DPI screens.
This is the starting point for the next chapter (05 Vertex Buffer — data on the GPU).

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
Browser has no WebGPU). The triangle is evenly shaped and re-sharpens when you resize
the window or move it between monitors with different scaling.

## What's inside

```
index.html                       # the web page — holds <canvas id="canvas">
Cargo.toml                       # crate + dependencies
Trunk.toml                       # how `trunk` compiles Rust → wasm and serves it
.cargo/config.toml               # default build target = wasm32 (the browser)
src/
├── lib.rs                       # winit/browser shell + desired_canvas_size() sizing
├── state.rs                     # State — driven each frame; resize() + render()
├── shaders/
│   └── triangle.wgsl            # the GPU program: 3 corners + colour
└── engine/
    ├── mod.rs                   # engine module index (gpu + pipelines)
    ├── gpu.rs                   # Gpu — device/queue/surface; resize configures it
    └── pipelines/
        ├── mod.rs               # Pipelines struct (holds the recipe)
        └── build.rs             # build_triangle_pipeline (the recipe)
```

Flow: `browser → lib.rs → state.rs → engine/gpu.rs → pipelines (triangle)`

## What changed vs 04 Pipeline

- `lib.rs` only: a new `desired_canvas_size()` helper (canvas CSS size ×
  devicePixelRatio), used in `user_event` (initial sizing) and every frame in
  `RedrawRequested` so the GPU surface always matches the canvas's real pixel size.
- No other files change — `gpu.rs`'s `resize()` already configures the surface.
