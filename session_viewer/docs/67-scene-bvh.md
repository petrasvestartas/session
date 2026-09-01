# 67 Scene BVH — one broad-phase, three consumers

> **Big picture.** *Phase 5 — acceleration, built BEFORE the features that need it.* Picking (70) and
> box-select (73) each ask "which objects are in this region?" against 744,000 objects, per click or
> per drag. Every real CAD app answers that with a spatial index built once and queried everywhere.
> Building it now — on 35's stable global row order, fed by 62's `all_objects()` so every
> geometry type (curves, surfaces, BReps, trims included) gets its box arm here — makes the later lessons *queries*
> instead of rewrites.

`Scene` (35) has a fixed, ordered object list — global rows appended per document by `add_file`.
Two lessons ahead need the same question answered fast: **which objects fall inside this box?** —
picking (70: box = a thin sliver around the ray) and box-select (73: box = the drag rectangle's
sub-frustum). Both do a *per-object* test (ray↔triangle, point-in-frustum) that is far too
expensive to run N times, so they can't afford to scan all ten sheets' objects. So `Scene` gains
one spatial index — an AABB **BVH** — and they query it for a short candidate list instead.
(Frustum culling, 68, turns out to ship a linear scan — it must touch every object's flag anyway —
so it *doesn't* need the tree; the same index could still accelerate it at extreme scale. Building
it once, here, means all of them share it.)

The kernel already ships the tree: `session_rust::SpatialBVH` (a Morton-code LBVH: radix-sort the
boxes' Morton codes, one linear build pass — Karras 2012; `build_from_aabbs` / `query_aabb` /
`ray_cast`). The roadmap's rule is *don't rewrite what exists* — so this lesson wires
that up, it doesn't reimplement a BVH. The real work is one subtlety the kernel can't do for us.

## Why the viewer builds its own boxes

Since the Xform refactor, **no geometry carries a placement**: an object's stored coordinates are
document-local, and its full world frame is `manifest place × session world xform` — which
`add_file` already composes and stores as the row's instance model, `tables.obj.rows[row].model`. The
kernel's own box routine (`Session::compute_bounding_box(geometry, xform)`) takes that xform as an
argument precisely because the geometry can't supply it — but it is private, per-Session, and knows
nothing about the manifest `place`. The viewer's BVH spans ALL documents in ONE world frame, so
`Scene` builds each box itself from the same placed frame the instance row draws with. That is the
invariant: **every box fed to the tree is a world box, computed from `tables.obj.rows[row].model`.**
(A version of that composition already exists on the engine side: `InstanceTable::append` in
`engine/gpu/objects.rs` puts each row's LOCAL `Row.bounds` through its model matrix into
`bounds_world`, and lists the rows that have one in `bounded_rows`. It covers only the rows the
solid lane's facing cull needs, it is private to the engine, and Step 1 explains why widening it
is a three-part edit this lesson does not make.)

(Browsing `session.rs` you will also find `Session.bvh` and a `cached_ray_bvh` behind a dirty
flag — the archive viewer (`session_viewer_archive`) leaned on exactly that cache, calling
`invalidate_bvh_cache()` after every edit. That was enough there because the archive was a
single-document app. This viewer is multi-document by design: one tree per session would mean
querying N trees and merging, each blind to its manifest `place`. One scene-level tree, placed
boxes in, is the simpler contract — and the per-session caches stay untouched for lesson 70's
narrow-phase `ray_cast`.)

This lesson also names that frame once and for all, because half the remaining course needs it.
Two helpers, both landing in `src/app/query.rs`, the read surface Step 3 creates — quoted here
for what they mean, typed there:

```rust
    pub fn placed_frame(&self, row: u32) -> Xform;   // the row's WORLD frame, OWNED
    pub fn doc_of_row(&self, row: u32) -> usize;     // which document a global row came from
```

