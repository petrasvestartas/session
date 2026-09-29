# 03 · Give the GPU three corners

**Plan about 1–2 hours.** 51 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Draw a pink triangle on the blue background.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Three shader positions → triangle coverage → fragment colour → canvas.

**Before you finish, explain:** If you change the top vertex, which part of the picture changes, and which stays the same?

The GPU can already paint an image. Today we ask it to paint a shape inside that image. We give it three corners and a colour, then connect them with a pipeline.

`triangle.wgsl` contains two small GPU programs. The vertex function places each corner. The fragment function chooses the colour of covered samples. `renderer.rs` connects them and records one draw. `browser.rs` only needs to tell the renderer the surface format.

![Three corner positions form a triangle; a fragment shader colours its covered pixels.](../illustrations/journey-03.svg)

For now, positions go directly into clip space: x and y near −1 and +1 reach the sides of the image. Our top corner is `(0.0, 0.6)`. The triangle stays away from the edges so its silhouette is easy to inspect. Perspective, world units and the camera come later.

A **pipeline** is a reusable drawing recipe. Compiling one per click would repeat setup unnecessarily, so we keep it in the renderer. `0..3` means 0, 1 and 2. The GPU calls the vertex function for those three indices. `0..1` asks for one instance.

`include_str!` puts the shader file’s text into the program at compile time; there is no second browser download. `Some` supplies an optional value and `None` leaves it absent. The empty `buffers` list means no vertex buffer supplies data yet. The pipeline still needs its two shader stages and a colour target, even for three hard-coded corners.

## Type the change

Continue [Ask the GPU to paint](02-clear.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-03-triangle`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/triangle.wgsl`

Create the shader. Rust runs on the CPU; these two functions run on the GPU. The fourth position component is 1 because we have not introduced perspective.

Create the file and type:

```wgsl
--8<-- "journey/code/03-triangle-01.wgsl"
```

### 2. `src/renderer.rs`

Keep the compiled drawing recipe beside the device and queue. It will be reused for every frame.

Find this exact block:

```rust
    pub queue: wgpu::Queue,
```

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-02.rs"
```

### 3. `src/renderer.rs`

Replace new. A pipeline connects the two shader entry points and the destination pixel format. No buffers are needed because the shader contains all three positions.

Find this exact block:

```rust
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }
```

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-03.rs"
```

### 4. `src/renderer.rs`

Name the pass and make its binding mutable so we can record drawing commands.

Find this exact block:

```rust
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-04.rs"
```

### 5. `src/renderer.rs`

After beginning the pass, select the recipe and draw vertices 0, 1 and 2 once. The upper end of a Rust range is excluded.

Find this exact block:

```rust
                ..Default::default()
            });
```

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-05.rs"
```

### 6. `src/browser.rs`

Pass the surface format to the renderer; a pipeline must agree with the texture it draws into.

Find this exact block:

```rust
    let renderer = Renderer::new(device, queue);
```

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-06.rs"
```

### 7. `src/browser.rs`

Update the visible result message.

Find this exact block:

```rust
    report("The GPU painted the canvas.");
```

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-07.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

A pink triangle appears on the blue background. Its top corner is above the centre. The background is still the render-pass clear from the previous lesson.

**Actual Chrome screenshot.**

![Actual browser result: Give the GPU three corners.](../screenshots/journey/03-triangle-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

## Try one small experiment

Predict the result of changing the last corner from `(0.0, 0.6)` to `(0.4, 0.6)`. Run it: the top should move right while the base stays still. Then change only the fragment colour. Explain why that changes the inside colour but not the silhouette. Restore both edits.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The triangle silhouette changes because its corner positions changed. The surrounding blue stays the same: the render pass clears the background before drawing the triangle.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 03-triangle
npm --prefix ../session_tests run course -- save 03-triangle
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The production viewer also separates shader programs, pipeline setup and frame recording. Later mesh buffers replace the shader’s three hard-coded positions. We will make that replacement explicitly, after understanding what the positions do.

[Validation status and course release](release.md).
