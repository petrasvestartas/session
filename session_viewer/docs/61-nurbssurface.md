# 61 NurbsSurface — tessellate once, transform matrices forever

> **Big picture.** *Phase 10.* A NURBS surface is a mathematical sheet; the GPU eats triangles. The
> bridge is tessellation — and the entire lesson is one economic rule: **tessellate once, cache the
> mesh, and never re-tessellate for a transform.** The archive measured the failure mode: gumball-
> dragging a surface re-tessellated it every commit, and frames died. A transform changes *where* a
> surface is, not *what shape* it is — matrices move it, exactly like every mesh since 33.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the surface tessellates once into a cached mesh with baked vertex normals; transforms only touch the instance matrix; only a shape edit invalidates the cache" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="34" width="130" height="32" fill="none" stroke="#6fb3ff"/><text x="75" y="54" fill="#d7dae0" text-anchor="middle">NurbsSurface</text>
  <rect x="180" y="34" width="150" height="32" fill="none" stroke="#6fb3ff"/><text x="255" y="48" fill="#d7dae0" text-anchor="middle">.mesh() ONCE</text><text x="255" y="60" fill="#666" text-anchor="middle" font-size="9">verts + baked normals</text>
  <rect x="370" y="34" width="120" height="32" fill="none" stroke="#6fb3ff"/><text x="430" y="54" fill="#d7dae0" text-anchor="middle">cache[guid]</text>
  <line x1="140" y1="50" x2="178" y2="50" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61)"/>
  <line x1="330" y1="50" x2="368" y2="50" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61)"/>
  <text x="530" y="44" fill="#888">gumball drag → matrix only</text>
  <text x="530" y="60" fill="#888">shape edit (73+) → invalidate</text>
  <text x="340" y="106" fill="#666" text-anchor="middle">re-tessellating on transform was the archive's measured perf bug — a moved surface is the same surface</text>
  <defs><marker id="ah61" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/scene.rs   # surfaces join every map (60's discipline); tess cache: HashMap<guid, Mesh>
src/state.rs       # gumball path: surfaces take the mesh fast-path (matrix-only), never re-flatten
```

## Step 1 — the cache: `src/app/scene.rs`

The kernel does the hard part — `NurbsSurface::mesh()` returns a `Mesh` with **baked vertex normals**
(the deflection-refined pipeline shared by all three languages). Cache it by guid (kernel-gap #7 in
`_KERNEL_GAPS.md`: a kernel-side cached render mesh would serve every consumer, not just this viewer):

```rust
    pub tess_cache: std::collections::HashMap<String, Mesh>,   // ← ADD to Scene (init empty)

    /// The cached render mesh for a surface — tessellated on FIRST use, then reused for every
    /// rebuild, pick, and box until the SHAPE changes (edit lessons invalidate; transforms must not).
    fn surface_mesh(&mut self, guid: &str) -> Option<&Mesh> {
        if !self.tess_cache.contains_key(guid) {
            let ns = self.session.objects.nurbssurfaces.iter().find(|s| s.guid() == guid)?;
            self.tess_cache.insert(guid.to_string(), ns.mesh());
        }
        self.tess_cache.get(guid)
    }
```

## Step 2 — every map, again: `src/app/scene.rs`

The same four arms as 60, but the build arm goes through `push_mesh`/`flatten_mesh` — a tessellated
surface **is** a mesh, so 30–32's whole pipeline (arena, edge tubes, boundary glyphs) applies
untouched:

```rust
    // (1) ORDER — Scene::new: same loop as 60 over session.objects.nurbssurfaces.
    // (2) BUILD — the cached mesh flows through the EXISTING mesh path:
    for ns in &self.session.objects.nurbssurfaces {
        let guid = ns.guid().to_string();
        let ri = self.guid_to_row[&guid];
        let m = /* tess_cache entry (Step 1) */;
        objects_base_entry(ri, m.xform.duplicate(), ns_surface_color(ns), flags_for(&guid));
        push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
    }
    // (3) WORLD BOX — OBB::from_nurbssurface(ns, PAD) (kernel-exact, samples the surface itself).
    // (4) PICK — the cached mesh is the pick proxy: raycast_mesh (42) against tess_cache[guid],
    //     added as an arm in pick_mesh's candidate loop (surfaces resolve like BReps did).
