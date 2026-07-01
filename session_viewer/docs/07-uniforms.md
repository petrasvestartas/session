# 07 Uniforms & bind groups

Send a value that **changes every frame** from Rust into the shader — and watch the
triangle pulse.

You have already met two ways to get data onto the GPU: the `aspect` value (chapter 05,
a uniform in a bind group) and the per-vertex `position`/`color` (chapter 06, a vertex
buffer). This chapter makes the uniform path first-class and shows the one thing only a
uniform can do — a value that is the **same for every vertex/pixel but different every
frame**. We add a `time` uniform, advance it each frame from Rust, and use it in the
fragment shader to fade the triangle's colour up and down. The exact same machinery
carries the camera matrix in chapter 08, so this is the pattern to get comfortable with.


## Uniform vs vertex buffer vs bind group (read this first)

- **Vertex buffer** (ch.06) — one struct **per vertex**; the GPU walks the buffer and
  hands one to each vertex-shader invocation. Use it for geometry that differs corner to
  corner.
- **Uniform** — **one value shared by the whole draw** (every vertex, every pixel). Use it
  for "globals": the aspect ratio, the time, a tint colour, the camera matrix.
- **Bind group** — the wgpu object that attaches a set of uniforms (and later textures) to
  the shader at a `@group(N)` slot. The pipeline declares the *layouts* it expects up
  front; at draw time you `set_bind_group(N, …)` with a matching group.

You already have `@group(0)` = `aspect`. This chapter adds `@group(1)` = `time`. That
second group is the *"& bind groups"* in the title: a shader can read several groups, each
independent, and you bind them one at a time. Keeping `time` in its own group (instead of
adding it next to `aspect`) is the simplest change and gives you the multi-group pattern
the camera needs next.

> **Why not just a vertex attribute or a constant?** A vertex attribute can't change after
> upload without re-uploading the whole buffer, and a WGSL constant can't change at all.
> A uniform is a tiny buffer you overwrite cheaply (`queue.write_buffer`) every frame —
> perfect for per-frame globals.


## Files we touch

```
session_viewer/src/
├── shaders/triangle.wgsl        # EDIT — add @group(1) time; pulse colour in fs_main
└── engine/
    ├── pipelines/build.rs        # EDIT — pipeline now expects a 2nd bind-group layout
    ├── pipelines/mod.rs          # EDIT — thread the time layout through
    └── gpu.rs                    # EDIT — time buffer + bind group; tick & upload each frame
```

No new files. (In the archive all the per-frame globals — view/projection matrices, light
directions, screen size, time — live together in one `CameraUniform` struct in a single
bind group. We use a separate one-field group here to keep the moving parts visible.)


## Step 1 — add the time uniform to the shader: `shaders/triangle.wgsl`

At the top, next to the existing `aspect` line, declare a second uniform in **group 1**:

```wgsl
@group(0) @binding(0) var<uniform> aspect: f32;   // = width / height   (chapter 05)
@group(1) @binding(0) var<uniform> time: f32;     // seconds since start (this chapter)
```

`@group` and `@binding` are independent address spaces, so `time` is `@group(1)
@binding(0)`, **not** `binding(1)` — it's the first (only) entry of a *different* group.

Then use it in the fragment shader — fade the colour with a sine of time:

```wgsl
@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let pulse = 0.5 + 0.5 * sin(time * 2.0);   // 0..1, smooth ~3 s cycle
    return vec4<f32>(in.color * pulse, 1.0);
}
```

The vertex shader and `VsIn`/`VsOut` are unchanged — `aspect` still corrects the shape in
`vs_main`; `time` only touches the fragment stage.


## Step 2 — the pipeline now expects two bind-group layouts: `engine/pipelines/build.rs`

`build_triangle_pipeline` currently takes one layout (`aspect_layout`). Add a second
parameter and list **both** layouts, in group order (`0`, then `1`):

```rust
pub fn build_triangle_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
    time_layout: &wgpu::BindGroupLayout,        // <- ADD
) -> wgpu::RenderPipeline {
```

```rust
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("triangle.layout"),
        bind_group_layouts: &[Some(aspect_layout), Some(time_layout)],  // was &[Some(aspect_layout)]
        immediate_size: 0,
    });
```

The order here **is** the `@group` index: slot 0 = `aspect_layout` = `@group(0)`, slot 1 =
`time_layout` = `@group(1)`. Swap them and wgpu will reject the pipeline (stage/visibility
mismatch) or, worse, bind the wrong buffer.


## Step 3 — thread the layout through: `engine/pipelines/mod.rs`

```rust
impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,
        time_layout: &wgpu::BindGroupLayout,        // <- ADD
    ) -> Self {
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout, time_layout),
        }
    }
}
```


## Step 4 — create, tick, upload, bind: `engine/gpu.rs`

**(a)** Add three fields to `struct Gpu` — the clock value plus its buffer and bind group:

