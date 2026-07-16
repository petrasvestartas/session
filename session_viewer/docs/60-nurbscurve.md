# 60 NurbsCurve — evaluate, draw, create

> **Big picture.** *Phase 10 — curved geometry (60–64).* Everything drawn so far is straight or
> faceted; the kernel's NURBS types bring true curves and surfaces. One structural fact shapes the
> whole phase: **nurbs objects don't live in `session.lookup`** — they sit in their own collections
> (`session.objects.nurbscurves`, `.nurbssurfaces`, `.nurbssurfacetrimmeds`). Every map the viewer
> keeps (draw order, world boxes, picking, visibility) must include them *explicitly* — the archive
> forgot, repeatedly, and each forget was a bug ("surface draws but won't pick", "trimmed surface
> vanishes from the tree"). This phase's discipline: **one collection, every map.**

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

## Step 2 — one collection, every map: `src/app/scene.rs`

Four small arms, one per map. This is the lesson's discipline made concrete — write all four **now**,
not when each bug surfaces:

```rust
    // (1) ORDER — Scene::new, after the lookup loop:
    for nc in &session.objects.nurbscurves {
        guid_to_row.insert(nc.guid().to_string(), order.len() as u32);
        order.push(nc.guid().to_string());
    }

    // (2) BUILD — Scene::build gains a pass over the collection (samples → segments, like a polyline):
    for nc in &self.session.objects.nurbscurves {
        let ri = self.guid_to_row[nc.guid()];
        let flags = if self.hidden.contains(nc.guid()) { Instance::FLAG_HIDDEN } else { 0 };
        objects_base_entry(ri, Xform::identity(), nc.linecolors_default(), flags);   // world coords, like lines
        let pts = sample_curve(nc);
        for w in pts.windows(2) { segments.push(seg(w[0].to_f32(), w[1].to_f32(), ri)); }
        for i in 0..nc.cv_count() {                                        // control points as handles
            if let Some(p) = nc.cv_point(i) { glyphs.push(glyph(p.to_f32(), ri)); }
        }
    }

    // (3) WORLD BOX — a new arm where world_obb's match lives (36); the kernel has the exact ctor:
    //     curves aren't Geometry variants, so key this on the guid set, not the enum:
    if let Some(nc) = self.session.objects.nurbscurves.iter().find(|c| c.guid() == guid) {
        return OBB::from_nurbscurve(nc, PAD, true);
    }

    // (4) PICK — pick_thin (44) can't see curves (Session::ray_cast walks lookup only). Test the
    //     cached samples with the same screen-radius rule:
    for nc in &self.session.objects.nurbscurves {
        let pts = sample_curve(nc);                    // cache per guid in practice — see the note
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
    let nc = NurbsCurve::create(false, 3, &self.points);              // open, cubic, from control points
    state.commit_nurbscurve(nc);                                      // see below
    CmdStep::Done("curve added".into());
```

The ghost: sample a *temporary* `NurbsCurve::create` from the clicked points + cursor on every
`on_move` — the preview shows the real smoothed curve, not the control polygon. (Cheap: a handful of
CVs, ~64 samples.)

One honest wrinkle: `AddGeometry` (57) wraps `Geometry`, and curves aren't a `Geometry` variant. Give
them their own small Command — same absolute-snapshot pattern, the collection as the target:

```rust
pub struct AddNurbsCurve { snapshot: NurbsCurve }
impl Command for AddNurbsCurve {
    fn apply(&mut self, scene: &mut Scene, gpu: &mut Gpu) {
        scene.session.objects.nurbscurves.push(self.snapshot.clone());
        scene.register_curve(&self.snapshot, gpu);                    // arms 1–2 for ONE curve
    }
    fn revert(&mut self, scene: &mut Scene, gpu: &mut Gpu) {
        let guid = self.snapshot.guid().to_string();
        scene.session.objects.nurbscurves.retain(|c| c.guid() != guid);
        scene.unregister(&guid, gpu);                                 // gpu.remove_object + free_row
    }
    fn label(&self) -> String { "add curve".into() }
}
```

Register `"curve"` (+ aliases `"crv"`, `"nurbscurve"`) in the verb tables.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **`curve`**, click 5 points, Enter → a smooth cubic threads *near* your clicks (control points, not
  interpolation — the ghost already showed exactly this while you clicked). Its control points wear
  32a spheres.
- Zoom close → still smooth (the 16-per-span sampling holds up); a curve drawn from 5 points costs
  ~64 segments, not 512 (adaptive).
- The **every-map audit** — do all four, deliberately: it *draws* (map 2), it *picks* with a click on
  the curve body (map 4), **F** includes it in the fit and marquee catches it (map 3), `hide` hides
  it (map 1's row + flags). Any one failing means an arm was skipped — the phase's core bug class,
  caught in a minute.
- Ctrl+Z removes the whole curve; redo restores it with the same guid.

## Recap

```
Ch 59: snapping — Phase 9 closed.
Ch 60: NURBSCURVE. The structural fact of Phase 10: nurbs types live in session.objects.*, NOT
       lookup — so they must be added to EVERY map explicitly (order/rows, build, world boxes, pick;
       hide/selection ride the flags for free). Draw = sample point_at over the domain, spans×16
       clamped 32..512, → 31's segments; CVs → 32a glyphs (73's future handles); cache samples per
       guid. Box = OBB::from_nurbscurve (kernel-exact). Pick = ray↔segment over the cached samples
       (Session::ray_cast can't see curves). Tool = polyline-shaped, clicks are CONTROL points,
       ghost samples a temporary curve (real preview, not the control polygon), Enter →
       NurbsCurve::create(false, 3, points), ≥4 CVs. AddNurbsCurve Command targets the collection —
       same snapshot pattern, different container.
```

Edited: `app/scene.rs` (four arms + `register_curve`/`unregister` + sample cache),
`app/tools/curve.rs` (NEW), `app/history/add.rs` (`AddNurbsCurve`), `app/commands.rs` (`curve`).

## Next

`61-nurbssurface.md` — surfaces: the kernel tessellates a `NurbsSurface` to a mesh with baked vertex
normals, so smooth shading arrives free through 22's data-driven path. The rule that matters:
tessellate **once, cache it** — and transforms stay matrix-only (re-tessellating on every gumball
commit was the archive's measured perf bug).
