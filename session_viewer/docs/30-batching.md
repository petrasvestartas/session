# 30 Batching — many *different* meshes, still one draw call

Lesson 29 replayed **one** mesh a hundred times via `@builtin(instance_index)`, each copy reading its
row from the group-2 `instances` table — the trick for *identical* geometry. A real scene is a box, a
sphere, a torus — not clones. This lesson keeps the instance table as-is and swaps the **source**:
concatenate **N distinct meshes** into one vertex+index **arena**, drawn in **one** `draw_indexed`. Perf
still reads **3 draws** — now for **5 different objects**, and would read 3 for 500.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="N different meshes concatenated into one arena equals one draw" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <text x="90" y="20" fill="#888" text-anchor="middle">5 different meshes</text>
  <rect x="20" y="42" width="24" height="24" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <polygon points="76,40 92,52 86,70 66,70 60,52" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <circle cx="132" cy="56" r="14" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <ellipse cx="30" cy="94" rx="16" ry="9" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <ellipse cx="90" cy="94" rx="19" ry="8" fill="none" stroke="#6fb3ff" stroke-width="1.3"/>
  <ellipse cx="90" cy="94" rx="8" ry="4" fill="none" stroke="#6fb3ff" stroke-width="1"/>
  <text x="90" y="118" fill="#d7dae0" text-anchor="middle">box · dodeca · sphere · cyl · torus</text>
  <text x="190" y="72" fill="#6fb3ff" font-size="16">▶</text>
  <text x="330" y="20" fill="#888" text-anchor="middle">arena — verts + per-vertex id</text>
  <g stroke="#0d0f12">
    <rect x="240" y="40" width="38" height="60" fill="#2b4a63"/>
    <rect x="278" y="40" width="38" height="60" fill="#3a3a3a"/>
    <rect x="316" y="40" width="38" height="60" fill="#2b4a63"/>
    <rect x="354" y="40" width="38" height="60" fill="#3a3a3a"/>
    <rect x="392" y="40" width="38" height="60" fill="#2b4a63"/>
  </g>
  <g fill="#666" text-anchor="middle" font-size="10">
    <text x="259" y="112">id0</text>
    <text x="297" y="112">id1</text>
    <text x="335" y="112">id2</text>
    <text x="373" y="112">id3</text>
    <text x="411" y="112">id4</text>
  </g>
  <text x="452" y="72" fill="#6fb3ff" font-size="16">▶</text>
  <text x="470" y="132" fill="#666" font-size="10">1 draw_indexed(0..idx, 0, 0..1)</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.5">
    <rect x="565" y="55" width="28" height="28"/>
    <circle cx="628" cy="69" r="15"/>
  </g>
  <text x="602" y="118" fill="#d7dae0" text-anchor="middle">whole scene, one call</text>
</svg>

## Why

Instancing (29) and batching (30) collapse draws in opposite ways:

```
Instancing:  1 mesh's vertices in GPU, replayed N times   → cheap memory, geometry must be identical
Batching:    N meshes' vertices concatenated in one arena → arbitrary geometry,
             vertices stored once each
```

A draw call is a CPU→driver round-trip (lesson 28), and switching the bound vertex/index buffer between
objects is itself part of that cost. Batching binds the arena **once** and issues **one** `draw_indexed`
over all of it, so wholly different meshes cost the same three calls as the empty grid — the **scaling
successor to `gpu_mesh`-per-`Mesh`**: the old per-object loop (pre-29) rebound buffers and drew per
mesh; the arena removes both.

**The one new idea — a per-vertex row id.** In 29 the vertex→`instances[]` link was
`@builtin(instance_index)`, supplied by the draw's instance range. A batched draw has one instance
(`0..1`), so that builtin is always 0 — useless here. Instead every arena vertex carries its row id in a
**second vertex buffer** of `u32` at `@location(3)`: vertex 40 (sphere) reads id `2`, vertex 900 (torus)
reads id `4`, and the shader looks up `instances[id]`. That tag is the whole mechanism letting one draw
cover meshes with different geometry.

```
Ch 29:  background(1) + grid(1) + dodecahedra(1)   =  3 draws  / 100 identical copies
Ch 30:  background(1) + grid(1) + arena(1)         =  3 draws  /   5 distinct meshes
```

(Edges still sit out — they rejoin in 31 as instanced cylinders, one draw of their own.)

## Files we touch