`placed_frame` is `Xform::from_matrix(tables.obj.rows[row].model)` — the row already holds the
composed matrix, and `Xform`'s fields are not all public, so the frame is MINTED from it rather
than struct-literalled. Picking and snapping invert THIS to go world → document-local; nothing
ever asks the geometry for its placement again. `doc_of_row` binary-searches `doc_rows`, the
per-doc row starts Step 2 adds — rows are appended per file, so each document owns one contiguous
range. One guard to know about: its `Err(i) => i - 1` underflows on an EMPTY `doc_rows` — a query
before the first `add_file` — which no caller can hit today, since rows only exist once a doc
pushed its first row. Make it `i.saturating_sub(1)` if that ever changes.

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
  <g fill="#d7dae0"><text x="452" y="49">47 picking — box = ray sliver</text><text x="452" y="77">50 box-select — box = drag frustum</text><text x="452" y="105" fill="#888">(41 frustum cull — linear; tree optional)</text></g>
  <text x="10" y="140" fill="#666">one tree, appended per doc; picking &amp; box-select query it instead of scanning N objects.</text>
  <defs><marker id="ah36" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/spatial.rs   # NEW — the boxes and the tree: world_obb, rebuild_bvh, shapecast/Hit3, the parity test
src/app/query.rs     # NEW — the read surface: placed_frame(), doc_of_row(), objects_in(query)
src/app/scene.rs     # four fields, two use lines, three inserts in add_file
src/app/mod.rs       # the two `pub mod` lines that compile the new files at all
src/app/reconcile.rs # 64's commit: the touched-rows box re-walk, and one rebuild
```

Still document state: `spatial.rs` and `query.rs` are two more `impl Scene` files, sitting beside
`docs`/`order`/`guid_to_row`, and nothing in `engine/` learns the TREE exists (the 35 litmus test
still holds). Nothing under `engine/` is touched at all — Step 1 says why that is a decision and
not an oversight.

## Step 1 — a world box per object: `src/app/spatial.rs`

Two routes lead to the same box. This lesson takes the second, and the first is worth knowing for
what it would cost.

**The `Row.bounds` route.** Every producer under `app/walk/` returns a `Row`, and `Row.bounds` is
an AABB in MESH-LOCAL coordinates. `walk/mesh.rs` has filled it since 35 because the facing cull
reads it; the six other kinds still answer `Row::none()` (from `walk_geometry`'s arms in
`walk/mod.rs` — a linework arm builds its row inline rather than in its type file).
`InstanceTable::append` (`engine/gpu/objects.rs`) already lifts whatever is there through the row's
model matrix into `bounds_world`, so filling those in LOOKS like six one-line edits. It is not,
because two engine fields are DERIVED from that same box and both move the frame the moment a row
gains one: `bounded_rows` (the rows `update_inside_flags` walks — a newly bounded curve row starts
taking the facing cull's eye test) and `Instance.extent` (the ink lift's size, `0.0` today for
every unbounded row). The honest version of that route is three edits at once — the six arms, a
predicate narrowing `bounded_rows` back to the solid rows the cull wants, and a gate keeping
`extent` on the rows that had a box before — and any part of it alone is a silent ink change with
no lesson attached.

**The `world_obb` route**, which is what the tree is fed from here. The BVH wants a WORLD box per
row; `Row.bounds` is local, and the frame that lifts it — `tables.obj.rows[row].model` — is the
same one either way. A single free function over `Geometry` produces that box directly, costs the
walk nothing because it does not run in the walk, and leaves the engine's derived fields untouched.
The arms below are the map, one per kernel kind; 35's walk gives every one of the eleven types a
row, so every one needs a box.

**Create `src/app/spatial.rs`**

```rust
//! `spatial.rs` - the scene's broad-phase: one world box per object, one kernel BVH over them.
//!
//! Document state, like `scene.rs` and `query.rs`: nothing in `engine/` learns the tree exists.

use session_rust::element::ElementGeometry;
use session_rust::spatial_bvh::SpatialBVHNode;
use session_rust::{AABB, Geometry, OBB, Point, SpatialBVH, Xform};

use super::scene::Scene;
use super::walk::frames::PLANE_SIZE;

