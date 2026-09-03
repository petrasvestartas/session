# 03 Window — runnable skeleton

A standalone browser viewer that opens a canvas and clears it to grey.
This is the starting point for the next chapter (04 Pipeline — draw a triangle).

## Prerequisites (once)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Run

```bash
trunk serve            # builds wasm + serves at http://localhost:8770
```

Open http://localhost:8770 — you should see a plain grey window.

## What's inside

```
index.html             # the web page — holds <canvas id="canvas">
Cargo.toml             # crate + dependencies (trimmed to this chapter)
Trunk.toml             # how `trunk` compiles Rust → wasm and serves it
.cargo/config.toml     # default build target = wasm32 (the browser)
src/
├── lib.rs             # entry point: winit event loop + browser shell
├── state.rs           # State — the thing the event loop drives each frame
└── engine/
    ├── mod.rs         # engine module index
    └── gpu.rs         # Gpu — device / queue / surface; clears the frame
```

Flow: `browser → lib.rs → state.rs → engine/gpu.rs`
