# 60 NurbsCurve — evaluate, draw, create

> **Big picture.** *Phase 10 — curved geometry (60–64).* Everything drawn so far is straight or
> faceted; the kernel's NURBS types bring true curves and surfaces. One structural fact shaped this
> phase when it was first written: nurbs objects lived only in their own collections
> (`session.objects.nurbscurves`, …), outside `session.lookup` — and every map the viewer keeps had
> to remember both sources; the archive forgot repeatedly, each forget a bug. That audit became
> kernel-gap #4, and **the kernel has since been fixed**: `NurbsCurve` and `NurbsSurface` are now
> `Geometry` variants, registered in `lookup` on add and load, in all three languages. So curves ride
> the *existing* lookup walk with two new `match` arms — only `NurbsSurfaceTrimmed` (64) still lives
> collection-only. The discipline this phase teaches — **every map, checked** — still stands; the
> kernel just does most of the remembering now.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a nurbs curve is sampled at parameters into a polyline whose segments feed the cylinder path; the sample count adapts to the span count" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <path d="M 30,100 C 110,20 190,120 270,50" fill="none" stroke="#6fb3ff" stroke-width="2"/>
  <g fill="#d7dae0"><circle cx="30" cy="100" r="3"/><circle cx="98" cy="63" r="3"/><circle cx="152" cy="78" r="3"/><circle cx="208" cy="76" r="3"/><circle cx="270" cy="50" r="3"/></g>
  <text x="150" y="122" fill="#888" text-anchor="middle">point_at(t) samples → polyline → 31's tubes</text>
  <g transform="translate(360,20)">
    <text x="0" y="16" fill="#d7dae0">samples = spans × 16 (clamped 32..512)</text>
    <text x="0" y="36" fill="#666" font-size="10">more spans = more wiggle = more samples;</text>
    <text x="0" y="50" fill="#666" font-size="10">a straight-ish curve stays cheap</text>
    <text x="0" y="78" fill="#888">control points → 32a sphere glyphs</text>
    <text x="0" y="96" fill="#666" font-size="10">(the handles 73's editing will grab)</text>
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

/// Sample a curve into world points for drawing/picking. Adaptive: 16 per span, clamped.
fn sample_curve(nc: &NurbsCurve) -> Vec<Point> {
    let n = (nc.span_count() * 16).clamp(32, 512);
    let (t0, t1) = nc.domain();
    (0..=n).map(|i| nc.point_at(t0 + (t1 - t0) * i as f64 / n as f64)).collect()
}
```

## Step 2 — every map gains an arm: `src/app/scene.rs`

Curves arrive in `lookup` as `Geometry::NurbsCurve` (the kernel fix from gap #4), so the maps built
on the lookup walk — order, build, boxes — each gain a **match arm**, not a parallel loop. Only
picking needs curve-specific code:

```rust
    // (1) ORDER — is_renderable (35) admits the new variant; Scene::new needs nothing else:
    fn is_renderable(g: &Geometry) -> bool {
        matches!(g, Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::Line(_)
                  | Geometry::Polyline(_) | Geometry::Point(_) | Geometry::NurbsCurve(_))   // ← ADD
    }

    // (2) BUILD — one arm in Scene::build's existing match (samples → segments, like a polyline):
    Geometry::NurbsCurve(nc) => {
        // world coords, like lines
        objects_base.push((nc.xform.duplicate(), curve_color(nc), flags));
        let pts = sample_curve(nc);
        for w in pts.windows(2) { segments.push(seg(w[0].to_f32(), w[1].to_f32(), ri)); }
        // control points as handles
        for i in 0..nc.cv_count() {
            if let Some(p) = nc.get_cv(i) { glyphs.push(glyph(p.to_f32(), ri)); }
        }
    }

    // (3) WORLD BOX — one arm in world_obb's match (36); the kernel has the exact ctor:
    Geometry::NurbsCurve(nc) => OBB::from_nurbscurve(nc, PAD, true),

    // (4) PICK — Session::ray_cast's curve arm is a deliberate no-op (exact ray↔NURBS is out of
    //     scope kernel-side), so pick_thin tests the CACHED samples with 44's screen-radius rule:
    fn as_nurbscurve(g: &Geometry) -> Option<&NurbsCurve> {
        if let Geometry::NurbsCurve(nc) = g { Some(nc) } else { None }
    }
    for nc in self.session.lookup.values().filter_map(as_nurbscurve) {
        let pts = /* sample cache entry */;
        for w in pts.windows(2) { /* ray↔segment distance ≤ tol → candidate (44's formula) */ }
    }
```

(Exact field/method names to check against your kernel as you wire: the CV accessor — `cv_point` or
the 4-d `get_cv` family — and the curve's color field. `sample_curve` results should be **cached** in
a `HashMap<String, Vec<Point>>` invalidated on reconcile — sampling per pick is wasteful, per frame
would be a bug.)

Hide/selection need nothing: they key off `guid_to_row` and the instance flags, which arms (1)/(2)
already feed. That's the reward for flag-driven state (45/46) — new geometry types inherit it.

## Step 3 — the tool: `src/app/tools/curve.rs`

58's polyline tool with a different finish — clicks are **control points** (the curve smooths them,
it doesn't pass through), Enter builds a degree-3 curve:

```rust
    // finish (Enter), with self.points: Vec<Point> accumulated exactly like PolylineTool:
    if self.points.len() < 4 { return CmdStep::Cancel; }              // degree 3 needs ≥ 4 CVs
    // open, cubic, from control points
    let nc = NurbsCurve::create(false, 3, &self.points);
    state.commit(Box::new(AddGeometry::one(Geometry::NurbsCurve(nc)))); // 57's command, directly
    CmdStep::Done("curve added".into());
```

The ghost: sample a *temporary* `NurbsCurve::create` from the clicked points + cursor on every
`on_move` — the preview shows the real smoothed curve, not the control polygon. (Cheap: a handful of
CVs, ~64 samples.)

Since curves are `Geometry` variants now, no bespoke command is needed at all — 57's `AddGeometry`
takes them directly, and 51's `restore_geometry` grows one arm:

```rust
    // 51's restore_geometry, one new arm — the kernel's add_nurbscurve handles
    // objects/lookup/graph/tree:
    Geometry::NurbsCurve(c) => { self.session.add_nurbscurve(c, None); }

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
- Zoom close → still smooth (the 16-per-span sampling holds up); a curve drawn from 5 points costs
  ~32 segments, not 512 (adaptive — a 5-CV cubic has 2 spans, `(2*16).clamp(32,512)` = the 32 floor).
- The **every-map audit** — do all four, deliberately: it *draws* (map 2), it *picks* with a click on
  the curve body (map 4), **F** includes it in the fit and marquee catches it (map 3), `hide` hides
  it (map 1's row + flags). Any one failing means an arm was skipped — the phase's core bug class,
  caught in a minute.
- Ctrl+Z removes the whole curve; redo restores it with the same guid.

## Recap

```
Ch 59: snapping — Phase 9 closed.
Ch 60: NURBSCURVE. Curves are Geometry variants (kernel-gap #4, FIXED while writing this course) —
       registered in lookup by add_nurbscurve and on load — so every lookup-walking map gains a
       match ARM, not a parallel loop: is_renderable admits it, build samples it, world_obb boxes
       it (OBB::from_nurbscurve, kernel-exact). Draw = sample point_at over the domain, spans×16
       clamped 32..512, → 31's segments; CVs → 32a glyphs (73's future handles); cache samples per
       guid. Pick = ray↔segment over the cached samples (the kernel's curve ray arm is a deliberate
       no-op). Tool = polyline-shaped, clicks are CONTROL points, ghost samples a temporary curve,
       Enter → NurbsCurve::create(false, 3, points), ≥4 CVs — committed via 57's AddGeometry
       directly; 51's restore_geometry grows one arm. Only NurbsSurfaceTrimmed (64) still lives
       collection-only.
```

Edited: `app/scene.rs` (match arms in is_renderable/build/world_obb + sampled pick + sample cache),
`app/tools/curve.rs` (NEW), `app/scene.rs`'s `restore_geometry` (one arm), `app/commands.rs` (`curve`).

## Next

`61-nurbssurface.md` — surfaces: the kernel tessellates a `NurbsSurface` to a mesh with baked vertex
normals, so smooth shading arrives free through 22's data-driven path. The rule that matters:
tessellate **once, cache it** — and transforms stay matrix-only (re-tessellating on every gumball
commit was the archive's measured perf bug).