/// The object's WORLD box: local box x the row's placed frame. One rule for every kind -
/// no geometry carries a placement anymore, the row does.
pub(super) fn world_obb(geom: &Geometry, placed: &Xform) -> OBB {
    // planar meshes/sheets give a zero-thickness box; a hair of pad keeps intersects robust
    const PAD: f64 = 1e-6;
    let local = match geom {
        Geometry::Mesh(m) => OBB::from_mesh(m, PAD),
        // b.mesh() re-tessellates - a SECOND mesh per BRep per append (the walk built its own).
        // Once per load, never per query; revisit only if BReps ever number in the thousands.
        Geometry::BRep(b) => OBB::from_mesh(&b.mesh(), PAD),
        Geometry::Line(l) => OBB::from_line(l, PAD),
        Geometry::Polyline(pl) => OBB::from_polyline(pl, PAD),
        Geometry::NurbsCurve(c) => OBB::from_nurbscurve(c, PAD, true),
        Geometry::Point(p) => OBB::from_point((**p).clone(), PAD),
        // CV hull contains the surface (the NURBS convex-hull property): conservative, and no
        // SECOND tessellation - the box does not need the mesh the walk built
        Geometry::NurbsSurface(s) => OBB::from_nurbssurface(s, PAD),
        // exactly the square the walk draws (from_plane takes FULL extents, halves them)
        Geometry::Plane(p) => OBB::from_plane(p, 2.0 * PLANE_SIZE, 2.0 * PLANE_SIZE, 2.0 * PAD),
        // the box IS the geometry: its 8 corners, padded - same corners the 12 edges drew
        Geometry::OBB(b) => OBB::from_points(&b.corners(), PAD),
        Geometry::PointCloud(pc) => OBB::from_pointcloud(pc, PAD),
        // an element boxes as its baked geometry, exactly as the walk drew it
        Geometry::Element(e) => match e.geometry() {
            ElementGeometry::Mesh(m) => OBB::from_mesh(m, PAD),
            ElementGeometry::BRep(b) => OBB::from_mesh(&b.mesh(), PAD),
            // unreachable: add_file `continue`d the empty element - no row, so no box
            ElementGeometry::None => OBB::from_point(Point::new(0.0, 0.0, 0.0), PAD),
        },
        // NO wildcard, same rule as add_file's match: a 12th kernel type must not compile
    };
    local.transformed(placed)
}
```

`AABB` and `SpatialBVHNode` are imported for the two blocks Steps 2 and 3b append to this file;
`rustc` will say they are unused until then. Every `OBB::from_*` above is axis-aligned — they all
route through `AABB` and `OBB::from_aabb` in the kernel — so the only rotation in the result is
the placed frame's, applied last by `transformed`.

`query_aabb` collapses any OBB back to its enclosing AABB internally, so a tilted placed box just
yields a slightly looser world AABB — conservative, never a false miss. Broad-phase wants exactly
that.

## Step 2 — boxes append with the rows: `src/app/scene.rs` + `src/app/spatial.rs`

`build_from_aabbs` keeps its input order: the `object_id` a query returns is the **index into the
slice you passed** (the Morton sort shuffles only the tree's internal layout — each leaf keeps its
pre-sort index; `build_leaf_aabbs` in `spatial_bvh.rs` is the receipt). Feed it boxes in global
row order and the mapping back is just `self.order[id]` — object_id IS the row.

**2a. The fields.** Four of them, and `world_boxes` is the one that needs a defence: it is a
second copy of what `InstanceTable::bounds_world` (`engine/gpu/objects.rs`) keeps for the rows that
have a local box. That table is engine-side, private, and filled only for the solid lane, and
reaching into it from `Scene` would put a `&Gpu` in the signature of every query the rest of the
course writes. Row-indexed extents on `Scene` keep the read surface free of the engine; 48 bytes a
row is the price. `bvh` + `bvh_dirty` together are the INDEX — the tree, and whether it still fits
the rows — and `doc_rows` is what `doc_of_row` binary-searches. All four are `pub(super)`, because
`spatial.rs` and `query.rs` are SIBLING modules of `scene.rs`: a plain private field would be
invisible to exactly the two files that read it.

**Find** in `src/app/scene.rs` — the struct's last field and its closing brace:

```rust
    pub hidden: HashSet<String>,
}
```

**Replace with**:

```rust
    pub hidden: HashSet<String>,
    pub(super) bvh: SpatialBVH,                          // broad-phase; object_id == row
    pub(super) bvh_dirty: bool,                          // true = rows changed since the last build
    pub(super) world_boxes: Vec<([f64; 3], [f64; 3])>,   // one world AABB per row, extents only
    pub(super) doc_rows: Vec<u32>,                       // each doc's first row (doc_of_row's index)
}
```

Every field needs a value, so `Scene::new`'s `Self { … }` literal takes the same four.

**Find** in `src/app/scene.rs`:

```rust
        hidden: HashSet::new(),
