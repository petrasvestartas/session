# 10 · Keep the nearest surface

**Plan about 2–4 hours.** 123 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Draw two overlapping triangles in depth, keeping the nearer one visible even when it is drawn first.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Vertex z → transformed depth → depth comparison → colour written only for the nearer fragment.

**Before you finish, explain:** Why is it not enough to draw the far triangle first?

Place two coloured cards so they overlap. The card closer to your eye hides part of the other one. Until now our renderer only knew which draw happened last. Today we give each corner a depth and let the GPU keep the nearer surface.

The pink triangle is nearer, at z = 0.25. The turquoise one is farther, at z = 0.75. We deliberately draw pink first. If the later turquoise triangle covers their shared centre, the depth test is missing or wrong.

![Two fragments compete at one pixel; the depth test keeps 0.25 and rejects the later fragment at 0.75.](../illustrations/journey-10.svg)

For this lesson our matrix leaves z unchanged. WebGPU's visible depth interval is 0 through 1 after dividing by w. Smaller values are nearer. The depth texture starts at 1, the far end. `Less` accepts a fragment only when its depth is smaller than the stored value. Writing depth updates that stored value for later triangles.

A depth texture is an attachment alongside the colour texture. It is not a picture we show on the page. Its size and sample count must agree with the colour attachment. Both use the initial window size with one sample. `browser.rs` passes that size to `renderer.resize` before drawing the first frame; the renderer creates a matching depth texture. Lesson 20 will update both when the window changes size.

We also give each vertex a colour, so that you can see which triangle won. A vertex now occupies 24 bytes: three floats for position followed by three for colour. Location zero reads position; location one reads colour. The vertex shader passes colour to the fragment shader through a small output structure. All three corners of each triangle have the same colour here, so interpolation leaves it constant.

Opaque visibility is the purpose of this depth test. Transparent surfaces will need another treatment later. Keeping the two problems separate makes the first visibility rule clear.

## Type the change

Continue [Let one matrix describe the view](09-matrices.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-10-depth`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/triangle.wgsl`

Pass transformed 3D positions and vertex colours to the rasterizer. The fragment function uses the interpolated colour.

Find this exact block:

```wgsl
@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;

@vertex
fn vertex(@location(0) position: vec2<f32>) -> @builtin(position) vec4<f32> {
    return transform * vec4<f32>(position, 0.0, 1.0);
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(0.9, 0.25, 0.45, 1.0);
}
```

Replace that block with:

```wgsl
--8<-- "journey/code/10-depth-09.wgsl"
```

### 2. `src/renderer.rs`

Replace the flat diamond with two coloured triangles at different depths.

Find this exact block:

```rust
use wgpu::util::DeviceExt;

const POSITIONS: [[f32; 2]; 4] = [
    [-0.6, 0.0], [0.0, -0.6], [0.6, 0.0], [0.0, 0.6],
];
const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

pub struct Renderer {
    pub device: wgpu::Device,
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-fullscreen-3.rs"
```

### 3. `src/renderer.rs`

Replace the flat diamond with two coloured triangles at different depths.

Find this exact block:

```rust
    indices: wgpu::Buffer,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let bytes: Vec<u8> = POSITIONS.iter().flatten()
            .flat_map(|value| value.to_ne_bytes()).collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("rectangle positions"),
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-fullscreen-4.rs"
```

### 4. `src/renderer.rs`

Replace the flat diamond with two coloured triangles at different depths.

Find this exact block:

```rust
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 8,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-fullscreen-5.rs"
```

### 5. `src/renderer.rs`

Replace the flat diamond with two coloured triangles at different depths.

Find this exact block:

```rust
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-fullscreen-6.rs"
```

### 6. `src/renderer.rs`

Replace the flat diamond with two coloured triangles at different depths.

Find this exact block:

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        Self { device, queue, pipeline, vertices, indices, uniform, view_group }
    }

    pub fn draw(
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-fullscreen-7.rs"
```

### 7. `src/renderer.rs`

Replace the flat diamond with two coloured triangles at different depths.

Find this exact block:

```rust
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-fullscreen-8.rs"
```

### 8. `src/browser.rs`

Connect keep the nearest surface to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-window-1.rs"
```

### 9. `src/browser.rs`

Connect keep the nearest surface to the typed command path. Keep the scene state in its existing owner and redraw the dock after applying an action.

Find this exact block:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("A matrix carries pan, scale and rotation.");
    Ok(())
}
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-window-2.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Pink remains in front at the overlap, although turquoise is drawn afterward. Run `Pan Right`, `Zoom Out` and `Orbit Right`: moving the view must not reverse the depth relationship.

**Actual Chrome screenshot.**

![Actual browser result: Keep the nearest surface.](../screenshots/journey/10-depth-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Temporarily change the near triangle’s three z values from 0.25 to 0.9. Predict which colour will appear at the shared centre. Turquoise should now win. Restore the values, then reverse the two groups of indices: the picture should stay the same. Restore the original order before comparing.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A particular draw order might work for these two flat triangles, but a real scene can overlap differently from each viewpoint. A depth attachment remembers the nearest depth at each pixel. The comparison chooses visibility there, so drawing an opaque farther surface later does not cover a nearer one.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 10-depth
npm --prefix ../session_tests run course -- save 10-depth
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The finished viewer uses depth attachments for opaque visibility, then adds the rules needed for strokes, CAD boundaries and transparency. This lesson establishes why those rules must refer to visible surfaces rather than draw order.

[Validation status and course release](release.md).
