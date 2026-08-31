# 51 Five types, one body

> The last lesson of the block. After it, `scene.rs` is 284 lines and every geometry type has a
> file; lessons 52-114 are written against these paths.
> Nothing you can see changes.
> Answer key: `git diff end-of-50..end-of-51 -- session_viewer/src`.
>
> **Lessons 45-51 move code.** This one breaks that rule once, deliberately, and §4.3 says why.

## 1. Why this seam

### 1a. The evidence

```bash
awk '/^pub\(crate\) fn push_mesh\(/,/^}$/' src/app/scene.rs | wc -l
awk '/^pub\(crate\) fn push_mesh\(/,/\) -> /' src/app/scene.rs | grep -c ':'
grep -c 'push_mesh(' src/app/walk/mod.rs
grep -c 'let (b*, _) = push_mesh' src/app/walk/mod.rs
```

```text
314  lines in one function
  8  parameters
  5  callers
  4  of them discard its second return value
```

`push_mesh` returns `(bounds, closed)`. Four of five callers write `let (b, _) =`.

That discard is not a style problem. `closed` is what sets `FLAG_OPEN`, and `FLAG_OPEN` is what
tells the edge lanes to stop culling by facing — without it, an open mesh has its wireframe
stripped off interior surface you can see straight through. `Element(Mesh)` is missing that flag
today, and nobody decided it should be: the arm was copied from the Mesh arm with the block
deleted, and the tuple made the omission invisible.

### 1b. The law

**W4 — an option a caller must decide is a NAMED FIELD, never a positional argument and never a
value the caller may drop.**

### 1c. The rejected alternative

Have `brep.rs` copy the mesh body and change the two lines it needs. Do not. BRep and
NurbsSurface differ from Mesh by **one line each** — a colour source — and by two decisions.
Three copies of a 314-line body drift within a lesson or two of each other.

## 2. Where the code lives after this lesson

| symbol | new home | lines |
|---|---|---|
| `MeshTopo`, `mesh_topology`, `face_normal_raw`, `COPLANAR_DOT` | `walk/mesh_topology.rs` | 138 |
| `push_mesh`'s faces half + the three gates + `MESH_RAW_MIN` | `walk/mesh.rs` | 130 |
| `push_mesh`'s ink half + `WIREFRAME_BLACK_MIN` | `walk/mesh_ink.rs` | 290 |
| the BRep adapter | `walk/brep.rs` | **22** |
| the NurbsSurface adapter | `walk/surface.rs` | **18** |

The split point is the three gates. A dense mesh, a print fill and `VIEWER_NO_EDGES` all mean
"faces only" — so `mesh.rs` decides, and `mesh_ink.rs` only ever runs when ink is wanted.

**Exit litmus:** `grep -c push_mesh src/ -r` is **0**.

## 3. Files we touch

| file | what |
|---|---|
| `walk/{mesh_topology,mesh,mesh_ink,brep,surface}.rs` | **NEW** |
| `walk/mod.rs` | five arms become five calls |
| `app/scene.rs` | 739 → **284** |
| `app/knobs.rs` | its doc comment catches up |

## 4. The five files

### 4.1 `walk/mesh_topology.rs` — the fused pass

One walk over the faces answers four questions: the edges, each edge's two faces, the face
normals, and whether the mesh is closed. Asking separately cost two more `HashSet`s and a second
full sweep — `Mesh::is_closed()` alone was 10 ms on the bunny.


**Create `src/app/walk/mesh_topology.rs`**

