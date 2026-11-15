# Construct and edit geometry

Draw a line with typed or picked points, preview a transform, and commit one reversible edit.

## Prerequisites and ownership

Complete documents, commands and input. This chapter belongs to the same `application` checkpoint. It teaches `app/coords.rs`, `cplane.rs`, `snap.rs`, `deform.rs`, `splitting.rs`, the drawing/drag/tool/splitting state adapters and the shared command-tool implementations. Geometry verbs register their constructions through the command contract; they do not introduce another event loop or editing history.

## Build

Connect coordinate resolution and tool continuations to the existing draft, selection and document transaction owners.

### Resolve coordinates before constructing geometry

```rust
--8<-- "session_viewer/src/app/coords.rs:modeling-coordinates"
```

Parsing distinguishes a position from an offset, a polar displacement and a distance along the current direction. `@1,2` needs a previous point; a single distance needs a direction. Rejecting missing context is safer than silently inventing an axis. The `w` prefix explicitly requests world coordinates.

`coords::place` resolves this value using the fixed construction plane when one is selected. Plane coordinates are measured along its orthonormal axes from its origin. An offset uses those axes without adding the plane origin. In View mode, absolute coordinates remain world coordinates while picked points land on the view's drawing plane. Parsing text and deciding its reference frame are separate operations.

`CPlane::from_3_points` normalizes the origin-to-x direction, crosses it with the third direction to obtain a normal, then derives the perpendicular y axis. Collinear input has no frame and is rejected. `local` projects onto the axes; `world` expands along them. A named square-on view selects its facing world plane; an oblique view defaults to the ground. A grazing ray cannot provide a stable plane intersection and needs the explicitly handled fallback.

### Snap in the coordinate system the user sees

`snap.rs` derives endpoints, vertices, midpoints, centers, perpendicular feet and nearby curve points. Its ordered `SnapKind` makes preference explicit. Modes are independent bits: enabling Near need not enable Perpendicular. Grid snapping rounds the two local plane coordinates and preserves height, then maps back to world space; rounding world x and y would be wrong on a tilted plane.

`state/drag.rs::Targets` bins rows, points and strokes in screen cells. Large rows are handled outside the normal bin coverage limit. Moved objects are excluded from their own targets. The current view belongs to the bin state because a camera change invalidates screen-space proximity even if world geometry is unchanged.

Collection has per-event point, wire and time budgets. This bounds a pointer move's work instead of traversing an arbitrarily large scene synchronously. Snap reach is expressed in CSS pixels and converted consistently with the device-pixel query. A small world-space distance alone cannot establish that two points look close at the current zoom.

### A draft owns incomplete input

`state/drawing.rs::Draft` holds accepted points, hover position, plane, construction kind and an optional tool. Fixed-point constructions finish when their required point count is reached; open constructions finish through their explicit completion action. Rectangle and polygon constructions derive geometry in the chosen plane, so their preview and final points share a frame.

`finish_drawing` converts resolved points into an explicit world-coordinate command and invokes the same parser and action path as a fully typed command. This preserves validation, selection and undo behavior. If execution fails, the draft is restored so the learner can correct its input. A preview line is not already an independently committed scene object.

### Continue a tool without borrowing its host twice

The `Tool` trait supplies prompts, options, point placement, preview, and optional direct pointer/pick handling. Its `Next` result distinguishes waiting for more input, repeating an operation and finishing. The host temporarily removes the draft/tool while invoking it with mutable `State`, then restores the continuing draft. This makes Rust's exclusive mutable borrow match the application's single owner of edits.

`state/tool.rs` saves original row placements for previews. Cursor motion applies temporary placements; accepting input uses the normal edit transaction. Cancelling restores those placements and keeps selection. Thus ten preview moves followed by Escape do not create ten undo entries. `choices.rs` and `options.rs` provide option selection through the same tool contract, including command-line buttons for touch input.

### A delayed pick still belongs to the original drag

`ObjectDrag` retains the press, latest cursor and whether release arrived before the GPU answer. A late hit after release applies the accumulated move; a miss becomes rectangle selection. While held, a hit starts following and a miss starts the selection rectangle. Retaining the release state prevents a quick flick from disappearing merely because readback took another frame.

