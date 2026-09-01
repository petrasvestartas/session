# 66 Ray-cast meshes — nearest hit wins

> **Big picture.** *Phase 7.* The ray must now answer *which object* — fast, correct under occlusion,
> at 42k objects. This is the first real consumer of 40's BVH, and the shape of the answer
> (broad-phase over boxes, then a narrow-phase in each candidate's local frame) is the same pattern
> every ray-tracer and CAD kernel uses.

The 46 ray is aimed; now hit something with it. Click a mesh and the viewer must answer *which* object,
*where*, and — when several line up behind the cursor — the **nearest** one. WebGPU has no synchronous
depth readback, and this course goes **CPU-side**: cast the ray against geometry the kernel already
knows how to intersect. This is where 40's BVH finally pays off for real.

> **Why CPU — and what the GPU route would look like.** Picking on the GPU *is* possible: draw an
> offscreen id-buffer (object id as color) and `mapAsync` the one pixel under the cursor. Two reasons
> this course doesn't. First, the latency shape: `mapAsync` answers a frame or more *later* — fine for
> a click, wrong for hover-highlight and drag feedback, which want the hit within the same event.
> Second, the data is already here: 40's BVH + the kernel's triangle cast answer "which object, which
> point" in f64 world space with no extra render pass, no readback stall, and no second pipeline to
> keep in sync — and the same machinery serves marquee (58) and snapping (72). The id-buffer stays
> attractive when scenes outgrow even the broad-phase scan — 81 revisits it.

Two stages, and the second is the subtle one. **Broad-phase**: the scene BVH turns "test all 42,232
objects" into a short candidate list along the ray. **Narrow-phase**: for each candidate mesh, the ray
must be tested in the mesh's **local frame** — the row's **placed frame** (40's `placed_frame(row)`:
the manifest place × session world xform that `add_file` stored in `tables.objects[row].0`) is the
placement, the vertices and cached triangle BVH are local, so the *world* ray gets inverse-transformed
into local space before the test, and the hit transformed back. Nearest `t` along the ray wins; an occluded object never does.

<svg viewBox="0 0 680 200" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the world ray goes through the scene BVH broad-phase to candidate rows, then each candidate mesh is tested in its local frame via the inverse placed frame and the kernel triangle BVH, and the nearest t wins" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="110" height="30" fill="none" stroke="#6fb3ff"/><text x="65" y="49" fill="#d7dae0" text-anchor="middle">world ray (54)</text>
  <rect x="150" y="30" width="150" height="30" fill="none" stroke="#6fb3ff"/><text x="225" y="45" fill="#d7dae0" text-anchor="middle">scene BVH ray_cast</text><text x="225" y="56" fill="#666" text-anchor="middle" font-size="9">broad-phase → candidates</text>
  <line x1="120" y1="45" x2="148" y2="45" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah42)"/>
  <line x1="300" y1="45" x2="328" y2="45" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah42)"/>
  <rect x="330" y="20" width="120" height="24" fill="none" stroke="#3a3a3a"/><text x="390" y="36" fill="#888" text-anchor="middle">candidate A</text>
  <rect x="330" y="52" width="120" height="24" fill="none" stroke="#3a3a3a"/><text x="390" y="68" fill="#888" text-anchor="middle">candidate B …</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.1"><rect x="470" y="24" width="200" height="60"/></g>
  <text x="570" y="42" fill="#d7dae0" text-anchor="middle">per candidate (LOCAL frame):</text>
  <text x="570" y="57" fill="#666" text-anchor="middle" font-size="10">ray → inv(placed_frame) → local ray</text>
  <text x="570" y="72" fill="#666" text-anchor="middle" font-size="10">Mesh::triangle_bvh_ray_cast → t</text>
  <line x1="450" y1="45" x2="468" y2="45" stroke="#6fb3ff" stroke-width="1.1" marker-end="url(#ah42)"/>
  <rect x="250" y="120" width="180" height="30" fill="none" stroke="#6fb3ff"/><text x="340" y="139" fill="#d7dae0" text-anchor="middle">nearest t → PickHit{row, guid, point}</text>
  <line x1="570" y1="84" x2="400" y2="118" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah42)"/>
  <text x="340" y="175" fill="#888" text-anchor="middle">no WebGPU depth readback → CPU ray + BVH IS the interactive pick; occluded loses on t</text>
  <defs><marker id="ah42" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/pick.rs      # NEW — PickHit { row, guid, point, t }
