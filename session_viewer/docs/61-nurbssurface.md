# 61 NurbsSurface — tessellate once, transform matrices forever

> **Big picture.** *Phase 10.* A NURBS surface is a mathematical sheet; the GPU eats triangles. The
> bridge is tessellation — and the entire lesson is one economic rule: **tessellate once, cache the
> mesh, and never re-tessellate for a transform.** The archive measured the failure mode: gumball-
> dragging a surface re-tessellated it every commit, and frames died. Since the Xform refactor the
> rule holds **by construction**: placement lives ONLY in `session.xforms` (composed with the
> manifest place into the row's placed frame, 36), and an object's stored coordinates never move —
> so a tessellation cache keyed by guid holds pure SHAPE, and no transform can even *reach* it.
> Only a shape edit (73) invalidates.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the surface tessellates once into a cached mesh with baked vertex normals; transforms only touch the row's placed frame; only a shape edit invalidates the cache" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="34" width="130" height="32" fill="none" stroke="#6fb3ff"/><text x="75" y="54" fill="#d7dae0" text-anchor="middle">NurbsSurface</text>
  <rect x="180" y="34" width="150" height="32" fill="none" stroke="#6fb3ff"/><text x="255" y="48" fill="#d7dae0" text-anchor="middle">.mesh() ONCE</text><text x="255" y="60" fill="#666" text-anchor="middle" font-size="9">verts + baked normals</text>
  <rect x="370" y="34" width="140" height="32" fill="none" stroke="#6fb3ff"/><text x="440" y="48" fill="#d7dae0" text-anchor="middle">tess_cache[guid]</text><text x="440" y="60" fill="#666" text-anchor="middle" font-size="9">SHAPE only, local coords</text>
  <line x1="140" y1="50" x2="178" y2="50" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61)"/>
  <line x1="330" y1="50" x2="368" y2="50" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61)"/>
  <text x="530" y="44" fill="#888">gumball → placed frame (54)</text>
  <text x="530" y="60" fill="#888">shape edit (73+) → invalidate</text>
  <text x="340" y="106" fill="#666" text-anchor="middle">re-tessellating on transform was the archive's measured perf bug — a moved surface is the same surface</text>
  <defs><marker id="ah61" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/scene.rs   # surfaces join every map (60's discipline); tess_cache: HashMap<guid, Mesh>
```

Just `scene.rs` — and note what is *absent* from the list: nothing on the transform path. 54's
commit (`Scene::apply_world_delta`) is type-blind, so surfaces were transformable before this
lesson added a single arm.

## Step 1 — the cache: `src/app/scene.rs`

The kernel does the hard part — `NurbsSurface::mesh()` returns a `Mesh` with **baked vertex normals**
(the deflection-refined pipeline shared by all three languages). Cache it by guid (kernel-gap #7 in
`_KERNEL_GAPS.md`: a kernel-side cached render mesh would serve every consumer, not just this
viewer). One field on `struct Scene` (below `hidden`, beside 60's `curve_cache`):

```rust
    pub tess_cache: std::collections::HashMap<String, Mesh>,   // ← ADD (init empty in Scene::new)
```

The cache fills on first use, inside `add_file`'s walk (Step 2's arm) — and here is the whole
thesis in one type signature: the key is a guid, the value is a `Mesh` in **document-local
coordinates**. Placement never enters. Since the Xform refactor no geometry carries a transform —
`session.xforms` (× the manifest place) is the only placement store, and `add_file` composes it
into the row's instance model, `tables.objects[row].0`. So the cache is SHAPE **by construction**:
there is no field a transform could stale, and no code path from `apply_world_delta` to this map.
The old design (a transform baked into the surface, cache checked against it) had to *prove* its
cache valid; this one cannot be invalid.

One borrow honesty note, since the old draft of this lesson fought E0502 here: `add_file` takes
`let t = &mut self.tables;` before the walk, and the new arm calls
`self.tess_cache.entry(...)` *inside* it — two mutable borrows through `self`, legal only because
both are **direct field accesses** on disjoint fields (the borrow checker splits them). Wrap the
tessellation in a `&mut self` helper method and the error comes back: a method call borrows all of
`self`.

## Step 2 — every map, again: `src/app/scene.rs`

The same four arms as 60, but the build arm goes through the **existing** `push_mesh` — a
tessellated surface *is* a mesh, so 30–32's whole pipeline (arena, edge tubes, vertex glyphs)
applies untouched:

**(1) ORDER** — `is_renderable` admits `Geometry::NurbsSurface(_)` (the same one-word arm as 60).

**(2) BUILD** — in `add_file`'s walk match, surfaces currently sit in the skip arm. Find:

```rust
                Geometry::Plane(_) | Geometry::OBB(_) |
                Geometry::PointCloud(_) | Geometry::Element(_) |
                Geometry::NurbsSurface(_) => { continue }
