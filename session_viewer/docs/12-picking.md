# 12 · The viewer shell and picking

Picking draws each object's row number instead of its colour into a small window around the cursor and reads that window back a frame later. Around it this lesson builds the shell: the scene's row tables, State, keys, mouse and touch.

![A pointer release becomes a scissored ID window, an asynchronous bounded readback, a Scene lookup and a selected flag; stale generations are dropped.](illustrations/picking.svg)

## Step 1 · src/engine/gpu/pick.rs

The pick answer, the id textures, and the small window of pixels around the cursor that one pick reads.

`lessons/12/src/engine/gpu/pick.rs` · type this, new file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:pick-window"
```

## Step 2 · src/engine/gpu/pick.rs

The Picker: one request at a time, a generation counter against late answers, and a readback buffer made on first use.

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:picker"
```

## Step 3 · src/engine/gpu/pick.rs

A native-only copy of the whole id frame, which the offscreen tests wait for and read.

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:id-readback"
```

## Step 4 · src/engine/gpu/pick.rs

Open `impl Picker`: request, configure, cancel, the paged source point query and the pending request.

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:picker-request"
```

## Step 5 · src/engine/gpu/pick.rs

The id pass targets, sized to the window, and a second pass that adds ink ids over them.

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:picker-passes"
```

## Step 6 · src/engine/gpu/pick.rs

Copy the window out, map it after submit and poll for the answer frames later; the brace closes the impl.

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:picker-readback"
```

## Step 7 · src/engine/gpu/pick.rs

Choose the hit, ink before faces and then the nearest pixel; ids are one-based, so 0 means nothing.

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:pick-helpers"
```

## Step 8 · src/engine/gpu/pick.rs

Tests: the window stays inside the canvas, and ink beats faces before distance counts.

`lessons/12/src/engine/gpu/pick.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:pick-tests"
```

## Step 9 · src/engine/gpu/pick.rs

Implement the `Lane` trait, so the GPU resets the picker and counts its memory with the lanes.

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:pick-lane"
```

## Step 10 · src/engine/gpu/render.rs

The id pass: faces and clouds over the window and its halo, then ink tested against that depth, then the copy.

`lessons/12/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/render.rs:id-pass"
```

## Step 11 · src/engine/gpu/present.rs

A frame that runs only the id pass for a pick, and a whole-frame id render for native tests.

`lessons/12/src/engine/gpu/present.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/present.rs:pick-frame"
```

## Step 12 · src/app/selection.rs

What is selected inside one object: the whole object, one edge, one face or its control points.

`lessons/12/src/app/selection.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/selection.rs:selection-mode"
```

## Step 13 · src/app/selection.rs

One control point, an object's list of them with their net lines, and the start of `impl Controls`.

`lessons/12/src/app/selection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/selection.rs:controls"
```

## Step 14 · src/app/selection.rs

Collect the controls of each geometry kind, from line ends and polyline vertices to elements.

`lessons/12/src/app/selection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/selection.rs:controls-geometry"
```

## Step 15 · src/app/selection.rs

Mesh vertices, BRep parts, and the control polygons and nets of curves and surfaces; the brace closes the impl.

`lessons/12/src/app/selection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/selection.rs:controls-nets"
```

## Step 16 · src/app/selection.rs

Tests: a new parent replaces a sub-selection, and a line's controls are its two ends.

`lessons/12/src/app/selection.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/12/src/app/selection.rs:selection-tests"
```

## Step 17 · src/app/selection.rs

What a click selects: whole objects, edges or faces.

`lessons/12/src/app/selection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/selection.rs:selection-tool"
```

## Step 18 · src/app/scene_rows.rs

Special row owners, and the note bits an edit uses to say what it changed.

`lessons/12/src/app/scene_rows.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/scene_rows.rs:rows-owners"
```

## Step 19 · src/app/scene_rows.rs

Where an object's rows sit in the lanes, spare capacity, per-document state and one edit's note.

