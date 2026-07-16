# 64 Trimmed surfaces — first-class, and the every-map rule becomes structure

> **Big picture.** *Phase 10 closes.* `NurbsSurfaceTrimmed` — a surface with cut boundaries and holes
> — is the type the archive forgot **repeatedly**: it drew but didn't pick, picked but missed the
> tree, appeared everywhere except the visibility map. Each forget was the same bug: four maps
> maintained by hand, and a fifth geometry source nobody remembered to add. This lesson adds the
> trimmed surface *and retires the bug class*: one `all_objects()` iterator becomes the single place
> a geometry source is registered, and every map consumes it.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a trimmed surface with a hole; one all_objects iterator feeds order, build, boxes, and pick so a new type is added in exactly one place" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(40,20)">
    <path d="M 0,70 C 40,20 150,15 190,55 L 180,90 C 130,60 50,65 8,95 Z" fill="none" stroke="#6fb3ff" stroke-width="2"/>
    <ellipse cx="95" cy="55" rx="24" ry="12" fill="none" stroke="#6fb3ff" stroke-width="1.6"/>
    <text x="95" y="115" fill="#888" text-anchor="middle">trim boundary + hole — both real edges</text>
  </g>
  <g transform="translate(330,16)">
    <rect x="0" y="0" width="150" height="26" fill="none" stroke="#6fb3ff"/><text x="75" y="17" fill="#d7dae0" text-anchor="middle">all_objects()</text>
    <g stroke="#6fb3ff" stroke-width="1.1">
      <line x1="150" y1="13" x2="200" y2="-2" transform="translate(0,14)" marker-end="url(#ah64)"/>
      <line x1="150" y1="13" x2="200" y2="13" marker-end="url(#ah64)"/>
      <line x1="150" y1="13" x2="200" y2="28" transform="translate(0,-2)" marker-end="url(#ah64)"/>
      <line x1="150" y1="13" x2="200" y2="46" transform="translate(0,-6)" marker-end="url(#ah64)"/>
    </g>
    <g fill="#d7dae0" font-size="10">
      <text x="206" y="8">order / rows</text><text x="206" y="30">build</text><text x="206" y="48">world boxes</text><text x="206" y="66">pick</text>
    </g>
    <text x="75" y="95" fill="#666" font-size="10">a new geometry source is ONE arm here —</text>
    <text x="75" y="109" fill="#666" font-size="10">no map can be forgotten again</text>
  </g>
  <defs><marker id="ah64" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/scene.rs   # ObjRef + all_objects(); the four maps refactor onto it; trimmed arms
```

## Step 1 — the iterator: `src/app/scene.rs`

A tiny enum of borrowed references unifies the sources. This is the structural payoff of the phase —
compare adding a type before (edit four functions, forget one, ship a bug) and after (add one arm):

```rust
/// Every renderable object in the document, whatever container it lives in.
pub enum ObjRef<'a> {
    Geom(&'a Geometry),                                  // lookup: Mesh/BRep/Line/Polyline/Point
    Curve(&'a NurbsCurve),                               // objects.nurbscurves (60)
    Surface(&'a NurbsSurface),                           // objects.nurbssurfaces (61)
    Trimmed(&'a NurbsSurfaceTrimmed),                    // objects.nurbssurfacetrimmeds (NEW)
}

impl ObjRef<'_> {
    pub fn guid(&self) -> &str {
        match self {
            ObjRef::Geom(g) => g.guid(),
            ObjRef::Curve(c) => c.guid(),
            ObjRef::Surface(s) => s.guid(),
            ObjRef::Trimmed(t) => t.guid(),
        }
    }
}

impl Scene {
    /// THE registration point. Every map iterates this; a new geometry source is one new chain link.
    pub fn all_objects(&self) -> impl Iterator<Item = ObjRef<'_>> {
        self.session.lookup.values().filter(|g| is_renderable(g)).map(ObjRef::Geom)
            .chain(self.session.objects.nurbscurves.iter().map(ObjRef::Curve))
            .chain(self.session.objects.nurbssurfaces.iter().map(ObjRef::Surface))
            .chain(self.session.objects.nurbssurfacetrimmeds.iter().map(ObjRef::Trimmed))
    }
}
```

## Step 2 — the maps consume it: `src/app/scene.rs`

Mechanical refactor, worth doing carefully once:

- **`Scene::new` (order/rows):** one loop over `all_objects()` replaces the lookup loop + 60's and
  61's bolted-on loops.
- **`build` / `apply_object`:** one `match ObjRef` whose arms are the existing bodies — `Geom` keeps
  35/38b's inner match; `Curve` is 60's sampling arm; `Surface`/`Trimmed` go through the cache.
- **`world_obb`:** `match ObjRef` — `Trimmed` boxes via its cached mesh (`AABB::from_mesh` on the
  tessellation + xform bake, 36's recipe; the kernel's `from_nurbssurface` sampler doesn't know about
  trims, and the *cached mesh* is what's actually on screen anyway).
- **pick:** `Surface`/`Trimmed` resolve through `render_mesh` (63); `Curve` keeps 60's segment test.

`hidden`, selection, and the gumball never change — flags and `guid_to_row` again.

## Step 3 — the trimmed arms themselves: `src/app/scene.rs`

The kernel makes this almost free: `NurbsSurfaceTrimmed::mesh()` is `mesh_q(20.0, 0.005)` — the
deflection-refined trimmed tessellator (holes and cut boundaries already honored in the triangle
layout). Cache it like 61; its linework is the **trim boundary loops** — sample the trim curves, not
the untrimmed domain rectangle:

```rust
    // render_mesh (63) gains the source arm:
    } else if let Some(ts) = self.session.objects.nurbssurfacetrimmeds.iter().find(|t| t.guid() == guid) {
        let ri = self.guid_to_row[guid];
        Some((ts.mesh(), trimmed_linework(ts, ri)))     // boundary loops via 60's sampler over the trim curves
    }
