# 58 NurbsCurve — what already draws, made honest and reusable

> **Big picture.** *Phase 4b — curved geometry, pulled AHEAD of the interaction phases: every
> kernel type is rendered CORRECTLY and CHEAPLY before any tool touches one.* One structural fact
> shaped this phase when it was first written: nurbs objects lived only in their own collections
> (`session.objects.nurbscurves`, …), outside `session.lookup` — and every map the viewer keeps
> had to remember both sources; the archive forgot repeatedly, each forget a bug. That audit
> became kernel-gap #4, and **the kernel has since been fixed**: `NurbsCurve` and `NurbsSurface`
> are `Geometry` variants, registered in `lookup` on add and load, in all three languages — which
> is why your walk ALREADY draws curves: the arm rode in with 34/35, and 51 gave its producer a
> file of its own. `NurbsSurfaceTrimmed` is the one type still outside `Geometry` entirely, and
> 62 is where that bill comes due. This lesson is therefore not "add the type" — it is: read what
> the producer really does, fix its one trap, and factor its samples into a CACHE that the scene
> BVH (67), the pick (70) and the draw tool (86) will all read.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a nurbs curve is sampled at parameters into a polyline whose segments feed the cylinder path; the sample count follows curve size with a span floor" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <path d="M 30,100 C 110,20 190,120 270,50" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <g fill="#d7dae0"><circle cx="30" cy="100" r="3"/><circle cx="98" cy="63" r="3"/><circle cx="152" cy="78" r="3"/><circle cx="208" cy="76" r="3"/><circle cx="270" cy="50" r="3"/></g>
  <text x="150" y="122" fill="#888" text-anchor="middle">point_at(t) samples → polyline → 31's tubes</text>
  <g transform="translate(360,20)">
    <text x="0" y="16" fill="#d7dae0">n = size-scaled, span floor, clamped</text>
    <text x="0" y="36" fill="#666" font-size="10">uniform — budget follows size and span</text>
    <text x="0" y="50" fill="#666" font-size="10">count, NOT measured curvature</text>
    <text x="0" y="78" fill="#888">samples cached per guid</text>
    <text x="0" y="96" fill="#666" font-size="10">(the box, the pick and the ghost all reread them)</text>
  </g>
</svg>

## Files we touch

| file | what |
|---|---|
| `src/app/walk/curves.rs` | `sample_curve` split out of `nurbscurve_to_segments`; the span floor |
| `src/app/walk/mod.rs` | `Caches`; `WalkCx` gains the guid and a borrow of them; the arm becomes get-or-sample |
| `src/app/scene.rs` | `Scene.caches` — the OWNER, because the samples must outlive the walk |

Three files, and the split between them is the whole architecture of this phase. `walk/curves.rs`
is a producer: geometry in, rows out, no sink, no state. `walk/mod.rs` dispatches and owns every
push. `app/scene.rs` owns everything that survives a walk. A cache is state that survives a walk,
so it can only live in the third — and the first two get a borrow of it.

## Step 0 — read the producer you already have

Since 51 the NurbsCurve path is two halves. The arm in `src/app/walk/mod.rs` today reads:

```rust
        Geometry::NurbsCurve(c) => { t.seg.ribbons.extend(nurbscurve_to_segments(c, ri)); Row::none() }
```

`t.seg.ribbons` is the FLAT lane — free-floating linework, not ink lying on a surface — and
`Row::none()` says a curve earns no bounds, no spacing and no extra flags. Everything else happens
inside `nurbscurve_to_segments` in `src/app/walk/curves.rs`, which does three things worth
understanding before changing any:

- **It bounds the curve by its CONTROL NET** — a NURBS curve never leaves its control points'
  convex hull, so the CV box is a free, exact-enough size estimate. The loop handles **rational**
  curves: a weighted CV is stored `[x·w, y·w, z·w, w]`, so it divides by `w` (with a `w == 0`
  guard) to get the real point. Keep that loop — 67's box and 100's CV handles reread it.
- **It scales the sample count by SIZE**: `n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64)`
  — a 2 mm glyph outline gets 4 segments, a metre-long arc ~50.
- **It then IS a polyline**: `point_at(t)` at `n+1` even parameters, consecutive pairs →
  `CylinderSegment`s with the curve's own `linecolors`/`width` — 31's tube lane, nothing special.

## Step 1 — the trap, and the span floor

Two things are wrong with `n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64)`, and they're
the same two traps as `mesh_q`'s `0.005` (62 meets it): the `0.2` is an **absolute length in
kernel units** (a metre-unit file samples ~14× sparser than the same curve authored in
millimetres), and size says nothing about **complexity** — a 40-knot S-curve the size of a coin
gets the same 4 segments as a straight stub. Curvature-exact sampling is overkill for a display
polyline; the honest cheap signal for complexity is the **span count** (each span can bend
independently).

