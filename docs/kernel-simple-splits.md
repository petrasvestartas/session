# Simple curve and face splits

The `simple_split` namespace/module exposes the same five functions in C++, Python and Rust:

- `split_line_by_curves(line, cutters, tolerance)` retains line types and display attributes.
- `split_polyline_by_curves(polyline, cutters, tolerance)` retains all original corners and display attributes.
- `split_curve_by_curves(curve, cutters, tolerance)` returns every original NURBS interval at isolated 3D intersections. Lines and polylines can be represented as degree-one NURBS cutters. A closed curve’s storage seam does not introduce an extra split.
- `split_brep_face_by_curves(brep, face_index, cutters, tolerance)` replaces one face with all its regions inside the owning BRep. It subdivides shared edges and adjacent pcurves together, preserving closed-shell membership. Existing holes stay with the correct regions.
- `split_surface_by_curves(surface, cutters, tolerance)` wraps an individual surface’s natural boundary in a BRep and returns all resulting trimmed regions.

Inputs remain unchanged. Invalid inputs throw `invalid_argument` in C++, raise `ValueError` in Python, or return `Err(String)` in Rust. A valid cut that divides no region returns an unchanged-data copy. The tolerance is positive, finite, and expressed in model units.

Cutters must lie on the selected surface; no implicit projection is performed. Output edges and UV trims retain source curve intervals. Existing BRep JSON/protobuf serialization preserves the resulting surfaces, pcurves, vertices, wires, faces, shells and solids. Regression coverage includes crossings, rational and tangent curves, self-crossing polylines, closed Bezier cutters, holes, repeated face cuts, cylinder seams, and persistence.

These are face partitions, not volume divisions. There are no boolean operations, added caps, or implicit face extraction. Overlapping cutters, pole-edge splits, degenerate surface domains and unsupported seam configurations are rejected. Standalone closed/pole surface boundaries require a BRep with explicit seam topology. Work limits bound intersection subdivision and trim classification.

The implementation is independent; OCCT is a reference only. The checkpoint deliberately excludes unfinished trim/extend wrappers and viewer split commands. See `kernel-simple-splits-checkpoint.json` for exact sources and validation.

The split meshing regression verifies both half-face areas and unchanged neighboring face areas with quality-controlled tessellation. Faces with newly split boundary vertices use constrained meshing. The older Rust checkpoint also deep-copies tree nodes so source transactions and Undo cannot mutate another shared session.