# objects_along_ray (BVH broad-phase); raycast_mesh (local-frame cast); pick_ray (nearest)
src/app/scene.rs
src/state.rs         # on left-click: build ray (54) → scene.pick_ray → log/highlight the hit guid
```

`pick_ray` lives in `Scene` (app layer): it names `Mesh`/`BRep`/`Geometry` and mutates the kernel meshes
(the triangle BVH is built lazily on first cast). `engine/pick.rs` keeps only the ray math (54).

## Step 1 — broad-phase: which objects lie along the ray: `src/app/scene.rs`

40's `SpatialBVH` has a ray traversal built in — `ray_cast` walks only the nodes whose AABB the ray
pierces and returns their leaf object_ids. The tree was fed boxes in `order` order (52), so an
object_id indexes **`order`** — and the ROW is one `guid_to_row` lookup away, same mapping as 40's
`objects_in`. (object_id == row held on the first load only: 46's reconcile keeps rows stable while
`order` is rebuilt, and its `pick_after_reconcile` test pins exactly this indirection.)

```rust
    /// Candidates whose world box the ray pierces — the broad-phase set (usually a handful,
    /// even in the 42k-object stress file). object_id indexes `order` (52); the row is
    /// `guid_to_row[guid]` — identical on first load, correct after a reconcile (49).
    pub fn objects_along_ray(&self, origin: &Point, dir: &Vector) -> Vec<(u32, String)> {
        let mut ids: Vec<usize> = Vec::new();
        self.bvh.ray_cast(origin, dir, &mut ids, true);
        ids.iter()
            .filter_map(|&i| self.order.get(i))
            .map(|g| (self.guid_to_row[g], g.clone()))
            .collect()
    }
```

## Step 2 — narrow-phase: cast in the mesh's local frame: `src/app/scene.rs`

A mesh's vertices and its cached triangle BVH are **local** — the row's placed frame places them in
the world, and the geometry itself carries no placement (52). So the world ray can't be cast against
them directly — transform it into the local frame by the frame's inverse first, cast, then transform
the hit back to world. The frame comes in as a parameter: the caller reads it off the row
(`scene.placed_frame(row)`, the same matrix the instance row draws with).

```rust
// Line/Mesh are already in scene.rs's session_rust import (35); only the ray type is new:
use crate::engine::pick::Ray;   // 46's Ray { origin: Point, dir: Vector }

const PICK_EPS: f64 = 1e-9;