**Find** in `src/app/walk/curves.rs`:

```rust
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);
```

**Replace with:**

```rust
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    // Size-scaled (0.2 is kernel units - mm here; a unit-relative tolerance is the upgrade),
    // with a SPAN floor: each span can bend independently, so complexity has a vote too.
    // `span_count()` walks the knot vector and allocates - which is affordable exactly because
    // Step 2 makes this whole function run once per guid instead of once per walk.
    let n = ((size / 0.2).sqrt().ceil() as usize)
        .clamp(4, 64)
        .max((c.span_count() * 8).min(512));
```

A floor is a floor, so be honest about what it moves: `span_count()` answers 0 for a degenerate
curve (those are untouched) and 1 for a single-span cubic, whose count therefore becomes at least
8. Anything the size term already put at 8 or more is byte-identical; small single-span curves —
glyph outlines, short fillets — go from 4 to 8, and a 40-span curve goes from 64 to 320.

Honest label: this is still *uniform* sampling — curvature is never measured, and zooming close
enough will always find chords (Step 3 makes you look at them). When that bites, the drop-in is
chord-error refinement — subdivide until each segment's midpoint stays within `tol` of its chord.
Quoted here for reference, not for typing:

```rust
/// Adaptive by chord error: straight regions cost 1 segment, curvature concentrates samples.
fn sample_curve_adaptive(nc: &NurbsCurve, tol: f64) -> Vec<Point> {
    let (t0, t1) = nc.domain();
    let mut out = vec![nc.point_at(t0)];
    let mut stack = vec![(t0, t1)];
    while let Some((a, b)) = stack.pop() {
        let (pa, pb) = (nc.point_at(a), nc.point_at(b));
        let m = 0.5 * (a + b);
        let pm = nc.point_at(m);
        let chord_mid = pa.clone() + (pb.clone() - pa.clone()) * 0.5;
        if (pm - chord_mid).magnitude() <= tol {
            out.push(pb);                                  // flat enough — keep the chord
        } else {
            stack.push((m, b)); stack.push((a, m));        // split, left half first (LIFO)
        }
    }
    out
}
```

## Step 2 — factor the samples out, and cache them

Today the samples are born as `[f32; 3]`, converted inline, and die inside the segment build.
Three future consumers want the SAME points in f64: 67's box, 70's ray↔segment pick, 86's ghost.
Sampling per pick would be wasteful, per frame a bug — so the samples become a per-guid cache,
computed once per curve, surviving rebuilds.

The producer takes `Point` in and hands `Point` out now, so it needs the type.

**Find** in `src/app/walk/curves.rs`:

```rust
use session_rust::{Line, NurbsCurve, Polyline};
```

**Replace with:**

```rust
use session_rust::{Line, NurbsCurve, Point, Polyline};
```

**Split the function.** Everything from the CV-box loop through the `n` you just wrote becomes a
sibling that answers one question — *where is this curve* — in the kernel's own f64.

**Find** in `src/app/walk/curves.rs`:

```rust
pub fn nurbscurve_to_segments(c: &NurbsCurve, instance_id: u32) -> Vec<CylinderSegment> {
```

**Replace with:**

```rust
/// Sample a curve into f64 points ONCE. Drawing is only the first reader: the scene BVH's box
/// (67), the ray-vs-segment pick (70) and the draw tool's ghost (86) all want these same points,
/// and none of them may re-derive them. Uniform, by Step 1's policy.
pub fn sample_curve(c: &NurbsCurve) -> Vec<Point> {
```

The tail of the old body splits in two: the evaluation stays and stops converting to f32, and the
segment packing becomes the consumer, which now takes the points instead of making them.

**Find** in `src/app/walk/curves.rs`:

```rust
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

**Replace with:**

```rust
    let (t0, t1) = c.domain();
    (0..=n).map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64)).collect()
}

/// A sampled curve IS a polyline: consecutive pairs -> segments, exactly like
/// `polyline_to_segments`. The points arrive from the cache, so THIS runs on every walk and the
/// sampling above does not - which is the entire economic point of the split.
pub fn nurbscurve_to_segments(c: &NurbsCurve, pts: &[Point], instance_id: u32) -> Vec<CylinderSegment> {
    let color = pack_rgba(c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]));
    let radius = encode_width(c.width);
    pts.windows(2).map(|w| CylinderSegment {
        p0: w[0].to_f32(),
        radius,
        p1: w[1].to_f32(),
        instance_id,
        color,
        facing: FACING_UNKNOWN,
    }).collect()
}
```

**The cache type.** It goes in `src/app/walk/mod.rs`, next to `WalkCx`, because the walk is what
fills it and 59 through 62 are what widen it. Two imports first.

**Find** in `src/app/walk/mod.rs`:

```rust
use session_rust::Geometry;
use session_rust::element::ElementGeometry;
```

**Replace with:**

```rust
use std::collections::HashMap;

