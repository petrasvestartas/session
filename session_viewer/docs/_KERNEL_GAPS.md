# Kernel gaps the viewer tutorials exposed

Writing lessons 01–77 meant pressure-testing the kernel API from a consumer's seat. Every place a
tutorial had to *work around* the kernel instead of *calling* it is recorded here — ranked by how
much viewer code the kernel fix would delete. Two are already fixed; the rest are proposals.

Legend: ✅ fixed · 🔴 high value · 🟡 worthwhile · ⚪ note

---

## ✅ 1. `Xform::inverse()` was affine-only (all three languages) — FIXED

It inverted the 3×3 + translation and silently assumed a `[0,0,0,1]` bottom row — wrong for any
projective matrix. Lesson 51's screen-ray unprojection through `inverse(view_proj)` was broken by it
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

## ✅ 3. Placement semantics unified: `mesh.xform` is the placement, everywhere — FIXED

The kernel used to disagree with itself: `compute_bounding_box`'s BRep arm applied `b.xform` while
its Mesh arm read raw local vertices; `Session::ray_cast`'s Mesh arm was placement-blind.
**Fix shipped (all three languages):** the Mesh box arm bakes `m.xform` via `transform_point`; the
Mesh ray arm inverse-transforms the ray into the mesh's local frame, casts against the cached
triangle BVH, and returns the hit in world coordinates. The Ray Cast minitest gained a placed-mesh
check (translated mesh, ray at the new location, world hit asserted) in py/rust/cpp. Bonus finds
while sweeping: C++ had **no BRep box arm at all** (fell to a degenerate origin box — added), and
**both C++ and Python `remove_object` only removed `Point`** from the typed collections, so deleted
objects of any other type resurrected on save — both completed to Rust parity.

## ✅ 4. NURBS types as `Geometry` variants — FIXED for curve + surface (Trimmed remains)

The audit's premise turned out sharper than expected: **C++ (ground truth) already had
`NurbsCurve`/`NurbsSurface` in its `Geometry` variant and full `add_nurbscurve`/`add_nurbssurface`
methods, and Python registered them in `lookup` too — only Rust was behind.** And `lookup` is
`#[serde(skip)]` (derived from `objects.*` on load), so no proto change was ever needed.
**Fix shipped:** Rust `Geometry` gained the two variants; compiler-guided sweep covered guid
dispatch, all three lookup-rebuild sites, `remove_object`, `compute_bounding_box` (CV-hull arms, C++
parity), `ray_cast` (explicit skip arms, C++ parity), the transformed-lookup walk, and new
`add_nurbscurve`/`add_nurbssurface` methods + "Add Nurbscurve"/"Add Nurbssurface" minitests (ported
from py/cpp, which already had them).
**Remaining:** `NurbsSurfaceTrimmed` is still collection-only in *all three* languages — adding it to
the C++ variant + ports is the follow-up; until then the viewer's `all_objects()` (lesson 69) still
earns its keep for Trimmed.

## ✅ 5. `Xform::transform_point` / `transform_vector` — FIXED

