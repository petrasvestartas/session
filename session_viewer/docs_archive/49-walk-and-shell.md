# 49 The walk and the shell — one file per geometry type

> Fifth refactor lesson. Start from the end of lesson 48. Pixels stay identical.

<svg viewBox="0 0 720 300" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="walk_geometry: the Geometry arms on the left write only the Walk sinks their producer is handed, and return a Row that scene.rs turns into the object row" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <defs><marker id="wa" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#f0b35c"/></marker><marker id="wg" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#7ed37e"/></marker></defs>
  <rect x="14" y="10" width="692" height="26" fill="none" stroke="#7ed37e" stroke-width="1.3"/>
  <text x="360" y="27" fill="#d7dae0" font-size="11" text-anchor="middle">walk_geometry(w: &amp;mut Walk, cx: &amp;WalkCx, geom: &amp;Geometry) -&gt; Row      — app/walk/mod.rs, lesson 49</text>
  <text x="14" y="58" fill="#888" font-size="10">Geometry — 13 arms, one file each</text>
  <g fill="#d7dae0" font-size="10">
    <text x="14" y="78">Mesh <tspan fill="#f0b35c">mesh.rs</tspan></text>
    <text x="14" y="100">BRep <tspan fill="#f0b35c">brep.rs</tspan> <tspan fill="#888" font-size="9">→ walk_mesh</tspan></text>
    <text x="14" y="122">NurbsSurface <tspan fill="#f0b35c">surface.rs</tspan> <tspan fill="#888" font-size="9">→ walk_mesh</tspan></text>
    <text x="14" y="144">Line Polyline NurbsCurve <tspan fill="#f0b35c">curves.rs</tspan></text>
    <text x="14" y="166">Point <tspan fill="#f0b35c">points.rs</tspan></text>
    <text x="14" y="188">Plane OBB <tspan fill="#f0b35c">frames.rs</tspan></text>
    <text x="14" y="210">PointCloud <tspan fill="#f0b35c">cloud.rs</tspan></text>
    <text x="14" y="232">Element(Mesh · BRep · None) <tspan fill="#f0b35c">mesh.rs brep.rs · Row::none()</tspan></text>
  </g>
  <g stroke="#f0b35c" marker-end="url(#wa)">
    <line x1="220" y1="75" x2="288" y2="103"/><line x1="220" y1="75" x2="288" y2="125"/><line x1="220" y1="75" x2="288" y2="147"/>
    <line x1="220" y1="141" x2="288" y2="125"/><line x1="220" y1="163" x2="288" y2="147"/>
    <line x1="220" y1="185" x2="288" y2="125"/><line x1="220" y1="207" x2="288" y2="169"/>
  </g>
  <rect x="290" y="58" width="240" height="126" fill="none" stroke="#3a3a3a"/>
  <text x="298" y="74" fill="#888" font-size="10">Walk — the sinks, narrow per producer</text>
  <g fill="#d7dae0" font-size="10">
    <text x="298" y="107">arena: ArenaRows <tspan fill="#888" font-size="9">walk_mesh</tspan></text>
    <text x="298" y="129">seg: SegRows <tspan fill="#888" font-size="9">walk_mesh walk_line frames</tspan></text>
    <text x="298" y="151">glyph: GlyphRows <tspan fill="#888" font-size="9">walk_mesh walk_point</tspan></text>
    <text x="298" y="173">cloud: CloudRows <tspan fill="#888" font-size="9">walk_cloud</tspan></text>
    <text x="298" y="91" fill="#666">obj: ObjectRows <tspan font-size="9">— no producer writes it</tspan></text>
  </g>
  <line x1="530" y1="78" x2="546" y2="78" stroke="#7ed37e" marker-end="url(#wg)"/>
  <rect x="548" y="58" width="158" height="70" fill="none" stroke="#7ed37e"/>
  <g fill="#d7dae0" font-size="10">
    <text x="556" y="76">Row {</text><text x="556" y="90" font-size="9">bounds: Option&lt;(lo,hi)&gt;,</text><text x="556" y="104" font-size="9">spacing: f32, flags: u32 }</text>
  </g>
  <text x="556" y="120" fill="#7ed37e" font-size="9">returned, never pushed</text>
  <rect x="548" y="140" width="158" height="44" fill="none" stroke="#3a3a3a"/>
  <text x="556" y="158" fill="#d7dae0" font-size="10">WalkCx { vert_base,</text>
  <text x="556" y="172" fill="#d7dae0" font-size="9">cloud_base, cloud_px, row }</text>
  <text x="556" y="200" fill="#888" font-size="9">scene.rs pushes the obj row</text>
  <text x="556" y="212" fill="#888" font-size="9">from Row + place + colour</text>
  <g fill="#888" font-size="10">
    <text x="14" y="260">walk_mesh(arena, ink, m, &amp;MeshOpts) · walk_line(seg, ..) · walk_point(glyph, ..) · walk_cloud(cloud, ..)</text>
    <text x="14" y="276">which shaders a type can reach is readable off the signature — a producer receives ONLY the groups it writes</text>
    <text x="14" y="292">deleted here: push_mesh and the 13-arm match in scene.rs; amber = app side, green = new in lesson 49</text>
  </g>
</svg>

## Goal

`scene.rs` (1327 lines, a thirteen-arm match around a 314-line `push_mesh`) becomes a 212-line
document owner plus `app/walk/`, one producer per kernel geometry type. `lib.rs` (522 lines)
becomes a 150-line shell: the loader moves to `app/loader.rs`, the bindings to `app/input.rs`,
the fetch/decode/stream reader to three files of their own.

## Why

A producer that receives only the row groups it writes says, in its signature, which shaders
its type can reach; a producer that returns its object row cannot push one by accident. That is
what turns the 13-arm match into thirteen one-line calls, and `push_mesh`'s eight positional
parameters (four of its five callers dropped its second return value) into a `MeshOpts` with a named
`allow_open`. The shell split is the same idea for the browser side: fetching, decoding and
input each get a file, and `lib.rs` only wires events to `State`.

## Files

| file | change | lines after |
|---|---|---|
| `src/app/manifest.rs`, `knobs.rs` | created | 71 · 46 |
| `src/app/walk/encode.rs`, `mesh_topology.rs`, `mesh.rs`, `mesh_ink.rs` | created | 79 · 128 · 186 · 197 |
| `src/app/walk/brep.rs`, `surface.rs`, `curves.rs`, `points.rs`, `frames.rs`, `cloud.rs` | created | 16 · 17 · 85 · 21 · 51 · 114 |
| `src/app/walk/bounds.rs`, `mod.rs` | created | 149 · 125 |
| `src/app/fetch.rs`, `decode.rs`, `stream.rs`, `loader.rs`, `input.rs` | created | 82 · 116 · 120 · 218 · 121 |
| `src/app/scene.rs` | rewritten | 212 (was 1327) |
| `src/app/mod.rs`, `src/lib.rs` | rewritten | 14 · 150 (was 522) |
| `src/app/persistence.rs` | deleted | — |
| `src/math.rs`, `src/state.rs`, `src/selftest.rs`, `examples/*` | edited | — |

Steps 1-19 create files that name types from Steps 20 (`FileDoc`, `CloudBegin`) and 25 (`Msg`);
the tree does not compile again until Step 31 is done, and the first `cargo check` is in Check.
Six steps `Create` over an existing file (delete every line, then paste).

## Step 1 — `src/app/manifest.rs`

`Item`, `Manifest`, `auto_grid`, plus `Manifest::place`: the placement-or-grid choice that five
call sites spelled out.

**Create `src/app/manifest.rs`**

```rust
//! The scene manifest: WHICH files a scene is made of and WHERE each one sits. A drawing is
//! authored at its own page origin, so placement has to come from a text file next to the
//! assets (`at` = translation, `xform` = all 16 numbers, neither = the auto-grid); edit,
//! reload, no rebuild. Nothing here touches a kernel object or the GPU.

use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file to load and where to place it. Every file is authored at its
/// own origin, so an item carries `at` or `xform`; with neither it takes an `auto_grid` slot.
#[derive(Deserialize)]
pub struct Item {
    pub file: String,                 // asset path, e.g. "pb/draw_pf_he.pb"
    #[serde(default)]
    pub name: String,                 // display name; empty = use the session's own
    #[serde(default)]
    pub at: Option<[f64; 3]>,         // translation in world units
    #[serde(default)]
    pub xform: Option<[f64; 16]>,     // full 4x4 (wins over `at`); neither = auto_grid
    #[serde(default)]
    pub point_size: f64,              // raw-cloud px for this file; 0 = keep the pb'own
    #[serde(default)]
    pub stream: bool,                 // Range-stream this file's cloud instead of parsing it
    /// Release this file's kernel `Session` after the walk: a sheet is looked at, never picked
    /// or edited, and 10 sheets of `drawings` held 1.2 GB of documents for tables the GPU
    /// already owns (2056 MB -> 899 MB resident, frame byte-identical). Never on a model file.
    #[serde(default)]
    pub display_only: bool,
}

/// The parsed scene file: an ordered list of items, loaded in list order.
#[derive(Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub name: String,
    pub items: Vec<Item>,
}

impl Item {
    /// The placement this item asks for, or `None` when it wants the auto-grid.
    pub fn placement(&self) -> Option<Xform> {
        if let Some(m) = self.xform {
            let mut x = Xform::identity();
            x.m = m;
            return Some(x);
        }
        self.at.map(|a| Xform::translation(a[0], a[1], a[2]))
    }
}

impl Manifest {
    /// JSON first (every existing scene), TOML as the fallback - a .toml manifest gets
    /// real comments and no trailing-comma landmines; both land in the same structs.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
            .or_else(|| std::str::from_utf8(bytes).ok().and_then(|s| toml::from_str(s).ok()))
    }

    /// Where item `i` sits: its own placement, else its `auto_grid` slot on a grid of `cell` steps.
    pub fn place(&self, i: usize, cell: [f64; 2]) -> Xform {
        self.items[i].placement().unwrap_or_else(|| auto_grid(i, self.items.len(), cell))
    }
}

/// Fallback for items with no `at`/`xform`: lay them out on a grid of `cell` steps, in list order.
/// Deliberately dumb - it exists so a manifest can be written one sheet at a time, not as the way
/// a scene is normally described.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation((index % cols) as f64 * cell[0], (index / cols) as f64 * cell[1], 0.0)
}
```

## Step 2 — `src/app/knobs.rs`

The five environment switches, read once, behind named functions.

**Create `src/app/knobs.rs`**

```rust
//! Launch-time harness toggles, each read ONCE per process: an env lookup is a linear scan of
//! the environment block, and a sheet holds tens of thousands of fill meshes - three reads per
//! mesh was ~30 ms on a 33 MB sheet. Presence-only (`VAR=0` still enables). The only
//! `std::env::var` on the app side lives here.

use std::sync::OnceLock;

/// `std::env::var(name).is_ok()`, cached in `slot` on first use.
fn env_flag(name: &str, slot: &'static OnceLock<bool>) -> bool {
    *slot.get_or_init(|| std::env::var(name).is_ok())
}

static PROFILE: OnceLock<bool> = OnceLock::new();

/// VIEWER_PROFILE: print the walk's stage laps to stderr (native harness only).
pub fn profile() -> bool {
    env_flag("VIEWER_PROFILE", &PROFILE)
}

static DROP_SESSIONS: OnceLock<bool> = OnceLock::new();

/// VIEWER_DROP_SESSIONS: force `display_only` on every file - how the number in `Item::display_only` was measured.
pub fn drop_sessions() -> bool {
    env_flag("VIEWER_DROP_SESSIONS", &DROP_SESSIONS)
}

static NO_EDGES: OnceLock<bool> = OnceLock::new();

/// VIEWER_NO_EDGES: no wireframe, no dots, no mesh bounds - the walk stops before topology.
pub fn no_edges() -> bool {
    env_flag("VIEWER_NO_EDGES", &NO_EDGES)
}

static NO_DOTS: OnceLock<bool> = OnceLock::new();

/// VIEWER_NO_DOTS: pipes but no vertex dots, so a dense wireframe's ink can be split by lane.
pub fn no_dots() -> bool {
    env_flag("VIEWER_NO_DOTS", &NO_DOTS)
}

static ALL_EDGES: OnceLock<bool> = OnceLock::new();

/// VIEWER_ALL_EDGES: keep the coplanar interior edges the wireframe normally culls.
pub fn all_edges() -> bool {
    env_flag("VIEWER_ALL_EDGES", &ALL_EDGES)
}
```

## Step 3 — `src/app/walk/encode.rs`

The row encoders: pen width, packed colour, octahedral normals, the facing word.

**Create `src/app/walk/encode.rs`**

```rust
//! Row encodings shared by every producer: pen widths to radii, colours to RGBA8, normals to
//! oct16 and the packed `facing` word the ink shaders test. Pure functions on numbers - no
//! kernel type, no table.

/// An authored width (kernel millimetres) as the world-mm RADIUS the shaders project; the
/// untouched 1.0 default (and 0 / non-finite) is 0.0 = the screen-constant pen. A negative
/// value would mean "multiply the global pen", which is how a 30 mm polyline once drew 120 px wide.
pub fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 {
        (w as f32) * 0.5
    } else {
        0.0
    }
}

/// One colour channel to a byte, rounded.
fn quant8(v: f32) -> u32 {
    ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff
}

/// RGBA8 in one word, low byte red - the layout `unpack4x8unorm` expects in WGSL.
pub fn pack_rgba(c: [f32; 4]) -> u32 {
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}

/// `signum` that never returns 0: `f64::signum(0.0)` is 0.0, which folds (0,0,-1) onto the
/// code for (0,0,+1) - on an axis-aligned box that is the top and bottom faces, and the
/// collision landed on the "no adjacency" sentinel, so the facing test silently did nothing.
fn sign_not_zero(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

/// One octahedral coordinate to a signed byte.
fn quant_snorm8(v: f64) -> u32 {
    (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff
}

/// A unit vector in 16 bits, octahedral: project onto the octahedron, fold the lower hemisphere
/// out across the diagonals, and store the two coordinates as signed bytes. ~1.4 degrees of error,
/// which is generous for a value only ever used for the SIGN of a dot product.
pub fn oct16(n: &[f64; 3]) -> Option<u32> {
    let l = n[0].abs() + n[1].abs() + n[2].abs();
    if !(l > 0.0) {
        return None;
    }
    let (mut x, mut y) = (n[0] / l, n[1] / l);
    if n[2] < 0.0 {
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * sign_not_zero(x), (1.0 - ax) * sign_not_zero(y));
    }
    Some(quant_snorm8(x) | quant_snorm8(y) << 8)
}

/// Opaque black, packed. The wireframe's default pen, and what a dense mesh's edges draw as.
pub const BLACK: u32 = 0xff00_0000;

/// `facing` value meaning "this edge has no adjacency, always draw it". It cannot be 0, the
/// honest encoding of +Z; all four corners of the octahedral square collapse onto -Z, so the
/// all-ones word is a value the encoder can produce but never needs - the one safe sentinel.
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// The two faces an edge belongs to, packed into one word for the shader's facing test;
/// `FACING_UNKNOWN` when neither is known.
pub fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {
    let pair = match (n0, n1) {
        (Some(a), Some(b)) => (oct16(a), oct16(b)),
        // A naked edge is visible whenever its single face is, so duplicating the one normal is
        // the correct answer and needs no special case in the shader.
        (Some(a), None) | (None, Some(a)) => (oct16(a), oct16(a)),
        _ => (None, None),
    };
    match pair {
        (Some(a), Some(b)) => {
            let v = a | b << 16;
            if v == FACING_UNKNOWN { FACING_UNKNOWN } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}
```

