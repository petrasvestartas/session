# 05 Mesh ink and the thickness rule

- At the end every mesh in the scene wears a black wireframe and a marker on each vertex, `E` hides them, `L` flips flat quads to real tubes, and an outline drawn on a plate's underside never shows through its top face.
- The wires and markers are the SOLID lane: two more tables (`pipes`, `spheres`) over the very row types lesson 4 made, filled by one fused pass over the mesh (`mesh_topology.rs`, `mesh_ink.rs`) and drawn under a depth write with a prepass, so the AA feather of one stroke never rejects the next.
- The thickness rule closes here: faces never move (the arena recedes two depth-format steps, just enough to break an exact tie), ink that knows its faces is drawn IN them, and ink that knows none lifts a hair capped by its object's thickness.
- A free polyline lying on a plate face (`hosts.rs`) borrows that face's normal and the plate's thickness, so it lifts like the plate's own wires and can never be lifted through the plate.
- Tubes and Flat are one table and two pipelines: `LineStyle` flips at draw time with nothing re-uploaded, and the MSAA policy counts pipes and spheres as solid because, unlike ribbons and dots, they have hard edges.

<svg viewBox="0 0 720 384" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Lesson 5 on the two-halves map: mesh_topology and mesh_ink under app/walk fill the pipes and spheres tables of SegRows and GlyphRows in Upload; segments.rs and glyphs.rs under engine/gpu draw them through cylinder.wgsl, ribbon.wgsl and sphere.wgsl with a depth prepass; hosts.rs gives a plate outline its face" style="max-width:100%;height:auto;font:11px ui-monospace,SFMono-Regular,Menlo,Consolas,monospace">
  <defs><marker id="l5a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#f0b35c"/></marker><marker id="l5b" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
  <g fill="currentColor" font-size="11">
    <text x="14" y="18" fill="#f0b35c">app/  (the walk)</text>
    <text x="360" y="18" fill="#7ed37e" text-anchor="middle">Upload  (the line)</text>
    <text x="706" y="18" fill="#6fb3ff" text-anchor="end">engine/  (the GPU)</text>
  </g>
  <line x1="14" y1="24" x2="706" y2="24" stroke="currentColor" stroke-opacity="0.25"/>
  <g fill="none" stroke="#f0b35c">
    <rect x="14" y="36" width="210" height="40"/>
    <rect x="14" y="84" width="210" height="40" stroke-width="1.6"/>
    <rect x="14" y="132" width="210" height="40" stroke-width="1.6"/>
    <rect x="14" y="180" width="210" height="40"/>
    <rect x="14" y="228" width="210" height="40" stroke-width="1.6"/>
    <rect x="14" y="276" width="210" height="40"/>
  </g>
  <g fill="currentColor" font-size="10">
    <text x="22" y="51">walk/encode.rs</text><text x="22" y="66" fill-opacity="0.6">oct16 · pack_facing · BLACK · Pen.facing</text>
    <text x="22" y="99">walk/mesh_topology.rs  (new)</text><text x="22" y="114" fill-opacity="0.6">SlotMap · MeshTopo: edges, edge_faces, normals</text>
    <text x="22" y="147">walk/mesh_ink.rs  (new)</text><text x="22" y="162" fill-opacity="0.6">Ink · InkCx · push_pipes · push_markers</text>
    <text x="22" y="195">walk/mesh.rs · knobs.rs</text><text x="22" y="210" fill-opacity="0.6">MESH_RAW_MIN · COPLANAR_DOT · no_edges no_dots all_edges</text>
    <text x="22" y="243">walk/hosts.rs  (new) · curves.rs · scene.rs</text><text x="22" y="258" fill-opacity="0.6">Hosts::from_session · find -&gt; normal + thickness</text>
    <text x="22" y="291">walk/mod.rs · app/input.rs</text><text x="22" y="306" fill-opacity="0.6">Walk::solid · WalkCx.hosts · keys E L</text>
  </g>
  <line x1="224" y1="146" x2="262" y2="126" stroke="#f0b35c" marker-end="url(#l5a)"/>
  <line x1="224" y1="158" x2="262" y2="226" stroke="#f0b35c" marker-end="url(#l5a)"/>
  <line x1="224" y1="248" x2="262" y2="140" stroke="#f0b35c" marker-end="url(#l5a)"/>
  <g fill="none" stroke="#7ed37e" stroke-width="1.3">
    <rect x="264" y="100" width="192" height="52"/>
    <rect x="264" y="200" width="192" height="52"/>
  </g>
  <g fill="currentColor" font-size="10">
    <text x="272" y="115">seg: SegRows</text><text x="272" y="130" fill-opacity="0.6">pipes + ribbons: Vec&lt;CylinderSegment&gt;</text><text x="272" y="144" fill-opacity="0.6">facing = two oct16 face normals</text>
    <text x="272" y="215">glyph: GlyphRows</text><text x="272" y="230" fill-opacity="0.6">spheres + dots: Vec&lt;GlyphPoint&gt;</text><text x="272" y="244" fill-opacity="0.6">facing x3 = up to six face normals</text>
  </g>
  <line x1="456" y1="126" x2="504" y2="120" stroke="#6fb3ff" marker-end="url(#l5b)"/>
  <line x1="456" y1="226" x2="504" y2="178" stroke="#6fb3ff" marker-end="url(#l5b)"/>
  <g fill="none" stroke="#6fb3ff">
    <rect x="506" y="36" width="200" height="52"/>
    <rect x="506" y="94" width="200" height="52" stroke-width="1.6"/>
    <rect x="506" y="152" width="200" height="52" stroke-width="1.6"/>
    <rect x="506" y="210" width="200" height="52"/>
    <rect x="506" y="268" width="200" height="52"/>
  </g>
  <g fill="currentColor" font-size="10">
    <text x="514" y="51">gpu/buffers.rs · pipelines/mod.rs</text><text x="514" y="66" fill-opacity="0.6">Template { vbo, ibo } · ColorWrite::Masked</text><text x="514" y="80" fill-opacity="0.6">PipelineDesc::bias · template_layout</text>
    <text x="514" y="109">gpu/segments.rs + cylinder.wgsl  (new)</text><text x="514" y="124" fill-opacity="0.6">pipes · cylinder · ribbon_depth · unit_cylinder</text><text x="514" y="138" fill-opacity="0.6">draw_pipes: Tubes 1 draw, Flat prepass + colour</text>
    <text x="514" y="167">gpu/glyphs.rs + sphere.wgsl  (new)</text><text x="514" y="182" fill-opacity="0.6">spheres · sphere · sphere_depth · unit_quad</text><text x="514" y="196" fill-opacity="0.6">draw_spheres: prepass + colour, last of the solid</text>
    <text x="514" y="225">shaders/ribbon.wgsl · glyph.wgsl</text><text x="514" y="240" fill-opacity="0.6">edge_faces_camera · plane_step_mm · corner_step_mm</text><text x="514" y="254" fill-opacity="0.6">density_taper · fs_depth</text>
    <text x="514" y="283">gpu/arena.rs · view.rs · render.rs · mod.rs</text><text x="514" y="298" fill-opacity="0.6">FACE_BIAS · LineStyle · show_mesh_edges · markers</text><text x="514" y="312" fill-opacity="0.6">msaa: faces || pipes || spheres</text>
  </g>
  <line x1="14" y1="332" x2="706" y2="332" stroke="currentColor" stroke-opacity="0.25"/>
  <g fill="currentColor" font-size="10">
    <text x="14" y="350">scene_list: 1 background · 2 grid · 3 faces (recede FACE_BIAS) · <tspan fill="#6fb3ff">4 mesh edges</tspan> · <tspan fill="#6fb3ff">5 vertex markers</tspan> (write depth) · 6 lines · 7 point dots</text>
    <text x="14" y="366" fill-opacity="0.6">the rule: faces never move · ink with faces is drawn in them · ink without lifts a hair, capped by thickness · a hosted outline gets its plate's face</text>
    <text x="14" y="380" fill-opacity="0.6">orange = a producer, green = the rows Upload carries, blue = the lane; a thick border is a file created in this lesson</text>
  </g>
</svg>

## Step 1 - Encode a normal and pack the facing word

A mesh edge carries its two face normals in the `facing` word lesson 4 left at `FACING_UNKNOWN`: each normal is 16 bits of octahedral code, enough for the SIGN of a dot product, which is all the cull and the plane step ever ask. `Pen` learns the same word so a hosted polyline (Step 7) can carry a face too; every free pen says `FACING_UNKNOWN`.

_Type it._
**Find** in `src/app/walk/encode.rs`:

```rust
//! Row encodings shared by every producer: pen widths to radii, colours to RGBA8. Pure
//! functions on numbers.
```

**Replace with:**

```rust
//! Row encodings shared by every producer: pen widths to radii, colours to RGBA8, normals to
//! oct16 and the packed `facing` word the ink shaders test. Pure functions on numbers.
```

_Type it._
**Find** in `src/app/walk/encode.rs`:

```rust
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}
```

**Add below it:**

```rust

/// `signum` that never returns 0, so the -Z pole does not fold onto the +Z code.
fn sign_not_zero(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

/// One octahedral coordinate to a signed byte.
fn quant_snorm8(v: f64) -> u32 {
    (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

/// A unit vector in 16 bits, octahedral (~1.4 deg of error, used for the SIGN of a dot).
pub fn oct16(n: &[f64; 3]) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();
    if l.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater) {
        return None;
    }
    let (mut x, mut y) = (n[0] / l, n[1] / l);
    if n[2] < 0.0 {
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
    }
    Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}

/// Opaque black, packed: the wireframe's default pen.
pub const BLACK: u32 = 0xff00_0000;
```

_Type it._
**Find** in `src/app/walk/encode.rs`:

```rust
pub const FACING_UNKNOWN: u32 = u32::MAX;
```

**Add below it:**

```rust

/// The two faces an edge belongs to, packed into one word; a lone face is duplicated.
pub fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)),
        _ => (None, None),
    };
    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;
            if v == FACING_UNKNOWN { v ^ 1 } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}
```

_Type it._
**Find** in `src/app/walk/encode.rs`:

```rust
    pub color: u32,
```

**Add below it:**

```rust
    /// `FACING_UNKNOWN`, or the host face's normal twice for an outline lying on a plate.
    pub facing: u32,
```

Every pen a free producer builds names the word explicitly.

_Type it._
**Find** in `src/app/walk/curves.rs`:

```rust
        seg.ribbons.push(CylinderSegment { p0: w[0], radius: pen.radius, p1: w[1], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN });
```