```

**Replace with**:

```rust
        hidden: HashSet::new(),
        bvh: SpatialBVH::new(),
        bvh_dirty: false,
        world_boxes: Vec::new(),
        doc_rows: Vec::new(),
```

Two names arrive with them: `SpatialBVH`, the kernel type the field is declared as, and
`world_obb`, Step 1's function, which `add_file` is about to call.

**Find** in `src/app/scene.rs`:

```rust
use super::walk::{WalkCx, walk_geometry};
```

**Add below it**:

```rust
use super::spatial::world_obb;
use session_rust::SpatialBVH;
```

And the two new files have to be declared, or nothing in them is compiled at all.

**Find** in `src/app/mod.rs`:

```rust
pub mod walk;
```

**Add above it**:

```rust
pub mod query;
pub mod spatial;
```

**2b. Boxes append with the rows.** Two inserts in `add_file`. The first records where this
document's rows start, before a single row is pushed.

**Find** in `src/app/scene.rs` — the first of the six `let …0 = …len();` base lines:

```rust
        let seg0 = self.tables.seg.ribbons.len();
```

**Add above it**:

```rust
        self.doc_rows.push(self.tables.obj.rows.len() as u32);
```

The second is the box itself, at the end of the walk loop. One Rust trap: `placed` LOOKS in scope,
but the row push above the match **moved** it into `t.obj.rows.push(ObjectBase { model: placed, … })`
— so read the frame back out of the row it now lives in. Which is the invariant made literal: the
box is computed from the very matrix the row draws with, because it is the SAME value, not a copy
that could drift.

**Find** in `src/app/scene.rs` — the two bookkeeping lines that close the walk loop:

```rust
            self.guid_to_row.insert(guid.clone(), ri);
            self.order.push(guid);
```

**Add below it**:

```rust
            // Cache the box's AABB extents row-by-row. Computing a world box walks the object's
            // VERTICES - do it once per append, never per query. 68's per-frame cull and 73's
            // marquee read THIS cache; without it they'd re-walk every mesh every frame.
            // (`placed` was moved into the objects push above - the ROW owns the frame now.)
            let a = world_obb(geom, &Xform::from_matrix(t.obj.rows[ri as usize].model)).aabb();
            let (lo, hi) = (a.min_point(), a.max_point());
            self.world_boxes.push(([lo[0], lo[1], lo[2]], [hi[0], hi[1], hi[2]]));
```

**2c. Mark the tree stale at the end of `add_file`.**

**Find** in `src/app/scene.rs` — the last lines of `add_file`, the display-only release and the
doc push:

```rust
        let session = if display_only { Session::new(&name) } else { session };
        self.docs.push(Doc {
            name,
            place,
            session,
            cloud_px,
            display_only,
        });
```

**Replace with** — one line between them:

```rust
        let session = if display_only { Session::new(&name) } else { session };
        self.bvh_dirty = true;
        self.docs.push(Doc {
            name,
            place,
            session,
            cloud_px,
            display_only,
        });
```

A flag, not a build — deliberately. Nothing queries the tree DURING a load, so building per
append does ten throwaway builds of growing size and only the last one is ever used. Deferring
to the first query (Step 3) makes a ten-file load pay for exactly ONE build. Measured at this
lesson's own scale — 744k boxes — a build is ~250 ms; eager-per-append across ten files wastes
over a second of load for nothing.

The build itself goes into `src/app/spatial.rs`, which owns the tree and nothing else. It is an
`impl Scene` block of its own — a type can be implemented across as many blocks and files as the
crate likes, which is the whole reason `Scene`'s methods can be split by JOB instead of piled into
`scene.rs`.

**Append** to `src/app/spatial.rs`:

```rust