## Step 4 — `src/app/walk/mesh_topology.rs`

The fused face pass, with `SlotMap` in place of the `slot` closure.

**Create `src/app/walk/mesh_topology.rs`**

```rust
//! Mesh topology for the ink lanes, fused into one face walk: unique edges with their pen,
//! edge-to-face adjacency, face normals, closedness. Reads a kernel `Mesh`; writes no table.

use std::collections::HashMap;
use session_rust::{Mesh, Tolerance};
use super::encode::{pack_rgba, BLACK};

/// Vertex key -> slot (the key's position in the sorted `m.vertices()` order). Keys are
/// arbitrary usizes but in practice dense ids, so a Vec indexed BY KEY (u32::MAX = unused)
/// makes every lookup an array read; a sparse key space (a mesh after deletions) takes the map.
pub struct SlotMap {
    dense: Vec<u32>,
    sparse: HashMap<usize, u32>,
}

impl SlotMap {
    /// Dense when the largest key is under four times the vertex count.
    pub fn new(keys: &[usize]) -> Self {
        let max_key = keys.last().copied().unwrap_or(0);
        let dense = max_key < 4 * keys.len().max(1);
        let mut dense_vec: Vec<u32> = Vec::new();
        let mut sparse: HashMap<usize, u32> = HashMap::new();
        if dense {
            dense_vec = vec![u32::MAX; max_key + 1];
            for (s, &k) in keys.iter().enumerate() { dense_vec[k] = s as u32; }
        } else {
            sparse = keys.iter().enumerate().map(|(s, &k)| (k, s as u32)).collect();
        }
        Self { dense: dense_vec, sparse }
    }

    /// The slot of `key`. Dense path first: it is the one every CAD mesh takes.
    pub fn slot(&self, key: usize) -> usize {
        if !self.dense.is_empty() { self.dense[key] as usize } else { self.sparse[&key] as usize }
    }
}

/// Everything the ink lanes need from a mesh's faces, built in ONE pass: the unique edges with
/// their pen, each edge's two faces, the face normals, and whether the mesh is closed. The kernel
/// answers these in four passes (123 ms of the bunny's 137 ms walk); same order, same rules, same bytes.
pub struct MeshTopo {
    /// Unique edges as (low, high) vertex key + PACKED pen color, in `edges_with_colors` order -
    /// a kernel `Color` carries a String and a guid, and cloning one per edge was 104k allocations.
    pub edges: Vec<(usize, usize, u32)>,
    /// Per edge: the face walking (low, high) and the face walking (high, low), as SLOTS into
    /// `normals` (u32::MAX = none); a lone face always lands in slot 0.
    pub edge_faces: Vec<[u32; 2]>,
    /// Per face slot, in sorted-face-key order. `None` for a degenerate face.
    pub normals: Vec<Option<[f64; 3]>>,
    /// Every edge walked in BOTH directions, i.e. no border. Meshes with declared hole rings fall
    /// back to the kernel, which knows that a ring's own edges are not borders.
    pub closed: bool,
}

/// One face's normal, from the by-slot position table - no `Point`, no `Vector`, no allocation
/// and no map lookup. Same arithmetic and the same `ZERO_TOLERANCE` cut-off as `Mesh::face_normal`.
pub fn face_normal_raw(vs: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> Option<[f64; 3]> {
    if vs.len() < 3 { return None }
    let (p0, p1, p2) = (vpos[slots.slot(vs[0])], vpos[slots.slot(vs[1])], vpos[slots.slot(vs[2])]);
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

/// The fused pass. No hash table: edges hang off their LOW vertex on an intrusive chain (`head`
/// per vertex slot, `next` per edge), so "does (lo, hi) exist" is a walk of the two or three
/// edges sharing `lo` - array reads, and deterministic where a HashMap's order is seeded.
pub fn mesh_topology(m: &Mesh, keys: &[usize], vpos: &[[f64; 3]], slots: &SlotMap) -> MeshTopo {
    // SORTED (key, vertex list) pairs: `m.face` is a HashMap whose order changes between runs,
    // and the pen colors and packed `facing` words must come out reproducible.
    let mut faces: Vec<(usize, &Vec<usize>)> = m.face.iter().map(|(k, v)| (*k, v)).collect();
    faces.sort_unstable_by_key(|f| f.0);
    let cols = m.get_linecolors();

    let mut normals: Vec<Option<[f64; 3]>> = Vec::with_capacity(faces.len());
    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    let mut edge_faces: Vec<[u32; 2]> = Vec::new();
    let mut head: Vec<u32> = vec![u32::MAX; keys.len()];
    let mut next: Vec<u32> = Vec::new();

    for (fs, (_, vs)) in faces.iter().enumerate() {
        normals.push(face_normal_raw(vs, vpos, slots));
        let n = vs.len();
        for i in 0..n {
            let (u, v) = (vs[i], vs[(i + 1) % n]);
            // dir 0 = this face walks the edge low -> high, dir 1 = high -> low. The two are the
            // two SIDES of the edge, which is exactly what the facing test needs.
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
            // FIRST face wins, like the kernel's `or_insert`: on a non-manifold patch two faces
            // walk the same directed edge, and last-wins would make `facing` visit-order dependent.
            let f = &mut edge_faces[ei as usize][dir];
            if *f == u32::MAX { *f = fs as u32; }
        }
    }

    // The chain is only ever used for lookup: `edges` was built in first-seen order above, which
    // is `edges_with_colors`' order and what the pen colors are indexed by. Nothing to re-sort.
    let mut closed = !m.vertex.is_empty();
    for f in edge_faces.iter_mut() {
        if f[0] == u32::MAX || f[1] == u32::MAX { closed = false }
        // A lone face moves to slot 0, so a border edge's single normal is always `normal_of(0)`.
        if f[0] == u32::MAX { f[0] = f[1]; f[1] = u32::MAX; }
    }
    // A declared hole ring's edges are borders by this test but not by the kernel's, and only
    // the kernel knows the rings (rare: PDF poche fills, which return before this pass anyway).
    if !closed && !m.face_holes.is_empty() { closed = m.is_closed(); }

    MeshTopo { edges, edge_faces, normals, closed }
}
```

## Step 5 — `src/app/walk/mesh.rs`

`walk_mesh`: faces, the local box, the dense/print/no-edges gates, then the ink pass. `MeshOpts`
names what the five callers used to pass positionally; `Lap` is the profiling clock, a no-op on
wasm.

**Create `src/app/walk/mesh.rs`**

```rust
//! One mesh into the tables: its faces into the arena, its local box, then the ink pass
//! (`mesh_ink`) unless the mesh is dense, a print fill, or edges are switched off. The gates,
//! the thresholds and the walk's profiling clock live here. Nothing here reads the GPU.

use session_rust::Mesh;
use crate::app::knobs;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::Instance;
use crate::math::grow_bounds;
use super::{Row, WalkCx};
use super::mesh_ink::{edges_and_dots, Ink, InkCx};
use super::mesh_topology::{mesh_topology, SlotMap};

/// Above this many triangles a mesh draws as TRIANGLES ONLY - no per-edge cylinder, no
/// per-vertex sphere. At 200k the bunny (69k tri) keeps its wireframe, the armadillo and the
/// dragon do not - the honest line until an impostor makes the decoration cheap.
pub const MESH_RAW_MIN: usize = 200_000;

/// At or above this many edges a mesh's wireframe draws BLACK whatever the file says: at scan
/// density a pen is a property of the tessellation. 104,288 on the bunny; 12 on a box, whose
/// authored red pen always survives.
pub const WIREFRAME_BLACK_MIN: usize = 10_000;

/// Typical distance between a mesh's vertices, world units: the AABB diagonal over the square
/// root of the vertex count (a surface spreads its vertices over an AREA). The ink lanes drop
/// their markers once it projects below a few pixels - see WIRE_MIN_PX in ribbon.wgsl.
fn mesh_spacing(bounds: Option<([f32; 3], [f32; 3])>, verts: usize) -> f32 {
    let Some((lo, hi)) = bounds else { return 0.0 };
    if verts < 2 {
        return 0.0;
    }
    let d = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    d / (verts as f32).sqrt()
}

/// A fill (every PDF glyph, every poche region) broadcasts a single width of 0 - print, not
/// surface. One test drives the wireframe skip, the index run AND `FLAG_PRINT` (flat lighting,
/// so the sheet reads the same from the back), so the three cannot drift apart.
pub fn is_print_fill(m: &Mesh) -> bool {
    m.widths().len() == 1 && m.widths()[0] == 0.0
}

/// Two faces count as one flat region above this normal dot, so the edge between them is
/// interior tessellation. EXACT coplanarity: 0.9999 (0.81 deg) silently ate 14,644 of the
/// bunny's 104,288 edges - curvature is not tessellation, and same-plane normals agree to ULPs.
pub const COPLANAR_DOT: f64 = 1.0 - 1e-9;

/// How one mesh is walked: its object row, the arena rows already on the GPU, and the two
/// choices the caller makes - whether a print fill takes the sheet index runs (and `FLAG_PRINT`),
/// and whether an open mesh may raise `FLAG_OPEN`.
pub struct MeshOpts {
    pub row: u32,
    pub base_off: u32,
    pub sheet_lanes: bool,
    pub allow_open: bool,
}

impl MeshOpts {
    /// A `Mesh` object: print fills take the sheet runs; `allow_open` is the caller's call.
    pub fn sheet(cx: &WalkCx, allow_open: bool) -> Self {
        Self { row: cx.row, base_off: cx.vert_base, sheet_lanes: true, allow_open }
    }

    /// A tessellated BRep or surface: always the depth-tested run, never FLAG_OPEN.
    pub fn model(cx: &WalkCx) -> Self {
        Self { row: cx.row, base_off: cx.vert_base, sheet_lanes: false, allow_open: false }
    }
}

/// The clock behind VIEWER_PROFILE: `mark` prints the lap since the previous mark. A no-op on
/// wasm32, where `Instant::now()` PANICS ("time not implemented") - and this runs per mesh.
#[cfg(not(target_arch = "wasm32"))]
pub struct Lap {
    on: bool,
    at: std::time::Instant,
    prefix: &'static str,
}

/// The browser build's clock: nothing to read, nothing to print.
#[cfg(target_arch = "wasm32")]
pub struct Lap;

#[cfg(not(target_arch = "wasm32"))]
impl Lap {
    /// Start the clock; `prefix` names the caller in every printed line.
    pub fn start(prefix: &'static str) -> Self {
        Self { on: knobs::profile(), at: std::time::Instant::now(), prefix }
    }

    /// Print the lap since the previous mark under `name`, then restart.
    pub fn mark(&mut self, name: &str) {
        if self.on {
            eprintln!("  {} {name:<20} {:?}", self.prefix, self.at.elapsed());
            self.at = std::time::Instant::now();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Lap {
    /// No clock on wasm32.
    pub fn start(_prefix: &'static str) -> Self {
        Self
    }

    /// No clock on wasm32.
    pub fn mark(&mut self, _name: &str) {}
}

/// The object row a mesh reports: its local box, its vertex spacing and the flags it earned.
fn mesh_row(bounds: Option<([f32; 3], [f32; 3])>, m: &Mesh, flags: u32) -> Row {
    Row::solid(bounds, mesh_spacing(bounds, m.number_of_vertices()), flags)
}

/// Which index run a mesh's triangles join decides WHEN it is drawn: sheet fills composite in
/// document order with no depth arbitration, and lettering ("text", named by the PDF importer)
/// goes last of all, after the ink lanes. Everything else takes the depth-tested `idx` run.
fn index_run<'a>(arena: &'a mut ArenaRows, m: &Mesh, sheet: bool) -> &'a mut Vec<u32> {
    if !sheet {
        return &mut arena.idx;
    }
    if m.name == "text" { &mut arena.idx_text } else { &mut arena.idx_print }
}

/// Faces into the arena, the mesh-local box, then edges and dots unless a gate says no. A dense
/// mesh, a print fill and VIEWER_NO_EDGES stop after the faces and report NO box: they emit
/// no ink, so nothing would read `FLAG_INSIDE` - and a print fill still gets `FLAG_PRINT`.
pub fn walk_mesh(arena: &mut ArenaRows, ink: &mut Ink, m: &Mesh, o: &MeshOpts) -> Row {
    let base = o.base_off + arena.verts.len() as u32; // GPU rows already uploaded + rows pending in this delta
    let mut lap = Lap::start("push_mesh");
    let rm = m.to_render();
    lap.mark("to_render");

    // The mesh-local AABB rides the object row, so the edge lanes can be told "the eye is inside
    // this solid" (FLAG_INSIDE) - the facing cull's premise, both faces away = hidden, holds
    // only for an eye OUTSIDE. Reported only when the mesh actually draws ink (see the gates).
    let mut lo = [f32::INFINITY; 3];
    let mut hi = [f32::NEG_INFINITY; 3];
    for v in &rm.vertices{
        grow_bounds(&mut lo, &mut hi, v.position);
        arena.verts.push(*v);
        arena.vids.push(o.row);
    }
    let local_bounds = if lo[0] <= hi[0] { Some((lo, hi)) } else { None };
    let print = is_print_fill(m);
    let idx = index_run(arena, m, o.sheet_lanes && print);
    for &i in &rm.indices{
        idx.push(base+i);
    }
    lap.mark("vert+idx push");
    let flags = if o.sheet_lanes && print { Instance::FLAG_PRINT } else { 0 };

    // A DENSE mesh gets no wireframe and no vertex dots: on the Stanford ladder (1.29M tris)
    // the cylinders and spheres were 90x the geometry they decorated, 118 MB of tables and a
    // 12.4 s walk. Picking reads the kernel Mesh, never these rows, so selection is unaffected.
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return mesh_row(None, m, flags);
    }

    // A fill (every PDF glyph, every poche region) asks for no wireframe at all. Leave before
    // topology: for sheets of hundreds of thousands of tiny fills that pass was the walk's
    // biggest cost, and every edge it produced was then skipped.
    if print { return mesh_row(None, m, flags) }

    if knobs::no_edges() { return mesh_row(None, m, flags) }

    // Positions by slot from the KERNEL's vertex map, not `rm.vertices` (to_render DUPLICATES
    // vertices for per-face colors), and kept in f64: the face normals come from these, and
    // rounding first would flip a near-degenerate cross product's sign, i.e. a `facing` word.
    let keys = m.vertices();
    let slots = SlotMap::new(&keys);
    let vpos64: Vec<[f64; 3]> = keys.iter().map(|&k| { let v = &m.vertex[&k]; [v.x, v.y, v.z] }).collect();
    let vpos: Vec<[f32; 3]> = vpos64.iter().map(|p| [p[0] as f32, p[1] as f32, p[2] as f32]).collect();

    let topo = mesh_topology(m, &keys, &vpos64, &slots);
    lap.mark("topology");

    let mut cx = InkCx { row: o.row, vpos: &vpos, slots: &slots, lap: &mut lap };
    edges_and_dots(ink, m, &topo, &mut cx);

    // An open mesh (border edges) is not a solid: the facing cull would strip the wireframe off
    // interior surface visible through the hole. The topology pass already knows - an edge
    // walked in one direction IS a border - where `Mesh::is_closed()` was a second full sweep.
    let open = o.allow_open && local_bounds.is_some() && !topo.closed;
    mesh_row(local_bounds, m, if open { flags | Instance::FLAG_OPEN } else { flags })
}
```

