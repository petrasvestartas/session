# 29 Instancing — one mesh, a hundred copies, one draw call

Lesson 28's gauge read **8 draws for 3 objects** — one `draw_indexed` per mesh. Instancing draws the
**same** geometry many times from **one** call: buffers upload once; a **storage buffer** of per-copy
rows (transform + color) makes copy #7 differ from copy #42. Swap the three-mesh demo for a **10×10
field of dodecahedra** and perf drops to **3 draws for 100 objects**.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="one mesh plus N instance rows equals one draw" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <text x="70" y="20" fill="#888" text-anchor="middle">1 mesh</text>
  <polygon points="70,42 96.6,61.3 86.5,92.7 53.5,92.7 43.4,61.3" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="70" y="112" fill="#d7dae0" text-anchor="middle">dodecahedron, uploaded once</text>
  <text x="150" y="70" fill="#6fb3ff" font-size="16">▶</text>
  <text x="330" y="20" fill="#888" text-anchor="middle">instances[] — one row per copy</text>
  <rect x="240" y="34" width="190" height="88" fill="none" stroke="#3a3a3a"/>
  <line x1="240" y1="56" x2="430" y2="56" stroke="#3a3a3a"/>
  <line x1="240" y1="78" x2="430" y2="78" stroke="#3a3a3a"/>
  <line x1="240" y1="100" x2="430" y2="100" stroke="#3a3a3a"/>
  <text x="250" y="50" fill="#d7dae0">model  color  flags</text>
  <text x="250" y="72" fill="#d7dae0">model  color  flags</text>
  <text x="250" y="94" fill="#d7dae0">model  color  flags</text>
  <text x="250" y="116" fill="#555">… 100</text>
  <text x="452" y="70" fill="#6fb3ff" font-size="16">▶</text>
  <text x="470" y="132" fill="#666" font-size="10">1 draw_indexed(.., 0..100)</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2">
    <rect x="560" y="40" width="16" height="16"/><rect x="582" y="40" width="16" height="16"/><rect x="604" y="40" width="16" height="16"/><rect x="626" y="40" width="16" height="16"/>
    <rect x="560" y="62" width="16" height="16"/><rect x="582" y="62" width="16" height="16"/><rect x="604" y="62" width="16" height="16"/><rect x="626" y="62" width="16" height="16"/>
    <rect x="560" y="84" width="16" height="16"/><rect x="582" y="84" width="16" height="16"/><rect x="604" y="84" width="16" height="16"/><rect x="626" y="84" width="16" height="16"/>
  </g>
  <text x="602" y="118" fill="#d7dae0" text-anchor="middle">100 copies, one call</text>
</svg>

## Why

A draw call is a CPU→driver round-trip — on the single wasm thread that's the bottleneck long before the
GPU breaks a sweat. 100 objects as 100 calls wastes the budget re-binding and re-issuing geometry the
GPU already has. `draw_indexed(0..n, 0, 0..100)` issues the pipeline **once**; the GPU replays the mesh
100 times, handing the vertex shader a running `@builtin(instance_index)` 0..99.

**Where the per-copy data lives — the locked decision (roadmap Phase 4).** wgpu offers two ways to feed
per-instance data: a second vertex buffer with `step_mode: Instance`, or a **storage buffer** indexed by
`instance_index`. We take the storage route and skip the vertex-buffer one. Lesson 27 unlocked storage
buffers for exactly this, and the table is the path everything later builds on: frustum culling flips a
flag in a row (37), the GPU cull pass writes an indirect draw over the same buffer (76), selection is
one bit in `flags` (45). A per-instance vertex buffer would dead-end all of that.

```
Ch 28:  background(1) + grid(1) + meshes(3) + edges(3)  =  8 draws  /  3 objects
Ch 29:  background(1) + grid(1) + dodecahedra(1)        =  3 draws  / 100 objects
```

(Mesh **edges** step aside until lesson 31 — per-copy they'd mean 100 line draws, undoing the win. In 31
they return as instanced cylinders, one call of their own.)

## Files we touch

```
src/engine/gpu.rs              # Instance row + storage buffer + bind group; instanced draw
src/shaders/triangle.wgsl      # read instances[instance_index]; apply model + color
src/engine/pipelines/build.rs  # triangle pipeline layout gains group 2 (the instance table)
src/engine/pipelines/mod.rs    # thread the instance layout through Pipelines::new
```

## Step 1 — the instance row: `src/engine/gpu.rs`