```rust
//! `walk/mesh_topology.rs` - the fused topology pass.
//!
//! One walk over the faces answers four questions at once: the edges, which faces each edge
//! belongs to, the face normals, and whether the mesh is closed. Asking them separately meant
//! four sweeps and two more `HashSet`s - `Mesh::is_closed()` alone cost 10 ms on the bunny and
//! 91 ms on one sheet's 21 fill meshes, every millisecond of it thrown away.
//!
//! `closed` rides out of here for that reason: an edge walked by a face in only one direction IS
//! a border, and the pass already knows it.

use session_rust::{Mesh, Tolerance};

use crate::app::walk::encode::{BLACK, pack_rgba};

/// Two adjacent faces count as one flat region above this normal dot, so the edge between them is
/// interior tessellation rather than an edge of the shape.
///
/// EXACT coplanarity, not "nearly flat". The edges this is meant to remove - a diagonal across a
/// lofted plate cap, an earclipped joint area, any n-gon a kernel fanned out - lie in faces that
/// are the SAME plane, so their f64 normals agree to a few ULPs. 0.9999 was 0.81 deg of slack,
/// which is nothing on a CAD part but is most of the curvature between neighbouring triangles on
/// a dense scan: it silently ate 14,644 of the bunny's 104,288 edges (14%), all of them real
/// surface, and the wireframe came out full of holes. Curvature is not tessellation.
pub(crate) const COPLANAR_DOT: f64 = 1.0 - 1e-9;

/// Everything the ink lanes need to know about a mesh's topology, built in ONE pass over the
/// faces: the edge list (unique, with its pen color), each edge's two adjacent faces, the per-face
/// normal, and whether the mesh is closed.
///
/// It exists because the kernel answers the same four questions in four independent passes -
/// `edges_with_colors`, `edge_face_map`, `face_normals`, `is_closed` - each rebuilding its own
/// hash table over the same faces, and `face_normals` allocating a `Vector` (String name + guid
/// OnceLock) per face on top. That is the kernel's business: three languages share those APIs and
/// they answer honestly on their own. The VIEWER walks every mesh in a scene through all four at
/// once, so it pays for the repetition four times over - 123 ms of the bunny's 137 ms walk.
///
/// Byte-identical to the kernel by construction: same sorted-face-key order, same "first face to
/// walk a directed edge keeps it" rule, same `linecolors[i]` indexing by first-seen edge, same
/// cross-product-and-normalize as `Mesh::face_normal`.
pub(crate) struct MeshTopo {
    /// Unique edges as (low, high) vertex key + PACKED pen color, in `edges_with_colors` order.
    /// Packed here rather than kept as a kernel `Color`, which carries a `name` String and a guid
    /// OnceLock: cloning one per edge was 104k String allocations on the bunny, for four bytes.
    pub(crate) edges: Vec<(usize, usize, u32)>,
    /// Per edge: the face walking (low, high) and the face walking (high, low), as SLOTS into
    /// `normals` (u32::MAX = none). Compacted the way the old two-lookup loop compacted: a lone
    /// face always lands in slot 0.
    pub(crate) edge_faces: Vec<[u32; 2]>,
    /// Per face slot, in sorted-face-key order. `None` for a degenerate face.
    pub(crate) normals: Vec<Option<[f64; 3]>>,
    /// Every edge walked in BOTH directions, i.e. no border. Meshes with declared hole rings fall
    /// back to the kernel, which knows that a ring's own edges are not borders.
    pub(crate) closed: bool,
}

/// One face's normal, from the by-slot position table - no `Point`, no `Vector`, no allocation
/// and no map lookup. Same arithmetic and the same `ZERO_TOLERANCE` cut-off as `Mesh::face_normal`.
fn face_normal_raw(vs: &[usize], vpos: &[[f64; 3]], slot: &impl Fn(usize) -> usize) -> Option<[f64; 3]> {
    if vs.len() < 3 { return None }
    let (p0, p1, p2) = (vpos[slot(vs[0])], vpos[slot(vs[1])], vpos[slot(vs[2])]);
    let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > Tolerance::ZERO_TOLERANCE { Some([n[0] / len, n[1] / len, n[2] / len]) } else { None }
}

/// The fused pass. No hash table at all: edges hang off their LOW vertex on an intrusive chain
/// (`head` per vertex slot, `next` per edge), so finding whether (lo, hi) already exists is a walk
/// of the two or three edges that share `lo` - array reads, no hashing, and deterministic by
/// construction, where a HashMap's order depends on a per-process random seed.
pub(crate) fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slot: &impl Fn(usize) -> usize) -> MeshTopo {
    // SORTED face keys: the kernel sorts too, and it is what makes the pen colors and the packed
    // `facing` words reproducible - `m.face` is a HashMap, so its own iteration order changes
    // between runs of the same binary on the same file.
    // Sorted (key, vertex list) pairs, not sorted keys re-looked-up: `m.face` is a HashMap, so
    // indexing it per face is one hash per face on top of the walk.
    let mut faces: Vec<(usize, &Vec<usize>)> = m.face.iter().map(|(k, v)| (*k, v)).collect();
    faces.sort_unstable_by_key(|f| f.0);
    let cols = m.get_linecolors();

    let mut normals: Vec<Option<[f64; 3]>> = Vec::with_capacity(faces.len());
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()];
    let mut next: Vec<u32> = Vec::new();

    for (fs, (_, vs)) in faces.iter().enumerate() {
        normals.push(face_normal_raw(vs, vpos, slot));
        let n = vs.len();
        for i in 0..n {
            let (u, v) = (vs[i], vs[(i + 1) % n]);
            // dir 0 = this face walks the edge low -> high, dir 1 = high -> low. The two are the
            // two SIDES of the edge, which is exactly what the facing test needs.
            let (lo, hi, dir) = if u < v { (u, v, 0) } else { (v, u, 1) };
            let ls = slot(lo);
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
            // FIRST face wins, like the kernel's `or_insert`: on an inconsistently wound or
            // non-manifold patch two faces walk the same directed edge, and letting the last one
            // win makes the packed `facing` word depend on which face was visited first.
            let f = &mut edge_faces[ei as usize][dir];
            if *f == u32::MAX { *f = fs as u32; }
        }
    }

    // The chain is built by pushing to the FRONT, so edges come out newest-first per vertex -
    // which is not the kernel's order. `edges_with_colors` emits them in first-seen order, and the
    // pen colors are indexed by that, so the list is built in first-seen order too (above) and the
    // chain is only ever used for lookup. Nothing to re-sort.
    let mut closed = !m.vertex.is_empty();
    for f in edge_faces.iter_mut() {
        if f[0] == u32::MAX || f[1] == u32::MAX { closed = false }
        // A lone face moves to slot 0 - the old two-lookup loop filled the slots in lookup order
        // and stopped at the first miss, so a border edge's single normal was always `normal_of(0)`.
        if f[0] == u32::MAX { f[0] = f[1]; f[1] = u32::MAX; }
    }
    // A declared hole ring's edges are borders by this test but not by the kernel's, and only the
    // kernel knows the rings. Rare (PDF poche fills), and it never reaches here anyway - a fill
    // returns before the topology pass.
    if !closed && !m.face_holes.is_empty() { closed = m.is_closed(); }

    MeshTopo { edges, edge_faces, normals, closed }
}
```

### 4.2 `walk/mesh_ink.rs` — the decoration

Everything drawn ON the faces. It is the larger half and the slower one, and it is skipped
entirely by all three gates — which is why it is a separate function rather than a branch.

It returns `closed`, and nothing else. The caller decides what to do with it.


**Create `src/app/walk/mesh_ink.rs`**