## Step 6 — `src/app/walk/mesh_ink.rs`

Edges and dots, writing the solid lane through `Ink { seg, glyph }`.

**Create `src/app/walk/mesh_ink.rs`**

```rust
//! The ink a mesh wears: one pipe per visible edge, one sphere per vertex - the SOLID lane.
//! Reads the fused topology (`mesh_topology`) and the positions by slot; writes `SegRows.pipes`
//! and `GlyphRows.spheres`, nothing else. The face pass and the gates are `mesh.rs`.

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

/// What the ink pass needs from the face pass: the object row, the f32 positions by slot, the
/// key -> slot map and the profiling clock.
pub struct InkCx<'a> {
    pub row: u32,
    pub vpos: &'a [[f32; 3]],
    pub slots: &'a SlotMap,
    pub lap: &'a mut Lap,
}

/// Edge `i`'s pen width. A single entry broadcasts to every edge (one number instead of one
/// per edge - lean .pb for thousands of glyph meshes); an absent entry is the 1.0 default.
fn width_at(w: &[f64], i: usize) -> f64 {
    if w.len() == 1 {
        w[0]
    } else {
        w.get(i).copied().unwrap_or(1.0)
    }
}

/// Width 0 = hidden: a triangulated PDF fill asks for no wireframe at all.
fn hidden(w: &[f64], i: usize) -> bool {
    width_at(w, i) == 0.0
}

/// The normal in face slot `k` of an edge's pair, borrowed - a kernel `Vector` carries a
/// String and a guid, and cloning one here was two heap allocations per edge.
fn normal_of(topo: &MeshTopo, f: [u32; 2], k: usize) -> Option<&[f64; 3]> {
    if f[k] == u32::MAX { None } else { topo.normals[f[k] as usize].as_ref() }
}

/// Append edge `ei`'s faces to `fkeys`, deduped; the face lists are cached from the topology
/// pass instead of a kernel call per incident edge.
fn push_faces(edge_faces: &[[u32; 2]], ei: usize, fkeys: &mut Vec<usize>) {
    for &f in edge_faces[ei].iter() {
        if f == u32::MAX { continue }
        let fk = f as usize;
        if !fkeys.contains(&fk) { fkeys.push(fk); }
    }
}

/// Word `k` of a dot's facing triple, by pack_facing's rules: a lone normal is duplicated, none
/// at all is FACING_UNKNOWN, and a pair colliding with the sentinel collapses to it.
fn facing_word(codes: &[u32], k: usize) -> u32 {
    match (codes.get(2 * k).copied(), codes.get(2 * k + 1).copied()) {
        (Some(a), b) => {
            let v = a | b.unwrap_or(a) << 16;
            if v == FACING_UNKNOWN { FACING_UNKNOWN } else { v }
        }
        _ => FACING_UNKNOWN,
    }
}

/// The pipe loop, the widest-edge and incidence passes, then the dots (unless VIEWER_NO_DOTS).
/// A dense wireframe (>= WIREFRAME_BLACK_MIN edges) draws BLACK whatever pens the file carries.
pub fn edges_and_dots(ink: &mut Ink, m: &Mesh, topo: &MeshTopo, cx: &mut InkCx) {
    let edges = &topo.edges;
    let edge_faces = &topo.edge_faces;
    let w = m.widths();
    let black_wire = edges.len() >= WIREFRAME_BLACK_MIN;

    for (i, (a, b, col)) in edges.iter().enumerate(){
        let f = edge_faces[i];

        // The two faces sharing this edge, so the shader decides visibility from the geometry
        // instead of the depth buffer: a pen has width, and ink tested against the surface it
        // decorates is either cut by it or floats in front - no offset wins at every slant.
        let facing = pack_facing(normal_of(topo, f, 0), normal_of(topo, f, 1));

        if hidden(w, i) {
            continue
        }

        // Interior tessellation, not an edge of the shape: a diagonal across a flat region
        // shares two COPLANAR faces. A border has one face and a crease two that disagree, so
        // both survive. VIEWER_ALL_EDGES brings the full tessellation back.
        if let (Some(n0), Some(n1)) = (normal_of(topo, f, 0), normal_of(topo, f, 1)) {
            let dot = n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2];
            if dot >= COPLANAR_DOT && !knobs::all_edges() {
                continue
            }
        }
        ink.seg.pipes.push(
            CylinderSegment{
                p0: cx.vpos[cx.slots.slot(*a)],
                radius: encode_width(width_at(w, i)),
                p1: cx.vpos[cx.slots.slot(*b)],
                instance_id: cx.row,
                color: if black_wire { BLACK } else { *col },
                facing,
            }
        )
    }
    cx.lap.mark("pipe loop");

    // Dots follow user-set pointcolors only (`m.vertices()` is sorted, the order to_render
    // indexes them by); the auto-seeded white vec is filtered by the mode gate.
    let pc = m.get_pointcolors();
    let dots_colored = m.color_mode == ColorMode::POINTCOLORS && pc.len() == m.number_of_vertices();

    // A vertex dot must be as fat as its pipes: the kernel has no per-vertex width, so take the
    // widest incident edge and remember WHICH - the dot leads that edge's `facing` adjacency.
    // Sentinel -inf, not 0: widths can be NEGATIVE world-mm radii. One slot per vertex key.
    let nv = cx.vpos.len();
    let mut vbest = vec![(f64::NEG_INFINITY, 0usize); nv];
    for (i, (a, b, _)) in edges.iter().enumerate(){
        if hidden(w, i){ // A vertex whose every edge is hidden gets no dot either
            continue;
        }
        let wi = width_at(w, i);
        for vk in [*a, *b] {
            let e = &mut vbest[cx.slots.slot(vk)];
            if wi > e.0 {
                *e = (wi, i);
            }
        }
    }

    // Incident EDGES per vertex as CSR (degree count, prefix sum, fill). Hidden edges count
    // too: their face can still carry a visible band, and the dot must hug it to stay in front.
    let mut vstart = vec![0u32; nv + 1];
    for (a, b, _) in edges.iter(){
        vstart[cx.slots.slot(*a) + 1] += 1;
        vstart[cx.slots.slot(*b) + 1] += 1;
    }
    for i in 0..nv{
        vstart[i + 1] += vstart[i];
    }
    let mut vinc = vec![0u32; 2 * edges.len()];
    let mut cur = vstart.clone();
    for (i, (a, b, _)) in edges.iter().enumerate(){
        for vk in [*a, *b] {
            let s = cx.slots.slot(vk);
            vinc[cur[s] as usize] = i as u32;
            cur[s] += 1;
        }
    }
    cx.lap.mark("vbest+vedges");

    // VIEWER_NO_DOTS: the harness can then tell how much of a dense wireframe's ink is dots.
    if knobs::no_dots() { return }

    // The row carries up to SIX normals (3 words x oct16 pair): a trihedral corner needs three,
    // and hugging only the widest edge's two lets the third face's band bite a sector out of
    // the disc at grazing slants. Widest edge's pair first, then every other incident edge's.
    let mut fkeys: Vec<usize> = Vec::new();
    let mut codes: Vec<u32> = Vec::new();
    for i in 0..nv{
        let (vw, ei) = vbest[i];
        if vw == f64::NEG_INFINITY { continue }

        fkeys.clear();
        push_faces(edge_faces, ei, &mut fkeys);
        for &j in &vinc[vstart[i] as usize..vstart[i + 1] as usize] {
            push_faces(edge_faces, j as usize, &mut fkeys);
        }
        codes.clear();
        codes.extend(
            fkeys.iter()
                .filter_map(|fk| topo.normals[*fk].as_ref())
                .filter_map(oct16)
                .take(6),
        );
        ink.glyph.spheres.push(
            GlyphPoint {
                center: cx.vpos[i],
                radius: encode_width(vw),
                // No pointcolors -> a fixed near-black marker whatever the pen: the dot must
                // read as a DOT (following the pen hid it on a black-penned cube).
                color: if dots_colored { pc[i].to_f32() } else { [0.1, 0.1, 0.1, 1.0] },
                instance_id: cx.row,
                facing: facing_word(&codes, 0),
                facing_ext: [facing_word(&codes, 1), facing_word(&codes, 2)],
            }
        );
    }
    cx.lap.mark("dots loop");
}
```

## Step 7 — `src/app/walk/brep.rs`

Tessellate, colour, `walk_mesh`.

**Create `src/app/walk/brep.rs`**

```rust
//! A BRep into the tables: tessellate, tint with the surface colour, hand the mesh to
//! `walk_mesh` as a MODEL mesh - no sheet lanes, no FLAG_OPEN (a BRep tessellation is often
//! numerically non-watertight and its solids would lose the facing cull wholesale).

use session_rust::BRep;
use crate::engine::gpu::arena::ArenaRows;
use super::{Row, WalkCx};
use super::mesh::{walk_mesh, MeshOpts};
use super::mesh_ink::Ink;

/// Tessellate and walk.
pub fn walk_brep(arena: &mut ArenaRows, ink: &mut Ink, b: &BRep, cx: &WalkCx) -> Row {
    let mut bm = b.mesh();
    bm.set_objectcolor(b.surfacecolor.clone());
    walk_mesh(arena, ink, &bm, &MeshOpts::model(cx))
}
```

## Step 8 — `src/app/walk/surface.rs`

The same, for a NURBS surface.

**Create `src/app/walk/surface.rs`**

```rust
//! A NURBS surface into the tables: tessellate, tint with its first face colour, hand the mesh
//! to `walk_mesh` as a MODEL mesh - no sheet lanes, no FLAG_OPEN.

use session_rust::NurbsSurface;
use crate::engine::gpu::arena::ArenaRows;
use super::{Row, WalkCx};
use super::mesh::{walk_mesh, MeshOpts};
use super::mesh_ink::Ink;

/// Tessellate and walk.
pub fn walk_surface(arena: &mut ArenaRows, ink: &mut Ink, s: &NurbsSurface, cx: &WalkCx) -> Row {
    let mut sm = s.mesh();
    if let Some(c) = s.facecolors.first() {
        sm.set_objectcolor(c.clone());
    }
    walk_mesh(arena, ink, &sm, &MeshOpts::model(cx))
}
```

## Step 9 — `src/app/walk/curves.rs`

Line, polyline and NURBS curve into ribbons.

**Create `src/app/walk/curves.rs`**

```rust
//! Lines, polylines and NURBS curves into the FLAT ribbon lane: one `CylinderSegment` per
//! span, `FACING_UNKNOWN` because free-standing linework has no adjacent faces and is always
//! drawn. Reads a kernel curve; writes `SegRows.ribbons` only.

use session_rust::{Line, NurbsCurve, Polyline};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use super::Row;
use super::encode::{encode_width, pack_rgba, FACING_UNKNOWN};

/// One ribbon segment.
pub fn walk_line(seg: &mut SegRows, l: &Line, row: u32) -> Row {
    seg.ribbons.push(CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
        instance_id: row,
        color: pack_rgba(l.linecolor.to_f32()),
        facing: FACING_UNKNOWN,
    });
    Row::none()
}

/// One segment per span of the polyline.
pub fn walk_polyline(seg: &mut SegRows, pl: &Polyline, row: u32) -> Row {
    let pts = pl.get_points();
    let color = pack_rgba(pl.linecolor.to_f32());
    seg.ribbons.extend(pts.windows(2).map(|w| CylinderSegment {
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id: row,
        color,
        facing: FACING_UNKNOWN,
    }));
    Row::none()
}

/// Sample the curve into a polyline whose segment count follows its SIZE, then walk that.
pub fn walk_nurbscurve(seg: &mut SegRows, c: &NurbsCurve, row: u32) -> Row {
    // Bounding box of the CONTROL POINTS - cheap, and it bounds the curve (a NURBS curve
    // never leaves its control net), so it stands in for "how big is this curve".
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        if let Some(cv) = c.cv(i) {
            // Rational curves store WEIGHTED CVs [x*w, y*w, z*w, w] - divide by w to get
            // the real point; non-rational (or w=0 guard) uses the coords as-is.
            let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 {
                cv[3]
            } else {
                1.0
            };
            for k in 0..3 {
                lo[k] = lo[k].min(cv[k] / w);
                hi[k] = hi[k].max(cv[k] / w);
            }
        }
    }
    // No CV ever grew the box (empty/invalid curve) -> lo is still MAX: nothing to draw.
    if lo[0] > hi[0] {
        return Row::none();
    }
    // Sample count follows curve SIZE (box diagonal): a 2mm glyph outline gets 4 segments,
    // a metre-long arc ~50 - sqrt scaling, clamped so nothing under- or over-tessellates.
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    // Evaluate the curve at n+1 evenly spaced parameters across its domain [t0, t1] ...
    let (t0, t1) = c.domain();
    let color = pack_rgba(c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]));
    let radius = encode_width(c.width);
    let pts: Vec<[f32; 3]> = (0..=n)
        .map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32())
        .collect();
    // ... then it IS a polyline: consecutive pairs -> segments, same as walk_polyline.
    seg.ribbons.extend(pts.windows(2).map(|w| CylinderSegment {
        p0: w[0],
        radius,
        p1: w[1],
        instance_id: row,
        color,
        facing: FACING_UNKNOWN,
    }));
    Row::none()
}
```

## Step 10 — `src/app/walk/points.rs`

A point into a dot.

**Create `src/app/walk/points.rs`**

