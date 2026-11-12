# Rust and the first GPU frame

Create a browser canvas and paint its first frame through wgpu.

## Prerequisites and ownership

Install the course's Rust, WebAssembly and Trunk prerequisites. This subject introduces `Cargo.toml`, `index.html`, `src/lib.rs`, `src/browser.rs` and `src/renderer.rs`.

This is the active [first-frame checkpoint](../01-canvas.md). It replaces the former `01-canvas`, `01a-gpu` and `02-clear` executable steps while preserving their cumulative endpoint. Later rendering subjects still replace these files, so their integration must be designed before this subject can satisfy the finished-file rule.

## Build

<a id="checkpoint-01-canvas"></a>

<details>
<summary>Implementation 01-canvas: Rust and the first GPU frame</summary>

A GPU adapter identifies a usable backend; its device creates resources and its queue accepts work. A surface connects those resources to the HTML canvas. CSS dimensions are logical pixels; multiply by device pixel ratio and clamp to the device limit before configuring the texture.

### 1. `Cargo.toml`

Create the manifest with the locked wgpu and browser dependencies.

Create the file and type:

```toml
--8<-- "journey/code/01-frame-1.toml"
```

### 2. `index.html`

Create the full-window canvas and hidden startup status.

Create the file and type:

```html
--8<-- "journey/code/01-frame-2.html"
```

### 3. `src/lib.rs`

Start asynchronous GPU setup from the WebAssembly entry point.

Create the file and type:

```rust
--8<-- "journey/code/01-frame-3.rs"
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

</details>

## Run and observe

From `session_viewer`, run `npm --prefix ../session_tests run course -- init` to supply the dependency lock. Put the five files above in `workspace/journey`, preserving their paths. In that project, run `trunk serve --port 8780` with the workspace build guards and shared Cargo target described in [Maintaining](../../maintaining.md).

Open `http://127.0.0.1:8780/`. The GPU clears the canvas to white. The hidden status element contains “The GPU painted the canvas.” A white page alone does not prove GPU startup; inspect that status and check the browser console for errors. This checkpoint draws once; reload after resizing.

## Predict and try

Predict the result of setting the clear colour's green and blue components to zero. Change those two values in `Renderer::draw`, reload, and check that the entire canvas becomes red. Restore the original values before continuing.

## Question and answer

Why are submission and presentation separate?

Submission sends recorded drawing commands to the GPU. Presentation makes the acquired surface texture available for display; neither operation replaces the other.

## Validation

Apart from named teaching markers, the five source files match the reconstructed `02-clear` output. The active checkpoint passes a WebAssembly build, native 640 × 480 GPU readback, and browser checks at 1280 × 720 and 390 × 844 (density 2). Every pixel matches white; rebuilding the prediction experiment produces red in both viewports. These checks use software Vulkan/SwiftShader, not hardware-driver validation. Source equality does not establish runtime success or finished-file ownership.
