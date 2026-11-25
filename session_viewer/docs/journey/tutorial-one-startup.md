# Tutorial 1 · Project architecture and browser initialization

## Goal

Create the two-crate Rust + WebAssembly workspace (`viewer-core` and `session_viewer`), configure Trunk's full-viewport `#canvas`, and start the `winit` browser event loop cleanly before asynchronous GPU initialization.

![Session Viewer startup shell with reference grid, RGB world axes, and Ready command dock](../screenshots/tutorials/t01-startup.png)

**What we achieve:** A clean full-viewport `#canvas` shell splitting pure-Rust domain state (`viewer-core`) from browser/GPU glue (`session_viewer`), ready to mount the reference grid, world coordinate triad, and bottom command dock as soon as WebGPU initializes.

## How it works

In the browser, `Trunk.toml` compiles the workspace to WebAssembly while `winit::EventLoop` must start synchronously before `navigator.gpu.requestAdapter()` resolves. Cloning `EventLoopProxy<Msg>` prior to `spawn_app` gives the async WebGPU task a typed channel back into the live window loop, while physical backing dimensions scale CSS layout pixels by `devicePixelRatio`.

![Two-crate workspace boundary, CSS-to-Retina DPR framebuffer scaling, 4 GiB WASM memory cap, and async EventLoopProxy handshake](../screenshots/tutorials/explain-01-startup.png)

## Compilable checkpoint slice

Tutorials 1 and 2 together form the first standalone compilable assembly (`frame`, `15` files).

When typing from an empty folder (`session_viewer/` + `viewer-core/`), start with these **minimal `frame` module roots** (`15` files total in the `frame` checkpoint) instead of the final 388-file `lib.rs`/`mod.rs` indexes below—that way `cargo check --target wasm32-unknown-unknown` compiles immediately at Tutorial 1:

```rust
// viewer-core/src/lib.rs (empty at checkpoint `frame`)

// src/lib.rs (at checkpoint `frame`)
/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    ($name:literal) => {
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

mod engine;
pub mod app;
#[cfg(all(target_arch = "wasm32", feature = "tutorial"))]
#[path = "../examples/course/entry.rs"]
mod course;

// src/engine/mod.rs
pub mod gpu;

// src/engine/gpu/mod.rs
pub mod device; // register:device

// src/app/mod.rs
#[cfg(target_arch = "wasm32")]
pub mod launch;

// examples/course/registration.rs
// --8<-- [start:course-registration]
#[path = "canvas.rs"]
mod canvas;
#[path = "frame.rs"]
mod frame;

type Start = fn() -> Result<(), wasm_bindgen::JsValue>;
pub(super) const STARTS: &[(&str, Start)] = &[
    ("frame", frame::start),
];
// --8<-- [end:course-registration]
```

## Core code walkthrough

### Pre-spawn `EventLoopProxy` handshake (`src/app/launch.rs:1:12`)

```rust
--8<-- "session_viewer/src/app/launch.rs:1:12"
```

Lines `8–10` solve a subtle browser/WebGPU lifetime constraint: `EventLoop::<Message>::with_user_event().build()?` owns the typed event channel, and `event_loop.create_proxy()` **must** be called before `event_loop.spawn_app(app)` consumes `event_loop` by value. Inside `ApplicationHandler::resumed`, `winit` only hands you `&ActiveEventLoop`, which cannot mint a `Message`-typed proxy for `wasm_bindgen_futures::spawn_local`.

### `Drawing` trait & `#canvas` error attribute (`examples/course/canvas.rs:13:46`)

```rust
--8<-- "session_viewer/examples/course/canvas.rs:13:46"
```

`Drawing` (lines `13–16`) isolates every tutorial checkpoint from `winit` plumbing: `draw()` receives the initialized `&DeviceSetup`, swapchain `&TextureView`, `devicePixelRatio` (`scale`), and monotonic timestamp (`now`), returning `true` only when animation requires another frame. If WebGPU adapter negotiation fails, `report()` (lines `38–46`) mirrors the error onto `#canvas[data-course-error]` so headless tests and users see the exact cause instead of a blank tab.