The moving group retains original transforms and a grabbed world point. The drag plane follows the ground except near a level view, where a vertical plane gives a meaningful displacement. Snapping moves the grabbed point to the target; it must not translate every object's origin independently to that point.

### Edit subobjects by their stable keys

`deform.rs::Target` distinguishes a control point, edge and face. A mesh face includes its hole rims; vertex keys are deduplicated before transformation. Mesh edge numbering follows first appearance rather than an arbitrary hash-map iteration. Surface controls use checked `(u,v)` indices, and boundary selection identifies a side of the control grid.

These adapters resolve the selected target into kernel geometry edits. Whole-object placement remains a separate path. A visually similar result does not make moving an object and deforming its control points interchangeable: later operations and undo need the correct underlying geometry and placement.

### Gather arguments before calling the kernel

`Recipe` stores ordered input steps, option buttons, an axis function and a builder. Answers distinguish points, numbers and distance vectors, while curve picks retain their selection order in separate groups. Typed options can appear among values; the last option wins. Remaining numeric defaults can fill a partially typed recipe only when all remaining steps are numeric. The builder returns several geometries as one `Made` result, so the host can add them as one undoable operation rather than exposing a partially completed surface.

`shape.rs` supplies the shared primitive-shape tool. Its questions distinguish points, sizes and heights: a typed number can mean distance along a rubber band, a literal dimension or distance along the construction-plane normal. The frame stores a right-handed origin and axes, converts between local/world coordinates and can turn its in-plane x direction. Shared Brep/Mesh choices and tessellation settings let primitive verbs vary their recipe while preserving the same input, preview and placement rules.

`surfacing.rs` prepares selected sections before invoking kernel surface constructors. Newell normals reject loops without area using a scale-relative threshold, and planar checks scale their tolerance with the points' bounds. Section alignment requires all curves to be open or all closed. Open sections reverse when their endpoint correspondence is shorter; closed sections align winding and move the seam near the preceding start point. A closed loft needs at least three sections and repeats the first at the end. Invalid kernel output becomes an error rather than entering the scene.

`tool/cut.rs` normalizes cutting inputs in the frame of the object being edited. Lines and polylines become degree-one NURBS curves; planar surfaces and one-face planar BReps can supply a cutting plane. Moving a plane transforms three defining points and rebuilds the normal, which remains correct under nonuniform scale. Moving a picking ray transforms its origin and a second point, then normalizes the resulting direction. These conversions prevent a scaled object from changing the meaning of geometric intersection tolerances.

`state/splitting.rs` owns a pending target and cutter list. A cutter must be an unlocked supported curve distinct from the target, with a bounded list length. Cancelling clears its highlights. `app/splitting.rs` performs the geometric checks and kernel splitting; the state adapter adopts the result through the document edit path. A selected BRep face requires cutters on its surface, not merely overlapping it in the camera image.

### Construct primitives in a local frame

`Box` interprets a picked corner as half-widths about the base center: its local coordinates produce twice their absolute x and y distances. Typed length and width must be positive, while height may be signed but nonzero. The kernel box uses the absolute height and is translated by half the signed height before the construction frame is applied. Thus a negative height places the whole box below its base without asking the kernel to construct an inverted solid. Its preview grows from a line to a rectangle to the complete box.

`Sphere` shares the center/radius questions and places its poles along the construction-plane normal. `Cylinder` asks for a base center, positive radius and signed nonzero height; `Cone` reuses those same questions and validation. For negative heights, the shared upright conversion reverses the frame and supplies a positive height to the kernel. Cylinder previews connect two rings, while cone previews connect the base ring to an apex. The Brep/Mesh choice changes the resulting representation through the shared solid builder, not the input frame.

`Torus` separates the major radius from the tube radius. A picked tube-radius point measures its radial distance from the major circle; a typed value supplies the radius directly. Both radii must be positive and the minor radius must remain smaller than the major radius. Preview rings show the inner, outer, upper and lower extents. The kernel constructs the torus about local z, then the construction frame places that axis along the chosen plane normal.

