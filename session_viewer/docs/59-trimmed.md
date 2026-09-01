# 59 Trimmed surfaces — first-class, and the every-map rule becomes structure

> **Big picture.** *The geometry block closes.* `NurbsSurfaceTrimmed` — a surface with cut
> boundaries and holes — is today's one genuinely INVISIBLE type: it has no `Geometry` variant,
> so it lives only in `session.objects.nurbssurfacetrimmeds`, and the walk (`for guid in
> session.order()` → `lookup.get`) never sees it. That is exactly the shape of the archive's
> worst bug class — it drew but didn't pick, picked but missed the tree, appeared everywhere
> except the visibility map, because several maps were maintained by hand and a fifth geometry
> source was forgotten. This lesson adds the trimmed surface *and retires the bug class*: one
> `all_objects()` iterator becomes the single place a geometry source is registered — the maps
> that exist consume it, and every LATER map (the BVH's boxes in 52, picking in 55) is born
> consuming it, so nothing can be forgotten again.

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
      <text x="206" y="8">order / rows</text><text x="206" y="30">build</text><text x="206" y="48">world boxes (52)</text><text x="206" y="66">pick (55)</text>
    </g>
    <text x="75" y="95" fill="#666" font-size="10">a new geometry source is ONE arm here —</text>
    <text x="75" y="109" fill="#666" font-size="10">no map can be forgotten again</text>
  </g>
  <defs><marker id="ah64" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/scene.rs   # ObjRef + all_objects(); the walk loop consumes it; the Trimmed arm
                   # (cache + trim-loop linework)
```

## Step 1 — the iterator: `ObjRef` + `all_objects`

Two sources, one stream, DETERMINISTIC order — `session.order()` first (the kernel's canonical
cross-language order; a guid's row here is the row it keeps), then the trimmed collection in
insertion order (`lookup.values()` would randomize rows run to run — never iterate a HashMap
into row space):

```rust
/// One object from either source. A new geometry source = one variant here, and the
/// compiler's exhaustiveness check finds every consumer for you.
enum ObjRef<'a> {
    Geom(&'a Geometry),
    Trimmed(&'a Rc<NurbsSurfaceTrimmed>),
}

/// EVERY per-object map iterates this - order/rows, build, and each later map (52's boxes,
/// 55's pick) from birth. The single registration point for geometry sources.
fn all_objects(session: &Session) -> impl Iterator<Item = (String, ObjRef<'_>)> {
    session.order().into_iter()
        .filter_map(|g| session.lookup.get(&g).map(|geom| (g.clone(), ObjRef::Geom(geom))))
        .chain(session.objects.nurbssurfacetrimmeds.iter()
            .map(|ts| (ts.guid().to_string(), ObjRef::Trimmed(ts))))
}
```

(Imports: `use session_rust::nurbssurface_trimmed::NurbsSurfaceTrimmed;` and `Rc` if the file
doesn't have it. `order()` returns owned `Vec<String>` — the `filter_map` clones nothing extra.)

## Step 2 — the walk consumes it

**Find** the loop head in `add_file`:

```rust
        for guid in session.order() {
            let Some(geom) = session.lookup.get(&guid) else { continue };
            if let Geometry::Element(e) = geom {
                if matches!(e.geometry(), ElementGeometry::None){
                    continue
                }
            }
```

**Replace with:**

```rust
        for (guid, obj) in all_objects(&session) {
            if let ObjRef::Geom(Geometry::Element(e)) = &obj {
                if matches!(e.geometry(), ElementGeometry::None){
                    continue
                }
            }
```

and wrap the existing geometry `match` one level: `match obj { ObjRef::Geom(geom) => match geom
{ …the whole existing match, unchanged… }, ObjRef::Trimmed(ts) => { …Step 3… } }`. The row
bookkeeping above the match (`ri`, `flags`, `placed`, the `objects` push) is already
source-agnostic — `placement(&guid)` returns identity for a guid the xform tree doesn't know,
which is the right default for a collection-only type.

## Step 3 — the trimmed arm: cache + trim-loop linework

The kernel makes the skin almost free: `NurbsSurfaceTrimmed::mesh()` is `mesh_q(20.0, 0.005)` —
the deflection-refined trimmed tessellator, holes and cut boundaries already honored in the
triangle layout. The linework must NOT be 45's iso grid over the untrimmed rectangle — the
honest edges are the **trim loops themselves**: UV curves (`m_outer_loop` + `m_inner_loops`),
sampled in parameter space and LIFTED to 3D through the surface:

```rust
/// Trim boundary loops as tubes: sample each UV loop (43's sampler - a UV curve's points
/// come back as (u, v, 0)), lift through point_at(u, v), tube. instance stamped at push.
fn trimmed_linework(ts: &NurbsSurfaceTrimmed) -> Vec<CylinderSegment> {
    let dark = pack_rgba([0.05, 0.05, 0.05, 1.0]);
    let mut out = Vec::new();
    for lp in ts.m_outer_loop.iter().chain(ts.m_inner_loops.iter()) {
        let uvs = sample_curve(lp);
        let mut prev: Option<[f32; 3]> = None;
        for q in &uvs {
            let Some(p) = ts.point_at(q[0], q[1]) else { prev = None; continue };
            let p = p.to_f32();
            if let Some(a) = prev {
                out.push(CylinderSegment { p0: a, radius: 0.0, p1: p,
                                           instance_id: 0, color: dark, facing: FACING_UNKNOWN });
            }
            prev = Some(p);
        }
    }
    out
}
```

and the arm itself is 45/46's pattern verbatim:

```rust
                ObjRef::Trimmed(ts) => {
                    let (tm, lw) = self.tess_cache.entry(guid.clone()).or_insert_with(|| {
                        let mut m = ts.mesh();                      // mesh_q(20°, 0.005)
                        m.set_objectcolor(ts.surfacecolor.clone());
                        (m, trimmed_linework(ts))
                    });
                    let b = push_mesh(
                        tm,
                        ri,
                        vb,
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres,
                        Edges::Suppress
                    );
                    t.pipes.extend(lw.iter().map(|seg| {
                        let mut seg = *seg;
                        seg.instance_id = ri;
                        seg
                    }));
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, tm.number_of_vertices()));
                }
```

> **What `mesh_q(20.0, 0.005)` means — the trap by name.** The first argument is an *angle* (max
> 20° normal deviation per triangle); the second is an **absolute** deflection in kernel length
> units — 5 microns on a metre-unit file (absurdly tight), 5 mm on a millimetre file (visibly
> coarse on a small part). Same family as 43's `0.2`. The tessellation is cached so the cost is
> paid once — but if trimmed parts mis-refine in your unit of choice, the honest fix is a
> unit-relative tolerance, not a new global constant.

One honest caveat: there is no `Session::add_nurbssurfacetrimmed`, so trimmed surfaces are
**read-only first-class** — they arrive via loaded files, and every viewer map treats them fully;
creating one in the viewer waits on the kernel. When it grows that call, the slot is already
obvious: a 70-style tool committing `AddGeometry`, one `restore_geometry` arm (64) — and **zero**
map work, because `all_objects()` already knows the type.

## Step 4 — verify

```bash
cargo check --target wasm32-unknown-unknown --lib
```

Load a trimmed-surface fixture (the kernel's split/trim test data dumps one, or
`NurbsSurfaceTrimmed::create_planar` in a native example):

- The trimmed surface renders **with its hole** — you can see through it, and both the outer cut
  and the hole's rim wear boundary tubes that RIDE the surface (lifted through `point_at`, not
  drawn flat in UV). The tessellation genuinely has no triangles in the hole — trim honored end
  to end, not painted over.
- The regression test the archive kept failing, now structural: **grep the walk for a
  geometry-type name** — sources come only from `all_objects()`; there is no hand-list left to
  forget. Add-a-type drill (thought experiment): one `ObjRef` variant → the compiler's
  exhaustiveness check FINDS every consumer.
- Everything else is pixel-identical — the other arms didn't change, they just moved one match
  level down.

## Recap

```
Ch 46: BRep — debts paid.
Ch 47: TRIMMED + STRUCTURE. The one invisible type (no Geometry variant, collection-only,
       never walked) becomes first-class: mesh_q(20° / 0.005-ABSOLUTE deflection - make it
       unit-relative if your files mis-refine) into the shared tess_cache with surfacecolor
       baked; linework = the TRIM LOOPS (UV curves sampled with 43's sampler, lifted through
       point_at(u,v)) — not 45's iso grid over the untrimmed rectangle; bounds from push_mesh
       like every cached skin; placement defaults identity (collection guids are outside the
       xform tree). READ-ONLY first-class — no Session::add yet. THE REFACTOR: ObjRef +
       all_objects() = session.order()∪trimmed insertion order (deterministic rows; never
       HashMap order), the ONE registration point; the walk consumes it today, 52's boxes and
       55's pick are born consuming it. The forgot-a-map bug class is structurally extinct.
       The geometry block is complete: lines, meshes, points, clouds, curves, surfaces,
       solids, trims — every kernel type renders.
```

Edited: `app/scene.rs` (`ObjRef`, `all_objects`, the re-leveled walk match, `trimmed_linework`,
the Trimmed arm).

## Next

`60-gpu-arena.md` — every geometry type renders; now the session infrastructure: the flat GPU
arena becomes per-object and addressable, so reconcile (49), save (50) and watch (51) can make
the `.pb` file a live document. (The GPU-tessellation sequel to THIS block is 73–76, once the
arena and the interaction phases exist.)
