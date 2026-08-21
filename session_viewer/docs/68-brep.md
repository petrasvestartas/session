# 68 BRep — many faces, one object

> **Big picture.** *Phase 10.* The BRep (boundary representation) is the kernel's solid-modeling
> heavyweight — the type booleans, STEP files, and real CAD assemblies live in. The viewer has
> half-supported it since 34 (`b.mesh()` → draw), but three debts accumulated: it **re-tessellates
> on every use** (34b's noted caveat, 47's pick does it *per ray*), its **edges** draw as tessellation
> wireframe instead of its real curve network, and its transform path re-flattens. This lesson pays
> all three with machinery that now exists: 66's cache, 67's linework split, 59's matrix-only commit.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a brep is faces plus shared edge curves as one object; the viewer caches one tessellated mesh and draws real edge curves; picking any face selects the whole brep" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(60,18)">
    <path d="M 0,60 L 60,80 L 130,55 L 70,38 Z" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <path d="M 0,60 L 0,22 L 70,2 L 70,38" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <path d="M 70,2 L 138,18 L 130,55" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <text x="70" y="100" fill="#888" text-anchor="middle">faces share EDGE CURVES</text>
  </g>
  <g transform="translate(300,20)">
    <text x="0" y="14" fill="#d7dae0">one guid · one row · one pick target</text>
    <text x="0" y="38" fill="#666" font-size="10">tess_cache[guid] — b.mesh() runs ONCE</text>
    <text x="0" y="54" fill="#666" font-size="10">edges = m_curves_3d sampled → 31's tubes (67's recipe)</text>
    <text x="0" y="70" fill="#666" font-size="10">transform = apply_world_delta (59) — set_xform, nothing bakes</text>
    <text x="0" y="94" fill="#888" font-size="10">click any face → the whole solid selects (object-level, 47)</text>
  </g>
</svg>

## Files we touch

```
src/app/scene.rs   # BRep joins the tess_cache; edge-curve linework; matrix-only commit arm
```

