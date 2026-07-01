# 19 Link the kernel — your first real Mesh

The hand-rolled cube proved the pipeline; now we draw a **real `session_rust::Mesh`**. The kernel is
already a dependency (the camera uses `Point`/`Vector`/`Quaternion`/`Xform`), so this lesson is about
one method: **`mesh.gpu_mesh(&device)`**. The kernel mesh is f64 (half-edge, the modeling
representation). The GPU wants f32. `gpu_mesh` is the bridge — it flattens the mesh to f32 **once**,
uploads it, and **caches** the result, handing back a `GpuMesh { vbo, ibo, index_count }` we draw
exactly like lesson 18's cube. No `cast_slice`, no vertex-layout offsets, no `as f32` loop in the
viewer — the kernel owns all of it (`RenderVertex::layout()`). This is the f64-compute / f32-draw
boundary from the precision rule, made concrete.

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

The cube we typed by hand is exactly what a mesh *is*: a vertex buffer + an index buffer. The only
differences from lesson 18 are that the buffers now come from the kernel (built from f64 geometry,
cached on the `Mesh`), the vertex format is the kernel's `RenderVertex` (position **+ normal +**
RGBA, stride 40), and the indices are **`u32`** (`Uint32`, not `Uint16`). Because `gpu_mesh` caches,
the flatten+upload runs once; every later frame just re-binds the same buffers — the whole point of
the split.

### How `Mesh` got wgpu (kernel side, done once)

This lesson doesn't add wgpu to the kernel — that plumbing already lives in `session_rust`, so the
viewer only ever calls one method. Three small additions made it possible:

- **`wgpu` is a dependency of `session_rust`.** On wasm it's pulled with the
  `fragile-send-sync-non-atomic-wasm` feature, so a `Mesh` that now holds a `wgpu::Buffer` still
  counts as `Send` (the kernel iterates `Vec<Mesh>` in parallel).
- **`Mesh` carries a `#[serde(skip)]` cache,** `gpu_cache: GpuCache` (an `Option<GpuMesh>`). Being
  `serde(skip)` it never touches JSON/proto, and its `Clone` resets it to `None` — so the cache is
  rebuilt on demand and silently dropped whenever the mesh is edited or copied.
- **`gpu_mesh(&device)` fills that cache:** it flattens the f64 half-edge mesh to f32 once
  (`to_render()` → `RenderVertex`), uploads the `vbo`/`ibo`, and stores the `GpuMesh`.

So all the wgpu types and the f64→f32 cast stay *inside* the kernel; from the viewer it's just
`mesh.gpu_mesh(&device)` plus a bind and a draw. f64 to compute, f32 to draw.

## Files we touch

```
src/engine/pipelines/build.rs   # pipeline vertex layout → RenderVertex::layout(); drop local Vertex
src/shaders/triangle.wgsl        # read RenderVertex's color (location 2, vec4)
src/engine/gpu.rs                # hold a Mesh; gpu_mesh() + draw_indexed in clear()
```

(`Cargo.toml` already lists `session_rust` — added back in the camera chapters. Nothing to add.)

## Step 1 — the pipeline speaks `RenderVertex`: `src/engine/pipelines/build.rs`

The hand-rolled `Vertex` struct (and its `bytemuck` import) are gone — the kernel's `RenderVertex` is
the vertex format now, and it already declares its own `layout()`. Delete the struct/impl, import
`RenderVertex`, and point the pipeline at it:

```rust
use session_rust::RenderVertex;     // replaces the local `Vertex` struct + bytemuck import

pub fn build_triangle_pipeline(/* …unchanged… */) -> wgpu::RenderPipeline {
    // …shader, layout, etc. unchanged…
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[RenderVertex::layout()],   // ← was Vertex::layout(); pos@0, normal@1, color@2
                compilation_options: Default::default(),
            },
    // …
}
```

## Step 2 — the shader reads `RenderVertex`'s color: `src/shaders/triangle.wgsl`

`RenderVertex` puts color at **location 2** (a `vec4`, RGBA), with the normal at location 1. We don't
shade yet (lesson 21), so the vertex shader just passes color through — but its input must match the
new layout. Two lines change:

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

(A shader may consume a *subset* of a layout's attributes, so skipping the normal at location 1 is
fine — we'll pick it up when we add lighting.)

## Step 3 — hold a `Mesh`: `src/engine/gpu.rs`

Swap the cube's buffers for a `Mesh`. Update the imports (drop the local `Vertex`, add `Mesh`/`Color`),
replace the three buffer fields with one `mesh`, and build a box in `new()`:

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

(A new `Mesh` defaults to `ColorMode::OBJECTCOLOR`, and `to_render` honors the mode — so the object
color shows. Per-vertex colors take over only when the mesh is in `POINTCOLORS` mode, i.e. after
`set_pointcolors(...)`.)

## Step 4 — draw the mesh: `src/engine/gpu.rs`

In `clear()`, ask the mesh for its `GpuMesh` and draw it. The first call flattens + uploads and
caches; every later frame returns the same cached buffers. Indices are `u32` → `Uint32`:

```rust
            let gm = self.mesh.gpu_mesh(&self.device);   // build+upload once, cached thereafter
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            pass.set_vertex_buffer(0, gm.vbo.slice(..));
            pass.set_index_buffer(gm.ibo.slice(..), wgpu::IndexFormat::Uint32);   // ← u32 now
            pass.draw_indexed(0..gm.index_count, 0, 0..1);
```

(`self.mesh.gpu_mesh(&self.device)` borrows `self.mesh` mutably and `self.device` shared — different
fields, so it compiles inside `&mut self`. The returned `&GpuMesh` lives as long as you use `gm`.)

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

A blue box — but this one is a genuine `session_rust::Mesh`, the same type the kernel builds NURBS,
booleans and file-loaded geometry into. Orbit, fit (`F`), named views all work. Swap
`Mesh::create_box(…)` for `Mesh::cube(1000.0)`, a sphere, or any kernel constructor and it just draws
— because everything now flows through `gpu_mesh`. (It's a flat solid for now; per-vertex normals and
lighting arrive in lesson 21, and they're already in the buffer at location 1.)

## Recap

```
Ch 18: a cube from a hand-typed vertex buffer + index buffer.
Ch 19: the buffers come from the kernel — mesh.gpu_mesh(&device) flattens the f64 Mesh to f32 ONCE,
       caches GpuMesh { vbo, ibo, index_count }, and we draw_indexed it (Uint32). The pipeline uses
       RenderVertex::layout(); no cast_slice / offsets / as-f32 in the viewer. f64 compute, f32 draw.
```

Edited: `pipelines/build.rs` (`RenderVertex::layout()`, drop local `Vertex`), `shaders/triangle.wgsl`
(color @location 2, vec4), `engine/gpu.rs` (`mesh: Mesh` field; `create_box` + `set_objectcolor`;
`gpu_mesh` + `draw_indexed` in `clear`).

## Next

`20-grid.md` — a procedural ground grid (a second pipeline: its own shader + one draw), so the box
has a floor to sit on and the scene reads as 3D space. After that, mesh shading (lesson 21) lights
the box using the normals already sitting in `RenderVertex`.