```rust
//! A free point into the FLAT glyph lane: one SDF dot, `FACING_UNKNOWN` because it decorates
//! no surface. Writes `GlyphRows.dots` only.

use session_rust::Point;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::GlyphPoint;
use super::Row;
use super::encode::{encode_width, FACING_UNKNOWN};

/// One SDF dot.
pub fn walk_point(glyph: &mut GlyphRows, p: &Point, row: u32) -> Row {
    glyph.dots.push(GlyphPoint {
        center: p.to_f32(),
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id: row,
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; 2],
    });
    Row::none()
}
```

## Step 11 — `src/app/walk/frames.rs`

A plane's square and a box's twelve edges.

**Create `src/app/walk/frames.rs`**

```rust
//! Planes and oriented boxes into the FLAT ribbon lane as their outlines: a 1 m square for a
//! plane, the 12 edges for a box. Writes `SegRows.ribbons` only.

use session_rust::{Plane, Point, Vector, OBB};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use super::Row;
use super::encode::{encode_width, pack_rgba, FACING_UNKNOWN};

/// A plane is infinite - draw a fixed square around its origin, spanned by its x/y axes.
/// Half-extent in world mm (a 1 m square).
const PLANE_SIZE: f64 = 500.0;

/// The square's corner at signs `s` = (sx, sy) along the plane's x/y axes.
fn corner(o: &Point, x: &Vector, y: &Vector, s: [f64; 2]) -> [f32; 3] {
    [0usize, 1, 2].map(|k| (o[k] + (x[k] * s[0] + y[k] * s[1]) * PLANE_SIZE) as f32)
}

/// The four edges of the plane's square.
pub fn walk_plane(seg: &mut SegRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let c = [corner(&o, &x, &y, [1.0, 1.0]), corner(&o, &x, &y, [-1.0, 1.0]), corner(&o, &x, &y, [-1.0, -1.0]), corner(&o, &x, &y, [1.0, -1.0])];
    let color = pack_rgba(pl.linecolor.to_f32());
    let radius = encode_width(pl.width);
    seg.ribbons.extend((0..4).map(|i| CylinderSegment { p0:c[i], radius, p1: c[(i+1) % 4], instance_id: row, color, facing: FACING_UNKNOWN }));
    Row::none()
}

/// A box is its 12 edges: bottom loop, top loop, four verticals - `corners_f32()` orders the
/// bottom face 0-3 and the top 4-7 with i / i+4 vertically aligned. The OBB type carries no
/// pen, so the edges draw black at screen-constant width (radius 0.0 = global default).
pub fn walk_obb(seg: &mut SegRows, b: &OBB, row: u32) -> Row {
    const EDGES: [[usize; 2]; 12] = [
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4],
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7]
    ];

    let c = b.corners_f32();
    seg.ribbons.extend(EDGES.iter().map(|&[i, j]| CylinderSegment { p0: c[i], radius: 0.0, p1: c[j], instance_id: row, color: pack_rgba([0.0, 0.0, 0.0, 1.0]), facing: FACING_UNKNOWN }));
    Row::none()
}
```

## Step 12 — `src/app/walk/cloud.rs`

A point cloud into the three point tables plus its octree nodes.

**Create `src/app/walk/cloud.rs`**

```rust
//! A walked point cloud into the splat lane: three flat point tables, the LOD octree's nodes
//! and ONE draw record per cloud. The per-file point size rides the object row's spacing
//! column. Writes `CloudRows` only.

use session_rust::{PointCloud, SpatialOctree};
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::{CloudDraw, LodNode};
use super::{Row, WalkCx};
use super::encode::oct16;

/// The points, the nodes, then the draw record. `first` is ABSOLUTE, counted from the start of
/// the scene: the GPU table is cumulative while `c.pos` is only this upload's delta.
pub fn walk_cloud(c: &mut CloudRows, pc: &PointCloud, cx: &WalkCx) -> Row {
    let first = cx.cloud_base + (c.pos.len() / 3) as u32;
    let node_first = c.nodes.len() as u32;
    push_points(c, pc);
    let node_count = c.nodes.len() as u32 - node_first;
    c.draws.push(CloudDraw { first, count: pc.len() as u32, instance: cx.row, spacing: cloud_spacing(pc), node_first, node_count });
    let px = if cx.cloud_px > 0.0 { cx.cloud_px } else { pc.point_size as f32 };
    Row::point_size_px(px)
}

/// The point tables and the octree nodes, straight from the kernel's flat arrays rather than
/// get_point/get_color (no per-point allocation).
fn push_points(rows: &mut CloudRows, pc: &PointCloud) {
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    rows.pos.reserve(n*3);
    rows.col.reserve(n);
    rows.nrm.reserve(n);
    // The LOD octree, built ONCE and read twice: `order()` is the permutation that makes every
    // node's points contiguous, the node table is this walk's second output. Root accept
    // spacing = the cube over 64; leaves absorb below 8192 points, so a shallow cloud is one node.
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for i in 0..n {
        for k in 0..3 {
            lo[k] = lo[k].min(coords[i * 3 + k]);
            hi[k] = hi[k].max(coords[i * 3 + k]);
        }
    }
    let size = (hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2]).max(1.0e-9);
    let tree = SpatialOctree::from_coords(coords, size / 64.0, 8192);
    // `first` and `children` are RELATIVE to this cloud's own first point / node slice: the
    // record builder adds the draw's base, so a re-upload at another offset rewrites nothing.
    for ni in 0..tree.node_count() {
        let (center, sz) = tree.node_cube(ni);
        let (f, count) = tree.node_range(ni);
        // `children` hands back only the octants that exist, so the empty slots stay -1 and
        // the record walk skips them; which octant a child was, the screen-error test never asks.
        let mut children = [-1i32; 8];
        for (slot, &ch) in tree.children(ni).iter().enumerate() {
            children[slot] = ch as i32;
        }
        rows.nodes.push(LodNode {
            center: [center[0] as f32, center[1] as f32, center[2] as f32],
            size: sz as f32,
            spacing: tree.node_spacing(ni) as f32,
            first: f as u32,
            count: count as u32,
            children,
        });
    }
    for &i in tree.order(){
        rows.pos.push(coords[i*3] as f32);
        rows.pos.push(coords[i*3+1] as f32);
        rows.pos.push(coords[i*3+2] as f32);

        // oct16 normal; all-ones = none (a scan without them still pays the 4 B, but the
        // shading branch stays uniform per cloud). Three f64s, not a kernel `Vector`: building
        // one per point was two heap allocations each - 27 million on the 13.8 M-point scan.
        rows.nrm.push(if i*3 + 2 < normals.len() {
            oct16(&[normals[i*3], normals[i*3+1], normals[i*3+2]]).unwrap_or(u32::MAX)
        } else {
            u32::MAX
        });
        let c = i * 4;

        // The colour is 8-bit at the source (proto 0-255): pack it back to the four bytes it is.
        rows.col.push(if c + 3 < colors.len() {
            (colors[c] as u32 & 255) | (colors[c + 1] as u32 & 255) << 8 | (colors[c+2] as u32 & 255) << 16 | (colors[c + 3] as u32 & 255) << 24
        } else {
            0xff00_0000
        });
    }
}

/// Median distance between consecutive points (world units): a scanner emits angular
/// neighbours in order, so successive points are usually adjacent on the surface. Potree gets
/// the same number from its octree; we sample it. Drives the attenuated splat radius.
pub fn cloud_spacing(pc: &PointCloud) -> f32 {
    let c = pc.coords();
    let n = pc.len();
    if n < 2 {
        return 20.0;
    }
    let step = (n / 1024).max(1);
    let mut d: Vec<f64> = Vec::with_capacity(1024);
    let mut i = 0;
    while i + 1 < n {
        let  (a, b) = (i * 3, (i + 1) * 3);
        let dd = (c[a] - c[b]).powi(2) + (c[a + 1] - c[b + 1]).powi(2) + (c[a + 2] - c[b + 2]).powi(2);
        if dd> 0.0 {
            d.push(dd.sqrt());
        }
        i += step;
    }
    if d.is_empty() {
        return 20.0;
    }
    d.sort_by(|x, y| x.partial_cmp(y).unwrap());
    d[d.len() / 2] as f32
}
```

## Step 13 — `src/app/walk/bounds.rs`

The two per-file sweeps (extent, sheet thickness) and the sheet marking, over the rows a file
added since `Baselines` were captured.

**Create `src/app/walk/bounds.rs`**

```rust
//! The per-file sweeps over the rows a walk just appended: the world box (scene bounds), the
//! thickness along the sheet normal (the planar test) and the sheet marking. Reads `Upload`
//! from a `Baselines` captured before the walk; never a kernel object.

use session_rust::{Vector, Xform};
use crate::engine::gpu::{CloudDraw, Instance, Upload};
use crate::math::{grow_bounds, xform_point, Aabb};

/// Row counts captured BEFORE a file is walked, so the sweeps read only that file's rows. In
/// the browser every file uploads before the next, so they are 0; batched harness runs make
/// them real. `cloud_base` is what a draw record's absolute `first` counts from.
pub struct Baselines {
    pub vert: usize,
    pub seg: usize,
    pub pipe: usize,
    pub sphere: usize,
    pub glyph: usize,
    pub obj: usize,
    pub draw: usize,
    pub cloud_base: u32,
}

impl Baselines {
    /// Every table's length now.
    pub fn capture(t: &Upload, cloud_base: u32) -> Self {
        Self {
            vert: t.arena.verts.len(),
            seg: t.seg.ribbons.len(),
            pipe: t.seg.pipes.len(),
            sphere: t.glyph.spheres.len(),
            glyph: t.glyph.dots.len(),
            obj: t.obj.rows.len(),
            draw: t.cloud.draws.len(),
            cloud_base,
        }
    }
}

/// The extent of a point set along one direction.
struct Span {
    n: [f32; 3],
    min: f32,
    max: f32,
}

impl Span {
    /// Empty, along `n`.
    fn new(n: &Vector) -> Self {
        Self { n: [n[0] as f32, n[1] as f32, n[2] as f32], min: f32::INFINITY, max: f32::NEG_INFINITY }
    }

    /// Widen by one point.
    fn add(&mut self, p: [f32; 3]) {
        let d = p[0] * self.n[0] + p[1] * self.n[1] + p[2] * self.n[2];
        self.min = self.min.min(d);
        self.max = self.max.max(d);
    }

    /// max - min; non-finite when nothing was added.
    fn width(&self) -> f32 {
        self.max - self.min
    }
}

/// This file's world extent: every new row through its object's placement, so the planar
/// test and the scene bounds see what is actually drawn.
pub fn file_extent(t: &Upload, from: &Baselines) -> Aabb {
    let (mut fmin, mut fmax) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    for (i, v) in t.arena.verts.iter().enumerate().skip(from.vert) {
        if let Some(&ri) = t.arena.vids.get(i) {
            if let Some((xf, _, _)) = t.obj.rows.get(ri as usize) {
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, v.position));
            }
        }
    }

    for s in t.seg.pipes.iter().skip(from.pipe).chain(t.seg.ribbons.iter().skip(from.seg)){
        if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p0));
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p1));
        }
    }

    for s in t.glyph.spheres.iter().skip(from.sphere).chain(t.glyph.dots.iter().skip(from.glyph)){
        if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.center));
        }
    }

    for &CloudDraw { first, count, instance: inst, .. } in t.cloud.draws.iter().skip(from.draw){
        let Some((xf, _, _)) = t.obj.rows.get(inst as usize) else { continue };
        // `first` is absolute; `cloud.pos` starts at the base.
        let cb = from.cloud_base;
        for i in (first - cb) as usize..(first - cb + count) as usize {
            let p = [t.cloud.pos[i*3], t.cloud.pos[i*3+1], t.cloud.pos[i*3 + 2]];
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
        }
    }

    Aabb { min: fmin, max: fmax }
}

/// The file's thickness along the SHEET's normal (the placement's Z). The 99% path - a
/// translation-only placement - reuses the z-extent of `extent`, no extra work; only a
/// rotated placement pays one dot-product pass over this file's rows (clouds excluded).
pub fn sheet_thickness(t: &Upload, from: &Baselines, place: &Xform, extent: &Aabb) -> f32 {
    let n = place.transform_vector(&Vector::new(0.0, 0.0, 1.0));
    if n[0].abs() < 1e-9 && n[1].abs() < 1e-9 {
        return extent.max[2] - extent.min[2];
    }
    let mut span = Span::new(&n);
    for (i, v) in t.arena.verts.iter().enumerate().skip(from.vert){
        if let Some(&ri) = t.arena.vids.get(i){
            if let Some((xf, _, _)) = t.obj.rows.get(ri as usize) {
                span.add(xform_point(xf, v.position));
            }
        }
    }
    for s in t.seg.pipes.iter().skip(from.pipe).chain(t.seg.ribbons.iter().skip(from.seg)){
        if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
            span.add(xform_point(xf, s.p0));
            span.add(xform_point(xf, s.p1));
        }
    }
    for g in t.glyph.spheres.iter().skip(from.sphere).chain(t.glyph.dots.iter().skip(from.glyph)){
        if let Some((xf, _, _)) = t.obj.rows.get(g.instance_id as usize) {
            span.add(xform_point(xf, g.center));
        }
    }
    span.width()
}

/// Every row of a planar file is page content: FLAG_SHEET on its objects (the ink lanes drop
/// their lift, which lets the lettering pass sit on top of the linework), and every unset pen
/// becomes a world-mm hairline so widths behave like plotter pens.
pub fn mark_sheet(t: &mut Upload, from: &Baselines) {
    for o in t.obj.rows.iter_mut().skip(from.obj) {
        o.2 |= Instance::FLAG_SHEET;
    }
    for s in t.seg.pipes.iter_mut().skip(from.pipe).chain(t.seg.ribbons.iter_mut().skip(from.seg)){
        // encode_width already returns a positive mm radius for any authored width, so only
        // the unset default (0.0) needs a value: 0.5 mm, the usual hairline.
        s.radius = if s.radius > 0.0 {
            s.radius
        } else {
            0.5
        }
    }
}
```

## Step 14 — `src/app/walk/mod.rs`

`Walk` (the sinks), `WalkCx`, `Row`, `is_drawable` and `walk_geometry`: thirteen arms, one line each.

**Create `src/app/walk/mod.rs`**