```

delete `Geometry::NurbsSurface(_) |` from it (the arm keeps the other four), and add the real arm
beside the curve arm (60):

```rust
                Geometry::NurbsSurface(ns) => {
                    // PLACEMENT is the row's placed frame (pushed with the row, like every kind);
                    // the cached mesh is SHAPE — local coordinates, tessellated on FIRST use only.
                    t.objects.push((placed, surface_color(ns), flags));
                    let m = self.tess_cache.entry(guid.clone())
                        .or_insert_with(|| ns.mesh());
                    push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                        &mut t.pipes, &mut t.spheres);
                }
```

Notice what is *not* here: no collection scan. `NurbsSurface` is a `Geometry` variant registered in
`lookup` (the gap-#4 fix, 60), so the walk hands us the object directly — the archive-era
`objects.nurbssurfaces.iter().find(guid)` was O(N) per surface and is simply gone. The color helper
goes beside 60's `curve_color` (surfaces carry no scalar color either):

```rust
pub fn surface_color(ns: &NurbsSurface) -> [f32; 4] {
    ns.facecolors.first().map(|c| c.to_f32()).unwrap_or([0.75, 0.75, 0.78, 1.0])
}
```

(`scene.rs`'s `session_rust` use gains `NurbsSurface`.)

**(3) WORLD BOX** — one arm in `world_obb`'s match (36): `Geometry::NurbsSurface(ns) =>
OBB::from_nurbssurface(ns, PAD),` (kernel-exact, samples the surface itself) — the local box goes
through the row's placed frame like every other kind, one rule.

**(4) PICK** — the cached tessellation is the pick proxy. In `pick_mesh`'s candidate match (42,
renamed by 44), add the arm beside `Geometry::BRep`:

```rust
                // cached tessellation as the pick proxy — same local-frame contract as the Mesh
                // arm (`frame` is the row's placed frame, the cached mesh is local), minus the
                // Rc: the cache OWNS its Mesh, so the lazy triangle-BVH build needs no make_mut.
                Some(session_rust::Geometry::NurbsSurface(_)) =>
                    self.tess_cache.get_mut(&guid)
                        .and_then(|m| raycast_mesh(m, &frame, ray, PICK_EPS)),