**Replace with:**

```rust
        seg.ribbons.push(CylinderSegment { p0: w[0], radius: pen.radius, p1: w[1], instance_id: pen.row, color: pen.color, facing: pen.facing });
```

_Type it._
**Find** in `src/app/walk/curves.rs`:

```rust
    let pen = Pen { row, radius: encode_width(c.width), color: pack_rgba(color) };
```

**Replace with:**

```rust
    let pen = Pen { row, radius: encode_width(c.width), color: pack_rgba(color), facing: FACING_UNKNOWN };
```

_Type it._
**Find** in `src/app/walk/frames.rs`:

```rust
    let pen = Pen { row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()) };
```

**Replace with:**

```rust
    let pen = Pen { row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()), facing: FACING_UNKNOWN };
```

_Type it._
**Find** in `src/app/walk/frames.rs`:

```rust
    let pen = Pen { row, radius: 0.0, color: pack_rgba([0.0, 0.0, 0.0, 1.0]) };
```

**Replace with:**

```rust
    let pen = Pen { row, radius: 0.0, color: pack_rgba([0.0, 0.0, 0.0, 1.0]), facing: FACING_UNKNOWN };
```

## Step 2 - Give both tables a solid side

A wire is a `CylinderSegment` and a marker a `GlyphPoint`, the rows lesson 4 already uploads, so `SegRows` and `GlyphRows` each gain a second `Vec` and nothing about the row layout - or the mirror test over it - changes.

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
/// One upload's segments: the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    pub ribbons: Vec<CylinderSegment>,
}

impl SegRows {
    /// Empty the table and hand the allocation back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.ribbons);
    }
}
```

**Replace with:**

```rust
/// One upload's segments: the solid lane's pipes and the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    pub pipes: Vec<CylinderSegment>,
    pub ribbons: Vec<CylinderSegment>,
}

impl SegRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.ribbons);
    }
}
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
/// One upload's glyphs: the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub dots: Vec<GlyphPoint>,
}

impl GlyphRows {
    /// Empty the table and hand the allocation back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.dots);
    }
}
```

**Replace with:**

```rust
/// One upload's glyphs: the solid lane's vertex markers and the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub spheres: Vec<GlyphPoint>,
    pub dots: Vec<GlyphPoint>,
}

impl GlyphRows {
    /// Empty both tables and hand the allocations back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.dots);
    }
}
```

## Step 3 - Fuse the mesh topology

One pass over the faces yields everything the ink needs - the unique edges with their pen colours, each edge's two faces, the face normals and whether the mesh is closed - without the kernel's separate passes and their hash tables: an edge hangs off its low vertex on an intrusive chain, so finding one again is a walk of two or three entries. Positions are read by slot from the sorted key list, in f64 so the normals match the kernel's.

_Type it._
**Create `src/app/walk/mesh_topology.rs`**

```rust
//! One pass over a mesh's faces for everything the ink lanes need: the unique edges with
//! their pen colours, each edge's two faces, the face normals, and whether the mesh is
//! closed. Byte-identical to the kernel's four separate passes, without their hash tables.

use session_rust::{Mesh, Tolerance};
use super::encode::{pack_rgba, BLACK};

/// Vertex key -> slot (the key's position in the sorted key list). Dense keys index a Vec;
/// a sparse key space falls back to a map.
pub struct SlotMap {
    dense: Vec<u32>,
    sparse: std::collections::HashMap<usize, u32>,
}

impl SlotMap {
    /// From the sorted keys `Mesh::vertices()` emits.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);
        if max_key < 4 * keys.len().max(1) {
            let mut dense = vec![u32::MAX; max_key + 1];
            for (s, &k) in keys.iter().enumerate() {
                dense[k] = s as u32;
            }
            return Self { dense, sparse: std::collections::HashMap::new() };
        }
        let mut sparse = std::collections::HashMap::with_capacity(keys.len());
        for (s, &k) in keys.iter().enumerate() {
            sparse.insert(k, s as u32);
        }
        Self { dense: Vec::new(), sparse }
    }

    /// The slot of key `k`.
    pub fn slot(&self, k: usize) -> usize {
        if self.dense.is_empty() { self.sparse[&k] as usize } else { self.dense[k] as usize }
    }
}

/// The fused topology of one mesh.
pub struct MeshTopo {
    /// Unique edges as (low key, high key, packed pen colour), in first-seen order.
    pub edges: Vec<(usize, usize, u32)>,
    /// Per edge: the face slots walking (low, high) and (high, low); u32::MAX = none. A lone
    /// face always sits in slot 0.
    pub edge_faces: Vec<[u32; 2]>,
    /// Per face slot, in sorted-face-key order; None for a degenerate face.
    pub normals: Vec<Option<[f64; 3]>>,
    /// Every edge walked in both directions: no border.
    pub closed: bool,
}

/// One face's normal from the by-slot position table, the kernel's arithmetic and cut-off.
fn face_normal(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 {
        return None;
    }
    let (p0, p1, p2) = (vpos[slots.slot(vs[0])], vpos[slots.slot(vs[1])], vpos[slots.slot(vs[2])]);
    let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > Tolerance::ZERO_TOLERANCE { Some([n[0] / len, n[1] / len, n[2] / len]) } else { None }
}

/// The fused pass. Edges hang off their LOW vertex on an intrusive chain (`head` per slot,
/// `next` per edge), so finding an existing (lo, hi) is a walk of two or three entries.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    let mut faces: Vec<(usize, &Vec<usize>)> = Vec::with_capacity(m.face.len());
    for (k, v) in m.face.iter() {
        faces.push((*k, v));
    }
    faces.sort_unstable_by_key(|f| f.0);
    let cols = m.get_linecolors();

    let mut normals: Vec<Option<[f64; 3]>> = Vec::with_capacity(faces.len());
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()];
    let mut next: Vec<u32> = Vec::new();

    for (fs, (_, vs)) in faces.iter().enumerate() {
        normals.push(face_normal(vs, vpos, slots));
        let n = vs.len();
        for i in 0..n {
            let (u, v) = (vs[i], vs[(i + 1) % n]);
            let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) };
            let ls = slots.slot(lo);
            let mut ei = head[ls];
            while ei != u32::MAX && edges[ei as usize].1 != hi {
                ei = next[ei as usize];
            }
            if ei == u32::MAX {
                ei = edges.len() as u32;
                let pen = cols.get(edges.len()).map_or(BLACK, |c| pack_rgba(c.to_f32()));
                edges.push((lo, hi, pen));
                edge_faces.push([u32::MAX; 2]);
                next.push(head[ls]);
                head[ls] = ei;
            }
            // First face wins, like the kernel's `or_insert`.
            let f = &mut edge_faces[ei as usize][dir];
            if *f == u32::MAX {
                *f = fs as u32;
            }
        }
    }

    let mut closed = !m.vertex.is_empty();
    for f in edge_faces.iter_mut() {
        if f[0] == u32::MAX || f[1] == u32::MAX {
            closed = false;
        }
        if f[0] == u32::MAX {
            f[0] = f[1];
            f[1] = u32::MAX;
        }
    }
    // A declared hole ring's edges are borders by this test but not by the kernel's.
    if !closed && !m.face_holes.is_empty() {
        closed = m.is_closed();
    }

    MeshTopo { edges, edge_faces, normals, closed }
}
```

## Step 4 - Ink the mesh: pipes and markers

`push_pipes` writes one segment per visible edge, skipping width-0 edges and the diagonals across exactly coplanar faces; `push_markers` writes one glyph per vertex that has a visible edge, carrying up to six incident face normals so the disc hides only when every face turns away. Past `WIREFRAME_BLACK_MIN` edges the wireframe draws black whatever the file says.

_Type it._
**Create `src/app/walk/mesh_ink.rs`**

```rust
//! The ink a mesh wears: one pipe per visible edge, one marker per vertex - the SOLID lane.
//! Reads the fused topology and the positions by slot; writes `SegRows.pipes` and
//! `GlyphRows.spheres`, nothing else.

use session_rust::Mesh;
use session_rust::mesh::ColorMode;
use crate::app::knobs;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use super::encode::{encode_width, oct16, pack_facing, BLACK, FACING_UNKNOWN};
use super::mesh::{Lap, COPLANAR_DOT, WIREFRAME_BLACK_MIN};
use super::mesh_topology::{MeshTopo, SlotMap};

/// The two ink lanes a mesh reaches: pipes for its edges, spheres for its vertices.
pub struct Ink<'a> {
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
}

/// What the ink pass needs from the face pass: the object row, the f32 positions by slot,
/// the key -> slot map and the profiling clock.
pub struct InkCx<'a> {
    pub row: u32,
    pub vpos: &'a [[f32; 3]],
    pub slots: &'a SlotMap,
    pub lap: &'a mut Lap,
}

/// Edge `i`'s pen width: one entry broadcasts to every edge, an absent one is the 1.0 default.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 { w[0] } else { w.get(i).copied().unwrap_or(1.0) }
}

/// Width 0 = hidden: a triangulated fill asks for no wireframe.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// The normal in face slot `k` of an edge's pair, borrowed.
fn normal_of(topo: &MeshTopo, f: [u32; 2], k: usize) -> Option<&[f64; 3]> {
    if f[k] == u32::MAX { None } else { topo.normals[f[k] as usize].as_ref() }
}

/// Append edge `ei`'s faces to `fkeys`, deduped.
fn push_faces(edge_faces: &[[u32; 2]], ei: usize, fkeys: &mut Vec<usize>) {
    for &f in edge_faces[ei].iter() {
        if f == u32::MAX {
            continue;
        }
        let fk = f as usize;
        if !fkeys.contains(&fk) {
            fkeys.push(fk);
        }
    }
}

