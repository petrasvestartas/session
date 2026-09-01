# 57 NurbsSurface — tessellate once, transform matrices forever

> **Big picture.** *Phase 4b.* A NURBS surface is a mathematical sheet; the GPU eats triangles.
> The bridge is tessellation — and the walk you have ALREADY crosses it: the surface arm calls
> `s.mesh()` and pushes the result. The entire lesson is one economic rule laid over that arm:
> **tessellate once, cache the mesh, and never re-tessellate — or re-clone — for a walk or a
> transform.** The archive measured the failure mode: gumball-dragging a surface re-tessellated
> it every commit, and frames died. Since the Xform refactor the rule holds **by construction**:
> placement lives ONLY in the instance row's placed frame (`&place * &placement(&guid)`, composed
> at walk time), and an object's stored coordinates never move — so a cache keyed by guid holds
> pure SHAPE, and no transform can even *reach* it. Only a shape edit (85) or an external reshape
> (49's `changed` bucket) invalidates.

## What `s.mesh()` really costs — read the kernel first

`NurbsSurface::mesh()` (session_rust `nurbssurface.rs`) is not always a remesh:

- if the surface carries a pre-baked `m_mesh`, it returns **`m.clone()`** — a full deep copy of
  every vertex, normal and index, per call;
- a planar surface takes a corner-quad shortcut;
- otherwise it runs the grid remesher (`RemeshNurbsSurfaceGrid`) over the span vectors.

So the walk pays either a tessellation or a deep clone — *plus* a `set_objectcolor` recolor pass —
for every surface, on every walk. And `rebuild()` re-walks EVERY doc (hide, future edits), so that
cost recurs on interaction, not just on load. A per-guid cache in the Scene collapses all of it to
"computed the first time this guid is walked".

## Files we touch

```
src/app/scene.rs   # Scene.tess_cache; the NurbsSurface arm reads through it
```

## Step 1 — the cache

One field on `struct Scene`, beside 43's `curve_cache`:

```rust
    pub tess_cache: HashMap<String, Mesh>, // per-guid COLORED tessellation; BReps join in 46
```

(`tess_cache: HashMap::new(),` in `Scene::new`. Like `curve_cache`, `rebuild` deliberately does
NOT clear it — reusing the cache across rebuilds is the whole point. Eviction is someone else's
verb: delete removes the entry (64), an external reshape replaces it (49).)

The entry holds the mesh **already colored**: `set_objectcolor` bakes the surface's face color
into the vertices, so recoloring is part of the shape bake, not the draw. (Consequence, noted for
much later: a `recolor` command must `tess_cache.remove(&guid)` or its repaint never shows.)

## Step 2 — the arm reads through the cache

**Find** the surface arm in `add_file`'s walk match:

```rust
                Geometry::NurbsSurface(s) => {
                    let mut sm = s.mesh();
                    if let Some(c) = s.facecolors.first() {
                        sm.set_objectcolor(c.clone());
                    }
                    let b = push_mesh(
                        &sm,
                        ri,
                        vb,
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, sm.number_of_vertices()));
                }
```

**Replace with:**

```rust
                Geometry::NurbsSurface(s) => {
                    // tessellate ONCE per guid; every later walk (and 45's linework, 55's pick)
                    // rereads this entry. Legal while `t` borrows self.tables: tess_cache is a
                    // DISJOINT field of self, and the borrow checker splits field borrows.
                    let sm = self.tess_cache.entry(guid.clone()).or_insert_with(|| {
                        let mut m = s.mesh();
                        if let Some(c) = s.facecolors.first() {
                            m.set_objectcolor(c.clone());
                        }
                        m
                    });
                    let b = push_mesh(
                        sm,
                        ri,
                        vb,
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, sm.number_of_vertices()));
                }
```

> **The archive needed a "priming pass" here — you don't.** Its cache filler was a `&mut self`
> *method*, and calling one while `t = &mut self.tables` is alive is E0502 (a method borrows ALL
> of `self`). It solved that by pre-filling the cache in a separate pass before the walk. The
> `entry`-on-a-disjoint-field pattern above (the same one 43's curve arm uses) makes that story
> obsolete: direct field access lets the compiler split the borrow. Remember the distinction —
> it recurs every time a cache lives beside the tables.

**Smooth shading arrives free.** Lesson 22 made the shader data-driven: vertices with zero
normals shade flat (screen-space derivatives), vertices with baked normals shade smooth. The
kernel's tessellators bake them — so a sphere surface renders smooth and a box mesh stays
faceted, same pipeline, no flag, no new shader. That decision from 22 lessons ago was for this
moment.

**(3) WORLD BOX and (4) PICK** — born with their maps: [65](65-scene-bvh.md) boxes the surface
(`OBB::from_nurbssurface`, kernel-exact) and [68](68-raycast-meshes.md) picks the cached
tessellation — the cache entry OWNS its `Mesh`, which is exactly what the lazy triangle-BVH
build wants. Both read what THIS lesson built; the cache is the contract.

## Step 3 — transforms: nothing to write, and that's the lesson

Trace the walk: the placed frame (`&place * &placement(&guid)`) goes into the instance ROW;
`push_mesh` pushes the mesh's LOCAL vertices; the vertex shader multiplies. Nothing about a
surface's stored coordinates moves when the object does — so there is no code to write here, and
no code that COULD stale the cache. When the gumball arrives (67), its commit rewrites
`session.xforms` and the row's placed frame, and this lesson's cache never hears about it. The
counter-experiment (sabotage a commit to clear the cache, feel the hitch) lives in 67's verify,
where a drag exists to feel it with.

## Step 4 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib
```

- Make a surface fixture the same way as 43's curves (`Session::add_nurbssurface`, or load any
  file that carries surfaces): it shades SMOOTH next to a flat-shaded box — one pipeline, data
  deciding (22's payoff).
- Add a `log::info!` line inside the `or_insert_with` closure ("tessellating {guid}") — load the
  scene, then trigger a `rebuild()` (hiding an object does it once hide exists; until then,
  loading the same session as two manifest items shows each guid tessellating once, not twice).
  The log line fires once per distinct surface, ever. Remove the line after.
- Renders are pixel-identical to the pre-cache walk — this lesson moves work, not pixels.

## Recap

```
Ch 43: curves — sample once, cache by guid.
Ch 44: SURFACES, same law, bigger payoff. s.mesh() is a remesh OR a deep clone (kernel m_mesh) +
       a recolor — per walk, per rebuild. Scene.tess_cache[guid] holds the COLORED local mesh:
       computed on first walk via entry().or_insert_with — legal beside `t` because tess_cache
       is a DISJOINT field (the archive's E0502 priming pass is obsolete folklore; know why).
       Cache survives rebuild by design; delete evicts (64), reshape replaces (49), recolor must
       remove (future). Placement lives in the instance row only — a transform CANNOT stale the
       cache, by construction, so gumball drags (67) will be matrix-only for free. Smooth
       shading free (kernel bakes normals; 22's data-driven shader). Box arm 52's, pick arm
       55's — both read this cache.
```

Edited: `app/scene.rs` (`tess_cache`, the cached surface arm).

## Next

`58-isocurves.md` — the tessellated body reads as a surface only when its **edges** say so:
boundary curves and iso-parameter lines (the u/v grid every CAD surface wears), extracted from
the kernel and drawn through the 31 tube path — replacing the triangle-wireframe look the mesh
edge lane gives surfaces today.
