# 67 Scene BVH — one broad-phase, three consumers

> **Big picture.** *Phase 5 — acceleration, built BEFORE the features that need it.* Picking (55) and
> box-select (58) each ask "which objects are in this region?" against 744,000 objects, per click or
> per drag. Every real CAD app answers that with a spatial index built once and queried everywhere.
> Building it now — on 35's stable global row order, fed by 47's `all_objects()` so every
> geometry type (curves, surfaces, BReps, trims included) gets its box arm here — makes the later lessons *queries*
> instead of rewrites.

`Scene` (35) has a fixed, ordered object list — global rows appended per document by `add_file`.
Two lessons ahead need the same question answered fast: **which objects fall inside this box?** —
picking (55: box = a thin sliver around the ray) and box-select (58: box = the drag rectangle's
sub-frustum). Both do a *per-object* test (ray↔triangle, point-in-frustum) that is far too
expensive to run N times, so they can't afford to scan all ten sheets' objects. So `Scene` gains
one spatial index — an AABB **BVH** — and they query it for a short candidate list instead.
(Frustum culling, 41, turns out to ship a linear scan — it must touch every object's flag anyway —
so it *doesn't* need the tree; the same index could still accelerate it at extreme scale. Building
it once, here, means all of them share it.)

The kernel already ships the tree: `session_rust::SpatialBVH` (a Morton-code LBVH: radix-sort the
boxes' Morton codes, one linear build pass — Karras 2012; `build_from_aabbs` / `query_aabb` /
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
boxes in, is the simpler contract — and the per-session caches stay untouched for lesson 70's
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
first. One guard to know about: `Err(i) => i - 1` underflows on an EMPTY `doc_rows` — a query
before the first `add_file` — which no caller can hit today, since rows only exist once a doc
pushed its first row. Make it `i.saturating_sub(1)` if that ever changes.)

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
src/app/scene.rs   # Scene gains bvh + world_boxes + doc_rows; placed_frame()/doc_of_row();
                   # world_obb() per object; boxes appended in add_file; objects_in(query)
```

Just `scene.rs` — the BVH is document state, so it lives beside `docs`/`order`/`guid_to_row`, and
nothing in `engine/` learns it exists (the 35 litmus test still holds).

## Step 1 — a world box per object: `src/app/scene.rs`

Find 35's document-side import — the long `use session_rust::{…}` line in the middle of the file
(the one that already brought `Session`, `Geometry`, `OBB`, `Point`, …) — and add two names to
its braces, `AABB` and `SpatialBVH`. Nothing is removed; `OBB` is already there because 35's box
converter needed it.

Then go to the very BOTTOM of `scene.rs` — below `pointcloud_to_glyphs`, the last of 35's
converters — and add this free function there. Since the Xform refactor EVERY object's stored
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

`build_from_aabbs` keeps its input order: the `object_id` a query returns is the **index into the
slice you passed** (the Morton sort shuffles only the tree's internal layout — each leaf keeps its
pre-sort index; `build_leaf_aabbs` in `spatial_bvh.rs` is the receipt). Feed it boxes in global
row order and the mapping back is just `self.order[id]` — object_id IS the row.

**2a. Add the fields.** In `pub struct Scene`, find its last field and the closing brace:

```rust
    pub hidden: HashSet<String>,
}
```

and insert three fields above that brace:

```rust
    pub hidden: HashSet<String>,
    bvh: SpatialBVH,                          // broad-phase over world boxes, object_id == row
    bvh_dirty: bool,                          // true = rows changed since the last build
    world_boxes: Vec<([f64; 3], [f64; 3])>,   // cached AABB extents, one per row
    doc_rows: Vec<u32>,                       // each doc's first row (doc_of_row's index)
}
```

Every field needs a value, so do the same to `Scene::new`'s `Self { … }` literal — find its
`hidden` line and add three below it:

```rust
            hidden: HashSet::new(),
            bvh: SpatialBVH::new(),
            bvh_dirty: false,
            world_boxes: Vec::new(),
            doc_rows: Vec::new(),
