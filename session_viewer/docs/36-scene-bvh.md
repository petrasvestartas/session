# 36 Scene BVH — one broad-phase, three consumers

> **Big picture.** *Phase 5 — acceleration, built BEFORE the features that need it.* Picking (42) and
> box-select (45) each ask "which objects are in this region?" against 744,000 objects, per click or
> per drag. Every real CAD app answers that with a spatial index built once and queried everywhere.
> Building it now, on the stable global row order 35 just created, makes the later lessons *queries*
> instead of rewrites.

`Scene` (35) has a fixed, ordered object list — global rows appended per document by `add_file`.
Two lessons ahead need the same question answered fast: **which objects fall inside this box?** —
picking (42: box = a thin sliver around the ray) and box-select (45: box = the drag rectangle's
sub-frustum). Both do a *per-object* test (ray↔triangle, point-in-frustum) that is far too
expensive to run N times, so they can't afford to scan all ten sheets' objects. So `Scene` gains
one spatial index — an AABB **BVH** — and they query it for a short candidate list instead.
(Frustum culling, 37, turns out to ship a linear scan — it must touch every object's flag anyway —
so it *doesn't* need the tree; the same index could still accelerate it at extreme scale. Building
it once, here, means all of them share it.)

The kernel already ships the tree: `session_rust::SpatialBVH` (a Morton-code LBVH: radix-sort the
boxes' Morton codes, one linear build pass — Karras 2012; `build_with_guids` / `query_aabb` /
`ray_cast`). The roadmap's rule is *don't rewrite what exists* — so this lesson wires
that up, it doesn't reimplement a BVH. The real work is one subtlety the kernel can't do for us.

## Why the viewer builds its own boxes

Since the Xform refactor, **no geometry carries a placement**: an object's stored coordinates are
document-local, and its full world frame is `manifest place × session world xform` — which
`add_file` already composes and stores as the row's instance model, `tables.objects[row].0`. The
kernel's own box routine (`Session::compute_bounding_box(geometry, xform)`) takes that xform as an
argument precisely because the geometry can't supply it — but it is private, per-Session, and knows
nothing about the manifest `place`. The viewer's BVH spans ALL documents in ONE world frame, so
`Scene` builds each box itself from the same placed frame the instance row draws with. That is the
invariant: **every box fed to the tree is a world box, computed from `tables.objects[row].0`.**

(Browsing `session.rs` you will also find `Session.bvh` and a `cached_ray_bvh` behind a dirty
flag — the archive viewer (`session_viewer_archive`) leaned on exactly that cache, calling
`invalidate_bvh_cache()` after every edit. That was enough there because the archive was a
single-document app. This viewer is multi-document by design: one tree per session would mean
querying N trees and merging, each blind to its manifest `place`. One scene-level tree, placed
boxes in, is the simpler contract — and the per-session caches stay untouched for lesson 42's
narrow-phase `ray_cast`.)

This lesson also names that frame once and for all, because half the remaining course needs it:

```rust
    /// The row's full WORLD placement — manifest place × the session's world xform, exactly
    /// what add_file stored as the instance model. Picking/snapping invert THIS to go
    /// world → document-local; nothing ever asks the geometry for its placement again.
    pub fn placed_frame(&self, row: u32) -> &Xform {
        &self.tables.objects[row as usize].0
    }

    /// Which document a global row belongs to. Rows are appended per file, so each doc owns one
    /// contiguous range; `doc_rows[i]` is doc i's first row.
    pub fn doc_of_row(&self, row: u32) -> usize {
        match self.doc_rows.binary_search(&row) {
            Ok(i) => i,
            Err(i) => i - 1,
        }
    }
```

(Read them now, type them in Step 3 — `doc_of_row` reads a `doc_rows` field that Step 2 adds
first.)