/// Word `k` of a marker's facing triple, by `pack_facing`'s rules.
fn facing_word(codes: &[u32], k: usize) -> u32 {
    match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
        (Some(a), b) => {
            let v = a | b.unwrap_or(a) << 16;
            if v == FACING_UNKNOWN { v ^ 1 } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// The pipe loop: one segment per visible, non-coplanar edge.
fn push_pipes(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &InkCx) {
    let w = m.widths();
    let black_wire = topo.edges.len() >= WIREFRAME_BLACK_MIN;
    ink.seg.pipes.reserve(topo.edges.len());
    for (i, (a, b, col)) in topo.edges.iter().enumerate() {
        let f = topo.edge_faces[i];
        let facing = pack_facing(normal_of(topo, f, 0), normal_of(topo, f, 1));
        if hidden(w, i) {
            continue;
        }
        // Interior tessellation: a diagonal across a flat region shares two coplanar faces.
        if let (Some(n0), Some(n1)) = (normal_of(topo, f, 0), normal_of(topo, f, 1)) {
            let dot = n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2];
            if dot >= COPLANAR_DOT && !knobs::all_edges() {
                continue;
            }
        }
        ink.seg.pipes.push(CylinderSegment {
            p0: cx.vpos[cx.slots.slot(*a)],
            radius: encode_width(width_at(w, i)),
            p1: cx.vpos[cx.slots.slot(*b)],
            instance_id: cx.row,
            color: if black_wire { BLACK } else { *col },
            facing,
        });
    }
}

/// Per vertex: the widest visible incident edge (its width and index), and the incident
/// edge list as CSR (`vstart`, `vinc`). Hidden edges still count for adjacency.
struct Incidence {
    best: Vec<(f64, usize)>,
    vstart: Vec<u32>,
    vinc: Vec<u32>,
}

/// Build the incidence tables over the topology.
fn incidence(m: &Mesh, topo: &MeshTopo, cx: &InkCx) -> Incidence {
    let w = m.widths();
    let nv = cx.vpos.len();
    let mut best = vec![(f64::NEG_INFINITY, 0usize); nv];
    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        if hidden(w, i) {
            continue;
        }
        let wi = width_at(w, i);
        for vk in [*a, *b] {
            let e = &mut best[cx.slots.slot(vk)];
            if wi > e.0 {
                *e = (wi, i);
            }
        }
    }

    let mut vstart = vec![0u32; nv + 1];
    for (a, b, _) in topo.edges.iter() {
        vstart[cx.slots.slot(*a) + 1] += 1;
        vstart[cx.slots.slot(*b) + 1] += 1;
    }
    for i in 0..nv {
        vstart[i + 1] += vstart[i];
    }
    let mut vinc = vec![0u32; 2 * topo.edges.len()];
    let mut cur = vstart.clone();
    for (i, (a, b, _)) in topo.edges.iter().enumerate() {
        for vk in [*a, *b] {
            let s = cx.slots.slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }
    Incidence { best, vstart, vinc }
}

/// The marker loop: one glyph per vertex with a visible edge, carrying up to six incident
/// face normals (widest edge's pair first) so the disc hugs every face at a corner.
fn push_markers(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &mut InkCx) {
    let inc = incidence(m, topo, cx);
    cx.lap.mark("incidence");
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();
    let nv = cx.vpos.len();
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    ink.glyph.spheres.reserve(nv);
    for (i, &(vw, ei)) in inc.best.iter().enumerate().take(nv) {
        if vw == f64::NEG_INFINITY {
            continue;
        }
        fkeys.clear();
        push_faces(&topo.edge_faces, ei, &mut fkeys);
        for &j in &inc.vinc[inc.vstart[i] as usize..inc.vstart[i + 1] as usize] {
            push_faces(&topo.edge_faces, j as usize, &mut fkeys);
        }
        codes.clear();
        for fk in &fkeys {
            if codes.len() >= 6 {
                break;
            }
            if let Some(n) = topo.normals[*fk].as_ref() && let Some(code) = oct16(n) {
                codes.push(code);
            }
        }
        ink.glyph.spheres.push(GlyphPoint {
            center: cx.vpos[i],
            radius: encode_width(vw),
            color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
            instance_id: cx.row,
            facing: facing_word(&codes, 0),
            facing_ext: [facing_word(&codes, 1), facing_word(&codes, 2)],
        });
    }
}

/// Pipes, then markers unless VIEWER_NO_DOTS.
pub fn edges_and_dots(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &mut InkCx) {
    push_pipes(ink, m, topo, cx);
    cx.lap.mark("pipe loop");
    if knobs::no_dots() {
        return;
    }
    push_markers(ink, m, topo, cx);
    cx.lap.mark("markers");
}
```

## Step 5 - Gate the ink in walk_mesh

A scan draws triangles only - past `MESH_RAW_MIN` triangles the decoration outweighs the geometry - and `VIEWER_NO_EDGES` does the same for the harness; under the gate the walk builds the slot map and the topology and calls the ink. The thresholds live next to the gate, and `knobs` is read on both targets now, so its `cfg` goes.

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
//! One mesh into the tables: its faces into the arena, its local box. Nothing here reads
//! the GPU.
```

**Replace with:**

```rust
//! One mesh into the tables: its faces into the arena, its local box, then the ink pass
//! (`mesh_ink`) unless the mesh is dense or edges are switched off. The gates
//! and thresholds live here. Nothing here reads the GPU.
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
use session_rust::Mesh;
#[cfg(not(target_arch = "wasm32"))]
use crate::app::knobs;
```

**Replace with:**

```rust
use session_rust::Mesh;
use crate::app::knobs;
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
use super::{Row, WalkCx};
```

**Add below it:**

```rust
use super::mesh_ink::{edges_and_dots, Ink, InkCx};
use super::mesh_topology::{mesh_topology, SlotMap};

/// Above this many triangles a mesh draws as TRIANGLES ONLY - no edges, no markers: on a
/// scan the decoration is 90x the geometry. The bunny (69k tri) keeps its wireframe.
pub const MESH_RAW_MIN: usize = 200_000;

/// At or above this many edges a mesh's wireframe draws BLACK whatever the file says.
pub const WIREFRAME_BLACK_MIN: usize = 10_000;

/// Two faces count as one flat region above this normal dot: EXACT coplanarity, so curvature
/// on a dense scan is never mistaken for tessellation.
pub const COPLANAR_DOT: f64 = 1.0 - 1e-9;
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
/// Faces into the arena and the mesh-local box.
pub fn walk_mesh(arena: &mut ArenaRows, m: &Mesh, mc: &MeshCx) -> Row {
```

**Replace with:**

```rust
/// Faces into the arena, the mesh-local box, then edges and dots unless a gate says no.
pub fn walk_mesh(arena: &mut ArenaRows, ink: &mut Ink, m: &Mesh, mc: &MeshCx) -> Row {
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
    Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags: 0, faces: true, thickness }
```

**Replace with:**

```rust
    let row = Row { bounds, spacing: mesh_spacing(&bounds, m.number_of_vertices()), flags: 0, faces: true, thickness };

    if rm.indices.len() / 3 > MESH_RAW_MIN || knobs::no_edges() {
        return row;
    }

    // Positions by slot from the KERNEL's vertex map, kept in f64 for the face normals.
    let keys = m.vertices();
    let slots = SlotMap::new(&keys);
    let mut vpos64: Vec<[f64; 3]> = Vec::with_capacity(keys.len());
    for &k in &keys {
        let v = &m.vertex[&k];
        vpos64.push([v.x, v.y, v.z]);
    }
    let mut vpos: Vec<[f32; 3]> = Vec::with_capacity(keys.len());
    for p in &vpos64 {
        vpos.push([p[0] as f32, p[1] as f32, p[2] as f32]);
    }
    let topo = mesh_topology(m, &keys, &vpos64, &slots);
    lap.mark("topology");

    let mut icx = InkCx { row: cx.row, vpos: &vpos, slots: &slots, lap: &mut lap };
    edges_and_dots(ink, m, &topo, &mut icx);
    row
```

Three more presence flags, the shape of the two lesson 3 made.

_Paste it._
**Find** in `src/app/knobs.rs`:

```rust
static DROP_SESSIONS: OnceLock<bool> = OnceLock::new();
```

**Add below it:**

```rust
static NO_EDGES: OnceLock<bool> = OnceLock::new();
static NO_DOTS: OnceLock<bool> = OnceLock::new();
static ALL_EDGES: OnceLock<bool> = OnceLock::new();
```

_Paste it._
**Find** in `src/app/knobs.rs`:

```rust
    env_flag("VIEWER_DROP_SESSIONS", &DROP_SESSIONS)
}
```

**Add below it:**

```rust

/// VIEWER_NO_EDGES: faces only, no wireframe and no markers.
pub fn no_edges() -> bool {
    env_flag("VIEWER_NO_EDGES", &NO_EDGES)
}

/// VIEWER_NO_DOTS: edges but no vertex markers.
pub fn no_dots() -> bool {
    env_flag("VIEWER_NO_DOTS", &NO_DOTS)
}

/// VIEWER_ALL_EDGES: keep the coplanar interior edges the wireframe normally culls.
pub fn all_edges() -> bool {
    env_flag("VIEWER_ALL_EDGES", &ALL_EDGES)
}
```

## Step 6 - Hand the ink lanes to the walk

A mesh reaches the arena AND the two ink tables, so `Walk` hands out the SOLID lane as a pair: the arena plus an `Ink` borrowing `seg` and `glyph`. Only the mesh arm changes.

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use mesh::{walk_mesh, MeshCx, MeshOpts};
```

**Add below it:**

```rust
use mesh_ink::Ink;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
pub mod mesh;
```

**Add below it:**

```rust
pub mod mesh_ink;
pub mod mesh_topology;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Self { arena: &mut t.arena, seg: &mut t.seg, glyph: &mut t.glyph }
    }
```

**Add below it:**

```rust

    /// The SOLID lane a mesh reaches: the arena for its faces and the ink pair.
    fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
        (self.arena, Ink { seg: self.seg, glyph: self.glyph })
    }
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::Mesh(m) => walk_mesh(w.arena, m, &MeshCx { cx, opts: &MeshOpts::OBJECT }),
```

**Replace with:**

```rust
        Geometry::Mesh(m) => {
            let (arena, mut ink) = w.solid();
            walk_mesh(arena, &mut ink, m, &MeshCx { cx, opts: &MeshOpts::OBJECT })
        }
```

## Step 7 - Hosts: the plate face a free outline lies on

An outline drawn on a plate is a polyline with no faces of its own: lifted as free ink it can climb through a thin plate and show from the other side. `Hosts` collects every distinct face plane of the file's meshes once per file (scans skipped, and nothing at all when the file has no polylines), and a polyline whose every point lies on one takes that face's normal as its `facing` and the plate's thickness as its lift budget.

_Type it._
**Create `src/app/walk/hosts.rs`**

```rust
//! The plate faces a free outline can lie on. A polyline drawn exactly on a mesh face (a
//! plate outline, a contact area) must lift off that face like the mesh's own wires and
//! never through the plate: it takes the face's normal and the mesh's thickness with it.

use std::collections::HashMap;
use session_rust::{Geometry, Mesh, Session};
use super::bounds::mesh_thickness;
use super::mesh::MESH_RAW_MIN;