```
src/engine/gpu.rs              # build the arena (concat verts + ids + indices); one draw over it
src/shaders/triangle.wgsl      # VsIn gains @location(3) inst_id; read instances[inst_id]
src/engine/pipelines/build.rs  # triangle vertex state gains a 2nd buffer (the per-vertex u32 ids)
```

The group-2 instance table, its layout, and its bind group are **unchanged from 29** — batching reuses
them verbatim.

## Step 1 — tag each vertex in the shader: `src/shaders/triangle.wgsl`

**1a. Find `struct VsIn`** (it currently ends at `@location(2)`) and add a fourth field:

```wgsl
struct VsIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>, // unit when baked, zero when not
    @location(2) color: vec3<f32>,
    // ← ADD THIS LINE — which instances[] row this vertex belongs to
    @location(3) inst_id: u32,
}
```

**1b. Find `fn vs_main`** — it currently reads `fn vs_main(in: VsIn, @builtin(instance_index) ii: u32)`
with `let inst = instances[ii];` on the next non-blank line. Change **only those two lines**; leave the
rest of the body (`world`, `o.pos`, `o.color`, `o.world_pos`, `o.normal`, `return o`) unchanged:

```wgsl
@vertex
fn vs_main(in: VsIn) -> VsOut {          // ← was: (in: VsIn, @builtin(instance_index) ii: u32)
    let inst = instances[in.inst_id];    // ← was: let inst = instances[ii];
    let world = inst.model * vec4<f32>(in.position, 1.0);
    // …everything below here is unchanged…
```

`Instance`, the `@group(2)` binding, `VsOut`, and `fs_main` stay untouched. Faceted meshes arrive with
zero normals (flat shading via screen-space derivatives, lesson 22); meshes with baked vertex normals
light smoothly instead — proof the arena preserves each mesh's own attributes.

## Step 2 — the pipeline gains a second vertex buffer: `src/engine/pipelines/build.rs`

**2a. At the top of the file, right after `pub const MSAA_SAMPLES: u32 = 4;`, add the id layout** — a
free function + attribute array, one `Uint32` at `@location(3)`, stride 4, `step_mode: Vertex`:

```rust
const INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 3,
    format: wgpu::VertexFormat::Uint32,
}];

fn instance_id_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 4,                              // one u32 per vertex
        step_mode: wgpu::VertexStepMode::Vertex,      // advances per-vertex, like position
        attributes: &INSTANCE_ID_ATTRIBS,
    }
}
```

**2b. Inside `build_triangle_pipeline` only, find the vertex `buffers:` line** — it currently reads
`buffers: &[RenderVertex::layout()],` — and append the id layout as slot 1:

```rust
                // ← was: &[RenderVertex::layout()]
                buffers: &[RenderVertex::layout(), instance_id_layout()],
```

Leave `build_grid_pipeline`, `build_edges_pipeline`, and `build_background_pipeline` alone — only the
triangle pipeline reads the ids.

## Step 3 — build the arena: `src/engine/gpu.rs`

**3a. In `Gpu::new`, replace the single mesh and the whole 10×10 loop.** Find the block starting at
`let mut mesh = Mesh::create_dodecahedron(300.0);` and running through the closing `}` of the nested
`for iy … for ix …` loop — down to but NOT including `let instance_buffer = …`:

```rust
// ─────────── DELETE from here ───────────
let mut mesh = Mesh::create_dodecahedron(300.0);
mesh.set_objectcolor(Color::white());

let n = 10i32;
let step = 900.0;
let origin = -step * (n as f64 - 1.0) * 0.5;
let mut instances: Vec<Instance> = Vec::with_capacity((n*n) as usize);
for iy in 0..n {
    for ix in 0..n{
        // …
    }
}
// ─────────── DELETE to here ───────────
```

Put this in its place — it still ends by producing a `Vec<Instance>` called `instances`, so the
`instance_buffer` block right below keeps working unchanged.

Only `create_box` and `create_dodecahedron` are `Mesh` factories; `sphere`/`cylinder`/`torus` are `BRep`
factories — call `.mesh()` on those to get a `Mesh`. Add `BRep` to the `use session_rust::{…}` import at
the top of the file (and drop `Color`, since the arena no longer calls `Color::white()`):

```rust
use session_rust::{Mesh, Xform, RenderVertex, BRep};   // ← was: {Color, Mesh, Xform, RenderVertex}
```

