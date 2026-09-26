# 01 · First WebGPU frame

This lesson writes the whole renderer core: opening the GPU, buffers, render targets, pipelines, the object table and the backdrop lane. Nothing calls it yet; lesson 12 opens the window that asks it for frames.

## Step 1 · build.rs

A build script copies every shader into OUT_DIR, `#include` lines expanded and comments, indentation and blank lines stripped.

`lessons/01/build.rs` · type this, new file

```rust
--8<-- "lessons/01/build.rs"
```

## Step 2 · src/lib.rs

Mark the moment the module starts on the browser's performance timeline.

`lessons/01/src/lib.rs` · add the line tagged `register:frame`

```rust
--8<-- "lessons/01/src/lib.rs:entry"
```

## Step 3 · src/lib.rs

The `shader!` macro loads one minified shader, and `mod engine;` pulls the renderer into the crate.

`lessons/01/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/lib.rs:shader-macro"
```

## Step 4 · src/engine/mod.rs

The engine has three parts: the GPU, frame timing and the pipelines.

`lessons/01/src/engine/mod.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/mod.rs"
```

## Step 5 · src/engine/performance.rs

Frame timing state: the smoothed frame time, the quality tier given up while dragging, and the slow-frame detector.

`lessons/01/src/engine/performance.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/performance.rs:performance"
```

## Step 6 · src/engine/performance.rs

Record each frame and move the tier by the median of the last drag frames.

`lessons/01/src/engine/performance.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/performance.rs:performance-frame"
```

## Step 7 · src/engine/performance.rs

The clock, startup marks and memory size, once for the browser and once for native tests.

`lessons/01/src/engine/performance.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/performance.rs:clock"
```

## Step 8 · src/engine/performance.rs

The tier tests.

`lessons/01/src/engine/performance.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/performance.rs:tests"
```

## Step 9 · src/engine/gpu/mod.rs

The files of the gpu folder, one line each; later lessons add their own lines here.

`lessons/01/src/engine/gpu/mod.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:modules"
```

## Step 10 · src/engine/gpu/device.rs

What opening the GPU returns: surface, device, queue, canvas configuration, GPU type and the first error.

`lessons/01/src/engine/gpu/device.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:setup"
```

## Step 11 · src/engine/gpu/device.rs

Open the GPU in four calls, instance → surface → adapter → device and queue, then configure the canvas.

`lessons/01/src/engine/gpu/device.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:open"
```

## Step 12 · src/engine/gpu/device.rs

Pick a native GPU by name, and keep only the first error message.

`lessons/01/src/engine/gpu/device.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:helpers"
```

## Step 13 · src/engine/gpu/device.rs

A test that a broken shader reaches the error callback.

`lessons/01/src/engine/gpu/device.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:shader-test"
```

## Step 14 · src/engine/gpu/device.rs

Store browser GPU errors and a lost device in that one message.

`lessons/01/src/engine/gpu/device.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:browser-errors"
```

## Step 15 · src/engine/gpu/device.rs

The first-error test.

`lessons/01/src/engine/gpu/device.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/device.rs:tests"
```

## Step 16 · src/engine/gpu/buffers.rs

The GPU connection every other file takes: device, queue and the pipeline cache.

`lessons/01/src/engine/gpu/buffers.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:ctx"
```

## Step 17 · src/engine/gpu/buffers.rs

Usage flags for the three kinds of growing buffer: storage rows, vertices and indices.

`lessons/01/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:usages"
```

## Step 18 · src/engine/gpu/buffers.rs

A buffer that grows by half when full, copying its rows GPU to GPU.

`lessons/01/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:growbuf"
```

## Step 19 · src/engine/gpu/buffers.rs

Overwrite rows in place, fill a run with one row, and pack runs into a new buffer.

`lessons/01/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:rows"
```

## Step 20 · src/engine/gpu/buffers.rs

Forget or free the rows, and ask how large a buffer this device allows.

`lessons/01/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:growbuf-state"
```

## Step 21 · src/engine/gpu/buffers.rs

A small mesh uploaded once and drawn many times.

`lessons/01/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:template"
```

## Step 22 · src/engine/gpu/buffers.rs

Buffer helpers: zeroed, replaced, uniform, a bind group over buffers, and the growth sizes.

`lessons/01/src/engine/gpu/buffers.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:helpers"
```

## Step 23 · src/engine/gpu/buffers.rs

The growth tests.

`lessons/01/src/engine/gpu/buffers.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/buffers.rs:tests"
```

## Step 24 · src/engine/gpu/view.rs

Display settings, each read once at start from the page URL or an environment variable.

`lessons/01/src/engine/gpu/view.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/view.rs:view"
```

## Step 25 · src/engine/gpu/view.rs