/// A point is on a plane within this distance, local units (mm).
const ON_PLANE: f32 = 0.5;

/// One face plane of one mesh and that mesh's thickness.
struct HostPlane {
    n: [f32; 3],
    d: f32,
    thickness: f32,
}

/// What a hosted polyline inherits: the face normal and the host's thickness.
pub struct Host {
    pub normal: [f32; 3],
    pub thickness: f32,
}

/// Every distinct face plane of the file's meshes (huge meshes skipped: outlines lie on
/// plates, not on scans), so a polyline can find the face it lies on.
pub struct Hosts {
    planes: Vec<HostPlane>,
}

impl Hosts {
    /// No planes: nothing to host.
    pub fn empty() -> Self {
        Self { planes: Vec::new() }
    }

    /// The planes of a session's meshes, built only when the session has polylines to host.
    pub fn from_session(s: &Session) -> Self {
        let mut hosts = Self::empty();
        if s.objects.polylines.is_empty() {
            return hosts;
        }
        for g in s.order() {
            if let Some(Geometry::Mesh(m)) = s.lookup.get(&g) {
                hosts.add_mesh(m);
            }
        }
        hosts
    }

    /// One mesh's distinct face planes with its thickness.
    fn add_mesh(&mut self, m: &Mesh) {
        if m.number_of_faces() > MESH_RAW_MIN || m.number_of_faces() == 0 {
            return;
        }
        let keys = m.vertices();
        let mut slot: HashMap<usize, u32> = HashMap::with_capacity(keys.len());
        let mut pts: Vec<[f32; 3]> = Vec::with_capacity(keys.len());
        for (i, k) in keys.iter().enumerate() {
            let v = &m.vertex[k];
            slot.insert(*k, i as u32);
            pts.push([v.x as f32, v.y as f32, v.z as f32]);
        }
        let mut tris: Vec<u32> = Vec::new();
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for f in m.faces() {
            let Some(vs) = m.face_vertices(f) else { continue };
            let mut ids: Vec<u32> = Vec::with_capacity(vs.len());
            for k in vs {
                if let Some(i) = slot.get(k) {
                    ids.push(*i);
                }
            }
            for i in 2..ids.len() {
                tris.extend_from_slice(&[ids[0], ids[i - 1], ids[i]]);
            }
            faces.push(ids);
        }
        let thickness = mesh_thickness(&pts, &tris);
        let mut seen: HashMap<[i32; 4], ()> = HashMap::new();
        for ids in &faces {
            let Some((n, d)) = face_plane(&pts, ids) else { continue };
            let key = [(n[0] * 1000.0) as i32, (n[1] * 1000.0) as i32, (n[2] * 1000.0) as i32, (d * 2.0) as i32];
            if seen.insert(key, ()).is_none() {
                self.planes.push(HostPlane { n, d, thickness });
            }
        }
    }

    /// The plane every point of `pts` lies on, if there is one.
    pub fn find(&self, pts: &[[f32; 3]]) -> Option<Host> {
        if pts.len() < 2 {
            return None;
        }
        for p in &self.planes {
            let mut on = true;
            for q in pts {
                if (q[0] * p.n[0] + q[1] * p.n[1] + q[2] * p.n[2] - p.d).abs() > ON_PLANE {
                    on = false;
                    break;
                }
            }
            if on {
                return Some(Host { normal: p.n, thickness: p.thickness });
            }
        }
        None
    }
}

