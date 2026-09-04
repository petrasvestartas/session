# 06 BReps, NURBS and 2D sheets

- At the end a BRep or a NURBS surface tessellates into the arena and draws like a mesh, and a PDF page draws its fills in document order with its lettering on top - before this lesson both were empty rows and a page was linework only.
- Sheet content differs from solid faces only in WHEN it is drawn and whether it writes depth, so it shares the vertex table, the vids and the shader: the arena grows two more index runs, not a lane.
- A page's regions are exactly coplanar; no depth bias can sort them, the file's order can. The fill run writes no depth and the lettering run draws after the lines.
- A sheet is recognised per file, after its walk, from the rows it added: every row at the file placement, every local box flat in z. No importer flag, no file-type switch.
- An open mesh is not a solid: `FLAG_OPEN` turns the facing cull off for its ink, and `FLAG_PRINT` lights a fill flat from both sides while a real back face turns red.

<svg viewBox="0 0 720 330" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Lesson 6 on the two-halves map: app/ walks BReps, surfaces and sheets into the three index runs of ArenaRows and the Instance flags; engine/ uploads the runs, builds the sheet pipeline and draws them in frame order" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="s6a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#f0b35c"/></marker><marker id="s6b" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
  <text x="14" y="16" fill="#f0b35c" font-size="11">app/  kernel -&gt; rows</text>
  <text x="360" y="16" fill="#7ed37e" font-size="11" text-anchor="middle">Upload  the contract</text>
  <text x="706" y="16" fill="#6fb3ff" font-size="11" text-anchor="end">engine/  rows -&gt; pixels</text>
  <g fill="none" stroke="#f0b35c">
    <rect x="14" y="28" width="220" height="48" stroke="#7ed37e" stroke-width="1.3"/>
    <rect x="14" y="84" width="220" height="44"/>
    <rect x="14" y="136" width="220" height="44"/>
    <rect x="14" y="188" width="220" height="44"/>
    <rect x="14" y="240" width="220" height="26"/>
  </g>
  <g fill="#d7dae0" font-size="10">
    <text x="22" y="43">walk/brep.rs  (new)</text>
    <text x="22" y="99">walk/mesh.rs</text>
    <text x="22" y="151">walk/mod.rs</text>
    <text x="22" y="203">walk/bounds.rs</text>
    <text x="22" y="257">scene.rs</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="22" y="56">walk_brep · walk_surface</text>
    <text x="22" y="69">-&gt; walk_mesh(MODEL): no sheet runs</text>
    <text x="22" y="113">is_print_fill · MeshOpts · index_run</text>
    <text x="22" y="126">FLAG_PRINT · FLAG_OPEN</text>
    <text x="22" y="165">BRep · NurbsSurface · Element arms</text>
    <text x="22" y="178">all on the SOLID lane</text>
    <text x="22" y="217">is_planar · mark_sheet</text>
    <text x="22" y="230">FLAG_SHEET · unset pen -&gt; 0.5 mm</text>
    <text x="100" y="257">is_planar? mark_sheet</text>
  </g>
  <line x1="234" y1="130" x2="252" y2="130" stroke="#f0b35c" marker-end="url(#s6a)"/>
  <rect x="254" y="28" width="212" height="238" fill="none" stroke="#7ed37e"/>
  <text x="360" y="46" fill="#d7dae0" font-size="10" text-anchor="middle">ArenaRows</text>
  <g fill="#888" font-size="9">
    <text x="264" y="62">verts · vids</text>
    <text x="264" y="76">idx        faces, depth write</text>
    <text x="264" y="90">idx_print  fills, doc order  (new)</text>
    <text x="264" y="104">idx_text   lettering, last  (new)</text>
  </g>
  <line x1="264" y1="116" x2="456" y2="116" stroke="#3a3a3a"/>
  <text x="360" y="134" fill="#d7dae0" font-size="10" text-anchor="middle">Instance.flags</text>
  <g fill="#888" font-size="9">
    <text x="264" y="150">FLAG_PRINT  8   lit flat, both sides</text>
    <text x="264" y="164">FLAG_OPEN  16   no facing cull</text>
    <text x="264" y="178">FLAG_SHEET 32   ink takes no lift</text>
  </g>
  <line x1="264" y1="190" x2="456" y2="190" stroke="#3a3a3a"/>
  <text x="360" y="208" fill="#d7dae0" font-size="10" text-anchor="middle">SegRows pens</text>
  <g fill="#888" font-size="9">
    <text x="264" y="224">radius 0 on a sheet -&gt; 0.5 mm</text>
    <text x="264" y="238">(a plotter hairline)</text>
  </g>
  <line x1="466" y1="130" x2="484" y2="130" stroke="#6fb3ff" marker-end="url(#s6b)"/>
  <g fill="none" stroke="#6fb3ff">
    <rect x="486" y="28" width="220" height="70"/>
    <rect x="486" y="106" width="220" height="44"/>
    <rect x="486" y="158" width="220" height="44"/>
    <rect x="486" y="210" width="220" height="56"/>
  </g>
  <g fill="#d7dae0" font-size="10">
    <text x="494" y="43">gpu/arena.rs</text>
    <text x="494" y="121">shaders/triangle.wgsl</text>
    <text x="494" y="173">ribbon · cylinder · sphere.wgsl</text>
    <text x="494" y="225">gpu/render.rs</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="494" y="57">5 GrowBufs: + print, text</text>
    <text x="494" y="70">sheet pipeline: Blended, ReadOnly</text>
    <text x="494" y="83">draw_print · draw_text</text>
    <text x="494" y="135">print: lit 1.0 · back face: red</text>
    <text x="494" y="187">OPEN skips the cull · SHEET no lift</text>
    <text x="494" y="239">frame list: fills after faces,</text>
    <text x="494" y="252">lettering after lines</text>
  </g>
  <line x1="14" y1="280" x2="706" y2="280" stroke="#3a3a3a"/>
  <text x="14" y="298" fill="#888" font-size="9">frame  1 background · 2 grid · 3 faces · 4 sheet fills · 5 mesh edges · 6 markers · 7 lines · 8 lettering · 9 dots</text>
  <text x="14" y="314" fill="#888" font-size="9">green = created in lesson 6 · 4 and 8 are new; everything that writes depth still comes first</text>