## Prerequisites and files

None — this first tutorial establishes the Cargo workspace, `.cargo/config.toml` default `wasm32-unknown-unknown` target, module index roots, and course harness dispatcher.

**Core files to type inline (6 files):**

1. `Cargo.toml`
2. `.cargo/config.toml`
3. `viewer-core/Cargo.toml`
4. `src/app/launch.rs`
5. `examples/course/index.html`
6. `examples/course/canvas.rs`

**Reference catalog & test files (8 files, folded below):** `Cargo.lock`, `viewer-core/src/lib.rs`, `src/lib.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`, `src/app/mod.rs`, `examples/course/registration.rs`, `examples/course/entry.rs`.

## Build

### Step 1 · `Cargo.toml`

`Cargo.toml` configures the `[workspace]` (`members = [".", "viewer-core"]`), compiles `session_viewer` as both `cdylib` (browser WASM) and `rlib` (native tests/CLI), and pins `wgpu = "29"`, `winit = "0.30"`, `egui = "0.34.3"`, `glyphon = "0.11"`, and `session_rust` (`viewer-display`). Trap: keep `panic = "abort"` and `opt-level = 3` in `[profile.release]` while leaving fat LTO to CI so incremental WASM checks stay fast.

```toml
--8<-- "session_viewer/Cargo.toml"
```

### Step 2 · `.cargo/config.toml`

`.cargo/config.toml` sets `build.target = "wasm32-unknown-unknown"`, caps WebAssembly linear memory at 4 GiB via `-C link-arg=--max-memory=4294967296`, and defines the `cargo xtest` alias (`test --workspace --target x86_64-unknown-linux-gnu`) for native CPU and Vulkan verification. Trap: running bare `cargo test` without `xtest` compiles tests for `wasm32`, where native-only `pollster` and filesystem fixtures do not exist.

```toml
--8<-- "session_viewer/.cargo/config.toml"
```

### Step 3 · `viewer-core/Cargo.toml`

`viewer-core/Cargo.toml` defines the UI- and GPU-independent core crate, depending only on `session_rust`, `bytemuck`, `prost`, `serde`, `serde_json`, `serde_yaml_ng`, and optional `toml`. Trap: never add `wgpu`, `winit`, `egui`, `web-sys`, `wasm-bindgen`, or `glyphon` here—`cargo tree -p viewer-core` must stay pure Rust.

```toml
--8<-- "session_viewer/viewer-core/Cargo.toml"
```

### Step 4 · `src/app/launch.rs`

`src/app/launch.rs` builds the `winit::event_loop::EventLoop<M>`, clones its `EventLoopProxy<M>` up front, and hands the constructed handler to `EventLoopExtWebSys::spawn_app`. Trap: `EventLoop::create_proxy` must run before `spawn_app` consumes the `EventLoop`, because `&ActiveEventLoop` inside `resumed` cannot mint typed user-event proxies.

```rust
--8<-- "session_viewer/src/app/launch.rs"
```

### Step 5 · `examples/course/index.html`

`examples/course/index.html` provides the minimal Trunk HTML shell used by cumulative tutorial checkpoints (`?tutorial=<id>`), mounting a full-viewport `#canvas` with `data-wasm-opt="0"` so debug and release tutorial bundles start identically. Trap: omitting `margin: 0` or `100vw`/`100vh` CSS sizing causes feedback resize loops when `devicePixelRatio` is fractional.

```html
--8<-- "session_viewer/examples/course/index.html"
```

### Step 6 · `examples/course/canvas.rs`

`examples/course/canvas.rs` implements the shared browser `#canvas` sizing, DPR clamping, and error-attribute reporting helper used by standalone tutorial checkpoints before `app/runtime.rs` arrives. Trap: always clamp framebuffer dimensions to `[1, max_texture_dimension_2d]` before `Surface::configure`, or a zero-sized hidden tab panics `wgpu`.