```rust
pub struct Gpu {
    // …existing fields…
    pub aspect_buffer: wgpu::Buffer,
    pub aspect_bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub time: f32,                          // <- ADD  seconds, advanced each frame
    pub time_buffer: wgpu::Buffer,          // <- ADD
    pub time_bind_group: wgpu::BindGroup,   // <- ADD
}
```

**(b)** In `Gpu::new`, right after the `aspect_bind_group` block (and **before**
`Pipelines::new`), build the time uniform the same way — buffer starting at `0.0`, a
layout marked **`FRAGMENT`** this time (that's where `time` is read), and a bind group:

```rust
        // Time uniform — one f32 we overwrite every frame (see clear()). FRAGMENT-visible
        // because it's used in fs_main, unlike aspect which is VERTEX-visible.
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("time.buffer"),
            contents: bytemuck::bytes_of(&0.0f32),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let time_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("time.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });
        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("time.bind_group"),
            layout: &time_layout,
            entries: &[wgpu::BindGroupEntry{ binding: 0, resource: time_buffer.as_entire_binding() }],
        });
```

Pass the new layout to the pipelines, and add the three fields to the returned struct:

```rust
        let pipelines = Pipelines::new(&device, config.format, &aspect_layout, &time_layout);  // + &time_layout
        // …TRIANGLE / vertex_buffer / num_vertices unchanged…
        Ok(Self { surface, device, queue, config, pipelines,
                  aspect_buffer, aspect_bind_group, vertex_buffer, num_vertices,
                  time: 0.0, time_buffer, time_bind_group })
```

**(c)** In `clear`, advance the clock and upload it once per frame, then bind group 1 in
the render pass. Put the tick at the **top** of `clear`, before acquiring the frame:

```rust
    pub fn clear(&mut self, color: wgpu::Color) -> anyhow::Result<()> {
        // Advance the clock and push it to the GPU. ~1/60 s per frame — frame-tied for now;
        // a real delta-time clock arrives with the input/camera chapters.
        self.time += 1.0 / 60.0;
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::bytes_of(&self.time));
        // …existing get_current_texture()/encoder/begin_render_pass…
```

and in the render pass, between the aspect bind group and the vertex buffer:

```rust
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.aspect_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);   // <- ADD
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..self.num_vertices, 0..1);
```

The slot number in `set_bind_group(1, …)` must match `@group(1)` in the shader and the
slot-1 layout in Step 2. Three places, one number — keep them in sync.


## Step 5 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770  (Chrome/Edge)
```

The triangle now **pulses** — its colours fade toward black and back on a ~3-second
cycle — while keeping its shape (aspect) and its per-vertex gradient (vertex buffer). All
three data paths are now live at once: per-vertex (buffer), per-frame (time uniform), and
per-resize (aspect uniform).

Quick experiments: change `time * 2.0` to `time * 6.0` (faster), or replace the fragment
body with a colour cycle, e.g. `vec3<f32>(0.5 + 0.5*sin(time), 0.5 + 0.5*sin(time + 2.0),
0.5 + 0.5*sin(time + 4.0))`, to see the uniform drive hue instead of brightness.


## What changed vs Chapter 6 (recap)

```
Chapter 6:  per-vertex data (vertex buffer) + aspect uniform @group(0)
Chapter 7:  + a per-frame uniform `time` @group(1), overwritten with write_buffer each
            frame and read in fs_main → the triangle animates
            └── the exact path the camera matrix takes in chapter 8 (just a mat4, not an f32)
```

Edited: `triangle.wgsl` (declare + use `time`), `build.rs` + `pipelines/mod.rs` (second
layout), `gpu.rs` (buffer + bind group + per-frame tick/upload/bind). Untouched: `lib.rs`,
`state.rs`.


## Compare to the archive

`session_viewer_archive` does the same thing, consolidated:

- It does **not** keep a separate bind group per value. All per-frame globals live in one
  `CameraUniform` struct — `view`, `proj`, `view_proj`, camera position, screen size,
  light directions, and yes a time/animation field — uploaded as a single uniform buffer
  in one bind group. One `write_buffer` per frame updates the whole thing.
- That's the natural next step once you have more than one or two globals: a single
  `#[repr(C)] #[derive(Pod)]` struct (watch `std140`/16-byte alignment — pad `vec3`s to
  `vec4`) beats juggling a bind group each. We split them here only so each piece is
  visible on its own.
- Fragments-vs-vertex visibility still matters there: the layout marks each binding with
  the stages that read it, exactly like `FRAGMENT` vs `VERTEX` above.


## Next

`time` proves the per-frame uniform path end to end. Next (`08-mvp.md`) we send the value
that turns this from a clip-space toy into a 3D viewer: a **`view_proj` matrix** (built
with our own `session_rust::Xform`) in its own uniform. The vertex shader multiplies each position by it instead
of the hand-rolled `p.x / aspect`, and the triangle finally sits in a real world we can
look at from any angle. From there: perspective vs ortho, then the orbit camera.
