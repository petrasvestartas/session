# 51 A producer's signature names what it can reach

> Lesson [58](58-nurbscurve.md) adds a geometry type by writing one file and two lines; lesson
> [120](120-id-buffer-picking.md) needs to know which face a triangle came from, and adds one
> field in one place. Both are cheap because after this lesson the walk is one file per KERNEL
> TYPE, and a producer hands back its object row instead of pushing it.
> Nothing you can see changes.
> Answer key: `git diff end-of-49..end-of-50 -- session_viewer/src`.
>
> **Lessons 45-51 move code. Every body you cut is pasted byte-identical except for path
> re-roots inside ONE file.**

## 1. Why this seam

### 1a. The evidence

```bash
wc -l src/app/scene.rs
grep -c 'Geometry::' src/app/scene.rs
grep -c 't.obj.bounds.push\|t.obj.spacing.push' src/app/scene.rs
grep -c 'obj.rows.last_mut()' src/app/scene.rs
```

```text
1333  lines in one file
  16  Geometry:: arms
  14  hand-pushes of the two per-object columns
   3  reach-backs that patch the row already pushed
```

The engine is done. Everything above it is still one file: the manifest, the knobs, the encoders,
thirteen geometry arms, two file sweeps, and a 314-line `push_mesh`.

The fourteen hand-pushes are the actual hazard. Every arm owes the object table exactly two
columns, `bounds` and `spacing`, and pushes them itself — so an arm that pushes one and forgets
the other shifts **every later row's data by one**, with nothing to report it. Three arms then
reach back with `t.obj.rows.last_mut().unwrap().flags |= ..` to patch a row they did not push.

### 1b. The law

**W1 — a producer writes rows and RETURNS its object row; it never pushes one.** The caller owns
the object table, so the count cannot drift.

### 1c. The rejected alternative

Group the files by kernel name — `mesh.rs`, `line.rs`, `polyline.rs`, one per variant. Do not.
Line, Polyline and NurbsCurve produce the same row and differ by ten lines each; three files
would be three copies of one import block. **Group by output row**, which is why `curves.rs`
holds three types and `frames.rs` two.

## 2. Where the code lives after this lesson

| symbol | new home |
|---|---|
| `Item`, `Manifest`, `auto_grid` | `app/manifest.rs` |
| `env_flag` + the five `OnceLock`s | `app/knobs.rs` |
| `encode_width`, `pack_rgba`, `oct16`, `pack_facing`, `BLACK` | `app/walk/encode.rs` |
| Line, Polyline, NurbsCurve | `app/walk/curves.rs` |
| Point | `app/walk/points.rs` |
| Plane, OBB, `PLANE_SIZE` | `app/walk/frames.rs` |
| PointCloud | `app/walk/cloud.rs` |
| the two file sweeps | `app/walk/bounds.rs` |
| the thirteen arms | `app/walk/mod.rs` — `walk_geometry` |

```text
   engine/gpu/   organised by ROW FORMAT     what a shader reads
   app/walk/     organised by KERNEL TYPE    what a producer starts from
                 ^ many-to-many: a Mesh makes triangles AND segments AND glyphs,
                   and a CylinderSegment comes from six types. Neither axis serves both.
```

**Exit litmus:** `grep -c 'Geometry::' src/app/scene.rs` is **0** — `scene.rs` no longer knows
what a geometry is.

## 3. Files we touch

| file | what |
|---|---|
| `app/manifest.rs` | **NEW** 85 |
| `app/knobs.rs` | **NEW** 16 |
| `app/walk/{mod,encode,curves,points,frames,cloud,bounds}.rs` | **NEW** 215/80/85/23/53/115/129 |
| `app/scene.rs` | 1,333 → **739** |
| `app/mod.rs` | two module lines |

## 4. The nine files, created first

### 4.1 `manifest.rs` — the head of the file leaves whole

`scene.rs` opens with 85 lines that answer WHERE, not WHAT. They move unchanged.


**Create `src/app/manifest.rs`**

```rust
//! The scene manifest: WHICH files a scene is made of and WHERE each one sits.
//!
//! A drawing is authored at its own page origin, so any number of them loaded raw would stack on
//! top of each other. Placement therefore has to come from somewhere - and the honest place is a
//! text file next to the assets, not arithmetic buried in the GPU layer. Edit `at`, reload, no
//! rebuild; a web deployment can be re-arranged without a compiler.
//!
//! ```json
//! { "items": [ { "file": "pb/draw_pf_he.pb", "name": "HE", "at": [3400, 0, 0] } ] }
//! ```
//! `at` is a translation in world units. `xform` takes all 16 numbers instead when a sheet needs
//! rotation or scale. An item with neither falls back to the auto-grid below.
use serde::Deserialize;
use session_rust::Xform;

/// One manifest entry: a file to load and where to place it. Every file is authored at its own
/// origin, so each item carries a placement transform (`at` or `xform`) — without one, all
/// files would stack at (0,0,0); items with neither get an `auto_grid` slot instead.
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
    /// `display_only = true` releases this file's kernel `Session` once it has been walked into
    /// the GPU tables. It is the single biggest memory lever a scene has, and it is a scene's
    /// call to make rather than the loader's, because of exactly what it gives up.
    ///
    /// What a Doc's `Session` is FOR, once the walk is done, is reading geometry back: picking
    /// (ray against the kernel meshes), editing, saving, and `Scene::rebuild`. A drawing sheet
    /// does none of those - it is ink on paper that is looked at - and it is also where the
    /// memory is: 10 sheets of the `drawings` scene hold 1.2 GB of kernel documents to draw
    /// tables the GPU already owns. Measured on that scene: 2056 MB resident -> 899 MB, frame
    /// byte-identical.
    ///
    /// A model file (the bunny, a BRep, anything the user will click) must NOT set this.
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
}

/// Fallback for items with no `at`/`xform`: lay them out on a grid of `cell` steps, in list order.
/// Deliberately dumb - it exists so a manifest can be written one sheet at a time, not as the way
/// a scene is normally described.
pub fn auto_grid(index: usize, count: usize, cell: [f64; 2]) -> Xform {
    let cols = (count as f64).sqrt().ceil().max(1.0) as usize;
    Xform::translation((index % cols) as f64 * cell[0], (index / cols) as f64 * cell[1], 0.0)
}

// ─────────────────────────────────────────────────────────────────────────────────────────────
```

**Remove** `src/app/scene.rs` `//! The scene manifest: WHICH files a scene is made of and WHERE each one sits.` **through** `// ─────────────────────────────────────────────────────────────────────────────────────────────`

### 4.2 `knobs.rs` and `walk/encode.rs`

Five environment switches, and the four encoders that turn what a DOCUMENT says into what a row
carries. Both are pure: no sink, no `Upload`, no wgpu type.


**Create `src/app/knobs.rs`**