```rust
--8<-- "session_viewer/examples/course/canvas.rs"
```

## Reference catalog and test files (8 files)

The 6 core files above implement the complete mechanism for this lesson. Expand this reference section whenever you want to inspect or copy the remaining 8 sibling verbs, adapters, and unit-test suites (`Step 7`–`Step 14`).

### Step 7 · `Cargo.lock`

`Cargo.lock` records the resolved workspace dependency graph (`wgpu 29.0.4`, `naga 29.0.4`, `winit 0.30.13`, `egui 0.34.3`, `glyphon 0.11.0`, `prost 0.14`); Cargo always writes transitive platform crates (`ash` Vulkan, `objc2` macOS, `wayland-client`, `android-activity`) into one cross-platform lockfile, whereas `cargo build --target wasm32-unknown-unknown` compiles only the WebGPU/WebAssembly `cfg` subgraph into `.wasm`. Trap: upgrading `wgpu`, `egui-wgpu`, or `glyphon` independently breaks `TextureFormat` and `RenderPass` compatibility.

*Cargo generates `Cargo.lock` automatically from `Cargo.toml`; the auto-generated lockfile is omitted inline so this tutorial stays focused on handwritten source.*

### Step 8 · `viewer-core/src/lib.rs`

`viewer-core/src/lib.rs` exports the 35 pure-Rust domain modules (`camera`, `command`, `op`, `rows`, `scene`, `selection`, `session_io`, `snap`, `walk`, and geometry helpers) plus `GpuSink`. Trap: keep module visibility `pub` across the `viewer_core` crate boundary so both `session_viewer` (browser) and `session-viewer-cli` (headless) call the same APIs.

```rust
--8<-- "session_viewer/viewer-core/src/lib.rs"
```

### Step 9 · `src/lib.rs`

`src/lib.rs` defines the `shader!` macro over `OUT_DIR`, mounts `engine`, `app`, `command_dock`, `state`, `text_quality`, and `selftest`, and exports `#[wasm_bindgen(start)] pub fn run_web()` plus `reload_scene` and `execute_op`. Trap: gate `examples/course/entry.rs` behind `#[cfg(all(target_arch = "wasm32", feature = "tutorial"))]` so production WASM starts `app::run_web()` directly.

```rust
--8<-- "session_viewer/src/lib.rs"
```

### Step 10 · `src/engine/mod.rs`

`src/engine/mod.rs` wires the four rendering-engine subsystems—`diagnostics`, `gpu`, `performance`, and `pipelines`—under `crate::engine`. Trap: keep `engine` free of `crate::state` imports so GPU pipelines and headless self-tests compile without interactive UI state.

```rust
--8<-- "session_viewer/src/engine/mod.rs"
```

### Step 11 · `src/engine/gpu/mod.rs`

`src/engine/gpu/mod.rs` declares all 33 WebGPU resource, lane, pass, target, text, outline, SSAO, splat, clip, and `GpuSink` adapter modules under `engine::gpu` and re-exports `Gpu`. Trap: `completion.ts` filters these `mod` declarations per cumulative assembly, so keep one `mod <name>;` declaration per line.

```rust
--8<-- "session_viewer/src/engine/gpu/mod.rs"
```

### Step 12 · `src/app/mod.rs`

`src/app/mod.rs` mounts browser shell modules (`launch`, `runtime`, `mode`, `stage`, `input`, `touch`, `keys`, `gesture`, `ui`, `loader`, `live`, `fetch`) and re-exports shared `viewer_core` modules into `crate::app`. Trap: keep thin browser adapters (`decode`, `session_io`, `stream`, `cloud_query`, `sheet_query`) delegating to `viewer_core` rather than duplicating logic.

```rust
--8<-- "session_viewer/src/app/mod.rs"
```

### Step 13 · `examples/course/registration.rs`

