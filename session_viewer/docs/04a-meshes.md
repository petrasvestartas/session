# 04a · Meshes on the GPU
<!-- locator: off -->

A blue triangle draws from mesh buffers while the camera still orbits and zooms.

![One growable arena holds every mesh's vertices and a parallel table gives every vertex its object row; a mesh is a range of indices, and a draw binds both vertex buffers, binds one index run and calls draw_indexed.](illustrations/arena.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/engine/gpu/buffers.rs
Growable buffers keep existing rows while new geometry arrives. Rebuild bindings whenever an allocation moves.
<!-- file: 04a session_viewer/src/engine/gpu/buffers.rs type lines=1-35 -->
<!-- file: 04a session_viewer/src/engine/gpu/buffers.rs type lines=36-100 -->
<!-- file: 04a session_viewer/src/engine/gpu/buffers.rs type lines=101-124 -->
<!-- file: 04a session_viewer/src/engine/gpu/buffers.rs type lines=125-187 -->
<!-- file: 04a session_viewer/src/engine/gpu/buffers.rs type lines=188-209 -->
## Step 2 · src/engine/pipelines/layouts.rs
Bind-group layouts describe the resources shared by the drawing modules. Binding numbers and shader stages must agree with WGSL.
<!-- file: 04a session_viewer/src/engine/pipelines/layouts.rs type lines=1-48 -->
<!-- file: 04a session_viewer/src/engine/pipelines/layouts.rs type lines=49-102 -->
## Step 3 · src/engine/pipelines/mod.rs
Pipeline descriptions keep color, depth and sample-count choices together. The attachment formats must match the render pass.
<!-- file: 04a session_viewer/src/engine/pipelines/mod.rs type lines=1-48 -->
<!-- file: 04a session_viewer/src/engine/pipelines/mod.rs type lines=49-103 -->
<!-- file: 04a session_viewer/src/engine/pipelines/mod.rs type lines=104-168 -->
<!-- file: 04a session_viewer/src/engine/pipelines/mod.rs type lines=169-191 -->
<!-- file: 04a session_viewer/src/engine/pipelines/mod.rs type lines=192-223 -->
<!-- file: 04a session_viewer/src/engine/pipelines/mod.rs type lines=224-274 -->
## Step 4 · src/shaders/scene.wgsl
Shared shader declarations mirror the object rows and frame uniforms. Preserve their layout when later features add flags.
<!-- file: 04a session_viewer/src/shaders/scene.wgsl type -->
## Step 5 · src/engine/gpu/instance.rs
Each object row carries placement, color and selection flags for later interaction. Rust field offsets must match the shader byte for byte.
<!-- file: 04a session_viewer/src/engine/gpu/instance.rs type -->
## Step 6 · src/shaders/normals.wgsl
Normal helpers keep lighting correct under transformed placements. Nonuniform scale requires the inverse transpose.
<!-- file: 04a session_viewer/src/shaders/normals.wgsl type -->
<!-- check: 04a -->
## Step 7 · src/engine/gpu/targets.rs
Targets own the depth and color attachments for a frame. Reversed depth clears to zero and compares nearer values as greater.
<!-- file: 04a session_viewer/src/engine/gpu/targets.rs type lines=1-67 -->
<!-- file: 04a session_viewer/src/engine/gpu/targets.rs type lines=68-133 -->
<!-- file: 04a session_viewer/src/engine/gpu/targets.rs type lines=134-163 -->
## Step 8 · src/engine/gpu/frame.rs
Frame uniforms carry the camera and screen dimensions to every drawing module. Apply device pixel ratio once when converting CSS sizes.
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=1-39 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=40-98 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=99-169 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=170-192 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=193-240 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=241-320 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=321-338 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=339-383 -->
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs type lines=384-401 -->
Download this part from its link to the path shown.
<!-- file: 04a session_viewer/src/engine/gpu/frame.rs copy lines=402-408 -->
## Step 9 · src/engine/gpu/view.rs
Download this file from its link to the path shown.
<!-- file: 04a session_viewer/src/engine/gpu/view.rs copy -->
## Step 10 · src/app/mod.rs
The application module connects source loading and interaction helpers. Declare each file before importing it elsewhere.
<!-- file: 04a session_viewer/src/app/mod.rs type -->
## Step 11 · src/app/route.rs
Route helpers read viewer options from the page URL. Missing options must retain usable defaults.
<!-- file: 04a session_viewer/src/app/route.rs type -->
## Step 12 · src/engine/gpu/objects.rs
The object table stores GPU rows separately from source identity. Rebase translations before converting to f32 so distant objects stay stable.
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=1-21 -->
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=22-58 -->
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=59-108 -->
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=109-179 -->
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=180-250 -->
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=251-316 -->
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=317-371 -->
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs type lines=372-432 -->
Download this part from its link to the path shown.
<!-- file: 04a session_viewer/src/engine/gpu/objects.rs copy lines=433-482 -->
<!-- check: 04a -->
## Step 13 · src/shaders/triangle.wgsl
The mesh shader places vertices and shades visible faces. Its instance row must be the row uploaded with that vertex.
<!-- file: 04a session_viewer/src/shaders/triangle.wgsl type lines=1-21 -->
<!-- file: 04a session_viewer/src/shaders/triangle.wgsl type lines=22-63 -->
<!-- file: 04a session_viewer/src/shaders/triangle.wgsl type lines=64-121 -->
<!-- file: 04a session_viewer/src/shaders/triangle.wgsl type lines=122-130 -->
## Step 14 · src/engine/gpu/arena.rs
The arena holds mesh vertices and indices across objects. Add the vertex base to local indices before appending a mesh.
<!-- file: 04a session_viewer/src/engine/gpu/arena.rs type lines=1-59 -->
<!-- file: 04a session_viewer/src/engine/gpu/arena.rs type lines=60-110 -->
<!-- file: 04a session_viewer/src/engine/gpu/arena.rs type lines=111-158 -->
<!-- file: 04a session_viewer/src/engine/gpu/arena.rs type lines=159-225 -->
## Step 15 · src/shaders/text_outline.wgsl
Download this file from its link to the path shown.
<!-- file: 04a session_viewer/src/shaders/text_outline.wgsl copy -->
## Step 16 · src/engine/gpu/text_outline.rs
Outline text draws vector glyph geometry. Keep print fills separate from depth-writing solid faces.
<!-- file: 04a session_viewer/src/engine/gpu/text_outline.rs type lines=1-49 -->
<!-- file: 04a session_viewer/src/engine/gpu/text_outline.rs type lines=50-61 -->
<!-- file: 04a session_viewer/src/engine/gpu/text_outline.rs type lines=62-109 -->
Download this part from its link to the path shown.
<!-- file: 04a session_viewer/src/engine/gpu/text_outline.rs copy lines=110-246 -->
## Step 17 · src/engine/gpu/upload.rs
An upload collects object rows and geometry before sending them to the GPU. Keep local and global offsets distinct when appending.
<!-- file: 04a session_viewer/src/engine/gpu/upload.rs type -->
## Step 18 · src/fixture.rs
Download this file from its link to the path shown.
<!-- file: 04a session_viewer/src/fixture.rs copy -->
## Step 19 · src/engine/gpu/mod.rs
The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 04a session_viewer/src/engine/gpu/mod.rs type whole lines=1-35 -->
<!-- file: 04a session_viewer/src/engine/gpu/mod.rs type whole lines=36-99 -->
<!-- file: 04a session_viewer/src/engine/gpu/mod.rs type whole lines=100-145 -->
<!-- file: 04a session_viewer/src/engine/gpu/mod.rs type whole lines=146-180 -->
## Step 20 · src/engine/mod.rs
The engine module exposes the rendering implementation. A missing module declaration leaves its file outside the build.
<!-- file: 04a session_viewer/src/engine/mod.rs type -->
## Step 21 · src/lib.rs
The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 04a session_viewer/src/lib.rs type whole lines=1-54 -->
<!-- file: 04a session_viewer/src/lib.rs type whole lines=55-103 -->
## Step 22 · src/scene.rs
Remove this file; its replacement is now part of the rendering modules.
<!-- file: 04a session_viewer/src/scene.rs -->
## Step 23 · src/shaders/first.wgsl
Remove this file; its replacement is now part of the rendering modules.
<!-- file: 04a session_viewer/src/shaders/first.wgsl -->
## Step 24 · index.html
Download this file from its link to the path shown.
<!-- file: 04a session_viewer/index.html copy -->
<!-- check: 04a -->
## Check

<!-- checkpoint: 04a -->

Expected: A blue triangle draws from mesh buffers while the camera still orbits and zooms; status: **Checkpoint 04a · 1 objects**.

![Checkpoint 04a: the first mesh drawn from arena buffers through the object table.](screenshots/04a.png)

If it fails:

- The canvas stays empty: the surface is not configured or the arena has no rows.
- Geometry is scrambled: the vertex stride or object row layout differs from the shader.

## What changed

<!-- tree: 04a session_viewer/src -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 04a](../lessons/04a/index.md).

## Next
[04b · Strokes](04b-strokes.md): the segment drawing module, `ribbon.wgsl`, and the shared ink visibility rule.

## Expected viewer result

Checkpoint 04a: the first mesh drawn from arena buffers through the object table.

[![Full viewer result for 04a meshes](screenshots/04a.png)](screenshots/04a.png)
