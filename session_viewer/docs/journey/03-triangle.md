# 03 · Give the GPU three corners

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Draw one pink triangle inside the white GPU frame. The shader supplies three corners; the renderer keeps one pipeline and records its draw.

The vertex shader places corners in clip space, where x and y near −1 and +1 reach the image edges. The fragment shader supplies colour. `0..3` draws three vertices; `0..1` draws one instance.

## Type

Continue from [Give browser presentation its own function](02-clear.md). [Save or recover your work](recovery.md).

### 1. `src/triangle.wgsl`

Create GPU vertex and fragment functions; the vertex function chooses a corner and the fragment function returns pink.

Create the file and type:

```wgsl
--8<-- "journey/code/03-triangle-01.wgsl"
```

### 2. `src/renderer.rs`

Keep the compiled drawing recipe beside the device and queue. It will be reused for every frame.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }

    pub fn draw(&self, view: &wgpu::TextureView) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-fullscreen-2.rs"
```

### 3. `src/renderer.rs`

Keep the compiled drawing recipe beside the device and queue. It will be reused for every frame.

<details>
<summary>Locate the existing block</summary>

```rust
                })],
                ..Default::default()
            });
        }
        self.queue.submit([encoder.finish()]);
    }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-fullscreen-3.rs"
```

### 4. `src/browser.rs`

Pass the surface format to the renderer; a pipeline must agree with the texture it draws into.

<details>
<summary>Locate the existing block</summary>

```rust
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue);
    present(&surface, &renderer)?;
    report("The GPU painted the canvas.");
    Ok(())
}
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/03-triangle-window-1.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

A pink triangle appears on white. Change its top vertex in the shader, save, and check that only the triangle’s shape changes. Restore the vertex.

**Verified checkpoint in Chrome.**

![Actual browser result: Give the GPU three corners.](../screenshots/journey/03-triangle-browser.png)

[Verification scope](release.md).

<details>
<summary>Code explanation and diagram</summary>

A pipeline is the reusable recipe connecting these shaders to the colour target. Keep it between frames. `include_str!` embeds the WGSL text at compile time. An empty vertex-buffer list means the shader still supplies its own positions; Rust will supply geometry in lesson 05. Use the same sRGB view format as browser presentation.

Three shader positions → triangle coverage → fragment colour → canvas.

![Three corner positions form a triangle; a fragment shader colours its covered pixels.](../illustrations/journey-03.svg)

If you change the top vertex, which part of the picture changes, and which stays the same?

The triangle silhouette changes because its corner positions changed. The surrounding white stays the same: the render pass clears the background before drawing the triangle.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the result of changing the last corner from `(0.0, 0.6)` to `(0.4, 0.6)`. Run it: the top should move right while the base stays still. Then change only the fragment colour. Explain why that changes the inside colour but not the silhouette. Restore both edits.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 03-triangle
npm --prefix ../session_tests run course -- save 03-triangle
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The production viewer also separates shader programs, pipeline setup and frame recording. Later mesh buffers replace the shader’s three hard-coded positions. We will make that replacement explicitly, after understanding what the positions do.



[Full validation scope](release.md).

</details>
