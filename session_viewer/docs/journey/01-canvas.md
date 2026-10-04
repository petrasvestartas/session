# 01 · A page that Rust can reach

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Create a white canvas that fills the window and let Rust write a startup message to the hidden status element. This checks browser startup before we use the GPU.

## Type

From `session_viewer`, run `npm --prefix ../session_tests run course -- init`. It supplies the dependency lock. Create `workspace/journey/src`, then type the three files below. Keep using this project throughout the course.

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

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The window is white. In the browser console, evaluate `document.getElementById("status").textContent`. It should say “Rust is running. The canvas is ready.” Rust has started; GPU drawing comes next.

**Verified checkpoint in Chrome.**

![Actual browser result: A page that Rust can reach.](../screenshots/journey/01-canvas-browser.png)

[Verification scope](release.md).

<details>
<summary>Code explanation and diagram</summary>

`index.html` owns the page; `Cargo.toml` declares libraries; `src/lib.rs` supplies the browser entry point. `Result<(), JsValue>` returns either success with no value or a browser error. `?` stops at a failed window, document or element lookup.

The canvas has no margin or scrolling. The hidden status keeps startup feedback out of the drawing. For unfamiliar syntax, see [values](../foundations/01-values.md) and [errors](../foundations/05-errors.md).

HTML canvas → WebAssembly entry point → hidden startup status.

![HTML fills the window with a white canvas; Rust records its startup result.](../illustrations/journey-01.svg)

Does seeing the canvas prove that the GPU has drawn anything?

No. HTML creates the canvas and CSS gives it a background. The status proves Rust ran; GPU commands begin in the next lesson.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change the sentence passed to `set_text_content`, save, and read the hidden status again in the browser console. The sentence changes while the white canvas stays still. Restore the original sentence before comparing your code.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 01-canvas
npm --prefix ../session_tests run course -- save 01-canvas
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Our entry point will grow into the maintained viewer’s `lib.rs`. Browser setup is deliberately small here; the later application needs winit event routing, resizing and device recovery.

A white canvas fills the window. Rust startup is checked through the hidden status; there is deliberately no visible heading or message.

[Full validation scope](release.md).

</details>