/// The unit normal and offset of a polygon's plane (Newell), or `None` when degenerate.
fn face_plane(pts: &[[f32; 3]], ids: &[u32]) -> Option<([f32; 3], f32)> {
    let mut n = [0.0f32; 3];
    for i in 0..ids.len() {
        let (a, b) = (pts[ids[i] as usize], pts[ids[(i + 1) % ids.len()] as usize]);
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len <= 0.0 {
        return None;
    }
    let n = [n[0] / len, n[1] / len, n[2] / len];
    let a = pts[ids[0] as usize];
    Some((n, a[0] * n[0] + a[1] * n[1] + a[2] * n[2]))
}
```

The walk context carries the file's hosts, which gives it a lifetime.

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use frames::{walk_obb, walk_plane};
```

**Add below it:**

```rust
use hosts::Hosts;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
pub mod frames;
```

**Add below it:**

```rust
pub mod hosts;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
pub struct WalkCx {
    pub vert_base: u32,
    pub row: u32,
}
```

**Replace with:**

```rust
pub struct WalkCx<'a> {
    pub vert_base: u32,
    pub row: u32,
    /// The file's plate faces, so a free outline lying on one inherits its normal and thickness.
    pub hosts: &'a Hosts,
}
```

_Type it._
**Find** in `src/app/walk/mesh.rs`:

```rust
    pub cx: &'a WalkCx,
```

**Replace with:**

```rust
    pub cx: &'a WalkCx<'a>,
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
use crate::app::walk::bounds::{file_extent, Baselines};
```

**Add below it:**

```rust
use crate::app::walk::hosts::Hosts;
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        let mut lap = Lap::start("walk");
```

**Add below it:**

```rust
        let hosts = Hosts::from_session(&session);
        lap.mark("hosts");
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
            let cx = WalkCx { vert_base: self.bases.vert, row };
```

**Replace with:**

```rust
            let cx = WalkCx { vert_base: self.bases.vert, row, hosts: &hosts };
```

The polyline producer asks for its host; every other producer keeps `FACING_UNKNOWN`.

_Type it._
**Find** in `src/app/walk/curves.rs`:

```rust
use super::encode::{encode_width, pack_rgba, Pen, FACING_UNKNOWN};
```

**Replace with:**

```rust
use super::encode::{encode_width, pack_facing, pack_rgba, Pen, FACING_UNKNOWN};
```

_Type it._
**Find** in `src/app/walk/curves.rs`:

```rust
/// One segment per span, straight from the flat coordinate array.
pub fn walk_polyline(seg: &mut SegRows, pl: &Polyline, cx: &WalkCx) -> Row {
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(pl.coords.len() / 3);
    for c in pl.coords.chunks_exact(3) {
        pts.push([c[0] as f32, c[1] as f32, c[2] as f32]);
    }
    let pen = Pen { row: cx.row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()) };
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    let thickness = polyline_thickness(&pts);
```

**Replace with:**

```rust
/// One segment per span, straight from the flat coordinate array. An outline lying on a
/// plate face takes that face's normal (it lifts off it like the plate's own wires) and the
/// plate's thickness (so it can never be lifted through it).
pub fn walk_polyline(seg: &mut SegRows, pl: &Polyline, cx: &WalkCx) -> Row {
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(pl.coords.len() / 3);
    for c in pl.coords.chunks_exact(3) {
        pts.push([c[0] as f32, c[1] as f32, c[2] as f32]);
    }
    let host = cx.hosts.find(&pts);
    let facing = match &host {
        Some(h) => pack_facing(Some(&[h.normal[0] as f64, h.normal[1] as f64, h.normal[2] as f64]), None),
        None => FACING_UNKNOWN,
    };
    let pen = Pen { row: cx.row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()), facing };
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    let thickness = match &host {
        Some(h) => h.thickness,
        None => polyline_thickness(&pts),
    };
```

## Step 8 - A template mesh and a masked colour write

A tube and a marker are one unit mesh drawn once per row, so `Template` uploads positions and indices once and binds them at vertex slot 0; `ColorWrite::Masked` is the depth-only prepass, and `PipelineDesc::bias` carries the hardware depth bias the arena sets in Step 14.

_Type it._
**Find** in `src/engine/gpu/buffers.rs`:

```rust
//! The GPU floor every lane stands on: `GpuCtx` (device + queue), `GrowBuf` (a table that
//! grows by appending, its live prefix copied GPU-side) and the two buffer helpers. No lane,
//! no shader and no per-frame state lives here.
```

**Replace with:**

```rust
//! The GPU floor every lane stands on: `GpuCtx` (device + queue), `GrowBuf` (a table that
//! grows by appending, its live prefix copied GPU-side), `Template` (a unit mesh drawn N
//! times) and the two buffer helpers. No lane, no shader and no per-frame state lives here.
```

_Type it._
**Find** in `src/engine/gpu/buffers.rs`:

```rust
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
```

**Add below it:**

```rust

/// A unit mesh drawn N times by an instanced lane (the cylinder, the marker quad).
pub struct Template {
    pub vbo: wgpu::Buffer,
    pub ibo: wgpu::Buffer,
    pub index_count: u32,
}

impl Template {
    /// Upload positions and indices once.
    pub fn new(ctx: &GpuCtx, label: &str, verts: &[[f32; 3]], idx: &[u32]) -> Self {
        let vbo = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label}.vbo")),
            contents: bytemuck::cast_slice(verts),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ibo = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label}.ibo")),
            contents: bytemuck::cast_slice(idx),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self { vbo, ibo, index_count: idx.len() as u32 }
    }

    /// Bind the template as vertex slot 0 and the index buffer.
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vbo.slice(..));
        pass.set_index_buffer(self.ibo.slice(..), wgpu::IndexFormat::Uint32);
    }
}
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
    Blended,
```

**Add below it:**

```rust
    /// Nothing: a depth-only prepass (the pass still has a colour attachment to declare).
    Masked,
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
            ColorWrite::Blended => (Some(wgpu::BlendState::ALPHA_BLENDING), wgpu::ColorWrites::ALL),
```

**Add below it:**

```rust
            ColorWrite::Masked => (None, wgpu::ColorWrites::empty()),
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub depth: DepthMode,
```

**Add below it:**

```rust
    /// Hardware depth bias (constant in format steps, slope in depth-per-pixel); faces use it
    /// to recede by about a pixel of their own slope so the ink drawn on them wins.
    pub bias: wgpu::DepthBiasState,
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
        Self { label: "", shader, vs: "vs_main", fs: "fs_main", groups, vertex_buffers, topology, color: ColorWrite::Opaque, depth: DepthMode::Opaque }
```

**Replace with:**

```rust
        Self { label: "", shader, vs: "vs_main", fs: "fs_main", groups, vertex_buffers, topology, color: ColorWrite::Opaque, depth: DepthMode::Opaque, bias: wgpu::DepthBiasState::default() }
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
        self.depth = depth;
        self
    }
```

**Add below it:**

```rust

    /// The same pipeline with a hardware depth bias.
    pub fn bias(mut self, bias: wgpu::DepthBiasState) -> Self {
        self.bias = bias;
        self
    }
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
    format: wgpu::VertexFormat::Uint32,
}];
```

**Add below it:**

```rust

const TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3,
}];
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
    wgpu::VertexBufferLayout { array_stride: 4, step_mode: wgpu::VertexStepMode::Vertex, attributes: &INSTANCE_ID_ATTRIBS }
}
```

**Add below it:**

```rust

/// A unit template's positions at `@location(0)` (the cylinder, the marker quad).
pub fn template_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout { array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex, attributes: &TEMPLATE_ATTRIBS }
}
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
/// of them: one colour target, `Depth32Float`, no cull, no depth bias, fill mode.
```

**Replace with:**

```rust
/// of them: one colour target, `Depth32Float`, no cull, the desc's depth bias, fill mode.
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
            bias: wgpu::DepthBiasState::default(),
```

**Replace with:**

```rust
            bias: desc.bias,
```

## Step 9 - The cylinder shader

Each pipe row places the unit cylinder along `p0 -> p1`, scales its radius to the pen and drops the whole segment when both of its faces turn away from the eye; a wire shorter than a few pen widths on screen thins, so a dense mesh does not fill with ink.

_Type it._
**Create `src/shaders/cylinder.wgsl`**

```wgsl
// Mesh edges as tubes: a unit cylinder instanced per segment. Group 3 = the segment table.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

struct CylinderSegment {
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,
    facing: u32,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
    feather: f32,
};

const FACING_UNKNOWN: u32 = 0xffffffffu;
const FLAG_INSIDE: u32 = 4u;

// Density taper: a tube thins when its projected length is under this many pen widths.
const WIRE_MIN_PENS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

// Octahedral 16-bit normal decode (signNotZero fold, matching the encoder).
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}

// An edge whose two faces both turn away from the eye is inside the solid: not drawn.
fn edge_faces_camera(seg: CylinderSegment, model: mat4x4<f32>, mid: vec3<f32>) -> bool {
    if (seg.facing == FACING_UNKNOWN) {
        return true;
    }
    let n0 = (model * vec4<f32>(oct16_decode(seg.facing & 0xffffu), 0.0)).xyz;
    let n1 = (model * vec4<f32>(oct16_decode(seg.facing >> 16u), 0.0)).xyz;
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - mid;
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}

// World radius that projects to `thickness` px, whatever the zoom.
fn screen_radius(clip_w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return line.thickness * line.ortho_h / line.vp_h;
    }
    return line.thickness * clip_w / (line.proj_y * line.vp_h);
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) inst_id: u32,
}

fn dead_vertex() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.inst_id = 0u;
    return dead;
}

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) si: u32) -> VsOut {
    let seg = segments[si];
    let inst = instances[seg.instance_id];
    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));

    let inside = (inst.flags & FLAG_INSIDE) != 0u;
    if (!inside && !edge_faces_camera(seg, inst.model, (w0 + w1) * 0.5)) {
        return dead_vertex();
    }

    // An orthonormal frame around the axis; the template's z runs p0 -> p1.
    let axis = w1 - w0;
    let len = length(axis);
    let dir = select(vec3<f32>(0.0, 0.0, 1.0), axis / len, len > 1e-9);
    let ref0 = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), abs(dir.z) > 0.9);
    let right = normalize(cross(ref0, dir));
    let up = cross(dir, right);
    let center = w0 + dir * (len * tmpl.z);
    let clip_c = mvp * vec4<f32>(center, 1.0);

    let r = select(screen_radius(clip_c.w), seg.radius, seg.radius > 0.0);
    var rt = r;
    if (seg.facing != FACING_UNKNOWN) {
        let ca = mvp * vec4<f32>(w0, 1.0);
        let cb = mvp * vec4<f32>(w1, 1.0);
        if (ca.w > 0.0 && cb.w > 0.0) {
            let vp = vec2<f32>(line.vp_w, line.vp_h);
            let sa = (ca.xy / ca.w * 0.5 + 0.5) * vp;
            let sb = (cb.xy / cb.w * 0.5 + 0.5) * vp;
            var px: f32;
            if (line.ortho_h > 0.0) {
                px = r * line.vp_h * 0.5 / line.ortho_h;
            } else {
                px = r * line.proj_y * line.vp_h * 0.5 / max(clip_c.w, 1e-6);
            }
            let room = WIRE_MIN_PENS * 2.0 * max(px, 1e-6);
            rt = r * clamp(length(sb - sa) / room, TAPER_MIN, 1.0);
        }
    }

    let world = center + (right * tmpl.x + up * tmpl.y) * rt;
    var o: VsOut;
    o.pos = mvp * vec4<f32>(world, 1.0);
    o.color = unpack4x8unorm(seg.color) * inst.color;
    o.inst_id = seg.instance_id;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return in.color;
}
```

## Step 10 - The segment lane draws pipes

The lane gains its second table, the cylinder pipeline and a depth-only ribbon pipeline: Tubes is one instanced draw over the template, Flat is the ribbon shader twice over the pipe table - prepass, then colour. `GreaterEqual` on the colour pass is what lets a wire sitting exactly at its faces' depth pass the test.

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
//! The segment lane: every straight piece of ink. One table of 40 B rows - ribbons
//! (line/polyline/curve, the FLAT lane, blended camera-facing quads). `SegRows` is one upload.
```

**Replace with:**

```rust
//! The segment lane: every straight piece of ink. Two tables of the same 40 B row - pipes
//! (mesh/BRep edges, the SOLID lane, tubes or flat quads with a depth prepass) and ribbons
//! (line/polyline/curve, the FLAT lane, blended camera-facing quads). `SegRows` is one upload.
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
use crate::engine::pipelines::{build, module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, index_buffer, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
```

**Replace with:**

```rust
use crate::engine::pipelines::{build, module, template_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, index_buffer, GpuCtx, GrowBuf, Template, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use super::view::LineStyle;
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];
```

**Replace with:**

```rust
pub const SHADERS: &[(&str, &str)] = &[("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")), ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// Sides of the unit cylinder: six is the fewest that reads as round at pen widths.
const CYL_SIDES: u32 = 6;
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
/// One segment row, 40 B, the layout ribbon.wgsl declares. The ends are
```

**Replace with:**

```rust
/// One segment row, 40 B, the layout cylinder.wgsl and ribbon.wgsl declare. The ends are
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
/// The shader module the lane's pipeline is built from.
struct SegShaders {
    ribbon: wgpu::ShaderModule,
}

/// The pipeline over the table: the blended, depth-read-only quad.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: the table, the ribbon's index pattern, the shader, the
/// pipeline.
pub struct SegmentLane {
    ribbons: SegTable,
    ribbon_ibo: wgpu::Buffer,
    shaders: SegShaders,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// A one-row table, the shader and the pipeline.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shaders = SegShaders {
            ribbon: module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);
        let ribbon_ibo = index_buffer(ctx, "ribbon.ibo", &RIBBON_INDICES);

        Self { ribbons: SegTable::new(ctx, l, "ribbons"), ribbon_ibo, shaders, gpu }
```

**Replace with:**

```rust
/// The two shader modules the lane's pipelines are built from.
struct SegShaders {
    cylinder: wgpu::ShaderModule,
    ribbon: wgpu::ShaderModule,
}

/// The pipelines over the two tables. `ribbon` serves both lanes' colour pass: the same
/// blended, depth-read-only quad.
struct SegPipelines {
    cylinder: wgpu::RenderPipeline,
    ribbon: wgpu::RenderPipeline,
    ribbon_depth: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: two tables, the unit cylinder, the ribbon's index pattern,
/// the shaders, the pipelines.
pub struct SegmentLane {
    pipes: SegTable,
    ribbons: SegTable,
    template: Template,
    ribbon_ibo: wgpu::Buffer,
    shaders: SegShaders,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// Two one-row tables, the unit cylinder, both shaders and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let template = Template::new(ctx, "cyl.template", &cyl_v, &cyl_i);
        let shaders = SegShaders {
            cylinder: module(&ctx.device, "cylinder.shader", include_str!("../../shaders/cylinder.wgsl")),
            ribbon: module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);
        let ribbon_ibo = index_buffer(ctx, "ribbon.ibo", &RIBBON_INDICES);

        Self { pipes: SegTable::new(ctx, l, "pipes"), ribbons: SegTable::new(ctx, l, "ribbons"), template, ribbon_ibo, shaders, gpu }
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
    /// Append one file's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        self.ribbons.append(ctx, l, &up.ribbons);
    }
```

**Replace with:**

```rust
    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        self.pipes.append(ctx, l, &up.pipes);
        self.ribbons.append(ctx, l, &up.ribbons);
    }

    /// The solid lane: mesh/BRep edges as tubes (1 draw) or as flat quads with a depth prepass
    /// (2 draws) - the prepass keeps the blended AA feather from depth-rejecting later strokes.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, style: LineStyle) -> u32 {
        match style {
            LineStyle::Tubes => self.draw_tubes(pass, b, &self.gpu.cylinder),
            LineStyle::Flat => self.draw_table(pass, b, &self.gpu.ribbon_depth, &self.pipes) + self.draw_table(pass, b, &self.gpu.ribbon, &self.pipes),
        }
    }
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
    }
```

**Add below it:**

```rust

    /// The pipes as instanced cylinders through `pipeline`; 0 draws when empty.
    fn draw_tubes(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.pipes.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.pipes.group, &[]);
        self.template.bind(pass);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.pipes.buf.len());
        1
    }
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
        self.ribbons.buf.reset();
```

**Add above it:**

```rust
        self.pipes.buf.reset();
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
    /// Hand the buffer back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.ribbons.release(ctx, l);
    }
```

**Replace with:**

```rust
    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.pipes.release(ctx, l);
        self.ribbons.release(ctx, l);
    }

    /// Solid-lane rows on the GPU - the MSAA policy reads it.
    pub fn pipe_count(&self) -> u32 {
        self.pipes.buf.len()
    }
```

The unit cylinder is a ring of `CYL_SIDES` quads with a fan at each end; the shader rescales its xy by the radius and maps z along the segment.

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
/// Every segment pipeline for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &SegShaders, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let quad = PipelineDesc::new(&s.ribbon, &groups, &[], TriangleList);
    let dev = &ctx.device;

    SegPipelines {
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
    }
}
```

**Replace with:**

