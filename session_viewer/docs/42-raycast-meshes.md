# 42 Ray-cast meshes — nearest hit wins

> **Big picture.** *Phase 7.* The ray must now answer *which object* — fast, correct under occlusion,
> at 42k objects. This is the first real consumer of 36's BVH, and the shape of the answer
> (broad-phase over boxes, then a narrow-phase in each candidate's local frame) is the same pattern
> every ray-tracer and CAD kernel uses.

The 41 ray is aimed; now hit something with it. Click a mesh and the viewer must answer *which* object,
*where*, and — when several line up behind the cursor — the **nearest** one. WebGPU has no synchronous
depth readback (you can't ask "what's under pixel (x,y)?" without stalling the pipeline), so the
interactive pick path is CPU-side: cast the ray against geometry the kernel already knows how to
intersect. This is where 36's BVH finally pays off for real.

Two stages, and the second is the subtle one. **Broad-phase**: the scene BVH turns "test all 42,232
objects" into a short candidate list along the ray. **Narrow-phase**: for each candidate mesh, the ray
must be tested in the mesh's **local frame** — `mesh.xform` is the placement (33–35), its vertices and
cached triangle BVH are local, so the *world* ray gets inverse-transformed into local space before the
test, and the hit transformed back. Nearest `t` along the ray wins; an occluded object never does.

<svg viewBox="0 0 680 200" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the world ray goes through the scene BVH broad-phase to candidate guids, then each candidate mesh is tested in its local frame via inverse xform and the kernel triangle BVH, and the nearest t wins" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="110" height="30" fill="none" stroke="#6fb3ff"/><text x="65" y="49" fill="#d7dae0" text-anchor="middle">world ray (41)</text>
  <rect x="150" y="30" width="150" height="30" fill="none" stroke="#6fb3ff"/><text x="225" y="45" fill="#d7dae0" text-anchor="middle">scene BVH ray_cast</text><text x="225" y="56" fill="#666" text-anchor="middle" font-size="9">broad-phase → candidates</text>
  <line x1="120" y1="45" x2="148" y2="45" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah42)"/>
  <line x1="300" y1="45" x2="328" y2="45" stroke="#6fb3ff" stroke-width="1.4" marker-end="url(#ah42)"/>
  <rect x="330" y="20" width="120" height="24" fill="none" stroke="#3a3a3a"/><text x="390" y="36" fill="#888" text-anchor="middle">candidate A</text>
  <rect x="330" y="52" width="120" height="24" fill="none" stroke="#3a3a3a"/><text x="390" y="68" fill="#888" text-anchor="middle">candidate B …</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.1"><rect x="470" y="24" width="200" height="60"/></g>
  <text x="570" y="42" fill="#d7dae0" text-anchor="middle">per candidate (LOCAL frame):</text>
  <text x="570" y="57" fill="#666" text-anchor="middle" font-size="10">ray → inv(mesh.xform) → local ray</text>
  <text x="570" y="72" fill="#666" text-anchor="middle" font-size="10">Mesh::triangle_bvh_ray_cast → t</text>
  <line x1="450" y1="45" x2="468" y2="45" stroke="#6fb3ff" stroke-width="1.1" marker-end="url(#ah42)"/>
  <rect x="250" y="120" width="180" height="30" fill="none" stroke="#6fb3ff"/><text x="340" y="139" fill="#d7dae0" text-anchor="middle">nearest t → PickHit{guid, point}</text>
  <line x1="570" y1="84" x2="400" y2="118" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah42)"/>
  <text x="340" y="175" fill="#888" text-anchor="middle">no WebGPU depth readback → CPU ray + BVH IS the interactive pick; occluded loses on t</text>
  <defs><marker id="ah42" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/pick.rs      # NEW — PickHit { guid, point, t }