```

**2b. Append per row in `add_file`.** At the TOP of `add_file`, find the first of the six `let
…0 = …len();` base lines:

```rust
        let seg0 = self.tables.segments.len();
```

and insert one line directly ABOVE it, recording the doc's first row before anything is pushed:

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
            // VERTICES — do it once per append, never per query. 41's per-frame cull and 50's
            // marquee read THIS cache; without it they'd re-walk every mesh every frame.
            // (`placed` was moved into the objects push above — the ROW owns the frame now.)
            let a = world_obb(geom, &t.objects[ri as usize].0).aabb();
            let (lo, hi) = (a.min_point(), a.max_point());
            self.world_boxes.push(([lo[0], lo[1], lo[2]], [hi[0], hi[1], hi[2]]));
```

**2c. Mark the tree stale at the end of `add_file`.** Find the last lines of `add_file` — the
planar block's closing brace, then the doc push:

```rust
        let _ = obj0;
        self.docs.push(Doc {
            name,
            place,
            session,
            cloud_px
        });
```

and insert one line between them:

```rust
        let _ = obj0;
        self.bvh_dirty = true;
        self.docs.push(Doc { name, place, session });
```

A flag, not a build — deliberately. Nothing queries the tree DURING a load, so building per
append does ten throwaway builds of growing size and only the last one is ever used. Deferring
to the first query (Step 3) makes a ten-file load pay for exactly ONE build. Measured at this
lesson's own scale — 744k boxes — a build is ~250 ms; eager-per-append across ten files wastes
over a second of load for nothing.

The build itself goes inside `impl Scene`, directly below `add_file`'s closing brace (still
ABOVE the `}` that closes the impl):

```rust
    /// Rebuild the whole tree from the cached extents (LBVH: O(n) after the Morton radix
    /// sort — ~250 ms at 744k boxes). Runs lazily from `objects_in` whenever `bvh_dirty`;
    /// 49's reconcile just sets the flag again after applying a diff — the per-row extents
    /// cache is what keeps that cheap; a true incremental refit stays future work. Boxes go
    /// in ROW order → object_id == row == index into `order`.
    ///
    /// `build_from_aabbs`, NOT `build_with_guids`: the guid path clones one String per row
    /// into the tree and wraps every AABB in an OBB — measured 3.2x the build time and 42.6 MB
    /// of retained Strings at 744k rows — and the viewer never reads them: a query returns
    /// object_ids, which ARE rows here, and `order[row]` already knows the guid. The `ws`
    /// argument is metadata only (the kernel normalizes Morton codes over the input's own
    /// bounds); pass the extents' own span so the field stays honest.
    fn rebuild_bvh(&mut self) {
        let aabbs: Vec<AABB> = self.world_boxes.iter()
            .map(|(lo, hi)| AABB::new(
                (lo[0] + hi[0]) * 0.5, (lo[1] + hi[1]) * 0.5, (lo[2] + hi[2]) * 0.5,
                (hi[0] - lo[0]) * 0.5, (hi[1] - lo[1]) * 0.5, (hi[2] - lo[2]) * 0.5))
            .collect();
        let ws = aabbs.iter().fold(0.0f64, |w, a| {
            w.max(2.0 * (a.cx.abs() + a.hx)).max(2.0 * (a.cy.abs() + a.hy)).max(2.0 * (a.cz.abs() + a.hz))
        });
        self.bvh = SpatialBVH::new();
        self.bvh.build_from_aabbs(&aabbs, ws);   // empty slice → empty tree; query returns []
        self.bvh_dirty = false;
    }
```

(Zero inflate is right here: the pad already went in when `world_obb` filled the cache entry.
No pointer tree is built — the kernel's arena path is flat `(2n-1) × 64 B` nodes, which at
744k rows is ~91 MB; that plus the 36 MB extents cache is the honest price of a broad-phase
at this scale, and both are O(n) with small constants.)