impl Scene {
    /// Rebuild the whole tree from the cached extents (LBVH: O(n) after the Morton radix
    /// sort - ~250 ms at 744k boxes). Runs lazily from `objects_in` whenever `bvh_dirty`;
    /// 64's reconcile just sets the flag again after applying a diff - the per-row extents
    /// cache is what keeps that cheap; a true incremental refit stays future work. Boxes go
    /// in ROW order -> object_id == row == index into `order`.
    ///
    /// `build_from_aabbs`, NOT `build_with_guids`: the guid path clones one String per row
    /// into the tree and wraps every AABB in an OBB - measured 3.2x the build time and 42.6 MB
    /// of retained Strings at 744k rows - and the viewer never reads them: a query returns
    /// object_ids, which ARE rows here, and `order[row]` already knows the guid. The `ws`
    /// argument is metadata only (the kernel normalizes Morton codes over the input's own
    /// bounds); pass the extents' own span so the field stays honest.
    pub(super) fn rebuild_bvh(&mut self) {
        let aabbs: Vec<AABB> = self.world_boxes.iter()
            .map(|(lo, hi)| AABB::new(
                (lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5, (lo[2] + hi[2]) * 0.5,
                (hi[0] - lo[0]) * 0.5, (hi[1] - lo[1]) * 0.5, (hi[2] - lo[2]) * 0.5))
            .collect();
        let ws = aabbs.iter().fold(0.0f64, |w, a| {
            w.max(2.0 * (a.cx.abs() + a.hx)).max(2.0 * (a.cy.abs() + a.hy)).max(2.0 * (a.cz.abs() + a.hz))
        });
        self.bvh = SpatialBVH::new();
        self.bvh.build_from_aabbs(&aabbs, ws);   // empty slice -> empty tree; query returns []
        self.bvh_dirty = false;
    }
}
```

(Zero inflate is right here: the pad already went in when `world_obb` filled the cache entry.
No pointer tree is built — the kernel's arena path is flat `(2n-1) × 64 B` nodes, which at
744k rows is ~91 MB; that plus the 36 MB extents cache is the honest price of a broad-phase
at this scale, and both are O(n) with small constants.)

> **The boxes go stale when a row moves.** Any lesson that transforms an object in place —
> 64's reconcile — which already exists: its `commit` marked the hook point, and the step below
> installs it — and later the gumball drags (82–84) must write the row's new world box into
> `world_boxes[row]` and set `bvh_dirty = true` as part of its commit, or picks and marquee
> queries keep seeing the old geometry. The next query rebuilds; the tree and the extents
> cache are only as fresh as the last writer.

## Step 3 — the query every later lesson calls: `src/app/query.rs`

Three methods, all going in the same place: `src/app/query.rs`, the third `impl Scene` file — the
read surface, kept apart from `spatial.rs`'s tree so that the lessons which only ASK questions
(70's pick, 73's marquee, 87's snap, 102's work plane) have one small file to open. Two of them
were printed near the top of this lesson; here they are with a file around them (the `doc_rows`
field they read exists as of Step 2).

**Create `src/app/query.rs`**

```rust
//! `query.rs` - the scene's READ surface: where a row sits, which document it came from, and
//! which rows a box touches. No tree code lives here - that is `spatial.rs`. This file only asks.

use session_rust::{OBB, Xform};

use super::scene::Scene;

impl Scene {
    /// The row's full WORLD placement - manifest place x the session's world xform, exactly
    /// what add_file stored as the instance model. Picking/snapping invert THIS to go
    /// world -> document-local; nothing ever asks the geometry for its placement again.
    /// OWNED, not borrowed: the row stores a bare `[f64; 16]`, and the Xform is minted here.
    pub fn placed_frame(&self, row: u32) -> Xform {
        Xform::from_matrix(self.tables.obj.rows[row as usize].model)
    }

    /// Which document a global row belongs to. Rows are appended per file, so each doc owns one
    /// contiguous range; `doc_rows[i]` is doc i's first row.
    pub fn doc_of_row(&self, row: u32) -> usize {
        match self.doc_rows.binary_search(&row) {
            Ok(i) => i,
            Err(i) => i - 1,
        }
    }
}
```

Then, below them, the query 70/73 build on — those two lessons differ only in the box they pass.

**Find** in `src/app/query.rs`:

```rust
    pub fn doc_of_row(&self, row: u32) -> usize {
        match self.doc_rows.binary_search(&row) {
            Ok(i) => i,
            Err(i) => i - 1,
        }
    }
```

**Add below it**:

```rust