```rust
//! `walk/mesh_ink.rs` - a mesh's DECORATION: one cylinder per edge, one marker per vertex.
//!
//! The faces are `mesh.rs`'s job; this is everything drawn ON them. It is the larger half and the
//! slower one - the fused topology walk, the facing words, the per-vertex adjacency - and it is
//! skipped entirely for a dense mesh, a print fill, or `VIEWER_NO_EDGES`, which is why those
//! three gates live in `mesh.rs` and this function is only ever called when ink is wanted.
//!
//! Returns `closed`: an edge walked by a face in only one direction is a border, and the topology
//! pass already knows. The caller turns that into `FLAG_OPEN` - or does not, for a BRep, whose
//! tessellation is often non-watertight.

use session_rust::Mesh;
use session_rust::mesh::ColorMode;

use crate::engine::gpu::{CylinderSegment, GlyphPoint};
use crate::engine::gpu::segments::FACING_UNKNOWN;

use crate::app::knobs::*;

use super::encode::{BLACK, encode_width, oct16, pack_facing};
use super::mesh_topology::{COPLANAR_DOT, mesh_topology};

/// At or above this many edges a mesh's wireframe draws BLACK whatever the file says - see
/// below. 104,288 on the bunny; 12 on a box, whose authored red pen always survives.
const WIREFRAME_BLACK_MIN: usize = 10_000;

/// Push this mesh's edge cylinders and vertex markers. Returns whether the mesh is CLOSED.
pub(crate) fn push_ink(
    m: &Mesh,
    ri: u32,
    segments: &mut Vec<CylinderSegment>,
    glyphs: &mut Vec<GlyphPoint>,
) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    let prof = env_flag("VIEWER_PROFILE", &VIEWER_PROFILE);
    #[cfg(not(target_arch = "wasm32"))]
    let mut lap = std::time::Instant::now();
    #[cfg(not(target_arch = "wasm32"))]
    let mut mark = |name: &str, lap: &mut std::time::Instant| {
        if prof { eprintln!("  push_mesh {name:<20} {:?}", lap.elapsed()); *lap = std::time::Instant::now(); }
    };
    // Same signature on wasm so every `mark(..)` call site below stays identical.
    #[cfg(target_arch = "wasm32")]
    let mut lap = ();
    #[cfg(target_arch = "wasm32")]
    let mut mark = |_name: &str, _lap: &mut ()| {};
    let _ = &mut lap;

    // Edge width 0 = hidden wireframe, A mesh only has explicit widths if someone called
    // set_linecolors, so the 1.0 default below leaves every ordinary mesh untouched - but a triangulated PDF
    // fill (a letter, a pocket region) ask for no wireframe at all, and without
    // this every glyph would render outlined in tubes and dotted at each vertex.
    // A single width broadcasts to every edge - one entry instead of one per edge, which for
    // thousands of small glyph meshes is the difference between a lean .pb and a fat one.
    let width_at = |i: usize| -> f64 {
        let w = m.widths();
        if w.len() == 1 {
            w[0]
        } else {
            w.get(i).copied().unwrap_or(1.0)
        }
    };

    let hidden = |i: usize| width_at(i) == 0.0;

    // ONE face walk builds all three things the lanes need: the edge list with its pen colors,
    // each edge's two adjacent faces, and the face normals (MESH-LOCAL, matching p0/p1 - the
    // shader rotates them by the instance model the same way it transforms the endpoints).
    //
    // The kernel answers the same three questions in three separate passes over the faces, each
    // building its own hash table - and `face_normals` allocates a `Vector` per face, which
    // carries a `name` String and a guid OnceLock. On the bunny (69k faces, 104k edges) that was
    // 39 ms (edges_with_colors) + 43 ms (face_normals) + 28 ms (edge_face_map) + 13 ms of lookups
    // against 30 ms for the fused pass - the single biggest cost in the whole walk. Same walk
    // order (sorted face keys, first face to walk a directed edge keeps it), so the pen colors
    // and the `facing` words come out byte-identical.
    // Vertex keys are arbitrary usizes; everything below indexes by SLOT, the key's position in
    // the sorted order m.vertices() emits. One map build here replaces ~250k kernel map lookups
    // over the three passes below.
    // Vertex keys are arbitrary usizes, but in practice they are dense ids: a Vec indexed BY
    // KEY (u32::MAX sentinel) turns every key->slot lookup below into an array read, where a
    // HashMap trades the same cost the kernel's vertex_point was just removed for. Sparse key
    // spaces (a mesh after deletions) fall back to the map.
    let keys = m.vertices();
    let max_key = keys.last().copied().unwrap_or(0);
    let dense = max_key < 4 * keys.len().max(1);
    let mut slot_vec: Vec<u32> = Vec::new();
    let mut slot_map: std::collections::HashMap<usize, u32> = std::collections::HashMap::new();
    if dense {
        slot_vec = vec![u32::MAX; max_key + 1];
        for (s, &k) in keys.iter().enumerate() { slot_vec[k] = s as u32; }
    } else {
        slot_map = keys.iter().enumerate().map(|(s, &k)| (k, s as u32)).collect();
    }
    let slot = |k: usize| -> usize {
        if dense { slot_vec[k] as usize } else { slot_map[&k] as usize }
    };

    // Positions by slot, from the KERNEL's vertex map - one lookup per vertex instead of two
    // per edge. NOT rm.vertices[slot]: to_render DUPLICATES vertices for per-face colors, so
    // its rows are in vertex order only when no duplication happens (the colors_widths boxes
    // are exactly the case where it does).
    // Straight out of the vertex table, not `vertex_point`, which builds a `Point` (name String
    // + guid OnceLock) per vertex only to read three numbers back off it. Kept in f64 as well:
    // the face normals below are computed from these, and rounding to f32 first would change the
    // sign of a near-degenerate cross product, i.e. the packed `facing` word.
    let vpos64: Vec<[f64; 3]> = keys.iter().map(|&k| { let v = &m.vertex[&k]; [v.x, v.y, v.z] }).collect();
    let vpos: Vec<[f32; 3]> = vpos64.iter().map(|p| [p[0] as f32, p[1] as f32, p[2] as f32]).collect();

    let topo = mesh_topology(m, &keys, &vpos64, &slot);
    let edges = &topo.edges;
    let closed = topo.closed;
    mark("topology", &mut lap);

    // Each edge's adjacent faces, kept for the dots pass: edge_faces allocates a Vec per call,
    // and the dots used to repeat it per incident edge per vertex - the walk's biggest cost.
    // Hidden edges contribute faces too (their face can still carry a visible band), so the
    // call happens even when the segment below is skipped.
    // `topo.edge_faces` holds two slots per edge, not a Vec: an edge has at most two adjacent
    // faces and the code below reads at most two. The old `Vec<usize>` per edge heap-allocated
    // once per edge - 104k allocations on the bunny alone, 87 ms of the pipe loop. u32::MAX = no
    // face in that slot, and the entries are face SLOTS (index into `topo.normals`), not keys.
    let edge_faces = &topo.edge_faces;

    // A DENSE wireframe draws BLACK, whatever linecolors the file carries: at scan density an
    // edge's color is a property of the tessellation, not of the model, and per-edge pens
    // stopped being readable thousands of edges ago. Authored colors on ordinary meshes (the
    // red pen box) are always honored - the gate sits far above any CAD part.
    let black_wire = edges.len() >= WIREFRAME_BLACK_MIN;

    for (i, (a, b, col)) in edges.iter().enumerate(){
        let f = edge_faces[i];

        // The two faces sharing this edge, so the shader can decide whether it faces the camera
        // without asking the depth buffer. An edge with both faces turned away is HIDDEN and the
        // shader drops it; one that keeps only one is a silhouette. This is the whole point of the
        // exercise: a pen has width, so ink tested against the surface it decorates either gets
        // cut by it or has to float in front of it, and no offset wins at every slant. Deciding
        // visibility from the geometry instead sidesteps the trade entirely.
        // Borrowed, never cloned: `Vector` carries a `name` String and a guid OnceLock, so a
        // `.cloned()` here was two heap allocations per edge - 200k on the bunny's wireframe.
        let normal_of = |slot: usize| -> Option<&[f64; 3]> {
            if f[slot] == u32::MAX { None } else { topo.normals[f[slot] as usize].as_ref() }
        };
        let facing = pack_facing(normal_of(0), normal_of(1));

        if hidden(i) {
            continue
        }

        // Interior tessellation, not an edge of the shape. A flat region arrives triangulated -
        // a lofted plate cap, an earclipped joint area, any n-gon a kernel fanned out - and every
        // diagonal across it shares two COPLANAR faces. Drawing those puts a wireframe over what
        // the eye reads as one polygon, which is exactly the triangulation nobody modelled.
        // A boundary edge has one face and a crease has two that disagree, so both survive;
        // this only ever removes ink that lies flat inside a face. VIEWER_ALL_EDGES brings the
        // full tessellation back for debugging a mesh's actual topology.
        if let (Some(n0), Some(n1)) = (normal_of(0), normal_of(1)) {
            let dot = n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2];
            if dot >= COPLANAR_DOT && !env_flag("VIEWER_ALL_EDGES", &VIEWER_ALL_EDGES) {
                continue
            }
        }
        segments.push(
            CylinderSegment{
                p0: vpos[slot(*a)],
                radius: encode_width(width_at(i)),
                p1: vpos[slot(*b)],
                instance_id: ri,
                color: if black_wire { BLACK } else { *col },
                facing,
            }
        )
    }
    mark("pipe loop", &mut lap);

    // Dots are used for user set pointcolors.
    // The auto-seeded white vec is filtered by the mode gate.
    // m.vertices() is sorted  - the same order to_render indexes pointcolors by.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();

    // A vertex sphere must be as fas as the pipes.
    // The kernel has no per-vertex width, so take the widest incident edge - and remember WHICH
    // edge it was. The dot inherits that edge's pen color and leads its `facing` adjacency, so
    // the sphere lane hugs the faces the bands meeting at the vertex already hug: a marker
    // floating on the old constant lift loses the depth test to its own hugged bands over most
    // of its disc at close zoom, and shows up as a lopsided chunk smaller than the band width.
    // Same two passes, by slot instead of by key: vbest as a flat Vec (sentinel -inf = no
    // visible edge yet; widths can be NEGATIVE world-mm radii, so the sentinel is not 0) and
    // the incident list as CSR (degree count, prefix sum, fill) instead of a Vec per vertex.
    let mut vbest = vec![(f64::NEG_INFINITY, 0usize); keys.len()];
    for (i, (a, b, _)) in edges.iter().enumerate(){
        if hidden(i){ // A vertex whose every edge is hidden gets no dot either
            continue;
        }
        let w = width_at(i);
        for vk in [*a, *b] {
            let e = &mut vbest[slot(vk)];
            if w > e.0 {
                *e = (w, i);
            }
        }
    }

    // Incident EDGES per vertex, for the face list below. Hidden edges contribute faces too:
    // a hidden edge's adjacent face can still carry a visible band from another edge, and the
    // dot must hug that face to stay in front of it.
    let mut vstart = vec![0u32; keys.len() + 1];
    for (a, b, _) in edges.iter(){
        vstart[slot(*a) + 1] += 1;
        vstart[slot(*b) + 1] += 1;
    }
    for i in 0..keys.len(){
        vstart[i + 1] += vstart[i];
    }
    let mut vinc = vec![0u32; 2 * edges.len()];
    let mut cur = vstart.clone();
    for (i, (a, b, _)) in edges.iter().enumerate(){
        for vk in [*a, *b] {
            let s = slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }
    mark("vbest+vedges", &mut lap);

    // VIEWER_NO_DOTS drops the per-vertex dots, so the harness can tell how much of a dense
    // wireframe's ink is dots and how much is edges.
    if env_flag("VIEWER_NO_DOTS", &VIEWER_NO_DOTS) { return closed }

    // Widest edge's faces first, then every other incident edge's, deduped - one reused Vec,
    // and the face lists cached from the pipe pass instead of a kernel call per incident edge.
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    for i in 0..keys.len(){
        let (vw, ei) = vbest[i];
        if vw == f64::NEG_INFINITY { continue }

        // Face keys, widest edge's pair first, then every other incident edge's, deduped. The
        // row carries up to SIX normals (3 words x oct16 pair): a trihedral corner needs three,
        // and hugging only the widest edge's two leaves the third face's band able to bite a
        // sector out of the disc at grazing slants - the marker is meant to go in front.
        fkeys.clear();
        let take = |ei: usize, fkeys: &mut Vec<usize>| {
            for &f in edge_faces[ei].iter() {
                if f == u32::MAX { continue }
                let fk = f as usize;
                if !fkeys.contains(&fk) { fkeys.push(fk); }
            }
        };
        take(ei, &mut fkeys);
        for &j in &vinc[vstart[i] as usize..vstart[i + 1] as usize] {
            take(j as usize, &mut fkeys);
        }
        codes.clear();
        codes.extend(
            fkeys.iter()
                .filter_map(|fk| topo.normals[*fk].as_ref())
                .filter_map(oct16)
                .take(6),
        );
        // pack_facing's rules: a lone normal is duplicated, none at all is FACING_UNKNOWN, and a
        // pair colliding with the all-ones sentinel collapses to it (accepted loss, same as edges).
        let word = |k: usize| -> u32 {
            match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
                (Some(a), b) => {
                    let v = a | b.unwrap_or(a) << 16;
                    if v == FACING_UNKNOWN { FACING_UNKNOWN } else { v }
                }
                _ => FACING_UNKNOWN,
            }
        };
        glyphs.push(
            GlyphPoint {
                center: vpos[i],
                radius: encode_width(vw),
                // No pointcolors -> fixed near-black marker, whatever the pen color is: the dot
                // must read as a DOT so the joint can be checked by eye (following the pen color
                // hid the marker exactly where checking happens - black on a black-penned cube).
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: ri,
                facing: word(0),
                facing_ext: [word(1), word(2)],
            }
        );
    }
    mark("dots loop", &mut lap);
    closed
}
```