<svg viewBox="0 0 680 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="per-row world boxes are built by Scene::add_file from the row's placed frame and fed to the kernel SpatialBVH, whose query_aabb serves picking and box-select" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="10" y="18" fill="#888">Scene::add_file — one world box per row (placed frame applied)</text>
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="14" y="28" width="150" height="26"/>
    <rect x="14" y="60" width="150" height="26"/>
    <rect x="14" y="92" width="150" height="26"/>
  </g>
  <g fill="#d7dae0"><text x="24" y="45">local box × placed frame</text><text x="24" y="77">= tables.objects[row].0</text><text x="24" y="109">rows append per doc</text></g>
  <line x1="164" y1="73" x2="214" y2="73" stroke="#6fb3ff" stroke-width="1.5" marker-end="url(#ah36)"/>
  <rect x="216" y="52" width="150" height="42" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="291" y="70" fill="#d7dae0" text-anchor="middle">SpatialBVH</text>
  <text x="291" y="85" fill="#666" text-anchor="middle">(kernel — reused)</text>
  <text x="291" y="112" fill="#555" text-anchor="middle" font-size="10">query_aabb(OBB) → object_ids == rows</text>
  <g stroke="#6fb3ff" stroke-width="1.3"><line x1="366" y1="73" x2="440" y2="45" marker-end="url(#ah36)"/><line x1="366" y1="73" x2="440" y2="73" marker-end="url(#ah36)"/><line x1="366" y1="73" x2="440" y2="101" marker-end="url(#ah36)"/></g>
  <g fill="none" stroke="#3a3a3a"><rect x="442" y="32" width="220" height="26"/><rect x="442" y="60" width="220" height="26"/><rect x="442" y="88" width="220" height="26"/></g>
  <g fill="#d7dae0"><text x="452" y="49">42 picking — box = ray sliver</text><text x="452" y="77">45 box-select — box = drag frustum</text><text x="452" y="105" fill="#888">(37 frustum cull — linear; tree optional)</text></g>
  <text x="10" y="140" fill="#666">one tree, appended per doc; picking &amp; box-select query it instead of scanning N objects.</text>
  <defs><marker id="ah36" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/scene.rs   # Scene gains bvh + world_boxes + doc_rows; placed_frame()/doc_of_row();
                   # world_obb() per object; boxes appended in add_file; objects_in(query)
```

Just `scene.rs` — the BVH is document state, so it lives beside `docs`/`order`/`guid_to_row`, and
nothing in `engine/` learns it exists (the 35 litmus test still holds).

## Step 1 — a world box per object: `src/app/scene.rs`

Extend 35's document-side import (it already brought `Session`/`Geometry`/`OBB`/`Point`/…):

```rust
use session_rust::{AABB, SpatialBVH};   // ← ADD to the existing session_rust use (OBB is
                                        //   already there — 35's box converter needed it)
