# Mathematics in the real scene

Animate kernel geometry beside a mathematical column, with labels and matrix entries connected to the same scene.

## Prerequisites and ownership

Complete [mathematics](completion-mathematics.md), [rows](completion-rows.md), [scene rendering](completion-geometry.md), [decoding](completion-decoding.md) and [the application lifecycle](completion-application.md). This chapter of the application subject teaches the scene adapter in `spatial.rs`, the scene-host implementation in `mode.rs`, and `StageApp` in `src/app/stage/application.rs`. It reuses the completed mathematics modules, including `geometry.rs`, `view.rs` and `net.rs`. The explanations below connect their existing outputs to scene rows, cameras and displayed matrices; scene integration must not rewrite those finished implementations.

The `application` checkpoint contains the production scene host and retains every earlier subject implementation. Its own source compiles for wasm and links native tests. This chapter uses that executable checkpoint. All five shipped decks have passed cumulative browser playback checks; the validation section distinguishes those checks from physical-device acceptance.

## Build

Study the named sections in the live modules; retain the completed stage core and its deck schema.

### Geometry descriptions become kernel objects

[geometry.rs](../../src/app/stage/geometry.rs) evaluates named points and emits `Spatial` descriptions for the current frame: an ID, kind, control points, colour, width, dash flag and reveal fraction. Cuboids emit twelve edges. Labels remain mathematical primitives projected from their world attachment. Geometry is not painted as a second set of egui lines.

```rust
--8<-- "session_viewer/src/app/stage/spatial.rs:stage-kernel-geometry"
```

`geometry` makes real kernel `Point`, `Line` and `Polyline` values. An arrow is a kernel line with an end arrowhead. The regular scene display code therefore supplies their GPU representation and depth testing. Alpha is composited over the stage background before conversion to the opaque kernel colour.

Units matter at this boundary. Point diameters use device pixels and include the device-pixel ratio. Line widths use the display contract's units. Dash lengths use the current world size of a screen pixel. Dots scale directly with the slide; lines use the shallower `scale.powf(0.3)` rule so they remain readable when the slide shrinks. Do not apply the device ratio to both the frame mapping and the same line width.

### Order one scene frame

```rust
--8<-- "session_viewer/src/app/stage/mode.rs:stage-frame-order"
```

The scene mode implements the same `Mode<Host>` contract as the mathematics mode, with `State` as its host. Tick the stage clock before reading the animated camera. Apply that camera, then derive the label lens from the resulting real view. Painting can now place equations, projected labels and fly-outs in the same slide coordinates.

Painting also produces this frame's spatial descriptions. Only afterwards can `Scene3::sync` update the retained rows. Synchronizing before painting would display the preceding frame's geometry. `std::mem::take` temporarily moves the description vector out of the stage, allowing the adapter to borrow the viewer state while synchronizing; putting it back retains its allocation for the next frame.

The callback returns egui output and a repaint request. The application still owns texture uploads, GPU submission and presentation. The mode's background comes from the same presentation palette used for stage painting, keeping the scene clear colour and opaque geometry colour conversion consistent.

### Retain rows and update their properties

```rust
--8<-- "session_viewer/src/app/stage/spatial.rs:stage-scene-rows"
```

`Scene3` maps each authored object ID to a scene row, its baked control points and the last applied description. A new object is put into a one-object kernel session and appended through `State::append`. The row count before append identifies its first row; the count is checked afterwards before recording it.

These rows are non-pickable decoration. Their visibility, colour, placement and reveal are GPU row properties, not document selection operations. A point moving from its baked position receives a translation. A segment receives the transform between its baked frame and current frame, so moving or stretching it does not require walking the kernel geometry again. A segment born at zero length is baked as a unit segment and placed with a tiny nonzero axis to preserve the downstream inverse.

A changing polyline shape needs a new row. A changed width, dash flag or geometry kind also rebuilds the representation: hide the old row, then append its replacement. Within a frame, missing stage objects are hidden. Stage mode owns the displayed scene: when chapter replay changes the epoch, `reset` calls `State::clear` if it has allocated rows, then forgets its row map. A slide-scale change greater than two percent takes the same reset path to rebuild scale-dependent pens. This adapter is therefore a whole-scene mode; it is not an overlay that preserves an independently loaded document through a reset.

Reveal and visibility are separate. An object is considered visible only above both the alpha and reveal thresholds. Point reveal additionally controls its dot effect. Updating colour writes both point and line colour channels, so the same stage operation works for either representation.

### The scene camera also projects labels

