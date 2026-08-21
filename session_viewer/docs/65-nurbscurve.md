# 65 NurbsCurve — evaluate, draw, create

> **Big picture.** *Phase 10 — curved geometry (65–69).* Everything drawn so far is straight or
> faceted; the kernel's NURBS types bring true curves and surfaces. One structural fact shaped this
> phase when it was first written: nurbs objects lived only in their own collections
> (`session.objects.nurbscurves`, …), outside `session.lookup` — and every map the viewer keeps had
> to remember both sources; the archive forgot repeatedly, each forget a bug. That audit became
> kernel-gap #4, and **the kernel has since been fixed**: `NurbsCurve` and `NurbsSurface` are now
> `Geometry` variants, registered in `lookup` on add and load, in all three languages. So curves ride
> the *existing* lookup walk with two new `match` arms — only `NurbsSurfaceTrimmed` (69) still lives
> collection-only. The discipline this phase teaches — **every map, checked** — still stands; the
> kernel just does most of the remembering now.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a nurbs curve is sampled at parameters into a polyline whose segments feed the cylinder path; the sample count adapts to the span count" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <path d="M 30,100 C 110,20 190,120 270,50" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <g fill="#d7dae0"><circle cx="30" cy="100" r="3"/><circle cx="98" cy="63" r="3"/><circle cx="152" cy="78" r="3"/><circle cx="208" cy="76" r="3"/><circle cx="270" cy="50" r="3"/></g>
  <text x="150" y="122" fill="#888" text-anchor="middle">point_at(t) samples → polyline → 31's tubes</text>
  <g transform="translate(360,20)">
    <text x="0" y="16" fill="#d7dae0">samples = spans × 16 (clamped 32..512)</text>
    <text x="0" y="36" fill="#666" font-size="10">uniform per span — budget follows span</text>
    <text x="0" y="50" fill="#666" font-size="10">count, NOT measured curvature</text>
    <text x="0" y="78" fill="#888">control points → 32a sphere glyphs</text>
    <text x="0" y="96" fill="#666" font-size="10">(the handles 78's editing will grab)</text>
  </g>
</svg>

## Files we touch

```
src/app/scene.rs        # curves join EVERY map: order, build (sample→segments), world_obb, pick
src/app/tools/curve.rs  # NEW — NurbsCurveTool: N control clicks + Enter
src/app/commands.rs     # `curve` verb
```

## Step 1 — sampling: `src/app/scene.rs`

A NURBS curve is exact; the screen wants segments. Sample `point_at(t)` across the domain, density
scaled by `span_count()` (each span can bend independently — a curve with more knots needs more
samples; one with few stays cheap):

```rust
use session_rust::NurbsCurve;

/// Sample a curve into world points for drawing/picking. Uniform per span: 16 per span, clamped —
/// the budget follows the SPAN COUNT, not the measured curvature (true adaptivity is below).
fn sample_curve(nc: &NurbsCurve) -> Vec<Point> {
    let n = (nc.span_count() * 16).clamp(32, 512);
    let (t0, t1) = nc.domain();
    (0..=n).map(|i| nc.point_at(t0 + (t1 - t0) * i as f64 / n as f64)).collect()
}
```

Honest label: this is *uniform* sampling with a span-scaled budget — curvature is never measured,
so a tight single-span S-curve gets the same 16 samples as a straight span, and zooming close
enough will always find the chords (see Step 4). When that bites, the drop-in upgrade is
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

## Step 2 — every map gains an arm: `src/app/scene.rs`

Curves arrive in `lookup` as `Geometry::NurbsCurve` (the kernel fix from gap #4), so the maps built
on the lookup walk — order, build, boxes — each gain a **match arm**, not a parallel loop. Only
picking needs curve-specific code:

**(1) ORDER** — `is_renderable` (35/43b) admits the new variant; `Scene::new` needs nothing else:

```rust
    fn is_renderable(g: &Geometry) -> bool {
        matches!(g, Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::Line(_)
                  | Geometry::Polyline(_) | Geometry::Point(_) | Geometry::NurbsCurve(_))   // ← ADD
    }
```

**(2) BUILD** — one arm in `Scene::build`'s existing match (samples → segments, like a polyline;
CVs → glyphs). The samples also land in a cache the pick arm reads — add
`pub curve_cache: std::collections::HashMap<String, Vec<Point>>` to `struct Scene` (init empty in
`Scene::new`; reconcile's `changed` bucket removes the entry), and note the cache write means
`build` takes `&mut self` from here on. Two small helpers first, beside `sample_curve`:

```rust
/// A curve's draw color: the first linecolor if the user set one, else near-black.
fn curve_color(nc: &NurbsCurve) -> [f32; 4] {
    nc.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.10, 0.10, 0.10, 1.0])
}
```

```rust
    // the new arm in Scene::build's match, beside Polyline's:
    Geometry::NurbsCurve(nc) => {
        // world coords, like lines
        objects_base.push((nc.xform.duplicate(), curve_color(nc), flags));
        let pts = sample_curve(nc);
        let color = curve_color(nc);
        for w in pts.windows(2) {
            segments.push(CylinderSegment { p0: w[0].to_f32(), radius: 0.0, p1: w[1].to_f32(),
                                            instance_id: ri, color });
        }
        // control points as handles (78's future grab targets)
        for i in 0..nc.cv_count() {
            if let Some(p) = nc.get_cv(i) {
                glyphs.push(GlyphPoint { center: p.to_f32(), radius: 0.0,
                    color: [0.2, 0.2, 0.2, 1.0], instance_id: ri, _pad: [0; 3] });
            }
        }
        curve_samples.push((nc.guid().to_string(), pts));   // drained into curve_cache below
    }
```

(Declare `let mut curve_samples: Vec<(String, Vec<Point>)> = Vec::new();` beside `build`'s other
accumulators, and after the walk loop ends, drain it:
`for (g, pts) in curve_samples { self.curve_cache.insert(g, pts); }` — the indirection keeps the
loop free of a `&mut self` borrow.)

**(3) WORLD BOX** — one arm in `world_obb`'s match (40); the kernel has the exact ctor:

```rust
        Geometry::NurbsCurve(nc) => OBB::from_nurbscurve(nc, PAD, true),
```

**(4) PICK** — `Session::ray_cast`'s curve arm is a deliberate no-op (exact ray↔NURBS is out of
scope kernel-side), so `pick_thin` (49) tests the **cached samples** with 49's rule. In `pick_thin`,
find its final `None` (after the kernel-cast loop) → replace with (it runs only when the kernel
cast matched nothing):

```rust
        // curves: ray↔segment over the cached samples (49's tolerance, same tol)
        let mut best: Option<PickHit> = None;
        for doc in &self.docs {                          // multi-doc (43b): EVERY doc's lookup —
            for (guid, geom) in &doc.session.lookup {    // the singular `session` walk was stale
                let Geometry::NurbsCurve(_) = geom else { continue };
                let Some(pts) = self.curve_cache.get(guid) else { continue };
                for w in pts.windows(2) {
                    let line = Line::from_points(&w[0], &w[1]);
                    if let Some(hit) = session_rust::intersection::line_line(
                        // 1.0e7 = 46's far-point cap — beyond it this pick ray ends early
                        &Line::from_points(&ray.origin, &(ray.origin.clone() + &ray.dir * 1.0e7)),
                        &line, tol) {
                        let t = (hit.clone() - ray.origin.clone()).magnitude();
                        if best.as_ref().map_or(true, |b| t < b.t) {
                            best = Some(PickHit { guid: guid.clone(), point: hit, t });
                        }
                    }
                }
            }
        }
        best
```

Two honest caveats on that loop. **Cost:** it's O(curves × samples) per click — fine at this
course's scene sizes; when it isn't, the broad-phase is 40's BVH (gather candidate guids with
`objects_in` around the ray's box, then test only those cache entries — the same pattern 64's
snap uses). **Placement:** the cached samples are object-local — `build` draws them through the
instance row's xform. A curve with a non-identity placed frame must transform the samples (or the
ray) first, exactly like 47's mesh arm does; at the identity placements this phase uses, the code
above is correct as written.

