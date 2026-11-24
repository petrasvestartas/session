# Tutorial 1 · Project architecture and browser initialization

## Goal

Create the two-crate Rust + WebAssembly workspace (`viewer-core` and `session_viewer`), configure Trunk's full-viewport `#canvas`, and start the `winit` browser event loop cleanly before asynchronous GPU initialization.

## Diagram

![Startup sequence and workspace split from `index.html` and `Trunk.toml` through `run_web()` and `EventLoopProxy<Msg>` into `viewer-core` and `session_viewer` (native wgpu readback of stage deck)](../../examples/stage/tutorials-foundations/01-startup.png)

Stage deck source: [`01-startup.yaml`](../../examples/stage/tutorials-foundations/01-startup.yaml).

## Prerequisites and files

None — this first tutorial establishes the Cargo workspace, `.cargo/config.toml` default `wasm32-unknown-unknown` target, module index roots, and course harness dispatcher.

Owned files in build order (`14` files):

1. `Cargo.toml`
2. `Cargo.lock`
3. `.cargo/config.toml`
4. `viewer-core/Cargo.toml`
5. `viewer-core/src/lib.rs`
6. `src/lib.rs`
7. `src/engine/mod.rs`
8. `src/engine/gpu/mod.rs`
9. `src/app/mod.rs`
10. `src/app/launch.rs`
11. `examples/course/index.html`
12. `examples/course/canvas.rs`
13. `examples/course/registration.rs`
14. `examples/course/entry.rs`

## Build

### Step 1 · `Cargo.toml`

Write `Cargo.toml` with both `cdylib` (for the browser `.wasm` target) and `rlib` (for native tests), linking `wgpu 29`, `winit 0.30`, `egui 0.34.3`, and `session_rust` with `default-features = false` and `viewer-display`. Copy `Cargo.lock` alongside it so `skrifa`/`swash` font crates, `wasm-bindgen`, `web-sys` DOM bindings, JS `getrandom` on wasm, and native-only `pollster` stay locked to exact versions. Keep this `101`-line configuration synchronized with the workspace toolchain and deterministic build rules. Keep this `97`-line configuration synchronized with the workspace toolchain and deterministic build rules.

```toml
--8<-- "session_viewer/Cargo.toml"
```

### Step 2 · `Cargo.lock`

`Cargo.lock` pins exact dependency versions across the workspace (`wgpu 29.0.4`, `naga 29.0.4`, `winit 0.30.13`, `egui 0.34.3`, `glyphon 0.11.0`, `prost 0.14`) so every checkpoint and CI run resolves identical crate sources. Trap: upgrading `wgpu`, `egui-wgpu` or `glyphon` independently breaks `TextureFormat` and `RenderPass` lifetime compatibility.

```toml
--8<-- "session_viewer/Cargo.lock"
```

### Step 3 · `.cargo/config.toml`

`.cargo/config.toml` sets `build.target = "wasm32-unknown-unknown"`, caps WebAssembly linear memory at 4 GiB via `-C link-arg=--max-memory=4294967296`, and defines the `cargo xtest` alias (`test --workspace --target x86_64-unknown-linux-gnu`) for native CPU and Vulkan verification. Trap: running bare `cargo test` without `xtest` compiles tests for `wasm32`, where native-only `pollster` and filesystem fixtures do not exist.

```toml
--8<-- "session_viewer/.cargo/config.toml"
```

### Step 4 · `viewer-core/Cargo.toml`

Create `src/lib.rs` plus the three additive module roots (`src/app/mod.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`), alongside `viewer-core/Cargo.toml` and `viewer-core/src/lib.rs` so the UI- and GPU-free `viewer-core` workspace member grows additively from Tutorial 5 onward without `wgpu`, `winit`, `egui` or `web-sys` dependencies: Keep this `22`-line configuration synchronized with the workspace toolchain and deterministic build rules. Keep this `22`-line configuration synchronized with the workspace toolchain and deterministic build rules.

```toml
--8<-- "session_viewer/viewer-core/Cargo.toml"
```

### Step 5 · `viewer-core/src/lib.rs`

Create `src/lib.rs` plus the three additive module roots (`src/app/mod.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`), alongside `viewer-core/Cargo.toml` and `viewer-core/src/lib.rs` so the UI- and GPU-free `viewer-core` workspace member grows additively from Tutorial 5 onward without `wgpu`, `winit`, `egui` or `web-sys` dependencies: Spanning `40` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `40` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/lib.rs"
```

### Step 6 · `src/lib.rs`

Create `src/lib.rs` plus the three additive module roots (`src/app/mod.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`), alongside `viewer-core/Cargo.toml` and `viewer-core/src/lib.rs` so the UI- and GPU-free `viewer-core` workspace member grows additively from Tutorial 5 onward without `wgpu`, `winit`, `egui` or `web-sys` dependencies: Spanning `37` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `37` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/lib.rs"
```

### Step 7 · `src/engine/mod.rs`

