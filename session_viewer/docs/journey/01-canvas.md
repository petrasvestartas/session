# 1.1.1 · Rust and the first GPU frame

**Chapter 1 · Browser, Rust and the GPU · Section 1.1 · Rust and the first GPU frame** · [Contents](contents.md)

**Typing: 58–116 minutes.** [Estimate](typing-load.md).

Plan several study sessions for this subject. Save at file boundaries; its executable checkpoint remains one complete subject.

Create one browser application with a manifest, HTML page, module root, renderer, surface adapter, fixed entry point and constructor registration table. Keep these files in the same project throughout the course.

## Type

A GPU adapter identifies a usable backend; its device creates resources and its queue accepts work. A surface connects those resources to the HTML canvas. CSS dimensions are logical pixels; multiply by device pixel ratio and clamp to the device limit before configuring the texture.

### 1. `Cargo.toml`

Create the manifest with the locked wgpu and browser dependencies.

Create the file and type:

```toml
--8<-- "journey/code/01-canvas-cumulative-kernel-feature-0.toml"
```

### 2. `index.html`

Create the full-window canvas and hidden startup status.

Create the file and type:

```html
--8<-- "journey/code/01-frame-2.html"
```

### 3. `src/lib.rs`

Register the browser, renderer and fixed WebAssembly entry modules.

Create the file and type:

```rust
--8<-- "journey/code/01-canvas-cumulative-2-lib.rs"
```

### 4. `src/renderer.rs`

Record and submit the clear pass.

Create the file and type:

```rust
--8<-- "journey/code/01-frame-4.rs"
```

### 5. `src/browser.rs`

Configure the bounded surface, draw and present its texture.

Create the file and type:

```rust
--8<-- "journey/code/01-frame-5.rs"
```

## Register startup

The fixed browser entry selects one constructor from the registration table and forwards startup errors to its error sink. A constructor returns a pinned future because GPU setup is asynchronous; the common return type lets the table hold different constructors. Only the chosen constructor runs. Later subjects register content here without replacing the entry implementation.

`Start` is a function-pointer type. Each table entry pairs a name with one such function; `&str` borrows the name without copying its bytes. An asynchronous function returns a future, which represents work that can pause while the GPU request is pending. `Box` gives that future an owned allocation, `Pin` keeps it at a stable address while it is polled, and `dyn Future` lets different constructors share this return type. `Result<(), JsValue>` carries either successful completion or a browser error into the common error sink.

Before adding another mode, predict what `?tutorial=missing-mode` will do. It must show a contextual startup error rather than silently choosing the viewer. Restore `?tutorial=viewer` to draw the white frame. This is the same runnable checkpoint, selected through a different URL input.

### `examples/course/registration.rs`

Register the page lifetime startup and constructor once.

```rust
--8<-- "journey/code/01-canvas-cumulative-registration.rs"
```

### `examples/course/entry.rs`

Register the page lifetime startup and constructor once.

```rust
--8<-- "journey/code/01-canvas-cumulative-entry.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The canvas remains white. Change its clear colour to `r: 0.9, g: 0.9, b: 0.9`: it becomes light grey through the sRGB view. Restore all three values to `1.0`.

<details>
<summary>Code explanation and diagram</summary>

The renderer records one clear pass. Its inner scope releases the encoder borrow before `finish` consumes the encoder. Submission schedules the commands; `present` releases the acquired frame for display.

The entry point starts asynchronous GPU setup without blocking the browser. Errors reach the developer console and the hidden status element. This checkpoint draws once: reload after resizing.

configure within device limits → present(surface, renderer) → independent draw → page feedback.



What starts the GPU work, and what makes its image appear in the canvas?

queue.submit sends the recorded commands to the GPU. The browser presents its canvas image automatically. We still call frame.present to mark the end of our surface frame through wgpu’s shared API; native surface backends use that call to request presentation.

Study estimate, including typing and experiments: 2–4 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the result of setting the clear colour’s green and blue components to zero. Change those values in Renderer::draw, reload, and verify a red canvas. Restore the original values.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 1.1.1
npm --prefix ../session_tests run course -- save 1.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This checkpoint introduces browser setup and GPU submission in one runnable subject. Later subjects still rewrite these files; finished-file ownership across the complete course remains unfinished.

Verify GPU startup and the clear-colour experiment; a white HTML page alone is not a rendered-frame check.

[Full validation scope](release.md).

</details>
