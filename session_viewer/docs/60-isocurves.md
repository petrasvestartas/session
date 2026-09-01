# 60 Iso-curves — the lines that make a surface read

> **Big picture.** *Phase 4b.* Shading alone doesn't say "surface" — a smooth gray blob could be
> anything. What makes CAD surfaces legible is their **linework**: the boundary curves and the u/v
> iso-parameter lines (the grid every Rhino surface wears). Today a surface wears the WRONG
> linework: `walk/surface.rs` hands its tessellation to `walk_mesh`, which gives it the mesh
> treatment — tubes along the triangle edges — and that is exactly the look that says "mesh". This
> is a short lesson because the infrastructure already exists: iso lines are more `point_at`
> samples through 31's tube path, and 59 left a cache entry with a hole in it shaped like them.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a curved surface patch with boundary curves and interior u v iso lines sampled from point_at" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <path d="M 40,90 C 120,40 220,40 300,80 L 290,110 C 210,72 130,72 55,115 Z" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <path d="M 60,100 C 130,55 220,52 292,88" fill="none" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 50,105 C 125,62 215,58 296,95" fill="none" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 110,63 L 100,98" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <path d="M 200,52 L 195,86" stroke="#6fb3ff" stroke-width="0.8" opacity="0.6"/>
  <text x="170" y="118" fill="#888" text-anchor="middle" font-size="10">boundary (dark) + u/v iso lines (light) — parameter-space lines, not triangle edges</text>
  <g transform="translate(380,18)">
    <text x="0" y="14" fill="#d7dae0">iso lines = point_at along one FIXED parameter</text>
    <text x="0" y="32" fill="#666" font-size="10">u = ¼, ½, ¾ (v sweeps) and v = ¼, ½, ¾ (u sweeps)</text>
    <text x="0" y="56" fill="#d7dae0">cached in Tess beside the mesh — one lifecycle</text>
    <text x="0" y="74" fill="#666" font-size="10">tubes in the SOLID lane: they protrude, no z-fight</text>
  </g>
</svg>

## Files we touch

| file | what |
|---|---|
| `src/app/walk/surface.rs` | `surface_linework`, and the adapter fills and pushes `Tess.ink` |
| `src/app/walk/mod.rs` | `Tess` grows its second field; the two Mesh arms say `Edges::Draw` |
| `src/app/walk/mesh.rs` | the `Edges` flag, the fifth `MeshOpts` field, and one two-line gate |
| `src/app/walk/brep.rs` | one word — the BRep adapter says `Edges::Draw` until 61 flips it |

Four files, and only one of them grows: the producer half lives with the type it produces from,
which is what `walk/` is for.

## Step 1 — the extractor, and the entry it fills

Two decisions carry the whole lesson.

**Where the lines come from.** Not from the tessellation. The boundary is the domain's four edges
and the interior grid is a light 3×3 of lines at ¼, ½ and ¾ of each direction, each one sampled
with `point_at(u, v)` along one FIXED parameter. Those are lines OF the parameterisation — orbit
and they foreshorten with the curvature, because they lie on the mathematical surface and not on
a triangle. Kernel honesty rides along: `domain(dir)` and `point_at(u, v)` both return `Option`,
so a degenerate surface yields no lines and a sample that cannot answer breaks the polyline rather
than joining two points that were never adjacent.

**Where they go.** Into `t.seg.pipes`, the SOLID lane — never `ribbons`. A flat ribbon lies in the
skin it decorates and z-fights it at every grazing angle; a tube protrudes by construction, which
is 31's whole design, so there is no depth-bias knob here because none is needed. `radius: 0.0` is
31's other convention: the screen-constant global pen.

And because they are pipes in LOCAL space, `walk/bounds.rs` sweeps them into the file extent with
the mesh ink already there — a surface whose lines reach past its own tessellation measures right,
with no special case anywhere.

`walk/surface.rs` is small enough to state whole, and the three additions are worth reading in
place: the extractor, the `ink` half of the cache entry, and the stamp-at-push loop.