```

**Smooth shading arrives free.** Lesson 22 made the shader data-driven: vertices with zero normals
shade flat (screen-space derivatives), vertices with baked normals shade smooth. The kernel bakes
them — so a sphere surface renders smooth and a box mesh stays faceted, same pipeline, no flag, no
new shader. That decision from 39 lessons ago was for this moment.

## Step 3 — transforms stay matrices: `src/state.rs` + `src/app/scene.rs`

54's commit path calls `apply_delta` then `apply_object` (re-flatten). For surfaces, re-flatten would
re-tessellate — the bug. Split the commit by *what changed*:

```rust
    // in apply_delta (54): surfaces compose their xform like meshes —
    //   the tessellation is LOCAL; placement lives on the instance row. NO cache invalidation here.
    /* Geometry-less arm, keyed by guid like 60's box arm: */
    if let Some(ns) = session.objects.nurbssurfaces.iter_mut().find(|s| s.guid() == guid) {
        ns.xform = delta * &ns.xform;
    }

    // in the commit loop (54): surfaces (and meshes/BReps) take the FAST path —
    let is_matrix_only = /* Mesh | BRep | NurbsSurface */;
    if is_matrix_only {
        gpu.set_object_row(row, new_model, color, flags);     // one row write — 38b's verb
    } else {
        scene.apply_object(gpu, &guid, geom, row);            // thin geometry re-flattens (54)
    }
```

`tess_cache` is invalidated in exactly two places, both later: a shape edit (73's control-point drag,
which calls `tess_cache.remove(guid)` then re-flattens once on release) and reconcile's `changed`
bucket (38b — an external edit may have reshaped the surface). Transforms touch neither.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Load a fixture with surfaces (or add a demo `NurbsSurface` — a torus or sphere patch shows curvature
best):

- The surface shades **smooth** — no facet lines across the body — while boxes in the same scene stay
  flat-shaded. One draw path, data decides (22's payoff).
- Select it, **gumball-drag it around, rotate it, scale it** — watch the perf HUD: framerate flat,
  **zero** upload spikes on commit. Now the counter-experiment: temporarily route surfaces through
  `apply_object` in the commit path and drag again — every release hitches as it re-tessellates.
  Revert. That hitch is the archive's bug, reproduced and understood.
- Pick it (click lands on the tessellated body), marquee it, hide it, `F` includes it — the 60 audit,
  run again for surfaces. Undo/redo transforms round-trip exactly.

## Recap

```
Ch 60: curves — one collection, every map.
Ch 61: SURFACES. NurbsSurface::mesh() (kernel deflection-refined pipeline) → tess_cache[guid],
       computed on FIRST use only. The cached mesh IS a mesh: build flows through push_mesh (arena +
       edge tubes + glyphs untouched), pick reuses 42's raycast_mesh, box = OBB::from_nurbssurface.
       Smooth shading is free — the kernel bakes vertex normals and 22's data-driven shader does the
       rest (flat boxes and smooth spheres in one pipeline). THE RULE: transforms are matrix-only —
       compose ns.xform, set_object_row, cache untouched (re-tessellating on gumball commit = the
       archive's measured perf bug; a moved surface is the same surface). Cache invalidates only on
       shape edits (73) and reconcile's changed bucket (38b).
```

Edited: `app/scene.rs` (`tess_cache`, `surface_mesh`, four arms, matrix-only commit split),
`state.rs` (surface arm in `apply_delta`).

## Next

`62-isocurves.md` — the tessellated body reads as a surface only when its **edges** say so: boundary
curves and iso-parameter lines (the u/v grid lines every CAD surface wears), extracted from the
kernel and drawn through the 31 tube path, hugging the surface with no z-fighting.
