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

<svg viewBox="0 0 680 170" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a corner travels through model to world space, view to camera space, projection to clip space; the three matrices premultiply into one mvp" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="34" width="120" height="44"/><rect x="180" y="34" width="120" height="44"/><rect x="352" y="34" width="120" height="44"/><rect x="524" y="34" width="148" height="44"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="68" y="52">object space</text><text x="68" y="66" fill="#666" font-size="9">corner as authored</text>
    <text x="240" y="52">world space</text><text x="240" y="66" fill="#666" font-size="9">placed in the scene</text>
    <text x="412" y="52">camera space</text><text x="412" y="66" fill="#666" font-size="9">seen from the eye</text>
    <text x="598" y="52">clip space</text><text x="598" y="66" fill="#666" font-size="9">−1..1, ready to raster</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.3">
    <line x1="128" y1="55" x2="178" y2="55" marker-end="url(#ah08)"/><line x1="300" y1="55" x2="350" y2="55" marker-end="url(#ah08)"/><line x1="472" y1="55" x2="522" y2="55" marker-end="url(#ah08)"/>
  </g>
  <g fill="#6fb3ff" text-anchor="middle" font-size="10">
    <text x="153" y="48">Model</text><text x="325" y="48">View</text><text x="497" y="48">Projection</text>
  </g>
  <g fill="#888" text-anchor="middle" font-size="9">
    <text x="153" y="94">spin / place</text><text x="325" y="94">where the camera is</text><text x="497" y="94">perspective + aspect</text>
  </g>
  <text x="340" y="130" fill="#666" text-anchor="middle" font-size="10">the three multiply ONCE on the CPU (f64 kernel Xform) → one mvp uniform → the shader does one multiply per corner</text>
  <text x="340" y="150" fill="#555" text-anchor="middle" font-size="10">read right-to-left: the matrix nearest the corner applies first</text>
  <defs><marker id="ah08" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

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
