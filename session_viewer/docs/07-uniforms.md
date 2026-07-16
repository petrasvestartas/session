# 07 Uniforms & bind groups

Send a value that **changes every frame** from Rust into the shader — and watch the
triangle pulse.

Two ways onto the GPU so far: `aspect` (ch.05, a uniform) and per-vertex
`position`/`color` (ch.06, a vertex buffer). This chapter makes the uniform path
first-class with what only it does — a value **same for every vertex/pixel but
different every frame**. A `time` uniform, advanced each frame from Rust, fades the
colour in `fs_main` — the exact path the camera matrix takes in chapter 08.


## Uniform vs vertex buffer vs bind group (read this first)

- **Vertex buffer** (ch.06) — one struct **per vertex**, corner to corner.
- **Uniform** — **one value shared by the whole draw**: aspect ratio, time, tint,
  camera matrix.
- **Bind group** — wgpu's object attaching uniforms (later textures) to a
  `@group(N)` slot; pipelines declare layouts up front, draws bind with
  `set_bind_group(N, …)`.

`@group(0)` is already `aspect`; this chapter adds `@group(1)` = `time` — the *"&
bind groups"* of the title: a shader reads several independent groups, bound one at
a time. A separate group (not folded into `aspect`) previews the multi-group pattern
the camera needs next.

> **Why not a vertex attribute or a constant?** An attribute needs the whole buffer
> re-uploaded to change; a WGSL constant can't change at all. A uniform is cheaply
> overwritten (`queue.write_buffer`) every frame — perfect for per-frame globals.

<svg viewBox="0 0 680 180" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="rust writes a value into a uniform buffer via the queue; a bind group attaches that buffer to group slot 1; the shader declares group 1 binding 0 and reads the same value in every invocation" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="90" y="18" fill="#888" text-anchor="middle">CPU (Rust), every frame</text>
  <rect x="14" y="26" width="160" height="34" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="94" y="47" fill="#d7dae0" text-anchor="middle">queue.write_buffer(time)</text>
  <line x1="174" y1="43" x2="230" y2="43" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah07)"/>
  <text x="340" y="18" fill="#888" text-anchor="middle">GPU memory</text>
  <rect x="234" y="26" width="130" height="34" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="299" y="47" fill="#d7dae0" text-anchor="middle">uniform buffer (4 B)</text>
  <rect x="234" y="80" width="130" height="30" fill="none" stroke="#888"/>
  <text x="299" y="99" fill="#888" text-anchor="middle">bind group</text>
  <path d="M 299,60 V 78" stroke="#888" stroke-width="1.1" marker-end="url(#ah07g)"/>
  <text x="392" y="99" fill="#666" font-size="10">= "this buffer sits in slot @group(1)"</text>
  <text x="560" y="18" fill="#888" text-anchor="middle">shader (WGSL)</text>
  <rect x="470" y="26" width="200" height="60" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <text x="480" y="46" fill="#d7dae0">@group(1) @binding(0)</text>
  <text x="480" y="62" fill="#d7dae0">var&lt;uniform&gt; time: f32;</text>
  <text x="480" y="78" fill="#666" font-size="10">every vertex + pixel reads the SAME value</text>
  <line x1="364" y1="43" x2="468" y2="43" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah07)"/>
  <text x="340" y="150" fill="#666" text-anchor="middle" font-size="10">three parts, one per frame-rate: buffer (written per frame) · bind group (built once) · layout declared in the pipeline (built once)</text>
  <defs>
    <marker id="ah07" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker>
    <marker id="ah07g" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#888"/></marker>
  </defs>
</svg>


## Files we touch

```
session_viewer/src/
├── shaders/triangle.wgsl        # EDIT — add @group(1) time; pulse colour in fs_main
└── engine/
    ├── pipelines/build.rs        # EDIT — pipeline now expects a 2nd bind-group layout
    ├── pipelines/mod.rs          # EDIT — thread the time layout through
    └── gpu.rs                    # EDIT — time buffer + bind group; tick & upload each frame
```

No new files. (In the archive, all per-frame globals — view/projection, light
directions, screen size, time — live in one `CameraUniform` struct, one bind group.
Split here so each piece is visible on its own.)


