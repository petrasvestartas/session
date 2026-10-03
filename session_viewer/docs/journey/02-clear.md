# 02 · Ask the GPU to paint

**Combined study estimate: 2–3 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 44–87 minutes.** 106 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Splitting required:** this verified checkpoint exceeds the one-hour typing target. Smaller runnable lessons are still being prepared.

**Today:** Paint the whole canvas white with a real GPU command.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Browser canvas → surface → renderer records a clear → queue → presented image.

**Before you finish, explain:** What starts the GPU work, and what makes its image appear in the canvas?

Now let us give the GPU one simple instruction: clear the whole sheet to white. We do not need a triangle yet. Keeping these jobs separate will make a blank picture easier to diagnose.

There are two new owners. `browser.rs` owns page access and the drawing surface. `renderer.rs` owns the device and queue. The browser hands a texture view to the renderer; the renderer records a clear and submits it. The browser presents that image automatically. We also call wgpu’s present method to express the end of the surface frame; it is an explicit presentation request on native surface backends.

![The browser supplies a surface image; the renderer records a clear and submits commands before the browser presents it.](../illustrations/journey-02.svg)

An **adapter** is an available GPU connection. A **device** creates resources. A **queue** receives work. A **surface** supplies the images that can appear in the canvas. Follow those four values in `run` before reading individual descriptor options.

`struct` groups the values an owner keeps. `impl` groups its functions. `&self` borrows the renderer for one call. `async` allows a function to wait, and `.await` marks a wait for the browser's GPU request. It does not freeze the page.

The first edit revisits the entry point for a reason: GPU setup must wait asynchronously, while the first lesson only changed a DOM element. Your previous work is still the starting point.

`pub` lets another module use an item. `#[cfg(target_arch = "wasm32")]` includes browser code only in the browser build; our native checker uses the renderer alone. `map_err` converts a GPU error into the browser error type returned by `run`. The short `|error| ...` expressions are callbacks: functions called with that error. `Arc::new` gives wgpu a shared owner for its error callback so it remains valid after setup.

For descriptor fields, `Default::default()` selects the library’s ordinary settings. We will replace defaults when a lesson needs a different behavior. Today the choices that matter are the WebGPU backend, a compatible surface, the image size, and keeping the cleared result with `Store`. The named `_pass` stays alive until the closing brace; the underscore says we intentionally do not call methods on it yet.

We allow an sRGB texture view and use it when drawing. It converts linear colour values into the display encoding, matching our native reference images. The pipeline added next must use that same view format.

CSS controls how much space the canvas occupies. The GPU also needs image dimensions: read the window width and height, cap them at the device limit, and give that same size to the canvas and surface. For now one CSS pixel uses one drawing pixel. Lesson 20 adds display density and updates after a window resize.

## Type the change

Continue [A page that Rust can reach](01-canvas.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-02-clear`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/lib.rs`

Replace start’s file. Opening a GPU is asynchronous: the browser must remain free while we wait. The browser module owns page access; the renderer will also work without a page.

Find this exact block:

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

Replace that block with:

```rust
--8<-- "journey/code/02-clear-fullscreen-2.rs"
```

### 2. `src/renderer.rs`

Create the renderer. A clear is a real render operation, even though it has no triangle and needs no shader yet.

Create the file and type:

```rust
--8<-- "journey/code/02-clear-fullscreen-4.rs"
```

### 3. `src/browser.rs`

Create the browser adapter. It owns the surface, asks for a compatible GPU and gives one texture to the renderer. The renderer does not look up HTML elements.

Create the file and type:

```rust
--8<-- "journey/code/02-clear-window-1.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

The canvas stays white. Its pixels now come from a submitted GPU clear. A startup failure shows its reason over the canvas; successful startup adds no text. WebGPU needs a supporting browser and a secure context such as localhost.

**Actual Chrome screenshot.**

This freshly captured white image deliberately looks like lesson 01. The change is who paints it: the GPU now clears and presents the image. The native check verifies every background pixel.

![Actual browser result: Ask the GPU to paint.](../screenshots/journey/02-clear-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

## Try one small experiment

Temporarily change the renderer’s clear colour to `r: 0.9, g: 0.9, b: 0.9`. The canvas should become light grey: that confirms the GPU owns these pixels. Restore all three values to 1.0 for the white background.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

queue.submit sends the recorded commands to the GPU. The browser presents its canvas image automatically. We still call frame.present to mark the end of our surface frame through wgpu’s shared API; native surface backends use that call to request presentation.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 02-clear
npm --prefix ../session_tests run course -- save 02-clear
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

These owners become the production GPU device setup and presentation modules. The drawing operation stays independent of page lookup. Surface resizing, retry and recovery are later lessons, not hidden in this one.

[Validation status and course release](release.md).