`lessons/12/src/app/scene_rows.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene_rows.rs:rows-footprint"
```

## Step 20 · src/app/scene_rows.rs

A deleted object's hidden rows, and the GPU work one sync stages.

`lessons/12/src/app/scene_rows.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene_rows.rs:rows-staged"
```

## Step 21 · src/app/scene_rows.rs

The side table for footprints that span several lanes, with slots that are reused.

`lessons/12/src/app/scene_rows.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene_rows.rs:rows-spans"
```

## Step 22 · src/app/scene_rows.rs

Freed row ids, handed out again only after the sync that freed them.

`lessons/12/src/app/scene_rows.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene_rows.rs:rows-ids"
```

## Step 23 · src/app/scene_rows.rs

Tests: a footprint is 12 bytes, and freed ids come back last-freed first after the sync.

`lessons/12/src/app/scene_rows.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene_rows.rs:rows-tests"
```

## Step 24 · src/app/scene.rs

The row modules, a loaded file, what a pick landed on, and the list of row namers.

`lessons/12/src/app/scene.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-types"
```

## Step 25 · src/app/scene.rs

The Scene: documents, hidden, locked and coloured objects, and the tables that map row ids to objects.

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-struct"
```

## Step 26 · src/app/scene.rs

Open `impl Scene`: an empty scene, clearing it, and forgetting every row.

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-new"
```

## Step 27 · src/app/scene.rs

Upload: the staged edits first, then the newly walked rows appended to the GPU.

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-upload"
```

## Step 28 · src/app/scene.rs

Add a file: one row per drawable object, walked into GPU rows, then the flat-sheet test.

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-add"
```

## Step 29 · src/app/scene.rs

Questions about a row: what a pick hit, its document, geometry, name, edge, faces and cloud point; the brace closes the impl.

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-lookup"
```

## Step 30 · src/app/scene.rs

An object's placement, the attribute copies that get no row, and kills joined into runs per lane.

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-helpers"
```

## Step 31 · src/app/scene.rs

Tests: duplicate guids stay two objects, shared sessions split on edit, and kills merge into runs.

`lessons/12/src/app/scene.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-tests"
```

## Step 32 · src/app/scene.rs

The key of the tree a node cache was filled from.

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:scene-tree-key"
```

## Step 33 · src/app/feedback.rs

The status line, download progress and the error panel, all plain elements of the page.

`lessons/12/src/app/feedback.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/feedback.rs:feedback-status"
```

## Step 34 · src/app/feedback.rs

Give the canvas keyboard focus; natively there is nothing to focus.

`lessons/12/src/app/feedback.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/feedback.rs:feedback-focus"
```

## Step 35 · src/app/feedback.rs

The rows of the layers panel and of the graph table.

`lessons/12/src/app/feedback.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/feedback.rs:feedback-rows"
```

## Step 36 · src/app/route.rs

Where scenes come from: the public bucket, the local scene, and a scene route.

`lessons/12/src/app/route.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/route.rs:route-consts"
```

## Step 37 · src/app/route.rs

Read the page URL: integer knobs, localhost, the scene in the path, and a safe `?scene=`.

`lessons/12/src/app/route.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/route.rs:route-query"
```

## Step 38 · src/app/route.rs

Turn a scene name into its manifest URL and the prefix of its files.

`lessons/12/src/app/route.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/route.rs:route-scene"
```

## Step 39 · src/app/route.rs

After a lost GPU device, reload once at device scale 1 and show the reason.

`lessons/12/src/app/route.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/route.rs:route-recovery"
```

## Step 40 · src/state/features.rs

The Features sub-struct, empty for now: each later feature adds one field line here.

`lessons/12/src/state/features.rs` · type this, new file

```rust
--8<-- "lessons/12/src/state/features.rs:features-struct"
```

## Step 41 · src/state/features.rs

Four hook lists, also empty: before picks, after picks, pick takers and click wideners.

`lessons/12/src/state/features.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state/features.rs:features-hooks"
```

## Step 42 · src/state.rs

State holds the window, GPU, camera, scene and selection, plus the Features sub-struct.

`lessons/12/src/state.rs` · type this, new file

```rust
--8<-- "lessons/12/src/state.rs:state-struct"
```

## Step 43 · src/state.rs

Open `impl State`: open the GPU and upload the scene rows once.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-new"
```