```rust
//! The walk: one producer per kernel geometry type, each receiving ONLY the row families it
//! writes (which shaders a type can reach is readable off its signature). `walk_geometry`
//! dispatches; `Row` is what a producer hands back for its object row - producers never push one.

use session_rust::Geometry;
use session_rust::element::ElementGeometry;
use crate::engine::gpu::Upload;
use crate::engine::gpu::arena::ArenaRows;
use crate::engine::gpu::cloud::CloudRows;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::objects::ObjectRows;
use crate::engine::gpu::segments::SegRows;
use brep::walk_brep;
use cloud::walk_cloud;
use curves::{walk_line, walk_nurbscurve, walk_polyline};
use frames::{walk_obb, walk_plane};
use mesh::{walk_mesh, MeshOpts};
use mesh_ink::Ink;
use points::walk_point;
use surface::walk_surface;

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

pub use encode::FACING_UNKNOWN;

/// The sinks a producer may write: the five groups of one `Upload`, borrowed for one object.
pub struct Walk<'a> {
    pub obj: &'a mut ObjectRows,
    pub arena: &'a mut ArenaRows,
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
    pub cloud: &'a mut CloudRows,
}

impl<'a> Walk<'a> {
    /// Every group of `t`.
    pub fn of(t: &'a mut Upload) -> Self {
        Self { obj: &mut t.obj, arena: &mut t.arena, seg: &mut t.seg, glyph: &mut t.glyph, cloud: &mut t.cloud }
    }

    /// The SOLID lane a tessellated surface reaches: the arena for its faces, the ink pair
    /// (pipes, spheres) for its edges and dots.
    pub fn solid(&mut self) -> (&mut ArenaRows, Ink<'_>) {
        (self.arena, Ink { seg: self.seg, glyph: self.glyph })
    }
}

/// Where one object's rows land: the arena rows already on the GPU (`walk_mesh` bases its
/// indices on it), the cloud points already uploaded (a draw's `first` counts from it), the
/// file's point-size override in px (0 = the pb's own) and the object row being walked.
pub struct WalkCx {
    pub vert_base: u32,
    pub cloud_base: u32,
    pub cloud_px: f32,
    pub row: u32,
}

/// What a producer reports for its object row: the mesh-local box (meshes that drew ink only),
/// the point/vertex spacing and the flags it earned. The caller pushes the columns.
pub struct Row {
    pub bounds: Option<([f32; 3], [f32; 3])>,
    pub spacing: f32,
    pub flags: u32,
}

impl Row {
    /// Linework, points, frames: no box, no spacing, no flags.
    pub fn none() -> Self {
        Self { bounds: None, spacing: 0.0, flags: 0 }
    }

    /// A tessellated surface's row.
    pub fn solid(bounds: Option<([f32; 3], [f32; 3])>, spacing: f32, flags: u32) -> Self {
        Self { bounds, spacing, flags }
    }

    /// A cloud's row: the per-file point size rides the spacing column.
    pub fn point_size_px(px: f32) -> Self {
        Self { bounds: None, spacing: px, flags: 0 }
    }
}

/// An `Element` with no geometry gets no row at all; everything else does.
pub fn is_drawable(geom: &Geometry) -> bool {
    match geom {
        Geometry::Element(e) => !matches!(e.geometry(), ElementGeometry::None),
        _ => true,
    }
}

/// One object into the tables. 3D geometry takes the SOLID lane (edges are cylinders,
/// vertices spheres); free linework and points the FLAT lane; every cloud the splat lane.
/// FLAG_OPEN for `Mesh` objects only - an Element's mesh never raised it.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
    // 3D geometry takes the SOLID lane (edges are cylinders, vertices spheres); free
    // linework and points the FLAT lane; every cloud the splat lane. FLAG_OPEN for
    // `Mesh` objects only - an Element's mesh never raised it.
    match geom {
        Geometry::Mesh(m) => { let (arena, mut ink) = w.solid(); walk_mesh(arena, &mut ink, m, &MeshOpts::sheet(cx, true)) }
        Geometry::BRep(b) => { let (arena, mut ink) = w.solid(); walk_brep(arena, &mut ink, b, cx) }
        Geometry::Line(l) => walk_line(w.seg, l, cx.row),
        Geometry::Polyline(pl) => walk_polyline(w.seg, pl, cx.row),
        Geometry::NurbsCurve(c) => walk_nurbscurve(w.seg, c, cx.row),
        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),
        Geometry::PointCloud(pc) => walk_cloud(w.cloud, pc, cx),
        Geometry::NurbsSurface(s) => { let (arena, mut ink) = w.solid(); walk_surface(arena, &mut ink, s, cx) }
        Geometry::Plane(p) => walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) => walk_obb(w.seg, b, cx.row),
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(m) => { let (arena, mut ink) = w.solid(); walk_mesh(arena, &mut ink, m, &MeshOpts::sheet(cx, false)) }
            ElementGeometry::BRep(b) => { let (arena, mut ink) = w.solid(); walk_brep(arena, &mut ink, b, cx) }
            ElementGeometry::None => Row::none(),
        },
    }
}
```

## Step 15 — `src/app/fetch.rs`

Whole-file and range fetches, and the macrotask yield.

**Create `src/app/fetch.rs`**

```rust
//! The browser's network edge (wasm32 has no filesystem, so `fetch()` is the only way to a
//! .pb or a manifest): whole-file GETs and HTTP Range reads, both started EAGERLY so the next
//! request travels while the current bytes parse, plus `next_tick`, the macrotask that lets
//! the browser paint between slices.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

/// A request already IN FLIGHT: the browser's fetch() promise is eager, only the Rust await is
/// lazy - so starting the next file's fetch before parsing the current one overlaps network
/// with parse (State::new pipelines with a window of 2).
pub struct Fetch { fut: JsFuture }

/// Start a same-origin GET without awaiting it.
pub fn fetch_start(url: &str) -> Result<Fetch, JsValue>{
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(Fetch { fut: JsFuture::from(window.fetch_with_request(&request)) })
}

/// Await one started request's whole body.
pub async fn fetch_finish(f: Fetch) -> Result<Vec<u8>, JsValue>{
    let resp: Response = f.fut.await?.dyn_into()?;
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// GET 'url' - trunk-served, same origin as the page and return raw bytes.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue>{
    fetch_finish(fetch_start(url)?).await
}

/// Start a range request WITHOUT awaiting it. `window.fetch()` is eager - the browser has the
/// request in flight the moment this returns - so the caller can keep slice n+1 travelling
/// while it converts slice n. That overlap is the difference between 11 sequential round trips
/// and 11 hidden ones; see `loader::stream_coords`.
pub fn fetch_range_start(url: &str, start: u64, len: u64) -> Result<Fetch, JsValue> {
    let headers = Headers::new()?;
    headers.set("Range", &format!("bytes={}-{}", start, start + len - 1))?;
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    opts.set_headers(&headers);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(Fetch { fut: JsFuture::from(window.fetch_with_request(&request)) })
}

/// Finish one, insisting on `206` - see `fetch_range`.
pub async fn fetch_range_finish(f: Fetch) -> Result<Vec<u8>, JsValue> {
    let resp: Response = f.fut.await?.dyn_into()?;
    if resp.status() != 206 {
        return Err(JsValue::from_str("server ignored Range (no 206) - refusing to pull the whole body"));
    }
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// GET a byte range. Refuses anything but `206`: a server that ignores `Range` answers `200`
/// with the WHOLE body, which for a 411 MB scan would be catastrophic and silent.
/// `trunk serve` (axum + tower-http) does support ranges; `docs/serve.py`
/// (SimpleHTTPRequestHandler) does NOT.
pub async fn fetch_range(url: &str, start: u64, len: u64) -> Result<Vec<u8>, JsValue> {
    fetch_range_finish(fetch_range_start(url, start, len)?).await
}

/// One macrotask (setTimeout 0). A microtask (Promise.resolve) would NOT let the browser paint.
pub async fn next_tick() {
    let p = js_sys::Promise::new(&mut schedule_macrotask);
    let _ = JsFuture::from(p).await;
}

/// `setTimeout(resolve, 0)` - the executor `Promise::new` wants.
fn schedule_macrotask(resolve: js_sys::Function, _reject: js_sys::Function) {
    web_sys::window().unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
        .unwrap();
}
```

## Step 16 — `src/app/decode.rs`

The chunked proto-to-`Session` conversion.

**Create `src/app/decode.rs`**

```rust
//! The whole-file decode: prost decodes the proto in one short block, then the kernel objects
//! are converted CHUNK at a time with a macrotask between chunks, so a 250k-object parse no
//! longer freezes the page. Reads bytes; hands back a kernel `Session`.

use std::rc::Rc;
use prost::Message;
use session_rust::proto;
use session_rust::{Geometry, Line, Mesh, NurbsCurve, NurbsSurface, OBB, Plane, Point, Polyline, PointCloud, BRep, Element, Session, Xform};
use session_rust::tree::{Tree, TreeNode};
use super::fetch::next_tick;

/// Objects converted per slice before the loader hands the browser one macrotask — the whole
/// point is that a frame can render BETWEEN slices, so a 250k-object parse stops freezing the UI.
const CHUNK: usize = 25_000;

/// `Session::pb_loads`, unrolled with awaits: decode the proto whole (one short block — prost is
/// fast), then convert objects CHUNK at a time. Same result, no multi-second freeze. `.json`
/// files stay on the synchronous path (they are small).
pub async fn session_from_bytes_chunked(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(bytes));
    }
    let Ok(p) = proto::Session::decode(bytes) else { return Session::default() };
    let mut s = Session::new(&p.name);
    s.set_guid(p.guid.clone());

    let mut n = 0usize;
    // The same conversion loop for all 11 types, written once: proto -> object, stored, paused
    // every CHUNK so the browser can paint.
    macro_rules! chunk {
        ($vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let g = Rc::new($ty::from_proto(x));
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
        // from_proto -> Result for the nested types; a bad object is skipped, not fatal
        (fallible $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let Ok(v) = $ty::from_proto(x) else { continue };
                let g = Rc::new(v);
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
    }

    if let Some(o) = p.objects {
        s.objects.set_guid(o.guid);
        s.objects.name = o.name;
        chunk!(o.points, Point, Point, points);
        chunk!(o.lines, Line, Line, lines);
        chunk!(o.planes, Plane, Plane, planes);
        chunk!(fallible o.bboxes, OBB, OBB, bboxes);
        chunk!(o.polylines, Polyline, Polyline, polylines);
        chunk!(o.pointclouds, PointCloud, PointCloud, pointclouds);
        chunk!(o.meshes, Mesh, Mesh, meshes);
        chunk!(o.nurbscurves, NurbsCurve, NurbsCurve, nurbscurves);
        chunk!(fallible o.nurbssurfaces, NurbsSurface, NurbsSurface, nurbssurfaces);
        chunk!(fallible o.breps, BRep, BRep, breps);
        chunk!(fallible o.elements, Element, Element, elements);
    }

    // Xforms first: they decide whether the tree is needed at all.
    for entry in &p.xforms {
        if let Some(xf) = &entry.xform {
            let mut xform = Xform::identity();
            xform.set_guid(xf.guid.clone());
            xform.name = xf.name.clone();
            for (i, val) in xf.matrix.iter().enumerate().take(16) {
                xform.m[i] = *val;
            }
            s.xforms.insert(entry.guid.clone(), xform);
        }
    }

    // The graph is real session data, not scratch: it was being decoded and dropped.
    if let Some(gp) = &p.graph {
        s.graph = session_rust::Graph::new(&gp.name);
        s.graph.set_guid(gp.guid.clone());
        for (name, v) in &gp.vertices {
            s.graph.add_node(name, &v.attribute);
        }
        for e in &gp.edges {
            s.graph.add_edge(&e.v0, &e.v1, &e.attribute);
        }
    }

    // The tree comes from the same decode as everything else. It used to be skipped and then
    // re-decoded by a second mirror struct; a Session that loads its own tree is both simpler
    // and honest about what it holds.
    if let Some(tp) = &p.tree {
        s.tree = Tree::new(&tp.name);
        s.tree.set_guid(tp.guid.clone());
        if let Some(rp) = &tp.root {
            /// One proto node and its children, recursively.
            fn build(proto: &proto::TreeNode) -> Rc<std::cell::RefCell<TreeNode>>{
                let node = TreeNode::new(&proto.name);
                for c in &proto.children {
                    let child = build(c);
                    node.borrow_mut().add(&child);
                }
                node
            }
            let root = build(rp);
            s.tree.add(&root, None);
        }
    }

    s
}
```

## Step 17 — `src/app/stream.rs`

The wire-format reader that finds a cloud's packed arrays without decoding the file.

**Create `src/app/stream.rs`**

```rust
//! Streaming a point cloud: HTTP Range in, GPU rows out, nothing large in between - the
//! whole-file path peaks at bytes + proto + kernel object + rows, this one never holds more
//! than a slice. Two wire facts make it possible (checked on a real scan): every hop
//! Session.3 -> Objects.8 -> PointCloud.3/.4 is length-delimited, and `coords` is packed
//! DOUBLE, so its length prefix gives the exact point count before a byte of payload is read.

use super::fetch::fetch_range;

/// Where the two packed arrays live in the file, as absolute byte offsets.
pub struct CloudFields {
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,
    pub count: u32,
}

/// One protobuf varint. Returns the value and how many bytes it ate.
fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;
    loop {
        let byte = *b.get(i)?;
        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;
        if byte & 0x80 == 0 { return Some((v, i - start)) }
        shift += 7;
        if shift > 63 { return None }
    }
}

/// Walk `head` (the first few KB of the file) down Session.3 -> Objects.8 -> PointCloud, and
/// report where `coords` starts. Descends into exactly the three fields it cares about and
/// skips every other one by its length - no allocation, no decoding.
///
/// Returns `None` for anything that is not a single-cloud file, which is the signal to fall
/// back to the whole-file prost path.
fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let mut i = 0usize;
    let mut end = head.len();
    for want in [3u32, 8u32] {
        let mut found = false;
        while i < end {
            let (tag, n) = varint(head, i)?;
            i += n;
            let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
            if wire != 2 { return None } // every hop we care about is length-delimited
            let (len, n) = varint(head, i)?;
            i += n;
            if field == want { end = i + len as usize; found = true; break }
            i += len as usize; // skip this sub-message whole
        }
        if !found { return None }
    }
    // inside PointCloud now: find field 3 (coords)
    while i < end {
        let (tag, n) = varint(head, i)?;
        i += n;
        let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
        if wire != 2 {
            // point_size is a fixed64, everything else we skip by wire type
            i += match wire { 0 => varint(head, i)?.1, 1 => 8, 5 => 4, _ => return None };
            continue;
        }
        let (len, n) = varint(head, i)?;
        i += n;
        if field == 3 { return Some((i as u64, len)) }
        if field == 4 { return None } // colours before coords - not a layout we can size from
        i += len as usize;
    }
    None
}

/// Convert one already-fetched coords slice to f32 triples.
pub fn positions_from(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 8);
    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()) as f32);
    }
    out
}

/// Locate both packed arrays with two small reads: one at the head for `coords`, then one at
/// the end of the coords payload, where the `colors` header must be.
pub async fn cloud_fields(url: &str) -> Option<CloudFields> {
    let head = fetch_range(url, 0, 8192).await.ok()?;
    let (coords_at, coords_len) = walk_to_coords(&head)?;
    if coords_len == 0 || coords_len % 24 != 0 { return None }

    let hdr = fetch_range(url, coords_at + coords_len, 16).await.ok()?;
    let (tag, n) = varint(&hdr, 0)?;
    if (tag >> 3) != 4 || (tag & 7) != 2 { return None } // expected the colours field next
    let (colors_len, n2) = varint(&hdr, n)?;
    Some(CloudFields {
        coords_at,
        coords_len,
        colors_at: coords_at + coords_len + (n + n2) as u64,
        colors_len,
        count: (coords_len / 24) as u32,
    })
}

/// Read the whole `colors` run and pack it to RGBA8. Packed uint32 is VARINT on the wire - not
/// memcpy-able the way `coords` is - so this decodes sequentially. It is 27 MB against the
/// coords' 87 MB, and taking it in one piece buys complete freedom from split-varint handling.
pub async fn cloud_colors(url: &str, at: u64, len: u64, count: u32) -> Option<Vec<u32>> {
    let raw = fetch_range(url, at, len).await.ok()?;
    let mut out = Vec::with_capacity(count as usize);
    let mut i = 0usize;
    for _ in 0..count {
        let mut rgba = [255u8; 4];
        for k in 0..4 {
            let (v, n) = varint(&raw, i)?;
            i += n;
            rgba[k] = (v & 255) as u8;
        }
        out.push(u32::from_le_bytes(rgba));
    }
    Some(out)
}
```

