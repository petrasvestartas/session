# Open a GPU frame

Open a WebGPU surface on `#canvas`, clear every pixel to white on the GPU queue, and resize cleanly across desktop and upright viewports.

![Four wgpu objects before any drawing: Instance, Surface, Adapter and Device + Queue](../illustrations/gpu-objects.svg)

![Tutorial 1 startup and white-frame submission loop](../illustrations/active/frame-clear.svg)

## Prerequisites and ownership

Install Rust `1.97.1`, `wasm32-unknown-unknown`, Trunk `0.21.14` and a WebGPU browser ([setup](../README.md)). Checkpoint `frame` starts from scratch with `13` files:

| File | Role |
| --- | --- |
| [Cargo.toml](../../Cargo.toml), [Cargo.lock](../../Cargo.lock) | Crate manifest (`cdylib` + `rlib`) and pinned dependency graph. |
| [src/lib.rs](../../src/lib.rs) | Crate root, `shader!` macro and `#[cfg(all(target_arch = "wasm32", feature = "tutorial"))]` entry gate. |
| [src/app/mod.rs](../../src/app/mod.rs), [src/engine/mod.rs](../../src/engine/mod.rs), [src/engine/gpu/mod.rs](../../src/engine/gpu/mod.rs) | Module index files extended strictly by additive `mod` declarations in later checkpoints. |
| [src/app/launch.rs](../../src/app/launch.rs) | Generic `winit` event-loop launcher passing `EventLoopProxy<Message>` into the app factory. |
| [src/engine/gpu/device.rs](../../src/engine/gpu/device.rs) | Production WebGPU/native adapter request, `256 MiB` storage limit cap, error capture and surface configuration. |
| [examples/course/index.html](../../examples/course/index.html) | Full-viewport `#canvas` entry compiled by Trunk with `--features tutorial`. |
| [examples/course/canvas.rs](../../examples/course/canvas.rs), [frame.rs](../../examples/course/frame.rs), [registration.rs](../../examples/course/registration.rs), [entry.rs](../../examples/course/entry.rs) | Browser window adapter, white render-pass clear, and `?tutorial=frame` URL dispatcher. |

- `src/lib.rs` declares the top-level modules, defines the compile-time `shader!` `OUT_DIR` include macro, and gates the course `entry.rs` behind `wasm32` plus the `tutorial` feature so native and production builds stay separate.
- `src/app/mod.rs` opens the application namespace with `pub mod launch;` and grows strictly by additive module declarations as stage, walk, decoding and scene modules join later checkpoints.
- `src/engine/mod.rs` anchors the rendering engine namespace with `pub mod gpu;`, keeping GPU setup isolated until pipeline caches, frame timing and diagnostics join in checkpoint `rows`.
- `src/engine/gpu/mod.rs` exposes `pub mod device;` first and adds `ui`, `buffers`, `hull`, `instance` and `objects` without rewriting existing GPU modules.

## Build

Materialize checkpoint `frame` into a fresh directory, or create the files below directly:

```sh
npm --prefix ../session_tests run course -- checkpoint frame ../session_viewer/target/learn-frame
```

### 1. Crate manifest and lock

`Cargo.toml` emits both `cdylib` for the browser `.wasm` bundle and `rlib` for native tests, linking `wgpu 29`, `winit 0.30`, `egui 0.34.3`, and `session_rust` with `default-features = false` and `viewer-display`.

`Cargo.lock` pins the exact dependency graph, including single-version `skrifa`/`swash` font crates, `wasm-bindgen` bindings, `web-sys` DOM features, JS `getrandom` on wasm, and native-only `pollster`.

```toml
--8<-- "session_viewer/Cargo.toml:1:26"
```

### 2. HTML canvas and event-loop launcher

Size the initial surface from `canvas.clientWidth * devicePixelRatio` before `winit`'s first resize callback arrives, and build the `EventLoopProxy` ahead of `spawn_app` so `async` GPU initialization can send `Ready` back without borrowing the loop across `.await`.

```html
--8<-- "session_viewer/examples/course/index.html:course-page"
```

`src/app/launch.rs` builds a typed `EventLoop<Message>`, hands a cloneable `EventLoopProxy<Message>` to the `FnOnce` constructor before `spawn_app`, and avoids blocking the browser main thread.

```rust
--8<-- "session_viewer/src/app/launch.rs"
```

```rust
--8<-- "session_viewer/examples/course/canvas.rs:course-canvas"
```

### 3. WebGPU device and surface

`src/engine/gpu/device.rs` selects `BROWSER_WEBGPU` on wasm or `PRIMARY` natively, caps `max_storage_buffer_binding_size` at `256 MiB`, prefers an sRGB surface format, and records the first uncaptured GPU error or device loss in `Arc<Mutex<Option<String>>>`.

```rust
--8<-- "session_viewer/src/engine/gpu/device.rs:20:154"
```

### 4. Clear pass and URL dispatcher

Drop `_pass` before calling `encoder.finish()`, submit the command buffer to `gpu.queue`, and return `false` so static frames sleep until resize or input.

```rust
--8<-- "session_viewer/examples/course/frame.rs:course-frame"
```

Read `?tutorial=` in `entry.rs` so every later checkpoint keeps `?tutorial=frame` runnable side-by-side:

```rust
--8<-- "session_viewer/examples/course/entry.rs:course-entry"
```

## Run and observe

Serve the checkpoint with Trunk (`CARGO_TARGET_DIR` shared across checkpoints) and open `http://127.0.0.1:8780/?tutorial=frame`:

```sh
trunk serve --port 8780
```

Every pixel clears to `#ffffff` with no `data-course-error` attribute on `#canvas`, and resizing to `390×844` reconfigures the surface cleanly without stale borders.

## Predict and try

Change `wgpu::Color::WHITE` in `frame.rs` to `wgpu::Color::BLACK` before rebuilding: predict that both initial load and upright resize stay solid `#000000`, verify in the browser, then restore `WHITE`.

## Question and answer

**Why does `device.create_command_encoder(...)` alone leave the canvas unchanged?**
`CommandEncoder` only records commands into CPU-side driver memory; nothing executes until `encoder.finish()` is passed to `queue.submit(...)` and the acquired `SurfaceTexture` calls `present()`.

## Validation

Verified by locked `wasm32-unknown-unknown` compilation, cumulative source-retention tests, and desktop (`1280×800`) + upright (`390×844`) checks in the recorded Windows Intel WebGPU environment ([receipt](release/completion-partials-windows-20261115.json)). Upright browser emulation does not substitute for physical-phone testing.
