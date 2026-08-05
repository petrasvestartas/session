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

    /// The cached render mesh for a surface — tessellated on FIRST use, then reused
    /// for every rebuild, pick, and box until the SHAPE changes (edit lessons
    /// invalidate; transforms must not).
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

**(1) ORDER** — `is_renderable` admits `Geometry::NurbsSurface(_)` (the same one-word arm as 60).

**(2) BUILD** — the cached mesh flows through the **existing** mesh path, as one more arm in
`Scene::build`'s match. One catch: `surface_mesh` takes `&mut self`, so it cannot run inside the
walk (the loop already borrows `self.order`/`self.session` — E0502). Prime the cache first — insert
at the **top of `build`**, before the accumulators:

```rust
        // tessellate-on-first-use, BEFORE the walk borrows self immutably (E0502 otherwise)
        let ns_guids: Vec<String> = self.session.objects.nurbssurfaces.iter()
            .map(|s| s.guid().to_string()).collect();
        for guid in &ns_guids { self.surface_mesh(guid); }
```

then add the arm beside 60's curve arm:

```rust
            Geometry::NurbsSurface(ns) => {
                // PLACEMENT is ns.xform — the cached mesh carries an IDENTITY xform (SHAPE only);
                // feeding m.xform here would snap a moved/rotated surface back to origin.
                objects_base.push((ns.xform.duplicate(), surface_color(ns), flags));
                if let Some(m) = self.tess_cache.get(guid) {
                    push_mesh(m, ri, &mut verts, &mut vids, &mut idx, &mut segments, &mut glyphs);
                }
            }
```

with the color helper beside 60's `curve_color` (surfaces carry no scalar color either):

```rust
pub fn surface_color(ns: &NurbsSurface) -> [f32; 4] {
    ns.facecolors.first().map(|c| c.to_f32()).unwrap_or([0.75, 0.75, 0.78, 1.0])
}
```

(`scene.rs`'s `session_rust` use gains `NurbsSurface`.)

**(3) WORLD BOX** — one arm in `world_obb` (36): `Geometry::NurbsSurface(ns) =>
OBB::from_nurbssurface(ns, PAD),` (kernel-exact, samples the surface itself).

**(4) PICK** — the cached mesh is the pick proxy: in `pick_mesh`'s candidate match (42), the
`NurbsSurface` arm ray-casts `self.tess_cache[guid]` exactly like the Mesh arm (remember the
inverse-transform uses `ns.xform`, not the cached mesh's identity `m.xform` — the same placement
rule as the build arm).

**Smooth shading arrives free.** Lesson 22 made the shader data-driven: vertices with zero normals
shade flat (screen-space derivatives), vertices with baked normals shade smooth. The kernel bakes
them — so a sphere surface renders smooth and a box mesh stays faceted, same pipeline, no flag, no
new shader. That decision from 39 lessons ago was for this moment.

## Step 3 — transforms stay matrices: `src/state.rs` + `src/app/scene.rs`

54's commit path calls `apply_delta` then `apply_object` (re-flatten). For surfaces, re-flatten would
re-tessellate — the bug. Split the commit by *what changed*:

<svg viewBox="0 0 680 176" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="SHAPE versus PLACEMENT: the cached tessellation carries an identity xform and holds only shape; placement lives on ns.xform, which the gumball composes deltas into and set_object_row uploads as the instance model; feeding the mesh's own identity xform snaps the surface back to origin" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="120" y="22" fill="#888" text-anchor="middle">SHAPE — what it is</text>
  <text x="430" y="22" fill="#888" text-anchor="middle">PLACEMENT — where it sits</text>
  <rect x="10" y="32" width="220" height="60" fill="none" stroke="#6fb3ff"/>
  <text x="24" y="52" fill="#d7dae0">tess_cache[guid] : Mesh</text>
  <text x="24" y="68" fill="#666">verts — local / model space</text>
  <text x="24" y="84" fill="#666">xform = IDENTITY  (never moves)</text>
  <rect x="320" y="32" width="220" height="60" fill="none" stroke="#6fb3ff"/>
  <text x="334" y="52" fill="#d7dae0">ns.xform : Xform</text>
  <text x="334" y="68" fill="#666">gumball drag composes a delta:</text>
  <text x="334" y="84" fill="#6fb3ff">ns.xform = delta * &amp;ns.xform</text>
  <line x1="430" y1="92" x2="430" y2="120" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah61b)"/>
  <text x="24" y="134" fill="#d7dae0">set_object_row(row, model, color, flags)</text>
  <text x="330" y="134" fill="#5bbf87">model = ns.xform.duplicate()  ✓ stays put</text>
  <text x="330" y="152" fill="#e06c6c">model = m.xform (= I)         ✗ snaps to origin</text>
  <defs><marker id="ah61b" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

In `apply_delta` (54, `app/scene.rs`), surfaces compose their xform like meshes — the tessellation
is LOCAL; placement lives on the instance row; **no cache invalidation here**. Add the arm beside
`Geometry::BRep`:

```rust
        Geometry::NurbsSurface(ns) => ns.xform = delta * &ns.xform,
```

(Kernel wrinkle: `Session` stores surfaces in `lookup` *and* in `objects.nurbssurfaces` — the
`add_nurbssurface` dual write. `apply_delta` mutates the `lookup` copy, which is what `build`'s
placement arm above reads; keep it that way consistently and the collection copy is just the
tessellation source, whose *shape* never changes on a transform.)

Then split the commit by *what changed* — in `TransformObjects::restore` (54,
`app/history/transform.rs`), find `scene.apply_object(gpu, &guid, geom, row);` → replace with:

```rust
            let flags = if scene.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            match geom {
                // matrix-only: shape untouched, one row write (38b's verb), cache untouched
                Geometry::Mesh(m) =>
                    gpu.set_object_row(row, m.xform.duplicate(), m.objectcolor().to_f32(), flags),
                Geometry::BRep(b) =>
                    gpu.set_object_row(row, b.xform.duplicate(), b.surfacecolor.to_f32(), flags),
                Geometry::NurbsSurface(ns) =>
                    gpu.set_object_row(row, ns.xform.duplicate(), surface_color(ns), flags),
                // thin geometry bakes coords → re-flatten (54's path, unchanged)
                _ => scene.apply_object(gpu, &guid, geom, row),
            }
```

(`set_object_row(row, model, color, flags)` — 38b's row verb, writing `objects_base[row]` + the
instance row. `transform.rs` needs `use crate::app::scene::surface_color;` +
`use crate::engine::gpu::Instance;`, and `surface_color` goes `pub` for it.)

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