Transforming a point used to require the carry-xform idiom (`p.xform = xf; p.transformed()` — a
clone and a field mutation per use); the viewer hand-rolled row-dot `M·v` in three lessons.
**Fix shipped (all three languages):** `transform_point(&Point) -> Point` (full homogeneous multiply,
divides by `w` when projective — so it's also correct through perspective matrices) and
`transform_vector(&Vector) -> Vector` (rotation/scale only — translation doesn't move directions).
Two new minitests each in py/rust/cpp ("Transform Point" incl. a projective w-divide check,
"Transform Vector" incl. the translation-doesn't-move-vectors semantic). The kernel dogfoods it:
`compute_bounding_box`'s BRep arm replaced its hand-rolled loop, and gap #3's fixes are built on it.

## 🟡 6. No deterministic content fingerprint on `Geometry`

Reconcile (43b) and save-gating (44) need "did this object change?". `{:?}` is unusable — `Mesh`
stores vertices/faces in `HashMap`s whose Debug order is randomized per instance (every object would
read as changed every load). The tutorials hash the *sorted* `jsondump`, but that's per-type: `BRep`
has **no `jsondump`**, so 38b fingerprints it via `mesh() + xform` — expensive and lossy.
**Proposal:** `Geometry::jsondump()` (uniform dispatch, BRep included) and/or a kernel
`content_hash() -> u64` over canonical bytes. Also useful for the C++/Python parity tests themselves.

## 🟡 7. `BRep::mesh()` re-tessellates on every call

The kernel comments on it itself (`ray_cast`: "BRep tessellation is expensive… viewers must use
pre-cached tessellations"). The viewer built a guid-keyed cache (lessons 66/63); every other consumer
(C++, Python, future tools) must rebuild the same cache. **Proposal:** cache the render mesh on the
BRep (the `Mesh::to_render`/`invalidate` pattern already exists in-kernel), invalidated by mutating
ops. Deletes the viewer's `render_mesh` plumbing and fixes `Session::ray_cast`'s skipped-BRep arm.

## 🟡 8. No generic `Session::add_geometry(Geometry)`

Removal is generic (`remove_object(guid)`), insertion is per-type (`add_mesh`, `add_line`, …). Undo
(56) and reconcile-restore paths need a hand-written variant match (`restore_geometry`) that must be
maintained as types grow. **Proposal:** one `add_geometry(geom: Geometry, parent) -> node` that
dispatches internally — the exact inverse of `remove_object`, next to it.

## 🟡 9. `Mesh` ray-cast requires `&mut` for a read

`triangle_bvh_ray_cast(&mut self, …)` — lazy BVH build via plain mutation — forces the viewer's
picking to take `get_mut` on the session and infects `pick_ray(&mut self)` (47). The guid field
already solves this exact problem with `OnceLock`. **Proposal:** interior mutability for the cached
triangle BVH (`OnceLock`/`RefCell` per language convention) so ray queries take `&self`. Same for
`Session::ray_cast`'s cached BVH.

## 🔴 11. STEP codec is C++-only

`file_step` exists only in `session_cpp` — no Rust or Python port — so the wasm viewer cannot read
or write STEP at all (lesson 84 wires the dispatch arm to warn loudly instead). This is the largest
remaining gap by user impact: STEP is *the* CAD exchange format. C++ is ground truth; the port is a
real project (the reader alone is substantial), not an afternoon.

## ✅ 12. `file_obj` was path-based only — FIXED

`read_file_obj(filepath)` / `write_file_obj(mesh, filepath)` hit `std::fs` — dead on wasm. Fixed
with the `_from_str` / `_to_string` pair (the same split the Session codecs always had), path
functions now delegating, ×3 languages + a "String Roundtrip" minitest ×3. Enabled lesson 84.
Follow-up noted: the writer takes ONE mesh; multi-object OBJ (`o name` groups) is a small extension.

## ✅ 13. No way to give a cloned object a fresh guid — FIXED

Guids are lazily-minted `OnceLock`s; `Clone` copies the minted value and `set_guid` is
`let _ = lock.set(g)` — a **silent no-op** on any clone. A copied object therefore *was* its
original to `lookup`, undo, and reconcile — inserting it overwrote the source. Fixed with
`refresh_guid(&mut self)` (clear the lock → fresh guid mints lazily) on the 7 geometry types,
×3 languages (`_guid.clear()` in C++, `_guid = None` in Python) + a "Refresh Guid" minitest ×3.
Enabled lesson 90's copy/array/Alt-drag. Bonus find: C++ `Mesh`'s hand-written copy
constructor/assignment (it exists for the BVH caches) *cleared* `_guid` on copy — the only type in
any language where a copy silently changed identity. Now copies the guid like everything else;
`refresh_guid` is the one explicit way to re-mint.

## ⚪ 10. Smaller notes

- **`OBB::from_nurbssurface` is trim-blind** — fine for untrimmed, but `NurbsSurfaceTrimmed` boxes
  must come from the tessellation (lesson 69 does); a trimmed-aware overload would centralize it.
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