## Step 18 — `src/app/loader.rs`

The manifest loader: the whole-file path and the streaming path, producing `Msg`s; it touches
no GPU.

**Create `src/app/loader.rs`**

```rust
//! The async manifest loader: fetch the manifest, bring the canvas up EMPTY, then post every
//! item to the event loop as a `Msg` - whole files through `decode` (prefetched one ahead),
//! `stream` items through `stream` as 8 MB slices. Touches no GPU. `reload_scene` re-enters
//! here from the page with the `State` kept.

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use session_rust::Xform;
use crate::{Msg, State};
use crate::engine::performance::now_ms;
use crate::math::Aabb;
use super::decode::session_from_bytes_chunked;
use super::fetch::{fetch_bytes, fetch_finish, fetch_range_finish, fetch_range_start, fetch_start, next_tick, Fetch};
use super::manifest::{Item, Manifest};
use super::scene::{CloudBegin, FileDoc, Scene};
use super::stream::{cloud_colors, cloud_fields, positions_from, CloudFields};

/// The scene when the page names none: fetched at runtime, so re-arranging it is a text edit
/// in assets/scenes, not a rebuild.
const DEMO_SCENE_URL: &str = "scenes/bunny_drawings.toml";

/// The manifest to load: `?scene=<path under assets/>` when the page supplies one, else
/// [`DEMO_SCENE_URL`]. One build can therefore serve many scenes - the docs embed a single
/// 7.7 MB wasm in an iframe per example and vary only the query string.
///
/// The value is a path under `assets/`, exactly like a manifest's own `file` entries. It is
/// rejected unless it stays inside that tree: an absolute URL, a scheme, or any `..` segment
/// would let a page point the viewer at another origin.
fn scene_url() -> String {
    /// The `scene=` query value, when present and inside `assets/`.
    fn from_query() -> Option<String> {
        let search = web_sys::window()?.location().search().ok()?;
        let raw = search.strip_prefix('?')?;
        let value = raw
            .split('&')
            .find_map(|pair| pair.strip_prefix("scene="))?;
        let decoded = js_sys::decode_uri_component(value).ok()?.as_string()?;
        let safe = !decoded.is_empty()
            && !decoded.starts_with('/')
            && !decoded.contains("//")
            && !decoded.contains(':')
            && !decoded.split('/').any(|seg| seg == "..");
        safe.then_some(decoded)
    }
    from_query().unwrap_or_else(|| DEMO_SCENE_URL.to_string())
}

thread_local! {
    /// A proxy kept past start-up so [`reload_scene`] can post files into the
    /// running event loop. `resumed` takes `self.proxy`, so without this copy
    /// there is no way back into the app once it is going.
    static RELOAD_PROXY: std::cell::RefCell<Option<winit::event_loop::EventLoopProxy<Msg>>> =
        const { std::cell::RefCell::new(None) };
}

/// Keep the start-up proxy so [`reload_scene`] can post into the running event loop.
pub fn keep_proxy(proxy: &EventLoopProxy<Msg>) {
    RELOAD_PROXY.with(|slot| *slot.borrow_mut() = Some(proxy.clone()));
}

/// Reload the scene in place: same canvas, same camera, new geometry.
///
/// The page calls this after rewriting a `.pb` (see the docs' Thebe cells) so an
/// edit redraws the MODEL instead of restarting the viewer - reloading the
/// iframe would rebuild the WebGPU device and throw away the view you had
/// framed. `url` is a manifest path under `assets/`, as with `?scene=`.
#[wasm_bindgen]
pub fn reload_scene(url: Option<String>) {
    let proxy = RELOAD_PROXY.with(|slot| slot.borrow().clone());
    let Some(proxy) = proxy else {
        log::warn!("reload_scene: viewer is not running yet");
        return;
    };
    let url = url.unwrap_or_else(scene_url);
    wasm_bindgen_futures::spawn_local(async move {
        let _ = proxy.send_event(Msg::Clear);
        load_manifest(url, &proxy).await;
    });
}

/// Fetch a manifest and post every parsed file as a `Msg::File`, in manifest order - the
/// [`reload_scene`] path: no prefetch, no streaming, the `State` already exists.
async fn load_manifest(url: String, proxy: &EventLoopProxy<Msg>) {
    let manifest_bytes = fetch_bytes(&url).await.unwrap_or_default();
    let Some(manifest) = Manifest::parse(&manifest_bytes) else {
        log::error!("cannot read the scene manifest at {url}");
        return;
    };
    for (i, item) in manifest.items.iter().enumerate() {
        let bytes = fetch_bytes(&item.file).await.unwrap_or_default();
        let session = session_from_bytes_chunked(&item.file, &bytes).await;
        if session.lookup.is_empty() {
            continue;
        }
        let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
        let place = manifest.place(i, [0.0, 0.0]);
        let _ = proxy.send_event(Msg::File(FileDoc { name, session, place, point_px: item.point_size as f32, display_only: item.display_only }));
    }
}

/// Start-up: manifest, `State` around an EMPTY scene (posted as `Msg::Ready`), then every item
/// in manifest order.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    let t0 = now_ms();
    let scene_url = scene_url();
    let manifest_bytes = fetch_bytes(&scene_url).await.unwrap_or_default();
    let manifest = Manifest::parse(&manifest_bytes).unwrap_or_else(|| panic!("cannot read the scene manifest at {scene_url}"));
    log::info!("scene '{}': {} items", manifest.name, manifest.items.len());

    // The canvas and the GPU come up FIRST, empty. A streamed cloud writes into GPU buffers, so
    // the GPU has to exist before the first byte of geometry is fetched - and as a bonus the
    // viewport is live immediately, not after a parse.
    let state = State::new(window, Scene::new()).await.expect("State init failed");
    log::info!("canvas live {:.0}ms after manifest fetch", now_ms() - t0);
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));

    // Pipelined: `fetch_start` is eager, so file n+1 is in flight while file n parses; and
    // progressive: every file streams in as its own `Msg` the moment it is ready.
    let mut next = manifest.items.first().and_then(prefetch);
    for (i, item) in manifest.items.iter().enumerate() {
        let cur = next.take();
        next = manifest.items.get(i + 1).and_then(prefetch);
        let place = manifest.place(i, [0.0, 0.0]);
        if item.stream {
            stream_item(&proxy, item, place).await;
        } else {
            whole_item(&proxy, item, cur, place).await;
        }
    }
}

/// The whole-file prefetch; `stream` items are skipped - a plain GET on a 431 MB scan would
/// pull the entire body.
fn prefetch(it: &Item) -> Option<Result<Fetch, JsValue>> {
    (!it.stream).then(|| fetch_start(&it.file))
}

/// A `stream` cloud never becomes a kernel object and never exists whole in wasm memory: two
/// small Range reads find the packed arrays, the coords run streams, the colours follow whole.
async fn stream_item(proxy: &EventLoopProxy<Msg>, item: &Item, place: Xform) {
    let f0 = now_ms();
    let named = if item.name.is_empty() { item.file.clone() } else { item.name.clone() };
    let Some(f) = cloud_fields(&item.file).await else {
        log::warn!("'{}': stream requested but no Range-addressable cloud found - skipped", named);
        return;
    };
    log::info!("streaming '{}': {} points | coords {:.0} MB + colours {:.0} MB",
        named, f.count, f.coords_len as f64 / 1048576.0, f.colors_len as f64 / 1048576.0);
    let _ = proxy.send_event(Msg::CloudBegin(CloudBegin { name: named.clone(), place, count: f.count, px: item.point_size as f32 }));
    let local = stream_coords(proxy, &item.file, &f).await;
    if let Some(col) = cloud_colors(&item.file, f.colors_at, f.colors_len, f.count).await {
        let _ = proxy.send_event(Msg::CloudCol(col));
    }
    let _ = proxy.send_event(Msg::CloudEnd(local));
    log::info!("streamed '{}' in {:.0}ms", named, now_ms() - f0);
}

/// The coords run in 8 MB slices, each converted, posted and dropped; returns the cloud's own
/// box. PIPELINED, and this is the loader's whole performance story: `fetch_range(..).await`
/// resolves off network I/O and cannot resume until the current FRAME is done, so a sequential
/// loop pays a frame per slice - slice n+1 in flight while n converts hides both.
async fn stream_coords(proxy: &EventLoopProxy<Msg>, url: &str, f: &CloudFields) -> Aabb {
    // 8 MB, rounded DOWN to a whole number of points: a slice boundary can then never fall
    // inside a point, let alone inside one of its doubles.
    const SLICE: u64 = (8 * 1024 * 1024 / 24) * 24;
    let (mut at, mut left) = (f.coords_at, f.coords_len);
    let (mut lo, mut hi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    let mut inflight = if left > 0 {
        fetch_range_start(url, at, SLICE.min(left)).ok()
    } else {
        None
    };
    while let Some(f_in) = inflight.take() {
        let n = SLICE.min(left);
        at += n;
        left -= n;
        // next one on the wire BEFORE we spend time on this one
        inflight = if left > 0 {
            fetch_range_start(url, at, SLICE.min(left)).ok()
        } else {
            None
        };
        let Ok(raw) = fetch_range_finish(f_in).await else { break };
        let pos = positions_from(&raw);
        drop(raw);
        for q in pos.chunks_exact(3) {
            for k in 0..3 { lo[k] = lo[k].min(q[k]); hi[k] = hi[k].max(q[k]); }
        }
        let _ = proxy.send_event(Msg::CloudPos(pos));
        // A real macrotask between slices: with a warm cache the fetch promises resolve as
        // MICROtasks, which never let the browser paint - the freeze the sliced parse avoids.
        next_tick().await;
    }
    Aabb { min: lo, max: hi }
}

/// One whole file: finish its prefetch, decode in chunks, post it as a `Msg::File`.
async fn whole_item(proxy: &EventLoopProxy<Msg>, item: &Item, fetched: Option<Result<Fetch, JsValue>>, place: Xform) {
    let f0 = now_ms();
    let bytes = match fetched {
        Some(Ok(f)) => fetch_finish(f).await.unwrap_or_default(),
        _ => Vec::new(),
    };
    let f1 = now_ms();
    let session = session_from_bytes_chunked(&item.file, &bytes).await;
    let name = if item.name.is_empty() {
        session.name.clone()
    } else {
        item.name.clone()
    };
    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), bytes.len(), f1 - f0, now_ms() - f1);
    if session.lookup.is_empty() {
        return; // failed fetch - skipped file
    }
    let _ = proxy.send_event(Msg::File(FileDoc { name, session, place, point_px: item.point_size as f32, display_only: item.display_only }));
}
```

## Step 19 — `src/app/input.rs`

`Input` and every key and mouse binding; each handler says whether a frame is needed.

**Create `src/app/input.rs`**

```rust
//! The gesture state machine and every key binding: RMB orbits, MMB (or Ctrl+RMB) pans, the
//! wheel zooms toward the cursor; 1-7 named views, Space projection, C reset, F fit, Q/W/E/L
//! lane toggles, [ ] cloud size. Mutates `camera` and `gpu.view` and says whether to redraw.

use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use crate::camera::View;
use crate::engine::gpu::LineStyle;
use crate::State;

/// What the mouse is doing between events.
pub struct Input {
    pub orbiting: bool,
    pub panning: bool,
    pub last_cursor: (f64, f64),
    pub ctrl: bool,
}

impl Input {
    /// Nothing held, cursor at the origin.
    pub fn new() -> Self {
        Self { orbiting: false, panning: false, last_cursor: (0.0, 0.0), ctrl: false }
    }

    /// One key press (the caller filters repeats). True when the frame must be redrawn.
    pub fn key(&mut self, state: &mut State, key: Key<&str>) -> bool {
        match key {
            Key::Named(NamedKey::Space) => {
                let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                state.camera.toggle_projection_framed(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
            }
            Key::Character("1") => state.camera.set_view(View::Front),
            Key::Character("2") => state.camera.set_view(View::Back),
            Key::Character("3") => state.camera.set_view(View::Left),
            Key::Character("4") => state.camera.set_view(View::Right),
            Key::Character("5") => state.camera.set_view(View::Top),
            Key::Character("6") => state.camera.set_view(View::Bottom),
            Key::Character("7") => state.camera.set_view(View::Iso),
            Key::Character("c" | "C") => state.camera.reset(),
            // Q / W / E hide a whole KIND of thing so an overlap can be taken apart by eye; L
            // draws the SOLID lane's edges as tubes or as flat quads - same table, a free A/B.
            Key::Character("q" | "Q") => {
                state.gpu.view.show_points = !state.gpu.view.show_points;
                log::info!("points: {}", state.gpu.view.show_points);
            }
            Key::Character("w" | "W") => {
                state.gpu.view.show_lines = !state.gpu.view.show_lines;
                log::info!("lines: {}", state.gpu.view.show_lines);
            }
            Key::Character("e" | "E") => {
                state.gpu.view.show_mesh_edges = !state.gpu.view.show_mesh_edges;
                log::info!("mesh edges: {}", state.gpu.view.show_mesh_edges);
            }
            Key::Character("l" | "L") => {
                state.gpu.view.line_style = match state.gpu.view.line_style {
                    LineStyle::Tubes => LineStyle::Flat,
                    LineStyle::Flat => LineStyle::Tubes,
                };
                log::info!("line style: {:?}", state.gpu.view.line_style);
            }
            // live cloud point size
            Key::Character("[") => {
                state.gpu.view.cloud_size = (state.gpu.view.cloud_size - 0.25).max(0.25);
                log::info!("cloud size scale: x{}", state.gpu.view.cloud_size);
            }
            Key::Character("]") => {
                state.gpu.view.cloud_size = (state.gpu.view.cloud_size + 0.25).min(8.0);
                log::info!("cloud size scale: x{}", state.gpu.view.cloud_size);
            }
            Key::Character("f" | "F") => {
                let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
                state.camera.fit(state.gpu.bounds.min, state.gpu.bounds.max, aspect);
            }
            _ => return false,
        }
        true
    }

    /// Buttons, motion, wheel and modifiers. True when the camera moved.
    pub fn mouse(&mut self, state: &mut State, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { state: btn, button: MouseButton::Right, .. } => {
                self.orbiting = *btn == ElementState::Pressed; // hold RMB to orbit
                false
            }
            WindowEvent::MouseInput { state: btn, button: MouseButton::Middle, .. } => {
                self.panning = *btn == ElementState::Pressed; // hold MMB to pan (CAD standard)
                false
            }
            WindowEvent::CursorMoved { position, .. } => {
                let dragging = self.orbiting || self.panning;
                if dragging {
                    let dx = (position.x - self.last_cursor.0) as f32;
                    let dy = (position.y - self.last_cursor.1) as f32;
                    if self.panning || self.ctrl {
                        state.camera.pan(dx, dy);
                    } else {
                        state.camera.orbit(dx, dy)
                    };
                }
                self.last_cursor = (position.x, position.y);
                dragging
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 100.0,
                };
                // Zoom toward the cursor - the point under the mouse stays put
                let vp = (state.gpu.config.width as f64, state.gpu.config.height as f64);
                state.camera.zoom_at(amount, self.last_cursor, vp);
                true
            }
            WindowEvent::ModifiersChanged(mods) => {
                self.ctrl = mods.state().control_key();
                false
            }
            _ => false,
        }
    }
}
```

