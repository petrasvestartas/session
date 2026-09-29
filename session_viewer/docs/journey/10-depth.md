# 10 · Keep the nearest surface

**Plan about 2–4 hours.** 62 lines to type, including comments and blank lines. Allow time to read, predict and experiment; this is an estimate, not a deadline.

**Today:** Draw two overlapping triangles in depth, keeping the nearer one visible even when it is drawn first.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Vertex z → transformed depth → depth comparison → colour written only for the nearer fragment.

**Before you finish, explain:** Why is it not enough to draw the far triangle first?

Place two coloured cards so they overlap. The card closer to your eye hides part of the other one. Until now our renderer only knew which draw happened last. Today we give each corner a depth and let the GPU keep the nearer surface.

The pink triangle is nearer, at z = 0.25. The turquoise one is farther, at z = 0.75. We deliberately draw pink first. If the later turquoise triangle covers their shared centre, the depth test is missing or wrong.

![Two fragments compete at one pixel; the depth test keeps 0.25 and rejects the later fragment at 0.75.](../illustrations/journey-10.svg)

For this lesson our matrix leaves z unchanged. WebGPU's visible depth interval is 0 through 1 after dividing by w. Smaller values are nearer. The depth texture starts at 1, the far end. `Less` accepts a fragment only when its depth is smaller than the stored value. Writing depth updates that stored value for later triangles.

A depth texture is an attachment alongside the colour texture. It is not a picture we show on the page. Its size and sample count must agree with the colour attachment. Both are currently 640 × 480 with one sample. We will make their sizes follow the window together in the resizing lesson.

We also give each vertex a colour, so that you can see which triangle won. A vertex now occupies 24 bytes: three floats for position followed by three for colour. Location zero reads position; location one reads colour. The vertex shader passes colour to the fragment shader through a small output structure. All three corners of each triangle have the same colour here, so interpolation leaves it constant.

Opaque visibility is the purpose of this depth test. Transparent surfaces will need another treatment later. Keeping the two problems separate makes the first visibility rule clear.

## Type the change

Continue [Let one matrix describe the view](09-matrices.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-10-depth`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/renderer.rs`

Replace the flat diamond with two coloured triangles at different depths.

Find this exact block:

```rust
const POSITIONS: [[f32; 2]; 4] = [
    [-0.6, 0.0], [0.0, -0.6], [0.6, 0.0], [0.0, 0.6],
];
const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-01.rs"
```

### 2. `src/renderer.rs`

Upload the new six-float records using the byte conversion you already know.

Find this exact block:

```rust
POSITIONS.iter().flatten()
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-02.rs"
```

### 3. `src/renderer.rs`

Retain a view of the depth attachment. The view keeps its underlying GPU texture alive.

Find this exact block:

```rust
    view_group: wgpu::BindGroup,
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-03.rs"
```

### 4. `src/renderer.rs`

Advance 24 bytes to the next vertex.

Find this exact block:

```rust
                    array_stride: 8,
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-04.rs"
```

### 5. `src/renderer.rs`

Read three position floats followed by three colour floats. The macro calculates the second offset as 12 bytes.

Find this exact block:

```rust
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-05.rs"
```

### 6. `src/renderer.rs`

Tell the pipeline to keep a fragment only when it is nearer, then write its depth.

Find this exact block:

```rust
            depth_stencil: None,
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-06.rs"
```

### 7. `src/renderer.rs`

Create the depth texture with the same dimensions and sample count as the colour image.

Find this exact block:

```rust
        Self { device, queue, pipeline, vertices, indices, uniform, view_group }
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-07.rs"
```

### 8. `src/renderer.rs`

Attach depth to the pass and clear it to the farthest value before drawing.

Find this exact block:

```rust
                label: Some("canvas"),
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-08.rs"
```

### 9. `src/triangle.wgsl`

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

### 10. `src/browser.rs`

Describe the visibility rule demonstrated by this checkpoint.

Find this exact block:

```rust
    report("A matrix carries pan, scale and rotation.");
```

Replace that block with:

```rust
--8<-- "journey/code/10-depth-10.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Two overlapping triangles appear. Pink remains in front at the centre, even though the turquoise triangle is drawn afterward. Pan, zoom and rotation move the view without reversing that relationship.

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