Real pixels per CSS pixel, and the switch that drops to scale 1 after a lost GPU.

`lessons/01/src/engine/gpu/view.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/view.rs:pixel-ratio"
```

## Step 26 · src/engine/gpu/view.rs

Read one setting: `?name=` in the browser, an environment variable natively.

`lessons/01/src/engine/gpu/view.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/view.rs:knobs"
```

## Step 27 · src/engine/gpu/targets.rs

How many pixels each kind of GPU may draw at 4x MSAA.

`lessons/01/src/engine/gpu/targets.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:budgets"
```

## Step 28 · src/engine/gpu/targets.rs

The frame's depth, colour and triangle id textures at one sample count.

`lessons/01/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:targets"
```

## Step 29 · src/engine/gpu/targets.rs

Choose 1 or 4 samples from the GPU type, the canvas size and the device scale.

`lessons/01/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:samples"
```

## Step 30 · src/engine/gpu/targets.rs

Open the face pass and the ink pass over those textures.

`lessons/01/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:passes"
```

## Step 31 · src/engine/gpu/targets.rs

Create a 2D texture, and pair it with its view so dropping the pair frees the memory.

`lessons/01/src/engine/gpu/targets.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:textures"
```

## Step 32 · src/engine/gpu/targets.rs

The MSAA budget tests.

`lessons/01/src/engine/gpu/targets.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/targets.rs:tests"
```

## Step 33 · src/engine/pipelines/mod.rs

Count the pipelines and shader modules compiled so far.

`lessons/01/src/engine/pipelines/mod.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:head"
```

## Step 34 · src/engine/pipelines/mod.rs

A GPU object made on its first use and shared by every clone.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:lazy"
```

## Step 35 · src/engine/pipelines/mod.rs

The cache that hands back the same shader, layout or pipeline when asked twice.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:cache"
```

## Step 36 · src/engine/pipelines/mod.rs

The format and sample count a pipeline draws into, and how it uses depth and colour.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:modes"
```

## Step 37 · src/engine/pipelines/mod.rs

The description of one pipeline, with chainable methods for the parts that differ.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:desc"
```

## Step 38 · src/engine/pipelines/mod.rs

The shared WGSL every scene shader ends with, one line per snippet.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:prelude"
```

## Step 39 · src/engine/pipelines/mod.rs

Two vertex layouts: an object row per vertex, and a bare position.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:vertex-layouts"
```

## Step 40 · src/engine/pipelines/mod.rs

