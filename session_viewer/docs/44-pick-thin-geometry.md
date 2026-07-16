# 44 Pick thin geometry — lines and points need a radius

> **Big picture.** *Phase 7.* The stress file is 42,000 lines and the cursor must select one. But a
> line is 1-D and a point is 0-D — a mathematical ray passes *near* them, never *through* them, so
> 42's triangle test can't ever hit one. Every CAD app solves this the same way: give the pick a
> **radius**. "Did I click the line?" becomes "is the line within ~8 px of the cursor?" — and one
> priority rule keeps it sane when thin and solid geometry overlap.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a ray misses a line exactly but a tolerance cylinder around the ray catches it; at equal depth a mesh face beats a line lying on it" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <line x1="20" y1="70" x2="300" y2="60" stroke="#4a4a4a" stroke-width="1"/>
  <line x1="20" y1="55" x2="300" y2="45" stroke="#3a3a3a" stroke-dasharray="4 3"/>
  <line x1="20" y1="85" x2="300" y2="75" stroke="#3a3a3a" stroke-dasharray="4 3"/>
  <text x="310" y="63" fill="#888">ray + tolerance tube</text>
  <line x1="80" y1="110" x2="240" y2="20" stroke="#6fb3ff" stroke-width="2"/>
  <text x="245" y="18" fill="#6fb3ff">line — inside the tube → HIT</text>
  <text x="30" y="135" fill="#666">tolerance = R_PX converted to world units at the pick depth</text>
  <g transform="translate(430,0)">
    <rect x="10" y="40" width="180" height="60" fill="none" stroke="#4a4a4a"/>
    <line x1="10" y1="70" x2="190" y2="70" stroke="#6fb3ff" stroke-width="2"/>
    <text x="100" y="30" fill="#888" text-anchor="middle">line ON a face, equal depth</text>
    <text x="100" y="120" fill="#d7dae0" text-anchor="middle">priority: MESH wins ties</text>
    <text x="100" y="136" fill="#666" text-anchor="middle" font-size="10">thin wins only if clearly in front</text>
  </g>
</svg>

## The plan — reuse the kernel's cast, keep 42's mesh path

The kernel already ships exactly this: `Session::ray_cast(origin, dir, tolerance) -> Vec<RayHit>`
tests **lines** with `intersection::line_line(ray, line, tolerance)`, **polylines** per segment,
**points** by perpendicular distance ≤ tolerance — and it *force-adds all thin geometry to the
candidate list* because their BVH boxes are near-degenerate (the kernel documents this itself). It
was the archive's entire pick path.