```rust
//! `knobs.rs` - the environment switches, and the one function that reads them.
//!
//! Five booleans, each read from the environment ONCE and cached in a `OnceLock`. They are
//! debugging switches for the walk, not settings: nothing in the UI writes one, and a scene
//! cannot change one. `View` (`engine/gpu/view.rs`) is the other half of this - the knobs that
//! gate a DRAW live there, because the engine reads them every frame; these gate what the walk
//! PRODUCES, so they are read once per file and belong on this side of the line.

pub fn env_flag(name: &str, slot: &'static std::sync::OnceLock<bool>) -> bool {
    *slot.get_or_init(|| std::env::var(name).is_ok())
}
pub static VIEWER_PROFILE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
pub static VIEWER_DROP_SESSIONS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
pub static VIEWER_NO_EDGES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
pub static VIEWER_NO_DOTS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
pub static VIEWER_ALL_EDGES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
```

**Create `src/app/walk/encode.rs`**

```rust
//! `walk/encode.rs` - the document conventions, and only those.
//!
//! Four encoders and one constant that turn what a DOCUMENT says into what a row carries: a pen
//! width into a radius, an RGBA float quad into a packed word, a normal into oct16, a pair of
//! adjacent face normals into one facing word. They are conventions of the format, not of the
//! GPU: the row FORMATS live with their families in `engine/gpu/`, and these decide what goes
//! into a field, not how wide it is.
//!
//! Everything here is a pure function of its arguments. No sink, no `Upload`, no wgpu type.

use crate::engine::gpu::segments::FACING_UNKNOWN;

/// The kernel's `width` is in MILLIMETRES - the drawings lane talks in 0.09-0.5 mm plot pens
/// and `Line`/`Polyline` default to 1.0. This used to return `-(w)`, and a NEGATIVE radius means
/// "multiply the global pen" to every shader - so a 30 mm polyline became 2 px x 30 = a 60 px
/// half-width, a 120 px slab. Millimetres were being read as a multiplier.
///
/// Now: an explicit width is a world-mm RADIUS (half the width, positive => the projected
/// branch), and only the untouched 1.0 default falls back to the screen-constant pen. That
/// keeps mesh edges - which never set a width - at a zoom-independent 2 px, while a pen someone
/// actually authored measures what it says.
pub fn encode_width(w: f64) -> f32{
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 {
        (w as f32) * 0.5
    } else {
        0.0
    }
}

/// RGBA8 in one word, low byte red - the layout `unpack4x8unorm` expects in WGSL.
pub fn pack_rgba(c: [f32; 4]) -> u32 {
    let q = |v: f32| ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff;
    q(c[0]) | q(c[1]) << 8 | q(c[2]) << 16 | q(c[3]) << 24
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
        // signNotZero, NOT signum. `f64::signum(0.0)` is 0.0, which folds (0,0,-1) onto (0,0) -
        // the code for (0,0,+1) - so the two poles collided. On an axis-aligned box that is the
        // top and bottom faces, i.e. most of its edges, and the collision then landed on the
        // all-zero "no adjacency" sentinel: the facing test silently did nothing for them.
        let s = |v: f64| if v < 0.0 { -1.0 } else { 1.0 };
        let (ax, ay) = (x.abs(), y.abs());
        (x, y) = ((1.0 - ay) * s(x), (1.0 - ax) * s(y));
    }
    let q = |v: f64| (((v.clamp(-1.0, 1.0) * 127.0).round() as i32) as u32) & 0xff;
    Some(q(x) | q(y) << 8)
}

/// Opaque black, packed. The wireframe's default pen, and what a dense mesh's edges draw as.
pub const BLACK: u32 = 0xff00_0000;

/// The two faces an edge belongs to, packed into one word for the shader's facing test.
///
/// `FACING_UNKNOWN` means "no adjacency known, always draw" - see the constant for why it is the
/// all-ones word and not 0.
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

### 4.3 The four converter files

Grouped by the row they write, not by kernel name — see §1c.


**Create `src/app/walk/curves.rs`**

```rust
//! `walk/curves.rs` - Line, Polyline and NurbsCurve.
//!
//! Three kernel types, one output row: a `CylinderSegment` in the FLAT lane. They share a file
//! because they share that row, not because they are alike - grouping is by what a producer
//! WRITES, which is the only grouping the sink can enforce.
//!
//! A NurbsCurve is a Polyline once it has been sampled, and the code says so: after the sample
//! count is chosen it calls `polyline_to_segments`.

use session_rust::{Line, NurbsCurve, Polyline};

use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::segments::FACING_UNKNOWN;

use super::encode::{encode_width, pack_rgba};

pub fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment {
    CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
        instance_id,
        color: pack_rgba(l.linecolor.to_f32()),
        facing: FACING_UNKNOWN, // free-standing linework has no adjacent faces: always drawn
    }
}

pub fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment> {
    let pts = pl.get_points();
    let color = pack_rgba(pl.linecolor.to_f32());
    pts.windows(2).map( |w| CylinderSegment {
        p0: w[0].to_f32(),
        radius: encode_width(pl.width),
        p1: w[1].to_f32(),
        instance_id,
        color,
        facing: FACING_UNKNOWN,
    }).collect()
}

pub fn nurbscurve_to_segments(c: &NurbsCurve, instance_id: u32) -> Vec<CylinderSegment> {
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
        return Vec::new();
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
    // ... then it IS a polyline: consecutive pairs -> segments, same as polyline_to_segments.
    pts.windows(2).map(|w| CylinderSegment {
        p0: w[0],
        radius,
        p1: w[1],
        instance_id,
        color,
        facing: FACING_UNKNOWN,
    }).collect()
}
```

**Create `src/app/walk/points.rs`**

```rust
//! `walk/points.rs` - the Point type.
//!
//! Ten lines, and it is the whole file on purpose: a free-standing point decorates no surface, so
//! it has no adjacency to pack and no width to encode beyond its own. `facing` and `facing_ext`
//! are the UNKNOWN sentinel, which is what tells the shader to draw it unconditionally.

use session_rust::Point;

use crate::engine::gpu::GlyphPoint;
use crate::engine::gpu::segments::FACING_UNKNOWN;

use super::encode::encode_width;

pub fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint {
    GlyphPoint {
        center: p.to_f32(),
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id,
        facing: FACING_UNKNOWN, // a free point decorates no surface
        facing_ext: [FACING_UNKNOWN; 2],
    }
}
```

**Create `src/app/walk/frames.rs`**

```rust
//! `walk/frames.rs` - Plane and OBB, the two types that are drawn as a FRAME rather than as
//! themselves.
//!
//! Neither carries geometry a renderer could show directly: a Plane is infinite and an OBB is a
//! box with no surface. Both become a fixed, small set of `CylinderSegment`s in the flat lane -
//! four edges and twelve - and both therefore encode a display CONVENTION rather than the thing
//! itself. `PLANE_SIZE` is that convention made explicit.

use session_rust::{OBB, Plane};

use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::segments::FACING_UNKNOWN;

use super::encode::{encode_width, pack_rgba};

