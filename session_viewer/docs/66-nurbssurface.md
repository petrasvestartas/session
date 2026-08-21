# 66 NurbsSurface — tessellate once, transform matrices forever

> **Big picture.** *Phase 10.* A NURBS surface is a mathematical sheet; the GPU eats triangles. The
> bridge is tessellation — and the entire lesson is one economic rule: **tessellate once, cache the
> mesh, and never re-tessellate for a transform.** The archive measured the failure mode: gumball-
> dragging a surface re-tessellated it every commit, and frames died. Since the Xform refactor the
> rule holds **by construction**: placement lives ONLY in `session.xforms` (composed with the
> manifest place into the row's placed frame, 40), and an object's stored coordinates never move —
> so a tessellation cache keyed by guid holds pure SHAPE, and no transform can even *reach* it.
> Only a shape edit (78) invalidates.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the surface tessellates once into a cached mesh with baked vertex normals; transforms only touch the row's placed frame; only a shape edit invalidates the cache" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="34" width="130" height="32" fill="none" stroke="#6fb3ff"/><text x="75" y="54" fill="#d7dae0" text-anchor="middle">NurbsSurface</text>
  <rect x="180" y="34" width="150" height="32" fill="none" stroke="#6fb3ff"/><text x="255" y="48" fill="#d7dae0" text-anchor="middle">.mesh() ONCE</text><text x="255" y="60" fill="#666" text-anchor="middle" font-size="9">verts + baked normals</text>
  <rect x="370" y="34" width="140" height="32" fill="none" stroke="#6fb3ff"/><text x="440" y="48" fill="#d7dae0" text-anchor="middle">tess_cache[guid]</text><text x="440" y="60" fill="#666" text-anchor="middle" font-size="9">SHAPE only, local coords</text>
  <line x1="140" y1="50" x2="178" y2="50" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61)"/>
  <line x1="330" y1="50" x2="368" y2="50" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61)"/>
  <text x="530" y="44" fill="#888">gumball → placed frame (59)</text>
  <text x="530" y="60" fill="#888">shape edit (78) → invalidate</text>
  <text x="340" y="106" fill="#666" text-anchor="middle">re-tessellating on transform was the archive's measured perf bug — a moved surface is the same surface</text>
  <defs><marker id="ah61" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/scene.rs   # surfaces join every map (65's discipline); tess_cache: HashMap<guid, Mesh>
```

Just `scene.rs` — and note what is *absent* from the list: nothing on the transform path. 59's
commit (`Scene::apply_world_delta`) is type-blind, so surfaces were transformable before this
lesson added a single arm.

## Step 1 — the cache: `src/app/scene.rs`

The kernel does the hard part — `NurbsSurface::mesh()` returns a `Mesh` with **baked vertex normals**
(the deflection-refined pipeline shared by all three languages). Cache it by guid (kernel-gap #7 in
`_KERNEL_GAPS.md`: a kernel-side cached render mesh would serve every consumer, not just this
viewer). One field on `struct Scene` (below `hidden`, beside 65's `curve_cache`):

```rust
    pub tess_cache: std::collections::HashMap<String, Mesh>,   // ← ADD (init empty in Scene::new)
```

plus the filler — a `&mut self` helper beside `sample_curve`'s helpers:

```rust
    /// The surface's cached render mesh — tessellated on FIRST use, SHAPE-pure thereafter.
    /// Scans every doc's lookup (43b's multi-doc rule — no collection scan: the gap-#4 fix
    /// registers surfaces in `lookup`). 67 widens the entry to (mesh, linework); 68 renames
    /// this `render_mesh`.
    fn surface_mesh(&mut self, guid: &str) -> Option<&Mesh> {
        if !self.tess_cache.contains_key(guid) {
            let ns = self.docs.iter().find_map(|d| match d.session.lookup.get(guid) {
                Some(Geometry::NurbsSurface(ns)) => Some(ns),
                _ => None,
            })?;
            self.tess_cache.insert(guid.to_string(), ns.mesh());
        }
        self.tess_cache.get(guid)
    }