Compile each shader text once, on first use, and share bind group layouts through the cache.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:shader-modules"
```

## Step 41 · src/engine/pipelines/mod.rs

Build a pipeline from its description; it compiles when a pass first sets it.

`lessons/01/src/engine/pipelines/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:build"
```

## Step 42 · src/engine/pipelines/mod.rs

The first-use test.

`lessons/01/src/engine/pipelines/mod.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/mod.rs:tests"
```

## Step 43 · src/engine/pipelines/layouts.rs

Helpers for one binding: a buffer, a read-only storage array, a layout with one uniform.

`lessons/01/src/engine/pipelines/layouts.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/pipelines/layouts.rs:entries"
```

## Step 44 · src/engine/pipelines/layouts.rs

The bind group layouts: pen settings, object rows, ink, markers, lines, points and the point resolve.

`lessons/01/src/engine/pipelines/layouts.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/layouts.rs:groups"
```

## Step 45 · src/engine/pipelines/layouts.rs

Build every shared layout once.

`lessons/01/src/engine/pipelines/layouts.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/pipelines/layouts.rs:layouts"
```

## Step 46 · src/shaders/scene.wgsl

The WGSL every scene shader shares: bind groups 0 to 2, the object row, its flags and `place`.

`lessons/01/src/shaders/scene.wgsl` · type this, new file

```wgsl
--8<-- "lessons/01/src/shaders/scene.wgsl"
```

## Step 47 · src/shaders/physical.wgsl

What a face fragment writes: its colour and the id of its triangle.

`lessons/01/src/shaders/physical.wgsl` · type this, new file

```wgsl
--8<-- "lessons/01/src/shaders/physical.wgsl"
```

## Step 48 · src/shaders/normals.wgsl

The normal transform every shader shares, correct for stretched and mirrored objects.

`lessons/01/src/shaders/normals.wgsl` · type this, new file

```wgsl
--8<-- "lessons/01/src/shaders/normals.wgsl"
```

## Step 49 · src/engine/gpu/frame.rs

What the app hands the renderer each frame, and the three bind groups every draw sets first.

`lessons/01/src/engine/gpu/frame.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:inputs"
```

## Step 50 · src/engine/gpu/frame.rs

The pen and view block every shader reads and the point cloud block, byte for byte as in WGSL.

`lessons/01/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:uniforms"
```

## Step 51 · src/engine/gpu/frame.rs

The window a pick draws, and the matrix that maps the canvas onto it.

`lessons/01/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:pick-view"
```

## Step 52 · src/engine/gpu/frame.rs

Create the uniform buffers and bind groups for the frame and for the pick.

`lessons/01/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:frame-uniforms"
```

## Step 53 · src/engine/gpu/frame.rs

Write this frame's camera, pen and cloud settings, the clipping planes, and the copies the pick reads.

`lessons/01/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:frame-write"
```

## Step 54 · src/engine/gpu/frame.rs

The pick window test.

`lessons/01/src/engine/gpu/frame.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:tests"
```

## Step 55 · src/engine/gpu/frame.rs

The frame buffers count as a lane, so their bytes show up in the totals.

`lessons/01/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:frame-lane"
```

## Step 56 · src/engine/gpu/frame.rs

The clipping planes block every shader binds, and the field of view.

`lessons/01/src/engine/gpu/frame.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/frame.rs:clip-uniform"
```

## Step 57 · src/engine/gpu/lane.rs

What a pick may answer with.

`lessons/01/src/engine/gpu/lane.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:pick-mode"
```

## Step 58 · src/engine/gpu/lane.rs

The Lane trait: the hooks every drawing lane may fill, each with a default.

`lessons/01/src/engine/gpu/lane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:lane-trait"
```

## Step 59 · src/engine/gpu/lane.rs

Registered lanes: the trait for their rows, and the list, still empty, that later lessons add to.

`lessons/01/src/engine/gpu/lane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:registry"
```

## Step 60 · src/engine/gpu/lane.rs

The row tables of registered lanes, found by their type.

`lessons/01/src/engine/gpu/lane.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:lane-rows"
```

## Step 61 · src/engine/gpu/lane.rs

The row table test.

`lessons/01/src/engine/gpu/lane.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/lane.rs:tests"
```

## Step 62 · src/engine/gpu/pass.rs

What every pass of one frame shares.

`lessons/01/src/engine/gpu/pass.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:frame"
```

## Step 63 · src/engine/gpu/pass.rs

The Pass trait: the hooks an optional stage of the frame may fill.

`lessons/01/src/engine/gpu/pass.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:pass-trait"
```

## Step 64 · src/engine/gpu/pass.rs

The pass list, empty until a later lesson adds a pass, and the stand-in used while one runs.

`lessons/01/src/engine/gpu/pass.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:passes"
```

## Step 65 · src/engine/gpu/pass.rs

Run every pass in order, and find one pass by its type.

`lessons/01/src/engine/gpu/pass.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/pass.rs:each-pass"
```

## Step 66 · src/engine/gpu/instance.rs

One object row as the shaders read it, and its flag bits.

`lessons/01/src/engine/gpu/instance.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/instance.rs:instance"
```

## Step 67 · src/engine/gpu/instance.rs

The tests that the WGSL structs mirror the Rust ones.

`lessons/01/src/engine/gpu/instance.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/instance.rs:tests"
```

## Step 68 · src/engine/gpu/hull.rs

Vector helpers, and the point that is farthest by some measure.

`lessons/01/src/engine/gpu/hull.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:math"
```

## Step 69 · src/engine/gpu/hull.rs

Points read straight out of GPU rows.

`lessons/01/src/engine/gpu/hull.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:points"
```

## Step 70 · src/engine/gpu/hull.rs

Keep only the points a turned box can touch: the corners of their convex hull.

`lessons/01/src/engine/gpu/hull.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:extreme"
```

## Step 71 · src/engine/gpu/hull.rs

The world box of those points under a placement.

`lessons/01/src/engine/gpu/hull.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:boxes"
```

## Step 72 · src/engine/gpu/hull.rs

The hull tests.

`lessons/01/src/engine/gpu/hull.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/hull.rs:tests"
```

## Step 73 · src/engine/gpu/upload.rs

Every lane's rows for one file, collected on the CPU and uploaded together.

`lessons/01/src/engine/gpu/upload.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/upload.rs:upload"
```

## Step 74 · src/engine/gpu/objects.rs

The object row the CPU builds, and what moving the anchor reports.

`lessons/01/src/engine/gpu/objects.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:rows"
```

## Step 75 · src/engine/gpu/objects.rs

World boxes, the contact radius, and positions measured from the anchor.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:boxes"
```

## Step 76 · src/engine/gpu/objects.rs

The object table: rows on the GPU, exact positions and boxes on the CPU.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:table"
```

## Step 77 · src/engine/gpu/objects.rs

Create the table and append one upload's rows.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:table-append"
```

## Step 78 · src/engine/gpu/objects.rs