</svg>

## Step 1 - Give the arena two more index runs

- A page's fills are exactly coplanar, so only draw order can sort them: their triangles go to a second run drawn with depth write off, and its lettering to a third drawn last. All three runs index the one vertex table.
- The header also names the two pipeline knobs (`ColorWrite`, `DepthMode`) the sheet run takes in Step 9.

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
//! The mesh lane: one vertex table every mesh shares, and the index run drawn from it -
//! solid faces. `ArenaRows` is one upload's delta; `ArenaLane` is the GPU side.

use crate::engine::pipelines::{build, instance_id_layout, module, vertex_layout, Layouts, PipelineDesc, Target};
```

**Replace with:**

```rust
//! The mesh lane: one vertex table every mesh, BRep and sheet fill shares, and the three
//! index runs drawn from it - solid faces, sheet fills (depth write off, document order) and
//! lettering (last of all). `ArenaRows` is one upload's delta; `ArenaLane` is the GPU side.

use crate::engine::pipelines::{build, instance_id_layout, module, vertex_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
```

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
/// One upload's mesh rows: vertices, their object rows, and the index run.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
}
```

**Replace with:**

```rust
/// One upload's mesh rows: vertices, their object rows, and the three index runs.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
        drop_rows(&mut self.idx);
```

**Add below it:**

```rust
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
```

## Step 2 - Tell a print fill from a surface

- The PDF importer gives every fill - each glyph, each poche region - a single width of 0, and that one test drives the wireframe skip, the index run and `FLAG_PRINT`.
- `MeshOpts` grows two switches so a BRep's tessellation can decline the sheet runs and the open flag: a tessellation is often numerically non-watertight and would lose its facing cull wholesale.

_Paste it._
**Find** in `src/app/walk/mesh.rs`:

```rust
//! (`mesh_ink`) unless the mesh is dense or edges are switched off. The gates
```

**Replace with:**

```rust
//! (`mesh_ink`) unless the mesh is dense, a print fill, or edges are switched off. The gates
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
use crate::engine::gpu::arena::ArenaRows;
```

**Add below it:**

```rust
use crate::engine::gpu::Instance;
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
/// How one mesh is walked.
pub struct MeshOpts {}

impl MeshOpts {
    /// A `Mesh` object.
    pub const OBJECT: MeshOpts = MeshOpts {};
}
```

**Replace with:**

```rust
/// A fill (every PDF glyph, every poche region) broadcasts a single width of 0: print, not
/// surface. One test drives the wireframe skip, the index run and `FLAG_PRINT`.
pub fn is_print_fill(m: &Mesh) -> bool {
    m.widths().len() == 1 && m.widths()[0] == 0.0
}

/// How one mesh is walked: whether a print fill takes the sheet index runs (and
/// `FLAG_PRINT`), and whether an open mesh may raise `FLAG_OPEN`.
pub struct MeshOpts {
    pub sheet_lanes: bool,
    pub allow_open: bool,
}

impl MeshOpts {
    /// A `Mesh` object: print fills take the sheet runs; open meshes are flagged.
    pub const OBJECT: MeshOpts = MeshOpts { sheet_lanes: true, allow_open: true };
    /// A tessellated BRep or surface: always the depth-tested run, never `FLAG_OPEN`.
    pub const MODEL: MeshOpts = MeshOpts { sheet_lanes: false, allow_open: false };
    /// An element's mesh: sheet runs, but an element is never flagged open.
    pub const ELEMENT: MeshOpts = MeshOpts { sheet_lanes: true, allow_open: false };
}
```

## Step 3 - Send a mesh's triangles to their run

- Which run a mesh joins decides WHEN it is drawn; the mesh the importer names `text` goes last of all. A print fill skips the ink pass - a glyph has no edges worth a wire.
- An open mesh walked as an OBJECT earns `FLAG_OPEN`, so the ink shaders leave the interior seen through its hole alone.

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
    pub fn mark(&mut self, _name: &str) {}
}
```

**Add below it:**

```rust

