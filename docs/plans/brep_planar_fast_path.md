# BRep from polylines with holes, at mesh-loft speed

Date: 2026-09-17. Status: planned, not started.

## The measurement that motivates it

Example 3 of wood (inplane_hexshell, 23 solved plates, 1318 planar faces in total), building the
same solid twice from the merged outlines:

| step | time |
|---|---|
| `Mesh::loft` of all 23 plates | 7 ms |
| `BRep::from_polylines` of the 23 solids | 9.3 s |
| `BRep::mesh()` on them, what the viewer pays at load | 9.3 s |

Scaling probe on one plate's side quads (`from_polylines` on 4, 8, 16, 24 open faces, no shell):

| faces | from_polylines | face_meshes |
|---|---|---|
| 4 | 0.1 ms | 23 ms |
| 8 | 0.0 ms | 38 ms |
| 16 | 0.1 ms | 76 ms |
| 24 | 0.1 ms | 115 ms |

So the builder itself is linear and cheap (0.4 ms for one hundred single-quad BReps). All the
time is in `face_meshes_q`, about 5 ms per planar face, and `from_polylines` only looks slow
because `close_free_faces` calls `face_meshes()` to orient each shell by its signed volume.
Vertex matching (`find_or_add_vertex`, a linear scan) is a second-order cost that matters only
past a few thousand vertices.

## Why a planar face costs 5 ms

`face_meshes_q` treats every face as a trimmed NURBS surface: `direct_face` fails for a face
whose wire is not the surface's natural boundary, so the face goes through `trim_loops`
(every edge sampled by angle and chord against the NURBS, projected to uv), then
`NurbsSurfaceTrimmed::mesh_loops` (grid, constrained Delaunay, uv to xyz evaluation), then
`tag_edge_uses`. For a planar patch bounded by straight edges every one of those steps is
redundant: the loop vertices are the tessellation input, the plane is the parameterisation,
and one triangulation of a polygon with holes is the answer.

## Plan

Three changes in the kernel, in all three languages, each with its own minitest. Wood then
gets `Plate::model_brep()` for free.

### 1. `BRep::from_polylines(polylines, holes)` — faces with inner wires

An overload of the existing `from_polylines`: `holes[i]` are the closed polylines that bound
the holes of face `i`, empty by default. It mirrors `from_nurbscurves(curves, holes)`, which
already does this for curves, so the wire code is the same: one outer wire, one wire per hole,
each hole edge with its pcurve on the face's planar patch, `add_face(si, {outer, hole...})`.
Shared vertices and edges are found through the same builder maps, so a hole edge on the
bottom face and the side quad standing on it share their edge, and `close_free_faces` sees a
closed sheet with a through-hole as one solid.

- C++: `static BRep from_polylines(const std::vector<Polyline>& polylines, const std::vector<std::vector<Polyline>>& holes);`
- Python: `from_polylines_holes(polylines, holes)`; Rust: `from_polylines_holes(&[Polyline], &[Vec<Polyline>])`.
- Test "From Polylines Holes": a box whose top and bottom carry a square hole and four inner
  side quads; expect one solid, 10 faces, volume = outer minus hole, `is_closed()`.

### 2. A planar fast path in `face_meshes_q`

Before the NURBS path, per face:

```
if surface.is_planar(&plane) and every edge of every wire is a degree-1 curve:
    loops = the wire vertices in wire order, xyz         (no sampling)
    uv    = plane.to_uv(loops)                            (one dot product each)
    mesh  = Mesh::from_polygon_with_holes(loops)          (outer first, then holes)
    tag_edge_uses(mesh, loops, uses)                      (one segment per edge)
    flip when the face is Reversed, as today
```

`Mesh::from_polygon_with_holes` already exists in all three kernels and takes xyz loops. The
fast path emits exactly one vertex per wire vertex and one triangle fan or constrained
triangulation per face, so the plate above becomes 1318 faces × a few microseconds. Faces with
curved edges or curved surfaces keep the current path unchanged; the quality parameters do
not apply to a planar face with straight edges, which is why the two paths agree on the
boundary and `refine_shared_boundaries` has nothing to refine there.

- Test "Planar Fast Path": the box from test 1 tessellated by both paths (force the NURBS path
  through a debug flag or by comparing against `from_polygon_with_holes` directly); same face
  count, same area per face, same signed volume, and the fast path under 1 ms for the box.
- Test "Mesh Orientation" (existing) must still pass: the fast path flips Reversed faces the
  same way.

### 3. `close_free_faces` without a tessellation

Shell orientation needs only the signed volume of each shell. For a shell whose faces are all
planar with straight edges, the signed volume is the sum over faces of the polygon area
vector dotted with a vertex, straight from the wire loops; no mesh. Faces on the slow path
keep the current `face_meshes()` call, restricted to the shells that need it.

- Test "Close Free Faces Planar": `from_polylines` on the plate solid must not call the
  tessellator (assert through a counter in debug builds, or by timing under 1 ms).

### Wood, once 1 to 3 are in

`Plate::compute_model_brep()`: bottom face with holes from `features.bottom`, top face with
holes from `features.top`, one quad per outer edge and per hole edge, through
`BRep::from_polylines(faces, holes)`. Cached like `model_geometry()`, lazy, invalidated by the
merge. `WoodSession::pb_dump` keeps writing the mesh; the BRep is opt-in, because the viewer
still tessellates a BRep at load and the file has no mesh cache.

## What this does not do

- No new surface type. A planar face stays a bilinear NURBS patch in the pools and on the
  wire; the fast path only recognises it. Adding a `Plane` surface kind would shrink the file
  but changes the proto and all three readers, a separate decision.
- No spatial index for vertex matching. The linear scan is fine up to a few thousand vertices
  per BRep; a hash grid is a one-function change when a dataset needs it.
- No change to `Mesh::loft`. It stays the default plate geometry.

## Expected result

For the example above: `from_polylines` with holes about 5 ms for 23 plates, `mesh()` about
10 ms, against 7 ms for `Mesh::loft` today. The viewer's load cost for a BRep scene falls by
the same factor since it runs the same `face_meshes_q`.

## Order of work

Python, then Rust, then C++, per class, `./bash/quicktest.sh brep --py|--rust|--cpp` after
each, `./bash/minitest.sh --no-web` before the commit, one commit per kernel and one pointer
bump in session, then CI green on all four repos before wood's `model_brep()`.
