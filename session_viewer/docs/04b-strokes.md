# 04b · Strokes
<!-- locator: off -->

A yellow polyline appears beside the blue triangle and keeps its width while zooming.

![Six vertices place a camera-facing quad around the projected axis, and band_area integrates one pixel box against the capsule so coverage is an area rather than a distance ramp.](illustrations/ribbon.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/engine/gpu/segments.rs

The segment buffers store strokes and the object rows they belong to. Preserve source IDs when expanding or joining segments.
<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=1-48 -->
<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=49-107 -->
<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=108-175 -->
<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=176-236 -->
<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=237-248 -->
<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=249-271 -->
Download this part from its link to the path shown.
<!-- file: 04b session_viewer/src/engine/gpu/segments.rs copy lines=272-312 -->
## Step 2 · src/shaders/ink_visibility.wgsl

The ink test decides which stroke samples are covered by faces. Compare depth in the same coordinate space as the face pass.
<!-- file: 04b session_viewer/src/shaders/ink_visibility.wgsl type -->
## Step 3 · src/shaders/ribbon.wgsl

The stroke shader expands segments into screen-space ribbons. Clip the near plane before dividing by w.
<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=1-0 -->
<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=1-17 -->
<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=18-25 -->
<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=26-97 -->
<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=98-158 -->
<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=159-214 -->
<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=215-333 -->
<!-- check: 04b -->
## Step 4 · src/engine/pipelines/mod.rs

Pipeline descriptions keep color, depth and sample-count choices together. The attachment formats must match the render pass.
<!-- file: 04b session_viewer/src/engine/pipelines/mod.rs type -->
## Step 5 · src/engine/pipelines/layouts.rs

Bind-group layouts describe the resources shared by the drawing modules. Binding numbers and shader stages must agree with WGSL.
<!-- file: 04b session_viewer/src/engine/pipelines/layouts.rs type -->
## Step 6 · src/engine/gpu/upload.rs

An upload collects object rows and geometry before sending them to the GPU. Keep local and global offsets distinct when appending.
<!-- file: 04b session_viewer/src/engine/gpu/upload.rs type -->
## Step 7 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 04b session_viewer/src/engine/gpu/mod.rs type -->
## Step 8 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 04b session_viewer/src/fixture.rs copy -->
## Step 9 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 04b session_viewer/src/lib.rs type -->
## Step 10 · index.html

Download this file from its link to the path shown.
<!-- file: 04b session_viewer/index.html copy -->
## Check

<!-- checkpoint: 04b -->

Expected: A yellow polyline appears beside the blue triangle and keeps its width while zooming; status: **Checkpoint 04b · 2 objects**.

![Checkpoint 04b: strokes expanded on the GPU into screen-space ribbons beside the mesh.](screenshots/04b.png)

If it fails:

- Lines change width with distance: the offset is applied before the perspective divide.
- A line disappears near the camera: a segment endpoint crosses the near plane without clipping.

## What changed

<!-- tree: 04b session_viewer/src/engine -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 04b](../lessons/04b/index.md).

## Next

[04c · Markers](04c-markers.md): vertex markers on a quad template and free dots as SDF triangles.

## Expected viewer result

Checkpoint 04b: strokes expanded on the GPU into screen-space ribbons beside the mesh.

[![Full viewer result for 04b strokes](screenshots/04b.png)](screenshots/04b.png)