**Create `src/app/walk/surface.rs`**

```rust
//! `walk/surface.rs` - the NurbsSurface adapter, and the one producer it owns.
//!
//! It re-enters the mesh producer exactly like `brep.rs`, and three things are its own. The colour
//! comes from `facecolors.first()` and not from `surfacecolor` - the single divergence from BRep.
//! The tessellation is CACHED: built the first time this guid is walked, read by every walk after
//! it. And the LINEWORK is its own row format after all: a surface is drawn with the lines of its
//! parameterisation - the domain boundary and a light interior grid - and never with the triangle
//! edges of a tessellation nobody asked to see.

use session_rust::NurbsSurface;

use crate::engine::gpu::segments::FACING_UNKNOWN;
use crate::engine::gpu::{CylinderSegment, Upload};

use super::encode::pack_rgba;
use super::mesh::{Edges, MeshOpts, walk_mesh};
use super::{Row, Tess, WalkCx};

/// Interior iso lines per direction, as fractions of the domain.
const ISO_FRACS: [f64; 3] = [0.25, 0.5, 0.75];
/// Samples along one line. A constant and not a setting: a density change is a cache clear away,
/// never a per-frame cost - and the day it becomes a setting it scales off the surface's span
/// counts, the way 58 scales a curve's, not off a number typed in a shader.
const ISO_SAMPLES: usize = 48;

/// Boundary + iso lines for one surface, LOCAL space, `instance_id` 0 - the walk stamps the row.
///
/// Both kernel calls are `Option`-honest and this respects both: a surface with no domain yields
/// no lines at all, and a parameter the surface cannot evaluate ends the current polyline instead
/// of bridging the gap.
fn surface_linework(s: &NurbsSurface) -> Vec<CylinderSegment> {
    let mut out = Vec::new();
    let (Some((u0, u1)), Some((v0, v1))) = (s.domain(0), s.domain(1)) else { return out };
    let dark = pack_rgba([0.05, 0.05, 0.05, 1.0]);
    let light = pack_rgba([0.35, 0.35, 0.35, 1.0]);
    // One polyline along a fixed u (v sweeps) or along a fixed v (u sweeps).
    let mut line = |fix: f64, u_fixed: bool, color: u32| {
        let mut prev: Option<[f32; 3]> = None;
        for i in 0..=ISO_SAMPLES {
            let t = i as f64 / ISO_SAMPLES as f64;
            let (u, v) = if u_fixed { (fix, v0 + (v1 - v0) * t) } else { (u0 + (u1 - u0) * t, fix) };
            let Some(p) = s.point_at(u, v) else { prev = None; continue };
            let p = p.to_f32();
            if let Some(q) = prev {
                out.push(CylinderSegment {
                    p0: q, radius: 0.0, p1: p, instance_id: 0, color, facing: FACING_UNKNOWN,
                });
            }
            prev = Some(p);
        }
    };
    line(u0, true, dark);   // boundary: the four edges of the domain
    line(u1, true, dark);
    line(v0, false, dark);
    line(v1, false, dark);
    for f in ISO_FRACS {    // interior grid, lighter
        line(u0 + (u1 - u0) * f, true, light);
        line(v0 + (v1 - v0) * f, false, light);
    }
    out
}

/// Walk a surface as its tessellation plus its own linework, deriving both at most ONCE per guid.
///
/// `s.mesh()` is either a grid remesh or - when the kernel already carries `m_mesh` - a deep clone
/// of every vertex, normal and index, and `set_objectcolor` then walks the vertices again. That is
/// per-surface, per-walk work, and `rebuild` re-walks every document, so it lands on interaction
/// and not just on load. The linework is sampled beside it, in the same entry, and dies with it.
pub(crate) fn walk_surface(s: &NurbsSurface, t: &mut Upload, cx: &mut WalkCx, ri: u32) -> Row {
    let base_off = cx.vert_base;        // read before the cache borrows `cx`
    let key = (cx.guid.to_string(), 0); // one String against one remesh - the trade is not close
    let tess = cx.caches.tess.entry(key).or_insert_with(|| {
        let mut m = s.mesh();
        if let Some(c) = s.facecolors.first() { m.set_objectcolor(c.clone()); }
        Tess { mesh: m, ink: surface_linework(s) }
    });
    // The entry holds instance 0. Rows are POSITIONAL - the same guid lands on a different row as
    // soon as a document ahead of it changes - so the row is stamped HERE, on the copy that goes
    // to the GPU, and never in the cache.
    t.seg.pipes.extend(tess.ink.iter().map(|seg| CylinderSegment { instance_id: ri, ..*seg }));
    walk_mesh(&tess.mesh, t, &MeshOpts {
        ri, base_off, sheet_lanes: false, allow_open: false, edges: Edges::Suppress,
    })
}
```