src/app/scene.rs     # objects_along_ray (BVH broad-phase); raycast_mesh (local-frame cast); pick_ray (nearest)
src/state.rs         # on left-click: build ray (41) → scene.pick_ray → log/highlight the hit guid
```

`pick_ray` lives in `Scene` (app layer): it names `Mesh`/`BRep`/`Geometry` and mutates the kernel meshes
(the triangle BVH is built lazily on first cast). `engine/pick.rs` keeps only the ray math (41).

## Step 1 — broad-phase: which objects lie along the ray: `src/app/scene.rs`

36's `SpatialBVH` has a ray traversal built in — `ray_cast` walks only the nodes whose AABB the ray
pierces and returns their leaf object_ids. Map those back to guids exactly like 36's `objects_in`:

```rust
    /// Guids whose world box the ray pierces — the broad-phase candidate set (usually a handful, even
    /// in the 42k-object stress file). object_id → guid via `order`, same mapping as `objects_in` (36).
    pub fn objects_along_ray(&self, origin: &Point, dir: &Vector) -> Vec<String> {
        let mut ids: Vec<usize> = Vec::new();
        self.bvh.ray_cast(origin, dir, &mut ids, true);
        ids.iter().filter_map(|&i| self.order.get(i)).cloned().collect()
    }
```

## Step 2 — narrow-phase: cast in the mesh's local frame: `src/app/scene.rs`

A mesh's vertices and its cached triangle BVH are **local** (`mesh.xform` places them in the world). So
the world ray can't be cast against them directly — transform it into the mesh's local frame first, cast,
then transform the hit back to world. The kernel's transform idiom: give a `Point` an `xform` and call
`transformed()` (the same move 36 used for world boxes).

```rust
use session_rust::{Line, Mesh};

const PICK_EPS: f64 = 1e-9;

/// Cast the world ray at one mesh IN ITS LOCAL FRAME. Returns (world hit point, t along the ray).
/// `&mut Mesh` because `triangle_bvh_ray_cast` builds the triangle BVH lazily and caches it on the mesh.
fn raycast_mesh(m: &mut Mesh, ray: &Ray, eps: f64) -> Option<(Point, f64)> {
    let inv = m.xform.inverse()?;                                  // world → local; None if degenerate
    let world_far = ray.origin + ray.dir * 1.0e7;                  // a point far down the world ray
    let local_ray = Line::from_points(&inv.transform_point(&ray.origin), &inv.transform_point(&world_far));

    let local_hit = m.triangle_bvh_ray_cast(&local_ray, eps)?;     // nearest local hit, or None
    let world_hit = m.xform.transform_point(&local_hit);           // local hit → world

    let d = world_hit.clone() - ray.origin.clone();                // Point − Point → Vector
    let t = d[0]*ray.dir[0] + d[1]*ray.dir[1] + d[2]*ray.dir[2];   // signed distance along the (unit) ray
    if t >= 0.0 { Some((world_hit, t)) } else { None }             // behind the eye → not a hit
}
```

> `transform_point` is the kernel API this course *added* (kernel-gap #5, now fixed) — earlier
> drafts had to carry an xform on a cloned `Point` and call `transformed()`. The kernel's own
> `Session::ray_cast` now does exactly this local-frame dance for meshes too (gap #3, fixed); we
> keep the viewer-side cast because it reuses 61/63's cached tessellations for surfaces and BReps.

> **Why transform the ray, not the mesh.** Inverse-transforming one ray (two points) is O(1); baking
> `mesh.xform` into every vertex would be O(vertices) *and* would throw away the mesh's cached local
> triangle BVH — the whole reason the kernel's `triangle_bvh_ray_cast` is fast. Move the ray to the
> geometry's frame, never the geometry to the ray's.

## Step 3 — nearest wins: `src/app/scene.rs`

Broad-phase to candidates, cast each, keep the smallest `t`. `Mesh` and `BRep` both resolve to a mesh
(`BRep::mesh()`); everything else falls to 44's thin-geometry path:

```rust
impl Scene {
    pub fn pick_ray(&mut self, ray: &Ray) -> Option<crate::app::pick::PickHit> {
        // Owned guids so the broad-phase borrow of self.bvh/self.order is released before we mutate meshes.
        let cands: Vec<String> = self.objects_along_ray(&ray.origin, &ray.dir);
        let mut best: Option<crate::app::pick::PickHit> = None;
        for guid in cands {
            let hit = match self.session.lookup.get_mut(&guid) {
                Some(session_rust::Geometry::Mesh(m)) => raycast_mesh(m, ray, PICK_EPS),
                Some(session_rust::Geometry::BRep(b)) => { let mut bm = b.mesh(); raycast_mesh(&mut bm, ray, PICK_EPS) }
                _ => None,   // Line/Polyline/Point → lesson 44 (thin geometry needs a pick radius)
            };
            if let Some((point, t)) = hit {
                if best.as_ref().map_or(true, |h| t < h.t) {
                    best = Some(crate::app::pick::PickHit { guid, point, t });
                }
            }
        }
        best
    }
}
```

(`pick_ray` needs `&mut self` only because the kernel's lazy triangle BVH builds through plain
mutation — kernel-gap #9 in `_KERNEL_GAPS.md`; interior mutability there would make picking `&self`.)

And the tiny result type, `src/app/pick.rs`:

```rust
use session_rust::Point;