Move the anchor when the camera drifts far, and rewrite every relative position.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:anchor"
```

## Step 79 · src/engine/gpu/objects.rs

Flag the objects the camera is inside, and add the gumball's identity row.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:inside"
```

## Step 80 · src/engine/gpu/objects.rs

Move, replace and track single rows, writing only the row that changed.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:placement"
```

## Step 81 · src/engine/gpu/objects.rs

Hide deleted rows for good or until an undo, never freeing them, and grow a row's box.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:bury"
```

## Step 82 · src/engine/gpu/objects.rs

Colours and flags, reset and release, and the questions the rest of the viewer asks the table.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:queries"
```

## Step 83 · src/engine/gpu/objects.rs

The object table tests.

`lessons/01/src/engine/gpu/objects.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:tests"
```

## Step 84 · src/engine/gpu/objects.rs

The object table is a lane too.

`lessons/01/src/engine/gpu/objects.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/objects.rs:table-lane"
```

## Step 85 · src/engine/gpu/backdrop.rs

The backdrop lane: its shader, its pipeline, and one fullscreen triangle drawn through the three bind groups.

`lessons/01/src/engine/gpu/backdrop.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/backdrop.rs:backdrop"
```

## Step 86 · src/engine/gpu/backdrop.rs

The background pipeline skips the depth test, and the lane rebuilds it for a new target.

`lessons/01/src/engine/gpu/backdrop.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/backdrop.rs:background-pipeline"
```

## Step 87 · src/shaders/background.wgsl

Three corners that cover the screen, filled white, or light grey while ambient occlusion is on.

`lessons/01/src/shaders/background.wgsl` · type this, new file

```wgsl
--8<-- "lessons/01/src/shaders/background.wgsl"
```

![Clip space is a square from -1 to +1 with y up; the viewport transform turns it into pixels with y down.](illustrations/clip-space.svg)

## Step 88 · src/engine/gpu/render.rs

Encode one frame: each pass prepares, the face pass draws, then the passes after the faces.

`lessons/01/src/engine/gpu/render.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/render.rs:encode"
```

## Step 89 · src/engine/gpu/render.rs

Open the face pass, draw the backdrop unless a pass already did, then the faces.

`lessons/01/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/render.rs:face-passes"
```

## Step 90 · src/engine/gpu/present.rs

Write the frame uniforms before anything is encoded.

`lessons/01/src/engine/gpu/present.rs` · type this, new file

```rust
--8<-- "lessons/01/src/engine/gpu/present.rs:frame-uniforms"
```

## Step 91 · src/engine/gpu/present.rs

Draw one frame to the canvas: take its texture, encode, submit, present, and time it.

`lessons/01/src/engine/gpu/present.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/present.rs:present"
```

## Step 92 · src/engine/gpu/present.rs

Draw one frame into a texture and read its pixels back, for native tests.

`lessons/01/src/engine/gpu/present.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/present.rs:offscreen"
```

## Step 93 · src/engine/gpu/mod.rs

Bring in the lane types, and re-export the ones the rest of the viewer names.

`lessons/01/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:uses"
```

## Step 94 · src/engine/gpu/mod.rs

The Gpu struct: the device, the frame and one field per lane.

`lessons/01/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-struct"
```

## Step 95 · src/engine/gpu/mod.rs

One list of the lanes, and two macros that loop over it, shared or mutable.

`lessons/01/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:lane-list"
```

## Step 96 · src/engine/gpu/mod.rs

Count the GPU bytes, and open the GPU with every lane and pass built empty.

`lessons/01/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-bytes"

--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-new"

--8<-- "lessons/01/src/engine/gpu/mod.rs:gpu-build"
```

## Step 97 · src/engine/gpu/mod.rs

Upload a scene, choose the sample count, resize, and forget or free every lane's rows.

`lessons/01/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:set-scene"

--8<-- "lessons/01/src/engine/gpu/mod.rs:retarget"

--8<-- "lessons/01/src/engine/gpu/mod.rs:msaa"

--8<-- "lessons/01/src/engine/gpu/mod.rs:resize"

--8<-- "lessons/01/src/engine/gpu/mod.rs:reset"

--8<-- "lessons/01/src/engine/gpu/mod.rs:release"
```

## Step 98 · src/engine/gpu/mod.rs

The shader list the tests read.

`lessons/01/src/engine/gpu/mod.rs` · copy this part, append at the end of the file

```rust
--8<-- "lessons/01/src/engine/gpu/mod.rs:lane-shaders"
```

Run `cargo check` in `lessons/01/`.

## Check

`cargo check` compiles, with warnings about code nothing calls yet. `cargo xtest` runs 26 unit tests; the two that need a real GPU are ignored unless you add `--ignored`. The browser shows nothing new: lesson 12 opens the window that asks this renderer for its first frame, a white background.

## Next

[02 · Camera](02-camera.md)