```rust
--8<-- "session_viewer/src/app/stage/spatial.rs:stage-camera-projection"
```

The authored camera is orthographic. Its origin is placed inside the scene part of the slide by setting the camera frame. Zoom is expressed in slide pixels per world unit; the adapter converts it to the production camera's distance using the orthographic half-height relation and the current frame scale.

Reapply the authored orientation while its tween is busy, when its serial changes, or when the canvas changes. Otherwise retain the user's orbit. Resetting the view increments the serial, which deliberately reapplies the authored pose.

`lens` converts that actual camera into a two-row affine map from world coordinates to slide coordinates. Its axes come from the production camera's right and up vectors; its translation includes the camera target and the framed screen centre. The mathematical labels and fly-out source positions use this same lens. Recomputing labels only from the authored camera would make them drift away when the user orbits.

`geometry.rs::place_labels` chooses the label's offset from that projected anchor. An authored direction fixes the offset. Otherwise it scores sixteen directions at two gaps against earlier labels, lines, points and the slide boundary. The previous choice receives a fifteen-point advantage, reducing direction changes during motion. This remembered `label_dir` makes automatic placement depend on the path taken through an animation. Backward replay reconstructs authored actions without replaying every painted frame, so it need not reproduce the same automatic label offsets. Preserve this distinction when testing: exact local-versus-production comparisons need the same clock and event sequence; a forward-versus-replay pixel assertion can fail in the unchanged production viewer.

```rust
--8<-- "session_viewer/src/app/stage/view.rs:stage-camera-fit"
```

Fitting first chooses the requested view direction, projects the chosen points at unit zoom and measures their two-dimensional bounds. It shifts the camera centre by the projected box centre and uses the smaller of the horizontal and vertical available-space scales. An explicit centre or zoom overrides the fitted result. This fits the scene region rather than the whole slide, which also contains the mathematical column.

### Derive, move and fly retain their meanings

The completed [action implementation](../../src/app/stage/actions.rs) already owns these operations. Scene integration supplies current geometry and projection; it does not replace the action interpreter.

`derive` constructs a new destination while retaining its source. For geometry it stores the source control points for interpolation; for mathematical content it pairs named or matching primitives and copies them toward the new expression. `move` changes a named point's value, so attached geometry and labels follow the same point. `fly` resolves source and destination subsets, reveals the destination subset and animates matched pieces between their current positions. Its `keep` option determines whether source pieces remain visible.

A deck's `fill` block flies each fixed node's coordinate label from its camera-projected node into a row of `xF`, with restoring the short node label as a separate action, while [the matrices chapter](../../examples/stage/force-density-matrices/02-matrices.yaml) flies connectivity, force-density and load entries across column matrices as scene cables and nodes highlight in turn. Their common slide coordinates make movements across the scene and mathematical column coherent.

### Build and solve the cable-net matrices

```rust
--8<-- "session_viewer/src/app/stage/net.rs:stage-net-equilibrium"
```

Each connectivity row represents one cable: plus one at its start node, minus one at its end. Restricting the columns gives the free-node and fixed-node matrices. `density` forms `C_rows^T Q C_cols` by summing each cable's signed contribution multiplied by its force density.

The unknown positions satisfy `D_N x_N = p - D_F x_F`. `rhs` subtracts the fixed-node contribution from each free node's load. `solve` performs elimination on the free-node matrix, solving all three coordinate columns together, then writes those coordinates back into a copy of the complete node list. Fixed nodes retain their positions.

The small elimination routine has no pivoting or singular-system recovery. Teach it for the anchored positive-density examples used by these decks, not as a general robust solver for arbitrary input. An unsupported unconstrained or degenerate system must not be presented as an accepted result.

`Net::matrix` derives the displayed connectivity, density, coordinate and force matrices from the same net used for solving. Row and column labels follow the chosen free/fixed ordering. A matrix on the slide is therefore a view of the calculation, rather than a separately transcribed table that can disagree with the scene.

### Input and portrait layout

Keyboard stepping, chapter navigation, theme changes and camera reset are owned by `StageApp::key`. Pointer coordinates are converted from physical pixels to CSS points before the frame maps them into slide space. Dragging the scene side orbits; the wheel zooms that side. A double click on that side resets the authored camera. Clicking the mathematical column advances the step.

```rust
--8<-- "session_viewer/src/app/stage/application.rs:stage-touch"
```

