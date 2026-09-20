# 04d · Point clouds
<!-- locator: off -->

A grid of blue points appears beside the mesh, polyline and marker.

![One node, one question: a spacing that projects wider than lod_px descends into the eight children, and one that fits draws the node whole.](illustrations/lod.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/engine/gpu/cloud.rs

Cloud buffers retain positions, attributes and source IDs. A displayed prefix must not renumber the original points.
<!-- file: 04d session_viewer/src/engine/gpu/cloud.rs type lines=1-48 -->
<!-- file: 04d session_viewer/src/engine/gpu/cloud.rs type lines=49-106 -->
<!-- file: 04d session_viewer/src/engine/gpu/cloud.rs type lines=107-136 -->
<!-- file: 04d session_viewer/src/engine/gpu/cloud.rs type lines=137-206 -->
<!-- file: 04d session_viewer/src/engine/gpu/cloud.rs type lines=207-258 -->
## Step 2 · src/engine/gpu/lod.rs

The cloud hierarchy chooses visible detail from projected size. Keep child ranges within the uploaded buffers.
<!-- file: 04d session_viewer/src/engine/gpu/lod.rs type lines=1-43 -->
<!-- file: 04d session_viewer/src/engine/gpu/lod.rs type lines=44-117 -->
<!-- file: 04d session_viewer/src/engine/gpu/lod.rs type lines=118-155 -->
## Step 3 · src/engine/gpu/splat.rs

The splat pass chooses visible points before compositing their color and depth. Invalidate cached results when the camera or point data changes.
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=1-56 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=57-122 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=123-145 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=146-218 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=219-266 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=267-340 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=341-356 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=357-451 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=452-509 -->
<!-- file: 04d session_viewer/src/engine/gpu/splat.rs type lines=510-523 -->
## Step 4 · src/shaders/splat.wgsl

Point projection writes the nearest visible cloud samples. Keep source IDs attached to the winning samples.
<!-- file: 04d session_viewer/src/shaders/splat.wgsl type lines=1-52 -->
<!-- file: 04d session_viewer/src/shaders/splat.wgsl type lines=53-101 -->
<!-- file: 04d session_viewer/src/shaders/splat.wgsl type lines=102-192 -->
## Step 5 · src/shaders/splat_resolve.wgsl

The resolve writes point color and depth into the scene. Background samples must leave geometry untouched.
<!-- file: 04d session_viewer/src/shaders/splat_resolve.wgsl type -->
<!-- check: 04d -->
## Step 6 · src/engine/pipelines/layouts.rs

Bind-group layouts describe the resources shared by the drawing modules. Binding numbers and shader stages must agree with WGSL.
<!-- file: 04d session_viewer/src/engine/pipelines/layouts.rs type -->
## Step 7 · src/engine/gpu/upload.rs

An upload collects object rows and geometry before sending them to the GPU. Keep local and global offsets distinct when appending.
<!-- file: 04d session_viewer/src/engine/gpu/upload.rs type -->
## Step 8 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 04d session_viewer/src/engine/gpu/mod.rs type -->
## Step 9 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 04d session_viewer/src/fixture.rs copy -->
## Step 10 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 04d session_viewer/src/lib.rs type -->
## Step 11 · index.html

Download this file from its link to the path shown.
<!-- file: 04d session_viewer/index.html copy -->
## Check

<!-- checkpoint: 04d -->

Expected: A grid of blue points appears beside the mesh, polyline and marker; status: **Checkpoint 04 · 4 objects**.

![Checkpoint 04d: a point cloud through the splat prelude and resolve, beside the mesh, stroke and marker lanes.](screenshots/04d.png)

If it fails:

- A cloud paints over a solid: the resolve does not write the winning point depth.
- Points change size on a high-DPI screen: CSS pixels and framebuffer pixels are mixed.

## What changed

<!-- tree: 04d session_viewer/src/engine -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 04d](../lessons/04d/index.md).

## Next

[05 · Depth and visible ink](05-visibility.md): the physical pass, the surface-carry visibility rule, and multisampling.

## Expected viewer result

Checkpoint 04d: a point cloud through the splat prelude and resolve, beside the mesh, stroke and marker lanes.

[![Full viewer result for 04d clouds](screenshots/04d.png)](screenshots/04d.png)