### 4.3 `walk/mesh.rs` — the faces, the gates, and `MeshOpts`

Read `MeshOpts` before you paste it. Four fields against eight parameters, and two of them are
decisions the CALLER owns rather than facts the mesh carries.

**This is the one place the block's moves-only rule is broken, and on purpose.** `allow_open`
gives `Element(Mesh)` back a flag it never should have lost — see §1a. It changes no gate scene
(no mandatory scene holds an Element), so the pixel gate cannot prove it either way; what proves
it is that the omission is now impossible to make silently.


**Create `src/app/walk/mesh.rs`**

```rust
//! `walk/mesh.rs` - the Mesh producer: faces, bounds, and the three gates.
//!
//! The decoration - one cylinder per edge, one marker per vertex - is `mesh_ink.rs`. This half
//! writes the triangles, measures the mesh-local AABB, and decides whether the ink half runs at
//! all. Three gates say no: a DENSE mesh (over `MESH_RAW_MIN` triangles), a print FILL, and
//! `VIEWER_NO_EDGES`.
//!
//! `MeshOpts` replaced eight positional parameters and a two-element tuple return. The tuple was
//! the bug: `push_mesh` returned `(bounds, closed)` and three of its five callers discarded
//! `closed`, which is how `Element(Mesh)` came to be missing `FLAG_OPEN` - not by decision, by
//! omission. Now the caller states `allow_open` and the flags come back inside the `Row`.

use session_rust::Mesh;

use crate::engine::gpu::{Instance, Upload};

use crate::app::knobs::*;
use crate::math::grow_bounds;

use super::Row;
use super::mesh_ink::push_ink;
use crate::app::scene::{is_print_fill, mesh_spacing};

/// Above this many triangles a mesh draws as TRIANGLES ONLY - no per-edge cylinder, no
/// per-vertex sphere. Below it, the wireframe and vertex dots are what make a CAD solid
/// readable. At 200k the bunny (69k tri) keeps its wireframe and the armadillo and dragon
/// do not - which is the honest line until an impostor makes the decoration cheap. A PDF fill
/// (tens of triangles) and a demo box (12) stay decorated; a scan does not.
const MESH_RAW_MIN: usize = 200_000;

/// How one mesh is walked. Four fields against `push_mesh`'s eight parameters, and two of them
/// are decisions the CALLER owns rather than facts the mesh carries.
pub(crate) struct MeshOpts {
    /// The object row this mesh draws against.
    pub ri: u32,
    /// GPU vertex rows already uploaded, so indices land at the right absolute offset.
    pub base_off: u32,
    /// Route a print fill into the sheet index runs. True for Mesh and Element(Mesh); false for
    /// BRep and NurbsSurface, whose tessellations are always solid geometry.
    pub sheet_lanes: bool,
    /// May this mesh set `FLAG_OPEN`? Only a real Mesh may: a BRep tessellation is often
    /// numerically non-watertight and would lose the facing cull wholesale, and an Element's
    /// mesh is a fabrication output rather than a solid to look inside.
    pub allow_open: bool,
}

/// Walk a mesh into the arena and (unless a gate says otherwise) the ink lanes.
pub(crate) fn walk_mesh(m: &Mesh, t: &mut Upload, o: &MeshOpts) -> Row {
    // Which index run these triangles join decides WHEN they are drawn, and for a drawing that is
    // the whole answer: sheet fills composite in document order with no depth arbitration, and
    // lettering goes last of all. The "text" name is set by the PDF importer, which knows a glyph
    // from a region.
    let print = o.sheet_lanes && is_print_fill(m);
    let print_flag = if print { Instance::FLAG_PRINT } else { 0 };
    let lane = if print {
        if m.name == "text" { &mut t.arena.idx_text } else { &mut t.arena.idx_print }
    } else {
        &mut t.arena.idx
    };
    let base = o.base_off + t.arena.verts.len() as u32; // GPU rows already uploaded + rows pending in this delta
    // VIEWER_PROFILE=1 times the walk's stages. HARNESS-ONLY, and the cfg is load-bearing, not
    // tidiness: `Instant::now()` on wasm32-unknown-unknown does not return a dummy, it PANICS
    // ("time not implemented on this platform"), and this line runs for every mesh - so an
    // ungated clock here kills the browser build on the first mesh it walks.
    #[cfg(not(target_arch = "wasm32"))]
    let prof = env_flag("VIEWER_PROFILE", &VIEWER_PROFILE);
    #[cfg(not(target_arch = "wasm32"))]
    let mut lap = std::time::Instant::now();
    #[cfg(not(target_arch = "wasm32"))]
    let mut mark = |name: &str, lap: &mut std::time::Instant| {
        if prof { eprintln!("  push_mesh {name:<20} {:?}", lap.elapsed()); *lap = std::time::Instant::now(); }
    };
    // Same signature on wasm so every `mark(..)` call site below stays identical.
    #[cfg(target_arch = "wasm32")]
    let mut lap = ();
    #[cfg(target_arch = "wasm32")]
    let mut mark = |_name: &str, _lap: &mut ()| {};
    let rm = m.to_render();
    mark("to_render", &mut lap);

    // The mesh-local AABB rides the object row, so the edge lanes can be told "the eye is inside
    // this solid" (Instance::FLAG_INSIDE) - the facing cull's premise, both faces away = hidden,
    // is only valid for an eye OUTSIDE. Computed even when the wireframe below is skipped: the
    // flag costs nothing and the lanes ignore it when there are no edges.
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for v in &rm.vertices{
        grow_bounds(&mut lo, &mut hi, v.position);
        t.arena.verts.push(*v);
        t.arena.vids.push(o.ri);
    }
    let local_bounds = if lo[0] <= hi[0] { Some((lo, hi)) } else { None };
    for &i in &rm.indices{
        lane.push(base+i);
    }
    mark("vert+idx push", &mut lap);

    // A DENSE mesh gets no wireframe and no vertex dots. This is the same call the cloud lane
    // makes at CLOUD_RAW_MIN, and for the same reason - decoration that is free on a CAD solid
    // is ruinous on a scan.
    //
    // Measured on the Stanford ladder (1.29M mesh triangles): the per-edge cylinders and
    // per-vertex spheres added 23.2M and 92.9M triangles respectively - 90x the geometry they
    // were decorating - and 118 MB of segment/glyph tables against 25 MB of actual mesh arena.
    // The walk cost 12.4 s, most of it in edges_with_colors() building 1.9M edges and a HashSet.
    //
    // Selection is NOT affected: picking a vertex, an edge or a whole mesh reads the kernel
    // Mesh (positions, indices, BVH), never these drawn tubes and dots. When a dense mesh is
    // selected, its wireframe can be emitted for that one mesh on demand.
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return Row::solid(None, mesh_spacing(None, m.number_of_vertices()), print_flag);
    }


    // A fill (every PDF glyph, every poché region) broadcasts a single width of 0 - no wireframe
    // at all. Leave before edges_with_colors, which builds a HashSet over the faces: for sheets
    // made of hundreds of thousands of tiny fills, that set was the walk's biggest single cost
    // and every edge it produced was then skipped.
    if is_print_fill(m) { return Row::solid(None, mesh_spacing(None, m.number_of_vertices()), print_flag) }

    if env_flag("VIEWER_NO_EDGES", &VIEWER_NO_EDGES) { return Row::solid(None, mesh_spacing(None, m.number_of_vertices()), print_flag) }

    let closed = push_ink(m, o.ri, &mut t.seg.pipes, &mut t.glyph.spheres);
    // An open mesh (boundary edges) is not a solid: the facing cull's premise - both adjacent
    // faces away means the far side of a solid, hidden - is void, because an interior surface can
    // be genuinely visible through the hole. The answer rides out of the topology pass, which
    // already knows: an edge walked by a face in only one direction IS a border.
    let open_flag = if o.allow_open && !closed { Instance::FLAG_OPEN } else { 0 };
    Row::solid(local_bounds, mesh_spacing(local_bounds, m.number_of_vertices()), print_flag | open_flag)
}
```

