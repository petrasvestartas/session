# 36 Scene BVH — one broad-phase, three consumers

> **Big picture.** *Phase 5 — acceleration, built BEFORE the features that need it.* Picking (42) and
> box-select (45) each ask "which objects are in this region?" against 42,000 objects, per click or
> per drag. Every real CAD app answers that with a spatial index built once and queried everywhere.
> Building it now, on the stable object list 35 just created, makes the later lessons *queries*
> instead of rewrites.

`Scene` (35) has a fixed, ordered object list. Two lessons ahead need the same question answered fast:
**which objects fall inside this box?** — picking (42: box = a thin sliver around the ray) and
box-select (45: box = the drag rectangle's sub-frustum). Both do a *per-object* test (ray↔triangle,
point-in-frustum) that is far too expensive to run N times, so they can't afford to scan all 42,232
objects in the stress file. So `Scene` gains one spatial index — an AABB **BVH** — and they query it
for a short candidate list instead. (Frustum culling, 37, turns out to ship a linear scan — it must
touch every object's flag anyway — so it *doesn't* need the tree; the same index could still accelerate
it at extreme scale. Building it once, here, means all of them share it.)

The kernel already ships the tree: `session_rust::SpatialBVH` (median-split, `build_with_guids` /
`query_aabb` / `ray_cast`). The roadmap's rule is *don't rewrite what exists* — so this lesson wires
that up, it doesn't reimplement a BVH. The real work is one subtlety the kernel can't do for us.

## Why the viewer builds its own boxes

`Session` can build a BVH itself (`get_collisions` does, for its collision graph). But its
per-object box routine, `compute_bounding_box`, treats a **Mesh** as its raw stored vertices:

```rust
// session_rust/src/session.rs — the kernel's own mesh box:
Geometry::Mesh(m) => {
    let points = m.vertex.values().map(|v| Point::new(v.x, v.y, v.z)).collect();
    OBB::from_points(&points, inflate)     // LOCAL verts — mesh.xform is NOT applied
}
```

That *was* the kernel's mesh-box code when this lesson was first written — and it disagreed with the
viewer's convention from 33–35 (`Mesh::to_render()` ignores `mesh.xform`, so **`mesh.xform` is the
placement**). Writing this lesson surfaced the inconsistency as kernel-gap #3, and **the kernel has
since been fixed**: `compute_bounding_box` and `Session::ray_cast` now bake `mesh.xform` in all
three languages. `Scene` still builds its own `world_obb` below — its BVH wants viewer-controlled
padding and the tessellation-backed boxes for surfaces — but it now agrees with the kernel instead
of correcting it. Either way the invariant stands: every box fed to the tree is a **world** box,
computed from the same placement the instance row uses.

<svg viewBox="0 0 680 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="per-object world OBBs are built by Scene and fed to the kernel SpatialBVH, whose query_aabb serves culling, picking and box-select" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="10" y="18" fill="#888">Scene::build_bvh — world boxes (placement applied)</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="14" y="28" width="150" height="26"/>
    <rect x="14" y="60" width="150" height="26"/>
    <rect x="14" y="92" width="150" height="26"/>
  </g>
  <g fill="#d7dae0"><text x="24" y="45">Mesh → local box · xform</text><text x="24" y="77">Line/Polyline → world box</text><text x="24" y="109">Point → world box</text></g>
  <line x1="164" y1="73" x2="214" y2="73" stroke="#6fb3ff" stroke-width="1.5" marker-end="url(#ah36)"/>
  <rect x="216" y="52" width="150" height="42" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="291" y="70" fill="#d7dae0" text-anchor="middle">SpatialBVH</text>
  <text x="291" y="85" fill="#666" text-anchor="middle">(kernel — reused)</text>
  <text x="291" y="112" fill="#555" text-anchor="middle" font-size="10">query_aabb(OBB) → object_ids</text>
  <g stroke="#6fb3ff" stroke-width="1.3"><line x1="366" y1="73" x2="440" y2="45" marker-end="url(#ah36)"/><line x1="366" y1="73" x2="440" y2="73" marker-end="url(#ah36)"/><line x1="366" y1="73" x2="440" y2="101" marker-end="url(#ah36)"/></g>
  <g fill="none" stroke="#3a3a3a"><rect x="442" y="32" width="220" height="26"/><rect x="442" y="60" width="220" height="26"/><rect x="442" y="88" width="220" height="26"/></g>
  <g fill="#d7dae0"><text x="452" y="49">42 picking — box = ray sliver</text><text x="452" y="77">45 box-select — box = drag frustum</text><text x="452" y="105" fill="#888">(37 frustum cull — linear; tree optional)</text></g>
  <text x="10" y="140" fill="#666">one tree, built once here; picking &amp; box-select query it instead of scanning N objects.</text>
  <defs><marker id="ah36" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/scene.rs   # Scene gains `bvh: SpatialBVH`; world_obb() per object;
                   # objects_in(query) → guids; #[cfg(test)] parity
