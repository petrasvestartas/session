# 07 · Send one view setting to every corner

**Combined study estimate: 2–3 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–32 minutes.** 35 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Scale and shift the diamond without changing its stored positions.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** View values → uniform buffer → bind group 0 → shader binding 0 → transformed position.

**Before you finish, explain:** If the diamond moves, must its position buffer have changed?

Let us move our diamond without rewriting its corners. Keep the original drawing on the desk and imagine sliding a frame over it. We need a second kind of data: a setting shared by every vertex in a draw.

A **uniform buffer** supplies that shared setting. Our four numbers are horizontal scale, vertical scale, horizontal offset and vertical offset. The shader calculates `position * scale + offset`. Try one corner on paper: the right corner starts at `(0.6, 0.0)`. Scaling by 0.75 and adding `(0.25, 0.15)` puts it at `(0.70, 0.15)`.

![View values travel through a uniform buffer and bind group; positions still arrive through the vertex buffer.](../illustrations/journey-07.svg)

The vertex buffer changes its input for each corner. The uniform holds the same values throughout this draw. The renderer knows how to upload those numbers, while the browser currently supplies the choice. Later the camera will calculate them.

A **bind group** connects actual resources to the slots expected by a shader. Here group zero contains binding zero, our uniform buffer. The pipeline derives its layout from the shader, and `get_bind_group_layout(0)` lets us use that exact layout when creating the group. This keeps the little example readable; the full renderer uses explicit layouts when several pipelines share resources.

A WGSL `vec4<f32>` occupies 16 bytes and requires 16-byte alignment. Four consecutive Rust floats give us those 16 bytes. We start at byte zero. Do not assume every Rust structure and shader structure will agree automatically: padding becomes important when we introduce matrices and larger records.

The buffer has two usages. `UNIFORM` lets a shader read it as uniform data. `COPY_DST` permits queue uploads to change its contents. `write_buffer` schedules an upload before the following queue submission; we do not need to rebuild the pipeline to change a view setting.

## Type the change

Continue [Share a corner between triangles](06-indices.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-07-uniforms`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/triangle.wgsl`

Declare the uniform resource before the vertex function. Its group and binding identify a resource slot.

Find this exact block:

```wgsl
@vertex
```

Replace that block with:

```wgsl
--8<-- "journey/code/07-uniforms-01.wgsl"
```

### 2. `src/triangle.wgsl`

Use the first pair as scale and the second pair as offset. The swizzles xy and zw select those pairs.

Find this exact block:

```wgsl
    return vec4<f32>(position, 0.0, 1.0);
```

Replace that block with:

```wgsl
--8<-- "journey/code/07-uniforms-02.wgsl"
```

### 3. `src/renderer.rs`

Retain the uniform for uploads and the bind group for draws.

Find this exact block:

```rust
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
}

impl Renderer {
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-fullscreen-6.rs"
```

### 4. `src/renderer.rs`

Retain the uniform for uploads and the bind group for draws.

Find this exact block:

```rust
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline, vertices, indices }
    }

    pub fn draw(&self, view: &wgpu::TextureView, background: &crate::background::Background) {
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-fullscreen-7.rs"
```

### 5. `src/renderer.rs`

Retain the uniform for uploads and the bind group for draws.

Find this exact block:

```rust
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-fullscreen-8.rs"
```

### 6. `src/browser.rs`

Connect send one view setting to every corner to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    );
    panel.update(None, &canvas)?;
    let mut background = Background::default();
    present(&surface, &renderer, &background, &mut panel)?;
    let input_canvas = canvas.clone();
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let line = match panel.update(Some(&event), &input_canvas) {
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-window-1.rs"
```

### 7. `src/browser.rs`

Connect send one view setting to every corner to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
            report(&format!("Cannot lay out commands: {error:?}"));
            return;
        }
        if let Err(error) = present(&surface, &renderer, &background, &mut panel) {
            report(&format!("Cannot redraw: {error:?}"));
        }
    });
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-window-2.rs"
```

### 8. `src/browser.rs`

Connect send one view setting to every corner to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Four shared corners make two triangles.");
    Ok(())
}
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-window-3.rs"
```

### 9. `src/browser.rs`

Connect send one view setting to every corner to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-window-4.rs"
```

### 10. `src/browser.rs`

Connect send one view setting to every corner to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view, background);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
```

Replace that block with:

```rust
--8<-- "journey/code/07-uniforms-window-5.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

The diamond becomes smaller and moves toward the upper right. `Background` still changes only the background. Its stored positions and indices are unchanged.

**Actual Chrome screenshot.**

![Actual browser result: Send one view setting to every corner.](../screenshots/journey/07-uniforms-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

## Try one small experiment

Predict the result of changing only the first scale from 0.75 to 0.25. The diamond should become narrower without becoming shorter. Then restore it and set the horizontal offset to -0.25: the diamond moves left. Explain why neither experiment edits POSITIONS or INDICES.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

No. The position buffer still contains the same four corners. A separate uniform supplies scale and offset. Every vertex invocation reads those shared values and transforms its corner before the GPU decides which pixels are covered.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 07-uniforms
npm --prefix ../session_tests run course -- save 07-uniforms
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The full viewer sends camera and viewport information through frame uniforms. This is the same boundary: application state calculates values; the renderer uploads them; shaders use them while drawing unchanged geometry.

[Validation status and course release](release.md).