```

**Smooth shading arrives free.** Lesson 22 made the shader data-driven: vertices with zero normals
shade flat (screen-space derivatives), vertices with baked normals shade smooth. The kernel bakes
them — so a sphere surface renders smooth and a box mesh stays faceted, same pipeline, no flag, no
new shader. That decision from 39 lessons ago was for this moment.

## Step 3 — transforms: nothing to write

This is the step the archive version of this lesson spent its pages on — a per-type commit split so
surfaces would take a matrix-only path. Since the Xform refactor the split does not exist to write:
54's `Scene::apply_world_delta` commits every transform as `session.set_xform` + the row's placed
frame + the cached world box, **type-blind** — it never matches on geometry, so no surface arm, no
special case, and the tess cache is untouched because no line of the transform path can reach it.
SHAPE and PLACEMENT live in different stores; "transform" and "tessellation" share zero code.

<svg viewBox="0 0 680 176" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="SHAPE versus PLACEMENT: tess_cache holds local-coordinate triangles keyed by guid and carries no transform at all; placement is session.xforms times the manifest place, stored as the row's placed frame, which apply_world_delta rewrites; the cache has no field a transform could invalidate" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="120" y="22" fill="#888" text-anchor="middle">SHAPE — what it is</text>
  <text x="440" y="22" fill="#888" text-anchor="middle">PLACEMENT — where it sits</text>
  <rect x="10" y="32" width="230" height="60" fill="none" stroke="#6fb3ff"/>
  <text x="24" y="52" fill="#d7dae0">tess_cache[guid] : Mesh</text>
  <text x="24" y="68" fill="#666">verts — document-LOCAL coords</text>
  <text x="24" y="84" fill="#666">no xform field exists to go stale</text>
  <rect x="320" y="32" width="250" height="60" fill="none" stroke="#6fb3ff"/>
  <text x="334" y="52" fill="#d7dae0">session.xforms[guid] × manifest place</text>
  <text x="334" y="68" fill="#666">= tables.objects[row].0 (placed frame, 36)</text>
  <text x="334" y="84" fill="#6fb3ff">apply_world_delta rewrites BOTH (54)</text>
  <line x1="440" y1="92" x2="440" y2="120" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61b)"/>
  <text x="24" y="134" fill="#5bbf87">✓ transform = two matrix writes, cache untouched, one instance-row upload</text>
  <text x="24" y="152" fill="#e06c6c">✗ re-tessellating on commit — the archive's measured bug (reproduce it in Step 4, then revert)</text>
  <defs><marker id="ah61b" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

> **One allocation, not two.** The kernel registers a surface in `lookup` AND in
> `objects.nurbssurfaces` — but both hold the same `Rc<NurbsSurface>`: one allocation, two handles
> (the C++ `shared_ptr` model). Mutation is copy-on-write — `Rc::make_mut` on one handle splits off
> a private copy while the *other* handle keeps the old allocation — and the kernel's contract
> (documented in `session.rs`) is **lookup wins**: mutate through `lookup.get_mut`, and
> `objects_synced()` re-shares any COW split at save time. Nothing in this lesson mutates a surface
> (73 will, through lookup); the walk reads through the lookup handle, and the tess cache stores
> its own independent `Mesh`.

`tess_cache` is invalidated in exactly two places, both later: a shape edit (73's control-point
drag, which calls `tess_cache.remove(guid)` then rebuilds once on release) and reconcile's
`changed` bucket (38b — an external edit may have reshaped the surface). Transforms touch neither —
they can't.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Load a fixture with surfaces (or add a demo `NurbsSurface` — a torus or sphere patch shows curvature
best):

- The surface shades **smooth** — no facet lines across the body — while boxes in the same scene stay
  flat-shaded. One draw path, data decides (22's payoff).
- Select it, **gumball-drag it around, rotate it, scale it** — watch the perf HUD: framerate flat,
  **zero** upload spikes on commit (a transform is two matrix writes and one 96-byte instance row).
  Now the counter-experiment: temporarily make the commit re-tessellate — in `apply_world_delta`,
  add `self.tess_cache.remove(&guid);` and rebuild + `set_scene` after it — and drag again: every
  release hitches as the surface re-meshes. Revert. That hitch is the archive's bug, reproduced and
  understood — and note you had to *sabotage two stores at once* to cause it; the default design
  cannot express it.
- Pick it (click lands on the tessellated body), marquee it, hide it, `F` includes it — the 60 audit,
  run again for surfaces. Undo/redo transforms round-trip exactly (54's `TransformObjects` snapshots
  placements; the surface's bytes never changed).

## Recap

```
Ch 60: curves — one collection, every map.
Ch 61: SURFACES. NurbsSurface::mesh() (kernel deflection-refined pipeline) → tess_cache[guid],
       computed on FIRST use in add_file's walk (entry/or_insert_with — legal beside `t = &mut
       self.tables` only because disjoint FIELD borrows split). The cached mesh IS a mesh: build
       flows through push_mesh (arena + edge tubes + glyphs untouched), pick reuses 42's
       raycast_mesh on the cache (it owns its Mesh — no make_mut), box = OBB::from_nurbssurface ×
       placed frame. Smooth shading free (kernel bakes normals, 22's data-driven shader).
       THE RULE, now BY CONSTRUCTION: placement lives ONLY in session.xforms × manifest place =
       the row's placed frame; the cache is keyed by guid and holds LOCAL triangles — pure SHAPE,
       no field a transform could stale. 54's apply_world_delta is type-blind: no surface arm, no
       commit split, cache untouched. Rc truth: lookup and objects share ONE allocation; edits are
       COW via make_mut through lookup (lookup wins; objects_synced re-shares at save). Cache
       invalidates only on shape edits (73) and reconcile's changed bucket (38b).
```

Edited: `app/scene.rs` (`tess_cache`, `surface_color`, four arms: is_renderable / add_file build /
world_obb / pick_mesh).

## Next

`62-isocurves.md` — the tessellated body reads as a surface only when its **edges** say so: boundary
curves and iso-parameter lines (the u/v grid lines every CAD surface wears), extracted from the
kernel and drawn through the 31 tube path, hugging the surface with no z-fighting.