```

Add a free function next to 35's converters. Since the Xform refactor EVERY object's stored
coordinates are local, so the rule is uniform: build the LOCAL box, then transform it by the
row's placed frame — the same matrix the instance row draws with. One arm per kernel kind:
35's walk gives every one of the 11 types a row now, so every one needs a box:

```rust
/// The object's WORLD box: local box × the row's placed frame. One rule for every kind —
/// no geometry carries a placement anymore, the row does.
fn world_obb(geom: &Geometry, placed: &Xform) -> OBB {
    // planar meshes/sheets give a zero-thickness box; a hair of pad keeps intersects robust
    const PAD: f64 = 1e-6;
    let local = match geom {
        Geometry::Mesh(m) => OBB::from_aabb(AABB::from_mesh(m, PAD)),
        // b.mesh() re-tessellates — a SECOND mesh per BRep per append (the walk built its own).
        // Once per load, never per query; revisit only if BReps ever number in the thousands.
        Geometry::BRep(b) => OBB::from_aabb(AABB::from_mesh(&b.mesh(), PAD)),
        Geometry::Line(l) => OBB::from_line(l, PAD),
        Geometry::Polyline(pl) => OBB::from_polyline(pl, PAD),
        Geometry::NurbsCurve(c) => OBB::from_nurbscurve(c, PAD, true),
        Geometry::Point(p) => OBB::from_point((**p).clone(), PAD),
        // CV hull ⊇ surface (the NURBS convex-hull property): conservative, and no SECOND
        // tessellation — the box does not need the mesh the walk built
        Geometry::NurbsSurface(s) => OBB::from_nurbssurface(s, PAD),
        // exactly the square the walk draws (from_plane takes FULL extents, halves them)
        Geometry::Plane(p) => OBB::from_plane(p, 2.0 * PLANE_SIZE, 2.0 * PLANE_SIZE, 2.0 * PAD),
        // the box IS the geometry: its 8 corners, padded — same corners the 12 edges drew
        Geometry::OBB(b) => OBB::from_points(&b.corners(), PAD),
        Geometry::PointCloud(pc) => OBB::from_pointcloud(pc, PAD),
        // an element boxes as its baked geometry, exactly as the walk drew it
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(m) => OBB::from_aabb(AABB::from_mesh(m, PAD)),
            ElementGeometry::BRep(b) => OBB::from_aabb(AABB::from_mesh(&b.mesh(), PAD)),
            // unreachable: add_file `continue`d the empty element — no row, so no box
            ElementGeometry::None => OBB::from_point(Point::new(0.0, 0.0, 0.0), PAD),
        },
        // NO wildcard, same rule as add_file's match: a 12th kernel type must not compile
    };
    local.transformed(placed)
}
```

`query_aabb` collapses any OBB back to its enclosing AABB internally, so a tilted placed box just
yields a slightly looser world AABB — conservative, never a false miss. Broad-phase wants exactly
that.

## Step 2 — boxes append with the rows: `src/app/scene.rs`

`build_with_guids` keeps its input order: the `object_id` a query returns is the **index into the
slice you passed** (the Morton sort shuffles only the tree's internal layout — each leaf keeps its
pre-sort index; `build_leaf_aabbs` in `spatial_bvh.rs` is the receipt). Feed it boxes in global
row order and the mapping back is just `self.order[id]` — object_id IS the row.

**2a. Add the fields** to `struct Scene` (below `hidden`):

```rust
    bvh: SpatialBVH,                          // broad-phase over world boxes, object_id == row
    world_boxes: Vec<([f64; 3], [f64; 3])>,   // cached AABB extents, one per row
    doc_rows: Vec<u32>,                       // each doc's first row (doc_of_row's index)
```

and to `Scene::new`'s literal: `bvh: SpatialBVH::new(), world_boxes: Vec::new(),
doc_rows: Vec::new()`.

**2b. Append per row in `add_file`.** At the TOP of `add_file` (before the walk), record the doc's
first row:

```rust
        self.doc_rows.push(self.tables.objects.len() as u32);
```

Then in the walk loop, right after the two bookkeeping lines

```rust
            self.guid_to_row.insert(guid.clone(), ri);
            self.order.push(guid);
```

append the row's box. One Rust trap: `placed` LOOKS in scope, but `Xform` is not `Copy`, and the
row push above the match **moved** it into `t.objects.push((placed, …))` — so borrow the frame back out of the
row it now lives in. Which is the invariant made literal: the box is computed from the very
matrix the row draws with, because it is the SAME value, not a copy that could drift:

```rust
            // Cache the box's AABB extents row-by-row. Computing a world box walks the object's
            // VERTICES — do it once per append, never per query. 37's per-frame cull and 45's
            // marquee read THIS cache; without it they'd re-walk every mesh every frame.
            // (`placed` was moved into the objects push above — the ROW owns the frame now.)
            let a = world_obb(geom, &t.objects[ri as usize].0).aabb();
            let (lo, hi) = (a.min_point(), a.max_point());
            self.world_boxes.push(([lo[0], lo[1], lo[2]], [hi[0], hi[1], hi[2]]));
