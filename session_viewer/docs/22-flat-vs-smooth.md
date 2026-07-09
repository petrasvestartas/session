# 22 Flat vs smooth — the normal chooses

Lesson 21 lit the box with a normal *reconstructed* from screen derivatives — every triangle one
flat facet. Right for a box; wrong for anything curved: a sphere-ish mesh renders as a disco ball.
This lesson finally uses the **per-vertex normal that has been sitting in `RenderVertex` at
location 1 since lesson 19**: the kernel computes smooth vertex normals, we store them on the mesh,
and the fragment shader picks — **smooth when the vertex normal exists, flat when it's zero**. One
mesh keeps hard edges, another goes smooth, same shader.

## Why

A vertex normal is the average of the face normals around that vertex (the kernel weights by face
area). The GPU interpolates it across each triangle, and normalizing per pixel gives a normal that
*curves* across the surface — so lighting bends smoothly instead of jumping at each facet:

```
flat   (lesson 21):  n = cross(dpdy, dpdx)      same n for a whole triangle → facets
smooth (this one):   n = normalize(in.normal)   interpolated per pixel      → curved look

who chooses?  the DATA:  to_render() writes the vertex's nx/ny/nz attributes if the mesh
carries them, and ZERO if not (render_mesh.rs's documented contract). So:
    vertex normal == 0  → fall back to the flat derivative normal
    vertex normal == 1  → use it, smooth
```

That zero-fallback is the whole design: a box we never touch stays flat automatically; a mesh we
*bake* normals onto turns smooth. (The archive did this with a per-object FLAG_SMOOTH bit — we get
the same choice data-driven; the flag version returns with instancing, lesson 29/30.)

## Files we touch

```
src/shaders/triangle.wgsl   # read the normal @location(1); smooth-or-flat select in fs_main
src/engine/gpu.rs           # meshes: Vec<Mesh>; two dodecahedra, one with baked normals
```

## Step 1 — pass the vertex normal through: `src/shaders/triangle.wgsl`

`RenderVertex` is position @0, **normal @1**, color @2 — we've been skipping @1 until now. Add it
to `VsIn`, hand it to the fragment via `VsOut` (the GPU interpolates everything in `VsOut`):

```wgsl
struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,     // ← NEW: unit when baked, zero when not
    @location(2) color: vec3<f32>,
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,     // ← NEW: interpolated across the triangle
}

@vertex
fn vs_main(in: VsIn) -> VsOut {
    var o: VsOut;
    o.pos = mvp * vec4<f32>(in.position, 1.0);
    o.color = in.color;
    o.world_pos = in.position;
    o.normal = in.normal;               // ← NEW
    return o;
}
```

## Step 2 — let the data choose: `fs_main`

Only the normal-picking block changes; the lights and the final multiply stay exactly lesson 21.
A baked normal has length 1, an unbaked one is exactly zero — so `dot(n, n) > 0.5` cleanly asks
"does this mesh carry normals?". (Careful naming the flag: `smooth` is a **reserved word** in WGSL —
naga rejects it — so we call it `has_normal`.)

```wgsl
@fragment
fn fs_main(in: VsOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // flat fallback — derivatives first, in uniform control flow (lesson 21)
    let flat_n = normalize(cross(dpdy(in.world_pos), dpdx(in.world_pos)));

    // baked vertex normal → smooth; zero (never baked) → flat ("smooth" is WGSL-reserved)
    let has_normal = dot(in.normal, in.normal) > 0.5;
    var n = select(flat_n, normalize(in.normal), has_normal);
    if !front { n = -n; }

    // …key/fill/hemi lighting and `in.color * lit` unchanged from lesson 21…
```

(Interpolating unit normals shortens them slightly mid-face — that's why we re-`normalize` per
pixel. And `select` evaluates both operands: `normalize(zero)` produces a NaN that is simply
discarded when `flat_n` is chosen — harmless.)

## Step 3 — one flat, one smooth: `src/engine/gpu.rs`

Swap the single `mesh` field for a list (this also readies the draw loop for every mesh that
follows):

```rust
    pub meshes: Vec<Mesh>,      // replaces `mesh: Mesh`
```

In `new()`, keep the blue box (never baked → stays flat), and add **two identical dodecahedra**:
the left one untouched (flat), the right one with kernel-computed normals baked in. Note the order —
**transform first, then bake** (a rotation would invalidate baked normals; keep the habit even for
a translation):