```

Just `scene.rs` — the BVH is document state, so it lives beside `session`/`order`/`guid_to_row`, and
nothing in `engine/` learns it exists (the 35 litmus test still holds).

## Step 1 — a world box per object: `src/app/scene.rs`

Extend the top-of-file import (35 already brought `Session`/`Geometry`/`Mesh`/`Point`/…):

```rust
use session_rust::{AABB, OBB, SpatialBVH};   // ← ADD to the existing session_rust use
```

Add a free function next to 35's converters — the kernel's own transform idiom (set the box's
`xform`, then `transformed()` bakes it), so the mesh box lands where the instance row draws it:

```rust
/// The object's WORLD box. Mesh/BRep verts are LOCAL (to_render ignores xform, 33-35), so build
/// the local box and BAKE the placement — exactly the transform the instance model applies.
/// Line, Polyline and Point already hold world coordinates (identity model in objects_base), so
/// their box is direct.
fn world_obb(geom: &Geometry) -> OBB {
    // planar meshes give a zero-thickness box; a hair of pad keeps intersects robust
    const PAD: f64 = 1e-6;
    match geom {
        Geometry::Mesh(m) => {
            let mut o = OBB::from_aabb(AABB::from_mesh(m, PAD));
            o.xform = m.xform.duplicate();     // the placement 33's rebuild_instances also uses
            // bake xform → world (may tilt the box; query collapses to its AABB)
            o.transformed()
        }
        Geometry::BRep(b) => {
            let bm = b.mesh();
            let mut o = OBB::from_aabb(AABB::from_mesh(&bm, PAD));
            o.xform = b.xform.duplicate();
            o.transformed()
        }
        Geometry::Line(l) => OBB::from_line(l, PAD),
        Geometry::Polyline(pl) => OBB::from_polyline(pl, PAD),
        Geometry::Point(p) => OBB::from_point(p.clone(), PAD),
        // unreachable: `order` is pre-filtered to the 5 above
        _ => OBB::from_point(Point::new(0.0, 0.0, 0.0), PAD),
    }
}
```

`query_aabb` collapses any OBB back to its enclosing AABB internally, so a tilted mesh box just yields
a slightly looser world AABB — conservative, never a false miss. Broad-phase wants exactly that.

## Step 2 — build the tree in lock-step with `order`: `src/app/scene.rs`

`build_with_guids` keeps its input order: the `object_id` a query returns is the **index into the slice
you passed**. Build that slice in `self.order` order and the mapping back to a guid is just
`self.order[id]` — no dependence on the tree's internals.

**2a. Add the field** to `struct Scene`:

```rust
pub struct Scene {
    pub session: Session,
    order: Vec<String>,
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
    bvh: SpatialBVH,   // ← ADD — broad-phase over world boxes, object_id == index into `order`
    world_boxes: Vec<([f64; 3], [f64; 3])>,   // ← ADD — cached AABB extents, same order as `order`
}
```

**2b. Build it in `Scene::new`**, right after `order`/`guid_to_row` are filled (35), and add it to the
initializer:

```rust
        let (bvh, world_boxes) = Self::build_bvh(&session, &order);
        Self { session, order, guid_to_row, hidden: HashSet::new(), bvh, world_boxes }
    }

    /// Rebuild the whole tree from `order`. Called once at construction; a later lesson (38)
    /// refits incrementally on edit instead of rebuilding. Boxes go in `order` order →
    /// object_id == order index.
    fn build_bvh(session: &Session, order: &[String]) -> (SpatialBVH, Vec<([f64; 3], [f64; 3])>) {
        let boxes: Vec<(OBB, String)> = order.iter()
            .map(|guid| (world_obb(&session.lookup[guid]), guid.clone()))
            .collect();
        // Cache each box's AABB extents alongside the tree. Computing a world box walks the
        // object's VERTICES — do it once per (re)build, never per query. Lesson 37's per-frame
        // cull and 45's marquee read THIS cache; without it they'd re-walk every mesh every frame.
        let extents = boxes.iter().map(|(o, _)| {
            let a = o.aabb();
            let (lo, hi) = (a.min_point(), a.max_point());
            ([lo[0], lo[1], lo[2]], [hi[0], hi[1], hi[2]])
        }).collect();
        let mut bvh = SpatialBVH::new();
        bvh.build_with_guids(&boxes);   // empty slice → empty tree; query returns [] (no panic)
        (bvh, extents)
    }