```rust
/// Every segment pipeline for `target`. `GreaterEqual` on the ribbon is load-bearing: a mesh
/// edge sits EXACTLY on its faces' depth, and strict `Greater` shreds it.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &SegShaders, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let template = [template_layout()];
    let quad = PipelineDesc::new(&s.ribbon, &groups, &[], TriangleList);
    let tube = PipelineDesc::new(&s.cylinder, &groups, &template, TriangleList);
    let dev = &ctx.device;

    SegPipelines {
        cylinder: build(dev, target, &tube.with("cylinder", "fs_main")),
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
        ribbon_depth: build(dev, target, &quad.with("ribbon.depth", "fs_depth").color(ColorWrite::Masked)),
    }
}

/// Unit-cylinder template along +Z, radius 1, z in [0, 1], with cap fans. The shader rescales
/// xy by the pen radius and maps z along (p1 - p0).
fn unit_cylinder(sides: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    for s in 0..sides {
        let a = s as f32 / sides as f32 * std::f32::consts::TAU;
        v.push([a.cos(), a.sin(), 0.0]);
        v.push([a.cos(), a.sin(), 1.0]);
    }
    for s in 0..sides {
        let b0 = 2 * s;
        let b1 = 2 * ((s + 1) % sides);
        idx.extend_from_slice(&[b0, b1, b1 + 1, b0, b1 + 1, b0 + 1]);
    }
    let cb = v.len() as u32;
    v.push([0.0, 0.0, 0.0]);
    let ct = v.len() as u32;
    v.push([0.0, 0.0, 1.0]);
    for s in 0..sides {
        let b0 = 2 * s;
        let b1 = 2 * ((s + 1) % sides);
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]);
    }
    (v, idx)
}
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
    /// ribbon.wgsl reads the 40 B segment row (ends as scalars).
```

**Replace with:**

```rust
    /// cylinder.wgsl and ribbon.wgsl read the same 40 B segment row (ends as scalars).
```

## Step 11 - The sphere shader

A marker is the quad template trimmed to a disc by the fragment, lifted a hair more than the wires so it wins the tie at the vertex it marks, and dropped when every one of its faces turns away; `fs_depth` is its prepass entry. The bulk repeats glyph.wgsl - `faces_front`, the template corner and the lift are the parts worth reading.

_Paste it._
**Create `src/shaders/sphere.wgsl`**

```wgsl
// Mesh vertex markers: a camera-facing quad template per glyph, trimmed to a disc by the
// fragment SDF, hidden when every incident face turns away. Group 3 = the glyph table.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

struct GlyphPoint {
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
    facing: u32,
    facing_ext: vec2<u32>,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
    feather: f32,
};

const FACING_UNKNOWN: u32 = 0xffffffffu;
const FLAG_INSIDE: u32 = 4u;
const MM_TO_M: f32 = 0.001;

// Half a width more than the wires: the disc must win the tie at the vertex it marks.
const LIFT_HAIR_PX: f32 = 0.5;
const LIFT_MAX_MM: f32 = 0.5;
const LIFT_MAX_THICK: f32 = 0.25;

// A marker thins when the object's vertex spacing is under this many marker diameters.
const MARKER_MIN_DIAMS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

fn lift_capped(lift: f32, w: f32, thickness: f32) -> f32 {
    var cap_mm = LIFT_MAX_MM;
    if (thickness > 0.0) {
        cap_mm = min(cap_mm, LIFT_MAX_THICK * thickness);
    }
    let max_lift = cap_mm * MM_TO_M / max(w, 1e-9);
    return clamp(min(lift, max_lift), 0.0, 0.5);
}

fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}

fn screen_radius(clip_w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return line.thickness * line.ortho_h / line.vp_h;
    }
    return line.thickness * clip_w / (line.proj_y * line.vp_h);
}

// A world length in px at eye depth `w`.
fn to_px(world: f32, w: f32) -> f32 {
    if (line.ortho_h > 0.0) {
        return world * line.vp_h * 0.5 / line.ortho_h;
    }
    return world * line.proj_y * line.vp_h * 0.5 / max(w, 1e-6);
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) @interpolate(flat) px: f32,
    @location(3) @interpolate(flat) inst_id: u32,
};

fn dead_dot() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.corner = vec2<f32>(0.0);
    dead.px = 0.0;
    dead.inst_id = 0u;
    return dead;
}

// Whether any of the vertex's incident faces turns toward the eye; `known` = it has any.
fn faces_front(g: GlyphPoint, model: mat4x4<f32>, to_eye: vec3<f32>) -> vec2<bool> {
    let fwords = array<u32, 3>(g.facing, g.facing_ext.x, g.facing_ext.y);
    var known = false;
    var front = false;
    for (var w = 0u; w < 3u; w = w + 1u) {
        let fw = fwords[w];
        if (fw == FACING_UNKNOWN) {
            continue;
        }
        known = true;
        for (var h = 0u; h < 2u; h = h + 1u) {
            let n = (model * vec4<f32>(oct16_decode((fw >> (16u * h)) & 0xffffu), 0.0)).xyz;
            if (dot(n, to_eye) > 0.0) {
                front = true;
            }
        }
    }
    return vec2<bool>(known, front);
}

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) gi: u32) -> VsOut {
    let g = glyphs[gi];
    let inst = instances[g.instance_id];
    let centre = place(g.instance_id, g.center);
    let clip = mvp * vec4<f32>(centre, 1.0);
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    let r = select(screen_radius(clip.w), g.radius, g.radius > 0.0);
    var px = to_px(r, clip.w);
    if (inst.spacing > 0.0) {
        let sp_px = to_px(inst.spacing, clip.w);
        px = px * clamp(sp_px / max(MARKER_MIN_DIAMS * 2.0 * px, 1e-6), TAPER_MIN, 1.0);
    }
    if (px > max(line.vp_w, line.vp_h)) {
        return dead_dot();
    }
    px = max(px, 0.5);

    // Lift: in w for perspective, in ndc z for ortho, both capped by the thickness.
    let ozn = select(0.0, length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z)), line.ortho_h > 0.0);
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - centre;
    let lift = LIFT_HAIR_PX * 2.0 * MM_TO_M / (line.proj_y * line.vp_h);
    var wn = clip.w * (1.0 - lift_capped(lift, clip.w, inst.thickness));
    var zlift = 0.0;
    if (line.ortho_h > 0.0) {
        wn = clip.w;
        let lw = LIFT_HAIR_PX * 2.0 * line.ortho_h / line.vp_h;
        zlift = min(lw, select(LIFT_MAX_MM, min(LIFT_MAX_MM, LIFT_MAX_THICK * inst.thickness), inst.thickness > 0.0)) * ozn;
    }
    let off = tmpl.xy * (px + 0.5 * line.feather) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * wn;

    // Hidden vertices never reach the rasterizer, unless the eye is inside the object.
    let inside = (inst.flags & FLAG_INSIDE) != 0u;
    let kf = faces_front(g, inst.model, to_eye);
    if (kf.x && !kf.y && !inside) {
        return dead_dot();
    }

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy / clip.w * wn + off, clip.z + zlift * wn, wn);
    o.color = g.color * inst.color;
    o.corner = tmpl.xy;
    o.px = px;
    o.inst_id = g.instance_id;
    return o;
}

fn coverage(in: VsOut) -> f32 {
    let d = length(in.corner) * (in.px + 0.5 * line.feather);
    return clamp((in.px + 0.5 * line.feather - d) / line.feather, 0.0, 1.0);
}

@fragment
fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec4<f32>(0.0);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let alpha = coverage(in);
    if (alpha <= 0.0) {
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

The dot shader exposes the same depth-only entry, so both glyph shaders offer one contract to the lane.

_Type it._
**Find** in `src/shaders/glyph.wgsl`:

```wgsl
    return clamp((in.px + 0.5 * line.feather - d) / line.feather, 0.0, 1.0) * in.fade;
}
```

**Add below it:**

```wgsl

@fragment
fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec4<f32>(0.0);
}
```

## Step 12 - The glyph lane draws markers

The mirror of Step 10: a `spheres` table, the quad template, the `sphere` and `sphere_depth` pipelines, and `draw_spheres` = prepass then colour, drawn last of the solid lane so a tie with a wire's cap goes to the marker.

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
//! The glyph lane: every vertex-sized piece of ink. One table of 48 B rows -
//! dots (free points, the FLAT lane, three verts per dot). `GlyphRows` is one upload.
```

**Replace with:**

```rust
//! The glyph lane: every vertex-sized piece of ink. Two tables of the same 48 B row - spheres
//! (mesh/BRep vertex markers, the SOLID lane, on a quad template with a depth prepass) and
//! dots (free points, the FLAT lane, three verts per dot). `GlyphRows` is one upload.
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
use crate::engine::pipelines::{build, module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
```

**Replace with:**

```rust
use crate::engine::pipelines::{build, module, template_layout, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, Template, ROWS};
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
pub const SHADERS: &[(&str, &str)] = &[("glyph.wgsl", include_str!("../../shaders/glyph.wgsl"))];
```

**Replace with:**

```rust
pub const SHADERS: &[(&str, &str)] = &[("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")), ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl"))];
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
/// One dot row, 48 B, the layout glyph.wgsl declares.
```

**Replace with:**

```rust
/// One marker or dot row, 48 B, the layout sphere.wgsl and glyph.wgsl declare.
```

_Paste it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
/// The shader module the lane's pipeline is built from.
struct GlyphShaders {
    dot: wgpu::ShaderModule,
}

/// The pipeline over the table.
struct GlyphPipelines {
    dot: wgpu::RenderPipeline,
}

/// The glyph lane on the GPU: the table, the shader, the pipeline.
pub struct GlyphLane {
    dots: GlyphTable,
    shaders: GlyphShaders,
    gpu: GlyphPipelines,
}