## Step 44 · src/state.rs

Canvas size, appending and clearing documents, and fitting the camera to everything or the selection.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-scene"
```

## Step 45 · src/state.rs

Resize at most every 100 ms, cloud point size, x-ray, and `touch` after any change.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-resize"
```

## Step 46 · src/state.rs

Select one row, several rows or a click's whole group; hide the selection and show everything.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-select"
```

## Step 47 · src/state.rs

Apply a pick answer: waiting features first, then an edge, face, control or object selection.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-apply-pick"
```

## Step 48 · src/state.rs

One frame: hooks, GPU errors, the pick answer, the anchored camera, present, then a waiting pick frame.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-render"
```

## Step 49 · src/state.rs

Request a pick of the right kind, Esc, the status line and the perf line; the brace closes the impl.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-request"
```

## Step 50 · src/state.rs

The control points as JSON for browser tests, and a position as the f32 the GPU takes.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-inspect"
```

## Step 51 · src/state.rs

Keep the selected rows in the order they were picked.

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-order"
```

## Step 52 · src/state.rs

Test: the selection keeps pick order.

`lessons/12/src/state.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:state-tests"
```

## Step 53 · src/app/keys.rs

A key binding: the key, the modifiers it needs and the function it runs.

`lessons/12/src/app/keys.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/keys.rs:keys-binding"
```

## Step 54 · src/app/keys.rs

Two builders for the table: a plain character press and a named key.

`lessons/12/src/app/keys.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/keys.rs:keys-builders"
```

## Step 55 · src/app/keys.rs

Every shortcut, first match wins: projection, standard views, display toggles, hide, show and x-ray.

`lessons/12/src/app/keys.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/keys.rs:keys-table"
```

## Step 56 · src/app/gesture/mod.rs

The Gesture registry: left-button tools tried in order, empty until lesson 21.

`lessons/12/src/app/gesture/mod.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/gesture/mod.rs:gesture-table"
```

## Step 57 · src/app/gesture/mod.rs

Find the first tool that takes a press, or a plain press dragged past the click slop.

`lessons/12/src/app/gesture/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/gesture/mod.rs:gesture-find"
```

## Step 58 · src/app/gesture/mod.rs

Test: the tools are tried control, gizmo, object.

`lessons/12/src/app/gesture/mod.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/12/src/app/gesture/mod.rs:gesture-tests"
```

## Step 59 · src/app/touch.rs

Touch thresholds, what one touch event asks for, and the fingers on the screen.

`lessons/12/src/app/touch.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/touch.rs:touch-types"
```

## Step 60 · src/app/touch.rs

Open `impl Touches`: a finger lands, moves, lifts or is taken away by the browser.

`lessons/12/src/app/touch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/touch.rs:touch-event"
```

## Step 61 · src/app/touch.rs

One finger orbits; two pan by their midpoint and zoom by their distance.

`lessons/12/src/app/touch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/touch.rs:touch-moved"
```

## Step 62 · src/app/touch.rs

A lift may be a tap or a double tap; the brace closes the impl.

`lessons/12/src/app/touch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/touch.rs:touch-lifted"
```

## Step 63 · src/app/touch.rs

`Default` for Touches, the same as `new`.

`lessons/12/src/app/touch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/touch.rs:touch-default"
```

## Step 64 · src/app/input.rs

Mouse, keyboard and finger state kept between events.

`lessons/12/src/app/input.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/input.rs:input-struct"
```

## Step 65 · src/app/input.rs

`Default`, and `impl Input` opened with nothing held.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:input-new"
```

## Step 66 · src/app/input.rs

A key press runs the first binding that matches it.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:input-key"
```

