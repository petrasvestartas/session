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

`tool/gather.rs` describes a recipe as ordered curve selections, points, bounded numbers and distances. Curve order is retained where an operation depends on it. A default number is accepted explicitly; invalid ranges produce a contextual prompt instead of calling the kernel with unusable values. `shape.rs` constructs geometric shapes through the common drawing contract.

`surfacing.rs` validates shared geometric facts such as loop normal, scale and planarity. Newell's sum obtains a loop normal; its magnitude is checked against the square of the model scale. Planarity compares point distance to a scale-relative tolerance. Individual surface verbs can then state their own construction rather than independently implementing inconsistent tolerance rules.

`state/splitting.rs` owns a pending target and cutter list. A cutter must be an unlocked supported curve distinct from the target, with a bounded list length. Cancelling clears its highlights. `app/splitting.rs` performs the geometric checks and kernel splitting; the state adapter adopts the result through the document edit path. A selected BRep face requires cutters on its surface, not merely overlapping it in the camera image.

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