Three borrows are alive in that function and none of them fights another: `tess` borrows
`cx.caches`, `t` is a separate `&mut` the caller handed in, and `base_off` was copied out of `cx`
before the cache took hold of it — 59's habit, and this is the lesson that makes it pay.

## Step 2 — the cache entry grows its second field

`Tess` was written for this in 59: a struct rather than a bare `Mesh`, because a mesh is not the
only thing a walk derives from one shape and then throws away. Linework shares the tessellation's
lifecycle EXACTLY — born with the shape, dead on the edit that reshapes it — so it shares the
entry, and there is one eviction story rather than two.

`walk/mod.rs` has to name the row type to hold it.

**Find** in `src/app/walk/mod.rs`:

```rust
use crate::engine::gpu::{CloudDraw, Upload};
```

**Replace with:**

```rust
use crate::engine::gpu::{CloudDraw, CylinderSegment, Upload};
```

Then the field itself.

**Find** in `src/app/walk/mod.rs`:

```rust
pub struct Tess {
    pub mesh: Mesh,
}
```

**Replace with:**

```rust
pub struct Tess {
    pub mesh: Mesh,
    /// The linework drawn ON this shape, in LOCAL space with `instance_id` 0: a surface's
    /// boundary and iso curves here, a BRep face's real edge curves at 61. Zero because rows are
    /// positional - the walk stamps the row it is filling as it copies these to the GPU.
    pub ink: Vec<CylinderSegment>,
}
```

`Caches`'s own doc comment promised this map would grow; one of the two things it was waiting for
has arrived.

**Replace-all** `src/app/walk/mod.rs` `Two maps today, and both grow: 60 gives Tess its ink, 62 the trim loops;` → `Two maps today, and both grow: 62 gives Tess the trim loops;` (1 hit)

## Step 3 — `Edges::Suppress`: one flag, never a fork

`walk_mesh` always runs its ink half — the fused topology pass, the width map, the tubes, the
vertex spheres — unless one of its three gates fires (a dense mesh, a print fill, `VIEWER_NO_EDGES`).
A surface wants the vertex and index push and the bounds, and none of the decoration. That is one
more decision the CALLER owns, so it is one more named field on `MeshOpts` — the struct 52 created
for exactly this kind of answer.

**Find** in `src/app/walk/mesh.rs`:

```rust
    pub allow_open: bool,
}
```

**Replace with:**

```rust
    pub allow_open: bool,
    /// Draw this tessellation's wireframe, or leave the faces bare? A surface here and a BRep at
    /// 61 carry linework of their own - iso curves, edge curves - and triangle edges drawn on top
    /// of it are the one look that says "mesh" about something that is not one.
    pub edges: Edges,
}

/// Whether `walk_mesh` runs its ink half at all.
///
/// A FLAG, and never a forked function: a faces-only copy of `walk_mesh` would duplicate the three
/// gates, the bounds sweep and the `FLAG_OPEN` decision, and two copies of that drift within a
/// lesson of each other. A field and not a fifth PARAMETER for the same reason `sheet_lanes` and
/// `allow_open` are fields - `MeshOpts` exists so that a caller's decision has a name.
#[derive(Clone, Copy)]
pub(crate) enum Edges {
    /// A mesh IS its tessellation: the wireframe is the shape, so draw it.
    Draw,
    /// Faces and bounds only - the caller draws better lines than these itself.
    Suppress,
}
```