## Step 67 · src/app/input.rs

Right orbits, middle pans, left goes to `left`; a move past the slop turns a press into a drag.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:mouse-buttons"
```

## Step 68 · src/app/input.rs

Wheel zoom at the cursor, the modifier keys, and losing focus.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:mouse-wheel"
```

## Step 69 · src/app/input.rs

Fingers: one may run a tool and a second cancels it; otherwise they move the camera, tap to pick, double tap to fit.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:mouse-touch"
```

## Step 70 · src/app/input.rs

Forget every gesture in progress.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:input-cancel"
```

## Step 71 · src/app/input.rs

Left press and release: a tool, a gesture, or a click that requests a pick; the brace closes the impl.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:input-left"
```

## Step 72 · src/app/input.rs

Listen for the browser's `pointercancel` and send it into the event loop as a message.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:pointer-cancel"
```

## Step 73 · src/app/input.rs

Physical pixels per CSS pixel.

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:input-dpr"
```

## Step 74 · src/lib.rs

Bring in State and define the messages the loader sends into the event loop.

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:state-msg"
```

## Step 75 · src/lib.rs

The App that winit calls with every event.

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:app-struct"
```

## Step 76 · src/lib.rs

Start the event loop, adopt the ready state, and ask for a redraw only when something changed.

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:app-run"
```

## Step 77 · src/lib.rs

Handle the window once it exists, each loader message, and each window event.

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:app-events"
```

## Step 78 · src/lib.rs

Canvas helpers: find it, check its focus, check the tab is visible, read its pixel size.

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:canvas"
```

## Step 79 · src/lib.rs

Start the viewer, showing the notice first if the page reloaded after a lost device.

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:start"
```

## Step 80 · src/app/inspection.rs

With `?inspect=1`, write a JSON snapshot of the viewer onto the canvas for browser tests.

`lessons/12/src/app/inspection.rs` · type this, new file

```rust
--8<-- "lessons/12/src/app/inspection.rs:inspection-publish"
```

## Step 81 · src/app/inspection.rs

The selected identity, every drawn text label, and a geometry's kind name.

`lessons/12/src/app/inspection.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/inspection.rs:inspection-helpers"
```

## Step 82 · src/engine/gpu/text_outline.rs

GPU tests from lesson 04a that need the id pass: an outlined glyph keeps its coverage, id and selection colour.

`lessons/12/src/engine/gpu/text_outline.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/text_outline.rs:outline-tests"
```

## Step 83 · tests and assets

Copy these files and test modules from `lessons/12/`; they are checked, not explained.

- `src/engine/gpu/vectors.rs`: the test module `shell_tests` at the end, which draws, selects, hides and picks one arrow on a headless GPU.
- `tests/selection.cjs`: the browser selection test.
- `assets/view_local.yaml`: the scene a local `trunk serve` loads from lesson 14 on.

## Step 84 · registration lines

Copy the lines tagged `register:shell`, `register:pick`, `register:features` and the module tags below from these files of `lessons/12/`:

- `src/app/mod.rs`: the modules `feedback`, `gesture`, `input`, `inspection`, `keys`, `route`, `scene`, `selection` and `touch`.
- `src/engine/gpu/mod.rs`: the `pick` module, the picker and the two control lanes, their creation, and the picker reset on resize.
- `src/engine/gpu/present.rs` and `render.rs`: mapping the pick after submit, the waiting id pass, and the control draws.
- `src/lib.rs`: the call to `start`.
- `src/state.rs`: the `features` module.

Run `cargo check` in `lessons/12/`.

## Check

`cargo check` compiles, and in `lessons/12/` the commands `cargo xtest --lib gpu::pick`, `cargo xtest --lib selection` and `cargo xtest --lib scene` pass this lesson's unit tests. The browser still shows an empty canvas: the loader that opens the GPU and sends `Ready` arrives in lesson 14.