pub struct PickHit {
    pub guid: String,
    pub point: Point,   // world-space hit
    pub t: f64,         // distance along the ray (nearest wins)
}
```

> **BRep re-tessellates per pick.** `b.mesh()` builds a fresh `Mesh` (and thus a fresh triangle BVH) on
> every ray — fine for a click, wasteful for hover-picking a BRep-heavy scene. The fix is to cache each
> BRep's render mesh (the same cache 34 already wants for drawing); noted, not built here.

## Step 4 — wire the click + a headless test: `src/state.rs`

```rust
    // on left-button press (extends 41's ray build):
    if let Some(ray) = engine::pick::screen_to_world_ray(&vp, &origin, self.cursor, viewport) {
        match self.scene.pick_ray(&ray) {
            Some(hit) => log::info!("picked {} at ({:.1},{:.1},{:.1}), t={:.1}", hit.guid, hit.point[0], hit.point[1], hit.point[2], hit.t),
            None      => log::info!("picked nothing"),
        }
    }
```

A `#[cfg(test)]` pins the math without a browser: a mesh with a known triangle, a ray fired straight at
it → `pick_ray` returns that guid with a `point` on the triangle; a ray offset to miss → `None`; two
stacked meshes → the **nearer** guid, never the far one (flip the ray direction and the answer flips).

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **Click each object** → its guid logs, and the `point` sits on the surface you clicked (drop a 32
  glyph at `hit.point` to see it land on the face).
- **Occlusion** → click where a near box hides a far one; the near guid wins every time. Orbit so the
  far one is now in front, click again — the answer flips. That flip is the `t` comparison working.
- **Placement** → move a mesh far from the origin with its `xform` and pick it; the hit still lands (the
  local-frame transform, Step 2). If a moved mesh becomes unpickable, `raycast_mesh` is testing the world
  ray against local vertices — the `inv`/`to_local` step is missing.
- **Miss** → click empty space → `picked nothing`, no candidate survives the triangle test.

## Recap

```
Ch 41: screen → world ray.
Ch 42: RAY-CAST MESHES. Broad-phase: 36's SpatialBVH::ray_cast walks only pierced nodes → candidate
       guids (objects_along_ray, object_id→order→guid). Narrow-phase per candidate, in the mesh's LOCAL
       frame: inverse-transform the world ray by mesh.xform (transform the RAY, not the mesh — O(1), keeps
       the cached local triangle BVH), call the kernel's Mesh::triangle_bvh_ray_cast (lazy triangle BVH,
       nearest local hit), transform the hit back to world, compute t along the ray. pick_ray keeps the
       smallest t → PickHit{guid, point, t}; occluded objects lose on t, always. BRep resolves via
       b.mesh() (re-tessellates per pick — cache noted). No WebGPU depth readback exists, so this CPU
       ray+BVH IS the interactive pick. Thin geometry (Line/Polyline/Point) has no area to hit — that's 44.
```

Edited: `app/pick.rs` (NEW — `PickHit`), `app/scene.rs` (`objects_along_ray` BVH broad-phase,
`raycast_mesh` local-frame cast, `pick_ray` nearest-wins), `state.rs` (click → ray → `pick_ray` → log).

## Next

`43-subobject-picking.md` — a hit tells you *which mesh*; sub-object picking tells you *which part*. From
the hit triangle, resolve the nearest **vertex** (within a screen-pixel radius), else the nearest **edge**
(point-to-segment distance), else the **face** — returning a `SubHit { guid, kind, key }` that the gumball
and edit tools act on. The pixel-radius test is the same screen-space trick 44 needs for thin geometry.
