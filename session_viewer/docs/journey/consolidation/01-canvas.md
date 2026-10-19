# Rust and the first GPU frame

Build a browser page whose drawing can be owned by Rust, then submit its first GPU frame. The visible result is a white canvas. The important change is who produced those pixels: first the page background, then a render pass submitted to a real GPU.

## From a page to an owned drawing operation

Create one project and keep it throughout the course. From `session_viewer`, run `npm --prefix ../session_tests run course -- init`, then create `workspace/journey/src`. The initialization supplies the dependency lock; the implementation below supplies the manifest, page and Rust modules.

`index.html` owns the canvas and its full-window CSS. Its hidden status element is a separate startup signal. `Cargo.toml` declares a `cdylib` for WebAssembly and an `rlib` so the same drawing code can be called natively. Rust's `#[wasm_bindgen(start)]` entry point looks up the document and writes the status. `Result<(), JsValue>` makes a missing element an explicit failure; `?` returns that failure to the caller.

This proves that the page can load Rust. It does not prove that a GPU has written the image. To cross that boundary, the entry point starts an asynchronous browser setup future. Adapter and device requests use `.await`, leaving the browser available while they complete. The canvas becomes a wgpu surface; the requested adapter must support that surface. The device creates resources and the queue accepts work.

We separate two owners immediately. `src/browser.rs` connects browser objects to a surface and obtains a surface image. `src/renderer.rs` owns the device and queue and borrows an image view for `draw`. That renderer is a learner module created here; the maintained viewer has no `src/renderer.rs` file.

The renderer creates a command encoder and opens a render pass whose color attachment clears to white and stores the result. The pass borrows the encoder. Its inner scope ends that borrow before `encoder.finish()` consumes the encoder to produce a command buffer. `queue.submit` sends the buffer to the GPU. The browser-side owner then calls `frame.present()` to finish the surface frame. These are distinct operations: recording, submission and presentation.

## Implement the startup and first clear

Apply the following implementation sections in order. They retain every original edit and snippet; an intermediate checkpoint is a useful place to save, not a separate reading assignment.

<a id="checkpoint-01-canvas"></a>

<details>
<summary>Implementation 01-canvas: A page that Rust can reach</summary>

### 1. `Cargo.toml`

Create the manifest. cdylib makes browser-loadable code; rlib will let our native checks use the same renderer.

Create the file and type:

```toml
--8<-- "journey/code/01-canvas-01.toml"
```

### 2. `index.html`

Let the canvas fill the window from the start. Keep startup information hidden; the documentation explains the lesson.

Create the file and type:

```html
--8<-- "journey/code/01-canvas-fullscreen-1.html"
```

### 3. `src/lib.rs`

Create Rust’s entry point. The browser calls start after loading WebAssembly; ? returns an error if a required element is missing.

Create the file and type:

```rust
--8<-- "journey/code/01-canvas-fullscreen-2.rs"
```

</details>

<a id="checkpoint-01a-gpu"></a>

<details>
<summary>Implementation 01a-gpu: Paint the first GPU frame</summary>

### 1. `src/lib.rs`

Start asynchronous GPU setup without blocking the browser. A setup error goes to the developer console.

<details>
<summary>Locate the existing block</summary>

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let status = document.get_element_by_id("status").ok_or("Missing status")?;
    status.set_text_content(Some("Rust is running. The canvas is ready."));
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/01a-gpu-01.rs"
```

### 2. `src/renderer.rs`

Own the device and queue; record and submit one white clear.

Create the file and type:

```rust
--8<-- "journey/code/01a-gpu-02.rs"
```

### 3. `src/browser.rs`

Connect the canvas to an adapter, configure its image and present the clear.

Create the file and type:

```rust
--8<-- "journey/code/01a-gpu-03.rs"
```

</details>

## Run and keep a checkpoint

After typing the implementation, run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 01a-gpu
npm --prefix ../session_tests run course -- check 1.1
npm --prefix ../session_tests run course -- save 01a-gpu
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it. The source comparison checks your typed project without replacing it. Restore experimental changes before comparing or saving; [recovery instructions](../recovery.md) explain how to return to a saved checkpoint.

## Result and a useful experiment

At endpoint `01a-gpu`, the canvas is white and startup reports “The GPU painted the canvas.” Temporarily change `wgpu::Color::WHITE` to `wgpu::Color::BLACK`: the canvas turns black. Restore white. The first page's white background could not establish this drawing path; changing the GPU clear does.

The maintained dependency path begins at [`lib.rs`](../../../src/lib.rs): `run_web` starts the winit `App`; its `resumed` callback binds the page canvas and starts `app::loader::boot`; a `Msg::Ready` transfers the resulting `State` back into the event loop. Our small async setup teaches that handoff before introducing the event loop and recovery machinery. GPU opening lives in [`engine/gpu/device.rs`](../../../src/engine/gpu/device.rs), where surface, device, queue and configuration have explicit owners.

Continue with [Browser presentation](02-clear.md).