`MeshOpts`'s own doc comment counts its fields out loud, so it counts one more.

**Replace-all** `src/app/walk/mesh.rs` `How one mesh is walked. Four fields against ` → `How one mesh is walked. Five fields against ` (1 hit)

**Replace-all** `src/app/walk/mesh.rs` `and two of them` → `and three of them` (1 hit)

The gate goes in immediately after the vertex and index push, which is the earliest point at which
`local_bounds` exists — and BEFORE the dense-mesh early-out, deliberately.

**Find** in `src/app/walk/mesh.rs`:

```rust
    mark("vert+idx push", &mut lap);
```

**Add below it:**

```rust

    // A surface draws the lines of its parameterisation, not the edges of a tessellation nobody
    // asked to see - so the decoration is suppressed here, and the BOUNDS are kept. Read the
    // dense gate below with that in mind: it computes `local_bounds` and then returns `None` for
    // it, so a dense mesh loses the facing cull's premise silently. That is a real hole, known and
    // left where it is; suppression must not copy it.
    if matches!(o.edges, Edges::Suppress) {
        return Row::solid(local_bounds, mesh_spacing(local_bounds, m.number_of_vertices()), print_flag);
    }
```

For reference, the dense gate it now sits above is unchanged and reads:

```rust
    if rm.indices.len() / 3 > MESH_RAW_MIN {
        return Row::solid(None, mesh_spacing(None, m.number_of_vertices()), print_flag);
    }
```

That `None` is the hole the new comment names. It is not fixed here, because fixing it changes
what dense meshes draw and this lesson is about surfaces; it is a one-word change the day
something needs dense-mesh bounds.

## Step 4 — the three callers say `Draw`

A new field with no default means the compiler lists every construction site for you, and there
are four: two Mesh arms in `walk/mod.rs`, the BRep adapter, and the surface adapter Step 1 already
rewrote. Mechanical, and the point of doing it this way — nothing is opted in by silence.

**Find** in `src/app/walk/mod.rs`:

```rust
use mesh::{MeshOpts, walk_mesh};
```

**Replace with:**

```rust
use mesh::{Edges, MeshOpts, walk_mesh};
```

**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::Mesh(m) => walk_mesh(m, t, &MeshOpts {
            ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: true,
        }),
```

**Replace with:**

```rust
        Geometry::Mesh(m) => walk_mesh(m, t, &MeshOpts {
            ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: true, edges: Edges::Draw,
        }),
```

**Find** in `src/app/walk/mod.rs`:

```rust
            ElementGeometry::Mesh(m) => walk_mesh(&m, t, &MeshOpts {
                ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: false,
            }),
```

**Replace with:**

```rust
            ElementGeometry::Mesh(m) => walk_mesh(&m, t, &MeshOpts {
                ri, base_off: cx.vert_base, sheet_lanes: true, allow_open: false, edges: Edges::Draw,
            }),
```

The BRep adapter keeps its wireframe for now — it has no curve network on the GPU yet, and 61 is
where it gets one and flips this word.

**Find** in `src/app/walk/brep.rs`:

```rust
use super::mesh::{MeshOpts, walk_mesh};
```

**Replace with:**

```rust
use super::mesh::{Edges, MeshOpts, walk_mesh};
```

**Find** in `src/app/walk/brep.rs`:

```rust
    walk_mesh(&bm, t, &MeshOpts { ri, base_off, sheet_lanes: false, allow_open: false })
```

**Replace with:**

```rust
    walk_mesh(&bm, t, &MeshOpts {
        ri, base_off, sheet_lanes: false, allow_open: false, edges: Edges::Draw,
    })