(No new files — this is a debts-paid lesson. BRep already sits in `lookup` as a `Geometry` variant,
so 65's "every map" work was done back in 34–38; what changes is *how well* each arm treats it.)

## Step 1 — cache the tessellation: `src/app/scene.rs`

Every `b.mesh()` call re-runs the tessellation pipeline. Route BReps through 66's cache — same map,
keyed by guid, filled on first touch:

```rust
    // 66's surface_mesh generalizes: rename to render_mesh(guid) and add the BRep source —
    fn render_mesh(&mut self, guid: &str) -> Option<&(Mesh, Vec<CylinderSegment>)> {
        if !self.tess_cache.contains_key(guid) {
            let entry = self.docs.iter().find_map(|d| match d.session.lookup.get(guid) {
                Some(Geometry::BRep(b)) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());        // color BEFORE caching!
                    let lw = brep_linework(b, &bm);                    // mesh = fallback source
                    Some((bm, lw))                                     // Step 2
                }
                Some(Geometry::NurbsSurface(ns)) =>
                    Some((ns.mesh(), surface_linework(ns))),           // 66/67, unchanged
                _ => None,
            });
            if let Some(e) = entry { self.tess_cache.insert(guid.to_string(), e); }
        }
        self.tess_cache.get(guid)
    }
```

One trap worth its own sentence: `add_file`'s BRep arm does `let mut bm = b.mesh();
bm.set_objectcolor(b.surfacecolor.clone());` **before** `push_mesh` — the cache must store that
**colored** mesh (as above), or every cached BRep comes back white. Which states the recolor rule
this cache now lives under: **the color is baked into the cached mesh, so recoloring a BRep must
invalidate** — a future `color` command does `tess_cache.remove(&guid)` alongside the session
write, exactly like 43b's `changed` bucket, or the repaint never shows. (The alternative — keep
color out of the mesh and tint per-row — doesn't exist here, because `push_mesh` reads vertex
colors.) And like 67, the cache stays
row-agnostic (`instance_id: 0`, stamped at push): `guid_to_row` reads would be fine at the
build/pick/box call sites — those all run *after* the doc was walked — but rows don't exist yet
when a priming pass runs, so the push site stamps.

Now sweep the three call sites that still call `b.mesh()` directly and point them here:
`add_file`'s BRep arm, the pick arm (47's `pick_mesh` — this one was re-tessellating **per
click**), and 40's `world_obb` BRep arm. The pick fix alone is the difference between instant and
sluggish clicks in a BRep-heavy file. Two ripples make that sweep borrow-legal: **rename every
earlier `surface_mesh` call** (the priming pass at the top of `add_file`, 67's warm/read sites) to
`render_mesh` — same signature, 67 already made the cache row-agnostic — and **widen the priming
pass's filter** to admit `Geometry::BRep(_)` beside `NurbsSurface`, so `add_file`'s BRep arm (which
holds `t = &mut self.tables` and can no more call the `&mut self` helper than 66's surface arm
could) reads the *warmed* cache with `self.tess_cache.get(&guid)`, exactly like 67's arm.

(Two cost footnotes on `render_mesh`, both fine at this course's scale: the `docs.iter().find_map`
scan is O(docs × 1) but runs **only on a cache miss** — once per object ever, so don't reach for a
guid index; and a cache miss on the *pick* path tessellates inline on the click — the first click
on a never-drawn BRep pays its meshing once. If that ever matters, warm from the load path, not
here.)

## Step 2 — real edges: `src/app/scene.rs`

A BRep carries its actual edge curve network (`m_curves_3d` — the 3-D edge curves the kernel maintains
for booleans and STEP; `m_curves_2d` is the parametric trim pcurves, not what we draw). Sample them
with 65's curve recipe instead of showing tessellation wireframe:

```rust
/// The BRep's edge curves as tubes — its real topology, not its tessellation. LOCAL space (the
/// curves live in the BRep's frame; the row's placed frame places them, same as the skin).
/// instance_id stays 0 in the cache — stamped at push time (67's rule).
/// `mesh` is the already-built tessellation — the empty-curves fallback reads IT, never
/// a second `b.mesh()` call (re-tessellating is the bug this lesson exists to kill).
fn brep_linework(b: &BRep, mesh: &Mesh) -> Vec<CylinderSegment> {
    let mut segs = Vec::new();
    let edge = [0.10, 0.10, 0.10, 1.0];
    for c in &b.m_curves_3d {
        let pts = sample_curve(c);                     // 65's sampler, reused as-is
        for w in pts.windows(2) {
            if w[0].distance(&w[1], None) > 1e-9 {     // zero-length filter — degenerate curve
                segs.push(CylinderSegment { p0: w[0].to_f32(), radius: 0.0,
                    p1: w[1].to_f32(), instance_id: 0, color: edge });
            }
        }
    }
    if segs.is_empty() {
        // a BRep with empty m_curves_3d (real in imported data) must not draw edgeless —
        // fall back to the tessellation's own edges, HERE, at the one place that has both
        for (a, b) in mesh_edges(mesh) {   // the mesh's edges_with_colors, colors dropped
            segs.push(CylinderSegment { p0: a.to_f32(), radius: 0.0,
                                        p1: b.to_f32(), instance_id: 0, color: edge });
        }
    }
    segs
}
```

(The field is `b.m_curves_3d: Vec<NurbsCurve>` — `brep.rs:115` — the 3-D edge curves the BRep
struct maintains; the fallback lives at the end of `brep_linework` as shown — not at the push
site, which couldn't distinguish "empty curves" from "not a BRep" — and reads the mesh the caller
already built, via its `edges_with_colors` (dropping the colors; shape the iterator to your
mesh's API). The zero-length filter is a real archive fix:
degenerate seam curves otherwise emit NaN-direction tubes that flicker. One
density note: `sample_curve`'s 32-sample floor applies **per edge curve**, so a 200-edge BRep
caches ≥ 6400 points of linework — usually right, but if imported assemblies get heavy, the
lever is a smaller floor for short edges, not a global cut.)

Like 67's surfaces, BReps pass `Edges::Suppress` — the edge *curves* are the
silhouette — and like 67, the cached linework is appended to **`tables.pipes`** (the solid lane,
`instance_id` stamped with the row at push); putting it in flat `segments` would z-fight the skin
at grazing angles. The visual upgrade is immediate on anything curved: a cylinder BRep shows two
circles and a seam, not forty triangle slivers.

## Step 3 — matrix-only transforms: `src/app/scene.rs`

Nothing to add — 59's `Scene::apply_world_delta(row, &delta)` is type-blind: it rewrites the
object's Session-local xform (`session.set_xform`) and the row's placed frame; no BRep-specific
arm exists, nothing bakes, and the cache is untouched by a transform:

```rust
    // commit (59): Scene::apply_world_delta(row, &delta) — set_xform + placed frame,
    //              no per-variant match, tess_cache untouched
```

And one line in reconcile (43b): the `changed` bucket must `tess_cache.remove(guid)` before
`apply_object` — an externally reshaped BRep re-tessellates once, exactly then. (This was already
66's invalidation rule; BRep just joins it.)

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Load a BRep-heavy file (any boolean-result fixture):

- Curved BReps wear **clean edge curves** — a cylinder shows its two rim circles, not triangle
  wireframe. No flickering micro-tubes anywhere (the zero-length filter).
- **Click any face** of a multi-face solid → the *whole* BRep selects and the gumball sits at its
  volume center. Drag it — skin and edges move as one, frame rate flat, and the release doesn't
  hitch (cached; compare with 66's counter-experiment if you want to feel the difference).
- Click-pick latency: instant, even clicking the same solid repeatedly — the per-pick re-tessellation
  is gone. Undo/redo transforms round-trip.
- Hide/show, marquee, `F` — the standard audit, still green (BRep was in `lookup` all along; the
  arms just got cheaper and prettier).

## Recap

```
Ch 67: iso lines — surfaces read as surfaces.
Ch 68: BREP, debts paid. (1) tess_cache absorbs BReps — render_mesh(guid) unifies the surface/BRep
       sources; the three direct b.mesh() sites (build, PICK-per-click, world box) now hit the
       cache.
       (2) Real edges: m_curves_3d (brep.rs:115) sampled with 65's sampler → tables.pipes
       (SOLID lane, row stamped at push), zero-length filter (degenerate seams emit NaN tubes —
       archive fix), triangle-edge tubes suppressed; a cylinder shows rims + seam, not wireframe.
       Cache stores the COLORED mesh (set_objectcolor before insert, as add_file's arm does) —
       so a future recolor command must tess_cache.remove(&guid) or the repaint never shows.
       (3) Transforms: 59's apply_world_delta is type-blind — nothing to add;
       reconcile's changed bucket is the ONLY other cache invalidation. One guid, one row, one pick
       target — faces and edges are one object everywhere.
```

Edited: `app/scene.rs` (`render_mesh` unification, `brep_linework`, cache sweep of the three
`b.mesh()` sites, reconcile invalidation line).

## Next

`69-trimmed.md` — the last citizen of Phase 10: `NurbsSurfaceTrimmed` — a surface with holes and cut
boundaries. First-class from day one (the archive forgot it from "every map" repeatedly), and the
occasion to turn that discipline into structure: one `all_objects()` iterator every map consumes.