/// Which index run a mesh's triangles join decides WHEN it is drawn: sheet fills composite
/// in document order, lettering ("text", named by the PDF importer) goes last of all.
fn index_run<'a>(arena: &'a mut ArenaRows, m: &Mesh, sheet: bool) -> &'a mut Vec<u32> {
    if !sheet {
        return &mut arena.idx;
    }
    if m.name == "text" { &mut arena.idx_text } else { &mut arena.idx_print }
}
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
    let cx = mc.cx;
```

**Replace with:**

```rust
    let (cx, o) = (mc.cx, mc.opts);
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
    let idx = &mut arena.idx;
```

**Replace with:**

```rust
    let print = is_print_fill(m);
    let idx = index_run(arena, m, o.sheet_lanes && print);
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
    let thickness = mesh_thickness(&positions(&rm.vertices), &rm.indices);
    let row = Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags: 0, faces: true, thickness };

    if rm.indices.len() / 3 > MESH_RAW_MIN || knobs::no_edges() {
```

**Replace with:**

```rust
    let flags = if o.sheet_lanes && print { Instance::FLAG_PRINT } else { 0 };
    let thickness = mesh_thickness(&positions(&rm.vertices), &rm.indices);
    let row = Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags, faces: true, thickness };

    if rm.indices.len() / 3 > MESH_RAW_MIN || print || knobs::no_edges() {
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
    edges_and_dots(ink, m, &topo, &mut icx);
    row
}
```

**Replace with:**

```rust
    edges_and_dots(ink, m, &topo, &mut icx);

    // An open mesh is not a solid: the facing cull would strip interior surface seen through
    // the hole, so the shaders skip it like FLAG_INSIDE.
    let open = o.allow_open && !topo.closed;
    Row { flags: if open { row.flags | Instance::FLAG_OPEN } else { row.flags }, ..row }
}
```

## Step 4 - Tessellate BReps and surfaces

- A BRep and a surface are meshes by the time the arena sees them: the kernel tessellates, the walk tints the mesh with the object's surface colour and hands it to `walk_mesh` as a MODEL mesh.
- Nothing else in the viewer learns a new geometry type: no lane, no shader, no pipeline.

_Type it._
**Create `src/app/walk/brep.rs`**

```rust
//! A BRep or a NURBS surface into the tables: tessellate, tint, hand the mesh to `walk_mesh`
//! as a MODEL mesh - no sheet lanes, no `FLAG_OPEN` (a tessellation is often numerically
//! non-watertight and would lose the facing cull wholesale).

use session_rust::{BRep, NurbsSurface};
use crate::engine::gpu::arena::ArenaRows;
use super::{Row, WalkCx};
use super::mesh::{walk_mesh, MeshCx, MeshOpts};
use super::mesh_ink::Ink;