`Nurbs Curve Circle` builds an exact kernel circle and transforms it into the construction plane; the sampled preview ring is only a drawing aid. `Nurbs Curve Ellipse` lets a picked first-axis endpoint rotate the local frame, whereas a typed first radius keeps its direction. A picked second radius measures perpendicular distance from that first axis, not distance from the center. Both positive radii then parameterize the kernel ellipse in that rotated frame.

`Nurbs Curve Arc` offers Center and 3 Points recipes. Center input uses a radius and an angle strictly between zero and 360 degrees in magnitude; a negative typed angle flips the frame for clockwise travel. Three-point input derives the circumcenter and rejects collinearity relative to the two chord lengths. The exact curve divides its sweep into rational quadratic spans of at most 90 degrees: each middle control has weight equal to the cosine of half the span angle. This is why increasing preview samples is unnecessary for geometric circle accuracy.

`Nurbs Curve Parabola` distinguishes an apex on the curve from a control point. It turns the frame toward the end point, rejects an apex on the chord using a length-relative threshold, and obtains the quadratic control as twice the apex minus half the sum of the endpoints. Evaluating that quadratic at one half therefore returns the requested apex. The preview samples 32 segments; the committed kernel parabola retains the exact curve through start, apex and end.

`Capsule` reuses the cylinder questions but interprets height as the total extent including both rounded caps. Its absolute height must exceed the diameter. The kernel capsule axis therefore starts one radius above the base and ends one radius below the top. Negative height reverses the construction frame through the shared upright conversion. This recipe offers Mesh only; its preview marks the cap junctions rather than pretending to be the complete curved surface.

`Block With Hole` reuses the box questions, including a corner that answers both base dimensions at once. It then asks for a positive hole radius smaller than half the shorter base side, preventing the hole from opening through an outer wall. The preview adds rings at both ends of the through-hole. As with Box, an absolute kernel height and a signed half-height placement keep the base fixed when height is negative.

`Pyramid` accepts a base edge length or a picked corner. A corner sets the edge to its radial distance times the square root of two and rotates the frame by the corner direction minus 45 degrees, matching the kernel square orientation. Signed height uses the shared upright conversion. The preview connects each square corner to the apex, so the picked corner, base and final solid use one frame.

Tetrahedron, Octahedron, Icosahedron and Dodecahedron share center/radius input while supplying their own kernel mesh constructor. Their input radius is the distance to corners, although the constructors take edge length. `polyhedron_of` measures a unit-edge mesh circumradius and scales the requested edge accordingly. Each adapter caches unit-radius face outlines in `OnceLock` for preview. Mesh keeps those faces; Brep converts their outlines into planar faces and rejects radii below 0.001 because face joining has an absolute tolerance.

`Quad Sphere` is a Mesh-only construction from six kernel surface patches, each sampled into eight by eight quads. Shared `to_mesh` removes repeated polygon endpoints and welds coincident patch boundaries using a scale-relative distance with a minimum threshold. Neighboring spatial cells are searched so rounding a point to a different cell does not split the seam. The resulting 384-face mesh is then placed in the construction frame; this patch approximation has its own accuracy and is distinct from an exact sphere BRep.

### Transform a selection without changing its identity

`Move` accepts a checked typed offset or starts a two-point tool. Its preview is the translation from the first point to the cursor; accepting the second point sends that same translation to `State::apply`. Both paths require selection and use the shared edit owner, so pointer sampling changes the preview frequency without creating a different transformation rule.

`Rotate` with a typed world axis turns about the gizmo center, falling back to the world origin. Its interactive form instead picks a center and measures reference vectors in the construction plane. The signed angle uses atan2 of their two-dimensional cross and dot products, rejecting a reference at the center. Translate the picked center to the origin, rotate about the plane normal, then translate back to compose the final transform. A typed angle after the center uses that same plane and commit path.