## Step 20 — `src/app/scene.rs`

Replace the whole file: `Doc`, `FileDoc`, `CloudBegin`, `CloudSlot`, `Scene`; `add_file` is a
loop of `walk_geometry` calls plus the sweeps.

**Create `src/app/scene.rs`**

```rust
//! The document side of the scene: `Scene` owns WHAT is loaded - every kernel `Session` with
//! its placement, the merged `Upload` tables and the row bookkeeping. `add_file` walks one new
//! session into the shared tables; rows are appended, never rebuilt. The producers live in
//! `walk/`; this file never names a `Geometry` variant.

use std::collections::{HashMap, HashSet};
use session_rust::{Session, Xform};
use crate::engine::gpu::{Gpu, Instance, Upload};
use crate::math::{mat_mul, Aabb, Mat4};
use crate::app::knobs;
use crate::app::walk::{is_drawable, walk_geometry, Walk, WalkCx};
use crate::app::walk::bounds::{file_extent, mark_sheet, sheet_thickness, Baselines};
use crate::app::walk::mesh::Lap;

/// One loaded file: the kernel `Session` kept alive (picking, undo and save read it) plus the
/// placement the manifest gave it.
pub struct Doc {
    pub name: String,
    pub place: Xform,
    pub session: Session,
    pub point_px: f32, // per-file cloud point size, px; 0 = the pb's own
    /// This doc's `session` was RELEASED after the walk (manifest `display_only`): an empty
    /// shell that still names the document and holds its placement. `rebuild` cannot bring it back.
    pub display_only: bool,
}

/// One parsed file on its way into the scene: what the loader hands `add_file`.
pub struct FileDoc {
    pub name: String,
    pub session: Session,
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
}

/// A cloud about to stream in: the count is known from the file's packed-double length prefix
/// before a single point has been fetched.
pub struct CloudBegin {
    pub name: String,
    pub place: Xform,
    pub count: u32,
    pub px: f32,
}

/// A cloud whose points never became kernel objects: the loader streamed them from the file
/// straight into GPU memory. This is the ENTIRE CPU-side footprint of a 13.8M-point scan.
pub struct CloudSlot {
    pub name: String,
    pub place: Xform,
    pub count: u32,
    pub px: f32,
    pub instance: u32,
}

/// The open document set + the merged GPU tables. Rows are appended, never rebuilt, so
/// progressive loading costs each file only its own walk. Viewer-only bookkeeping (row order,
/// guid -> row, hidden) lives here, never in the kernel type three languages share.
pub struct Scene {
    pub docs: Vec<Doc>,
    pub clouds: Vec<CloudSlot>,
    vert_base: u32,             // arena rows already uploaded - walk_mesh bases its indices on this
    cloud_base: u32,            // cloud points already uploaded - a draw record's `first` counts from here
    pub tables: Upload,
    order: Vec<String>, // renderable guids, global row order across docs
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
}

impl Scene {
    /// Empty: no documents, inverted bounds, bases at 0.
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            clouds: Vec::new(),
            vert_base: 0,
            cloud_base: 0,
            tables: Upload::default(),
            order: Vec::new(),
            guid_to_row: HashMap::new(),
            hidden: HashSet::new(),
        }
    }

    /// Drop every document and its GPU rows, keeping the scene usable: the counterpart to
    /// `rebuild`, same reset minus the re-walk, so a scene can be REPLACED without tearing down
    /// `State` (camera, surface and pipelines survive a reload).
    pub fn clear(&mut self, gpu: &mut Gpu) {
        self.docs.clear();
        self.tables = Upload::default();
        self.order.clear();
        self.guid_to_row.clear();
        self.hidden.clear();
        self.vert_base = 0;
        self.cloud_base = 0;
        gpu.reset_arena();
    }

    /// Widen the shared walk box by a streamed cloud's world AABB. Without this the box lives
    /// only in `Gpu` and the next `set_scene` from a real walk would replace it.
    pub fn grow_bounds(&mut self, world: &Aabb) {
        self.tables.bounds.union(world);
    }

    /// Reserve the document row for a cloud that is about to stream in. Returns the instance
    /// row the streamed points will draw against; `order` stays aligned with the rows.
    pub fn begin_cloud(&mut self, c: CloudBegin) -> u32 {
        let CloudBegin { name, place, count, px } = c;
        let row = self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push((place.m, [1.0; 4], 0));
        self.tables.obj.bounds.push(None);
        self.tables.obj.spacing.push(px); // the manifest px rides the spacing row, like the walk's clouds
        let guid = format!("cloud:{name}");
        self.guid_to_row.insert(guid.clone(), row);
        self.order.push(guid);
        self.clouds.push(CloudSlot { name, place, count, px, instance: row });
        row
    }

    /// Re-flatten EVERY document from its kernel `Session` and re-upload from scratch. Once
    /// `upload_to` drops the tables there is no CPU copy left to patch, so a geometry edit has
    /// nothing to rewrite; a full re-walk belongs behind an edit commit, not behind a drag.
    pub fn rebuild(&mut self, gpu: &mut Gpu) {
        let docs = std::mem::take(&mut self.docs);
        let clouds = std::mem::take(&mut self.clouds);
        self.tables = Upload::default();
        self.order.clear();
        self.guid_to_row.clear();
        self.vert_base = 0;
        self.cloud_base = 0;
        gpu.reset_arena();

        for d in docs {
            if d.display_only {
                // Nothing to re-walk - the kernel document was released after the first walk.
                // Saying so beats silently dropping the sheet out of the frame.
                log::warn!("rebuild: '{}' is display_only, its geometry was released", d.name);
            }
            self.add_file(FileDoc { name: d.name, session: d.session, place: d.place, point_px: d.point_px, display_only: d.display_only });
        }
        // Clouds keep their GPU rows; only the instance they draw against is re-issued and the
        // Gpu's stream draw list patched to match. Index i here is index i there.
        for (i, c) in clouds.into_iter().enumerate() {
            let row = self.begin_cloud(CloudBegin { name: c.name, place: c.place, count: c.count, px: c.px });
            gpu.stream.retarget(i, row);
        }
        self.upload_to(gpu);
    }

    /// Upload the walked tables, then FORGET the rows (`Upload::drop_uploaded`): the GPU is
    /// their only holder. Only the running bases stay, so the next file's indices still land in
    /// the right place.
    pub fn upload_to(&mut self, gpu: &mut Gpu) {
        gpu.set_scene(&self.tables);
        self.vert_base += self.tables.arena.verts.len() as u32;
        self.cloud_base += (self.tables.cloud.pos.len() / 3) as u32;
        self.tables.drop_uploaded();
    }

    /// Walk one session into the shared tables: one object row per guid in the kernel's
    /// canonical `order()` (the row a guid gets here is the row it keeps - picking relies on
    /// it), then the per-file sweeps: extent, planar test, sheet marking.
    pub fn add_file(&mut self, doc: FileDoc) {
        let FileDoc { name, session, place, point_px, display_only } = doc;
        let from = Baselines::capture(&self.tables, self.cloud_base);
        let (vb, cb) = (self.vert_base, self.cloud_base); // read before `t` borrows self.tables
        let world = session.world_xforms();
        let place_m = place.m;
        let mut lap = Lap::start("walk");
        let t = &mut self.tables;
        for guid in session.order() {
            let Some(geom) = session.lookup.get(&guid) else { continue };
            if !is_drawable(geom) { continue }
            let ri = t.obj.rows.len() as u32;
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            t.obj.rows.push((placement(&world, &place_m, &guid), [1.0; 4], flags));
            let cx = WalkCx { vert_base: vb, cloud_base: cb, cloud_px: point_px, row: ri };
            let r = walk_geometry(&mut Walk::of(t), &cx, geom);
            t.obj.rows.last_mut().unwrap().2 |= r.flags;
            t.obj.bounds.push(r.bounds);
            t.obj.spacing.push(r.spacing);
            self.guid_to_row.insert(guid.clone(), ri);
            self.order.push(guid);
        }
        lap.mark("objects");

        let extent = file_extent(t, &from);
        lap.mark("bounds");
        t.bounds.union(&extent);

        let thickness = sheet_thickness(t, &from, &place, &extent);
        let planar = thickness.is_finite() && thickness.abs() < 1e-3;

        if planar { mark_sheet(t, &from) }

        // The walk is done and the tables are about to be uploaded, so a display-only document
        // has nothing left to answer: release it here, the exact point after which nothing
        // reads it. VIEWER_DROP_SESSIONS=1 forces it on for every file.
        let display_only = display_only || knobs::drop_sessions();
        let session = if display_only { Session::new(&name) } else { session };
        self.docs.push(Doc { name, place, session, point_px, display_only });
    }
}

/// An object's placement: the manifest `place` times the session's own world xform for that
/// guid. The 99% path (a flat sheet, a mesh file) has NO local transforms, so every row's
/// placement IS the file placement - `place_m` is composed once, not 90k times with kernel `Xform`s.
fn placement(world: &HashMap<String, Xform>, place_m: &Mat4, guid: &str) -> Mat4 {
    match world.get(guid) {
        Some(local) => mat_mul(place_m, &local.m),
        None => *place_m,
    }
}
```

## Step 21 — `src/app/mod.rs`

Replace the whole file with the module list.

**Create `src/app/mod.rs`**

```rust
//! The app layer: what a scene IS (manifest, documents, the walk into rows) and how it gets
//! here (fetch, decode, stream, the loader) and is driven (input). Above the engine, below the
//! shell in lib.rs.

pub mod decode;
pub mod fetch;
pub mod input;
pub mod knobs;
#[cfg(target_arch = "wasm32")]
pub mod loader;
pub mod manifest;
pub mod scene;
pub mod stream;
pub mod walk;
```

## Step 22 — `src/app/persistence.rs`

It was split into `fetch.rs`, `decode.rs` and `stream.rs` (Steps 15-17).

**Delete `src/app/persistence.rs`**

## Step 23 — `src/math.rs`

`Aabb::placed`: a local box through a placement.

**Find** in `src/math.rs`:

```rust
        self.min.iter().chain(&self.max).all(|v| v.is_finite())
    }
```

**Add below it:**

```rust

    /// The box through a placement: the eight corners transformed, then re-boxed.
    pub fn placed(&self, m: &Mat4) -> Aabb {
        let mut world = Aabb::empty();
        for c in 0..8u32 {
            let corner = [
                if c & 1 == 0 { self.min[0] } else { self.max[0] },
                if c & 2 == 0 { self.min[1] } else { self.max[1] },
                if c & 4 == 0 { self.min[2] } else { self.max[2] },
            ];
            world.grow(xform_point(m, corner));
        }
        world
    }
```

## Step 24 — `src/state.rs`

`append`, `cloud_begin`, `cloud_end`, `fit_all`: the handler bodies that lived in `lib.rs`.

**Find** in `src/state.rs`:

```rust
use crate::app::scene::Scene;
use crate::engine::performance::now_ms;
```

**Replace with:**

```rust
use crate::app::scene::{CloudBegin, FileDoc, Scene};
use crate::engine::performance::{heap_mb, now_ms};
use crate::math::Aabb;
```

The added block starts with the `}` that closes `new` and ends inside `fit_all`; the brace
already below the anchor closes it.

**Find** in `src/state.rs`:

```rust
        Ok(Self {window, gpu, camera: Camera::new(), scene })
```

**Add below it:**

```rust
    }

    /// Append one parsed document: walk it into the shared tables, upload the delta.
    pub fn append(&mut self, doc: FileDoc) {
        let t0 = now_ms();
        self.scene.add_file(doc);
        let t1 = now_ms();
        self.scene.upload_to(&mut self.gpu);
        log::info!("appended: walk {:.0}ms · upload {:.0}ms | {} docs | heap {:.0} MB",
            t1 - t0, now_ms() - t1, self.scene.docs.len(), heap_mb());
    }

    /// A cloud about to stream: reserve its object row and its GPU range from the known count.
    /// Nothing here holds points - each slice is written and dropped.
    pub fn cloud_begin(&mut self, c: CloudBegin) {
        let count = c.count;
        let row = self.scene.begin_cloud(c);
        self.scene.upload_to(&mut self.gpu); // pushes the instance row
        self.gpu.cloud_begin(count, row);
    }

    /// A streamed cloud is complete: widen the scene by its placed box and refit the camera.
    pub fn cloud_end(&mut self, local: &Aabb) {
        // `local` is the cloud's own box; place it before it can fit the camera.
        if let Some(slot) = self.scene.clouds.last() {
            let world = local.placed(&slot.place.m);
            self.gpu.grow_scene(&world);
            self.scene.grow_bounds(&world);
        }
        // a finished scan is the dominant geometry - refit around everything so far
        self.fit_all();
    }

    /// Fit the camera around everything loaded so far.
    pub fn fit_all(&mut self) {
        let s = self.window.inner_size();
        let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
        self.camera.fit(self.gpu.bounds.min, self.gpu.bounds.max, aspect);
```

## Step 25 — `src/lib.rs`

Replace the whole file: `Msg`, `App`, the two winit callbacks, each arm one call into `State`.

**Create `src/lib.rs`**

