# 58 NurbsCurve — what already draws, made honest and reusable

> **Big picture.** *Phase 4b — curved geometry (43–47), pulled AHEAD of the interaction phases:
> every kernel type is rendered CORRECTLY and CHEAPLY before any tool touches one.* One structural
> fact shaped this phase when it was first written: nurbs objects lived only in their own
> collections (`session.objects.nurbscurves`, …), outside `session.lookup` — and every map the
> viewer keeps had to remember both sources; the archive forgot repeatedly, each forget a bug.
> That audit became kernel-gap #4, and **the kernel has since been fixed**: `NurbsCurve` and
> `NurbsSurface` are `Geometry` variants, registered in `lookup` on add and load, in all three
> languages — which is why your walk ALREADY draws curves: the arm rode in with 34/35. Only
> `NurbsSurfaceTrimmed` (47) still lives collection-only. This lesson is therefore not "add the
> type" — it is: read what the arm really does, fix its one trap, and factor its samples into a
> CACHE that the box map (52), the pick (57) and the draw tool (71) will all read.

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

```
src/app/scene.rs   # sample_curve factored out of nurbscurve_to_segments; span floor;
                   # Scene.curve_cache — samples computed once, reread by 52/57/71
```

## Step 0 — read the arm you already have

In `add_file`'s walk match:

```rust
                Geometry::NurbsCurve(c) => { t.segments.extend(nurbscurve_to_segments(c, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
```

and `nurbscurve_to_segments` below it does three things worth understanding before changing any:

- **It bounds the curve by its CONTROL NET** — a NURBS curve never leaves its control points'
  convex hull, so the CV box is a free, exact-enough size estimate. The loop handles **rational**
  curves: a weighted CV is stored `[x·w, y·w, z·w, w]`, so it divides by `w` (with a `w == 0`
  guard) to get the real point. Keep that loop — 52's box arm and 85's CV handles reread it.
- **It scales the sample count by SIZE**: `n = ((size / 0.2).sqrt()).clamp(4, 64)` — a 2 mm glyph
  outline gets 4 segments, a metre-long arc ~50.
- **It then IS a polyline**: `point_at(t)` at `n+1` even parameters, consecutive pairs →
  `CylinderSegment`s with the curve's own `linecolors`/`width` — 31's tube lane, nothing special.

## Step 1 — the trap, and the span floor

Two things are wrong with `n = ((size / 0.2).sqrt()).clamp(4, 64)`, and they're the same two
traps as `mesh_q`'s `0.005` (47 meets it): the `0.2` is an **absolute length in kernel units**
(a metre-unit file samples ~14× sparser than the same curve authored in millimetres), and size
says nothing about **complexity** — a 40-knot S-curve the size of a coin gets the same 4 segments
as a straight stub. Curvature-exact sampling is overkill for a display polyline; the honest cheap
signal for complexity is the **span count** (each span can bend independently).

**Find** (in `nurbscurve_to_segments`):

```rust
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);
```

**Replace with:**

```rust
    let size = ((hi[0]-lo[0]).powi(2) + (hi[1]-lo[1]).powi(2) + (hi[2]-lo[2]).powi(2)).sqrt();
    // Size-scaled (0.2 is kernel units - mm here; a unit-relative tolerance is the upgrade),
    // with a SPAN floor: each span can bend independently, so complexity has a vote too.
    let n = ((size / 0.2).sqrt().ceil() as usize)
        .clamp(4, 64)
        .max((c.span_count() * 8).min(512));
```

Honest label: this is still *uniform* sampling — curvature is never measured, and zooming close
enough will always find chords (Step 3 makes you look at them). When that bites, the drop-in is
chord-error refinement — subdivide until each segment's midpoint stays within `tol` of its chord:

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