/// Cast the world ray at one mesh IN ITS LOCAL FRAME — `frame` is the row's placed frame
/// (scene.placed_frame(row), 40). Returns (world hit point, t along the ray).
/// `&mut Mesh` because `triangle_bvh_ray_cast` builds the triangle BVH lazily and caches it
/// on the mesh.
fn raycast_mesh(m: &mut Mesh, frame: &Xform, ray: &Ray, eps: f64) -> Option<(Point, f64)> {
    // world → local; None if degenerate
    let inv = frame.inverse()?;
    let world_far = &ray.origin + &ray.dir * 1.0e7;                // a point far down the world ray
                                                                   // (borrow: Point/Vector aren't Copy)
    let local_ray = Line::from_points(&inv.transform_point(&ray.origin),
                                      &inv.transform_point(&world_far));

    let local_hit = m.triangle_bvh_ray_cast(&local_ray, eps)?;     // nearest local hit, or None
    let world_hit = frame.transform_point(&local_hit);             // local hit → world

    let d = world_hit.clone() - ray.origin.clone();                // Point − Point → Vector
    // signed distance along the (unit) ray
    let t = d[0]*ray.dir[0] + d[1]*ray.dir[1] + d[2]*ray.dir[2];
    if t >= 0.0 { Some((world_hit, t)) } else { None }             // behind the eye → not a hit
}
```

> `transform_point` is the kernel API this course *added* (kernel-gap #5, now fixed) — earlier
> drafts had to carry an xform on a cloned `Point` and call `transformed()`. The kernel's own
> `Session::ray_cast` composes the session's world xforms internally for its mesh arm too — but it
> knows nothing about the manifest `place`; we keep the viewer-side cast because it works in the
> full placed frame and reuses 66/68's cached tessellations for surfaces and BReps.

> **Why transform the ray, not the mesh.** Inverse-transforming one ray (two points) is O(1); baking
> the placed frame into every vertex would be O(vertices) *and* would throw away the mesh's cached local
> triangle BVH — the whole reason the kernel's `triangle_bvh_ray_cast` is fast. Move the ray to the
> geometry's frame, never the geometry to the ray's.

## Step 3 — nearest wins: `src/app/scene.rs`

Broad-phase to candidates, cast each, keep the smallest `t`. `Mesh` and `BRep` both resolve to a mesh
(`BRep::mesh()`); everything else falls to 49's thin-geometry path:

```rust
impl Scene {
    pub fn pick_ray(&mut self, ray: &Ray) -> Option<crate::app::pick::PickHit> {
        // Owned (row, guid) pairs so the broad-phase borrow of self.bvh/self.order is released
        // before we mutate meshes.
        let cands: Vec<(u32, String)> = self.objects_along_ray(&ray.origin, &ray.dir);
        let mut best: Option<crate::app::pick::PickHit> = None;
        for (row, guid) in cands {
            // Borrow order matters: CLONE the frame out first (placed_frame borrows &self),
            // THEN resolve the owning doc — get_mut below needs &mut on that doc's session,
            // and the two borrows must not overlap.
            let frame = self.placed_frame(row).clone();
            let d = self.doc_of_row(row);
            let hit = match self.docs[d].session.lookup.get_mut(&guid) {
                // lookup values are Rc — Rc::make_mut gives the &mut the lazy BVH build needs
                Some(session_rust::Geometry::Mesh(m)) =>
                    raycast_mesh(std::rc::Rc::make_mut(m), &frame, ray, PICK_EPS),
                Some(session_rust::Geometry::BRep(b)) => {
                    let mut bm = b.mesh();
                    raycast_mesh(&mut bm, &frame, ray, PICK_EPS)
                }
                // 66's tessellated surface is a solid too — route BOTH surface kinds through the
                // mesh cast, or they fall through here AND 49's thin filter and are unpickable.
                Some(session_rust::Geometry::NurbsSurface(s)) => {
                    let mut sm = s.mesh();
                    raycast_mesh(&mut sm, &frame, ray, PICK_EPS)
                }
                Some(session_rust::Geometry::Element(el)) => match el.geometry() {
                    // geometry() yields &Mesh — no &mut path — so clone; the lazy-BVH win is
                    // lost for elements (each pick re-clones + rebuilds). Noted, like BRep below.
                    ElementGeometry::Mesh(m) => {
                        let mut mc = m.clone();
                        raycast_mesh(&mut mc, &frame, ray, PICK_EPS)
                    }
                    ElementGeometry::BRep(b) => {
                        let mut bm = b.mesh();
                        raycast_mesh(&mut bm, &frame, ray, PICK_EPS)
                    }
                    ElementGeometry::None => None,   // add_file gave it no row — never a candidate
                },
                _ => None,   // Line/Polyline/Point/PointCloud → lesson 68 (thin geometry needs a
                             // pick radius). Plane/OBB draw as linework but have no pick arm in
                             // either lesson — a tracked gap, same shape as 49's four kinds
            };
            if let Some((point, t)) = hit {
                if best.as_ref().map_or(true, |h| t < h.t) {
                    best = Some(crate::app::pick::PickHit { row, guid, point, t });
                }
            }
        }
        best
    }
}
```

(`pick_ray` needs `&mut self` only because the kernel's lazy triangle BVH builds through plain
mutation — kernel-gap #9 in `_KERNEL_GAPS.md`; interior mutability there would make picking `&self`.)

And the tiny result type, `src/app/pick.rs` (declare it in `src/app/mod.rs` beside the others:
`pub mod pick;`):

```rust
use session_rust::Point;