/// Tessellate a BRep with its surface colour and walk it.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut bm = b.mesh();
    bm.set_objectcolor(b.surfacecolor.clone());
    walk_mesh(arena, ink, &bm, &MeshCx { cx, opts: &MeshOpts::MODEL })
}

/// Tessellate a surface with its first face colour and walk it.
pub fn walk_surface(arena: &mut ArenaRows, ink: &mut Ink, s: &NurbsSurface, cx: &WalkCx) -> Row {
    let mut sm = s.mesh();
    if let Some(c) = s.facecolors.first() {
        sm.set_objectcolor(c.clone());
    }
    walk_mesh(arena, ink, &sm, &MeshCx { cx, opts: &MeshOpts::MODEL })
}
```

## Step 5 - Dispatch BReps, surfaces and elements

- Three empty arms become real: a BRep and a surface take the SOLID lane through `brep.rs`, and an `Element` unwraps to its mesh or BRep. An element's mesh keeps the sheet runs but is never flagged open.

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use curves::{walk_line, walk_nurbscurve, walk_polyline};
```

**Add above it:**

```rust
use brep::{walk_brep, walk_surface};
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
pub mod bounds;
```

**Add below it:**

```rust
pub mod brep;
```

_Paste it._
**Find** in `src/app/walk/mod.rs`:

```rust
    /// The SOLID lane a mesh reaches: the arena for its faces and the ink pair.
    fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
```

**Replace with:**

```rust
    /// The SOLID lane a tessellated surface reaches: the arena for its faces and the ink pair.
    fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
```

_Paste it._
**Find** in `src/app/walk/mod.rs`:

```rust
/// One object into the tables. Meshes take the SOLID lane; free linework
/// and points the FLAT lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
```

**Replace with:**

```rust
/// One object into the tables. Meshes, BReps and surfaces take the SOLID lane; free linework
/// and points the FLAT lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::BRep(_) => Row::thin(Aabb::empty()),
        Geometry::NurbsSurface(_) => Row::thin(Aabb::empty()),
```

**Replace with:**

```rust
        Geometry::BRep(b) => {
            let (arena, mut ink) = w.solid();
            walk_brep(arena, &mut ink, b, cx)
        }
        Geometry::NurbsSurface(s) => {
            let (arena, mut ink) = w.solid();
            walk_surface(arena, &mut ink, s, cx)
        }
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::Element(_) => Row::thin(Aabb::empty()),
```

**Replace with:**

```rust
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(m) => {
                let (arena, mut ink) = w.solid();
                walk_mesh(arena, &mut ink, m, &MeshCx { cx, opts: &MeshOpts::ELEMENT })
            }
            ElementGeometry::BRep(b) => {
                let (arena, mut ink) = w.solid();
                walk_brep(arena, &mut ink, b, cx)
            }
            ElementGeometry::None => Row::thin(Aabb::empty()),
        },
```

## Step 6 - Recognise a sheet and mark it

- A drawing sheet is a file whose every new row sits at the file placement with local boxes flat in z - a page authored at z = 0. The test runs over the rows one walk added, so the baselines also remember where the pens started.
- Marking a sheet sets `FLAG_SHEET` on its objects (the ink lanes drop their lift) and turns every unset pen into a 0.5 mm world hairline, like a plotter pen.

_Type it._
**Find** in `src/app/walk/bounds.rs`:

```rust
//! local box): the file's world extent.

use std::collections::HashMap;
use crate::engine::gpu::Upload;
use crate::math::Aabb;
```

**Replace with:**

```rust
//! local box): the file's world extent, the planar test, and the sheet marking.

use std::collections::HashMap;
use crate::engine::gpu::{Instance, Upload};
use crate::math::{Aabb, Mat4};
```

_Type it._
**Find** in `src/app/walk/bounds.rs`:

```rust
pub struct Baselines {
    pub obj: usize,
}
```

**Replace with:**

```rust
pub struct Baselines {
    pub obj: usize,
    pub pipe: usize,
    pub ribbon: usize,
}
```

_Type it._
**Find** in `src/app/walk/bounds.rs`:

```rust
        Self { obj: t.obj.rows.len() }
```

**Replace with:**

```rust
        Self { obj: t.obj.rows.len(), pipe: t.seg.pipes.len(), ribbon: t.seg.ribbons.len() }
```

