# 63 BRep — many faces, one object

> **Big picture.** *Phase 10.* The BRep (boundary representation) is the kernel's solid-modeling
> heavyweight — the type booleans, STEP files, and real CAD assemblies live in. The viewer has
> half-supported it since 34 (`b.mesh()` → draw), but three debts accumulated: it **re-tessellates
> on every use** (34's noted caveat, 42's pick does it *per ray*), its **edges** draw as tessellation
> wireframe instead of its real curve network, and its transform path re-flattens. This lesson pays
> all three with machinery that now exists: 61's cache, 62's linework split, 54's matrix-only commit.

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
    <text x="0" y="54" fill="#666" font-size="10">edges = m_curves_3d sampled → 31's tubes (62's recipe)</text>
    <text x="0" y="70" fill="#666" font-size="10">transform = compose b.xform, set_object_row (61's rule)</text>
    <text x="0" y="94" fill="#888" font-size="10">click any face → the whole solid selects (object-level, 42)</text>
  </g>
</svg>

## Files we touch

```
src/app/scene.rs   # BRep joins the tess_cache; edge-curve linework; matrix-only commit arm
```

(No new files — this is a debts-paid lesson. BRep already sits in `lookup` as a `Geometry` variant,
so 60's "every map" work was done back in 34–38; what changes is *how well* each arm treats it.)

## Step 1 — cache the tessellation: `src/app/scene.rs`

Every `b.mesh()` call re-runs the tessellation pipeline. Route BReps through 61's cache — same map,
keyed by guid, filled on first touch:

```rust
    // 61's surface_mesh generalizes: rename to render_mesh(guid) and add the BRep source —
    fn render_mesh(&mut self, guid: &str) -> Option<&(Mesh, Vec<CylinderSegment>)> {
        if !self.tess_cache.contains_key(guid) {
            let entry = if let Some(Geometry::BRep(b)) = self.session.lookup.get(guid) {
                let ri = self.guid_to_row[guid];
                Some((b.mesh(), brep_linework(b, ri)))                 // Step 2
            } else if let Some(ns) = self.session.objects.nurbssurfaces.iter()
                .find(|s| s.guid() == guid) {
                let ri = self.guid_to_row[guid];
                Some((ns.mesh(), surface_linework(ns, ri)))            // 61/62, unchanged
            } else { None };
            if let Some(e) = entry { self.tess_cache.insert(guid.to_string(), e); }
        }
        self.tess_cache.get(guid)
    }
```

Now sweep the three call sites that still call `b.mesh()` directly and point them here: the build arm
(35/38b's `apply_object`), the pick arm (42's `pick_mesh` — this one was re-tessellating **per
click**), and 36's `world_obb` BRep arm. The pick fix alone is the difference between instant and
sluggish clicks in a BRep-heavy file. Also **rename every earlier `surface_mesh` call** (61's priming
pass, 62's build/warm sites) to `render_mesh` and **drop the now-internal `ri` argument**.

## Step 2 — real edges: `src/app/scene.rs`

A BRep carries its actual edge curve network (`m_curves_3d` — the 3-D edge curves the kernel maintains
for booleans and STEP; `m_curves_2d` is the parametric trim pcurves, not what we draw). Sample them
with 60's curve recipe instead of showing tessellation wireframe:

```rust
/// The BRep's edge curves as tubes — its real topology, not its tessellation. LOCAL space (the
/// curves live in the BRep's frame; b.xform on the row places them, same as the skin).
fn brep_linework(b: &BRep, ri: u32) -> Vec<CylinderSegment> {
    let mut segs = Vec::new();
    let edge = [0.10, 0.10, 0.10, 1.0];
    for c in &b.m_curves_3d {
        let pts = sample_curve(c);                     // 60's adaptive sampler, reused as-is
        for w in pts.windows(2) {
            if w[0].distance(&w[1], None) > 1e-9 {     // zero-length filter — degenerate curve
                segs.push(CylinderSegment { p0: w[0].to_f32(), radius: 0.0,
                    p1: w[1].to_f32(), instance_id: ri, color: edge });
            }
        }
    }
    segs
}
```

(Check the exact curves field against your kernel — `m_curves_3d` holds the 3-D edge curves in the BRep
struct; if a BRep in your data has empty curves, fall back to the mesh's `edges_with_colors` so
nothing draws edgeless. The zero-length filter is a real archive fix: degenerate seam curves
otherwise emit NaN-direction tubes that flicker.)

Like 62's surfaces, BReps suppress `push_mesh`'s triangle-edge tubes — the edge *curves* are the
silhouette. The visual upgrade is immediate on anything curved: a cylinder BRep shows two circles and
a seam, not forty triangle slivers.

## Step 3 — matrix-only transforms: `src/app/scene.rs`

Add BRep to 61's fast path — it was already listed there; the work is confirming both halves:

```rust
    // apply_delta (54): b.xform = delta * &b.xform      — already present since 54
    // commit split (61): Mesh | BRep | NurbsSurface → set_object_row only, cache untouched
```

And one line in reconcile (38b): the `changed` bucket must `tess_cache.remove(guid)` before
`apply_object` — an externally reshaped BRep re-tessellates once, exactly then. (This was already
61's invalidation rule; BRep just joins it.)

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Load a BRep-heavy file (any boolean-result fixture):

- Curved BReps wear **clean edge curves** — a cylinder shows its two rim circles, not triangle
  wireframe. No flickering micro-tubes anywhere (the zero-length filter).
- **Click any face** of a multi-face solid → the *whole* BRep selects and the gumball sits at its
  volume center. Drag it — skin and edges move as one, frame rate flat, and the release doesn't
  hitch (cached; compare with 61's counter-experiment if you want to feel the difference).
- Click-pick latency: instant, even clicking the same solid repeatedly — the per-pick re-tessellation
  is gone. Undo/redo transforms round-trip.
- Hide/show, marquee, `F` — the standard audit, still green (BRep was in `lookup` all along; the
  arms just got cheaper and prettier).

## Recap

```
Ch 62: iso lines — surfaces read as surfaces.
Ch 63: BREP, debts paid. (1) tess_cache absorbs BReps — render_mesh(guid) unifies the surface/BRep
       sources; the three direct b.mesh() sites (build, PICK-per-click, world box) now hit the
       cache.
       (2) Real edges: m_curves_3d sampled with 60's adaptive sampler → 31's tubes, zero-length filter
       (degenerate seams emit NaN tubes — archive fix), triangle-edge tubes suppressed; a cylinder
       shows rims + seam, not wireframe. (3) Matrix-only transforms confirmed (54/61's split);
       reconcile's changed bucket is the ONLY other cache invalidation. One guid, one row, one pick
       target — faces and edges are one object everywhere.
```

Edited: `app/scene.rs` (`render_mesh` unification, `brep_linework`, cache sweep of the three
`b.mesh()` sites, reconcile invalidation line).

## Next

`64-trimmed.md` — the last citizen of Phase 10: `NurbsSurfaceTrimmed` — a surface with holes and cut
boundaries. First-class from day one (the archive forgot it from "every map" repeatedly), and the
occasion to turn that discipline into structure: one `all_objects()` iterator every map consumes.