Create `src/lib.rs` plus the three additive module roots (`src/app/mod.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`), alongside `viewer-core/Cargo.toml` and `viewer-core/src/lib.rs` so the UI- and GPU-free `viewer-core` workspace member grows additively from Tutorial 5 onward without `wgpu`, `winit`, `egui` or `web-sys` dependencies: Spanning `6` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `6` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/mod.rs"
```

### Step 8 · `src/engine/gpu/mod.rs`

Create `src/lib.rs` plus the three additive module roots (`src/app/mod.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`), alongside `viewer-core/Cargo.toml` and `viewer-core/src/lib.rs` so the UI- and GPU-free `viewer-core` workspace member grows additively from Tutorial 5 onward without `wgpu`, `winit`, `egui` or `web-sys` dependencies: Spanning `61` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `61` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/mod.rs"
```

### Step 9 · `src/app/mod.rs`

Create `src/lib.rs` plus the three additive module roots (`src/app/mod.rs`, `src/engine/mod.rs`, `src/engine/gpu/mod.rs`), alongside `viewer-core/Cargo.toml` and `viewer-core/src/lib.rs` so the UI- and GPU-free `viewer-core` workspace member grows additively from Tutorial 5 onward without `wgpu`, `winit`, `egui` or `web-sys` dependencies: Spanning `60` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `60` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/mod.rs"
```

### Step 10 · `src/app/launch.rs`

Create `examples/course/index.html` with a full-viewport `#canvas` element, then add `src/app/launch.rs` so `EventLoopProxy<Message>` is cloned *before* `spawn_app` starts the `async` WebGPU handshake without borrowing the event loop across `.await`: Spanning `12` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `12` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/launch.rs"
```

### Step 11 · `examples/course/index.html`

`examples/course/index.html` provides the minimal Trunk HTML shell used by cumulative tutorial checkpoints (`?tutorial=<id>`), mounting a full-viewport `#canvas` with `data-wasm-opt="0"` so debug and release tutorial bundles start identically. Trap: omitting `margin: 0` or `100vw`/`100vh` CSS sizing causes feedback resize loops when `devicePixelRatio` is fractional.

```html
--8<-- "session_viewer/examples/course/index.html"
```

### Step 12 · `examples/course/canvas.rs`

`examples/course/canvas.rs` implements the shared browser `#canvas` sizing, DPR clamping, and error-attribute reporting helper used by standalone tutorial checkpoints before `app/runtime.rs` arrives. Trap: always clamp framebuffer dimensions to `[1, max_texture_dimension_2d]` before `Surface::configure`, or a zero-sized hidden tab panics `wgpu`.

```rust
--8<-- "session_viewer/examples/course/canvas.rs"
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

Materialize the `frame` assembly (`npm --prefix ../session_tests run course -- checkpoint frame target/learn-frame`) and run `cargo check --target wasm32-unknown-unknown -j8` plus `cargo tree -p viewer-core --depth 1`: `viewer-core` compiles with zero `wgpu`, `winit`, `egui`, `web-sys`, `wasm-bindgen`, or `glyphon` dependencies, and `#canvas` mounts at `clientWidth * devicePixelRatio` with no `data-course-error` attribute.

## Predict and try

Predict what happens if you run plain `cargo test` instead of `cargo xtest` from `session_viewer/`: because `.cargo/config.toml` sets `build.target = "wasm32-unknown-unknown"`, Cargo compiles unit tests for `wasm32` and fails resolving native-only `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` (`pollster`). Run `cargo xtest -p viewer-core --lib -- --list | head` to confirm the alias switches cleanly to `x86_64-unknown-linux-gnu`.

## Question and answer

**Why must `EventLoopProxy<Msg>` be created and cloned before `EventLoop::spawn_app` starts, rather than inside `ApplicationHandler::resumed`?**

**How to work it out:** Trace `winit` 0.30's `EventLoop<T>` ownership in `src/app/launch.rs` and `examples/course/frame.rs`: `EventLoop::create_proxy(&self)` borrows the `EventLoop` itself, whereas `spawn_app` consumes `EventLoop` by value on WebAssembly and hands `ApplicationHandler::resumed` only an `&ActiveEventLoop` (which cannot mint typed `EventLoopProxy<Msg>` handles).

**The answer:** `wgpu::Instance::request_adapter` and `Adapter::request_device` are `async` Futures on WebGPU (`Promise`-backed in the browser), while `ApplicationHandler::resumed` is synchronous. Cloning `EventLoopProxy<Msg>` before `spawn_app` gives `wasm_bindgen_futures::spawn_local` a channel to send the initialized `(Window, Gpu)` back into `ApplicationHandler::user_event` once the browser resolves the WebGPU promises.

## Validation

Verified on this host via `cargo check --target wasm32-unknown-unknown -j8` (`0` warnings), `cargo tree -p viewer-core --depth 1` (`0` UI/GPU crates), `npm --prefix ../session_tests run course -- active-audit`, and `DOCS_STRICT_LINKS=1` Vite documentation build. Interactive browser WebGPU rendering was not run on this Linux host because its Chromium software WebGPU adapter renders black.