```rust
// A scene of DIFFERENT meshes. Each gets one instance row (model + color);
// all share one arena + one draw.
let objects: Vec<(Mesh, Xform, [f32; 4])> = vec![
    (Mesh::create_box(600.0, 600.0, 600.0),
     Xform::translation(-2400.0, 0.0, 0.0), [0.90, 0.30, 0.30, 1.0]),
    (Mesh::create_dodecahedron(400.0),
     Xform::translation(-1200.0, 0.0, 0.0), [0.90, 0.70, 0.20, 1.0]),
    (BRep::create_sphere(380.0).mesh(),
     Xform::translation(    0.0, 0.0, 0.0), [0.30, 0.80, 0.40, 1.0]),
    (BRep::create_cylinder(320.0, 800.0).mesh(),
     Xform::translation( 1200.0, 0.0, 0.0), [0.30, 0.60, 0.90, 1.0]),
    (BRep::create_torus(360.0, 140.0).mesh(),
     Xform::translation( 2400.0, 0.0, 0.0), [0.70, 0.40, 0.90, 1.0]),
];

let mut verts: Vec<RenderVertex> = Vec::new();   // slot 0 — every mesh's vertices, concatenated
let mut vids:  Vec<u32>          = Vec::new();    // slot 1 — one row id per vertex (@location 3)
let mut idx:   Vec<u32>          = Vec::new();    // the shared index buffer
let mut instances: Vec<Instance> = Vec::with_capacity(objects.len());

for (ri, (mesh, model, color)) in objects.into_iter().enumerate() {
    instances.push(Instance { model: model.to_f32(), color, flags: 0, _pad: [0; 3] });
    let base = verts.len() as u32;               // where this mesh's vertices begin in the arena
    // f64 → f32 flatten (no gpu_mesh cache needed here)
    let rm = mesh.to_render();
    for v in &rm.vertices { verts.push(*v); vids.push(ri as u32); }
    // bake base_vertex into the index — one draw can't offset per-mesh
    for &i in &rm.indices { idx.push(base + i); }
}
let arena_index_count = idx.len() as u32;
```

Two things let a *single* draw span many meshes: the row id pushed once per vertex, and `base + i`
folding each mesh's local indices into arena-global ones (a lone `draw_indexed` has no per-mesh
`base_vertex` to lean on, so the offset is baked in).

The `instance_buffer` / `instance_layout` / `instance_bind_group` block that follows is **left exactly
as it is** — it already reads `&instances`.

**3b. Upload the three arena buffers.** Find the line `let instance_bind_group = device.create_bind_group(…)`
and, in the blank line right after its closing `});` (before `// Pipelines`), insert:

```rust
let arena_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("arena.vbo"), contents: bytemuck::cast_slice(&verts),
    usage: wgpu::BufferUsages::VERTEX,
});
let arena_vids = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("arena.vids"), contents: bytemuck::cast_slice(&vids),
    usage: wgpu::BufferUsages::VERTEX,
});
let arena_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("arena.ibo"), contents: bytemuck::cast_slice(&idx),
    usage: wgpu::BufferUsages::INDEX,
});
```

(`use wgpu::util::DeviceExt;` is already imported higher up in `new`, so `create_buffer_init` resolves.)

**3c. In the `pub struct Gpu` definition, replace the `pub mesh: Mesh,` field** with the four arena
fields (leave `instances` and `instance_bind_group` — the two lines below it — as they are):

```rust
    pub arena_vbo: wgpu::Buffer,          // ← was: pub mesh: Mesh,
    pub arena_vids: wgpu::Buffer,
    pub arena_ibo: wgpu::Buffer,
    pub arena_index_count: u32,
    instances: Vec<Instance>,             // (unchanged) non-pub — Instance is a private type
    pub instance_bind_group: wgpu::BindGroup,   // (unchanged)
```