use session_rust::{Geometry, Point};
use session_rust::element::ElementGeometry;
```

**Find** in `src/app/walk/mod.rs`:

```rust
use curves::{line_to_segment, nurbscurve_to_segments, polyline_to_segments};
```

**Replace with:**

```rust
use curves::{line_to_segment, nurbscurve_to_segments, polyline_to_segments, sample_curve};
```

`WalkCx` is what a producer is handed, so it is what carries the borrow — plus the guid, which is
the key. A guid changes per OBJECT, and that is the one real consequence of this lesson: `add_file`
has built `WalkCx` once per FILE since 51, and it cannot any more.

**Find** in `src/app/walk/mod.rs`:

```rust
pub struct WalkCx {
```

**Replace with:**

```rust
/// Since 58 it also carries the guid being walked and a borrow of the caches that outlive the
/// walk, which is why `add_file` builds one per OBJECT rather than once per file.
pub struct WalkCx<'a> {
```

**Find** in `src/app/walk/mod.rs`:

```rust
    pub cloud_px: f32,
}
```

**Replace with:**

```rust
    pub cloud_px: f32,
    /// The guid of the object being walked - the cache KEY, and the only field here that changes
    /// inside the loop.
    pub guid: &'a str,
    /// Borrowed, never owned. `Scene` holds the caches so they survive this walk and the next.
    pub caches: &'a mut Caches,
}