/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
/// Half-extent in world mm (a 1 m quare)
pub const PLANE_SIZE: f64 = 500.0;

pub fn plane_to_segments(pl: &Plane, instance_id: u32) -> Vec<CylinderSegment> {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let corner = |sx: f64, sy: f64| -> [f32; 3]{
         [0usize, 1, 2].map(|k| (o[k] + (x[k] * sx + y[k] * sy) * PLANE_SIZE) as f32)
    };
    let c = [corner(1.0, 1.0), corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, -1.0)];
    let color = pack_rgba(pl.linecolor.to_f32());
    let radius = encode_width(pl.width);
    (0..4).map(|i| CylinderSegment { p0:c[i], radius, p1: c[(i+1) % 4], instance_id, color, facing: FACING_UNKNOWN }).collect()
}

/// A box is its 12 edges: bottom loop, top loop, four verticals - `corner()` orders tge bottom face
/// face 0-3 and the top 4-7 with i / i+4 vertically aligned.
/// The OBB type carries no pen, so the edges draw black at screen-constant width (radius 0.0 = global default)
pub fn obb_to_segments(b: &OBB, instance_id: u32) -> Vec<CylinderSegment>{
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
    EDGES.iter().map(|&[i, j]| CylinderSegment { p0: c[i], radius: 0.0, p1: c[j], instance_id, color: pack_rgba([0.0, 0.0, 0.0, 1.0]), facing: FACING_UNKNOWN }).collect()

}
```

**Create `src/app/walk/cloud.rs`**

```rust
//! `walk/cloud.rs` - the PointCloud type.
//!
//! The one geometry that is its own compartment end to end: nothing else produces the rows it
//! produces, and nothing else draws them. It writes three columnar arrays rather than a row
//! struct - 20 bytes a point, and a lidar scan brings tens of millions - plus the octree the
//! engine walks to send a coarse subsample of a distant cloud instead of all of it.

use session_rust::{PointCloud, SpatialOctree};

use crate::engine::gpu::LodNode;

use super::encode::oct16;

/// The raw lane's rows, written straight into the shared table,
/// reading the kernel's flat arrays rather than get_point/get_color (no per_point allocs)
pub fn push_cloud(pc: &PointCloud, pos: &mut Vec<f32>, col: &mut Vec<u32>, nrm: &mut Vec<u32>, nodes: &mut Vec<LodNode>){
    let coords = pc.coords();
    let colors = pc.colors();
    let normals = pc.normals();
    let n = pc.len();
    pos.reserve(n*3);
    col.reserve(n);
    nrm.reserve(n);
    // The LOD octree, built ONCE and read twice: `order()` is the permutation that makes
    // every node's points contiguous, and the node table is this walk's second output.
    // Root accept spacing = the cube over 64 (the root's own subsample is a coarse
    // sketch); leaves absorb below 8192 points, so a shallow cloud stays one node.
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for i in 0..n {
        for k in 0..3 {
            lo[k] = lo[k].min(coords[i * 3 + k]);
            hi[k] = hi[k].max(coords[i * 3 + k]);
        }
    }
    let size = (hi[0] - lo[0]).max(hi[1] - lo[1]).max(hi[2] - lo[2]).max(1.0e-9);
    let tree = SpatialOctree::from_coords(coords, size / 64.0, 8192);
    // `first` is RELATIVE to this cloud's own first point, exactly as `children` are
    // relative to the cloud's node slice: the record builder adds the draw's base, so a
    // cloud can be re-uploaded at a different offset without rewriting its nodes.
    for ni in 0..tree.node_count() {
        let (center, sz) = tree.node_cube(ni);
        let (f, count) = tree.node_range(ni);
        // `children` hands back only the octants that exist, so the empty slots stay -1
        // and the record walk skips them; which octant a child was is nothing the
        // screen-error test asks about.
        let mut children = [-1i32; 8];
        for (slot, &ch) in tree.children(ni).iter().enumerate() {
            children[slot] = ch as i32;
        }
        nodes.push(LodNode {
            center: [center[0] as f32, center[1] as f32, center[2] as f32],
            size: sz as f32,
            spacing: tree.node_spacing(ni) as f32,
            first: f as u32,
            count: count as u32,
            children,
        });
    }
    for &i in tree.order(){
        pos.push(coords[i*3] as f32);
        pos.push(coords[i*3+1] as f32);
        pos.push(coords[i*3+2] as f32);

        // Normal, oct16-packed into 16 bits
        // All-ones = this point has nor normal: a scan without them still pays the 4 B,
        // but the shading branch stays uniform per cloud, which is what the GPU wants.
        // Three f64s, not a `Vector`: the kernel type carries a `name` String and a guid
        // OnceLock, so building one per point cost two heap allocations per scanned point -
        // 27 million of them on the 13.8 M-point lidar scan, for a value read once and dropped.
        nrm.push(if i*3 + 2 < normals.len() {
            oct16(&[normals[i*3], normals[i*3+1], normals[i*3+2]]).unwrap_or(u32::MAX)
        } else {
            u32::MAX
        });
        let c = i * 4;

        // The colour is 8-bit at the source (proto 0-255):
        // pack it back to the four bytes it is, instrad of four f32s carying four bytes of information
        col.push(if c + 3 < colors.len() {
            (colors[c] as u32 & 255) | (colors[c + 1] as u32 & 255) << 8 | (colors[c+2] as u32 & 255) << 16 | (colors[c + 3] as u32 & 255) << 24
        } else {
            0xff00_0000
        });
    }

}

