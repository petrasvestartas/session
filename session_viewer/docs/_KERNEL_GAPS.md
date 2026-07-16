# Kernel gaps the viewer tutorials exposed

Writing lessons 01–77 meant pressure-testing the kernel API from a consumer's seat. Every place a
tutorial had to *work around* the kernel instead of *calling* it is recorded here — ranked by how
much viewer code the kernel fix would delete. Two are already fixed; the rest are proposals.

Legend: ✅ fixed · 🔴 high value · 🟡 worthwhile · ⚪ note

---

## ✅ 1. `Xform::inverse()` was affine-only (all three languages) — FIXED

It inverted the 3×3 + translation and silently assumed a `[0,0,0,1]` bottom row — wrong for any
projective matrix. Lesson 41's screen-ray unprojection through `inverse(view_proj)` was broken by it
(the archive hit the identical bug and shipped its own `mat4_inverse` as a workaround).
**Fix shipped:** full cofactor 4×4 inverse, identical implementation in py/rust/cpp, plus a
perspective `P·P⁻¹ = I` check in the Inverse minitest. Affine inputs return the same results as
before; projective inputs are now correct.

## ✅ 2. Python `Xform.identity()` returned a mutable singleton — FIXED

`_identity_cache` meant `p = Xform.identity(); p.m[0] = 2.0` silently corrupted **every** later
`identity()` call — including inside `is_identity()` and any code building from identity. Rust/C++
return fresh values; Python diverged. **Fix shipped:** cache removed, fresh instance per call.
(Found because the new inverse test mutated the "identity" it was handed — the minitest caught a
second bug while verifying the first.)

## 🔴 3. Placement semantics are inconsistent: `mesh.xform` is sometimes honored, sometimes not

The kernel's own code disagrees with itself about what `Mesh.xform` means:

- `Session::compute_bounding_box`: the **BRep** arm transforms `m_vertices` by `b.xform`; the
  **Mesh** arm reads raw local vertices and ignores `m.xform`.
- `Session::ray_cast`: the Mesh arm casts against local vertices — placement-blind.
- `Mesh::to_render()`: ignores `xform` (the viewer relies on this: xform = instance model).

Consequences in the tutorials: lesson 36 builds its own world boxes (`world_obb` bakes the xform);
lesson 44 must *filter mesh hits out* of `Session::ray_cast` and keep a parallel viewer-side mesh
ray-cast (42). **Proposal:** pick one contract — "`xform` is the placement, all spatial queries apply
it" — and sweep `compute_bounding_box` / `ray_cast` / any AABB helpers to honor it (BRep already
does). This deletes the viewer's `world_obb` special-casing and makes `Session::ray_cast` usable as
the *whole* pick backend.

## 🔴 4. NURBS types are second-class: not `Geometry` variants, parallel collections

`NurbsCurve` / `NurbsSurface` / `NurbsSurfaceTrimmed` live in `session.objects.*` vectors, outside
`lookup: HashMap<guid, Geometry>`. Every consumer must remember **two sources** — the archive forgot
repeatedly ("draws but won't pick", "missing from the tree"), and lessons 60/64 had to build
`ObjRef` + `all_objects()` purely to compensate. `Session::remove_object` doesn't remove them;
`ray_cast` never sees them; reconcile (38b) needs extra arms.
**Proposal:** add the three variants to `Geometry` and register them in `lookup` like everything
else. This is the biggest-ticket item (touches the proto schema + serialization + all three
languages + existing fixtures), so it's a design decision, not a patch — but it retires an entire
bug class at the root instead of at every call site.

## 🔴 5. No `Xform × Point` — transforming a point requires the carry-xform idiom