```rust
        let mut mesh = Mesh::create_box(1000.0, 1000.0, 1000.0);
        mesh.set_objectcolor(Color::new(0.2, 0.5, 0.9, 1.0));

        let mut flat = Mesh::create_dodecahedron(500.0);
        flat.transform(&Xform::translation(-1600.0, 0.0, 0.0));
        flat.set_objectcolor(Color::new(0.9, 0.5, 0.2, 1.0));

        let mut smooth = Mesh::create_dodecahedron(500.0);
        smooth.transform(&Xform::translation(1600.0, 0.0, 0.0));
        smooth.set_objectcolor(Color::new(0.9, 0.5, 0.2, 1.0));
        smooth.compute_vertex_normals();   // area-weighted vertex normals, stored on each vertex

        let meshes = vec![mesh, flat, smooth];
```

(Two kernel notes. `transform(&xf)` — Rust mirrors C++'s two overloads with
`impl Into<Option<&Xform>>`: pass `&xf` directly, or `None` to apply the mesh's *stored* `xform`.
And `compute_vertex_normals()` — the familiar name from three.js/Open3D — is `vertex_normals()`
(the area-weighted average of the face normals around each vertex) stored per vertex via
`set_normal`, plus the GPU-cache invalidation; one call instead of a hand-written loop.)

Return `meshes` in the `Ok(Self { … })` instead of `mesh`.

## Step 4 — draw them all: `src/engine/gpu.rs`

In `clear()`, the pipeline and bind groups are set once; then loop the meshes — each binds its own
cached buffers and draws:

```rust
            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            for mesh in &mut self.meshes {
                let gm = mesh.gpu_mesh(&self.device);   // cached per mesh (lesson 19)
                pass.set_vertex_buffer(0, gm.vbo.slice(..));
                pass.set_index_buffer(gm.ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..gm.index_count, 0, 0..1);
            }
```

(One draw call per mesh is fine for three meshes; collapsing many draws into few is exactly the
batching story of lessons 28–30. If `F` no longer frames everything, widen the `SCENE_*` bounds in
`lib.rs` to cover x ≈ ±2300.)

## The invalidation contract (when does the GPU copy refresh?)

`gpu_mesh()` returns a *cached* snapshot — so when does it rebuild? Two rules:

- **Geometry edits through mesh APIs** (`add_vertex`, `remove_face`, `transform`, …) call
  `invalidate_triangle_bvh()`, which also drops the GPU cache (`gpu_cache.0 = None`) — the next
  `gpu_mesh()` re-flattens automatically.
- **Direct attribute pokes** — like `set_normal` on a vertex — do *not* invalidate.
  `compute_vertex_normals()` calls `invalidate_gpu()` for you (that's why it exists); poke
  attributes by hand and you must say so yourself:

```rust
        // try it: set_normal by hand AFTER the first frame and nothing changes — until you add:
        // smooth.invalidate_gpu();   // drop the cache; next gpu_mesh() re-flattens + re-uploads
        // (compute_vertex_normals() does this internally, so re-computing Just Works)
```

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Two orange dodecahedra flank the blue box — **identical geometry, different normals**. The left one
is faceted: twelve flat pentagons, hard edges, lighting jumps at every seam (the lesson-21
derivative normal). The right one reads as a smooth rounded solid: the interpolated vertex normals
bend the light continuously across the facets. The box is untouched and still perfectly crisp —
zero normals, flat fallback. Orbit and watch the highlight roll smoothly across the right
dodecahedron while it snaps facet-to-facet on the left.

## Recap

```
Ch 21: one normal per triangle, reconstructed from derivatives — everything faceted.
Ch 22: the RenderVertex normal @1 is finally used. One call — compute_vertex_normals() — computes
       the kernel's area-weighted vertex normals, stores them per vertex, and invalidates the GPU
       cache; to_render bakes them into the buffer, and fs_main selects: unit normal → smooth
       (normalize the interpolated value), zero → flat fallback. Data-driven flat-vs-smooth, no
       flags, no second pipeline. Plus the invalidation contract: API edits auto-drop the GpuMesh
       cache; raw attribute pokes need invalidate_gpu().
```

Edited: `shaders/triangle.wgsl` (normal @1 in, interpolated @2 out, smooth-or-flat select),
`engine/gpu.rs` (`meshes: Vec<Mesh>`, two dodecahedra + bake loop, draw loop).

## Next

`23-mesh-edges.md` — the other half of the "CAD look": dark edge lines over the shaded solid. An
edge overlay pipeline built from the kernel's `mesh.edges()`, reusing the lesson-20 overlay pattern.