One row per copy: a model matrix (column-major, from `Xform::to_f32()`), a color, and a `flags` word
reserved for later (selection bit in 45, culled bit in 37). Storage-buffer array elements align to
16-byte boundaries, so the row pads to **96 bytes**, matching the WGSL struct's stride. Add it at the
very bottom of `gpu.rs`, below `impl Gpu`:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    model: [f32; 16],   // 64 B — column-major, from Xform::to_f32()
    color: [f32; 4],    // 16 B
    flags: u32,         //  4 B — reserved (selection ch 45 / cull ch 37)
    _pad: [u32; 3],     // 12 B — pad the row to 96 B (storage array stride)
}
```

## Step 2 — read the row in the shader: `src/shaders/triangle.wgsl`

Declare the table at **group 2** and pull `instances[instance_index]` in the vertex stage. The model
matrix takes the local vertex to world; the row's color replaces the per-vertex color; the normal is
rotated by the model (`w = 0` drops translation — for the flat dodecahedra `in.normal` is zero anyway,
so the fragment shader keeps deriving flat normals from screen-space derivatives).

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> time: f32;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vs_main(in: VsIn, @builtin(instance_index) ii: u32) -> VsOut {
    let inst = instances[ii];
    let world = inst.model * vec4<f32>(in.position, 1.0);

    var o: VsOut;
    o.pos = mvp * world;
    o.color = inst.color.rgb;
    o.world_pos = world.xyz;
    o.normal = (inst.model * vec4<f32>(in.normal, 0.0)).xyz;  // rotate normal, drop translation
    return o;
}
```

`fs_main` is unchanged — it already lights `in.color` with the flat/smooth normal from lesson 22.

## Step 3 — the pipeline learns about group 2: `pipelines/build.rs` + `pipelines/mod.rs`

The triangle pipeline layout needs a third bind-group layout so group 2 is legal. Add an
`instance_layout` parameter and append it to `bind_group_layouts`:

```rust
pub fn build_triangle_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
    time_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,   // ← new
) -> wgpu::RenderPipeline {
    // …
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("triangle.layout"),
        // ← +group 2
        bind_group_layouts: &[Some(aspect_layout), Some(time_layout), Some(instance_layout)],
        immediate_size: 0,
    });
    // …unchanged…
}
```

Thread it through `Pipelines::new` (only the triangle pipeline needs it):

```rust
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,
        time_layout: &wgpu::BindGroupLayout,
        instance_layout: &wgpu::BindGroupLayout,   // ← new
    ) -> Self {
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout,
                                              time_layout, instance_layout),
            grid: build_grid_pipeline(device, color_format, aspect_layout),
            edges: build_edges_pipeline(device, color_format, aspect_layout),
            background: build_background_pipeline(device, color_format),
        }
    }
```

> Convention note: Phase 4 reserves **group 2 = material**, **group 3 = per-object**. No material group
> exists yet, so the instance table parks at group 2 and moves to 3 once materials land — worth the
> later one-line renumber to keep the layout gap-free now.

## Step 4 — build the field + the storage buffer: `src/engine/gpu.rs`

Replace the three demo meshes with **one** source dodecahedron (uploaded once by `gpu_mesh`) and 100
instance rows: a centered 10×10 grid of translations, tinted by position so the win is obvious.

