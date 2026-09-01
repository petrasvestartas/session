# 59 NurbsSurface — tessellate once, transform matrices forever

> **Big picture.** *Phase 4b.* A NURBS surface is a mathematical sheet; the GPU eats triangles.
> The bridge is tessellation — and the walk you have ALREADY crosses it: since 52 the surface arm
> is an ADAPTER, `walk/surface.rs`, which calls `s.mesh()` and re-enters the mesh producer with
> the result. The entire lesson is one economic rule laid over that adapter: **tessellate once,
> cache the mesh, and never re-tessellate — or re-clone — for a walk or a transform.** 58 built
> the map and the borrow that reaches it; a surface is simply the most expensive thing to put in
> it. The archive measured the failure mode: gumball-dragging a surface re-tessellated it every
> commit, and frames died. Since the Xform refactor the rule holds **by construction**: placement
> lives ONLY in the instance row's placed frame (`mat_mul(&place_m, &local.m)`, composed at walk
> time), and an object's stored coordinates never move — so a cache keyed by guid holds pure
> SHAPE, and no transform can even *reach* it. Only a shape edit (100) or an external reshape
> (64's `changed` bucket) invalidates.

## What `s.mesh()` really costs — read the kernel first

`NurbsSurface::mesh()` (session_rust `nurbssurface.rs`) is not always a remesh:

- if the surface carries a pre-baked `m_mesh`, it returns **`m.clone()`** — a full deep copy of
  every vertex, normal and index, per call;
- a planar surface takes a corner-quad shortcut;
- otherwise it runs the grid remesher (`RemeshNurbsSurfaceGrid`) over the span vectors.

So the walk pays either a tessellation or a deep clone — *plus* a `set_objectcolor` recolour pass —
for every surface, on every walk. And `rebuild()` re-walks EVERY doc (hide, future edits), so that
cost recurs on interaction, not just on load. Sampling a curve (58) was cheap enough that the cache
was mostly about the CONSUMERS wanting the same points; here the cache pays for itself on the walk
alone.

## Files we touch

| file | what |
|---|---|
| `src/app/walk/mod.rs` | `Tess`, and the second map in 58's `Caches`; the surface arm shrinks |
| `src/app/walk/surface.rs` | the adapter reads through the cache — tessellating is what it stops doing |
| `src/app/scene.rs` | one line: `clear` drops the new map with the old one |

Three files and six edits, because 58 already paid for the hard part. Read that as the shape of
this phase: the FIRST type to want a cache buys the plumbing, and every type after it buys a field.

## Step 0 — the adapter you already have

Since 52 the surface path is one small file. `src/app/walk/surface.rs` today reads:

```rust
pub(crate) fn walk_surface(s: &NurbsSurface, t: &mut Upload, ri: u32, base_off: u32) -> Row {
    let mut sm = s.mesh();
    if let Some(c) = s.facecolors.first() { sm.set_objectcolor(c.clone()); }
    walk_mesh(&sm, t, &MeshOpts { ri, base_off, sheet_lanes: false, allow_open: false })
}
```

Read what it is: an adapter, not a producer. It writes no row format of its own — `walk_mesh` does
that — and it holds no state. `sm` is born on line one and dies on line three, which is *exactly*
why every walk pays for it again: a local cannot outlive the function that made it. The colour is
its one real divergence from `brep.rs` (`facecolors.first()`, not `surfacecolor`), and it is baked
into the vertices, so the recolour is part of the shape bake and not part of the draw — which is
what lets one cache entry serve every later walk without a second thought about colour.

## Step 1 — a second map in the cache 58 built

Re-read what you already have in `src/app/walk/mod.rs`: `Caches` is owned by `Scene` and borrowed
by the walk, `WalkCx` carries the guid that keys it, and `walk_geometry` already takes `&mut
WalkCx`. None of that changes. What a surface needs on top of it is two decisions:

- **the value is a struct, not a bare `Mesh`.** A curve's cache entry is `Vec<Point>` and that is
  all a curve derives. A surface derives a mesh *and*, from 60, the iso-curves drawn on it — same
  shape, same lifetime, invalidated by the same edit. `Tess` is where the second one lands without
  re-keying anything.
- **the key is `(guid, u8)`, not `guid`.** One object is one shape — except a BRep, which is one
  object and many faces, and 61 caches those individually under one guid. A surface always writes
  slot `0`; the slot exists so 61 costs a loop index and not a second map.

**Find** in `src/app/walk/mod.rs`:

```rust
use session_rust::{Geometry, Point};
```

**Replace with:**

```rust
use session_rust::{Geometry, Mesh, Point};
```

**Find** in `src/app/walk/mod.rs`:

```rust
pub struct Caches {
    /// Per-guid curve samples, f64, in curve order - `sample_curve`'s output verbatim.
    pub curves: HashMap<String, Vec<Point>>,
}
```

**Replace with:**

```rust
pub struct Caches {
    /// Per-guid curve samples, f64, in curve order - `sample_curve`'s output verbatim.
    pub curves: HashMap<String, Vec<Point>>,
    /// Per-guid tessellations, ALREADY COLOURED - see `Tess`. Keyed by (guid, SLOT) because a
    /// BRep is one object with many faces, which 61 caches one entry apiece; a surface is always
    /// slot 0.
    pub tess: HashMap<(String, u8), Tess>,
}

/// What one tessellated shape left behind: the mesh a surface, a BRep face (61) or a trimmed
/// face (62) turned into, ALREADY COLOURED, so a repaint is an eviction rather than a special
/// case in the draw.
///
/// A struct and not a bare `Mesh` because the mesh is not the only thing a walk derives from one
/// shape and then throws away - 60 keeps this surface's iso-curves beside it, and both die on the
/// same edit.
pub struct Tess {
    pub mesh: Mesh,
}
```

**Replace-all** `src/app/walk/mod.rs` `One field today. 59 adds tessellations, 60 their ink, 62 the trim loops;` → `Two maps today, and both grow: 60 gives Tess its ink, 62 the trim loops;` (1 hit)

The arm gets SHORTER, and that is the reason the adapter takes `cx` instead of a fifth parameter:
`base_off` was only ever `cx.vert_base`, so handing over `cx` pays for itself and leaves
`walk_surface` at four parameters. `architecture.rs` caps a function at five, and 60 still needs
room under it.

**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::NurbsSurface(s) => walk_surface(s, t, ri, cx.vert_base),
```

**Replace with:**

```rust
        Geometry::NurbsSurface(s) => walk_surface(s, t, cx, ri),
```

## Step 2 — the adapter reads through the cache

`walk/surface.rs` is small enough to state whole, and the diff is worth seeing as one piece: the
tessellate-and-recolour pair moves inside an `entry().or_insert_with(..)`, the signature trades
`base_off` for `cx`, and the module doc stops claiming the file is two lines.

**Create `src/app/walk/surface.rs`**

```rust
//! `walk/surface.rs` - the NurbsSurface adapter.
//!
//! Still an adapter: it owns no row format and re-enters the mesh producer, exactly like
//! `brep.rs`. Two things are its own. The colour comes from `facecolors.first()` and not from
//! `surfacecolor` - the single divergence from BRep. And the tessellation is CACHED: it is built
//! the first time this guid is walked, and every walk after that reads the entry.

use session_rust::NurbsSurface;

use crate::engine::gpu::Upload;

use super::mesh::{MeshOpts, walk_mesh};
use super::{Row, Tess, WalkCx};

/// Walk a surface as its tessellation, tessellating at most ONCE per guid.
///
/// `s.mesh()` is either a grid remesh or - when the kernel already carries `m_mesh` - a deep clone
/// of every vertex, normal and index, and `set_objectcolor` then walks the vertices again. That is
/// per-surface, per-walk work, and `rebuild` re-walks every document, so it lands on interaction
/// and not just on load.
pub(crate) fn walk_surface(s: &NurbsSurface, t: &mut Upload, cx: &mut WalkCx, ri: u32) -> Row {
    let base_off = cx.vert_base;        // read before the cache borrows `cx`
    let key = (cx.guid.to_string(), 0); // one String against one remesh - the trade is not close
    let tess = cx.caches.tess.entry(key).or_insert_with(|| {
        let mut m = s.mesh();
        if let Some(c) = s.facecolors.first() { m.set_objectcolor(c.clone()); }
        Tess { mesh: m }
    });
    walk_mesh(&tess.mesh, t, &MeshOpts { ri, base_off, sheet_lanes: false, allow_open: false })
}
```

Two borrows live at once inside that function and neither fights the other: `tess` borrows
`cx.caches` while `t` is a separate `&mut` the caller handed in, and `base_off` was copied out of
`cx` before the cache took hold of it. Copying it out is not superstition — it is the habit that
makes the next reader stop wondering, and it costs a `u32`.

**Smooth shading arrives free.** Lesson 22 made the shader data-driven: vertices with zero normals
shade flat (screen-space derivatives), vertices with baked normals shade smooth. The kernel's
tessellators bake them — so a sphere surface renders smooth and a box mesh stays faceted, same
pipeline, no flag, no new shader. That decision from 37 lessons ago was for this moment.

**(3) WORLD BOX and (4) PICK** — born with their maps: [67](67-scene-bvh.md) boxes the surface
(`OBB::from_nurbssurface`, kernel-exact) and [70](70-raycast-meshes.md) picks the cached
tessellation — the `Tess` entry OWNS its `Mesh`, which is exactly what the lazy triangle-BVH build
wants. Both read what THIS lesson built; the cache is the contract.

## Step 3 — the owner needs one line

`Scene` already owns `caches` and `add_file` already hands the walk a borrow of it, so the whole
of this lesson's ownership work is one line — the one that says a new map is dropped with the old.
`rebuild` still deliberately leaves both alone: a re-walk that reuses the tessellations is the
point, and it is what makes `rebuild` affordable behind a hide or an edit commit. `clear` is the
opposite case: it drops every document, so every guid in the map is gone and keeping their meshes
is a pure leak — on a surface-heavy file, the largest one in the viewer.

**Find** in `src/app/scene.rs`:

```rust
        self.caches.curves.clear();
```

**Add below it:**

```rust
        self.caches.tess.clear();
```

> **The archive needed a "priming pass" here — you don't, and 58 is why.** Its cache filler was a
> `&mut self` *method*, and calling one while `t = &mut self.tables` is alive borrows ALL of
> `self`:
>
> ```text
> error[E0499]: cannot borrow `*self` as mutable more than once at a time
>    |
> 87 |         let t = &mut self.tables;
>    |                 ---------------- first mutable borrow occurs here
> ...
> 96 |             let sm = self.tessellate(&guid, s);
>    |                      ^^^^ second mutable borrow occurs here
> 97 |             let row = walk_mesh(&sm, t, &MeshOpts { .. });
>    |                                      - first borrow later used here
> ```
>
> It solved that by pre-filling the cache in a separate pass before the walk. `let caches = &mut
> self.caches;` taken one line before `let t = &mut self.tables;` makes that story obsolete: two
> DISJOINT fields, read out as locals, are two borrows the compiler splits — `self.hidden` and
> `self.guid_to_row` inside the same loop are the third and fourth. Direct field access is what
> lets it split; a method call cannot. Remember the distinction: it recurs every time a cache
> lives beside the tables.

## Step 4 — transforms: nothing to write, and that's the lesson

Trace the walk: the placed frame (`mat_mul(&place_m, &local.m)`) goes into the instance ROW;
`walk_mesh` pushes the mesh's LOCAL vertices; the vertex shader multiplies. Nothing about a
surface's stored coordinates moves when the object does — so there is no code to write here, and
no code that COULD stale the cache. When the gumball arrives (82), its commit rewrites
`session.xforms` and the row's placed frame, and this lesson's cache never hears about it. The
counter-experiment (sabotage a commit to clear the cache, feel the hitch) lives in 82's verify,
where a drag exists to feel it with.

## Step 5 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo xtest
```

- `xtest` is the one that can fail on architecture rather than on types. `walk_surface` traded a
  parameter for `cx` and stayed at **4** (the ceiling is 5, and 60 needs the room). `Caches` now
  has two fields and they share no stem, so `a_naming_convention_is_not_a_type` has nothing to
  say. Sizes: `app/walk/mod.rs` **156 → 171**, `app/walk/surface.rs` **18 → 30**,
  `app/scene.rs` **291 → 292** — nine lines under its 300, which is the number to remember when
  62 comes asking for room in the same file.
- Make a surface fixture the same way as 58's curves (`Session::add_nurbssurface`, or load any file
  that carries surfaces): it shades SMOOTH next to a flat-shaded box — one pipeline, data deciding
  (22's payoff).
- Put a `log::info!` line inside the `or_insert_with` closure ("tessellating {}", cx.guid) — load
  the scene, then trigger a `rebuild()` (hiding an object does it once hide exists; until then,
  loading the same session as two manifest items shows each guid tessellating once, not twice).
  The line fires once per distinct surface, ever. Take it out afterwards.
- Renders are pixel-identical to the pre-cache walk — this lesson moves work, not pixels. The mesh
  handed to `walk_mesh` is byte-for-byte the one it used to build inline; only its lifetime changed.

## Recap

```
Ch 58: curves — sample once, cache by guid; Caches, and the borrow that reaches it.
Ch 59: SURFACES, same law, bigger payoff, and only a field's worth of new plumbing.
       s.mesh() is a grid remesh OR a deep clone of the kernel's own m_mesh, plus a recolour
       pass — per surface, per walk, and rebuild re-walks every document. Caches.tess[(guid, 0)]
       holds the COLOURED local mesh, built on first walk through entry().or_insert_with, and
       walk/surface.rs stops being the thing that tessellates. The value is a Tess struct, not a
       Mesh, because 60 puts the iso-curves beside it; the key carries a SLOT because 61's BRep
       faces are many shapes under one guid. The arm hands over cx instead of base_off, which
       keeps walk_surface at 4 of its 5 parameters.
       Cache survives rebuild by design; clear drops it; delete evicts (79), reshape replaces (64),
       a recolour must remove. Placement lives in the instance row only — a transform CANNOT stale
       the cache, by construction, so gumball drags (82) are matrix-only for free. Smooth shading
       free (kernel bakes normals; 22's data-driven shader). Box arm 67's, pick arm 70's — both
       read this map.
```

Edited: `app/walk/mod.rs` (`Tess`, `Caches.tess`, the surface arm), `app/walk/surface.rs` (the
cached adapter), `app/scene.rs` (one line in `clear`).

## Next

`60-isocurves.md` — the tessellated body reads as a surface only when its **edges** say so:
boundary curves and iso-parameter lines (the u/v grid every CAD surface wears), extracted from
the kernel and drawn through the 31 tube path — replacing the triangle-wireframe look the mesh
edge lane gives surfaces today.
