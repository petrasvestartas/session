# CAD representation and meshing decisions

This design record explains the shared geometry producer behind chapters 06–09. [Chapter 18](18-finite-visibility.md)'s finite-triangle correction changes screen-space visibility, not this contract.

## Producer contract

Session's producer is the independent C++/Rust/Python NURBS and constrained-Delaunay implementation in the sibling geometry packages; it does not call an OCCT runtime. The **OCCT V8_0_1** sources below are an implementation reference, not a claim of pixel comparison against an OCCT renderer.

| Actual OCCT reference | Relevant contract | Session implementation |
|---|---|---|
| [`StdPrs_ShadedShape.cxx`, `fillFaceBoundaries` and `fillTriangles`](https://github.com/Open-Cascade-SAS/OCCT/blob/V8_0_1/src/Visualization/TKV3d/StdPrs/StdPrs_ShadedShape.cxx) | Use topological edges and ordered polygon nodes belonging to the face triangulation; coordinate winding, face orientation and mirrored placement. | `session_{cpp,rust,py}` BRep `face_meshes_q`; viewer `app/walk/brep_edges.rs`, `brep_orient.rs`, `brep.rs`; `shaders/normals.wgsl` and `triangle.wgsl`. |
| [`BRepLib_ToolTriangulatedShape.cxx`, `ComputeNormals`](https://github.com/Open-Cascade-SAS/OCCT/blob/V8_0_1/src/ModelingAlgorithms/TKTopAlgo/BRepLib/BRepLib_ToolTriangulatedShape.cxx) | Preserve valid normals; derive them from the supporting surface and UVs, with a triangulation fallback when necessary. | Shared `remesh_nurbssurface_grid` and `nurbssurface_trimmed`: analytic normals, deterministic incident-face fallback at singularities, one-sided C0 normals and separate shading vertices. |

`TrimLoops` carries outer and inner UV polygons, the optional XYZ position each vertex lifts to, and optional interior seeds. Original nodes keep `boundary/{loop}/{sample}` provenance; added knot intersections keep `boundary_interval/{loop}/{segment}` fractions, polygon intervals and **not** CAD curve parameters. BRep turns that provenance into edge-table indices and repeated-use identities, and both shading copies at a true crease keep theirs. The viewer strokes boundaries from exact face-mesh nodes, and every subdivision of one edge keeps its source edge ID through `SegRows.pipe_ids` and `Scene::edge_at`. Unavailable provenance stays unavailable; an explicitly warned analytic fallback never manufactures a topology ID.

The first incident grid gives the canonical shared-edge polygon; a later grid whose boundary samples differ is rebuilt through the constrained mesher from that polygon and its previous interior UV seeds, sorted to remove map-order dependence. Shared boundaries take their angular/chord refinement before curved constrained faces refine interiors, and every incident face then gets that same polygon, its original samples exact. The producing face keeps its pcurve parameters; an adjoining face maps the shared XYZ onto its own pcurve and checks the lifted position against edge/face tolerances, falling back to a bounded one-dimensional search on that pcurve when surface inversion reaches the wrong periodic branch. Interior C0 knot lines stay constrained inside the trim, their shading normals taken from each incident side.

This fixes a geometry defect visible on the teapot: interior centroids refined onto the curved surface crowded the fixed coarse boundary chords and buried them, and sharing endpoint vertices was not enough. The teapot's bytes, GUID, 32 patches and 512 controls are unchanged — the existing Newell/GLUT Utah NURBS asset, with no established 3ds Max export provenance.

The additional [COMPAS OCC tessellation comparison](https://github.com/compas-dev/compas_occ/blob/8dc35a32e447bb053b236f0836c2a92d8900f784/src/compas_occ/brep/brep.py#L1232) confirms the same polygon-on-triangulation contract: boundary indices reuse face triangulation nodes. The viewer's facing test reads exact incident triangle normals, both uses of a periodic seam included, and never substitutes a shading normal at a singular pole. Planar BRep face normals stay independent across faces. Raw derivatives are checked before an analytic normal is accepted, so a +Z fallback sentinel cannot masquerade as a cone or sphere pole derivative.

Quality is a normal-angle target in degrees and a chord factor of the surface bounding-box diagonal: the viewer asks for `(5°, 0.001)`, the shared default is `(20°, 0.005)`. Interior refinement stops at eight passes and 200000 vertices; BRep boundary refinement stops at eight split levels and 4096 added nodes per edge, with the caller's original samples exact. These are bounded heuristics, not a certified global chord bound. `mesh_loops` returns an empty mesh for invalid input, lost original boundary provenance or an unconstrained C0 crossing.

![Source geometry preparation and physical visibility have separate owners.](illustrations/ownership.svg)

Text alternative: one shared tessellation supplies the visible surface and its edge chains; analytic normals and crease splits govern shading; instance normal transforms and shared physical depth keep both drawing paths aligned; retained source identity answers a pick.


The same producer exists in Rust, C++ and Python; the Rust version is the one this viewer builds against, and the course installs the other two as supplied files.
