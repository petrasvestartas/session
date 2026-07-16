# 19 Link the kernel — your first real Mesh

The hand-rolled cube proved the pipeline; now a **real `session_rust::Mesh`**. One method does it:
**`mesh.gpu_mesh(&device)`**. Kernel mesh is f64 (half-edge); GPU wants f32 — `gpu_mesh` flattens
**once**, uploads, **caches**, returning `GpuMesh { vbo, ibo, index_count }`, drawn like lesson 18's
cube. No `cast_slice`, offsets, or `as f32` loop in the viewer — the kernel owns it all
(`RenderVertex::layout()`), the f64-compute/f32-draw boundary made concrete.

## Why

```
session_rust::Mesh  (f64, half-edge — the modeling truth)
      │  mesh.gpu_mesh(&device)     flatten once (to_render → f32 RenderVertex), upload, CACHE
      ▼
GpuMesh { vbo, ibo, index_count }   f32, GPU-resident; reused every frame, re-built only on edit
      │  pipeline buffers: RenderVertex::layout()   (pos @0, normal @1, color @2 — stride 40)
      ▼
set_vertex_buffer + set_index_buffer(Uint32) + draw_indexed(0..index_count)
```

The cube we typed by hand is exactly what a mesh *is*: a vertex buffer + an index buffer. Only three
differ from lesson 18: buffers come from the kernel (f64 geometry, cached on `Mesh`), vertex format is
`RenderVertex` (position **+ normal +** RGBA, stride 40), indices are **`u32`** (`Uint32`, not
`Uint16`). `gpu_mesh` caches, so flatten+upload runs once — later frames just re-bind, the whole point
of the split.

### How `Mesh` got wgpu (kernel side, done once)

This lesson doesn't add wgpu to the kernel — that plumbing already lives in `session_rust`; the viewer
calls just one method. Three additions made it possible:

- **`wgpu` is a dependency of `session_rust`** — on wasm via the `fragile-send-sync-non-atomic-wasm`
  feature, so a `Mesh` holding a `wgpu::Buffer` still counts as `Send` (kernel iterates `Vec<Mesh>` in
  parallel).
- **`Mesh` carries a `#[serde(skip)]` cache,** `gpu_cache: GpuCache` (`Option<GpuMesh>`) — skips
  JSON/proto; `Clone` resets it to `None`, rebuilt on demand when the mesh is edited or copied.
- **`gpu_mesh(&device)` fills that cache:** flattens the f64 half-edge mesh to f32 once
  (`to_render()` → `RenderVertex`), uploads `vbo`/`ibo`, stores the `GpuMesh`.

All wgpu types and the f64→f32 cast stay *inside* the kernel; from the viewer it's just
`mesh.gpu_mesh(&device)` plus a bind and a draw. f64 to compute, f32 to draw.

## Files we touch

```
src/engine/pipelines/build.rs   # pipeline vertex layout → RenderVertex::layout(); drop local Vertex
src/shaders/triangle.wgsl        # read RenderVertex's color (location 2, vec4)
src/engine/gpu.rs                # hold a Mesh; gpu_mesh() + draw_indexed in clear()
```

(`Cargo.toml` already lists `session_rust`, added in the camera chapters — nothing to add.)

## Step 1 — the pipeline speaks `RenderVertex`: `src/engine/pipelines/build.rs`

The hand-rolled `Vertex` struct (and its `bytemuck` import) are gone — `RenderVertex` is the vertex
format now, declaring its own `layout()`. Delete the struct/impl, import `RenderVertex`, point the
pipeline at it:

```rust
use session_rust::RenderVertex;     // replaces the local `Vertex` struct + bytemuck import

pub fn build_triangle_pipeline(/* …unchanged… */) -> wgpu::RenderPipeline {
    // …shader, layout, etc. unchanged…
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                // ← was Vertex::layout(); pos@0, normal@1, color@2
                buffers: &[RenderVertex::layout()],
                compilation_options: Default::default(),
            },
    // …
}
```

## Step 2 — the shader reads `RenderVertex`'s color: `src/shaders/triangle.wgsl`