    /// Rows whose world box intersects `query` - the broad-phase. Callers narrow further
    /// (70 does ray-vs-triangle, 73 tests the marquee frustum) on this short list, not all N.
    /// A row maps to its guid via `order[row]` and to its doc via `doc_of_row(row)`.
    /// `&mut self` from day one: the tree builds LAZILY on the first query after any change
    /// (2c's flag), so loads never pay for builds nobody reads. ~8 us per pick-sized query
    /// at 744k boxes once built.
    pub fn objects_in(&mut self, query: &OBB) -> Vec<u32> {
        if self.bvh_dirty {
            self.rebuild_bvh();
        }
        self.bvh.query_aabb(query)
            .into_iter()
            .map(|id| id as u32)   // object_id == row, by construction (Step 2)
            .collect()
    }
```

Rows, not guids: a row is unambiguous across documents (two files CAN carry the same guid), and
it is what the instance table, the flags, and `placed_frame` are all keyed by. The guid is one
`self.order[row]` away when the UI wants a name.

## Step 3b — one visitor, every query: `shapecast`

`objects_in` answers boxes; 70's ray, 73's marquee and 109's measure snaps all want the same
walk with a different test. The kernel tree's nodes are `pub` (`root`, `left`, `right`,
`aabb`, `object_id`) — so ONE generic visitor in the viewer serves them all, and no query
ever writes its own recursion again. It is traversal, not a query, so it lives with the tree.

**Append** to `src/app/spatial.rs`:

```rust

pub enum Hit3 { Miss, Intersects, Contained }

/// The one traversal every spatial query shares. `test` classifies a node's box;
/// `Contained` short-circuits: the WHOLE subtree is accepted without further tests —
/// that one arm is what makes 73's marquee stop caring how many objects the scene has.
pub fn shapecast(node: &SpatialBVHNode, test: &impl Fn(&OBB) -> Hit3,
                 out: &mut impl FnMut(u32)) {
    let Some(b) = &node.aabb else { return };
    match test(b) {
        Hit3::Miss => {}
        Hit3::Contained => accept_subtree(node, out),   // no more tests below here
        Hit3::Intersects => {
            if node.is_leaf() {
                if node.object_id >= 0 { out(node.object_id as u32) }
            } else {
                if let Some(l) = &node.left  { shapecast(l, test, out) }
                if let Some(r) = &node.right { shapecast(r, test, out) }
            }
        }
    }
}

fn accept_subtree(node: &SpatialBVHNode, out: &mut impl FnMut(u32)) {
    if node.is_leaf() { if node.object_id >= 0 { out(node.object_id as u32) } return }
    if let Some(l) = &node.left  { accept_subtree(l, out) }
    if let Some(r) = &node.right { accept_subtree(r, out) }
}
```

`objects_in` could be written as a closure over it and behave identically — it keeps
`query_aabb` because for a plain box the kernel's own traversal IS this walk. What `shapecast`
adds is the arms `query_aabb` has no place for, and the later lessons collect them: 70 walks it
nearest-child-first with an early-out (compare the ray's entry distance to each child box,
recurse the nearer first, stop when the best hit beats the next node), 73 gets the
`Contained` marquee arm — the whole subtree accepted with no further test — and 82 refits node
boxes in place during drags, all through these same `pub` nodes, no kernel changes. (If traversal ever profiles hot at millions of
objects, the cache upgrade is flattening these boxed nodes into a `Vec` of 32-byte
implicit-left-child records — an optimization, not a redesign, BECAUSE every caller goes
through `shapecast`.)

## Step 4 — prove it: `#[cfg(test)]` broad-phase == brute force

A BVH is only trustworthy if it returns *exactly* the brute-force set — no misses, no phantoms.
The test also proves the PLACEMENT half: the same document loaded twice with different manifest
placements must yield disjoint boxes. It goes at the very bottom of `spatial.rs`, below
`rebuild_bvh` and `shapecast`; it needs no GPU, so it runs headless.

**Append** to `src/app/spatial.rs`:

```rust

#[cfg(test)]
mod tests {
    use super::*;   // the file's own imports: Scene, Point, Xform, AABB, OBB
    use session_rust::{Line, Session};

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
        // add_file's last two arguments are the manifest's point-size override and its
        // display_only flag - neither matters here, so 0.0 and false.
        scene.add_file("a".into(), demo_session(), Xform::identity(), 0.0, false);
        // The same file again, pushed 50 m away - placement MUST move its boxes.
        scene.add_file("b".into(), demo_session(), Xform::translation(50_000.0, 0.0, 0.0), 0.0, false);

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

## Streamed clouds are not in this tree

A `CloudSlot` (44) pushes its object row with `object_bounds: None` — deliberately. Its
points live only on the GPU, so there is no per-object geometry to derive a box from at
walk time, and the BVH simply gets no leaf for it. That is consistent with where this
phase is headed: streamed clouds are display objects — unpickable, unselectable — until a
later lesson gives them kernel-side structure. (Their SCENE box still exists — lesson 43's
`grow_bounds` feeds the camera — it is the per-object index they sit out.)

## The curved types join `world_obb`

The geometry block (58–62) shipped four types with no box map to join — this lesson is where that
map is born, so their arms are steps HERE, not there, and Step 1 already typed them. They are the
interesting three, quoted here from Step 1's map because of what they choose:

```rust
        Geometry::NurbsCurve(c) => OBB::from_nurbscurve(c, PAD, true),
        Geometry::NurbsSurface(s) => OBB::from_nurbssurface(s, PAD),
        Geometry::BRep(b) => OBB::from_mesh(&b.mesh(), PAD),
```

A curve and a surface take the kernel ctor, which reads the CV net exactly — the control hull
contains the geometry, so the box is conservative and warms no cache. The BRep arm is the one with
an alternative: `b.mesh()` re-tessellates, and 61's walk already keeps a `(mesh, linework)` entry
per BRep, so a `&self` route could box the CACHED mesh instead and save the second tessellation —
and 62's trimmed surfaces would take that same route, their kernel sampler being trim-blind while
the cached mesh is what is actually on screen. `world_obb` is a free function precisely so it needs
no cache to be correct; swapping in the cached mesh is a later optimisation, and it only pays off
if BReps ever number in the thousands.

## Reconcile grows the hook

64's `commit` marked exactly where this lesson's state joins the reload path — the marker it
planted still says 52, the number this lesson carried before the restructure, so match it as it
is written. `world_obb` needs importing there too: `use super::spatial::world_obb;`.

**Find** in `src/app/reconcile.rs` — inside `commit` (64):

```rust
        // (52's hook lands here: touched-rows box re-walk + rebuild_bvh.)
```

**Replace with** — the extents cache is re-walked ONLY for rows the diff touched (`world_obb`
reads every vertex, so recomputing all N boxes would hand back the cost the diff just saved),
and the tree rebuilds once at the end:

```rust
        // The extents cache stays ROW-indexed at the rows reload kept. Unchanged rows keep
        // their cached box; a freed row keeps a degenerate box no live guid points at; resize
        // extends the cache for rows this reload added beyond the old high-water.
        let place = self.docs.first()
            .map(|d| d.place.duplicate()).unwrap_or_else(Xform::identity);
        self.world_boxes.resize(self.next_row as usize, ([0.0; 3], [0.0; 3]));
        for guid in diff.changed.iter().chain(&diff.added) {
            let row = self.guid_to_row[guid];
            let placed = &place * &world.get(guid).cloned().unwrap_or_else(Xform::identity);
            let a = world_obb(&new.lookup[guid], &placed).aabb();
            let (lo, hi) = (a.min_point(), a.max_point());
            self.world_boxes[row as usize] = ([lo[0], lo[1], lo[2]], [hi[0], hi[1], hi[2]]);
        }
        self.rebuild_bvh();
```

And note what reconcile does NOT break — by construction. `world_boxes` is ROW-indexed and
the BVH's leaf ids equal positions in it, so `id == row` survives a reload untouched: 64
keeps rows stable, and the hook above rewrites boxes AT their rows. The archive's variant
zipped `world_boxes` with `order` here and needed a post-reconcile repair; this design
doesn't. The one identity that DOES die is the reverse translation `self.order[row]`
(`objects_in`'s closing comment leans on it for names): after a reload, `order` is rebuilt
while rows persist, so a UI that wants a row's guid must invert `guid_to_row` (or keep a
`row_to_guid` map maintained beside it) — flag it now, pay it when a consumer appears (97's
scene tree is the first).

## What this costs — measured, not estimated

Numbers from a 744k-box synthetic sheet scene (30 m of drawings, mm units), the scale this
lesson's big-picture quotes:

```
build (lazy, once per load)   ~250 ms      LBVH: radix sort + O(n) Karras pass
pick-sized query               ~8 us       1000 queries in 8 ms, exact hits == brute force
arena memory                   (2n-1) x 64 B  ->  ~91 MB      flat nodes, no pointers
extents cache                  n x 48 B        ->  ~36 MB      serves 68's linear cull too
```

Two costs this lesson deliberately does NOT pay:

- **`build_with_guids`** — 3.2x the build time and 42.6 MB of retained `String`s at this
  scale, for a mapping (`object_id -> guid`) the viewer already owns as `order[row]`.
- **Eager rebuilds** — ten appends would trigger ten throwaway builds; the dirty flag defers
  to the first query.

And one the kernel used to pay, fixed while writing this lesson (all three languages,
minitests green): Morton codes were normalized over an origin-centered `world_size`, so a
scene sitting far from the origin — any georeferenced model — collapsed into a handful of
Morton cells and queries went **660x slower** (5.3 ms instead of 8 us each) with unchanged
results. Codes are now normalized over the input's own bounding CUBE (uniform scale: per-axis
stretch would scatter flat scenes — measured 4x — because a 10 mm z-span blown up to 1024
cells shuffles xy-neighbours apart in the sort). Tree quality is translation-invariant now;
`world_size` remains as serialized metadata.

## Run

```bash
# http://127.0.0.1:8770 — visuals UNCHANGED (nothing draws the BVH yet)
cd session_viewer && trunk serve
cargo test -p session_viewer bvh --target x86_64-unknown-linux-gnu   # .cargo/config.toml pins wasm — override for the headless test
```

Nothing on screen moves this lesson — the tree is pure infrastructure. Lesson 68 is where it earns
its keep: the drawn-object count on the perf HUD drops the moment you zoom in.

## Recap

```
Ch 35: Scene owns docs + tables + order + guid_to_row + hidden; rows are GLOBAL, appended per doc.
Ch 67: Scene gains ONE broad-phase — the kernel's SpatialBVH (reused, not rewritten) — and the two
       helpers half the course leans on: placed_frame(row) = tables.obj.rows[row].model (the
       manifest place × session world xform that add_file stored) and doc_of_row(row) (contiguous
       per-doc row ranges), both in app/query.rs; the tree itself in app/spatial.rs.
       Every box is world_obb(geometry, placed frame) — LOCAL geometry lifted by the row's own
       matrix — one rule, all ELEVEN kernel kinds (35's walk rows them all), no geometry placement
       to consult because none exists. The engine's Row.bounds/bounds_world/bounded_rows/extent
       chain is left EXACTLY as it was: filling Row.bounds for the unbounded kinds would silently
       move the facing cull and the ink lift, and that is three edits in one step, not one.
       Boxes append with the rows in add_file (extents cached once per append), the tree is marked
       dirty per file and built on the first query, and queries return ROWS — unambiguous across
       docs, straight into flags/instances/order.
       objects_in(OBB) → rows is the one call picking (70, ray sliver) and box-select (73,
       marquee) narrow from; 68's frustum cull stays a linear scan over the extents cache.
       The tree builds LAZILY (bvh_dirty, first query after a change) via build_from_aabbs —
       no guid Strings, no OBB wrap, one build per load (~250 ms at 744k; ~8 us/query;
       arena (2n-1)x64 B). Kernel fixed alongside: Morton codes normalize over the input's
       bounding cube, not an origin-centered world_size (660x off-origin query win, x3 langs).
       A #[cfg(test)] proves tree == brute force AND that placement lands in the boxes.
       Zero visual change — infrastructure for the lessons that query it.
```

Edited: `app/spatial.rs` (NEW — `world_obb`, lazy `rebuild_bvh` via `build_from_aabbs`,
`shapecast`/`Hit3`, the `#[cfg(test)]` parity test), `app/query.rs` (NEW — `placed_frame`,
`doc_of_row`, `objects_in(&OBB) -> Vec<u32>`), `app/mod.rs` (two `pub mod` lines),
`app/scene.rs` (four fields, two `use` lines, the `doc_rows` line and the box cache in `add_file`,
the `bvh_dirty` flag at its foot), `app/reconcile.rs` (64's hook: touched-rows box re-walk +
`rebuild_bvh`). Nothing under `engine/` and nothing under `app/walk/` — Step 1 says why.

## Next

`68-frustum-culling.md` — extract the view frustum's 6 planes from the ANCHORED `view_proj`
(Gribb–Hartmann, f64), rebase them by the anchor (the camera-relative matrix and this lesson's
world boxes are in different frames), plane-test every row's cached extents, and set `FLAG_CULLED` on
everything off-screen. The shader collapses a culled instance to a degenerate vertex, so the whole
arena still draws in **one** call — and the perf HUD's drawn/total split finally moves.