_Type it._
**Find** in `src/app/walk/bounds.rs`:

```rust
        out.union(&r.bounds.placed(&r.place));
    }
    out
}
```

**Add below it:**

```rust

/// A planar file: every new row sits at the FILE placement and their local boxes span less
/// than a micron along local z - a drawing sheet authored at z = 0.
pub fn is_planar(t: &Upload, from: &Baselines, place: &Mat4) -> bool {
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;
    for r in t.obj.rows.iter().skip(from.obj) {
        if r.place != *place {
            return false;
        }
        if !r.bounds.is_finite() {
            continue;
        }
        lo = lo.min(r.bounds.min[2]);
        hi = hi.max(r.bounds.max[2]);
    }
    lo.is_finite() && (hi - lo).abs() < 1e-3
}

/// Every row of a planar file is page content: `FLAG_SHEET` on its objects (the ink lanes
/// drop their lift) and every unset pen becomes a 0.5 mm world hairline, like a plotter pen.
pub fn mark_sheet(t: &mut Upload, from: &Baselines) {
    for o in t.obj.rows.iter_mut().skip(from.obj) {
        o.flags |= Instance::FLAG_SHEET;
    }
    for s in t.seg.pipes.iter_mut().skip(from.pipe).chain(t.seg.ribbons.iter_mut().skip(from.ribbon)) {
        if s.radius <= 0.0 {
            s.radius = 0.5;
        }
    }
}
```

## Step 7 - Mark the sheet after each file's walk

- The sweep runs right after the extent sweep, over the same baselines: the file's rows are all in the tables and nothing has been uploaded yet.

_Type it._
**Find** in `src/app/scene.rs`:

```rust
use crate::app::walk::bounds::{file_extent, Baselines};
```

**Replace with:**

```rust
use crate::app::walk::bounds::{file_extent, is_planar, mark_sheet, Baselines};
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.tables.bounds.union(&extent);
```

**Add below it:**

```rust
        if is_planar(&self.tables, &from, &place.m) {
            mark_sheet(&mut self.tables, &from);
        }
```

## Step 8 - Upload the two runs

- Two more `GrowBuf`s under the same growth policy; the sheet runs index the SAME vertex table, so `append` pushes five tables and the reset and release walk all five.
- `face_count` keeps reading the faces run only: sheet fills are not solid, and the MSAA policy must not switch to 4x for a page.

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
/// The arena on the GPU: three `GrowBuf`s under the one growth policy.
pub struct ArenaLane {
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
```

**Replace with:**

```rust
/// The arena on the GPU: five `GrowBuf`s under the one growth policy.
pub struct ArenaLane {
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
    /// Three one-row tables; the first upload sizes them.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
```

**Replace with:**

```rust
    /// Five one-row tables; the first upload sizes them.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
            faces: GrowBuf::new(ctx, "arena.ibo", 4, INDICES),
```

**Add below it:**

```rust
            print: GrowBuf::new(ctx, "arena.ibo.print", 4, INDICES),
            text: GrowBuf::new(ctx, "arena.ibo.text", 4, INDICES),
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
    /// Append one file's rows.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
    }
```

**Replace with:**

```rust
    /// Append one file's rows. The sheet runs index the SAME vertex table.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
    }
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
        self.faces.reset();
```

**Add below it:**

```rust
        self.print.reset();
        self.text.reset();
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
    /// Hand every buffer back: three one-row tables again.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
    }
