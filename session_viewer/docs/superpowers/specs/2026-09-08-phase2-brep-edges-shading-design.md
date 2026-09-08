# Phase 2: BRep edges and faces that agree, and smooth shading

2026-09-08. The user's scope: give this extra effort over several sessions and do it the way
modern CAD kernels do. Two deliverables: (1) a BRep or NURBS boundary curve never overlaps or
fights its face tessellation, because edge and faces share one discretisation; (2) BReps and
NURBS surfaces shade smooth, not faceted. This supersedes section 6 of
`2026-09-07-ink-visibility-design.md`.

## 1. What the kernel does today (measured in the source)

- `face_meshes_q` (identical in `session_rust/src/brep.rs:1302`, `session_cpp/src/brep.cpp:877`,
  `session_py/src/session_py/brep.py:939`) meshes a face in one of two ways: a grid over the
  whole surface domain (`RemeshNurbsSurfaceGrid::from_u_v_q`, emits `u`/`v` attributes and
  analytic normals) or a trimmed CDT (`NurbsSurfaceTrimmed::mesh_q`, emits normals, no `u`/`v`).
- Boundaries are sampled from each face's own pcurve (`divide_by_count` on `m_curves_2d`); the
  edge's 3D curve is never used by the mesher. Points are shared only between a grid face and
  a bilinear CDT face (`edge_bnd`, reprojected affinely and re-evaluated); grid-grid and
  CDT-CDT seams on curved surfaces are sampled twice, differently.
- `NurbsSurfaceTrimmed` takes its loops as UV `NurbsCurve`s and refines them by 3D deflection
  (`disc_loop`), so a loop's vertices are placed by the mesher, not by the caller.
- The viewer (`walk_brep`) welds every face mesh by position (`Mesh::from_polylines`), which
  drops the normals and the `u`/`v` attributes, so the shader falls back to flat derivative
  normals; then it draws `m_edges` from a third, independent sampling of the 3D curves.
- The trimmed mesher's `mesh_render` and its cut-plane fields exist in Rust only; C++ and
  Python lack them (a parity gap to close or to avoid depending on).

## 2. The design

### 2.1 One polygon per edge (kernel)

- `BRep::edge_polygons_q(quality) -> Vec<Vec<Point>>` (and `edge_polygons()` with the kernel
  default): edge `i`'s 3D curve `m_curves_3d[curve_3d_index]` sampled by turning angle and chord
  (`to_polyline_adaptive`, today used only by tests), endpoints exactly the edge's vertices,
  empty for a degenerated edge. Cached inside one `face_meshes_q` call.
- Every face builds its wire loops from those polygons: for each edge use, the parameters of
  the polygon's samples are mapped through the face's pcurve to UV (reversed for a Reversed
  use); the loop handed to the mesher is the UV polyline of those points, with refinement OFF
  along loops (`disc_loop` keeps the caller's vertices). After meshing, every boundary vertex's
  3D position is overwritten with the polygon's point by index, so all faces share the edge's
  points bit for bit; the surface normal at that vertex is kept from the face's own surface.
- Grid faces stop being special: an untrimmed face also goes through the loop path with the
  grid's interior nodes inserted as Steiner points (the CDT already accepts interior refinement
  by deflection; interior nodes come from the same span subdivision `from_u_v_q` uses).
- Boundary vertices carry `edge` (index) and `t` (parameter) attributes so a consumer can walk
  a face mesh's border per BRep edge.
- Parameterisation: pcurves follow the edge's direction (OCCT SameParameter convention already
  noted at `brep.rs:87`); where a pcurve's domain differs from the 3D curve's, parameters are
  mapped linearly between domains. Tolerance never enters: the 3D point is the edge's.

### 2.2 Faces keep their own vertices and normals (viewer)

- `walk_brep` no longer welds. Each face mesh is uploaded with its own vertices, its analytic
  normals (`nx/ny/nz` reach `to_render`), and `FLAG_SMOOTH`; the object row is one BRep as
  today. A reversed face is already flipped by the kernel.
- BRep edges become pipes: one row per polygon segment with `facing` from the two adjacent
  facets' normals (found by the `edge`/`t` attributes, exact adjacency), so the vertex-stage
  cull removes back edges as for meshes; no `FACING_UNKNOWN` ribbons for BReps.
- NURBS surfaces: `walk_surface` keeps the grid path's normals (it already does); its border
  edges come from the same attributes.
- The facing cull's `closed` test uses the BRep's own topology (`is_solid`), not a weld.

### 2.3 Smoothness where the tessellation is coarse

- The 5-degree quality stays; with true per-vertex normals a 5-degree facet reads as a smooth
  gradient under the headlight. The silhouette stays polygonal at the facet count; a finer
  angle is a display decision measured on frame time, not taken here.

## 3. Ports and tests

- C++ first, then Rust and Python, identical names, logic and line counts (CLAUDE.md). New
  minitests, one per method, identical across languages: `edge_polygons` (count equals
  `m_edges`, endpoints are the edge's vertices, degenerated empty); `face_meshes` boundary
  vertices equal the polygon's points exactly and carry `edge`/`t`; `mesh()` of box,
  cylinder, cone, sphere, torus, block-with-hole is closed after the 1e-6 weld; the existing
  `Mesh` and `Mesh Orientation` tests keep passing with re-measured expectations.
- `mesh_render`'s Rust-only fields are either ported or no longer relied on by the viewer.

## 4. Verification in the viewer

- The mixed-solids scene: every BRep edge draws at full width from every orbit (the orbit
  check's ink series on the sphere and torus, measured before and after); the census, the
  matrix and the joint probe unchanged.
- A new probe: a hidden line behind the BRep cylinder (the case the ink suite never covered),
  zero magenta at 1x and 4x.
- Smoothness: a sphere's shading scanline is monotonic between silhouette and highlight with
  no per-facet steps (measured as the maximum second difference along the scanline).

## 5. Sessions

1. Kernel design detail and the C++ implementation with its minitests.
2. Rust and Python ports, parity audit (`/sync`), submodule bumps, CI green.
3. Viewer: unwelded faces with normals, edges as pipes, the probes, docs.
