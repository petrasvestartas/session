# 05 · Depth and visible ink
<!-- locator: off -->

A grey box shows its visible red edges and black corners while its faces hide the far edges.

![Reversed depth, and why a thick stroke must transfer the surface depth to its axis before comparing.](illustrations/ink-visibility.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/shaders/physical.wgsl

Physical outputs store depth information beside face color. The fragment output locations must match both attachments.
<!-- file: 05 session_viewer/src/shaders/physical.wgsl type -->
## Step 2 · src/shaders/background.wgsl

The background fills uncovered pixels. Leave the physical depth gradient empty because the backdrop is not scene geometry.
<!-- file: 05 session_viewer/src/shaders/background.wgsl type -->
## Step 3 · src/shaders/grid.wgsl

The grid generates construction lines from vertex indices. Subtract the same camera anchor as the scene geometry.
<!-- file: 05 session_viewer/src/shaders/grid.wgsl type -->
## Step 4 · src/engine/gpu/backdrop.rs

The backdrop draws the background and construction grid. It must not overwrite the depth of scene geometry.
<!-- file: 05 session_viewer/src/engine/gpu/backdrop.rs type -->
<!-- check: 05 -->
## Step 5 · src/shaders/ink_visibility.wgsl

The ink test decides which stroke samples are covered by faces. Compare depth in the same coordinate space as the face pass.
<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=1-41 -->
<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=42-109 -->
<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=110-150 -->
<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=151-209 -->
<!-- file: 05 session_viewer/src/shaders/ink_visibility.wgsl type whole lines=210-282 -->
## Step 6 · src/shaders/triangle.wgsl

The mesh shader places vertices and shades visible faces. Its instance row must be the row uploaded with that vertex.
<!-- file: 05 session_viewer/src/shaders/triangle.wgsl type -->
## Step 7 · src/shaders/splat.wgsl

Point projection writes the nearest visible cloud samples. Keep source IDs attached to the winning samples.
<!-- file: 05 session_viewer/src/shaders/splat.wgsl type -->
## Step 8 · src/shaders/splat_resolve.wgsl

The resolve writes point color and depth into the scene. Background samples must leave geometry untouched.
<!-- file: 05 session_viewer/src/shaders/splat_resolve.wgsl type -->
## Step 9 · src/shaders/text_outline.wgsl

The resolve rejoins the private pass to the shared one: reads the drawing module's own depth and colour, lights each point from its neighbours, and writes `frag_depth` for the scene's depth test.
<!-- file: 05 session_viewer/src/shaders/text_outline.wgsl type -->
## Step 10 · src/engine/gpu/targets.rs

Targets own the depth and color attachments for a frame. Reversed depth clears to zero and compares nearer values as greater.
<!-- file: 05 session_viewer/src/engine/gpu/targets.rs type -->
## Step 11 · src/engine/pipelines/mod.rs

Pipeline descriptions keep color, depth and sample-count choices together. The attachment formats must match the render pass.
<!-- file: 05 session_viewer/src/engine/pipelines/mod.rs type -->
## Step 12 · src/engine/pipelines/layouts.rs

Bind-group layouts describe the resources shared by the drawing modules. Binding numbers and shader stages must agree with WGSL.
<!-- file: 05 session_viewer/src/engine/pipelines/layouts.rs type -->
## Step 13 · src/engine/gpu/instance.rs

Each object row carries placement, color and selection flags for later interaction. Rust field offsets must match the shader byte for byte.
<!-- file: 05 session_viewer/src/engine/gpu/instance.rs type -->
## Step 14 · src/engine/gpu/objects.rs

The object table stores GPU rows separately from source identity. Rebase translations before converting to f32 so distant objects stay stable.
<!-- file: 05 session_viewer/src/engine/gpu/objects.rs type -->
## Step 15 · src/engine/gpu/arena.rs

The arena holds mesh vertices and indices across objects. Add the vertex base to local indices before appending a mesh.
<!-- file: 05 session_viewer/src/engine/gpu/arena.rs type -->
## Step 16 · src/engine/gpu/splat.rs

The splat pass chooses visible points before compositing their color and depth. Invalidate cached results when the camera or point data changes.
<!-- file: 05 session_viewer/src/engine/gpu/splat.rs type -->
## Step 17 · src/engine/gpu/text_outline.rs

Outline text draws vector glyph geometry. Keep print fills separate from depth-writing solid faces.
<!-- file: 05 session_viewer/src/engine/gpu/text_outline.rs type -->
## Step 18 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 05 session_viewer/src/engine/gpu/mod.rs type -->
## Step 19 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 05 session_viewer/src/fixture.rs copy -->
## Step 20 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 05 session_viewer/src/lib.rs type -->
## Step 21 · index.html

Download this file from its link to the path shown.
<!-- file: 05 session_viewer/index.html copy -->
## Check

<!-- checkpoint: 05 -->

Expected: A grey box shows its visible red edges and black corners while its faces hide the far edges; status: **Checkpoint 05 · 1 objects**.

![Checkpoint 05: hidden lines stay hidden while visible strokes and corners stay readable, over the white backdrop.](screenshots/05.png)

If it fails:

- Every edge disappears: the depth clear and comparison disagree with reversed depth.
- Hidden edges show through: the face pipeline omits its physical depth gradient.

## What changed

<!-- tree: 05 session_viewer/src -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 05](../lessons/05/index.md).

## Next

[06 · CAD face rules](06-cad-rules.md): BRep faces, surfaces and boundary records flow from the shared kernel into display data.

## Expected viewer result

Checkpoint 05: hidden lines stay hidden while visible strokes and corners stay readable, over the white backdrop.

[![Full viewer result for 05 visibility](screenshots/05.png)](screenshots/05.png)