impl GlyphLane {
    /// A one-row table, the shader and the pipeline.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shaders = GlyphShaders {
            dot: module(&ctx.device, "glyph.shader", include_str!("../../shaders/glyph.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        Self { dots: GlyphTable::new(ctx, l, "dots"), shaders, gpu }
```

**Replace with:**

```rust
/// The two shader modules the lane's pipelines are built from.
struct GlyphShaders {
    sphere: wgpu::ShaderModule,
    dot: wgpu::ShaderModule,
}

/// The pipelines over the two tables.
struct GlyphPipelines {
    sphere: wgpu::RenderPipeline,
    sphere_depth: wgpu::RenderPipeline,
    dot: wgpu::RenderPipeline,
}

/// The glyph lane on the GPU: two tables, the marker quad, the shaders, the pipelines.
pub struct GlyphLane {
    spheres: GlyphTable,
    dots: GlyphTable,
    template: Template,
    shaders: GlyphShaders,
    gpu: GlyphPipelines,
}

impl GlyphLane {
    /// Two one-row tables, the marker quad, both shaders and the pipelines.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let (q_v, q_i) = unit_quad();
        let template = Template::new(ctx, "quad.template", &q_v, &q_i);
        let shaders = GlyphShaders {
            sphere: module(&ctx.device, "sphere.shader", include_str!("../../shaders/sphere.wgsl")),
            dot: module(&ctx.device, "glyph.shader", include_str!("../../shaders/glyph.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        Self { spheres: GlyphTable::new(ctx, l, "spheres"), dots: GlyphTable::new(ctx, l, "dots"), template, shaders, gpu }
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    /// Append one file's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        self.dots.append(ctx, l, &up.dots);
    }
```

**Replace with:**

```rust
    /// Append one file's rows to both tables.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        self.spheres.append(ctx, l, &up.spheres);
        self.dots.append(ctx, l, &up.dots);
    }

    /// Vertex markers, drawn LAST of the solid lane so a tie with a band cap goes to the marker:
    /// depth prepass then the blended colour pass, 2 draws.
    pub fn draw_spheres(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_markers(pass, b, &self.gpu.sphere_depth) + self.draw_markers(pass, b, &self.gpu.sphere)
    }
```

_Paste it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
        self.draw_dot_table(pass, b, &self.gpu.dot)
    }
```

**Add below it:**

```rust

    /// The marker table on the quad template through `pipeline`; 0 draws when empty.
    fn draw_markers(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.spheres.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.spheres.group, &[]);
        self.template.bind(pass);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.spheres.buf.len());
        1
    }
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
        self.dots.buf.reset();
```

**Add above it:**

```rust
        self.spheres.buf.reset();
```

_Paste it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    /// Hand the buffer back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.dots.release(ctx, l);
    }
```

**Replace with:**

```rust
    /// Hand both buffers back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.release(ctx, l);
        self.dots.release(ctx, l);
    }

    /// Solid-lane rows on the GPU - the MSAA policy reads it.
    pub fn sphere_count(&self) -> u32 {
        self.spheres.buf.len()
    }
```

_Paste it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
/// Every glyph pipeline for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &GlyphShaders, target: Target) -> GlyphPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let disc = PipelineDesc::new(&s.dot, &groups, &[], TriangleList);
    let dev = &ctx.device;

    GlyphPipelines {
        dot: build(dev, target, &disc.with("glyph", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
    }
}
```

**Replace with:**

```rust
/// Every glyph pipeline for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &GlyphShaders, target: Target) -> GlyphPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let template = [template_layout()];
    let marker = PipelineDesc::new(&s.sphere, &groups, &template, TriangleList);
    let disc = PipelineDesc::new(&s.dot, &groups, &[], TriangleList);
    let dev = &ctx.device;

    GlyphPipelines {
        sphere: build(dev, target, &marker.with("sphere", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
        sphere_depth: build(dev, target, &marker.with("sphere.depth", "fs_depth").color(ColorWrite::Masked)),
        dot: build(dev, target, &disc.with("glyph", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
    }
}

/// Camera-facing quad template for the markers; the fragment trims it to a circle.
fn unit_quad() -> (Vec<[f32; 3]>, Vec<u32>) {
    let v = vec![[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [1.0, 1.0, 0.0], [-1.0, 1.0, 0.0]];
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    /// glyph.wgsl reads the 48 B glyph row.
```

**Replace with:**

```rust
    /// sphere.wgsl and glyph.wgsl read the same 48 B glyph row.
```

## Step 13 - Ink drawn in its face

A ribbon that knows its faces no longer lifts to win: each side corner takes the depth of its face plane at that pixel (the deeper of the two at a crease), so the stroke lies in the surface and can only be covered by what covers the surface. The same shader now culls an edge whose faces both turn away, tapers crowded wires, never fades a solid wire, and offers `fs_depth` for the prepass.

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
// buffer), a capsule SDF in the fragment. Draws the ribbon table (free linework). Group 3 =
// the segment table.
```

**Replace with:**

```wgsl
// buffer), a capsule SDF in the fragment. Draws the ribbon table (free linework) and, with
// a depth prepass, the pipe table (mesh edges). Group 3 = the segment table.
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
const MM_TO_M: f32 = 0.001;
```

**Add above it:**

```wgsl
const FACING_UNKNOWN: u32 = 0xffffffffu;
const FLAG_INSIDE: u32 = 4u;
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
// Faces never move (arena.rs). A segment lifts a hair to win ties, capped by LIFT_MAX_THICK
// of its object's thickness and by LIFT_MAX_MM outright, so even far away it cannot cross the
// millimetres of a joint.
const LIFT_HAIR_PX: f32 = 0.25;
const LIFT_MAX_THICK: f32 = 0.25;
const LIFT_MAX_MM: f32 = 0.5;
```

**Replace with:**

```wgsl
// Faces never move (arena.rs). A segment that knows its faces is drawn IN them: each ribbon
// corner takes the depth of the face plane at its own pixel (the deeper of the two planes at
// a crease), so the ink lies on the surface and can never be in front of anything that covers
// that surface. A segment that knows no face lifts a hair to win ties, capped by
// LIFT_MAX_THICK of its object's thickness and by LIFT_MAX_MM outright, so even far away it
// cannot cross the millimetres of a joint.
const LIFT_HAIR_PX: f32 = 0.25;
const LIFT_MAX_THICK: f32 = 0.25;
const LIFT_MAX_MM: f32 = 0.5;
// A face seen edge-on has an unbounded depth slope; the corner step stops at this many
// half-widths of depth per half-width of screen.
const PLANE_MAX_SLOPE: f32 = 20.0;

// Density taper: a wire thins when shorter than this many pen widths; never below TAPER_MIN.
const WIRE_MIN_PENS: f32 = 3.0;
const TAPER_MIN: f32 = 0.15;
```

The plane step: the eye's axes read off the mvp, the face normal in eye space, and the depth the plane gains `off_px` pixels away from the edge across the ribbon, clamped for a face seen edge-on.

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}
```

**Add below it:**

```wgsl

fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(f32(i32(p << 24u) >> 24u) / 127.0, f32(i32(p << 16u) >> 24u) / 127.0);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }
    return normalize(n);
}

// An edge whose two faces both turn away from the eye is inside the solid: not drawn.
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN) {
        return true;
    }
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}

// The eye's axes in world space, read off the mvp: right and up from its x and y rows,
// forward from its w row (perspective) or its z row (ortho). Columns: right, up, forward.
fn eye_axes() -> mat3x3<f32> {
    let right = normalize(vec3<f32>(mvp[0].x, mvp[1].x, mvp[2].x));
    let up = normalize(vec3<f32>(mvp[0].y, mvp[1].y, mvp[2].y));
    let fwd_p = vec3<f32>(mvp[0].w, mvp[1].w, mvp[2].w);
    let fwd_o = vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z);
    let fwd = normalize(select(fwd_p, fwd_o, line.ortho_h > 0.0));
    return mat3x3<f32>(right, up, fwd);
}

// How much deeper (mm, negative = nearer) the plane with world normal `nw` is at a point
// `off_px` pixels from the segment along screen direction `n2` than at the segment: the
// plane's depth slope across the ribbon. `mmpp` is the world size of one pixel there.
fn plane_step_mm(nw: vec3<f32>, n2: vec2<f32>, off_px: f32, mmpp: f32) -> f32 {
    let a = eye_axes();
    let ne = vec3<f32>(dot(nw, a[0]), dot(nw, a[1]), dot(nw, a[2]));
    let nz = select(ne.z, select(0.05, -0.05, ne.z < 0.0), abs(ne.z) < 0.05);
    let step = -(ne.x * n2.x + ne.y * n2.y) * off_px * mmpp / nz;
    let bound = off_px * mmpp * PLANE_MAX_SLOPE;
    return clamp(step, -bound, bound);
}

// The corner's depth step for a segment with faces `n0`/`n1`: the deeper of the two planes,
// so at a crease the ribbon folds onto both faces and never floats in front of either.
fn corner_step_mm(n0: vec3<f32>, n1: vec3<f32>, n2: vec2<f32>, off_px: f32, mmpp: f32) -> f32 {
    return max(plane_step_mm(n0, n2, off_px, mmpp), plane_step_mm(n1, n2, off_px, mmpp));
}
```

The vertex carries a `solid` flag to the fragment, and the cull runs before any projection.

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    @location(5) @interpolate(flat) hw1: f32,
```

**Add below it:**

```wgsl
    @location(6) @interpolate(flat) solid: f32,
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
// triangles disagree along the diagonal.
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), hairline_fade(raw));
}
```

**Replace with:**

```wgsl
// triangles disagree along the diagonal. Solid-lane wires never fade: they blend under a
// depth write and half-alpha strokes resolve by draw-order luck.
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), select(hairline_fade(raw), 1.0, in.solid > 0.5));
}

fn density_taper(solid: bool, len_px: f32, px: f32) -> f32 {
    if (!solid) {
        return 1.0;
    }
    let room = WIRE_MIN_PENS * 2.0 * max(px, 1e-6);
    return clamp(len_px / room, TAPER_MIN, 1.0);
}
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    dead.hw1 = 0.0;
```

**Add below it:**

```wgsl
    dead.solid = 0.0;
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    let inst = instances[seg.instance_id];

    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
```

**Replace with:**

```wgsl
    let inst = instances[seg.instance_id];
    let model = inst.model;

    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));
    let mid = (w0 + w1) * 0.5;
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - mid;
    let n0 = (model * vec4<f32>(oct16_decode(seg.facing & 0xffffu), 0.0)).xyz;
    let n1 = (model * vec4<f32>(oct16_decode(seg.facing >> 16u), 0.0)).xyz;

    // Hidden edges never reach the rasterizer, unless the eye is inside the object.
    let inside = (inst.flags & FLAG_INSIDE) != 0u;
    if (!inside && !edge_faces_camera(seg.facing, n0, n1, to_eye)) {
        return dead_vertex();
    }
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    let px = floor_hairline(select(raw0, raw1, at_end1));
```

**Add below it:**

```wgsl
    let solid = seg.facing != FACING_UNKNOWN && (seg.facing & 0xffffu) != (seg.facing >> 16u);
    let crowd = density_taper(solid, len, px);