In `new()`, delete the lesson-22 mesh block — from `let mut mesh = Mesh::create_box(1000.0, 1000.0,
1000.0);` down through `let meshes = vec![mesh, flat, smooth];` — **and** the whole lesson-23
`edge_buffers` block right after it (`let mut edge_buffers: Vec<(wgpu::Buffer, u32)> = Vec::new();`
through the loop's closing `}`). Put this in their place:

```rust
// ONE source mesh; the instance rows place + tint 100 copies of it.
let mut mesh = Mesh::create_dodecahedron(300.0);
mesh.set_objectcolor(Color::white());          // instance color does the tinting

let n = 10i32;
let step = 900.0;                               // mm between centers
let origin = -step * (n as f64 - 1.0) * 0.5;    // center the grid on 0
let mut instances: Vec<Instance> = Vec::with_capacity((n * n) as usize);
for iy in 0..n {
    for ix in 0..n {
        let x = origin + ix as f64 * step;
        let y = origin + iy as f64 * step;
        let r = ix as f32 / (n as f32 - 1.0);
        let g = iy as f32 / (n as f32 - 1.0);
        instances.push(Instance {
            model: Xform::translation(x, y, 0.0).to_f32(),
            color: [r, g, 0.7, 1.0],
            flags: 0,
            _pad: [0; 3],
        });
    }
}
```

Upload the rows once as a **storage** buffer, and build its bind group (VERTEX-visible, read-only):

```rust
let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("instances.buffer"),
    contents: bytemuck::cast_slice(&instances),
    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
});

let instance_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    label: Some("instances.layout"),
    entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }],
});

let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    label: Some("instances.bind_group"),
    layout: &instance_layout,
    entries: &[wgpu::BindGroupEntry { binding: 0, resource: instance_buffer.as_entire_binding() }],
});
```

Then three wiring edits:

**4a.** Find `let pipelines = Pipelines::new(&device, config.format, &mvp_layout, &time_layout);`
and replace with the five-arg call — `instance_layout` must exist first, so if the line currently
sits above the block you just wrote, move it below:

```rust
let pipelines = Pipelines::new(&device, config.format, &mvp_layout, &time_layout, &instance_layout);
```

**4b.** In `pub struct Gpu { … }`, replace `pub meshes: Vec<Mesh>,` with:

```rust
    pub mesh: Mesh,
    instances: Vec<Instance>,       // non-pub — Instance is a private type
    pub instance_bind_group: wgpu::BindGroup,
```

and delete `pub edge_buffers: Vec<(wgpu::Buffer, u32)>,`.

**4c.** In the `Ok(Self { … })` initializer at the end of `new()`, replace `meshes,` with
`mesh, instances, instance_bind_group,` and delete `edge_buffers,`.

## Step 5 — draw the field in one call: `src/engine/gpu.rs`

The mesh loop collapses to a single instanced draw. In `clear()`, find the block from
`pass.set_pipeline(&self.pipelines.triangle);` through the closing `}` of the lesson-22
`for mesh in &mut self.meshes { … }` loop and replace it with — bind the table at group 2, set the
one source mesh's buffers, pass the instance range `0..100`:

```rust
        // Meshes — ONE draw, N instances (the storage table supplies each copy's model + color)
        pass.set_pipeline(&self.pipelines.triangle);
        pass.set_bind_group(0, &self.mvp_bind_group, &[]);
        pass.set_bind_group(1, &self.time_bind_group, &[]);
        pass.set_bind_group(2, &self.instance_bind_group, &[]);

        let gm = self.mesh.gpu_mesh(&self.device);
        pass.set_vertex_buffer(0, gm.vbo.slice(..));
        pass.set_index_buffer(gm.ibo.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..gm.index_count, 0, 0..self.instances.len() as u32);  // 100 copies
        draws += 1;
```

Also delete the lesson-23 edges block below it — `pass.set_pipeline(&self.pipelines.edges);` through
the `for (vbo, count) in &self.edge_buffers { … }` loop (edges return in 31). Then report the
instance count as the object count so the counter tells the real story:

```rust
        let objects = self.instances.len() as u32;   // was self.meshes.len()
        self.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects);
```

## Step 6 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

A 10×10 grid of dodecahedra, tinted red→green, floating over the grid. Open the console (F12) — the
perf line reads about:

```
perf: 60.0 fps | 16.67 ms | 3 draws | 100 objects
```

**3 draws for 100 objects.** Bump `n` to 30 (900 copies): still 3 draws, fps barely moves — the GPU
never re-uploaded a vertex, the CPU issued one call. That gap between *objects* and *draws* is the whole
point of the storage table; lesson 30 widens it to many different meshes.

## Recap

```
Ch 28: added the gauge — 8 draws / 3 objects, one draw_indexed per mesh.
Ch 29: draw the SAME mesh many times from ONE call. Per-copy data (model matrix + color + reserved
       flags) lives in a STORAGE buffer indexed by @builtin(instance_index) — the GPU-driven path
       (skip step_mode Instance; culling/indirect/selection build on this table later). A 10×10
       dodecahedron field draws in draw_indexed(.., 0..100): 3 draws / 100 objects. Edges parked
       until 31.
```

Edited: `engine/gpu.rs` (`Instance` row + storage buffer + group-2 bind group + instanced
`draw_indexed`, `meshes`→`mesh`, edges removed), `shaders/triangle.wgsl` (`instances[instance_index]`
→ model + color), `pipelines/build.rs` + `pipelines/mod.rs` (triangle layout gains group 2).

## Next

`30-batching.md` — the GPU arena. Instancing repeats **one** mesh; batching packs **many different**
meshes into one vertex+index buffer with a per-object row each, so a scene of distinct objects still
draws in a handful of calls — the scaling successor to `gpu_mesh`-per-`Mesh`.