### 4.4 The two adapters

Twenty-two lines and eighteen. Each re-enters the mesh producer with different options; the only
real divergence between them is one line of colour.


**Create `src/app/walk/brep.rs`**

```rust
//! `walk/brep.rs` - the BRep adapter.
//!
//! A BRep reaches the GPU as its own tessellation, so this is not a producer in its own right:
//! it re-enters the mesh producer with different options. Three lines of its own, and the two
//! that matter are the options.
//!
//! `allow_open: false` is the one real decision. A BRep tessellation is often numerically
//! non-watertight, so `FLAG_OPEN` would be set for nearly every solid and the facing cull would
//! be lost wholesale - the wireframe drawn on the far side of every part.

use session_rust::BRep;

use crate::engine::gpu::Upload;

use super::Row;
use super::mesh::{MeshOpts, walk_mesh};

pub(crate) fn walk_brep(b: &BRep, t: &mut Upload, ri: u32, base_off: u32) -> Row {
    let mut bm = b.mesh();
    bm.set_objectcolor(b.surfacecolor.clone());
    walk_mesh(&bm, t, &MeshOpts { ri, base_off, sheet_lanes: false, allow_open: false })
}
```

**Create `src/app/walk/surface.rs`**

```rust
//! `walk/surface.rs` - the NurbsSurface adapter.
//!
//! Two lines of its own, and the divergence from `brep.rs` is exactly one of them: the colour
//! comes from `facecolors.first()`, not from `surfacecolor`. Everything else is the same
//! re-entry into the mesh producer with the same options.

use session_rust::NurbsSurface;

use crate::engine::gpu::Upload;

use super::Row;
use super::mesh::{MeshOpts, walk_mesh};

pub(crate) fn walk_surface(s: &NurbsSurface, t: &mut Upload, ri: u32, base_off: u32) -> Row {
    let mut sm = s.mesh();
    if let Some(c) = s.facecolors.first() { sm.set_objectcolor(c.clone()); }
    walk_mesh(&sm, t, &MeshOpts { ri, base_off, sheet_lanes: false, allow_open: false })
}
```