```

**Replace with:**

```rust
    /// Hand every buffer back: five one-row tables again.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
    /// Indices in the SOLID faces run - the MSAA policy reads it.
    pub fn face_count(&self) -> u32 {
```

**Replace with:**

```rust
    /// Indices in the SOLID faces run - the MSAA policy reads it; sheet fills are not solid.
    pub fn face_count(&self) -> u32 {
```

## Step 9 - Build the sheet pipeline and its two draws

- One more pipeline over the same shader and vertex layout: alpha-blended, depth read-only. Fills composite in document order and 3D geometry in front still occludes them.
- The two draws are the faces draw with another pipeline and another run; `draw_run` already returns 0 for an empty run, so a scene without a page costs nothing.

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
/// The pipeline over the arena: solid faces (opaque: the shader writes alpha 1).
struct ArenaPipelines {
    faces: wgpu::RenderPipeline,
}
```

**Replace with:**

```rust
/// The two pipelines over the arena: solid faces (opaque: the shader writes alpha 1) and sheet
/// runs (blended, depth read-only).
struct ArenaPipelines {
    faces: wgpu::RenderPipeline,
    sheet: wgpu::RenderPipeline,
}
```

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
        self.draw_run(pass, b, &self.pipes.faces, &self.faces)
    }
```

**Add below it:**

```rust

    /// Sheet fills: same vertex table, depth write off, so a page's exactly coplanar regions
    /// composite in document order. 3D geometry in front still occludes them.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.sheet, &self.print)
    }

    /// Lettering, last of everything: a page paints its text on top of hatching and linework.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.sheet, &self.text)
    }
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
/// The arena pipelines for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> ArenaPipelines {
```

**Replace with:**

```rust
/// The two arena pipelines for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, shader: &wgpu::ShaderModule, target: Target) -> ArenaPipelines {
```

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
        faces: build(dev, target, &base.with("triangle", "fs_main").bias(FACE_BIAS)),
```

**Add below it:**

```rust
        sheet: build(dev, target, &base.with("triangle.sheet", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnly)),
```

_Paste it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
    /// Test only, strict `Greater`: the grid.
    ReadOnly,
```

**Replace with:**

```rust
    /// Test only, strict `Greater`: sheet fills and the grid.
    ReadOnly,
```

## Step 10 - Light print flat and paint back faces red

- Print is paper: read from both sides, lit at 1.0, so a page never shades with the key light. A back face on anything else is a flipped normal or the inside of an open solid, and it shows red so the fault is seen, not hidden.
- The flag travels as one interpolated float so the fragment stage needs no instance fetch.

_Type it._
**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    feather: f32,
};
```

**Add below it:**

```wgsl

const FLAG_PRINT: u32 = 8u;
const BACKFACE_COLOR: vec3<f32> = vec3<f32>(0.80, 0.05, 0.05);
```

_Type it._
**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    @location(2) normal: vec3<f32>,
```

**Add below it:**

```wgsl
    @location(3) print: f32,
```

_Type it._
**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    o.normal = (inst.model * vec4<f32>(in.normal, 0.0)).xyz;
```

**Add below it:**

```wgsl
    o.print = select(0.0, 1.0, (inst.flags & FLAG_PRINT) != 0u);
```

_Type it._
**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    let lit = hemi + key + fill;
    return vec4<f32>(in.color * lit, 1.0);
```

**Replace with:**

```wgsl
    let lit = hemi + key + fill;

    // A back face is a flipped normal or the inside of an open solid: shown red. Print is
    // paper, read from both sides, lit flat.
    let backface = !front && in.print <= 0.5;
    let base = select(in.color, BACKFACE_COLOR, backface);
    return vec4<f32>(base * select(lit, 1.0, in.print > 0.5), 1.0);
```

## Step 11 - Let open meshes and sheets through the ink

- The three ink shaders cull an edge or a vertex whose faces all turn away; on an open mesh that strips the interior seen through the hole, so `FLAG_OPEN` bypasses the cull exactly like `FLAG_INSIDE`.
- A sheet's ribbons take no lift: its fills write no depth, and its lettering must land on top of the linework, not under a lifted hairline.

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
const FLAG_INSIDE: u32 = 4u;
```

**Add below it:**

```wgsl
const FLAG_OPEN: u32 = 16u;
const FLAG_SHEET: u32 = 32u;
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    let inside = (inst.flags & FLAG_INSIDE) != 0u;
```

**Replace with:**

```wgsl
    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u;
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    // Lift the ink toward the camera: in w for perspective, in ndc z for ortho.
    // Depth: a segment with faces is drawn in them (each side corner at its plane's depth at
    // that pixel, the centre at the edge's own); one without lifts a hair.
    let thick = inst.thickness;
    var wn = e.w;
    var zn = e.z;
    if (seg.facing != FACING_UNKNOWN) {
```

**Replace with:**

```wgsl
    // Lift the ink toward the camera: in w for perspective, in ndc z for ortho; a sheet takes
    // none (its fills write no depth and its lettering must land on top).
    // Depth: a segment with faces is drawn in them (each side corner at its plane's depth at
    // that pixel, the centre at the edge's own); one without lifts a hair; a sheet takes
    // nothing (its fills write no depth).
    let thick = inst.thickness;
    var wn = e.w;
    var zn = e.z;
    let sheet = (inst.flags & FLAG_SHEET) != 0u;
    if (sheet) {
    } else if (seg.facing != FACING_UNKNOWN) {
```