**3d. In the `Ok(Self { … })` initializer at the end of `new`, replace the `mesh,` line** with the four
field names (order doesn't matter — keep them together where `mesh,` was):

```rust
            arena_vbo,          // ← was: mesh,
            arena_vids,
            arena_ibo,
            arena_index_count,
```

## Step 4 — draw the whole scene in one call: `src/engine/gpu.rs`

In `clear()`, find the mesh draw. It currently reads:

```rust
        let gm = self.mesh.gpu_mesh(&self.device); // build and upload once
        pass.set_vertex_buffer(0, gm.vbo.slice(..));
        pass.set_index_buffer(gm.ibo.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..gm.index_count, 0, 0..self.instances.len() as u32);
```

Replace those **four lines** with the arena binds + a single whole-scene draw (the `set_pipeline` and the
three `set_bind_group` lines just above them stay exactly as they are):

```rust
        pass.set_vertex_buffer(0, self.arena_vbo.slice(..));   // slot 0 — vertices
        pass.set_vertex_buffer(1, self.arena_vids.slice(..));  // slot 1 — per-vertex row ids
        pass.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.arena_index_count, 0, 0..1); // whole scene, ONE call
```

Nothing else in `clear()` changes — the `let objects = self.instances.len() as u32;` and
`self.performance.frame(draws, objects);` lines already report the row count as the object count.

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Five different solids in a row — red box, gold dodecahedron, green sphere, blue cylinder, purple torus
— over the grid. Console (F12):

```
perf: 60.0 fps | 16.67 ms | 3 draws | 5 objects
```

**3 draws for 5 distinct meshes.** Add a cone and a second box to `objects`: still 3 draws, fps unmoved
— the arena grew, the call count didn't. That flat count against a growing, *heterogeneous* scene is
what batching buys over the per-`Mesh` loop.

## Combining with instancing — distinct *and* repeated in one arena

The single draw above stores every object's vertices once, so 100 copies of a mesh would duplicate its
vertices 100 times — the arena's only weakness, and instancing (29) is its cure. The two are
**complementary, not either/or**: both read the *same* group-2 `instances` table. Keep the arena for
distinct meshes and let repeats replay a sub-range of it.

The trick: stop baking `base_vertex` into the indices, and instead remember, per distinct mesh, **where
it lives in the arena** and **which rows point at it**:

```rust
struct Draw {
    index_range: std::ops::Range<u32>,   // this mesh's slice of the shared index buffer
    base_vertex: i32,                     // where its vertices start in the arena
    // the instance rows that render it (1 = individual, N = instanced)
    rows: std::ops::Range<u32>,
}
```

Then the mesh pass is a short loop — buffers bound **once**, one instanced `draw_indexed` per *distinct*
mesh, each replaying its slice over however many rows share it:

```rust
        pass.set_vertex_buffer(0, self.arena_vbo.slice(..));   // bound ONCE for the whole loop
        pass.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
        for d in &self.draws {
            pass.draw_indexed(d.index_range.clone(), d.base_vertex, d.rows.clone());
            draws += 1;
        }
```

- an **individual** mesh → `rows` is length 1 (one row, one copy),
- an **instanced** mesh (100 chairs, every edge of the scene) → `rows` is length N, vertices stored once.

Here `@builtin(instance_index)` is the row selector again (the per-vertex `@location(3)` id from Step 1
is only needed by the *single*-draw variant; the per-mesh loop uses the builtin like 29 did). Cost is a
**handful of draws — one per unique mesh, not per object** — the shape real scenes want. Lesson 80
(multi-draw-indirect) folds that handful into a single GPU call, but the data layout is already this.

**Edges are the first real customer.** Lesson 31 draws the whole scene's wireframe as **instanced
cylinders**: the arena holds *one* short cylinder mesh, and the instance table gets one row per edge
(endpoints + color baked into `model`). That's this exact combined path — one distinct mesh replayed
over thousands of rows — so 31 is where "instanced *and* individual meshes in one arena" stops being
theory and ships.

## Recap

```
Ch 29: ONE mesh replayed N times — @builtin(instance_index) picks the row. Identical geometry only.
Ch 30: N DIFFERENT meshes concatenated into one vertex+index ARENA, drawn in a single draw_indexed.
       The vertex→row link moves from @builtin(instance_index) to a per-vertex u32 id (2nd vertex
       buffer, @location 3); indices are folded arena-global (base + i). Same group-2 instance
       table. 5 distinct solids → 3 draws / 5 objects, and it holds as the scene grows.
Both:  drop the baked base_vertex, keep a per-mesh {index_range, base_vertex, rows} descriptor, and
       loop one instanced draw_indexed per DISTINCT mesh (rows=1 individual, rows=N instanced).
       Arena + instancing share the group-2 table; a handful of draws, vertices stored once.
       This is 31's path.
```

Edited: `shaders/triangle.wgsl` (`VsIn` +`@location(3) inst_id`, read `instances[inst_id]`, drop the
builtin), `pipelines/build.rs` (triangle vertex state +`instance_id_layout`), `engine/gpu.rs` (arena
build replacing the single mesh; one `draw_indexed` over it; `mesh`→`arena_*` fields).

## Next

`31-edges.md` — edges come back as **instanced cylinders**: one 32-byte segment row per edge (endpoints
+ color), one cylinder mesh, one draw — the same instance-table idea from 29 applied to line geometry,
so the whole scene's edges cost a single call instead of one per mesh.