pub struct PickHit {
    pub row: u32,       // global row — doc resolution (doc_of_row) and highlight both key off it
    pub guid: String,
    pub point: Point,   // world-space hit
    pub t: f64,         // distance along the ray (nearest wins)
}
```

> **BRep re-tessellates per pick.** `b.mesh()` builds a fresh `Mesh` (and thus a fresh triangle BVH) on
> every ray — fine for a click, wasteful for hover-picking a BRep-heavy scene. The same goes for the
> `NurbsSurface` and `Element` arms above (a tessellation per pick, plus a full mesh *clone* for an
> element's mesh). The fix is to cache each object's render mesh (the same cache 66/68 want for
> drawing); noted, not built here.

> **`Rc::make_mut` deep-clones a shared mesh on first pick.** `make_mut` only mutates in place when
> the `Rc` is uniquely held; a mesh the kernel shares (the same geometry referenced from two tree
> nodes) is cloned wholesale the first time you pick it — a pause proportional to the mesh, and the
> clone's lazily-built BVH is what you pay next. Unique meshes (the common case after a load) pay
> nothing. If picks on instanced/shared content ever hitch, this line is why.

> **`PICK_EPS` is absolute world units.** `1e-9` is right for millimetre-scale CAD; a scene authored
> in metres-with-millimetre details (or microns) wants the epsilon scaled to the object's box —
> e.g. `1e-9 * frame_scale` or a fraction of the candidate's world diagonal. The kernel's
> `line_line` tolerance in 57 has the same property; both are constants here for clarity.

## Step 4 — wire the click + a headless test: `src/state.rs`

```rust
    // In State::on_left_click (46 Step 3b) — REPLACE the z=0 ground-plane block inside the
    // `if let Some(ray)` with the pick_ray match. The vp/origin/viewport locals are 46's,
    // unchanged; the whole method now reads:
    let vp = self.camera.view_proj(self.aspect());
    let origin = self.camera.origin();
    let viewport = (0.0, 0.0, self.gpu.config.width as f64, self.gpu.config.height as f64);
    if let Some(ray) =
        crate::engine::pick::screen_to_world_ray(&vp, &origin, self.cursor, viewport) {
        match self.scene.pick_ray(&ray) {
            Some(hit) => log::info!("picked {} at ({:.1},{:.1},{:.1}), t={:.1}",
                hit.guid, hit.point[0], hit.point[1], hit.point[2], hit.t),
            None      => log::info!("picked nothing"),
        }
    }
```

A `#[cfg(test)]` pins the math without a browser: a mesh with a known triangle, a ray fired straight at
it → `pick_ray` returns that guid with a `point` on the triangle; a ray offset to miss → `None`; two
stacked meshes → the **nearer** guid, never the far one (flip the ray direction and the answer flips).

## Splats are not hit — yet

The ray walks the scene BVH, and streamed clouds have no leaf in it (52); a click straight
through a 3.6M-point scan reports the sheet behind it. That is the declared contract from
42 — streamed clouds are display objects — and the honest fix (a screen-space depth probe
against the splat depth buffer, not a CPU ray-vs-points test) is its own future lesson.
Walked kernel `PointCloud` objects are different: they have rows and bounds, and land in
lesson 68 with the other thin geometry.

## Step 4b — curved types pick through their caches