```

**2c. Rebuild the tree at the end of `add_file`** (after the planar block, before
`self.docs.push(...)`). The build is a radix sort of Morton codes plus one linear pass —
milliseconds over cached extents; ten appends cost ten rebuilds, which is nothing next to the
walk that preceded each:

```rust
        self.rebuild_bvh();
```

```rust
    /// Rebuild the whole tree from the cached extents — called once per appended file (LBVH
    /// build: O(n) after the Morton radix sort). A later lesson (38) refits incrementally on
    /// edit instead. Boxes go in ROW order → object_id == row == index into `order`.
    fn rebuild_bvh(&mut self) {
        let boxes: Vec<(OBB, String)> = self.world_boxes.iter().zip(&self.order)
            .map(|((lo, hi), guid)| {
                let aabb = AABB::from_points(&[Point::new(lo[0], lo[1], lo[2]),
                                               Point::new(hi[0], hi[1], hi[2])], 0.0);
                (OBB::from_aabb(aabb), guid.clone())
            })
            .collect();
        self.bvh = SpatialBVH::new();
        self.bvh.build_with_guids(&boxes);   // empty slice → empty tree; query returns []
    }
```

(The kernel has no min/max constructor — `AABB::from_points` over the two corner points builds
the same box. Zero inflate is right here: the pad already went in when `world_obb` filled the
cache entry.)

## Step 3 — the query every later lesson calls: `src/app/scene.rs`

Three additions to `impl Scene`. First type in the two helpers from the top of the lesson —
`placed_frame` and `doc_of_row` (the `doc_rows` field they read exists as of Step 2). Then the
query 42/45 build on — they differ only in the box they pass:

```rust
    /// Rows whose world box intersects `query` — the broad-phase. Callers narrow further
    /// (42 does ray↔triangle, 45 tests the marquee frustum) on this short list, not all N.
    /// A row maps to its guid via `order[row]` and to its doc via `doc_of_row(row)`.
    pub fn objects_in(&self, query: &OBB) -> Vec<u32> {
        self.bvh.query_aabb(query)
            .into_iter()
            .map(|id| id as u32)   // object_id == row, by construction (Step 2)
            .collect()
    }
```

Rows, not guids: a row is unambiguous across documents (two files CAN carry the same guid), and
it is what the instance table, the flags, and `placed_frame` are all keyed by. The guid is one
`self.order[row]` away when the UI wants a name.

## Step 4 — prove it: `#[cfg(test)]` broad-phase == brute force

A BVH is only trustworthy if it returns *exactly* the brute-force set — no misses, no phantoms.
The test also proves the PLACEMENT half: the same document loaded twice with different manifest
placements must yield disjoint boxes. Add at the bottom of `scene.rs`; it needs no GPU, so it runs
headless:

```rust
#[cfg(test)]
mod tests {
    use super::*;   // 35's scene.rs imports already carry Session, Line, Point, Xform

    /// A tiny Session at known, separated positions. `add_line` returns a tree node we ignore;
    /// `None` = no parent.
    fn demo_session() -> Session {
        let mut s = Session::new("bvh_test");
        s.add_line(Line::from_points(&Point::new(0.0, 0.0, 0.0), &Point::new(100.0, 0.0, 0.0)), None);
        s.add_line(Line::from_points(&Point::new(300.0, 300.0, 0.0), &Point::new(400.0, 300.0, 0.0)), None);
        s.add_line(Line::from_points(&Point::new(9000.0, 9000.0, 0.0), &Point::new(9100.0, 9000.0, 0.0)), None);
        s
    }

    #[test]
    fn bvh_matches_brute_force() {
        let mut scene = Scene::new();
        scene.add_file("a".into(), demo_session(), Xform::identity());
        // The same file again, pushed 50 m away — placement MUST move its boxes.
        scene.add_file("b".into(), demo_session(), Xform::translation(50_000.0, 0.0, 0.0));

        // AABB::new is CENTER + HALF-EXTENTS (not min/max): this box spans -500..500 on each
        // axis, so it catches doc a's two near-origin lines and nothing from doc b.
        let query = OBB::from_aabb(AABB::new(0.0, 0.0, 0.0, 500.0, 500.0, 500.0));

        let mut got: Vec<u32> = scene.objects_in(&query);
        let mut want: Vec<u32> = (0..scene.world_boxes.len() as u32)
            .filter(|&row| {
                let (lo, hi) = scene.world_boxes[row as usize];
                let aabb = AABB::from_points(&[Point::new(lo[0], lo[1], lo[2]),
                                               Point::new(hi[0], hi[1], hi[2])], 0.0);
                scene.bvh.aabb_intersect(&OBB::from_aabb(aabb), &query)
            })
            .collect();
        got.sort(); want.sort();
        assert_eq!(got, want, "BVH broad-phase must equal the linear scan");
        assert_eq!(got.len(), 2, "doc b's copies sit 50 m away — placement is in the boxes");
        assert_eq!(scene.doc_of_row(got[0]), 0);
    }
}
```