## 5. The steps

### 5.1 `walk/mod.rs` — five arms become five calls


**Find** in `src/app/walk/mod.rs`:

```rust
pub mod bounds;
pub mod cloud;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod points;

use session_rust::Geometry;
use session_rust::element::ElementGeometry;

use crate::engine::gpu::{CloudDraw, Instance, Upload};

// The mesh family still lives in `scene.rs`; lesson 51 gives it `walk/mesh.rs` and these three
// come with it.
use super::scene::{is_print_fill, mesh_spacing, push_mesh};

use cloud::{cloud_spacing, push_cloud};
use curves::{line_to_segment, nurbscurve_to_segments, polyline_to_segments};
use frames::{obb_to_segments, plane_to_segments};
use points::point_to_glyph;
```

**Replace with:**

```rust
pub mod bounds;
pub mod brep;
pub mod cloud;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod mesh;
pub mod mesh_ink;
pub mod mesh_topology;
pub mod points;
pub mod surface;

use session_rust::Geometry;
use session_rust::element::ElementGeometry;

use crate::engine::gpu::{CloudDraw, Upload};

// The mesh family still lives in `scene.rs`; lesson 51 gives it `walk/mesh.rs` and these three
// come with it.

use cloud::{cloud_spacing, push_cloud};
use curves::{line_to_segment, nurbscurve_to_segments, polyline_to_segments};
use frames::{obb_to_segments, plane_to_segments};
use points::point_to_glyph;
```

**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::Mesh(m) => {
            // Which index run this mesh's triangles join decides WHEN it is drawn, and
            // for a drawing that is the whole answer: sheet fills composite in document
            // order with no depth arbitration, and lettering goes last of all - after the
            // ink lanes. `is_print_fill` is the sheet test the walk already uses; the
            // "text" name is set by the PDF importer, which knows a glyph from a region.
            let idx_lane = if is_print_fill(m) {
                if m.name == "text" { &mut t.arena.idx_text } else { &mut t.arena.idx_print }
            } else {
                &mut t.arena.idx
            };
            let (b, closed) = push_mesh(
                m,
                ri,
                cx.vert_base,
                &mut t.arena.verts,
                &mut t.arena.vids,
                idx_lane,
                &mut t.seg.pipes,
                &mut t.glyph.spheres
            );
            let mut flags = if is_print_fill(m) { Instance::FLAG_PRINT } else { 0 };
            // An open mesh (boundary edges) is not a solid: the facing cull would strip
            // the wireframe off interior surface that is genuinely visible through the
            // hole while the faces still draw. Meshes only - a BRep tessellation is often
            // numerically non-watertight and its solids would lose the cull wholesale.
            // ONLY when this mesh actually drew a wireframe. `b` is None for a print
            // fill and for a dense mesh - neither emits pipes or dots, and FLAG_OPEN is
            // read by nothing else (cylinder/sphere/ribbon shaders only). The answer rides
            // out of push_mesh because the fused topology pass already knows it - an edge
            // walked by a face in only one direction IS a border. `Mesh::is_closed()` was a
            // SECOND full sweep, two more HashSets over every directed face edge: 10 ms on
            // the bunny, 91 ms on one sheet's 21 fill meshes, every millisecond of it
            // thrown away.
            if b.is_some() && !closed {
                flags |= Instance::FLAG_OPEN;
            }
            Row::solid(b, mesh_spacing(b, m.number_of_vertices()), flags)
        }
```

**Replace with:**

```rust
        Geometry::Mesh(m) => walk_mesh(m, t, &MeshOpts {
            ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: true,
        }),