`Scale` with a typed positive factor scales uniformly about the gizmo center. The interactive tool chooses 1D, 2D or 3D around a picked origin: signed projection along a reference direction, radial reach in the construction plane, or spatial distance. After a reference point, a typed number means the new length, so the factor is that length divided by the old reach. Zero reference length and nonpositive factors are rejected. Scaling only selected orthonormal directions preserves the other directions, which is why 2D scaling leaves plane-normal height unchanged.

`Copy` uses a base point and repeated target points, returning `Next::Repeat` after each copy and keeping the originals selected. A typed offset copies once. Copying rejects locked, display-only and subobject selections, resolves each world delta into the source parent frame, and creates fresh identities under the same parents with names, colors and owned attributes. Changes are grouped into a history transaction per affected document before row synchronization. A copied object therefore has independent identity even when its geometry initially shares reference-counted storage.

`Orient 3 Points` builds an orthonormal frame from each reference triple and target triple. The first edge supplies x; a cross product supplies the normal, then a second cross product supplies y. Coincident or nearly collinear input is rejected. World-to-reference followed by target-to-world produces a rigid transform, so unequal reference and target triangle sizes do not imply scaling. During picking, the preview first translates, then aligns one direction, and finally uses the complete frame. Six accepted points commit through the shared edit owner.

### Build surfaces from ordered sections

`Nurbs Surface 4 Points` makes a bilinear patch from four ordered corners. It checks the crossed diagonals against a squared size threshold, then stores controls in the kernel order a, d, b, c so u runs from a to b and v from a to d. Corners need not be coplanar. The center of this degree-one-by-degree-one patch is their mean; changing one corner therefore moves the middle by one quarter of that displacement.

`Loft` preserves section pick order and offers Open or Closed traversal across those sections. `Nurbs Surface Loft` instead offers Smooth or Straight, choosing degree three or one across an open sequence. Both use the shared section alignment and checked kernel result. Closed traversal repeats the first section and requires at least three sections; it is separate from whether each individual section curve is itself closed. The result enters the document through one `Made` transaction.

`Nurbs Surface Revolve` gathers profile curves, two axis points and an angle, defaulting to a full 360-degree turn. It rejects coincident axis points and angles outside the positive range through 360, converts degrees to radians and uses right-handed rotation about the directed axis. Each profile produces one checked surface, but all profiles are collected before the shared transaction commits them. An invalid profile therefore prevents a partially created collection.

`Nurbs Surface Sweep1` locates each profile along a rail using its sampled center and the nearest rail parameter. Profiles are transformed into those local perpendicular frames and sorted by station. A straight auxiliary loft makes their knots compatible; between stations, homogeneous control coordinates and weights blend together. Sampling includes every profile station as well as bounded intermediate rail samples. Sections return to world frames and form a cubic loft, preserving the supplied profile locations instead of treating every profile as if it began at the rail start.

`Nurbs Surface Sweep2` gathers two rails followed by shape curves. It reverses the second rail when its start lies nearer the first rail end, and reverses a shape when its start lies nearer the second rail. This establishes a consistent first-rail-to-second-rail direction before calling the kernel sweep. Invalid kernel output is rejected. These endpoint-distance choices are alignment heuristics, not a general solution for ambiguous or self-intersecting input.

`Nurbs Surface Network` builds a crossing graph for four through 32 curves using bounds rejection and nearest-curve parameters within a scale-relative tolerance. Two-coloring that graph identifies the two families; disconnected input, crossings within one family and missing cross-family pairs are rejected. Curves are ordered by crossings and reversed to make crossing parameters increase. Four boundaries first try a Coons patch. Larger grids form a sampled Gordon construction: add interpolated surfaces across both families and subtract the tensor surface through their averaged crossings, then fit the resulting grid. Samples include crossing stations and are capped per direction. This numerical construction does not establish an exact unrestricted intersection or interpolation oracle.

### Measure placed geometry and state the approximation

The shared measurement adapter resolves selected rows to kernel geometry, world placement and an optional selected face. Released documents are requested again, with a message to retry after loading. Mesh measurements use cached triangles where available; the fallback fan sums oriented normals for a folded concave face instead of double-counting its area. Result marks store the scene row revision and disappear when that revision changes, preventing stale values from remaining attached to edited geometry. Formatting uses the scene unit with the appropriate power and scientific notation for small nonzero values.