```

## Step 3 — the query every later lesson calls: `src/app/scene.rs`

One public method, in `impl Scene`. It's what 37/42/45 build on — they differ only in the box they
pass:

```rust
    /// Guids whose world box intersects `query` — the broad-phase. Callers narrow further
    /// (37 tests the frustum's own planes, 42 does ray↔triangle) on this short list, not all N.
    pub fn objects_in(&self, query: &OBB) -> Vec<&str> {
        self.bvh.query_aabb(query)
            .into_iter()
            // object_id → guid, via the slice we built in Step 2
            .map(|id| self.order[id].as_str())
            .collect()
    }
```

## Step 4 — prove it: `#[cfg(test)]` broad-phase == brute force

A BVH is only trustworthy if it returns *exactly* the brute-force set — no misses, no phantoms. Add
this at the bottom of `scene.rs`; it needs no GPU, so `cargo test -p session_viewer` runs it headless:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Line;   // Point comes in via `use super::*` (35's imports)

    /// A tiny Session at known, separated positions. `add_line`/`add_mesh` return a tree node we
    /// ignore; `None` = no parent. Add a mesh the same way if you want a non-degenerate world box.
    fn demo_session() -> Session {
        let mut s = Session::new("bvh_test");
        s.add_line(Line::from_points(&Point::new(0.0, 0.0, 0.0), &Point::new(100.0, 0.0, 0.0)), None);
        s.add_line(Line::from_points(&Point::new(300.0, 300.0, 0.0), &Point::new(400.0, 300.0, 0.0)), None);
        s.add_line(Line::from_points(&Point::new(9000.0, 9000.0, 0.0), &Point::new(9100.0, 9000.0, 0.0)), None);
        s
    }

    #[test]
    fn bvh_matches_brute_force() {
        let scene = Scene::new(demo_session());
        // AABB::new is CENTER + HALF-EXTENTS (not min/max): this box spans -500..500 on each axis,
        // so it catches the two near-origin lines and excludes the one out at 9000.
        let query = OBB::from_aabb(AABB::new(0.0, 0.0, 0.0, 500.0, 500.0, 500.0));

        let mut got: Vec<&str> = scene.objects_in(&query);
        let mut want: Vec<&str> = scene.order.iter()
            .filter(|g| scene.bvh.aabb_intersect(&world_obb(&scene.session.lookup[*g]), &query))
            .map(|g| g.as_str())
            .collect();
        got.sort(); want.sort();
        assert_eq!(got, want, "BVH broad-phase must equal the linear scan");
    }
}
```

`SpatialBVH::aabb_intersect(&OBB, &OBB)` is the kernel's own overlap primitive — the same OBB →
enclosing AABB → `intersects` collapse `query_aabb` uses per node — so the brute-force scan and the
tree are tested against an identical predicate. The point is that the *tree's* set and the *scan's* set
agree on every object, at any scene size. Once green, every downstream query trusts the broad-phase.

## Run

```bash
# http://localhost:8770 — visuals UNCHANGED (nothing draws the BVH yet)
cd session_viewer && trunk serve
cargo test -p session_viewer bvh   # the parity test above
```

Nothing on screen moves this lesson — the tree is pure infrastructure. Lesson 37 is where it earns its
keep: the drawn-object count on the perf HUD drops the moment you zoom in.

## Recap

```
Ch 35: Scene owns session + order + guid_to_row + hidden, emits one ArenaUpload.
Ch 36: Scene gains ONE broad-phase — the kernel's SpatialBVH (reused, not rewritten). The catch:
       the kernel's own box routine reads mesh verts as world coords, but the viewer treats
       mesh.xform as the placement (33-35), so `world_obb()` builds each mesh/BRep box LOCAL then
       bakes xform (OBB::from_aabb → set .xform → transformed()); lines/polylines/points are already
       world. build_bvh feeds those boxes to build_with_guids in `order` order, so a query's
       object_id maps straight back to order[id] → guid. objects_in(OBB) → guids is the one call
       picking (42, ray sliver) and box-select (45, marquee) narrow from; 37's frustum cull stays a
       linear scan and doesn't need it. A #[cfg(test)] proves the tree's set equals brute force.
       Zero visual change — infrastructure for the lessons that query it.
```

Edited: `app/scene.rs` (`world_obb()` world-box-per-object, `bvh: SpatialBVH` field,
`Scene::build_bvh`, `objects_in(&OBB) -> Vec<&str>`, `#[cfg(test)]` parity).

## Next

`37-frustum-culling.md` — extract the view frustum's 6 planes from `view_proj` (Gribb–Hartmann, f64),
rebase them to world (the camera-relative `view_proj` and 36's world boxes are in different frames),
plane-test every object's world AABB, and set `FLAG_CULLED` on everything off-screen. The shader
collapses a culled instance to a degenerate vertex, so the whole arena still draws in **one** call —
and the perf HUD's drawn/total split finally moves.