_Type it._
**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
const FLAG_INSIDE: u32 = 4u;
```

**Add below it:**

```wgsl
const FLAG_OPEN: u32 = 16u;
```

_Type it._
**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
    let inside = (inst.flags & FLAG_INSIDE) != 0u;
```

**Replace with:**

```wgsl
    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u;
```

_Type it._
**Find** in `src/shaders/sphere.wgsl`:

```wgsl
const FLAG_INSIDE: u32 = 4u;
```

**Add below it:**

```wgsl
const FLAG_OPEN: u32 = 16u;
```

_Type it._
**Find** in `src/shaders/sphere.wgsl`:

```wgsl
    let inside = (inst.flags & FLAG_INSIDE) != 0u;
```

**Replace with:**

```wgsl
    let inside = (inst.flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u;
```

## Step 12 - Slot the sheet draws into the frame

- The order is the contract: fills right after the faces (they read the faces' depth and write none), lettering after the lines (it is the one thing a page's hatching and linework must never cover).
- The MSAA policy is untouched: a pure sheet has no faces, pipes or spheres and stays at 1x.

_Paste it._
**Find** in `src/engine/gpu/render.rs`:

```rust
//! the blended ink after.

use super::frame::Binds;
```

**Replace with:**

```rust
//! the blended ink after, lettering last.

use super::frame::Binds;
```

_Paste it._
**Find** in `src/engine/gpu/render.rs`:

```rust
    /// 1 background · 2 grid · 3 faces · 4 mesh edges · 5 vertex
    /// markers · 6 lines · 7 point dots. Lines write no depth: two lines on one
```

**Replace with:**

```rust
    /// 1 background · 2 grid · 3 faces · 4 sheet fills · 5 mesh edges · 6 vertex
    /// markers · 7 lines · 8 lettering · 9 point dots. Lines write no depth: two lines on one
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
        draws += self.arena.draw_faces(pass, b);
```

**Add below it:**

```rust
        draws += self.arena.draw_print(pass, b);
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
            draws += self.segments.draw_ribbons(pass, b);
        }
```

**Add below it:**

```rust
        draws += self.arena.draw_text(pass, b);
```

_Paste it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// pipes, spheres) and a canvas MSAA can afford.
    fn msaa_now(&self) -> u32 {
```

**Replace with:**

```rust
    /// pipes, spheres) and a canvas MSAA can afford; a pure sheet stays at 1x.
    fn msaa_now(&self) -> u32 {
```

## Run

```bash
trunk serve
```

- Open http://127.0.0.1:8770/ - the two sheets of the local scene (`querschnitt`, `treppenhaus`, the row at y = 4 m) now carry their fills and their lettering, not just their linework, and a region drawn later in the PDF sits on top of an earlier one.
- Any BRep or NURBS surface in a scene draws as a tessellated, lit mesh with its own edges; an open mesh shows its interior through the hole in red.

## Why

- One vertex table, three index runs: a run is a `Vec<u32>` and a `GrowBuf`, so sheet content costs no new lane, shader or vertex layout - only the pipeline state that differs (blend on, depth write off).
- Depth write off plus document order is the only correct compositor for coplanar regions; a bias or a lift would sort them by tessellation noise and still leak through 3D geometry.
- Lettering is its own run because it is the smallest thing on a page and the one thing nothing on the page may cover; the cost is one more indexed draw at the end of the frame.
- `is_print_fill` reads the mesh, not its name: the importer's single width of 0 marks a glyph and a poche region alike, and only `text` needs a name because only its ORDER differs.
- The sheet test is a per-file sweep over the rows a walk just added, so it needs no importer flag and cannot misfire on one flat mesh inside a 3D file: every row must sit at the file placement.
- MODEL opts keep tessellations honest: a BRep mesh is often not watertight to the bit, and flagging it OPEN would switch the facing cull off for the whole object.
- `FLAG_OPEN` reuses the `FLAG_INSIDE` path in all three ink shaders: the two situations want the same thing - draw the ink the cull would have hidden.
- Back faces turn red instead of being culled because a flipped normal is a modelling fault the viewer should show; print is exempt because paper has no wrong side.
