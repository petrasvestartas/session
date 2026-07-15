# 08 MVP matrix

Give the triangle a real **camera**: instead of `p.x / aspect`, send one 4×4 matrix
placing it in a 3D world and looking at it. The triangle **spins in perspective**.

## What an MVP matrix is

Three matrices multiplied into one, read right-to-left (the order a corner travels):

- **Model** — object position/rotation (spun by `time`).
- **View** — camera position and look direction.
- **Projection** — 3D → flat screen, perspective (far looks smaller); takes
  width/height too, so `aspect` goes away this chapter.

```
clip = Projection × View × Model × corner   →   one matrix: "mvp"
```

The shader just multiplies each corner by this one matrix.

## Files we touch

```
src/shaders/triangle.wgsl   # group 0 becomes a mat4; multiply position by it
src/engine/gpu.rs           # build the mvp each frame; replaces aspect
```

`build.rs` / `pipelines/mod.rs` don't change — group 0 is still one vertex-stage
uniform buffer, contents just grow from 1 float to 16.


## Step 1 — shader takes a matrix: `triangle.wgsl`

Group-0 uniform becomes a 4×4 matrix; multiply each corner by it. `time` (group 1)
is unchanged.

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;   // was: aspect: f32
@group(1) @binding(0) var<uniform> time: f32;
```

```wgsl
@vertex
fn vs_main(in: VsIn) -> VsOut {
    var o: VsOut;
    o.pos   = mvp * vec4<f32>(in.position, 1.0);   // was: p.x / aspect
    o.color = in.color;
    return o;
}
```

`fs_main` is unchanged.


## Step 2 — build & upload the matrix: `gpu.rs`

**(a)** Imports:

```rust
use session_rust::{Xform, Point, Vector};
```

**(b)** Rename the two `aspect_*` fields to `mvp_*` in `struct Gpu` (same types).

**(c)** Replace the aspect setup in `Gpu::new` with the mvp uniform — buffer +
layout + bind group. Keep `use wgpu::util::DeviceExt;`. Start at identity;
`clear()` fills it each frame:

```rust
        use wgpu::util::DeviceExt;
        let mvp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("mvp.buffer"),
            contents: bytemuck::cast_slice(&Xform::identity().to_f32()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let mvp_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("mvp.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });
        let mvp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mvp.bind_group"),
            layout: &mvp_layout,
            entries: &[wgpu::BindGroupEntry{ binding: 0, resource: mvp_buffer.as_entire_binding() }],
        });
```

Pass `&mvp_layout` to `Pipelines::new`, and put `mvp_buffer` / `mvp_bind_group` in
`Ok(Self { … })`.

**(d)** `resize` no longer writes a uniform — drop the aspect line, keep the surface
reconfigure.

**(e)** In `clear`, after the `time` tick, build and upload the matrix:

```rust
        let aspect = self.config.width as f64 / self.config.height as f64;
        let projection = Xform::perspective(60f64.to_radians(), aspect, 0.1, 100.0);
        let view  = Xform::look_at_right_handed(&Point::new(0.0,0.0,2.0), &Point::new(0.0,0.0,0.0), &Vector::new(0.0,1.0,0.0));
        let model = Xform::rotation_y(self.time as f64, false);   // radians
        let mvp   = projection * view * model;
        self.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&mvp.to_f32()));
```

`Xform::perspective` is 0..1-depth and column-major, so it uploads straight into
WGSL's `mat4x4` — no transpose.

Bind it in the pass (renamed field):

```rust
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);   // was aspect
            pass.set_bind_group(1, &self.time_bind_group, &[]);
```


## Step 3 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The triangle spins in perspective, still pulsing colour. Resize stays correct via
the projection's aspect. Experiments: move `Point::new(0.0,0.0,2.0)`; change the
`60`-degree lens.


## Recap

```
Ch 7:  group 0 = aspect (1 float);  p.x / aspect
Ch 8:  group 0 = mvp (4×4), rebuilt each frame;  mvp * position → model + camera + perspective
```

Edited: `triangle.wgsl`, `gpu.rs`. Untouched: `Cargo.toml`, `build.rs`, `mod.rs`,
`lib.rs`, `state.rs`.


## Next

`09-projection.md` — split view from projection, toggle **perspective** vs
**orthographic** (`Xform::perspective`/`Xform::orthographic`); ch 10 puts the camera
on the mouse (orbit + zoom).
</content>