```

**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::BRep(b) => {
            let mut bm = b.mesh();
            bm.set_objectcolor(b.surfacecolor.clone());
            let (bb, _) = push_mesh(
                &bm,
                ri,
                cx.vert_base,
                &mut t.arena.verts,
                &mut t.arena.vids,
                &mut t.arena.idx,
                &mut t.seg.pipes,
                &mut t.glyph.spheres
            );
            Row::solid(bb, mesh_spacing(bb, bm.number_of_vertices()), 0)
        }
```

**Replace with:**

```rust
        Geometry::BRep(b) => walk_brep(b, t, ri, cx.vert_base),
```

**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::NurbsSurface(s) => {
            let mut sm = s.mesh();
            if let Some(c) = s.facecolors.first() {
                sm.set_objectcolor(c.clone());
            }
            let (b, _) = push_mesh(
                &sm,
                ri,
                cx.vert_base,
                &mut t.arena.verts,
                &mut t.arena.vids,
                &mut t.arena.idx,
                &mut t.seg.pipes,
                &mut t.glyph.spheres
            );
            Row::solid(b, mesh_spacing(b, sm.number_of_vertices()), 0)
        }
```

**Replace with:**

```rust
        Geometry::NurbsSurface(s) => walk_surface(s, t, ri, cx.vert_base),
```

**Find** in `src/app/walk/mod.rs`:

```rust
            ElementGeometry::Mesh(m) => {
                let idx_lane = if is_print_fill(&m) {
                    if m.name == "text" { &mut t.arena.idx_text } else { &mut t.arena.idx_print }
                } else {
                    &mut t.arena.idx
                };
                let (b, _) = push_mesh(
                    &m,
                    ri,
                cx.vert_base,
                    &mut t.arena.verts,
                    &mut t.arena.vids,
                    idx_lane,
                    &mut t.seg.pipes,
                    &mut t.glyph.spheres
                );
                // Element(Mesh) is the ONE place FLAG_OPEN is deliberately withheld: an element's
                // mesh is a fabrication output, not a solid whose interior can be looked into.
                let flags = if is_print_fill(&m) { Instance::FLAG_PRINT } else { 0 };
                Row::solid(b, mesh_spacing(b, m.number_of_vertices()), flags)
            }
```

**Replace with:**

```rust
            // The ONE place FLAG_OPEN is withheld from a real mesh: an element's mesh is a
            // fabrication output, not a solid whose interior can be looked into.
            ElementGeometry::Mesh(m) => walk_mesh(&m, t, &MeshOpts {
                ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: false,
            }),
```

**Find** in `src/app/walk/mod.rs`:

```rust
            ElementGeometry::BRep(b) => {
                let mut bm = b.mesh();
                bm.set_objectcolor(b.surfacecolor.clone());
                let (bb, _) = push_mesh(
                    &bm,
                    ri,
                cx.vert_base,
                    &mut t.arena.verts,
                    &mut t.arena.vids,
                    &mut t.arena.idx,
                    &mut t.seg.pipes,
                    &mut t.glyph.spheres
                );
                Row::solid(bb, mesh_spacing(bb, bm.number_of_vertices()), 0)
            }
```

**Replace with:**

```rust
            ElementGeometry::BRep(b) => walk_brep(&b, t, ri, cx.vert_base),
```

**Find** in `src/app/walk/mod.rs`:

```rust
use points::point_to_glyph;
```

**Replace with:**

```rust
use points::point_to_glyph;
use brep::walk_brep;
use mesh::{MeshOpts, walk_mesh};
use surface::walk_surface;
```

### 5.2 `scene.rs` gives up the mesh


**Find** in `src/app/scene.rs`:

```rust
use std::collections::{HashMap, HashSet};
use session_rust::Xform;
use session_rust::{Session, Geometry, Mesh, RenderVertex, Tolerance};
use session_rust::element::ElementGeometry;
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{Upload, Instance, CylinderSegment, GlyphPoint, mat_mul};
use crate::engine::gpu::objects::ObjectBase;
use crate::engine::gpu::segments::FACING_UNKNOWN;
use super::knobs::*;
pub use super::manifest::{Item, Manifest, auto_grid};
use super::walk::encode::*;
use super::walk::bounds::{Baselines, file_extent, mark_sheet, sheet_thickness};
use super::walk::{WalkCx, walk_geometry};
pub use crate::math::{grow_bounds, xform_point};
```

**Replace with:**

```rust
use std::collections::{HashMap, HashSet};
use session_rust::Xform;
use session_rust::{Session, Geometry, Mesh};
use session_rust::element::ElementGeometry;
use crate::engine::gpu::{Upload, Instance, mat_mul};
use crate::engine::gpu::objects::ObjectBase;
use super::knobs::*;
pub use super::manifest::{Item, Manifest, auto_grid};
use super::walk::bounds::{Baselines, file_extent, mark_sheet, sheet_thickness};
use super::walk::{WalkCx, walk_geometry};
pub use crate::math::{grow_bounds, xform_point};
```

**Remove** `src/app/scene.rs` `/// EXACT coplanarity, not "nearly flat". The edges this is meant to remove - a diagonal across a` **through** `}`

**Remove** `src/app/scene.rs` `pub(crate) fn push_mesh(` **through** `}`

**Find** in `src/app/scene.rs`:

```rust
/// Debug toggles read ONCE per process instead of once per mesh. An env lookup is a linear
/// scan of the environment block, and a sheet can hold tens of thousands of tiny fill meshes -
/// at three reads per mesh (PROFILE, NO_EDGES, NO_DOTS) that alone was ~30 ms against HEAD's
/// two on a 33 MB sheet. These are launch-time harness toggles; setting one mid-process was
/// never a use case.
```

**Replace with:**

```rust

```

**Find** in `src/app/scene.rs`:

```rust
/// Two adjacent faces count as one flat region above this normal dot, so the edge between them is
/// interior tessellation rather than an edge of the shape.
///
```

**Replace with:**

```rust

```

**Remove** `src/app/scene.rs`

```rust
/// One face's normal, from the by-slot position table - no `Point`, no `Vector`, no allocation
```

```rust
}
```

**Remove** `src/app/scene.rs`

```rust
/// The fused pass. No hash table at all: edges hang off their LOW vertex on an intrusive chain
```

```rust
}
```

**Find** in `src/app/scene.rs`:

```rust


/// Above this many triangles a mesh draws as TRIANGLES ONLY - no per-edge cylinder, no
/// per-vertex sphere. Below it, the wireframe and vertex dots are what make a CAD solid
/// readable. At 200k the bunny (69k tri) keeps its wireframe and the armadillo and dragon
/// do not - which is the honest line until an impostor makes the decoration cheap. A PDF fill (tens of triangles) and a
/// demo box (12) stay decorated; a scan does not.
const MESH_RAW_MIN: usize = 200_000;

/// At or above this many edges a mesh's wireframe draws BLACK whatever the file says - see
/// push_mesh. 104,288 on the bunny; 12 on a box, whose authored red pen always survives.
const WIREFRAME_BLACK_MIN: usize = 10_000;

```

**Replace with:**

```rust



```

The doc comment for those five toggles travels with them - it was left behind in `scene.rs` when
the statics moved at lesson 50.

**Find** in `src/app/knobs.rs`:

```rust
pub fn env_flag(
```

**Add above it:**

```rust
/// Debug toggles read ONCE per process instead of once per mesh. An env lookup is a linear
/// scan of the environment block, and a sheet can hold tens of thousands of tiny fill meshes -
/// at three reads per mesh (PROFILE, NO_EDGES, NO_DOTS) that alone was ~30 ms against HEAD's
/// two on a 33 MB sheet. These are launch-time harness toggles; setting one mid-process was
/// never a use case.
```

**Find** in `src/app/scene.rs`:

```rust
    m.widths().len() == 1 && m.widths()[0] == 0.0
}










```

**Replace with:**

```rust
    m.widths().len() == 1 && m.widths()[0] == 0.0
}







```

## 6. Proving nothing changed

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets --target x86_64-unknown-linux-gnu
cargo xtest
./docs/_gate.sh                # twice
cargo run -q --release --example check_determinism --target x86_64-unknown-linux-gnu -- assets/pb/lion.pb
cargo run -q --release --example check_lean        --target x86_64-unknown-linux-gnu -- assets/pb/mesh_bunny.pb
```

```text
0 errors
test result: ok. 4 passed
gate OK                        (both runs)
lion.pb: DETERMINISTIC
mesh_bunny.pb: IDENTICAL
```

**What the gate cannot prove, stated plainly.** `allow_open` restores `FLAG_OPEN` to
`Element(Mesh)`, and **no mandatory scene contains an Element** — flipping that one field either
way leaves `bunny` at exactly 44,215 ink. The pixel gate is silent on the only behaviour this
lesson changes. What replaces it is that the decision is now a named field at five call sites
instead of a value four callers happened to drop.

## 7. What you can now do in one line

`allow_open` is one field. Turn it off for a real mesh and watch the bunny lose the wireframe on
its open base — the exact failure `Element(Mesh)` has been shipping.

**7a.** **Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::Mesh(m) => walk_mesh(m, t, &MeshOpts {
            ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: true,
        }),
```

**Replace with:**

```rust
        Geometry::Mesh(m) => walk_mesh(m, t, &MeshOpts {
            ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: false,
        }),
```

```bash
cargo run -q --release --example selftest --target x86_64-unknown-linux-gnu -- \
    /tmp/noopen.ppm assets/scenes/bunny.toml
```

```text
wrote /tmp/noopen.ppm  900x700  non-background pixels: 44180 (7.0%)
```

**605 pixels** vanish — ink 44,215 → 44,180 — all of them wireframe on the surface visible
through the bunny's open base, where the facing cull now wrongly decides both adjacent faces
point away.

**7b.** Put it back. **Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::Mesh(m) => walk_mesh(m, t, &MeshOpts {
            ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: false,
        }),
```

**Replace with:**

```rust
        Geometry::Mesh(m) => walk_mesh(m, t, &MeshOpts {
            ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: true,
        }),
```

## 8. What is deliberately not here

- **`app/persistence.rs`.** 453 lines, the largest leaf under `app/` now, and declared over cap
  since lesson 43. Its three-way split is lesson **59**.
- **`chain_table` / `compartments_hold`.** The tests that assert the geometry-to-shader map are
  worth having, but they assert a table this lesson only just completed. First lesson that adds a
  fourteenth row writes them.
- **Sub-object identity.** A row carries `instance_id` — the object — and still nothing that says
  which face or edge of the kernel mesh produced it. `Row` (lesson 50) is where that field goes;
  lesson **114** puts it there with the id buffer.
- **`enum Spacing { World, Pixels }`.** Still one f32 carrying two units, named at the write site
  by `Row::solid` and `Row::point_size_px`.

## 9. Expected state

```bash
wc -l src/app/scene.rs
grep -rc push_mesh src/ | grep -v ':0'
ls src/app/walk/
```

```text
284

(nothing)

bounds.rs  brep.rs  cloud.rs  curves.rs  encode.rs  frames.rs  mesh.rs
mesh_ink.rs  mesh_topology.rs  mod.rs  points.rs  surface.rs
```

| | end-of-44 | end-of-51 |
|---|---|---|
| `Gpu` fields | 116 | **18** |
| `engine/gpu/mod.rs` | 2,447 | **524** |
| `engine/pipelines/mod.rs` | 845 | **52** |
| `app/scene.rs` | 1,333 | **284** |
| `push_mesh` | 314 lines, 8 params | **gone** |
| largest `app/walk/` leaf | — | 290 (`mesh_ink.rs`) |

## Recap

```text
The block is done. 45 made a pipeline a value. 46 put a floor under the families. 47 built the
object table every row points at. 48 settled what a module is - the row, its tables, its
pipelines, its draws. 49 finished the engine and made the frame a list of eleven. 50 turned the
walk onto the other axis, one file per kernel type, and made a producer RETURN its object row.

51 splits the last god-function. push_mesh was 314 lines and eight parameters and returned a
tuple whose second element four of its five callers threw away - which is how Element(Mesh) came
to be missing FLAG_OPEN, by omission rather than decision. It is now three files split at the
three gates, and MeshOpts states as named fields what the caller must decide. BRep and
NurbsSurface are 22 and 18 lines: adapters that re-enter the mesh producer, differing from each
other by one line of colour.

Gpu went 116 fields to 18. scene.rs went 1,333 lines to 284. Nothing in any scene moved by one
pixel across seven lessons.
```

## Edited

`walk/{mesh_topology,mesh,mesh_ink,brep,surface}.rs` (NEW) · `walk/mod.rs` (five arms → five
calls) · `app/scene.rs` (739 → 284) · `app/knobs.rs` (its doc comment).

## Next

Lesson **52** — **NurbsCurve**, and the first lesson written against the new paths. A geometry
type is now one file under `app/walk/` and, if it needs a new row format, one file under
`engine/gpu/`. Nothing else has to move.