Two of its arms we still route around: its **Mesh** arm — which *used* to be placement-blind
(kernel-gap #3, since **fixed**: it now casts in the mesh's local frame and returns world hits) —
and its **BRep** arm, a no-op by design ("viewers must use pre-cached tessellations", gap #7, open).
We keep 42's viewer-side cast for solids anyway: it serves meshes, BReps, *and* tessellated surfaces
from one cached path, which the kernel can't until #7 lands. So: **kernel cast for thin, 42 for
solid, merge with a priority rule** — now a performance choice rather than a correctness workaround.

## Files we touch

```
src/camera.rs      # world_per_pixel(depth) — R_PX → world units (the shader's screen_radius, on the CPU)
src/app/scene.rs   # pick_thin (kernel ray_cast, filtered to thin guids); pick_ray merges solid + thin
src/state.rs       # unchanged call site — pick_ray now returns line/point hits too
```

## Step 1 — pixels → world units: `src/camera.rs`

The pick radius is *pixels* (zoom-independent, like 43), but the kernel wants a *world* tolerance. The
conversion is the same formula `cylinder.wgsl` (31) uses to size screen-constant tubes — evaluated
once on the CPU, at the camera-target depth (a good proxy for where the user is looking):

```rust
    /// World size of one screen pixel at `depth` (view-space distance). Mirrors screen_radius() in
    /// cylinder.wgsl: perspective scales with depth; ortho is constant.
    pub fn world_per_pixel(&self, depth: f64, proj_y: f64, ortho_h: f64, vp_h: f64) -> f64 {
        if ortho_h > 0.0 { ortho_h / vp_h } else { depth / (proj_y * vp_h) }
    }
```

(`proj_y`/`ortho_h`/`vp_h` are the numbers 31 already packs into the line uniform — pass the same
values. `depth` = `self.distance`, the orbit camera's target distance from lesson 10.)

## Step 2 — the thin cast: `src/app/scene.rs`

```rust
use session_rust::Geometry;

impl Scene {
    /// Nearest thin hit (Line / Polyline / Point) within `tol` world units of the ray, as a PickHit.
    /// `&mut self`: Session::ray_cast rebuilds its cached BVH lazily.
    pub fn pick_thin(&mut self, ray: &Ray, tol: f64) -> Option<PickHit> {
        let hits = self.session.ray_cast(&ray.origin, &ray.dir, tol);   // sorted by distance
        for h in hits {
            match self.session.lookup.get(h.guid()) {
                // Kernel mesh/BRep arms are placement-blind / no-op — 42's pick_mesh owns solids.
                Some(Geometry::Line(_)) | Some(Geometry::Polyline(_)) | Some(Geometry::Point(_)) => {
                    return Some(PickHit { guid: h.guid().to_string(), point: h.point.clone(), t: h.distance });
                }
                _ => continue,
            }
        }
        None
    }
}
```

Wait — `hits` borrows `self.session` mutably and the loop reads `lookup`. Split the borrow: collect
the `(guid, point, distance)` triples out of the returned `Vec<RayHit>` first (it's owned — the
mutable borrow ends when `ray_cast` returns), then match against `lookup`. The returned `Vec` *is*
owned, so the code above compiles as written; the note is here because `h.guid()` lazily fills a
`OnceLock` — read it once into a `String` if the borrow checker complains about the temporaries.

## Step 3 — merge: solid vs thin priority: `src/app/scene.rs`

Rename 42's `pick_ray` to `pick_mesh` and make `pick_ray` the umbrella:

```rust
    /// The one entry point clicks use. Mesh hit + thin hit → one winner:
    /// thin wins ONLY if it is clearly in front (more than `tol` nearer); ties go to the MESH —
    /// a line lying ON a face must not steal the click from the face under it.
    pub fn pick_ray(&mut self, ray: &Ray, tol: f64) -> Option<PickHit> {
        let solid = self.pick_mesh(ray);          // 42, renamed — unchanged inside
        let thin  = self.pick_thin(ray, tol);
        match (solid, thin) {
            (Some(s), Some(t)) => Some(if t.t < s.t - tol { t } else { s }),
            (s, t) => s.or(t),
        }
    }
```

> **Why mesh wins ties.** A polyline drawn on a floor slab sits at *exactly* the slab's depth. With a
> naive "smallest t wins", the fattened thin test would grab every click near it — the slab becomes
> unselectable anywhere close to its outline. Requiring the thin hit to be **more than `tol` nearer**
> means: line alone in space → picks fine; line on a surface → the surface wins, select the line by
> clicking where the surface isn't (or via the tree, 70). This is Rhino's behaviour, and the archive's
> rule (`reference_viewer_picking_system`).

The click site (42's Step 4) just adds the tolerance:

```rust
        let tol = self.camera.world_per_pixel(self.camera.distance, proj_y, ortho_h, vp_h) * 8.0;   // R_PX = 8
        match self.scene.pick_ray(&ray, tol) { … }
```

## Step 4 — verify (including the stress gate)

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **Line alone in space** → click within a few px of it → its guid logs; click 20 px away → nothing.
  Zoom far out — it stays pickable with the same *screen* slop (the world tolerance grew with depth).
- **Line lying ON a mesh face** → clicking the line picks the **mesh**; the line only wins where it
  hangs off the face. That's the priority rule doing its job.
- **STRESS GATE** — load the PDF drawing (34b), zoom mid-way, click single lines in dense hatching:
  the *intended* line logs (nearest along the ray inside the tolerance), and the click returns
  instantly — the kernel's cached ray-BVH plus per-type distance tests, no freeze at 42k objects.
- A `#[cfg(test)]` pins the merge: a mesh at t=10 and a line at t=10±ε → mesh; line at t=5 → line.

## Recap

```
Ch 43: sub-object — vertex/edge/face by screen-pixel proximity.
Ch 44: THIN GEOMETRY. A ray never exactly hits a 1-D/0-D object, so the pick gets a RADIUS: R_PX (8)
       converted to world units by the SAME formula cylinder.wgsl uses for screen-constant tubes,
       evaluated at target depth. The kernel's Session::ray_cast(origin, dir, tol) already implements
       the thin narrow-phase (line_line with tolerance, per-segment polylines, perpendicular distance
       for points — thin candidates force-added past the degenerate boxes); we filter its results to
       thin guids (its mesh arm is placement-blind, its BRep arm a deliberate no-op — 42 owns solids)
       and merge: thin wins only if MORE than tol nearer, ties → MESH, so a line on a face never
       steals the face's click. Stress gate: intended line picked from 42k instantly.
```

Edited: `camera.rs` (`world_per_pixel`), `app/scene.rs` (`pick_thin` via kernel `ray_cast`,
`pick_ray` = solid/thin merge, 42's cast renamed `pick_mesh`), `state.rs` (tolerance at the call).

## Next

`45-selection.md` — picking finds objects; selection *keeps* them. `FLAG_SELECTED` in the instance row
tints everything the object owns (faces, edges, glyphs — one bit, all pipelines), click/Shift+click
behave like Rhino, and a drag rectangle becomes a 4-plane sub-frustum queried against 36's BVH —
marquee select at 42k objects.