/// Everything a walk computed that a later walk must not compute again.
///
/// It is OWNED by `Scene` and only borrowed here, because outliving the walk is its entire
/// purpose: `rebuild` re-walks every document and has to find the work still done. Keyed by guid
/// because a guid names SHAPE and nothing else - since the Xform refactor an object's placement
/// rides its instance row, so moving a curve cannot stale an entry, and only an edit can.
///
/// One field today. 59 adds tessellations, 60 their ink, 62 the trim loops; 118 keys mesh LOD off
/// the same map. Eviction is not free-form: a deleted guid leaves with `delete` (79), and a
/// reshaped one is dropped by reconcile's `changed` bucket (64).
#[derive(Default)]
pub struct Caches {
    /// Per-guid curve samples, f64, in curve order - `sample_curve`'s output verbatim.
    pub curves: HashMap<String, Vec<Point>>,
}
```

The dispatch now hands out a mutable borrow, so it must take one.

**Find** in `src/app/walk/mod.rs`:

```rust
pub fn walk_geometry(t: &mut Upload, cx: &WalkCx, geom: &Geometry, ri: u32) -> Row {
```

**Replace with:**

```rust
pub fn walk_geometry(t: &mut Upload, cx: &mut WalkCx, geom: &Geometry, ri: u32) -> Row {
```

**The arm becomes get-or-sample.** Note what does NOT change: the push, the lane and the row are
still the dispatch's business, and `curves.rs` still never sees a sink.

**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::NurbsCurve(c) => { t.seg.ribbons.extend(nurbscurve_to_segments(c, ri)); Row::none() }
```

**Replace with:**

```rust
        Geometry::NurbsCurve(c) => {
            // Sampled ONCE per guid; every later walk of this curve repacks the same points.
            let pts = cx.caches.curves.entry(cx.guid.to_string()).or_insert_with(|| sample_curve(c));
            t.seg.ribbons.extend(nurbscurve_to_segments(c, pts, ri));
            Row::none()
        }
```

**The owner.** `Scene` gains one field, and the borrow checker is the reason it can work at all:
`add_file` already holds `let t = &mut self.tables;` live across the whole loop while
`self.guid_to_row` and `self.hidden` are used inside it. Those are DISJOINT FIELDS of `self`, which
the compiler splits; a fourth is free. Route the same access through a method on `&mut self` and it
is `E0499` instead — which is why the cache is read out as a local before `t` is taken.

**Find** in `src/app/scene.rs`:

```rust
use super::walk::{WalkCx, walk_geometry};
```

**Replace with:**

```rust
use super::walk::{Caches, WalkCx, walk_geometry};
```

**Find** in `src/app/scene.rs`:

```rust
    pub hidden: HashSet<String>,
```

**Add below it:**

```rust
    /// Work that outlives a walk - see `walk::Caches`. `rebuild` must NOT clear this.
    pub caches: Caches,
```

**Find** in `src/app/scene.rs`:

```rust
        hidden: HashSet::new(),
```

**Add below it:**

```rust
        caches: Caches::default(),
```

`rebuild` deliberately leaves the cache alone — a re-walk reusing the samples is the whole point,
and it is the reason `rebuild` is affordable behind a hide or an edit commit. `clear` is the
opposite case: it drops every document, so every guid in the map is gone and keeping their samples
is a pure leak.

**Find** in `src/app/scene.rs`:

```rust
        self.hidden.clear();
```

**Add below it:**

```rust
        self.caches.curves.clear();
```

**Find** in `src/app/scene.rs`:

```rust
        let t = &mut self.tables;
        let cx = WalkCx { vert_base: vb, cloud_base: cb, cloud_px };
```

**Replace with:**

```rust
        let caches = &mut self.caches;
        let t = &mut self.tables;
```

**Find** in `src/app/scene.rs`:

```rust
            let row = walk_geometry(t, &cx, geom, ri);
```

**Replace with:**

```rust
            // `cx` is built per OBJECT now: `guid` - the cache key - changes with every one.
            // `&mut *caches` is an explicit reborrow; moving `caches` would end the loop at one.
            let mut cx = WalkCx { vert_base: vb, cloud_base: cb, cloud_px, guid: &guid, caches: &mut *caches };
            let row = walk_geometry(t, &mut cx, geom, ri);
```

**What this lesson deliberately does NOT add: CV glyphs.** The archive drew every curve's control
points as 32a spheres, always — and a sheet of curves became a starfield. Real CAD shows control
points in an EDIT mode; that is lesson 100's job (`F10`, on the selected curve only), and the
weighted-CV read it needs is already sitting in `sample_curve`'s box loop.

## Step 3 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib   # the browser target
cargo xtest                                         # architecture.rs + the native examples
```

`cargo xtest` is the one that would catch this lesson overspending: `WalkCx` is now five fields
and `walk_geometry` still four parameters, `Caches` has one field so no naming convention is
hiding a type, and `app/scene.rs` lands at 291 lines of its 300-line budget — nine to spare, which
is why the get-or-sample went into `walk/mod.rs` and not into the loop that calls it.

- No curve fixture ships yet — make one in two minutes, natively: a tiny example that
  `Session::add_nurbscurve`s a few `NurbsCurve::create(false, 3, &points)` cubics and
  `pb_dump`s the session; point a manifest at it. Curves render as 31 tubes with their own
  color/width, unchanged in every respect except the sample count Step 1 raised.
- Zoom close on a long curve → you *will* find chords: the count is fixed at build time, and
  that honesty is the sampling policy (adaptive is the parked upgrade).
- Load the same file TWICE (two manifest items). Guids are per-document, so the second walk
  samples its own curves once and the map simply grows; duplicate the guids and it does not
  sample at all. Either way `rebuild()` — hide something, or call it from the console — re-walks
  every curve WITHOUT resampling one, and that is the case this lesson exists for.

## Recap

```
Ch 57: a naming convention is not a type — the restructure block closed.
Ch 58: NURBSCURVE, made honest. The arm already existed (gap #4 fixed kernel-side: curves are
       Geometry variants in lookup; add_nurbscurve registers) and 51 already gave it a producer
       file. What changed: the sampler's absolute-0.2 size heuristic gains a span_count floor
       (complexity gets a vote; the 0.2 stays kernel-unit-absolute — same trap family as mesh_q's
       0.005, called out, not hidden); sample_curve(c) -> Vec<Point> split out of
       nurbscurve_to_segments, which now just packs segments from &[Point] + linecolors/width;
       walk/mod.rs grows Caches, Scene OWNS it, and WalkCx carries the guid plus a borrow — so
       WalkCx is built per object, not per file. Samples are computed once per guid, SURVIVE
       rebuild, are dropped by clear, and get reread by the box (67), the pick (70) and the ghost
       (86); evicted on delete (79) and reshape (64). CV spheres deliberately NOT drawn — edit
       mode's job (100). The CV-box loop handles rational curves ([x·w,y·w,z·w,w] ÷ w) — keep it.
```

Edited: `src/app/walk/curves.rs` (`sample_curve` + the span floor + the slimmed consumer),
`src/app/walk/mod.rs` (`Caches`, `WalkCx<'a>`, the get-or-sample arm) and `src/app/scene.rs`
(`Scene.caches` and the per-object `WalkCx`).

## Next

Lesson [59](59-nurbssurface.md) — surfaces already draw too (`s.mesh()` straight into the mesh
producer) — and that call is the problem: every walk re-tessellates every surface. The fix is the
phase's central rule and the second field on the map you just built: tessellate **once, cache
it** — transforms never touch shape, so the cache can never go stale from moving things.