`SpatialBVH::aabb_intersect(&OBB, &OBB)` is the kernel's own overlap primitive, with one honesty
note: it reads each OBB as center ± half-extents and never looks at its axes, so it is exact only
for axis-aligned boxes. Here that holds by construction — the cache stores AABB extents and the
query came from `OBB::from_aabb` — and on that domain it is the same slab test `query_aabb` runs
per node. (A ROTATED query would differ: `query_aabb` first widens it to its enclosing AABB,
`aabb_intersect` would not — keep marquee/sliver queries axis-aligned or pre-collapse them with
`.aabb()`.) So the brute-force scan and the tree are judged by an identical predicate; once
green, every downstream query trusts the broad-phase.

## Run

```bash
# http://127.0.0.1:8770 — visuals UNCHANGED (nothing draws the BVH yet)
cd session_viewer && trunk serve
cargo test -p session_viewer bvh --target x86_64-unknown-linux-gnu   # .cargo/config.toml pins wasm — override for the headless test
```

Nothing on screen moves this lesson — the tree is pure infrastructure. Lesson 37 is where it earns
its keep: the drawn-object count on the perf HUD drops the moment you zoom in.

## Recap

```
Ch 35: Scene owns docs + tables + order + guid_to_row + hidden; rows are GLOBAL, appended per doc.
Ch 36: Scene gains ONE broad-phase — the kernel's SpatialBVH (reused, not rewritten) — and the two
       helpers half the course leans on: placed_frame(row) = tables.objects[row].0 (the manifest
       place × session world xform that add_file stored) and doc_of_row(row) (contiguous per-doc
       row ranges). Every box is LOCAL geometry × placed frame — one rule, all ELEVEN kernel
       kinds (35's walk rows them all), no geometry placement to consult because none exists. Boxes append with the rows in
       add_file (extents cached once per append), the tree rebuilds per file (milliseconds), and
       queries return ROWS — unambiguous across docs, straight into flags/instances/order.
       objects_in(OBB) → rows is the one call picking (42, ray sliver) and box-select (45,
       marquee) narrow from; 37's frustum cull stays a linear scan over the extents cache. A
       #[cfg(test)] proves tree == brute force AND that placement lands in the boxes.
       Zero visual change — infrastructure for the lessons that query it.
```

Edited: `app/scene.rs` (`placed_frame`/`doc_of_row`, `world_obb(geom, placed)`, `bvh` +
`world_boxes` + `doc_rows` fields, per-row box append in `add_file`, `rebuild_bvh`,
`objects_in(&OBB) -> Vec<u32>`, `#[cfg(test)]` parity).

## Next

`37-frustum-culling.md` — extract the view frustum's 6 planes from the ANCHORED `view_proj`
(Gribb–Hartmann, f64), rebase them by the anchor (the camera-relative matrix and 36's world boxes
are in different frames), plane-test every row's cached extents, and set `FLAG_CULLED` on
everything off-screen. The shader collapses a culled instance to a degenerate vertex, so the whole
arena still draws in **one** call — and the perf HUD's drawn/total split finally moves.