`examples/course/registration.rs` holds the `(id, start)` dispatch table mapping `?tutorial=<id>` URL parameters to each cumulative checkpoint's entry function (`frame::start` through `crate::run_web`). Trap: `completion-audit.ts` enforces strictly additive growth here—each later assembly appends its entry without rewriting earlier registrations.

```rust
--8<-- "session_viewer/examples/course/registration.rs"
```

### Step 14 · `examples/course/entry.rs`

`examples/course/entry.rs` is compiled into `src/lib.rs` behind `#[cfg(all(target_arch = "wasm32", feature = "tutorial"))]` and dispatches `?tutorial=<id>` to the matching `STARTS` function in `registration.rs`. Trap: production builds omit the `tutorial` feature so `#[wasm_bindgen(start)]` enters `run_web()` directly with zero course-harness dead weight.

```rust
--8<-- "session_viewer/examples/course/entry.rs"
```

## Integration

`src/lib.rs` defines the `shader!` macro (reading preprocessed WGSL from `OUT_DIR`), re-exports `viewer_core` (`core`, `op`, `rows`, `camera`), and gates `examples/course/entry.rs` behind `#[cfg(all(target_arch = "wasm32", feature = "tutorial"))]`. Every later tutorial appends declarations and `// register:TAG` lines to `viewer-core/src/lib.rs`, `src/app/mod.rs`, and `src/engine/gpu/mod.rs` without rewriting earlier files.

## Run and observe

From `session/session_viewer`, verify that the pure-Rust core crate has zero GPU/browser dependencies and compile the `wasm32` startup shell:

```sh
cargo tree -p viewer-core --depth 1
cargo check --target wasm32-unknown-unknown --features tutorial -j8
```

`viewer-core` lists only `session_rust`, `bytemuck`, `prost`, and `serde*` (`0` `wgpu`/`winit`/`egui`/`web-sys` crates), and the `wasm32` target compiles cleanly. Once you add `frame.rs` in Tutorial 2, `trunk serve --features tutorial --open /?tutorial=frame` mounts `#canvas` at full `clientWidth × devicePixelRatio` resolution.

## Predict and try

Predict what happens if you run plain `cargo test` instead of `cargo xtest` from `session_viewer/`: because `.cargo/config.toml` sets `build.target = "wasm32-unknown-unknown"`, Cargo compiles unit tests for `wasm32` and fails resolving native-only `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` (`pollster`). Run `cargo xtest -p viewer-core --lib -- --list | head` to confirm the alias switches cleanly to `x86_64-unknown-linux-gnu`.

## Question and answer

**Why must `EventLoopProxy<Msg>` be created and cloned before `EventLoop::spawn_app` starts, rather than inside `ApplicationHandler::resumed`?**

??? tip "Think first — click to reveal how to work it out and the answer"
    **How to work it out:** Trace `winit` 0.30's `EventLoop<T>` ownership in `src/app/launch.rs` and `examples/course/frame.rs`: `EventLoop::create_proxy(&self)` borrows the `EventLoop` itself, whereas `spawn_app` consumes `EventLoop` by value on WebAssembly and hands `ApplicationHandler::resumed` only an `&ActiveEventLoop` (which cannot mint typed `EventLoopProxy<Msg>` handles).

    **The answer:** `wgpu::Instance::request_adapter` and `Adapter::request_device` are `async` Futures on WebGPU (`Promise`-backed in the browser), while `ApplicationHandler::resumed` is synchronous. Cloning `EventLoopProxy<Msg>` before `spawn_app` gives `wasm_bindgen_futures::spawn_local` a channel to send the initialized `(Window, Gpu)` back into `ApplicationHandler::user_event` once the browser resolves the WebGPU promises.

## Validation

Verified on this host via `cargo check --target wasm32-unknown-unknown -j8` (`0` warnings), `cargo tree -p viewer-core --depth 1` (`0` UI/GPU crates), `npm --prefix ../session_tests run course -- active-audit`, and `DOCS_STRICT_LINKS=1` Vite documentation build. Interactive browser WebGPU rendering was not run on this Linux host because its Chromium software WebGPU adapter renders black.