```

Depth: a segment with faces takes the corner step on top of the hair (its flanks lie exactly in their planes and would otherwise tie on rounding alone); one without keeps lesson 4's lift.

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    let thick = inst.thickness;
    var wn = e.w;
    var zn = e.z;
    if (line.ortho_h > 0.0) {
        zn = e.z + ortho_lift_ndc(LIFT_HAIR_PX, thick);
```

**Replace with:**

```wgsl
    // Depth: a segment with faces is drawn in them (each side corner at its plane's depth at
    // that pixel, the centre at the edge's own); one without lifts a hair.
    let thick = inst.thickness;
    var wn = e.w;
    var zn = e.z;
    if (seg.facing != FACING_UNKNOWN) {
        // Every lane takes the hair too: the flanks lie exactly in their planes and would
        // otherwise tie with the faces on rounding alone.
        let mmpp = select(2.0 * e.w / (line.proj_y * line.vp_h), 2.0 * line.ortho_h / line.vp_h, line.ortho_h > 0.0);
        let step = corner_step_mm(n0, n1, n * side, off, mmpp) * abs(side);
        if (line.ortho_h > 0.0) {
            zn = e.z + ortho_lift_ndc(LIFT_HAIR_PX, thick) - step * ndc_z_per_world();
        } else {
            wn = max(lifted_w(LIFT_HAIR_PX, e, thick) + step * MM_TO_M, e.w * 0.5);
            zn = e.z / wn;
        }
    } else if (line.ortho_h > 0.0) {
        zn = e.z + ortho_lift_ndc(LIFT_HAIR_PX, thick);
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    o.hw0 = raw0;
    o.hw1 = raw1;
```

**Replace with:**

```wgsl
    o.hw0 = raw0 * crowd;
    o.hw1 = raw1 * crowd;
    o.solid = select(0.0, 1.0, solid);
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    return clamp((hf.x + 0.5 * line.feather - d) / line.feather, 0.0, 1.0) * hf.y;
}
```

**Add below it:**

```wgsl

// Depth-only prepass: binary at half coverage, colour masked by the pipeline.
@fragment
fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec4<f32>(0.0);
}
```

## Step 14 - Faces recede two format steps

Faces do not move: a push of any size - a fraction of eye depth, of the object's thickness, of the face's slope per pixel - brought ink through whatever sat closer behind the face. Two format steps of constant bias only break the exact tie with ink drawn on the face's own vertices; the ink lifts what it needs itself.

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
use wgpu::PrimitiveTopology::TriangleList;
```

**Add below it:**

```rust

/// Faces do NOT recede: a push of any size - a fraction of eye depth, of the object's own
/// thickness, or of the face's slope per pixel - brought ink through whatever sat closer
/// behind the face (3 mm joinery contacts, thin plates far away). Two format steps (reverse-Z:
/// negative = farther) only break the exact tie with ink drawn on the face's own vertices;
/// the ink lifts what it needs instead (ribbon.wgsl `lift_need_px`).
const FACE_BIAS: wgpu::DepthBiasState = wgpu::DepthBiasState { constant: -2, slope_scale: 0.0, clamp: 0.0 };
```

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
        faces: build(dev, target, &base.with("triangle", "fs_main")),
```

**Replace with:**

```rust
        faces: build(dev, target, &base.with("triangle", "fs_main").bias(FACE_BIAS)),
```

## Step 15 - LineStyle and the keys E and L

`View` learns the solid lane's three knobs - show it, show its markers, draw it as tubes or flat - read once at startup (`?style=tubes`, `?nomarkers=1`; `VIEWER_LINE_STYLE`, `BENCH_NO_MARKERS` natively) and flipped by `E` and `L` afterwards.

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
//! `View` - the runtime knobs a frame reads: what to show, the
//! pen weight. Read ONCE at startup from the query string
//! (wasm) or the environment (native); the key handlers flip them afterwards. No GPU here.
```

**Replace with:**

```rust
//! `View` - the runtime knobs a frame reads: what to show, how the solid ink is drawn, the
//! pen weight. Read ONCE at startup from the query string
//! (wasm) or the environment (native); the key handlers flip them afterwards. No GPU here.

/// How the SOLID lane draws mesh/BRep edges. Both read the same segment table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: the radius lifts the ink off the surface it decorates.
    Tubes,
    /// A camera-facing quad per edge through the flat lane's shader. Cheaper.
    Flat,
}
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
    pub show_lines: bool,
```

**Add below it:**

```rust
    /// Mesh/BRep edges and their vertex markers - the SOLID lane. `E`.
    pub show_mesh_edges: bool,
    /// Vertex markers on top of the solid ink; `BENCH_NO_MARKERS` turns them off for timing.
    pub markers: bool,
    /// Solid-lane style; `VIEWER_LINE_STYLE=tubes` picks Tubes at startup. `L`.
    pub line_style: LineStyle,
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
    pub fn from_env() -> Self {
        Self {
            show_points: true,
            show_lines: true,
```

**Replace with:**

```rust
    pub fn from_env() -> Self {
        let tubes = knob("VIEWER_LINE_STYLE", "style").map(|v| v.eq_ignore_ascii_case("tubes")).unwrap_or(false);

        Self {
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            markers: knob("BENCH_NO_MARKERS", "nomarkers").is_none(),
            line_style: if tubes { LineStyle::Tubes } else { LineStyle::Flat },
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
            msaa_forced: knob("VIEWER_MSAA", "msaa").and_then(|v| v.parse().ok()),
        }
    }
```

**Add below it:**

```rust

    /// Flip the solid-lane style.
    pub fn toggle_line_style(&mut self) {
        self.line_style = match self.line_style {
            LineStyle::Tubes => LineStyle::Flat,
            LineStyle::Flat => LineStyle::Tubes,
        };
    }
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
//! 1-7 named views, Space projection, C reset, F fit, Q/W lane toggles.
```

**Replace with:**

```rust
//! 1-7 named views, Space projection, C reset, F fit, Q/W/E lane toggles, L line style.
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
            Key::Character("w" | "W") => state.gpu.view.show_lines = !state.gpu.view.show_lines,
```

**Add below it:**

```rust
            Key::Character("e" | "E") => state.gpu.view.show_mesh_edges = !state.gpu.view.show_mesh_edges,
            Key::Character("l" | "L") => state.gpu.view.toggle_line_style(),
```

## Step 16 - Two more entries in the frame list

Mesh edges and vertex markers draw right after the faces and before the flat lanes because they write depth; the scene log counts them, and the MSAA policy calls a frame solid when it has faces, pipes or spheres.

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
    /// 1 background · 2 grid · 3 faces · 4 lines · 5 point dots. Lines write no depth: two lines on one
```

**Replace with:**

```rust
    /// 1 background · 2 grid · 3 faces · 4 mesh edges · 5 vertex
    /// markers · 6 lines · 7 point dots. Lines write no depth: two lines on one
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
        draws += self.arena.draw_faces(pass, b);
```

**Add below it:**

```rust
        if v.show_mesh_edges {
            draws += self.segments.draw_pipes(pass, b, v.line_style);
        }
        if v.show_mesh_edges && v.markers {
            draws += self.glyphs.draw_spheres(pass, b);
        }
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            "scene: {} objects, {} verts, {} ribbons, {} dots",
            self.objects.len(), self.arena.vert_count(), self.segments.ribbon_count(), self.glyphs.dot_count()
```

**Replace with:**

```rust
            "scene: {} objects, {} verts, {} pipes, {} ribbons, {} markers, {} dots",
            self.objects.len(), self.arena.vert_count(), self.segments.pipe_count(), self.segments.ribbon_count(),
            self.glyphs.sphere_count(), self.glyphs.dot_count()
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// The sample count for what is ON the GPU now: 4x only with solid geometry (faces) and a
    /// canvas MSAA can afford.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.face_count() > 0;
```

**Replace with:**

```rust
    /// The sample count for what is ON the GPU now: 4x only with solid geometry (faces,
    /// pipes, spheres) and a canvas MSAA can afford.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.face_count() > 0 || self.segments.pipe_count() > 0 || self.glyphs.sphere_count() > 0;
```

_Type it._
**Find** in `src/engine/gpu/targets.rs`:

```rust
    /// The sample count a frame gets: 4x only when SOLID geometry (faces) is on
```

**Replace with:**

```rust
    /// The sample count a frame gets: 4x only when SOLID geometry (faces, pipes, spheres) is on
```

## Run

```bash
trunk serve
```

- Open http://127.0.0.1:8770/ - every mesh in the local scene wears a black wireframe and a dark dot on each vertex, and the scene line in the console counts pipes and markers next to the ribbons and dots.
- `E` hides the mesh ink, `L` switches the flat quads for tubes, `?style=tubes` starts in tubes and `?nomarkers=1` leaves the markers out; `docs/_gate.sh` (runnable once lesson 11 adds the harness) renders a plate from above and fails on any pixel of its bottom outline.

## Why

- Faces never move: every push that was tried - a fraction of eye depth, of the object's thickness, of the face's slope per pixel - brought ink through whatever sat closer behind the face, so `FACE_BIAS` is two format steps, enough to break the exact tie with ink on the face's own vertices and nothing more.
- Ink that knows its faces is drawn in them: `corner_step_mm` puts each ribbon corner at its face plane's depth at that pixel, so the stroke can only be hidden by what hides the surface, and there is no lift left to leak through a joint.
- Ink that knows no faces keeps lesson 4's hair lift, capped by its object's thickness; `Hosts` turns a plate's outline from the second kind into the first, which is why the gate's bottom outline stays hidden from above.
- One row type, two tables: pipes and ribbons are the same 40 B `CylinderSegment`, so Flat reuses the ribbon shader with a prepass and Tubes adds only a template and a shader; `L` flips at draw time with nothing re-uploaded.
- The solid lane takes a depth prepass (`ColorWrite::Masked`, `fs_depth` at half coverage) because its strokes write depth: without it the blended feather of one stroke would depth-reject the stroke behind it; the flat lane still skips the prepass and resolves by order.
- The facing word is two oct16 normals because the shaders only ever ask for the SIGN of a dot: a back edge of a closed mesh never reaches the rasterizer, and `FLAG_INSIDE` (the eye within the object's box) switches the cull off.
- The topology is one fused pass with an intrusive edge chain per vertex: the kernel's separate passes and their hash tables were the walk's cost on a big mesh, and the same pass yields the normals the facing word needs.
- Gates before beauty: past `MESH_RAW_MIN` triangles a mesh is faces only, exactly coplanar neighbours hide their shared diagonal, and a wireframe past `WIREFRAME_BLACK_MIN` edges is black - on a scan the decoration outweighs the geometry.
