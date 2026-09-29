# 01 · A page that Rust can reach

**Plan about 1–2 hours.** 49 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Open a canvas and let Rust tell us it has started.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** HTML canvas → WebAssembly entry point → visible status.

**Before you finish, explain:** Does seeing the canvas prove that the GPU has drawn anything?

Think of the canvas as an empty sheet of paper. Today we put that sheet on the page and let Rust write a message above it. We are checking the connection before asking the GPU to draw.

You will make three files. `index.html` owns the page. `Cargo.toml` tells Rust which libraries the project uses. `src/lib.rs` contains the function the browser will call. There is no application state or renderer yet.

![HTML creates the canvas; WebAssembly starts Rust, which updates the status element.](../illustrations/journey-01.svg)

From `session_viewer`, create your project with `npm --prefix ../session_tests run course -- init`. This supplies only the fixed dependency lock. Create a `src` folder inside `workspace/journey`, then type the files below. All lessons use this same project.

`let` gives a value a name. `fn` defines a function. `Result<(), JsValue>` means “success with no returned value, or a browser error”. The `?` after a lookup stops the function if that lookup failed. Read each lookup as a question: do we have a window, then a document, then our status element?

The dependencies for the next lessons are listed now so this release can use one unchanged lock file. They are libraries, not hidden viewer code. `dev-dependencies` are used by the course's verification tools. You can explore [Rust values](../foundations/01-values.md) and [errors](../foundations/05-errors.md) before typing if these expressions are unfamiliar.

## Type the change

### 1. `Cargo.toml`

Create the manifest. cdylib makes browser-loadable code; rlib will let our native checks use the same renderer.

Create the file and type:

```toml
--8<-- "journey/code/01-canvas-01.toml"
```

### 2. `src/lib.rs`

Create Rust’s entry point. The browser calls start after loading WebAssembly; ? returns an error if a required element is missing.

Create the file and type:

```rust
--8<-- "journey/code/01-canvas-03.rs"
```

### 3. `index.html`

Keep the HTML page small. Feature input belongs to the command dock drawn inside the canvas.

Create the file and type:

```html
--8<-- "journey/code/01-canvas-page-1.html"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

The page says “Rust is running. The canvas is ready.” above a grey rectangle. This is HTML and CSS; it is not a GPU image yet.

**Actual Chrome screenshot.**

![Actual browser result: A page that Rust can reach.](../screenshots/journey/01-canvas-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

## Try one small experiment

Change only the sentence passed to `set_text_content`. Save and wait for Trunk to rebuild. Predict whether the canvas colour will change. It should not: the sentence belongs to Rust, while this background belongs to CSS. Restore the sentence before the source comparison.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

No. HTML creates the canvas and CSS gives it a background. The status proves Rust ran; GPU commands begin in the next lesson.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 01-canvas
npm --prefix ../session_tests run course -- save 01-canvas
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Our entry point will grow into the maintained viewer’s `lib.rs`. Browser setup is deliberately small here; the later application needs winit event routing, resizing and device recovery.

[Validation status and course release](release.md).