## Step 1 — add the time uniform to the shader: `shaders/triangle.wgsl`

At the top, next to `aspect`, declare a second uniform in **group 1**:

```wgsl
@group(0) @binding(0) var<uniform> aspect: f32;   // = width / height   (chapter 05)
@group(1) @binding(0) var<uniform> time: f32;     // seconds since start (this chapter)
```

`@group`/`@binding` are independent address spaces, so `time` is `@group(1)
@binding(0)`, **not** `binding(1)` — it's the first (only) entry of a *different*
group.

Use it in the fragment shader — fade the colour with a sine of time:

```wgsl
@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let pulse = 0.5 + 0.5 * sin(time * 2.0);   // 0..1, smooth ~3 s cycle
    return vec4<f32>(in.color * pulse, 1.0);
}
```

`vs_main`/`VsIn`/`VsOut` are unchanged — `aspect` still corrects shape in `vs_main`;
`time` only touches the fragment stage.


## Step 2 — the pipeline now expects two bind-group layouts: `engine/pipelines/build.rs`

`build_triangle_pipeline` currently takes one layout (`aspect_layout`). Add a second
parameter and list **both**, in group order (`0`, then `1`):

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

Order **is** `@group` index: slot 0 = `aspect_layout` = `@group(0)`, slot 1 =
`time_layout` = `@group(1)`. Swap them and wgpu rejects the pipeline or binds the
wrong buffer.


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

**(b)** In `Gpu::new`, right after `aspect_bind_group` (**before** `Pipelines::new`),
build the time uniform the same way — buffer at `0.0`, a **`FRAGMENT`**-visibility
layout this time (where `time` is read), and a bind group:

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

**(c)** In `clear`, advance the clock and upload it once per frame, then bind group 1
in the render pass. Put the tick at the **top** of `clear`, before acquiring the frame:

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

The slot number in `set_bind_group(1, …)` must match `@group(1)` in the shader and
the slot-1 layout in Step 2 — three places, one number, keep them in sync.


## Step 5 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770  (Chrome/Edge)
```

The triangle now **pulses** — colour fades toward black and back on a ~3-second
cycle — while keeping shape (aspect) and per-vertex gradient (vertex buffer). All
three data paths are live: per-vertex, per-frame (time), per-resize (aspect).

Quick experiments: `time * 2.0` → `time * 6.0` (faster), or drive hue instead of
brightness with `vec3<f32>(0.5 + 0.5*sin(time), 0.5 + 0.5*sin(time + 2.0), 0.5 +
0.5*sin(time + 4.0))`.


## What changed vs Chapter 6 (recap)

```
Chapter 6:  per-vertex data (vertex buffer) + aspect uniform @group(0)
Chapter 7:  + a per-frame uniform `time` @group(1), overwritten with write_buffer each
            frame and read in fs_main → the triangle animates
            └── the exact path the camera matrix takes in chapter 8 (just a mat4, not an f32)
```

Edited: `triangle.wgsl` (declare + use `time`), `build.rs` + `pipelines/mod.rs`
(second layout), `gpu.rs` (buffer + bind group + per-frame tick/upload/bind).
Untouched: `lib.rs`, `state.rs`.


## Compare to the archive

`session_viewer_archive` does the same thing, consolidated:

- No per-value bind group — per-frame globals (view, proj, view_proj, camera pos,
  screen size, light dirs, time) live in one `CameraUniform` struct, one bind group,
  one `write_buffer` per frame.
- Past one or two globals, a single `#[repr(C)] #[derive(Pod)]` struct (watch
  `std140` 16-byte alignment — pad `vec3`s to `vec4`) beats separate groups.
- Fragment-vs-vertex visibility still matters — each binding's layout marks the
  stages reading it, like `FRAGMENT`/`VERTEX` above.


## Next

`time` proves the per-frame uniform path end to end. Next (`08-mvp.md`) turns this
into a 3D viewer: a **`view_proj`** matrix (`session_rust::Xform`), multiplied into
each position instead of `p.x / aspect`. The triangle finally sits in a real world
we can view from any angle. Then: perspective vs ortho, orbit camera.
</content>