```

The cache fills on first use — but *not* inside `add_file`'s walk, and here is the borrow story
the old draft of this lesson fought (E0502): the walk holds `let t = &mut self.tables;`, so inside
it only **direct field accesses** on other fields of `self` are legal — `self.tess_cache.get(..)`
is fine, but calling the `&mut self` helper above borrows ALL of `self` and the compiler says no.
So `add_file` **primes** the cache before the walk — after the new doc lands in `self.docs`,
before any rows exist:

```rust
        // priming pass, top of add_file (after the doc is inserted, before the table walk):
        // warm every surface in the NEW doc — the walk below can only READ the cache
        if let Some(doc) = self.docs.last() {
            let ns_guids: Vec<String> = doc.session.lookup.iter()
                .filter(|(_, g)| matches!(g, Geometry::NurbsSurface(_)))
                .map(|(guid, _)| guid.clone())
                .collect();
            for guid in &ns_guids { self.surface_mesh(guid); }
        }
```

With the mechanics settled, here is the whole
thesis in one type signature: the key is a guid, the value is a `Mesh` in **document-local
coordinates**. Placement never enters. Since the Xform refactor no geometry carries a transform —
`session.xforms` (× the manifest place) is the only placement store, and `add_file` composes it
into the row's instance model, `tables.objects[row].0`. So the cache is SHAPE **by construction**:
there is no field a transform could stale, and no code path from `apply_world_delta` to this map.
The old design (a transform baked into the surface, cache checked against it) had to *prove* its
cache valid; this one cannot be invalid.

## Step 2 — every map, again: `src/app/scene.rs`

The same four arms as 65, but the build arm goes through the **existing** `push_mesh` — a
tessellated surface *is* a mesh, so 30–32's whole pipeline (arena, edge tubes, vertex glyphs)
applies untouched:

**(1) ORDER** — `is_renderable` admits `Geometry::NurbsSurface(_)` (the same one-word arm as 65).

**(2) BUILD** — in `add_file`'s walk match, surfaces currently sit in the skip arm. Find:

```rust
                Geometry::Plane(_) | Geometry::OBB(_) |
                Geometry::PointCloud(_) | Geometry::Element(_) |
                Geometry::NurbsSurface(_) => { continue }
```

delete `Geometry::NurbsSurface(_) |` from it (the arm keeps the other four), and add the real arm
beside the curve arm (65):

```rust
                Geometry::NurbsSurface(ns) => {
                    // PLACEMENT is the row's placed frame (pushed with the row, like every kind);
                    // the cached mesh is SHAPE — local coordinates, warmed by Step 1's priming
                    // pass (the walk holds `t` — it can only READ the cache, never fill it).
                    t.objects.push((placed, surface_color(ns), flags));
                    if let Some(m) = self.tess_cache.get(&guid) {
                        push_mesh(m, ri, &mut t.verts, &mut t.vids, &mut t.idx,
                            &mut t.pipes, &mut t.spheres);
                    }
                }