`sample_curve` runs once per curve per (re)build — sampling per pick would be wasteful, per frame a
bug; the cache is the difference.

And the cache has a **lifecycle**: entries are keyed by guid, so deleting a curve (`delete`, undo
of an add, 56's `RemoveObjects`) leaves its samples behind — a slow leak, and a stale-pick hazard
the day the pick loop trusts the cache without the lookup gate. Evict on the way out: 56's remove
path gains a `scene.curve_cache.remove(&guid)` per removed object (cheap no-op for non-curves), and
`Scene::build` — being a full rebuild — may simply `self.curve_cache.clear()` before draining
`curve_samples`, which makes every rebuild self-healing.

Hide/selection need nothing: they key off `guid_to_row` and the instance flags, which arms (1)/(2)
already feed. That's the reward for flag-driven state (50/51) — new geometry types inherit it.

## Step 3 — the tool: `src/app/tools/curve.rs`

63's polyline tool with a different finish — clicks are **control points** (the curve smooths them,
it doesn't pass through), Enter builds a degree-3 curve:

Copy `polyline.rs` to `curve.rs` (add `pub mod curve;` to `app/tools/mod.rs`), rename
`PolylineTool` → `NurbsCurveTool`, reword the prompts (`"curve: pick control point (Enter
finishes)"`), and make exactly two body changes. The Enter branch of `feed_text` — find
`if self.points.len() < 2 { return CmdStep::Cancel; }` and the two `Polyline` lines under it →
replace with:

```rust
            if self.points.len() < 4 { return CmdStep::Cancel; }      // degree 3 needs ≥ 4 CVs
            // open, cubic, from control points
            let nc = NurbsCurve::create(false, 3, &self.points);
            state.commit(Box::new(
                crate::app::history::add::AddGeometry::one(Geometry::NurbsCurve(nc))));
            return CmdStep::Done("curve added".into());
```

(and swap the file's `Polyline` import for `NurbsCurve`). Then the ghost — the preview shows the
real smoothed curve, not the control polygon (cheap: a handful of CVs, ~64 samples). Replace the
copied `ghost` helper's body with:

```rust
    fn ghost(&mut self, state: &mut crate::state::State, cursor: Option<&Point>) {
        let mut cvs: Vec<Point> = self.points.iter().cloned().collect();   // handful of CVs — OK
        if let Some(c) = cursor { cvs.push(c.clone()); }
        if cvs.len() < 2 { state.gpu.clear_preview(); return; }
        let deg = 3.min(cvs.len() - 1);                    // a cubic once there are enough CVs
        let tmp = NurbsCurve::create(false, deg, &cvs);    // tiny kernel object, built per move
        let (t0, t1) = tmp.domain();
        self.scratch.clear();                              // 63's reused staging — no per-move alloc
        let mut prev: Option<Point> = None;
        for i in 0..=64 {
            let p = tmp.point_at(t0 + (t1 - t0) * i as f64 / 64.0);
            if let Some(q) = &prev { self.scratch.push(super::polyline::ghost_segment(q, &p)); }
            prev = Some(p);
        }
        state.gpu.set_preview(&self.scratch);
    }
```

(The per-move `NurbsCurve::create` is deliberate: the CV count is single digits, so the temp curve
is microseconds — the *allocations* were the real churn, and those now ride the copied file's
`scratch` Vec from 63. If a profiler ever disagrees, cache `tmp` keyed on `(points.len(), cursor)`
— but measure first.)

Since curves are `Geometry` variants now, no bespoke command is needed at all — 62's `AddGeometry`
takes them directly, and 56's `restore_geometry` already grew the arm (`session` there is
`self.docs[d].session` — multi-doc since 43b, never a singular `self.session`):

```rust
    // already in 56's restore_geometry — the kernel's add_nurbscurve handles
    // objects/lookup/graph/tree:
    Geometry::NurbsCurve(c) => { session.add_nurbscurve((**c).clone(), None); }

    // the tool's finish (Step 3 above) is therefore just:
    state.commit(Box::new(AddGeometry::one(Geometry::NurbsCurve(nc))));
```

(`Session::remove_object` — which `AddGeometry`'s undo runs through — already retains the nurbs
collections since the gap-#4 fix, so the round trip is complete with zero curve-specific plumbing.)

Register `"curve"` (+ aliases `"crv"`, `"nurbscurve"`) in the verb tables.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **`curve`**, click 5 points, Enter → a smooth cubic threads *near* your clicks (control points, not
  interpolation — the ghost already showed exactly this while you clicked). Its control points wear
  32a spheres.
- Zoom close → eventually you *will* see chords: the sample count is fixed at build time and
  uniform per span, so nothing re-densifies as you zoom. That honesty is the lesson's sampling
  policy — if it bothers you, Step 1's `sample_curve_adaptive` is the drop-in (and 81 revisits
  zoom-dependent tessellation as a perf lever). What the span-scaled budget *does* buy: a curve
  drawn from 5 points costs ~32 segments, not 512 (a 5-CV cubic has 2 spans,
  `(2*16).clamp(32,512)` = the 32 floor).
- The **every-map audit** — do all four, deliberately: it *draws* (map 2), it *picks* with a click on
  the curve body (map 4), **F** includes it in the fit and marquee catches it (map 3), `hide` hides
  it (map 1's row + flags). Any one failing means an arm was skipped — the phase's core bug class,
  caught in a minute.
- Ctrl+Z removes the whole curve; redo restores it with the same guid.

## Recap

```
Ch 64: snapping — Phase 9 closed.
Ch 65: NURBSCURVE. Curves are Geometry variants (kernel-gap #4, FIXED while writing this course) —
       registered in lookup by add_nurbscurve and on load — so every lookup-walking map gains a
       match ARM, not a parallel loop: is_renderable admits it, build samples it, world_obb boxes
       it (OBB::from_nurbscurve, kernel-exact). Draw = sample point_at over the domain, spans×16
       clamped 32..512 — UNIFORM per span, not curvature-adaptive (chord-error refinement is the
       drop-in upgrade; zooming always finds chords eventually) → 31's segments; CVs → 32a glyphs
       (78's future handles); cache samples per guid, and EVICT the cache on remove/rebuild (56's
       remove path; build clears). Pick = ray↔segment over the cached samples, walking EVERY doc's
       lookup (43b — the singular `session` walk was stale), 1.0e7 far-point cap per 46 (the
       kernel's curve ray arm is a deliberate no-op). Tool = polyline-shaped, clicks are CONTROL
       points, ghost samples a temporary curve into 63's reused scratch, Enter →
       NurbsCurve::create(false, 3, points), ≥4 CVs — committed via 62's AddGeometry
       directly; 56's restore_geometry grows one arm. Only NurbsSurfaceTrimmed (69) still lives
       collection-only.
```

Edited: `app/scene.rs` (match arms in is_renderable/build/world_obb + sampled pick + sample cache),
`app/tools/curve.rs` (NEW), `app/scene.rs`'s `restore_geometry` (one arm), `app/commands.rs` (`curve`).

## Next

`66-nurbssurface.md` — surfaces: the kernel tessellates a `NurbsSurface` to a mesh with baked vertex
normals, so smooth shading arrives free through 22's data-driven path. The rule that matters:
tessellate **once, cache it** — and transforms stay matrix-only (re-tessellating on every gumball
commit was the archive's measured perf bug).