> **The boxes go stale when a row moves.** Any lesson that transforms an object in place —
> 49's reconcile — which already exists: its `commit` marked the hook point, and the step below
> installs it — and later the gumball drags (67–69) must write the row's new world box into
> `world_boxes[row]` and set `bvh_dirty = true` as part of its commit, or picks and marquee
> queries keep seeing the old geometry. The next query rebuilds; the tree and the extents
> cache are only as fresh as the last writer.

## Step 3 — the query every later lesson calls: `src/app/scene.rs`

Three methods, all going in the same place: inside `impl Scene`, below the `rebuild_bvh` you just
added. First type in `placed_frame` and `doc_of_row` exactly as they were printed near the top of
this lesson (the `doc_rows` field they read exists as of Step 2). Then, below them, the query
47/50 build on — those two lessons differ only in the box they pass:

```rust
    /// Rows whose world box intersects `query` — the broad-phase. Callers narrow further
    /// (47 does ray↔triangle, 50 tests the marquee frustum) on this short list, not all N.
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

`objects_in` answers boxes; 57's ray, 60's marquee and 96's measure snaps all want the same
walk with a different test. The kernel tree's nodes are `pub` (`root`, `left`, `right`,
`aabb`, `object_id`) — so ONE generic visitor in the viewer serves them all, and no query
ever writes its own recursion again:

```rust
pub enum Hit3 { Miss, Intersects, Contained }

/// The one traversal every spatial query shares. `test` classifies a node's box;
/// `Contained` short-circuits: the WHOLE subtree is accepted without further tests —
/// that one arm is what makes 60's marquee stop caring how many objects the scene has.
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

`objects_in`'s body becomes a closure over it (box-overlap test, `Contained` when the query
box contains the node box); the brute-force parity test below now covers the visitor for
every future caller at once. Later lessons collect their dividends: 57 walks it
nearest-child-first with an early-out (compare the ray's entry distance to each child box,
recurse the nearer first, stop when the best hit beats the next node), 60 gets the
`Contained` marquee arm, and 69 refits node boxes in place during drags — all through these
same `pub` nodes, no kernel changes. (If traversal ever profiles hot at millions of
objects, the cache upgrade is flattening these boxed nodes into a `Vec` of 32-byte
implicit-left-child records — an optimization, not a redesign, BECAUSE every caller goes
through `shapecast`.)

## Step 4 — prove it: `#[cfg(test)]` broad-phase == brute force

A BVH is only trustworthy if it returns *exactly* the brute-force set — no misses, no phantoms.
The test also proves the PLACEMENT half: the same document loaded twice with different manifest
placements must yield disjoint boxes. Add it at the very bottom of `scene.rs`, below the
`world_obb` you added in Step 1; it needs no GPU, so it runs headless:

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

## Streamed clouds are not in this tree

A `CloudSlot` (42) pushes its object row with `object_bounds: None` — deliberately. Its
points live only on the GPU, so there is no per-object geometry to derive a box from at
walk time, and the BVH simply gets no leaf for it. That is consistent with where this
phase is headed: streamed clouds are display objects — unpickable, unselectable — until a
later lesson gives them kernel-side structure. (Their SCENE box still exists — lesson 43's
`grow_bounds` feeds the camera — it is the per-object index they sit out.)

## The curved types join `world_obb`

The geometry block (43–47) shipped four types with no box map to join — this lesson is
where that map is born, so their arms are steps HERE, not there. In `world_obb`'s match,
beside the Mesh arm (all four local; the row's placed frame lifts them to world like every
other kind — one rule, no exceptions):

```rust
        // kernel-exact ctors where they exist:
        Geometry::NurbsCurve(nc) => OBB::from_nurbscurve(nc, PAD, true),
        Geometry::NurbsSurface(ns) => OBB::from_nurbssurface(ns, PAD),
        // BRep boxes its CACHED tessellation (46's entry is (mesh, linework) — box the .0;
        // warm by construction: the walk filled the cache before any BVH build):
        Geometry::BRep(_) => {
            let (m, _) = &self.tess_cache[&guid];
            OBB::from_mesh(m, PAD)
        }
```