```rust
//! session_viewer - a browser-only (WebGPU/wgpu + winit) 3D viewer, grown one documented chapter
//! at a time. This file is the shell only: the canvas window, the event loop and the `Msg`
//! handlers, each delegating to `State`. Loader: `app/loader.rs`; bindings: `app/input.rs`.

mod engine;
mod state;
mod camera;
pub mod math;
pub mod app;
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest; // headless render harness - see src/selftest.rs

pub use state::State;
use crate::app::scene::{CloudBegin, FileDoc};
use crate::math::Aabb;

/// Async init -> event-loop messages. `Ready` carries the `State` built around an EMPTY scene;
/// each `File` is one more parsed document appended live; the `Cloud*` messages are a streamed
/// cloud's slices; `Clear` drops the documents, keeping `State` (see `loader::reload_scene`).
pub enum Msg {
    Ready(Box<State>),
    File(FileDoc),
    CloudBegin(CloudBegin),
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    CloudEnd(Aabb),
    Clear,
}

#[cfg(target_arch = "wasm32")]
use {
    std::sync::Arc, wasm_bindgen::prelude::*, wasm_bindgen::JsCast, winit::application::ApplicationHandler,
    winit::event::{ElementState, WindowEvent}, winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys}, winit::window::{Window, WindowId}, crate::app::{input::Input, loader},
};

/// The winit application handler: owns the viewer `State` once async init completes, the
/// gesture state, and whether the camera has framed geometry yet.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<EventLoopProxy<Msg>>,
    input: Input,
    fitted: bool, // first geometry fits the camera; everything later only grows the extent
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App { proxy: Some(event_loop.create_proxy()), state: None, input: Input::new(), fitted: false };
        event_loop.spawn_app(app);
        Ok(())
    }

    /// `Ready`: adopt the State, size it to the canvas, draw - the scene is still empty.
    fn adopt(&mut self, mut state: State) {
        let (w, h) = desired_canvas_size().unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
        state.resize(w, h);
        state.window.request_redraw();
        self.state = Some(state);
    }
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler<Msg> for App {
    /// Bind to the `#canvas` element and start the loader; `State` comes back as `Msg::Ready`.
    /// `State::new` is async and winit's `resumed` is not - the documented wasm pattern.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() { return; }
        let canvas = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let attrs = Window::default_attributes().with_canvas(Some(canvas));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        if let Some(proxy) = self.proxy.take() {
            loader::keep_proxy(&proxy);
            wasm_bindgen_futures::spawn_local(loader::boot(window, proxy));
        }
    }

    /// Every message after `Ready` drives `State` and asks for a frame. The first document
    /// (or a finished scan) fits the camera; later ones only grow its extent.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        let msg = match msg { Msg::Ready(state) => return self.adopt(*state), other => other };
        let Some(state) = &mut self.state else { return };
        match msg {
            Msg::Ready(_) => {}
            Msg::Clear => state.scene.clear(&mut state.gpu),
            Msg::File(doc) => {
                state.append(doc);
                if self.fitted { state.camera.grow_extent(state.gpu.bounds.min, state.gpu.bounds.max) } else { state.fit_all() }
                self.fitted = true;
            }
            Msg::CloudBegin(c) => state.cloud_begin(c),
            Msg::CloudPos(pos) => state.gpu.cloud_pos(&pos), // the cloud grows on screen as it arrives
            Msg::CloudCol(col) => state.gpu.cloud_col(&col),
            Msg::CloudEnd(local) => { state.cloud_end(&local); self.fitted = true; }
        }
        state.window.request_redraw();
    }

    /// Redraw and resize here; keys and the mouse go to `Input`, which says whether to redraw.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
        let redraw = match event {
            WindowEvent::CloseRequested => { event_loop.exit(); false }
            WindowEvent::RedrawRequested => {
                // Make the GPU surface match the canvas's real pixel size before drawing:
                // a cheap check every frame, a reconfigure only on a genuine change.
                if let Some((w, h)) = desired_canvas_size() {
                    if (w, h) != (state.gpu.config.width, state.gpu.config.height) {
                        state.resize(w, h);
                    }
                }
                if let Err(e) = state.render() { log::error!("render: {e}"); }
                false
            }
            WindowEvent::KeyboardInput { event, .. } => {
                event.state == ElementState::Pressed && !event.repeat && self.input.key(state, event.logical_key.as_ref())
            }
            other => self.input.mouse(state, &other),
        };
        if redraw { state.window.request_redraw(); }
    }
}

/// The canvas's pixel size (CSS size × device-pixel-ratio), or `None` if zero or unavailable.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
    let win = web_sys::window()?;
    let dpr = win.device_pixel_ratio();
    let canvas = win.document()?
        .get_element_by_id("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    let w = (canvas.client_width()  as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}

/// wasm entry point: install the panic hook and run the app.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
```

## Step 26 — `src/selftest.rs`

`SceneFile` and `from_args` replace the manifest loop the three examples each spelled out;
`mb` and `median` become free functions.

**Find** in `src/selftest.rs`:

```rust
use crate::app::scene::Scene;
```

**Replace with:**

```rust
use crate::app::manifest::Manifest;
use crate::app::scene::{FileDoc, Scene};
```

**Find** in `src/selftest.rs`:

```rust
use session_rust::{Session, Xform};
```

**Add below it:**

```rust

/// One file the harness loads: its path, where it sits, its per-file point size (px, 0 = the
/// pb's own) and whether its kernel `Session` is released after the walk.
pub struct SceneFile {
    pub path: String,
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
}

impl SceneFile {
    /// The harness's arguments. A `.json`/`.toml` argument is a SCENE MANIFEST, resolved the way
    /// the browser resolves it (paths relative to the assets root = the manifest's grandparent,
    /// a 3 m auto-grid), so what the harness renders is what the viewer renders; anything else is
    /// one .pb at its own origin - which is how a 0.156-unit bunny once became an invisible speck.
    pub fn from_args(args: &[String]) -> Vec<SceneFile> {
        let mut out = Vec::new();
        for p in args {
            if !(p.ends_with(".json") || p.ends_with(".toml")) {
                out.push(SceneFile { path: p.clone(), place: Xform::identity(), point_px: 0.0, display_only: false });
                continue;
            }
            let bytes = std::fs::read(p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = Manifest::parse(&bytes).unwrap_or_else(|| panic!("cannot parse manifest {p}"));
            let root = std::path::Path::new(p).parent().and_then(|d| d.parent())
                .unwrap_or(std::path::Path::new(".")).to_path_buf();
            for (i, item) in man.items.iter().enumerate() {
                let path = root.join(&item.file).to_string_lossy().into_owned();
                out.push(SceneFile { path, place: man.place(i, [3000.0, 3000.0]), point_px: item.point_size as f32, display_only: item.display_only });
            }
        }
        out
    }
}

/// Bytes as MB, for the table footprint lines.
fn mb(b: usize) -> f64 {
    b as f64 / 1.048576e6
}

/// The median of a sample, sorting it in place.
fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}
```

The non-Linux `rss_mb` stub gets its doc comment, for the zero-warning build.

**Find** in `src/selftest.rs`:

```rust
        .unwrap_or(0.0)
}
```

**Add below it:**

```rust
/// No /proc: the staged RSS lines print 0.
```

**Find** in `src/selftest.rs`:

```rust
pub fn render_scene(files: &[(&str, Xform, f32, bool)], w: u32, h: u32, out: &str) -> String {
```

**Replace with:**

```rust
pub fn render_scene(files: &[SceneFile], size: (u32, u32), out: &str) -> String {
    let (w, h) = size;
```

**Find** in `src/selftest.rs`:

```rust
    let rss0 = rss_mb();
    for (path, place, px, only) in files {
```

**Replace with:**

```rust
    let rss0 = rss_mb();
    for f in files {
        let path = &f.path;
```

**Find** in `src/selftest.rs`:

```rust
        );
        scene.add_file(name, session, place.clone(), *px, *only);
```

**Replace with:**

```rust
        );
        scene.add_file(FileDoc { name, session, place: f.place.clone(), point_px: f.point_px, display_only: f.display_only });
```

**Find** in `src/selftest.rs`:

```rust
        let t = &scene.tables;
        let mb = |b: usize| b as f64 / 1.048576e6;
```

**Replace with:**

```rust
        let t = &scene.tables;
```

**Find** in `src/selftest.rs`:

```rust
    )
}

```

**Add below it:**

```rust
/// Read and parse one file for the benches, panicking on a bad path - a bench has no report to
/// put an error in.
fn load(f: &SceneFile) -> FileDoc {
    let path = &f.path;
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {path}: {e:?}"));
    let name = path.rsplit('/').next().unwrap_or(path).to_string();
    FileDoc { name, session, place: f.place.clone(), point_px: f.point_px, display_only: f.display_only }
}

```

**Find** in `src/selftest.rs`:

```rust
pub fn bench_scene(files: &[(&str, Xform)], w: u32, h: u32) -> String {
    use crate::engine::gpu::LineStyle;
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for (path, place) in files {
        let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
        let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {path}: {e:?}"));
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        scene.add_file(name, session, place.clone(), 0.0, false);
```

**Replace with:**

```rust
pub fn bench_scene(files: &[SceneFile], size: (u32, u32)) -> String {
    use crate::engine::gpu::LineStyle;
    let (w, h) = size;
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for f in files {
        scene.add_file(load(f));
```

**Find** in `src/selftest.rs`:

```rust
pub fn frame_profile(files: &[(&str, Xform, f32, bool)], w: u32, h: u32) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for (path, place, px, only) in files {
        let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
        let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {path}: {e:?}"));
        let name = path.rsplit('/').next().unwrap_or(path).to_string();
        scene.add_file(name, session, place.clone(), *px, *only);
```

**Replace with:**

```rust
pub fn frame_profile(files: &[SceneFile], size: (u32, u32)) -> String {
    let (w, h) = size;
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    for f in files {
        scene.add_file(load(f));
```

**Find** in `src/selftest.rs`:

```rust
        let med = |v: &mut Vec<f64>| { v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v[v.len() / 2] };
        let (u, e, g) = (med(&mut uni), med(&mut enc_ms), med(&mut gpu_ms));
```

**Replace with:**

```rust
        let (u, e, g) = (median(&mut uni), median(&mut enc_ms), median(&mut gpu_ms));
```

## Step 27 — `examples/bench_frame.rs`

Replace the whole file: the manifest loop becomes one `SceneFile::from_args` call.

**Create `examples/bench_frame.rs`**

```rust
// cargo run --example bench_frame --target x86_64-unknown-linux-gnu --release -- assets/scenes/<scene>.toml
//
// Splits a frame into uniforms / encode / gpu, for a still and a moving camera. `bench_lines`
// answers how fast the frame is; this answers which of the three legs owns the milliseconds.

struct StderrLog;
impl log::Log for StderrLog {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn log(&self, r: &log::Record) { eprintln!("[{}] {}", r.level(), r.args()); }
    fn flush(&self) {}
}

fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Warn);
    let a: Vec<String> = std::env::args().skip(1).collect();
    let files = session_viewer::selftest::SceneFile::from_args(&a);
    let size = (
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700),
    );
    print!("{}", session_viewer::selftest::frame_profile(&files, size));
}
```

## Step 28 — `examples/bench_lines.rs`

Replace the whole file: the same `from_args` call.

**Create `examples/bench_lines.rs`**

```rust
// cargo run --example bench_lines --target x86_64-unknown-linux-gnu --release -- <file.pb | scene.json>...
// Times BOTH line styles (tubes vs flat) on the same scene, fit and far views.

struct StderrLog;
impl log::Log for StderrLog {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn log(&self, r: &log::Record) { eprintln!("[{}] {}", r.level(), r.args()); }
    fn flush(&self) {}
}

fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Warn);
    let a: Vec<String> = std::env::args().skip(1).collect();
    let files = session_viewer::selftest::SceneFile::from_args(&a);
    let size = (
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(1568),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(724),
    );
    print!("{}", session_viewer::selftest::bench_scene(&files, size));
}
```

## Step 29 — `examples/bench_load.rs`

`FileDoc` instead of five arguments.

**Find** in `examples/bench_load.rs`:

```rust
use session_viewer::app::scene::Scene;
```

**Replace with:**

```rust
use session_viewer::app::scene::{FileDoc, Scene};
```

**Find** in `examples/bench_load.rs`:

```rust
    scene.add_file("bench".into(), s, Xform::identity(), 1.0, false);
```

**Replace with:**

```rust
    scene.add_file(FileDoc { name: "bench".into(), session: s, place: Xform::identity(), point_px: 1.0, display_only: false });
```

## Step 30 — `examples/check_determinism.rs`

`FileDoc` instead of five arguments.

**Find** in `examples/check_determinism.rs`:

```rust
use session_viewer::app::scene::Scene;
```

**Replace with:**

```rust
use session_viewer::app::scene::{FileDoc, Scene};
```

**Find** in `examples/check_determinism.rs`:

```rust
    sc.add_file("d".into(), s, Xform::identity(), 0.0, false);
```

**Replace with:**

```rust
    sc.add_file(FileDoc { name: "d".into(), session: s, place: Xform::identity(), point_px: 0.0, display_only: false });
```

## Step 31 — `examples/selftest.rs`

Replace the whole file: the same `from_args` call.

**Create `examples/selftest.rs`**

```rust
// cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- <out.ppm> <file.pb>...

// wgpu reports validation errors through `log`; with no logger installed a broken shader just
// renders black. The smallest possible stderr logger, so a broken frame says WHY.
struct StderrLog;
impl log::Log for StderrLog {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn log(&self, r: &log::Record) { eprintln!("[{}] {}", r.level(), r.args()); }
    fn flush(&self) {}
}

fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Info);
    let a: Vec<String> = std::env::args().skip(1).collect();
    let out = a.first().cloned().unwrap_or_else(|| "out.ppm".into());
    // A .json/.toml argument is a SCENE MANIFEST, resolved the way the browser does it.
    let files = session_viewer::selftest::SceneFile::from_args(&a[1.min(a.len())..]);
    let size = (
        std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900),
        std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700),
    );
    print!("{}", session_viewer::selftest::render_scene(&files, size, &out));
}
```

## Check

```bash
cargo check --lib --target wasm32-unknown-unknown            # 0 warnings
cargo check --all-targets --target x86_64-unknown-linux-gnu  # 0 warnings in the crate
cargo xtest                                                  # 4 passed
grep -c 'Geometry::' src/app/scene.rs                        # 0
./docs/_gate.sh                                              # gate OK
```

`scene.rs` is 212 lines, `lib.rs` 150; the goldens do not move.

## Recap

- A producer receives only the rows it writes and returns its object row; `walk_geometry` is the
  only place that knows every geometry type.
- An option a caller must decide is a named field (`MeshOpts::allow_open`), never a dropped
  return value.
- The shell wires events; loading, decoding, streaming and input each have one file.

## Next

Lesson [49](50-performance-memory.md) — performance and memory: render on demand, one owner per
object row, an MSAA policy, and the numbers before and after.