`RenderVertex` puts color at **location 2** (`vec4`, RGBA), normal at location 1. No shading yet
(lesson 21), so the vertex shader just passes color through — input must match the new layout. Two
lines change:

```wgsl
struct VsIn {
    @location(0) position: vec3<f32>,
    @location(2) color: vec4<f32>,      // ← was @location(1) vec3; normal @1 is present but unused
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var o: VsOut;
    o.pos = mvp * vec4<f32>(in.position, 1.0);
    o.color = in.color.rgb;             // ← RGBA → RGB; VsOut.color / fs_main stay as-is
    return o;
}
```

(A shader may consume a *subset* of a layout's attributes — skipping normal at location 1 is fine,
picked up when lighting arrives.)

## Step 3 — hold a `Mesh`: `src/engine/gpu.rs`

Swap the cube's buffers for a `Mesh`: update imports (drop `Vertex`, add `Mesh`/`Color`), replace the
three buffer fields with one `mesh`, build a box in `new()`:

```rust
use session_rust::{Color, Mesh, Xform};   // was: session_rust::Xform + build::Vertex
```

```rust
pub struct Gpu {
    // …surface/device/queue/config/pipelines/mvp/time/depth unchanged…
    pub mesh: Mesh,                         // replaces vertex_buffer / index_buffer / num_indices
}
```

```rust
        // in new(), where the CUBE / vertex_buffer / index_buffer used to be:
        let mut mesh = Mesh::create_box(1000.0, 1000.0, 1000.0);   // a 1 m box, authored in mm
        mesh.set_objectcolor(Color::new(0.2, 0.5, 0.9, 1.0));      // blue — visible on the grey bg

        // …then return `mesh` in `Ok(Self { … })` instead of the buffer fields.
```

(A new `Mesh` defaults to `ColorMode::OBJECTCOLOR`; `to_render` honors the mode, so object color
shows. Per-vertex colors take over only in `POINTCOLORS` mode, after `set_pointcolors(...)`.)

## Step 4 — draw the mesh: `src/engine/gpu.rs`

In `clear()`, ask the mesh for its `GpuMesh` and draw it — first call flattens/uploads/caches, later
frames return the same buffers. Indices are `u32` → `Uint32`:

```rust
            let gm = self.mesh.gpu_mesh(&self.device);   // build+upload once, cached thereafter
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            pass.set_vertex_buffer(0, gm.vbo.slice(..));
            pass.set_index_buffer(gm.ibo.slice(..), wgpu::IndexFormat::Uint32);   // ← u32 now
            pass.draw_indexed(0..gm.index_count, 0, 0..1);
```

(`self.mesh.gpu_mesh(&self.device)` borrows `self.mesh` mutably, `self.device` shared — different
fields, compiles inside `&mut self`. `&GpuMesh` lives as long as `gm` is used.)

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

A blue box — but a genuine `session_rust::Mesh`, the type the kernel builds NURBS, booleans and
file-loaded geometry into. Orbit, fit (`F`), named views work. Swap `Mesh::create_box(…)` for
`Mesh::cube(1000.0)`, a sphere, or any constructor — it draws through `gpu_mesh`. (Flat solid for now;
normals/lighting arrive lesson 21, already in the buffer at location 1.)

## Recap

```
Ch 18: a cube from a hand-typed vertex buffer + index buffer.
Ch 19: the buffers come from the kernel — mesh.gpu_mesh(&device) flattens the f64 Mesh to f32 ONCE,
       caches GpuMesh { vbo, ibo, index_count }, and we draw_indexed it (Uint32). The pipeline uses
       RenderVertex::layout(); no cast_slice / offsets / as-f32 in the viewer. f64 compute,
       f32 draw.
```

Edited: `pipelines/build.rs` (`RenderVertex::layout()`, drop local `Vertex`), `shaders/triangle.wgsl`
(color @location 2, vec4), `engine/gpu.rs` (`mesh: Mesh` field; `create_box` + `set_objectcolor`;
`gpu_mesh` + `draw_indexed` in `clear`).

## Next

`20-grid.md` — a procedural ground grid (a second pipeline, its own shader + one draw) so the box has
a floor. Then mesh shading (lesson 21) lights the box using the normals already in `RenderVertex`.