with `Trimmed` taking the same cached-mesh route through its `ObjRef` arm — 47's
`all_objects()` hands this walk both sources, so the arms are the ONLY per-type work. (The
kernel's `from_nurbssurface` sampler is trim-blind; the cached mesh is what's on screen
anyway. And 43's `curve_cache` samples would box a curve too — the kernel ctor is used
because it reads the CV net exactly, cache warm or not.)

## Reconcile grows the hook

49's `commit` marked exactly where this lesson's state joins the reload path. **Find** (in
`commit`, 49):

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
the BVH's leaf ids equal positions in it, so `id == row` survives a reload untouched: 49
keeps rows stable, and the hook above rewrites boxes AT their rows. The archive's variant
zipped `world_boxes` with `order` here and needed a post-reconcile repair; this design
doesn't. The one identity that DOES die is the reverse translation `self.order[row]`
(`objects_in`'s closing comment leans on it for names): after a reload, `order` is rebuilt
while rows persist, so a UI that wants a row's guid must invert `guid_to_row` (or keep a
`row_to_guid` map maintained beside it) — flag it now, pay it when a consumer appears (82's
tree is the first).

## What this costs — measured, not estimated

Numbers from a 744k-box synthetic sheet scene (30 m of drawings, mm units), the scale this
lesson's big-picture quotes:

```
build (lazy, once per load)   ~250 ms      LBVH: radix sort + O(n) Karras pass
pick-sized query               ~8 us       1000 queries in 8 ms, exact hits == brute force
arena memory                   (2n-1) x 64 B  ->  ~91 MB      flat nodes, no pointers
extents cache                  n x 48 B        ->  ~36 MB      serves 41's linear cull too
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

Nothing on screen moves this lesson — the tree is pure infrastructure. Lesson 41 is where it earns
its keep: the drawn-object count on the perf HUD drops the moment you zoom in.

## Recap

```
Ch 35: Scene owns docs + tables + order + guid_to_row + hidden; rows are GLOBAL, appended per doc.
Ch 40: Scene gains ONE broad-phase — the kernel's SpatialBVH (reused, not rewritten) — and the two
       helpers half the course leans on: placed_frame(row) = tables.objects[row].0 (the manifest
       place × session world xform that add_file stored) and doc_of_row(row) (contiguous per-doc
       row ranges). Every box is LOCAL geometry × placed frame — one rule, all ELEVEN kernel
       kinds (35's walk rows them all), no geometry placement to consult because none exists. Boxes append with the rows in
       add_file (extents cached once per append), the tree rebuilds per file (milliseconds), and
       queries return ROWS — unambiguous across docs, straight into flags/instances/order.
       objects_in(OBB) → rows is the one call picking (47, ray sliver) and box-select (50,
       marquee) narrow from; 41's frustum cull stays a linear scan over the extents cache.
       The tree builds LAZILY (bvh_dirty, first query after a change) via build_from_aabbs —
       no guid Strings, no OBB wrap, one build per load (~250 ms at 744k; ~8 us/query;
       arena (2n-1)x64 B). Kernel fixed alongside: Morton codes normalize over the input's
       bounding cube, not an origin-centered world_size (660x off-origin query win, x3 langs).
       A #[cfg(test)] proves tree == brute force AND that placement lands in the boxes.
       Zero visual change — infrastructure for the lessons that query it.
```

Edited: `app/scene.rs` (`placed_frame`/`doc_of_row`, `world_obb(geom, placed)`, `bvh` +
`bvh_dirty` + `world_boxes` + `doc_rows` fields, per-row box append in `add_file`,
lazy `rebuild_bvh` via `build_from_aabbs`, `objects_in(&OBB) -> Vec<u32>`, `#[cfg(test)]`
parity).

## Next

`68-frustum-culling.md` — extract the view frustum's 6 planes from the ANCHORED `view_proj`
(Gribb–Hartmann, f64), rebase them by the anchor (the camera-relative matrix and 40's world boxes
are in different frames), plane-test every row's cached extents, and set `FLAG_CULLED` on
everything off-screen. The shader collapses a culled instance to a degenerate vertex, so the whole
arena still draws in **one** call — and the perf HUD's drawn/total split finally moves.