(Park it `#[allow(dead_code)]` or skip typing it — it is the upgrade path, not today's policy.)

## Step 2 — factor the samples out, and cache them

Today the samples are born as `[f32; 3]`, converted inline, and die inside the segment build.
Three future consumers want the SAME points in f64: 52's box, 57's ray↔segment pick, 71's ghost.
Sampling per pick would be wasteful, per frame a bug — so the samples become a per-guid cache,
computed once per curve, surviving rebuilds.

**Split the function.** In `nurbscurve_to_segments`, everything from the CV-box loop through the
`pts` build moves into a sibling:

```rust
/// Sample a curve into f64 points once; drawing, the box (52), the pick (57) and the
/// ghost (71) all reread this. Uniform per Step 1's policy.
fn sample_curve(c: &NurbsCurve) -> Vec<Point> {
    // ... the CV box loop, the empty-curve early-out (return Vec::new()),
    //     Step 1's n, then:
    let (t0, t1) = c.domain();
    (0..=n).map(|i| c.point_at(t0 + (t1 - t0) * i as f64 / n as f64)).collect()
}
```

and `nurbscurve_to_segments` shrinks to the consumer — it now takes the points:

```rust
fn nurbscurve_to_segments(c: &NurbsCurve, pts: &[Point], instance_id: u32) -> Vec<CylinderSegment> {
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

**The cache.** One field on `struct Scene`, below `hidden`:

```rust
    pub curve_cache: HashMap<String, Vec<Point>>, // per-guid samples; 52/57/71 reread them
```

(`curve_cache: HashMap::new(),` in `Scene::new`; `rebuild` deliberately does NOT clear it — a
re-walk reuses the samples, that is the point. Eviction: a *deleted* curve's entry must leave
with it, which is `delete`'s lesson (64); a *reshaped* curve's entry is invalidated by
reconcile's `changed` bucket (49).) The walk arm becomes get-or-sample — and note it can touch
`self.curve_cache` while `t = &mut self.tables` is live, because those are DISJOINT fields of
`self`; the borrow checker splits field borrows. **Replace the arm with:**

```rust
                Geometry::NurbsCurve(c) => {
                    let pts = self.curve_cache.entry(guid.clone())
                        .or_insert_with(|| sample_curve(c));
                    t.segments.extend(nurbscurve_to_segments(c, pts, ri));
                    t.object_bounds.push(None); t.object_spacing.push(0.0);
                }
```

**What this lesson deliberately does NOT add: CV glyphs.** The archive drew every curve's control
points as 32a spheres, always — and a sheet of curves became a starfield. Real CAD shows control
points in an EDIT mode; that is lesson 100's job (`F10`, on the selected curve only), and the
weighted-CV read it needs is already sitting in `sample_curve`'s box loop.

## Step 3 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib     # and the native examples still build
```

- No curve fixture ships yet — make one in two minutes, natively: a tiny example that
  `Session::add_nurbscurve`s a few `NurbsCurve::create(false, 3, &points)` cubics and
  `pb_dump`s the session; point a manifest at it. Curves render as 31 tubes with their own
  color/width, exactly as before the refactor — **byte-identical for single-span curves**
  (the span floor only densifies multi-span ones).
- Zoom close on a long curve → you *will* find chords: the count is fixed at build time, and
  that honesty is the sampling policy (adaptive is the parked upgrade).
- Load the same file TWICE (two manifest items): the second walk logs no sampling cost — the
  cache is per-guid and both docs' curves share guids only if you duplicated the file; edit the
  example to give distinct guids and the cache grows instead. Either way `rebuild()` (hide
  something via the console later, or just trust the code path) re-walks WITHOUT resampling.

## Recap

```
Ch 42: streaming — the cloud arc closed.
Ch 43: NURBSCURVE, made honest. The arm already existed (gap #4 fixed kernel-side: curves are
       Geometry variants in lookup; add_nurbscurve registers). What changed: the sampler's
       absolute-0.2 size heuristic gains a span_count floor (complexity gets a vote; the 0.2
       stays kernel-unit-absolute — same trap family as mesh_q's 0.005, called out, not hidden);
       sample_curve(c) -> Vec<Point> factored out of nurbscurve_to_segments (which now just
       packs segments from &[Point] + linecolors/width); samples land in Scene.curve_cache by
       guid — computed once, SURVIVES rebuild, reread by the box (52), the pick (57), the ghost
       (71); evicted on delete (64) and reshape (49). CV spheres deliberately NOT drawn — edit
       mode's job (85). The CV-box loop handles rational curves ([x·w,y·w,z·w,w] ÷ w) — keep it.
```

Edited: `app/scene.rs` (`sample_curve` + span floor + `curve_cache` + the slimmed arm).

## Next

`59-nurbssurface.md` — surfaces already draw too (`s.mesh()` straight into `push_mesh`) — and
that call is the problem: every walk re-tessellates every surface. The fix is the phase's
central rule: tessellate **once, cache it** — transforms never touch shape, so the cache can
never go stale from moving things.