```

## The parameter ceiling this lesson was warned about

The plan for this lesson said it would push the mesh producer to five parameters and that anything
later would have to ride a context struct instead. Check what it costs today:

```bash
grep -h 'pub(crate) fn walk_mesh' src/app/walk/mesh.rs
grep -rn '&MeshOpts {' src/ | wc -l
grep -h 'pub(crate) fn walk_surface' src/app/walk/surface.rs
```

```text
pub(crate) fn walk_mesh(m: &Mesh, t: &mut Upload, o: &MeshOpts) -> Row {
4
pub(crate) fn walk_surface(s: &NurbsSurface, t: &mut Upload, cx: &mut WalkCx, ri: u32) -> Row {
```

Three parameters and four. The ceiling is real — `src/architecture.rs` fails the build at six, and
`walk_surface` is one slot from it, which is why 59 handed it `cx` instead of a fifth argument —
but it never binds on the producer, because 52 took the pressure off: eight positional arguments
became `MeshOpts`, and a struct absorbs a field without a signature moving. That is the whole
argument for the rule in one diff — `walk_mesh` took a new input and its signature did not change,
so 61 and 62 arrive with the same room this lesson had.

## Step 5 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo xtest architecture
```

- The 59 surface fixture now wears a dark boundary and a light 3×3 interior grid that **follows
  the curvature** (the lines are lines OF the parameterisation — orbit and watch them foreshorten
  correctly), and **no tessellation wireframe** — put a mesh box beside it: triangle edges say
  "mesh", iso lines say "surface", instantly.
- Zoom to a grazing angle: **no z-fighting flicker** between lines and skin (tubes protrude —
  31's design; there is no bias to tune).
- Hide and unhide the surface, which re-walks every document. The lines come back with no
  extraction cost: they were sampled once, into the same entry as the tessellation.
- `cargo xtest architecture` stays green: no function crossed five parameters, and `MeshOpts`
  holds no third field sharing a name stem.

## Recap

```
Ch 58: curves — sampled once per guid, in Caches.
Ch 59: surfaces — tessellated once per guid, into Tess, which is a struct precisely so this
       lesson has somewhere to put its second half.
Ch 60: LINEWORK. surface_linework in walk/surface.rs: boundary (the domain's four edges,
       near-black) + interior iso lines (¼ ½ ¾ per direction, lighter), point_at along one
       fixed parameter, 48 samples per line, Option-honest at both kernel calls (no domain =
       no lines; no sample = break the polyline) -> t.seg.pipes, the SOLID lane, because a flat
       ribbon would z-fight the skin it lies on while a tube protrudes. LOCAL space with
       instance 0 in the cache and the real row stamped at push, since rows are positional
       across rebuilds. Cached WITH the tessellation: Tess gains ink, one lifecycle, one
       eviction story. walk_mesh gains Edges - a FLAG on MeshOpts, never a forked function -
       and Edges::Suppress returns the real local_bounds, which is where the dense gate's
       return of None becomes visible as a known, deliberately unfixed hole. Density knobs stay
       constants; scale them by span counts (58's rule) when they become settings, because a
       density change is a cache clear away and never a per-frame cost.
```

Edited: `src/app/walk/surface.rs` (`surface_linework`, the `ink` half of the entry, the
stamp-at-push loop, `Edges::Suppress`), `src/app/walk/mod.rs` (`Tess.ink` and the two Mesh arms),
`src/app/walk/mesh.rs` (`Edges`, the fifth `MeshOpts` field, the suppression gate) and
`src/app/walk/brep.rs` (`Edges::Draw`, until 61).

## Next

Lesson [61](61-brep.md) — the boundary representation: many faces and shared edges as **one
object**. It has drawn since the first walk, through the same adapter shape you just edited
(`b.mesh()` → `walk_mesh`), and it carries the same two debts a surface carried: `mesh()` is a
real tessellation pipeline with no kernel-side cache, and its edges draw as tessellation wireframe
instead of the kernel's own curve network. Both are paid with machinery that now exists — the
`Tess` entry, 58's sampler, and the flag you just gave `MeshOpts`.
