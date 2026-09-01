# 60 BRep — many faces, one object

> **Big picture.** *Phase 4b.* The BRep (boundary representation) is the kernel's solid-modeling
> heavyweight — the type booleans, STEP files, and real CAD assemblies live in. The viewer has
> drawn it since the first walk (`b.mesh()` → `push_mesh`), but two debts ride along: `mesh()` is
> a REAL tessellation pipeline (classify faces → extract shared-edge discretizations → CDT-mesh
> faces against matched boundaries — no kernel-side cache to lean on), and it runs at **two call
> sites per walk** (`Geometry::BRep` and `ElementGeometry::BRep`), per rebuild; and its edges
> draw as tessellation wireframe instead of its real curve network (`m_curves_3d`). This lesson
> pays both with machinery that now exists: 44/45's cache entry and 43's sampler.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a brep is faces plus shared edge curves as one object; the viewer caches one tessellated mesh and draws real edge curves" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(30,14)">
    <path d="M 0,60 L 60,20 L 150,20 L 90,60 Z M 90,60 L 150,20 L 150,70 L 90,104 Z M 0,60 L 90,60 L 90,104 L 0,104 Z" fill="none" stroke="#6fb3ff" stroke-width="1.6"/>
    <text x="0" y="30" fill="#666" font-size="10">faces (m_faces → one mesh)</text>
    <text x="0" y="54" fill="#666" font-size="10">edges = m_curves_3d sampled → 31's tubes (45's recipe)</text>
    <text x="0" y="70" fill="#666" font-size="10" transform="translate(0,50)">one guid, one row — faces and edges move together</text>
  </g>
  <g transform="translate(330,16)">
    <rect x="0" y="0" width="180" height="24" fill="none" stroke="#6fb3ff"/><text x="90" y="16" fill="#d7dae0" text-anchor="middle">tess_cache[guid]</text>
    <text x="90" y="44" fill="#888" text-anchor="middle" font-size="10">(colored mesh, edge-curve tubes)</text>
    <text x="90" y="62" fill="#666" font-size="10" text-anchor="middle">filled once — walked, rebuilt, later picked</text>
    <text x="90" y="80" fill="#666" font-size="10" text-anchor="middle">from the SAME entry</text>
  </g>
</svg>

## Files we touch

```
src/app/scene.rs   # brep_linework (m_curves_3d → tubes); both BRep arms read tess_cache
```

## Step 1 — real edges: `brep_linework`

A BRep carries its edge geometry exactly — `pub m_curves_3d: Vec<NurbsCurve>` — and 43 already
built the sampler for it. Near-black tubes, LOCAL space, instance 0 (the push site stamps, 45's
rule). One archive-sourced trap kept: degenerate seam curves (a cone's apex seam, a collapsed
edge) sample to coincident points, and a zero-length tube renders as NaN garbage — filter them:

```rust
/// The BRep's REAL edge curves as tubes - not its tessellation's triangle edges.
fn brep_linework(b: &BRep) -> Vec<CylinderSegment> {
    let dark = pack_rgba([0.05, 0.05, 0.05, 1.0]);
    let mut out = Vec::new();
    for c in &b.m_curves_3d {
        let pts = sample_curve(c);                    // 43's sampler, reused as-is
        for w in pts.windows(2) {
            if w[0].distance(&w[1], None) < 1.0e-9 { continue }   // zero-length seam filter
            out.push(CylinderSegment { p0: w[0].to_f32(), radius: 0.0, p1: w[1].to_f32(),
                                       instance_id: 0, color: dark, facing: FACING_UNKNOWN });
        }
    }
    out
}
```

## Step 2 — both arms read the cache

Same shape as 45's surface arm — entry-on-the-disjoint-field, `Edges::Suppress`, stamp the
linework at push. **Find** the `Geometry::BRep(b)` arm:

```rust
                Geometry::BRep(b) => {
                    let mut bm = b.mesh();
                    bm.set_objectcolor(b.surfacecolor.clone());
                    let bb = push_mesh(
                        &bm,
                        ri,
                        vb,
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    t.object_bounds.push(bb); t.object_spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                }
```

**Replace with:**

```rust
                Geometry::BRep(b) => {
                    let (bm, lw) = self.tess_cache.entry(guid.clone()).or_insert_with(|| {
                        let mut m = b.mesh();                       // the CDT pipeline — once
                        m.set_objectcolor(b.surfacecolor.clone());
                        (m, brep_linework(b))
                    });
                    let bb = push_mesh(
                        bm,
                        ri,
                        vb,
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres,
                        Edges::Suppress            // real edge curves below, not triangle wireframe
                    );
                    t.pipes.extend(lw.iter().map(|seg| {
                        let mut seg = *seg;
                        seg.instance_id = ri;
                        seg
                    }));
                    t.object_bounds.push(bb); t.object_spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                }
```

and make the **same edit** to the `ElementGeometry::BRep(b)` arm inside the `Geometry::Element`
match — it is the identical block with the identical debt (the cache key is the ELEMENT's walk
guid, which is what `guid` already holds there). Two sites, one pattern; forgetting the Element
arm would be exactly the phase's classic bug, so check it off deliberately.

A cylinder now shows its two rim circles and its seam — not a triangle-edge cobweb — and a
boolean-result solid shows its true intersection curves.

## Step 3 — transforms and the cache: the contract, restated

Nothing to write: the placed frame lives in the instance row, the cached mesh and tubes are
LOCAL, so moving a BRep is two matrix writes that never touch this cache (44's rule, verbatim).
Invalidation stays the shared story — delete evicts (64), an external reshape replaces (49), a
recolor command must remove (future). BRep adds nothing new; that it adds nothing is the design
working.

## Step 4 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib
```

Load a BRep-carrying file (any boolean-result fixture from the kernel's test data):

- Curved BReps wear **clean edge curves** — a cylinder shows rims + seam, not wireframe; no
  flickering micro-tubes anywhere (the zero-length filter earning its line).
- The 44-style log-line experiment on the closure: each BRep guid tessellates ONCE across load +
  any rebuild, at both arms.
- Renders otherwise identical — same silhouettes, same shading; this lesson changed edge
  DECORATION and work timing, not the surface pixels.

## Recap

```
Ch 45: iso lines — surfaces read as surfaces.
Ch 46: BREP, debts paid. (1) tess_cache absorbs BReps — BOTH arms (Geometry::BRep and
       ElementGeometry::BRep, same block, same debt) fill (colored mesh, brep_linework) once
       per guid; b.mesh() is the real CDT pipeline (classify faces, shared-edge
       discretizations, boundary-matched meshing) with no kernel cache, so this is the
       phase's biggest single win. (2) Real edges: m_curves_3d sampled with 43's sampler →
       pipes (SOLID lane, instance stamped at push), zero-length seam filter (degenerate
       seams → NaN tubes, archive fix), Edges::Suppress on the skin. (3) Transforms: nothing
       to add — placement is the row's, cache is local shape, 44's contract holds unchanged.
       One guid, one row — faces and edges are one object everywhere.
```

Edited: `app/scene.rs` (`brep_linework`, both cached BRep arms).

## Next

`61-trimmed.md` — the last citizen of the geometry block: `NurbsSurfaceTrimmed` — a surface with
holes and cut boundaries — is TODAY'S one genuinely invisible type: it lives outside `lookup`, in
its own collection, and the walk never sees it. Adding it is the occasion to turn "every map,
checked" into structure: one `all_objects()` iterator every map consumes.