The only way to apply a transform is `p.xform = xf; p.transformed()` — mutate-a-field-then-call, with
a clone per use. The viewer hand-rolls row-dot `M·v` in three places (41 unproject, 43 projection,
65's uniform prep), and every `world_obb`/`to_local` in lessons 36/42/43 pays the clone-and-carry
dance. **Proposal:** `Xform::transform_point(&Point) -> Point` and `transform_vector(&Vector) ->
Vector` (plus optionally `impl Mul<&Point> for &Xform`) — pure functions, no field mutation, ported
×3. Cheap to add; large ergonomic payoff (the kernel's own BRep bbox code already hand-rolls exactly
this loop — it would use it too).

## 🟡 6. No deterministic content fingerprint on `Geometry`

Reconcile (38b) and save-gating (39) need "did this object change?". `{:?}` is unusable — `Mesh`
stores vertices/faces in `HashMap`s whose Debug order is randomized per instance (every object would
read as changed every load). The tutorials hash the *sorted* `jsondump`, but that's per-type: `BRep`
has **no `jsondump`**, so 38b fingerprints it via `mesh() + xform` — expensive and lossy.
**Proposal:** `Geometry::jsondump()` (uniform dispatch, BRep included) and/or a kernel
`content_hash() -> u64` over canonical bytes. Also useful for the C++/Python parity tests themselves.

## 🟡 7. `BRep::mesh()` re-tessellates on every call

The kernel comments on it itself (`ray_cast`: "BRep tessellation is expensive… viewers must use
pre-cached tessellations"). The viewer built a guid-keyed cache (lessons 61/63); every other consumer
(C++, Python, future tools) must rebuild the same cache. **Proposal:** cache the render mesh on the
BRep (the `Mesh::to_render`/`invalidate` pattern already exists in-kernel), invalidated by mutating
ops. Deletes the viewer's `render_mesh` plumbing and fixes `Session::ray_cast`'s skipped-BRep arm.

## 🟡 8. No generic `Session::add_geometry(Geometry)`

Removal is generic (`remove_object(guid)`), insertion is per-type (`add_mesh`, `add_line`, …). Undo
(51) and reconcile-restore paths need a hand-written variant match (`restore_geometry`) that must be
maintained as types grow. **Proposal:** one `add_geometry(geom: Geometry, parent) -> node` that
dispatches internally — the exact inverse of `remove_object`, next to it.

## 🟡 9. `Mesh` ray-cast requires `&mut` for a read

`triangle_bvh_ray_cast(&mut self, …)` — lazy BVH build via plain mutation — forces the viewer's
picking to take `get_mut` on the session and infects `pick_ray(&mut self)` (42). The guid field
already solves this exact problem with `OnceLock`. **Proposal:** interior mutability for the cached
triangle BVH (`OnceLock`/`RefCell` per language convention) so ray queries take `&self`. Same for
`Session::ray_cast`'s cached BVH.

## ⚪ 10. Smaller notes

- **`OBB::from_nurbssurface` is trim-blind** — fine for untrimmed, but `NurbsSurfaceTrimmed` boxes
  must come from the tessellation (lesson 64 does); a trimmed-aware overload would centralize it.
- **`Point` weight/color/xform on every instance** makes bulk sampling (curve/iso tessellation)
  allocation-heavy; a lean `[f64;3]` sampling path (`point_at_into(&mut [f64;3])`?) would help hot
  loops. Matches the parked `alloc-free point_at` idea in the viewer perf plan.
- **`Session::ray_cast` force-adds *all* thin geometry as candidates** (documented workaround for
  degenerate boxes). With a tolerance-inflated BVH query this collapses back into the broad-phase —
  worth revisiting if pick latency ever matters at >100k objects.
- **`Xform::to_cols()` naming** — it returns column-major `m[col][row]`; the tutorials must re-state
  that at every use. A doc comment with the exact indexing convention on the method would save every
  future consumer one experiment.

---

### How this list was made

Each entry traces to a concrete tutorial workaround: 36 (`world_obb`), 38b (`content_hash`), 41
(`mat4_inverse` — now deleted), 42 (`&mut` picking), 44 (mesh-hit filtering), 51
(`restore_geometry`), 60/64 (`all_objects`), 61/63 (tess cache). If a kernel change lands, delete
the corresponding viewer workaround **in the lesson too** — the tutorials should always show the
best current way, not the historical one.