```

Notice what is *not* here: no collection scan. `NurbsSurface` is a `Geometry` variant registered in
`lookup` (the gap-#4 fix, 65), so the walk hands us the object directly — the archive-era
`objects.nurbssurfaces.iter().find(guid)` was O(N) per surface and is simply gone. The color helper
goes beside 65's `curve_color` (surfaces carry no scalar color either):

```rust
pub fn surface_color(ns: &NurbsSurface) -> [f32; 4] {
    ns.facecolors.first().map(|c| c.to_f32()).unwrap_or([0.75, 0.75, 0.78, 1.0])
}
```

(`scene.rs`'s `session_rust` use gains `NurbsSurface`.)

**(3) WORLD BOX** — one arm in `world_obb`'s match (40): `Geometry::NurbsSurface(ns) =>
OBB::from_nurbssurface(ns, PAD),` (kernel-exact, samples the surface itself) — the local box goes
through the row's placed frame like every other kind, one rule.

**(4) PICK** — the cached tessellation is the pick proxy. In `pick_mesh`'s candidate match (47,
renamed by 49), add the arm beside `Geometry::BRep`:

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
new shader. That decision from 44 lessons ago was for this moment.

## Step 3 — transforms: nothing to write

This is the step the archive version of this lesson spent its pages on — a per-type commit split so
surfaces would take a matrix-only path. Since the Xform refactor the split does not exist to write:
59's `Scene::apply_world_delta` commits every transform as `session.set_xform` + the row's placed
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
  <text x="334" y="84" fill="#6fb3ff">apply_world_delta rewrites BOTH (59)</text>
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
> (78 will, through lookup); the walk reads through the lookup handle, and the tess cache stores
> its own independent `Mesh`.

`tess_cache` is invalidated in exactly two places, both later: a shape edit (78's control-point
drag, which calls `tess_cache.remove(guid)` then rebuilds once on release) and reconcile's
`changed` bucket (43b — an external edit may have reshaped the surface). Transforms touch neither —
they can't.

> **Memory policy.** The cache trades RAM for frame rate, and the bill is real: a deflection-refined
> surface mesh is easily hundreds of thousands of triangles — tens of MB per surface, CPU-side, one
> full copy per cached guid. (The upload path then copies *again* into the GPU arena; a zero-copy
> design would tessellate straight into arena memory, but that couples the kernel's mesher to this
> viewer's buffer layout — we deliberately pay the copy to keep the kernel viewer-agnostic.) Two
> rules keep the map honest: it is SHAPE-keyed, so its size is bounded by *distinct surfaces* —
> 85's hundred-copy array still costs one entry — and entries must leave with their object: 56's
> remove path and 43b's `removed` bucket both `tess_cache.remove(&guid)`, or a long session
> accumulates meshes for surfaces that no longer exist. If a real project blows the budget, evict
> least-recently-used and re-tessellate on demand — the cache is a pure function of the surface, so
> eviction is always safe.
>
> **Load-time cost.** `add_file` tessellates every surface synchronously on first use — a
> 200-surface file stalls the load frame for seconds. The fix is to chunk it across frames
> (tessellate a few surfaces per frame; rows draw as their entries land) — the same spread-the-work
> pattern as 39's streaming cloud. Not built here; noted so the stall is a known tradeoff, not a
> surprise.

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
  release hitches as the surface re-meshes. That hitch is the archive's bug, reproduced and
  understood — and note you had to *sabotage two stores at once* to cause it; the default design
  cannot express it.
- ☐ **Revert the sabotage.** Delete the `tess_cache.remove(&guid)` line *and* the rebuild +
  `set_scene` you just added in `apply_world_delta`, and drag once more to confirm the hitch is
  gone. Left in, every transform commit re-tessellates — the exact bug this lesson exists to kill,
  now permanent. Diff against your pre-experiment state to be sure.
- Pick it (click lands on the tessellated body), marquee it, hide it, `F` includes it — the 65 audit,
  run again for surfaces. Undo/redo transforms round-trip exactly (59's `TransformObjects` snapshots
  placements; the surface's bytes never changed).

## Recap

```
Ch 65: curves — one collection, every map.
Ch 66: SURFACES. NurbsSurface::mesh() (kernel deflection-refined pipeline) → tess_cache[guid],
       computed on FIRST use by surface_mesh (scans every doc's lookup) and warmed by a PRIMING
       PASS at the top of add_file — the walk holds `t = &mut self.tables`, so inside it only
       disjoint FIELD borrows are legal (`self.tess_cache.get`, fine; the `&mut self` filler,
       E0502). The cached mesh IS a mesh: build
       flows through push_mesh (arena + edge tubes + glyphs untouched), pick reuses 47's
       raycast_mesh on the cache (it owns its Mesh — no make_mut), box = OBB::from_nurbssurface ×
       placed frame. Smooth shading free (kernel bakes normals, 22's data-driven shader).
       THE RULE, now BY CONSTRUCTION: placement lives ONLY in session.xforms × manifest place =
       the row's placed frame; the cache is keyed by guid and holds LOCAL triangles — pure SHAPE,
       no field a transform could stale. 59's apply_world_delta is type-blind: no surface arm, no
       commit split, cache untouched. Rc truth: lookup and objects share ONE allocation; edits are
       COW via make_mut through lookup (lookup wins; objects_synced re-shares at save). Cache
       invalidates only on shape edits (78) and reconcile's changed bucket (43b) — and is EVICTED
       on remove, or it leaks meshes for dead surfaces (memory policy: bounded by DISTINCT
       surfaces, LRU-safe since it's a pure function of shape; chunk load-time tessellation across
       frames when the add_file stall matters).
```

Edited: `app/scene.rs` (`tess_cache`, `surface_mesh` + add_file priming pass, `surface_color`,
four arms: is_renderable / add_file build / world_obb / pick_mesh).

## Next

`67-isocurves.md` — the tessellated body reads as a surface only when its **edges** say so: boundary
curves and iso-parameter lines (the u/v grid lines every CAD surface wears), extracted from the
kernel and drawn through the 31 tube path, hugging the surface with no z-fighting.