The geometry block (43–47) put curves, surfaces, BReps and trims on screen before picking
existed; this map is born with their arms already in it. The cached tessellation IS the
pick proxy — it owns its `Mesh`, so the archive's re-tessellate-per-ray bug cannot come
back. In `pick_mesh`'s candidate match, beside the `Mesh` arm — surfaces, BReps and
Trimmed all live in the SAME `tess_cache` (44/46/47; the entry is `(mesh, linework)`), so
all three are one shape of arm:

```rust
                // cached tessellation as the pick proxy — same local-frame contract as the Mesh
                // arm (`frame` is the row's placed frame, the cached mesh is local), minus the
                // Rc: the cache OWNS its Mesh, so the lazy triangle-BVH build needs no make_mut.
                Some(session_rust::Geometry::NurbsSurface(_))
                | Some(session_rust::Geometry::BRep(_)) =>
                    self.tess_cache.get_mut(&guid)
                        .and_then(|(m, _)| raycast_mesh(m, &frame, ray, PICK_EPS)),
```

with `Trimmed` reaching the same two lines through its `ObjRef` arm. Curves are thin — they
land next lesson (57) with the other ray↔segment types. The `all_objects()` iterator (47)
feeds the candidate walk, so no source can be forgotten.

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **Click each object** → its guid logs, and the `point` sits on the surface you clicked (drop a 32
  glyph at `hit.point` to see it land on the face).
- **Occlusion** → click where a near box hides a far one; the near guid wins every time. Orbit so the
  far one is now in front, click again — the answer flips. That flip is the `t` comparison working.
- **Placement** → give a mesh's file a manifest `place` far from the origin and pick it; the hit still
  lands (the placed-frame transform, Step 2). If a moved mesh becomes unpickable, `raycast_mesh` is
  testing the world ray against local vertices — the `inv`/`to_local` step is missing.
- **Miss** → click empty space → `picked nothing`, no candidate survives the triangle test.

## Recap

```
Ch 46: screen → world ray.
Ch 47: RAY-CAST MESHES. Broad-phase: 40's SpatialBVH::ray_cast walks only pierced nodes → candidate
       (row, guid) pairs (objects_along_ray — object_id indexes `order`; the row is one
       guid_to_row lookup away, the identity only ever held before 46's reconcile). Narrow-phase
       per candidate, in the mesh's LOCAL frame: inverse-transform the world ray by the row's
       placed frame (placed_frame(row) — geometry carries no xform; transform the RAY, not the mesh
       — O(1), keeps the cached local triangle BVH), call the kernel's Mesh::triangle_bvh_ray_cast
       (lazy triangle BVH, nearest local hit), transform the hit back to world, compute t along the
       ray. pick_ray keeps the smallest t → PickHit{row, guid, point, t} (doc via doc_of_row, mesh
       via Rc::make_mut on the doc's lookup — a SHARED mesh is deep-cloned on first pick);
       occluded objects lose on t,
       always. BRep resolves via b.mesh(), NurbsSurface via s.mesh(), Element(Mesh) via a clone
       (all re-tessellate per pick — cache noted). CPU ray+BVH is the interactive pick BY CHOICE
       (the GPU id-buffer + mapAsync route answers a frame late — see the box up top). Thin geometry
       (Line/Polyline/Point) has no area to hit — that's 49.
```

Edited: `app/pick.rs` (NEW — `PickHit`), `app/scene.rs` (`objects_along_ray` BVH broad-phase,
`raycast_mesh` local-frame cast, `pick_ray` nearest-wins), `state.rs` (click → ray → `pick_ray` → log).

## Next

`67-subobject-picking.md` — a hit tells you *which mesh*; sub-object picking tells you *which part*. From
the hit triangle, resolve the nearest **vertex** (within a screen-pixel radius), else the nearest **edge**
(point-to-segment distance), else the **face** — returning a `SubHit { row, guid, kind }` that the gumball
and edit tools act on. The pixel-radius test is the same screen-space trick 49 needs for thin geometry.
