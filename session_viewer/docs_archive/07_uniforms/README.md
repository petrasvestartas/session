# 07 Uniforms & bind groups — a value that changes every frame

A standalone browser viewer that draws the vertex-buffer triangle and **pulses its
colour** by sending a per-frame `time` value from Rust into the shader.
This is the starting point for the next chapter (08 MVP matrix — a real camera).

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
Browser has no WebGPU). The triangle keeps its shape (aspect) and gradient (vertex
buffer), and its colours fade up and down on a ~3-second cycle (time uniform).

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
│   └── triangle.wgsl            # reads vertex inputs; aspect (group 0) + time (group 1)
└── engine/
    ├── mod.rs                   # engine module index (gpu + pipelines)
    ├── gpu.rs                   # Gpu — buffers + bind groups; ticks & uploads time each frame
    └── pipelines/
        ├── mod.rs               # Pipelines struct (takes both bind-group layouts)
        └── build.rs             # Vertex type + build_triangle_pipeline (2 bind-group layouts)
```

Flow: `browser → lib.rs → state.rs → engine/gpu.rs → pipelines (triangle)`

## What changed vs 06 Vertex Buffer

- A second uniform, `time: f32`, in its **own bind group** `@group(1)` (fragment stage),
  alongside the existing `aspect` uniform in `@group(0)` (vertex stage).
  - `shaders/triangle.wgsl` — declares `@group(1) @binding(0) var<uniform> time` and uses
    it in `fs_main` to scale the colour: `in.color * (0.5 + 0.5 * sin(time * 2.0))`.
  - `gpu.rs` — a `time` buffer + bind group; advanced (`self.time += 1/60`) and uploaded
    with `queue.write_buffer` every frame in `clear()`, then bound with `set_bind_group(1, …)`.
  - `pipelines/{mod,build}.rs` — pass the second bind-group layout into the pipeline.
- Key idea: a uniform is one value shared by the whole draw but cheap to overwrite each
  frame — the path the camera matrix takes next chapter.
