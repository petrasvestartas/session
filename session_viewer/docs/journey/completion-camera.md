# Project and fit a scene

Orbit a blue wireframe cube, switch projection, and fit its bounds using the production camera.

This fifth cumulative subject is reviewed against its native tests, independent wasm build and recorded Windows Intel WebGPU interactions at desktop and upright sizes. The later renderer and application have separate acceptance.

## Prerequisites and ownership

Keep the frame, painting, mathematics and command-dock implementations. This subject owns `src/camera.rs` and its projection experiment. The kernel supplies points, bounding boxes, quaternions and transforms; scene rows and GPU geometry arrive later.

The camera stores an orientation, target and distance. The eye and up vector are derived values; changing an orientation without updating them would leave the view matrix stale. Scene units and metres are deliberately distinct. Millimetre coordinates must be converted once when constructing the camera matrix.

```rust
--8<-- "session_viewer/src/camera.rs:camera-pose"
```

## Build

Add the camera module and registration, retaining the earlier source unchanged.

From `session_tests`, create a fresh checkpoint:

```sh
node --import ./scripts/typescript.mjs scripts/course.ts completion-checkpoint camera ../session_viewer/target/learn-camera
```

### From world coordinates to pixels

A view matrix places the eye at the origin and expresses points in its coordinate axes. Projection maps that space to normalized device coordinates. Perspective divides by the homogeneous coordinate, making distant objects smaller; orthographic projection keeps parallel edges parallel.

The production matrix composes scene-unit conversion, view, projection and a lens shift. The anchor keeps coordinates near the camera instead of sending large absolute values to the GPU. This experiment uses an anchor at the origin; the maintained viewer can use the camera target. Coordinates supplied to either matrix must be relative to its chosen anchor.

```rust
--8<-- "session_viewer/src/camera.rs:camera-projection"
```

The near and far arguments are reversed deliberately: nearby depth is one. A later depth-buffer subject must keep this convention. `depth_reach` includes the scene extent and construction grid so a fit does not cut off distant geometry. `frame` describes the unobscured viewport fraction; its lens shift centres geometry in the space left by panels.

### Motion and fitting

Read `orbit`, `pan`, `zoom_at` and `fit_in` in [the retained camera source](../../src/camera.rs). Orbit composes yaw about world up with pitch about the camera's right axis, then normalizes the quaternion. Pan moves the target in the camera plane; its distance-dependent scale keeps movement useful across zoom levels. Zoom changes distance multiplicatively and clamps it away from zero.

`ray` maps a screen position to a world-space origin and direction. Perspective rays begin at the eye; orthographic rays are parallel and begin behind the scene. Empty viewports or nonfinite cursor coordinates are rejected. `zoom_at` computes the cursor's offset on the target plane and shifts the target by the fraction removed by zooming, keeping that point under the cursor. Picking later consumes the same ray implementation.

Fitting visits the box corners in camera coordinates and chooses a distance that satisfies both horizontal and vertical limits. `fit_in` also accounts for panels through the usable frame; fitting only a box radius would miss that constraint. Changing units preserves the conversion between scene coordinates and metres. Named views select orthographic projection and set the orientation explicitly, while `grow_extent` extends the depth range without moving the camera.

### The projection experiment

The fixture projects eight kernel points with `Xform::transform_point`, which performs the homogeneous divide. The final mapping scales normalized coordinates by half the viewport and reverses vertical direction for screen coordinates. Each cube edge differs in exactly one coordinate; the three bit masks enumerate twelve edges without duplicates.

```rust
--8<-- "session_viewer/examples/course/camera.rs:course-camera"
```

The wireframe deliberately has no hidden-surface removal. Its lines are egui painting, while its camera mathematics is the retained production implementation. Scene geometry and depth rendering belong to later subjects.

## Run and observe

Serve the assembled checkpoint through the shared build cache and resource guard. Open `?tutorial=camera`. Expect a blue cube, white background and a projection label. Drag or press arrows to orbit; use the wheel to zoom. P switches projection, F fits the bounds, and Home selects the orthographic isometric view and fits it. Resize upright, then press F: the cube should remain inside the narrower viewport.

## Predict and try

Predict which edges converge when you press P. Compare the same orientation in both projections: orthographic parallel edges stay parallel, while perspective makes the distant face smaller. Orbit before comparing so front and back faces do not overlap exactly.

## Question and answer

Why must pixel mapping reverse the projected vertical coordinate?

The camera's normalized coordinates increase upward, while screen coordinates increase downward. Omitting the reversal reflects the image vertically even when the camera matrix is correct.

## Validation

The [camera candidate receipt](release/completion-camera-windows-20261115.json) records its independent wasm check, three five-subject source-retention tests, 33 passing native tests and one ignored GPU-dependent test. Chrome on Intel Gen-12LP passes orbit by keyboard and dragging, projection round trips, wheel zoom, fitting and named-view restoration at desktop and upright dimensions. The suite also reruns the preceding four subjects. Upright emulation is not a physical-phone test; complete course and production-endpoint acceptance remain pending.