Touch handling tracks one finger by ID. A short tap is distinguished from motion; a predominantly horizontal swipe steps forward or backward. Scene orbit waits for a hold before applying drag deltas, so a quick swipe can navigate instead. Cancelled touches clear the retained finger. These choices require actual interaction tests, not only a portrait screenshot.

`Frame::stacked` places the scene above the column in an upright viewport. Both regions keep their own reversible mappings to the authored slide. Projection, painting and hit testing must all use those mappings; simply rotating the desktop image would put touches and labels in the wrong places.

### Load the deck before playback

[index.html](../../index.html) begins fetching the four stage fonts, index and discoverable chapter files while wasm loads. This early fetch is an optimization: the Rust loader remains responsible for validating route input, decoding UTF-8, parsing the index and chapters, decoding pictures and reporting contextual errors.

`stage::open` builds one `Loaded` value containing fonts, parsed chapters and pictures. The loader boxes that value behind `Loaded<State>` and posts it through `Msg::Mode`. Application adoption consumes the loaded value through its `open` method and installs the resulting boxed `StageApp` in the active mode slot. Font failures include the font name, chapter failures include the chapter filename, and picture failures include the picture path. Do not silently replace a failed deck with an empty scene.

## Run and observe

Materialize `application` as described in [Maintaining](../maintaining.md). Build its `production.html` with Trunk through the shared cache and resource guards. Trunk emits `dist/index.html`; serve that directory and open `/?stage=decks/force-density-matrices/index.yaml&data=off&chapter=7&step=1`.

The pillow chapter begins on the dark ground with a flat 10-by-10 grid (36 fixed boundary nodes and 64 free interior nodes) beside the sparse pattern row `C -> CN, CF`. Advance to solve the cushion, then advance to the runnable SciPy Cholesky script whose solved peak height is `2.025`. Orbit the scene and press `T` to swap between dark and light grounds at the same step. At upright dimensions, the scene sits above the column.

![The materialization deck at its whole-net cut-length stage. This is a controlled-clock settled-state capture, not a real-time timing or mathematical-correctness proof.](../screenshots/completion/materialized-net.png)

The materialization deck at its whole-net cut-length stage. This is a controlled-clock settled-state capture, not a real-time timing or mathematical-correctness proof.

## Predict and try

In `07-pillow.yaml`, predict the solved cushion height before changing `load: [0, 0, 0.35]` to `[0, 0, 0.7]`: doubling the upward load doubles the free-node heights because `C_N^T Q C_N` stays fixed. Restore the load and change `q: 1` to `q: 2`: doubling every force density doubles the matrix, halving the heights while fixed boundary coordinates stay unchanged. Step backward and verify that replay recovers the flat grid.

## Question and answer

Why must the label lens be computed from the real camera after its update?

The user can orbit away from the authored pose. Projecting labels with that older pose would move the geometry without moving its labels. The real camera's affine lens keeps both in the same slide coordinates, including fly-outs into matrix rows.

## Validation

The standalone mathematics checkpoint passes 24 native tests. The cumulative application's stage suite separately passes 20 tests, including complete decks at rest, fixed-width row-slot centring, nested equation option merging, matrix construction and dark-default ground swaps. Default and tutorial wasm checks, native linking and optimized Trunk compilation pass for the cumulative source tree.

Initial cumulative browser playback covered 554 forward step checks across the five shipped decks at 1280 by 800 and 390 by 844. After upstream switched decks to the dark default ground, revised the Jacobian/materialization derivations and streamlined the matrix chapters to 22 total chapters, [the stage reconciliation receipt](release/completion-stage-reconciliation-windows-20261118.json) re-verified all 22 chapters of all five decks at 1280 by 800 and 390 by 844 (44 cases, 176 forward step checks, backward navigation and light/dark theme round-trips) with 68 reviewed captures. Settled overlap reports and uncaptured GPU errors are empty. These are controlled-clock checks of the cumulative executable, not a forward-versus-replay pixel-equivalence claim.

Separate interaction checks exercise mouse orbit, wheel zoom, camera reset, column clicks, keyboard backward navigation, and emulated upright touch swipe/tap navigation. Six deliberate font, chapter and picture loading failures show contextual filenames at both sizes. The actual adapter is Intel Gen-12LP in desktop Chrome; viewport and touch emulation do not establish physical-phone acceptance. Earlier failed local-server runs and the historical replay comparison remain preserved in the evidence record.

All 359 production source and support teaching witnesses are bound, while complete-course acceptance remains pending on the unresolved `instanced_frames_match_the_baked_scene` application GPU comparison and physical-device scope. Source copies and successful compilation alone do not meet those requirements.