/// Median distance between consecutive points - a scanner emits angular neighbours in order,
/// so successive points are usually adjacent on the surface, which makes this a cheap and
/// honest estimate of the clouds's point spacing (world units).
/// Potree gets the same number from its octree, we sample it.
/// Drives the attenuated world-sized splat radius.
pub fn cloud_spacing(pc: &PointCloud) -> f32{
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

### 4.4 `walk/bounds.rs` — the two file sweeps

Both walk only the rows THIS file added, which is why they take a `Baselines`: eight loose row
counts became one value, and the three functions dropped to four parameters each.


**Create `src/app/walk/bounds.rs`**

```rust
//! `walk/bounds.rs` - the two sweeps a file gets once its rows are written.
//!
//! Both walk only the rows THIS FILE added, which is why every function takes a `Baselines`:
//! the row counts as they stood before the walk started.
//!
//! `file_extent` gives the world AABB the camera fits to. `sheet_thickness` asks how thin the
//! file is along its own placement normal, and `mark_sheet` acts on the answer - a planar file
//! is PAPER, so every row of it gets `FLAG_SHEET` and every pen becomes a world-mm radius.

use session_rust::{Vector, Xform};

use crate::engine::gpu::objects::ObjectBase;
use crate::engine::gpu::{CloudDraw, Instance, Upload};
use crate::math::{grow_bounds, xform_point};

/// Row counts as they stood before this file's walk. Every sweep here runs over `[baseline..]`.
pub struct Baselines {
    pub vert: usize,
    pub seg: usize,
    pub pipe: usize,
    pub sphere: usize,
    pub glyph: usize,
    pub obj: usize,
    pub draw: usize,
    /// The cloud lane's absolute first-point offset for this file: `draws` carry absolute
    /// indices while `cloud.pos` is indexed from here.
    pub cloud_base: u32,
}

/// The world AABB of the rows this file added.
pub fn file_extent(t: &Upload, b: &Baselines) -> ([f32; 3], [f32; 3]) {
    let (mut fmin, mut fmax) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    for (i, v) in t.arena.verts.iter().enumerate().skip(b.vert) {
        if let Some(&ri) = t.arena.vids.get(i) {
            if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(ri as usize) {
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, v.position));
            }
        }
    }

    for s in t.seg.pipes.iter().skip(b.pipe).chain(t.seg.ribbons.iter().skip(b.seg)){
        if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(s.instance_id as usize){
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p0));
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p1));
        }
    }

    for s in t.glyph.spheres.iter().skip(b.sphere).chain(t.glyph.dots.iter().skip(b.glyph)){
        if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(s.instance_id as usize){
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.center));
        }
    }

    for &CloudDraw { first, count, instance: inst, .. } in t.cloud.draws.iter().skip(b.draw){
        let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(inst as usize) else { continue };
        // `first` is absolute; `cloud_pos` starts at `b.cloud_base`.
        for i in (first - b.cloud_base) as usize..(first - b.cloud_base + count) as usize {
            let p = [t.cloud.pos[i*3], t.cloud.pos[i*3+1], t.cloud.pos[i*3 + 2]];
            grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
        }
    }
    (fmin, fmax)
}

/// How thin this file is along its own placement normal.
pub fn sheet_thickness(t: &Upload, b: &Baselines, place: &Xform, fmin: [f32; 3], fmax: [f32; 3]) -> f32 {
    // 2D drawing sheets
    // flat linework - every PDF conversion gets paper space
    // keep kernel a real print
    // 3D model files keep screen-constant px linework
    // Planar = thin alon the SHEET's normal
    // The 99% path - translation only place, normal is Z+
    // reuseses the z-extent accumulated aboce - no extra work at all
    // only a rotated placement pays one dot-product pass over this file's new rows
    let n = place.transform_vector(&Vector::new(0.0, 0.0, 1.0));
    let thickness = if n[0].abs() < 1e-9 && n[1].abs() < 1e-9 {
        fmax[2] - fmin[2]
    } else {
        let (nx, ny, nz) = (n[0] as f32, n[1] as f32, n[2] as f32);
        let (mut dmin, mut dmax) = (f32::INFINITY, f32::NEG_INFINITY);
        let mut span = |p: [f32; 3]| {
            let d = p[0] * nx + p[1] * ny + p[2] * nz;
            dmin = dmin.min(d);
            dmax = dmax.max(d);
        };
        for (i, v) in t.arena.verts.iter().enumerate().skip(b.vert){
            if let Some(&ri) = t.arena.vids.get(i){
                if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(ri as usize) {
                    span(xform_point(xf, v.position));
                }
            }
        }
        for s in t.seg.pipes.iter().skip(b.pipe).chain(t.seg.ribbons.iter().skip(b.seg)){
            if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(s.instance_id as usize){
                span(xform_point(xf, s.p0));
                span(xform_point(xf, s.p1));
            }
        }
        for g in t.glyph.spheres.iter().skip(b.sphere).chain(t.glyph.dots.iter().skip(b.glyph)){
            if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(g.instance_id as usize) {
                span(xform_point(xf, g.center));
            }
        }
        dmax - dmin
    };
    thickness
}

/// Mark every row of a planar file as page content.
pub fn mark_sheet(t: &mut Upload, b: &Baselines) {
    // Every row of this file is page content. The ink lanes read the bit to drop their
    // lift (a sheet's fills no longer write depth, so there is nothing to lift off), and
    // that is what lets the lettering pass sit on top of the linework.
    for o in t.obj.rows.iter_mut().skip(b.obj) {
        o.flags |= Instance::FLAG_SHEET;
    }
    for s in t.seg.pipes.iter_mut().skip(b.pipe).chain(t.seg.ribbons.iter_mut().skip(b.seg)){
        // A flat sheet is paper: every pen becomes a world-mm radius so widths behave
        // like plotter pens. encode_width already returns a positive mm radius for any
        // authored width, so only the unset default (0.0) needs a value here - 0.5 mm,
        // the usual hairline. This used to read `radius < 0` because widths arrived as
        // NEGATIVE multipliers; they are millimetres now.
        s.radius = if s.radius > 0.0 {
            s.radius
        } else {
            0.5
        }
    }
}
```

### 4.5 `walk/mod.rs` — `walk_geometry` and `Row`

`Row` is the lesson. A producer returns one; `add_file` pushes it. Read the two constructors:
they also name the unit that one `spacing` float carries — world units for a mesh, pixels for a
cloud.

It is also where provenance will go. A row knows its OBJECT today and not which face or edge it
came from; lesson 120 adds a field here and touches no family file.


**Create `src/app/walk/mod.rs`**

```rust
//! `walk/` - one file per GEOMETRY TYPE.
//!
//! This is the other axis of the architecture. `engine/gpu/` is organised by ROW FORMAT, because
//! a row format is what a shader reads; the walk is organised by KERNEL TYPE, because a kernel
//! type is what a producer starts from. The two are many-to-many - a Mesh produces triangles AND
//! segments AND glyph points, and a `CylinderSegment` comes from six different types - so
//! neither axis can serve both ends.
//!
//! What crosses between them is a ROW, and every row carries an `instance_id` into the one object
//! table (`engine/gpu/objects.rs`).

pub mod bounds;
pub mod cloud;
pub mod curves;
pub mod encode;
pub mod frames;
pub mod points;

use session_rust::Geometry;
use session_rust::element::ElementGeometry;

use crate::engine::gpu::{CloudDraw, Instance, Upload};

// The mesh family still lives in `scene.rs`; lesson 52 gives it `walk/mesh.rs` and these three
// come with it.
use super::scene::{is_print_fill, mesh_spacing, push_mesh};

use cloud::{cloud_spacing, push_cloud};
use curves::{line_to_segment, nurbscurve_to_segments, polyline_to_segments};
use frames::{obb_to_segments, plane_to_segments};
use points::point_to_glyph;

/// The three running offsets a producer needs and cannot compute: where this file's vertices,
/// points and octree nodes start in tables that span every file loaded so far.
pub struct WalkCx {
    pub vert_base: u32,
    pub cloud_base: u32,
    /// The manifest's point-size override in PIXELS, or 0 to use the cloud's own.
    pub cloud_px: f32,
}

/// The object row a producer earns, returned rather than pushed.
///
/// Every geometry type produces EXACTLY ONE of these, and `add_file` pushes it - never a producer
/// body. Before this lesson eight arms hand-pushed `bounds` and `spacing` in pairs and three more
/// reached back with `t.obj.rows.last_mut().unwrap().flags |= ..`; an arm that forgot one shifted
/// every later row's data by one, silently.
///
/// It is also the seam for provenance. A row today knows its OBJECT (`instance_id`) but not which
/// FACE or EDGE of the kernel geometry it came from; when lesson 120 adds that, it adds a field
/// here and touches no family file.
pub struct Row {
    /// Mesh-LOCAL AABB. `None` for everything the solid lane's facing cull does not need.
    pub bounds: Option<([f32; 3], [f32; 3])>,
    /// Vertex spacing in WORLD units for meshes, or a size in PIXELS for clouds. One f32 carries
    /// both because the shaders read one field; `world_spacing` and `point_size_px` name which.
    pub spacing: f32,
    pub flags: u32,
}

impl Row {
    /// Linework, points and frames: no bounds, no spacing, no extra flags.
    pub fn none() -> Self {
        Self { bounds: None, spacing: 0.0, flags: 0 }
    }

    /// A tessellated solid - mesh, BRep or surface. `spacing` is in WORLD units.
    pub fn solid(bounds: Option<([f32; 3], [f32; 3])>, spacing: f32, flags: u32) -> Self {
        Self { bounds, spacing, flags }
    }

    /// A cloud. `px` is in PIXELS, not world units - the other unit this field carries.
    pub fn point_size_px(px: f32) -> Self {
        Self { bounds: None, spacing: px, flags: 0 }
    }
}

/// Walk one `Geometry` into the upload tables and return its object row.
///
/// Thirteen live arms. `ri` is the object row this geometry draws against, already pushed by the
/// caller, because every row a producer writes needs it and no producer may allocate one.
pub fn walk_geometry(t: &mut Upload, cx: &WalkCx, geom: &Geometry, ri: u32) -> Row {
        match geom{
        // 3D geometry takes the solid lane: edges are real cylinders and vertices - spheres
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
        Geometry::Line(l) => { t.seg.ribbons.push(line_to_segment(l, ri)); Row::none() }
        Geometry::Polyline(pl) => { t.seg.ribbons.extend(polyline_to_segments(pl, ri)); Row::none() }
        Geometry::NurbsCurve(c) => { t.seg.ribbons.extend(nurbscurve_to_segments(c, ri)); Row::none() }
        Geometry::Point(p) => { t.glyph.dots.push(point_to_glyph(p, ri)); Row::none() }
        // EVERY cloud takes the splat lane: split flat rows into share tables,
        // one draw record per cloud, and the per cloud point size rides the spacing spacing
        Geometry::PointCloud(pc) => {
            // ABSOLUTE first point, counted from the start of the scene: the GPU table is
            // cumulative while `cloud_pos` is only this upload's delta.
            let first = cx.cloud_base + (t.cloud.pos.len() / 3) as u32;
            let node_first = t.cloud.nodes.len() as u32;
            push_cloud(pc, &mut t.cloud.pos, &mut t.cloud.col, &mut t.cloud.nrm, &mut t.cloud.nodes);
            let node_count = t.cloud.nodes.len() as u32 - node_first;
            t.cloud.draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc), node_first, node_count });
            let px = if cx.cloud_px > 0.0 { cx.cloud_px } else { pc.point_size as f32 };
            Row::point_size_px(px)
        }
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
        Geometry::Plane(p) => { t.seg.ribbons.extend(plane_to_segments(p, ri)); Row::none() }
        Geometry::OBB(b) => { t.seg.ribbons.extend(obb_to_segments(b, ri)); Row::none() }
        Geometry::Element(e) => match e.geometry() {
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
            ElementGeometry::None => Row::none(),
        },
    }
}
```

## 5. Where the borrow checker bites — B5

> `walk_geometry` takes `&mut Upload` and reaches two columns at once:
>
> ```rust
> let idx_lane = if is_print_fill(m) { &mut t.arena.idx_print } else { &mut t.arena.idx };
> push_mesh(m, ri, cx.vert_base, &mut t.arena.verts, &mut t.arena.vids, idx_lane, ..)
> ```
>
> That compiles only because every `&mut` is a DISJOINT FIELD of one struct. Route them through a
> method — `t.arena_mut().idx()` — and it stops: a method borrows all of `t`. This is why the
> sinks stay plain fields.

## 6. The steps

### 6.1 The module list and the imports


**Find** in `src/app/mod.rs`:

```rust
pub mod persistence;
pub mod scene;
```

**Replace with:**

```rust
pub mod knobs;
pub mod manifest;
pub mod persistence;
pub mod scene;
pub mod walk;
```

**Find** in `src/app/scene.rs`:

```rust
use std::collections::{HashMap, HashSet};
use session_rust::{Session, Geometry, Mesh, Line, Point, Polyline, NurbsCurve, RenderVertex, Plane, OBB, PointCloud, SpatialOctree, Vector, Tolerance};
use session_rust::element::ElementGeometry;
use session_rust::mesh::ColorMode;
use crate::engine::gpu::{Upload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint, mat_mul};
use crate::engine::gpu::objects::ObjectBase;
use crate::engine::gpu::segments::FACING_UNKNOWN;
pub use crate::math::{grow_bounds, xform_point};
```

**Replace with:**

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

### 6.2 The moved bodies leave `scene.rs`

Sixteen removals. Each one is a body that now lives in the file named above it.


**Remove** `src/app/scene.rs` `fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment {` **through** `}`

**Remove** `src/app/scene.rs` `fn point_to_glyph(p: &Point, instance_id: u32) -> GlyphPoint {` **through** `}`

**Remove** `src/app/scene.rs` `fn encode_width(w: f64) -> f32{` **through** `}`

**Remove** `src/app/scene.rs` `fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment> {` **through** `}`

**Remove** `src/app/scene.rs` `fn nurbscurve_to_segments(c: &NurbsCurve, instance_id: u32) -> Vec<CylinderSegment> {` **through** `}`

**Remove** `src/app/scene.rs` `fn pack_rgba(c: [f32; 4]) -> u32 {` **through** `}`

**Remove** `src/app/scene.rs` `fn oct16(n: &[f64; 3]) -> Option<u32> {` **through** `}`

**Remove** `src/app/scene.rs` `const BLACK: u32 = 0xff00_0000;` **through** `const BLACK: u32 = 0xff00_0000;`

**Remove** `src/app/scene.rs` `fn pack_facing(n0: Option<&[f64; 3]>, n1: Option<&[f64; 3]>) -> u32 {` **through** `}`

**Remove** `src/app/scene.rs` `fn env_flag(name: &str, slot: &'static std::sync::OnceLock<bool>) -> bool {` **through** `}`

**Remove** `src/app/scene.rs` `static VIEWER_PROFILE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();` **through** `static VIEWER_ALL_EDGES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();`

**Remove** `src/app/scene.rs` `const PLANE_SIZE: f64 = 500.0;` **through** `const PLANE_SIZE: f64 = 500.0;`

**Remove** `src/app/scene.rs` `fn plane_to_segments(pl: &Plane, instance_id: u32) -> Vec<CylinderSegment> {` **through** `}`

**Remove** `src/app/scene.rs` `fn obb_to_segments(b: &OBB, instance_id: u32) -> Vec<CylinderSegment>{` **through** `}`

**Remove** `src/app/scene.rs` `fn push_cloud(pc: &PointCloud, pos: &mut Vec<f32>, col: &mut Vec<u32>, nrm: &mut Vec<u32>, nodes: &mut Vec<LodNode>){` **through** `}`

**Remove** `src/app/scene.rs` `fn cloud_spacing(pc: &PointCloud) -> f32{` **through** `}`

### 6.3 The thirteen arms become one call

This is the whole lesson in five lines. Note what disappears with the match: the fourteen
hand-pushes and the three `last_mut()` reach-backs.


**Find** in `src/app/scene.rs`:

```rust
fn mesh_spacing(bounds: Option<([f32; 3], [f32; 3])>, verts: usize) -> f32 {
```

**Replace with:**

```rust
pub(crate) fn mesh_spacing(bounds: Option<([f32; 3], [f32; 3])>, verts: usize) -> f32 {
```

**Find** in `src/app/scene.rs`:

```rust
fn is_print_fill(m: &Mesh) -> bool {
```

**Replace with:**

```rust
pub(crate) fn is_print_fill(m: &Mesh) -> bool {
```

**Find** in `src/app/scene.rs`:

```rust
fn push_mesh(
```

**Replace with:**

```rust
pub(crate) fn push_mesh(
```

**Find** in `src/app/scene.rs`:

```rust
// The DOCUMENT side of the scene: manifest above says WHERE, `Scene` below owns WHAT.

use std::collections
```

**Replace with:**

```rust
use std::collections
```

### 6.4 The sweeps, and the leftovers


**Find** in `src/app/scene.rs`:

```rust
        for guid in session.order() {
```

**Replace with:**

```rust
        let cx = WalkCx { vert_base: vb, cloud_base: cb, cloud_px };
        for guid in session.order() {
```

**Find** in `src/app/scene.rs`:

```rust
            match geom{
                // 3D geometry takes the solid lane: edges are real cylinders and vertices - spheres
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
                        vb,
                        &mut t.arena.verts,
                        &mut t.arena.vids,
                        idx_lane,
                        &mut t.seg.pipes,
                        &mut t.glyph.spheres
                    );
                    if is_print_fill(m) {
                        // The object row for this guid was pushed just above the match - .2 is flags.
                        t.obj.rows.last_mut().unwrap().flags |= Instance::FLAG_PRINT;
                    }
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
                        t.obj.rows.last_mut().unwrap().flags |= Instance::FLAG_OPEN;
                    }
                    t.obj.bounds.push(b); t.obj.spacing.push(mesh_spacing(b, m.number_of_vertices()));
                }
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());
                    let (bb, _) = push_mesh(
                        &bm,
                        ri,
                        vb,
                        &mut t.arena.verts,
                        &mut t.arena.vids,
                        &mut t.arena.idx,
                        &mut t.seg.pipes,
                        &mut t.glyph.spheres
                    );
                    t.obj.bounds.push(bb); t.obj.spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                }
                Geometry::Line(l) => { t.seg.ribbons.push(line_to_segment(l, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::Polyline(pl) => { t.seg.ribbons.extend(polyline_to_segments(pl, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::NurbsCurve(c) => { t.seg.ribbons.extend(nurbscurve_to_segments(c, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::Point(p) => { t.glyph.dots.push(point_to_glyph(p, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                // EVERY cloud takes the splat lane: split flat rows into share tables,
                // one draw record per cloud, and the per cloud point size rides the spacing spacing
                Geometry::PointCloud(pc) => {
                    // ABSOLUTE first point, counted from the start of the scene: the GPU table is
                    // cumulative while `cloud_pos` is only this upload's delta.
                    let first = cb + (t.cloud.pos.len() / 3) as u32;
                    let node_first = t.cloud.nodes.len() as u32;
                    push_cloud(pc, &mut t.cloud.pos, &mut t.cloud.col, &mut t.cloud.nrm, &mut t.cloud.nodes);
                    let node_count = t.cloud.nodes.len() as u32 - node_first;
                    t.cloud.draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc), node_first, node_count });
                    let px = if cloud_px > 0.0 { cloud_px } else { pc.point_size as f32 };
                    t.obj.bounds.push(None);
                    t.obj.spacing.push(px);
                }
                Geometry::NurbsSurface(s) => {
                    let mut sm = s.mesh();
                    if let Some(c) = s.facecolors.first() {
                        sm.set_objectcolor(c.clone());
                    }
                    let (b, _) = push_mesh(
                        &sm,
                        ri,
                        vb,
                        &mut t.arena.verts,
                        &mut t.arena.vids,
                        &mut t.arena.idx,
                        &mut t.seg.pipes,
                        &mut t.glyph.spheres
                    );
                    t.obj.bounds.push(b); t.obj.spacing.push(mesh_spacing(b, sm.number_of_vertices()));
                }
                Geometry::Plane(p) => { t.seg.ribbons.extend(plane_to_segments(p, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::OBB(b) => { t.seg.ribbons.extend(obb_to_segments(b, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::Element(e) => match e.geometry() {
                    ElementGeometry::Mesh(m) => {
                        let idx_lane = if is_print_fill(&m) {
                            if m.name == "text" { &mut t.arena.idx_text } else { &mut t.arena.idx_print }
                        } else {
                            &mut t.arena.idx
                        };
                        let (b, _) = push_mesh(
                            &m,
                            ri,
                        vb,
                            &mut t.arena.verts,
                            &mut t.arena.vids,
                            idx_lane,
                            &mut t.seg.pipes,
                            &mut t.glyph.spheres
                        );
                        if is_print_fill(&m) {
                            t.obj.rows.last_mut().unwrap().flags |= Instance::FLAG_PRINT;
                        }
                        t.obj.bounds.push(b); t.obj.spacing.push(mesh_spacing(b, m.number_of_vertices()));
                    }
                    ElementGeometry::BRep(b) => {
                        let mut bm = b.mesh();
                        bm.set_objectcolor(b.surfacecolor.clone());
                        let (bb, _) = push_mesh(
                            &bm,
                            ri,
                        vb,
                            &mut t.arena.verts,
                            &mut t.arena.vids,
                            &mut t.arena.idx,
                            &mut t.seg.pipes,
                            &mut t.glyph.spheres
                        );
                        t.obj.bounds.push(bb); t.obj.spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                    }
                    ElementGeometry::None => { t.obj.bounds.push(None); t.obj.spacing.push(0.0); },
                },
            }
```

**Replace with:**

```rust
            // ONE call, and ONE object row pushed here - never inside a producer.
            let row = walk_geometry(t, &cx, geom, ri);
            t.obj.rows[ri as usize].flags |= row.flags;
            t.obj.bounds.push(row.bounds);
            t.obj.spacing.push(row.spacing);
```

**Find** in `src/app/scene.rs`:

```rust
        let (mut fmin, mut fmax) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
        for (i, v) in t.arena.verts.iter().enumerate().skip(vert0) {
            if let Some(&ri) = t.arena.vids.get(i) {
                if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(ri as usize) {
                    grow_bounds(&mut fmin, &mut fmax, xform_point(xf, v.position));
                }
            }
        }

        for s in t.seg.pipes.iter().skip(pipe0).chain(t.seg.ribbons.iter().skip(seg0)){
            if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(s.instance_id as usize){
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p0));
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.p1));
            }
        }

        for s in t.glyph.spheres.iter().skip(sphere0).chain(t.glyph.dots.iter().skip(glyph0)){
            if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(s.instance_id as usize){
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, s.center));
            }
        }

        for &CloudDraw { first, count, instance: inst, .. } in t.cloud.draws.iter().skip(draw0){
            let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(inst as usize) else { continue };
            // `first` is absolute; `cloud_pos` starts at `cb`.
            for i in (first - cb) as usize..(first - cb + count) as usize {
                let p = [t.cloud.pos[i*3], t.cloud.pos[i*3+1], t.cloud.pos[i*3 + 2]];
                grow_bounds(&mut fmin, &mut fmax, xform_point(xf, p));
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if wprof { eprintln!("  walk bounds  {:?}", wlap.elapsed()); wlap = std::time::Instant::now(); }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = &wlap;
        for k in 0..3{
            t.min[k] = t.min[k].min(fmin[k]);
            t.max[k] = t.max[k].max(fmax[k]);
        }

        // 2D drawing sheets
        // flat linework - every PDF conversion gets paper space
        // keep kernel a real print
        // 3D model files keep screen-constant px linework
        // Planar = thin alon the SHEET's normal
        // The 99% path - translation only place, normal is Z+
        // reuseses the z-extent accumulated aboce - no extra work at all
        // only a rotated placement pays one dot-product pass over this file's new rows
        let n = place.transform_vector(&Vector::new(0.0, 0.0, 1.0));
        let thickness = if n[0].abs() < 1e-9 && n[1].abs() < 1e-9 {
            fmax[2] - fmin[2]
        } else {
            let (nx, ny, nz) = (n[0] as f32, n[1] as f32, n[2] as f32);
            let (mut dmin, mut dmax) = (f32::INFINITY, f32::NEG_INFINITY);
            let mut span = |p: [f32; 3]| {
                let d = p[0] * nx + p[1] * ny + p[2] * nz;
                dmin = dmin.min(d);
                dmax = dmax.max(d);
            };
            for (i, v) in t.arena.verts.iter().enumerate().skip(vert0){
                if let Some(&ri) = t.arena.vids.get(i){
                    if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(ri as usize) {
                        span(xform_point(xf, v.position));
                    }
                }
            }
            for s in t.seg.pipes.iter().skip(pipe0).chain(t.seg.ribbons.iter().skip(seg0)){
                if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(s.instance_id as usize){
                    span(xform_point(xf, s.p0));
                    span(xform_point(xf, s.p1));
                }
            }
            for g in t.glyph.spheres.iter().skip(sphere0).chain(t.glyph.dots.iter().skip(glyph0)){
                if let Some(ObjectBase { model: xf, .. }) = t.obj.rows.get(g.instance_id as usize) {
                    span(xform_point(xf, g.center));
                }
            }
            dmax - dmin
        };

        let planar = thickness.is_finite() && thickness.abs() < 1e-3;

        if planar {
            // Every row of this file is page content. The ink lanes read the bit to drop their
            // lift (a sheet's fills no longer write depth, so there is nothing to lift off), and
            // that is what lets the lettering pass sit on top of the linework.
            for o in t.obj.rows.iter_mut().skip(obj0) {
                o.flags |= Instance::FLAG_SHEET;
            }
            for s in t.seg.pipes.iter_mut().skip(pipe0).chain(t.seg.ribbons.iter_mut().skip(seg0)){
                // A flat sheet is paper: every pen becomes a world-mm radius so widths behave
                // like plotter pens. encode_width already returns a positive mm radius for any
                // authored width, so only the unset default (0.0) needs a value here - 0.5 mm,
                // the usual hairline. This used to read `radius < 0` because widths arrived as
                // NEGATIVE multipliers; they are millimetres now.
                s.radius = if s.radius > 0.0 {
                    s.radius
                } else {
                    0.5
                }
            }
        }

```

**Replace with:**

```rust
        // The two file sweeps (`walk/bounds.rs`), both over this file's rows only.
        let base = Baselines {
            vert: vert0, seg: seg0, pipe: pipe0, sphere: sphere0,
            glyph: glyph0, obj: obj0, draw: draw0, cloud_base: cb,
        };
        let (fmin, fmax) = file_extent(t, &base);
        for k in 0..3{
            t.min[k] = t.min[k].min(fmin[k]);
            t.max[k] = t.max[k].max(fmax[k]);
        }
```

**Find** in `src/app/scene.rs`:

```rust
        let (fmin, fmax) = file_extent(t, &base);
        for k in 0..3{
            t.min[k] = t.min[k].min(fmin[k]);
            t.max[k] = t.max[k].max(fmax[k]);
        }
```

**Replace with:**

```rust
        let (fmin, fmax) = file_extent(t, &base);
        for k in 0..3{
            t.min[k] = t.min[k].min(fmin[k]);
            t.max[k] = t.max[k].max(fmax[k]);
        }
        let thickness = sheet_thickness(t, &base, &place, fmin, fmax);
        if thickness.is_finite() && thickness.abs() < 1e-3 {
            mark_sheet(t, &base);
        }

```

**Find** in `src/app/scene.rs`:

```rust
/// The kernel's `width` is in MILLIMETRES - the drawings lane talks in 0.09-0.5 mm plot pens
/// and `Line`/`Polyline` default to 1.0. This used to return `-(w)`, and a NEGATIVE radius means
/// "multiply the global pen" to every shader - so a 30 mm polyline became 2 px x 30 = a 60 px
/// half-width, a 120 px slab. Millimetres were being read as a multiplier.
///
/// Now: an explicit width is a world-mm RADIUS (half the width, positive => the projected
/// branch), and only the untouched 1.0 default falls back to the screen-constant pen. That
/// keeps mesh edges - which never set a width - at a zoom-independent 2 px, while a pen someone
/// actually authored measures what it says.

/// RGBA8 in one word, low byte red - the layout `unpack4x8unorm` expects in WGSL.

/// A unit vector in 16 bits, octahedral: project onto the octahedron, fold the lower hemisphere
/// out across the diagonals, and store the two coordinates as signed bytes. ~1.4 degrees of error,
/// which is generous for a value only ever used for the SIGN of a dot product.

/// Opaque black, packed. The wireframe's default pen, and what a dense mesh's edges draw as.

/// The two faces an edge belongs to, packed into one word for the shader's facing test.
///
/// `FACING_UNKNOWN` means "no adjacency known, always draw" - see the constant for why it is the
/// all-ones word and not 0.
```

**Replace with:**

```rust

```

**Find** in `src/app/scene.rs`:

```rust
/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
/// Half-extent in world mm (a 1 m quare)


/// A box is its 12 edges: bottom loop, top loop, four verticals - `corner()` orders tge bottom face
/// face 0-3 and the top 4-7 with i / i+4 vertically aligned.
/// The OBB type carries no pen, so the edges draw black at screen-constant width (radius 0.0 = global default)
```

**Replace with:**

```rust

```

**Find** in `src/app/scene.rs`:

```rust
            mark_sheet(t, &base);
        }

        // The walk is done and the tables are about to be uploaded
```

**Replace with:**

```rust
            mark_sheet(t, &base);
        }


        // The walk is done and the tables are about to be uploaded
```

**Find** in `src/app/scene.rs`:

```rust
        });

    }

}







/// Typical distance between a mesh's vertices
```

**Replace with:**

```rust
        });

    }

}



/// Typical distance between a mesh's vertices
```

**Find** in `src/app/scene.rs`:

```rust
    (local_bounds, closed)
}



/// Above this many triangles a mesh draws as TRIANGLES ONLY
```

**Replace with:**

```rust
    (local_bounds, closed)
}


/// Above this many triangles a mesh draws as TRIANGLES ONLY
```

**Find** in `src/app/scene.rs`:

```rust
/// The raw lane's rows, written straight into the shared table,
/// reading the kernel's flat arrays rather than get_point/get_color (no per_point allocs)

/// Median distance between consecutive points - a scanner emits angular neighbours in order,
/// so successive points are usually adjacent on the surface, which makes this a cheap and
/// honest estimate of the clouds's point spacing (world units).
/// Potree gets the same number from its octree, we sample it.
/// Drives the attenuated world-sized splat radius.

```

**Replace with:**

```rust

```

## 7. Proving nothing changed

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets --target x86_64-unknown-linux-gnu
cargo xtest
python3 docs/_replay_check.py --moves <end-of-49 tree> /tmp/w50 docs/51-walk-sinks.md
./docs/_gate.sh                # twice
cargo run -q --release --example check_determinism --target x86_64-unknown-linux-gnu -- assets/pb/lion.pb
cargo run -q --release --example check_lean        --target x86_64-unknown-linux-gnu -- assets/pb/mesh_bunny.pb
```

```text
0 errors, warning set unchanged
test result: ok. 4 passed
docs/51-walk-sinks.md: 42 ops, 0 failed
gate OK                        (both runs)
lion.pb: DETERMINISTIC
mesh_bunny.pb: IDENTICAL
```

`check_lean` matters most here: it compares the walked tables byte for byte between two loads of
the same file, so a producer that dropped a column would show as a row-count mismatch rather than
as a wrong picture.

## 8. What you can now do in one line

Change every polyline in a scene without opening an engine file.

**8a.** **Find** in `src/app/walk/curves.rs`:

```rust
    let color = pack_rgba(pl.linecolor.to_f32());
    pts.windows(2)
```

**Replace with:**

```rust
    let color = 0xff0000ff;
    pts.windows(2)
```

```bash
cargo run -q --release --example selftest --target x86_64-unknown-linux-gnu -- \
    /tmp/red.ppm assets/scenes/drawings_rotated.toml
```

```text
wrote /tmp/red.ppm  900x700  non-background pixels: 25061 (4.0%)
```

**4,237 pixels change** across a sheet of 191,605 segments — reddish pixels go 4,878 → 6,333 — and
nothing under `engine/` was opened. The producer decides what a row CARRIES; the family decides
what a row MEANS.

**8b.** Put it back. **Find** in `src/app/walk/curves.rs`:

```rust
    let color = 0xff0000ff;
    pts.windows(2)
```

**Replace with:**

```rust
    let color = pack_rgba(pl.linecolor.to_f32());
    pts.windows(2)
```

## 9. What is deliberately not here

- **`walk/mesh.rs`.** `push_mesh` is still 314 lines and 8 parameters in `scene.rs`, and
  `is_print_fill`/`mesh_spacing`/`push_mesh` are `pub(crate)` so `walk_geometry` can call them.
  Lesson **51** splits it and takes all three.
- **Sink accessors.** `walk_geometry` takes `&mut Upload` and reaches its fields directly, because
  disjoint field borrows are what makes the mesh arm compile at all (§5). Narrowing further needs
  the mesh split first.
- **`chain_table` / `compartments_hold`.** The tests that assert this table are worth having once
  the mesh producers exist to assert about. **51**.
- **`enum Spacing { World, Pixels }`.** `Row::solid` and `Row::point_size_px` name the unit at the
  write site; the type itself waits for the first row that needs both.

## 10. Expected state

```bash
wc -l src/app/scene.rs
grep -c 'Geometry::' src/app/scene.rs
ls src/app/walk/
```

```text
739

0

bounds.rs  cloud.rs  curves.rs  encode.rs  frames.rs  mod.rs  points.rs
```

| | end-of-49 | end-of-50 |
|---|---|---|
| `app/scene.rs` | 1,333 | **739** |
| `Geometry::` arms in `scene.rs` | 16 | **0** |
| hand-pushed object columns | 14 | **0** |
| `last_mut()` reach-backs | 3 | **0** |
| files under `app/` | 2 | **11** |

## Recap

```text
45-49 restructured the engine by ROW FORMAT. 50 starts the other half, and it turns on the other
axis: the walk is one file per KERNEL TYPE, because a kernel type is what a producer starts from.

The thirteen arms left scene.rs for walk_geometry, and with them went the fourteen hand-pushes of
the two per-object columns and the three reach-backs that patched a row already pushed. A
producer now RETURNS a Row and cannot push one, so the object table's count cannot drift. Row
also names the unit its one spacing float carries, and it is where a face or edge id will go when
lesson 120 needs one.

scene.rs is 739 lines and no longer knows what a Geometry is.
```

## Edited

`app/manifest.rs`, `app/knobs.rs`, `app/walk/{mod,encode,curves,points,frames,cloud,bounds}.rs`
(all NEW) · `app/scene.rs` (1,333 → 739) · `app/mod.rs`.

## Next

Lesson [52](52-adapters.md) — **five types, one body.**

```bash
awk '/fn push_mesh/,/^\}$/' src/app/scene.rs | wc -l
grep -c 'push_mesh(' src/app/walk/mod.rs
```

`push_mesh` is 314 lines and eight parameters, called from five arms, and its second return value
is discarded by three of them — which is exactly how `Element(Mesh)` lost `FLAG_OPEN`. It splits
by job, and BRep and NurbsSurface become three- and two-line adapters that re-enter the mesh
producer with a `MeshOpts`.