`Measure Distance` accepts two absolute world points directly or opens the common point tool for partial and relative input. The moving readout measures from the first point to the cursor. Accepting the second point reports Euclidean distance and signed world x/y/z differences, then places a temporary projected mark between the points. Enter before either point cancels; after one point it asks for the missing endpoint. Measuring creates no geometry and no document edit.

`Length` transforms each supported line, polyline or NURBS curve into world coordinates before asking its kernel length. This handles nonuniform placement correctly; multiplying an untransformed length by one guessed scale would not. Supported results are summed, unsupported rows are counted as skipped, and an entirely unsupported selection gives an error. The displayed mark is temporary, independent of document history.

`Area` measures meshes, BReps, NURBS surfaces and element geometry, narrowing to the chosen face where supported. Mesh areas use placed triangles; BRep faces use the viewer-quality tessellation, so curved-face results depend on that approximation. A NURBS surface is transformed first, then its tangent cross-product magnitude is integrated with eight-by-eight Gauss points in each knot-span pair. This is numerical integration, not a claim of exact area for every surface.

`Volume` requires closed meshes or solid BReps and reports open objects separately from unsupported kinds. It sums signed tetrahedral volumes from the displayed triangles; BRep face orientation signs turn the face contributions consistently before taking the absolute total. The reference origin lies on the transformed mesh to reduce cancellation for models far from world zero. BRep volume inherits tessellation accuracy. Mixed selections may return the closed-object total while explicitly naming open objects that were not measured.

`BoundingBox` unions selected world bounds and creates an axis-aligned box through the normal geometry transaction. For an element it transforms the underlying element bounds, avoiding distant attribute features that can enlarge display bounds. Flat extents remain valid. `Length BoundingBox` uses those same bounds but only reports twice each half-extent as world X, Y and Z dimensions, with a temporary mark. Creating the box and measuring its extents therefore have different history effects even though they share the bounds calculation.

### Flatten geometry in its own object frame

`Project To Plane` edits supported points, lines, polylines, NURBS curves and meshes while preserving their placements. It composes inverse object placement, world-plane projection and object placement to obtain the local geometry transform. Named planes pass through world zero; the CPlane option chooses the plane facing the camera, while 3Point supplies an explicit origin and orientation. Singular placements, unsupported kinds and non-point geometry that would collapse to a point are skipped with a reason. Mesh normal attributes are cleared because the old normals no longer describe the flattened faces. All successful replacements use one scene edit path with undo.

## Run and observe

Run the `application` checkpoint and load the comparison fixture. Enter `Line 0,0,0 3000,0,0`, then `Fit`. Expand the Created layer and select `my_line`. Enter `Move 0 1000 0`, then Undo and Redo. The line changes position while the comparison objects stay fixed.

Reselect the line and enter `Move`. The dock asks for a base point. Press Escape: the draft closes without creating another object. Enter `Line 0,0,0 0,0,0`; the dock rejects coincident endpoints and the hierarchy count stays unchanged. These commands are the browser-verified starting experiment; snapping and delayed drag behavior require their separate pointer checks.

## Predict and try

Predict where a relative point will lie after changing a fixed construction plane. Start a line, type its first point, then enter `@1000,0`. Compare the plane's x direction with world x. Repeat with the explicit world prefix. A relative displacement follows the chosen frame; it is not an absolute position measured from the global origin.

## Question and answer

Why convert a finished draft to an explicit world-coordinate command?

The picked points have already been resolved. The world prefix prevents a fixed construction plane from transforming them a second time, while the normal command path preserves the same validation, selection and undo transaction as typed input.

## Validation

The complete cumulative native suite passes 609 tests with 60 explicit ignores, including coordinate, plane, snapping, drawing, drag and command tests. The command browser sequence verifies line creation, typed move, undo/redo, rejected endpoints and draft cancellation at two sizes. It does not establish every modeling verb, delayed GPU drag, snap target or physical-touch interaction. Those behaviors remain separate acceptance requirements.