```

(`trimmed_linework` walks the trim's boundary curves — check the exact accessor on your kernel's
`NurbsSurfaceTrimmed` (the loops that `mesh_q` itself consumes) and feed each through `sample_curve` →
tubes, like 63's `brep_linework`. Iso lines from 62 apply too if you want the interior grid — clip
them mentally or skip; the boundary is what must be right.)

Transforms: matrix-only, same list — `Mesh | BRep | NurbsSurface | NurbsSurfaceTrimmed`. Reconcile's
`changed` bucket invalidates its cache entry. Both are one-word edits to 61/63's arms.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Load the trimmed-surface fixture (the circle-trim test model):

- The trimmed surface renders **with its hole** — you can see through it, and the hole's rim wears a
  boundary tube. Picking through the hole hits what's *behind* (the tessellation genuinely has no
  triangles there — this is the trim honored end to end, not painted over).
- The full audit one last time — draw, pick, marquee, hide, `F`, gumball, undo — all green for all
  four sources. Then the regression test the archive kept failing: **grep the four maps for a
  geometry-type name** — they all just iterate `all_objects()`; there is no list left to forget.
- Add-a-type drill (thought experiment or really do it for `PointCloud`): one `ObjRef` variant, one
  arm per `match` — the compiler's exhaustiveness check now *finds* every map for you. That compiler
  assist is the whole reason for the enum.

## Recap

```
Ch 63: BRep — debts paid.
Ch 64: TRIMMED + STRUCTURE. NurbsSurfaceTrimmed::mesh() (= mesh_q(20°, 0.005), holes honored in the
       tessellation) joins the cache; linework = the TRIM boundary loops sampled to tubes (not the
       untrimmed domain rectangle); box from the cached mesh (the kernel's surface sampler is
       trim-blind); matrix-only + reconcile invalidation, one word each. THE REFACTOR: ObjRef +
       all_objects() — lookup ∪ nurbscurves ∪ nurbssurfaces ∪ nurbssurfacetrimmeds — becomes the ONE
       registration point; order/build/boxes/pick all iterate it, and match-exhaustiveness makes the
       compiler find every map when a type is added. The archive's forgot-the-trimmed bug class is
       structurally extinct. Phase 10 complete: lines, curves, surfaces, solids, trims — all first-class.
```

Edited: `app/scene.rs` (`ObjRef`, `all_objects`, four-map refactor, trimmed cache/linework/box/pick
arms, transform + invalidation one-worders).

## Next

`65-ground-grid.md` — Phase 11: rendering quality, engineered fast. First the stage: an analytic
ground plane with distance fade (per-pixel ray∩plane in the fragment shader — never a giant quad, it
flickers) and the infinite fragment-shader grid.
