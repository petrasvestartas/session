# 18 · Finite-triangle visibility and maintained viewer convergence
<!-- locator: off -->

Concave boundaries and contact seams stay continuous while genuinely covered edges remain hidden.

![The depth plane continues beyond the finite triangle; only a finite nearer hit can hide the axis.](illustrations/finite-triangle.svg)

<!-- step-status: start -->

**Does it compile yet?** `cargo check` passes after steps 1–7 and 11; steps 8–10 fail and build again at step 11.

<!-- step-status: end -->

Download each file to the path shown.
<!-- supplied: 18 -->
## Step 1 · src/shaders/physical.wgsl
Physical outputs store depth information beside face color. The fragment output locations must match both attachments.
<!-- file: 18 session_viewer/src/shaders/physical.wgsl type -->
## Step 2 · src/shaders/triangle.wgsl
The mesh shader places vertices and shades visible faces. Its instance row must be the row uploaded with that vertex.
<!-- file: 18 session_viewer/src/shaders/triangle.wgsl type -->
## Step 3 · src/shaders/background.wgsl
The background fills uncovered pixels. Leave the physical depth gradient empty because the backdrop is not scene geometry.
<!-- file: 18 session_viewer/src/shaders/background.wgsl type -->
## Step 4 · src/shaders/grid.wgsl

The grid generates construction lines from vertex indices. Subtract the same camera anchor as the scene geometry.
<!-- file: 18 session_viewer/src/shaders/grid.wgsl type -->
## Step 5 · src/shaders/splat.wgsl

Point projection writes the nearest visible cloud samples. Keep source IDs attached to the winning samples.
<!-- file: 18 session_viewer/src/shaders/splat.wgsl type -->
## Step 6 · src/shaders/splat_resolve.wgsl

The resolve writes point color and depth into the scene. Background samples must leave geometry untouched.
<!-- file: 18 session_viewer/src/shaders/splat_resolve.wgsl type -->
## Step 7 · src/shaders/text_outline.wgsl

And for the resolve, the pass that writes the scene's depth for a cloud.
<!-- file: 18 session_viewer/src/shaders/text_outline.wgsl type -->
## Step 8 · src/shaders/projected_triangle.wgsl

Projected triangle helpers test both edge coverage and interpolated depth. Reject points outside the finite triangle.
<!-- file: 18 session_viewer/src/shaders/projected_triangle.wgsl type -->
## Step 9 · src/shaders/project_triangles.wgsl

The projection pass clips triangles and records their screen bounds. Clip at the near plane before dividing by w.
<!-- file: 18 session_viewer/src/shaders/project_triangles.wgsl type lines=1-90 -->
<!-- file: 18 session_viewer/src/shaders/project_triangles.wgsl type lines=91-174 -->
## Step 10 · src/shaders/triangle_tiles.wgsl

Tile passes count and store projected triangle references. Overflow must be explicit instead of writing past a tile range.
<!-- file: 18 session_viewer/src/shaders/triangle_tiles.wgsl type -->
## Step 11 · src/shaders/scan_triangle_tiles.wgsl

A prefix scan assigns each tile its reference range. Saturate counts before an overflow can wrap into a valid offset.
<!-- file: 18 session_viewer/src/shaders/scan_triangle_tiles.wgsl type -->
## Step 12 · src/engine/gpu/triangle_tiles.rs

Triangle tiles limit visibility queries to finite projected geometry. Invalidate the cache whenever projection or source placements change.
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=1-58 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=59-80 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=81-192 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=193-224 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=225-258 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=259-342 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=343-417 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=418-453 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=454-486 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=487-512 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=513-620 -->
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs type lines=621-628 -->
Download this part from its link to the path shown.
<!-- file: 18 session_viewer/src/engine/gpu/triangle_tiles.rs copy lines=629-824 -->
<!-- check: 18 -->
## Step 13 · src/shaders/ink_visibility.wgsl

The ink test decides which stroke samples are covered by faces. Compare depth in the same coordinate space as the face pass.
<!-- file: 18 session_viewer/src/shaders/ink_visibility.wgsl type hunks=1-11 -->
<!-- file: 18 session_viewer/src/shaders/ink_visibility.wgsl type hunks=12-13 -->
## Step 14 · src/engine/gpu/faces.rs

Face buffers preserve source face addresses alongside triangles. Several display triangles can name the same source face.
<!-- file: 18 session_viewer/src/engine/gpu/faces.rs type -->
## Step 15 · src/engine/gpu/arena.rs

The arena holds mesh vertices and indices across objects. Add the vertex base to local indices before appending a mesh.
<!-- file: 18 session_viewer/src/engine/gpu/arena.rs type -->
## Step 16 · src/engine/pipelines/layouts.rs

Bind-group layouts describe the resources shared by the drawing modules. Binding numbers and shader stages must agree with WGSL.
<!-- file: 18 session_viewer/src/engine/pipelines/layouts.rs type -->
## Step 17 · src/engine/pipelines/mod.rs

Pipeline descriptions keep color, depth and sample-count choices together. The attachment formats must match the render pass.
<!-- file: 18 session_viewer/src/engine/pipelines/mod.rs type -->
## Step 18 · src/engine/gpu/objects.rs

The object table stores GPU rows separately from source identity. Rebase translations before converting to f32 so distant objects stay stable.
<!-- file: 18 session_viewer/src/engine/gpu/objects.rs type -->
## Step 19 · src/engine/gpu/targets.rs

Targets own the depth and color attachments for a frame. Reversed depth clears to zero and compares nearer values as greater.
<!-- file: 18 session_viewer/src/engine/gpu/targets.rs type -->
## Step 20 · src/engine/gpu/pick.rs

Picking reads an object and subobject ID asynchronously. Reject replies from a superseded request.
<!-- file: 18 session_viewer/src/engine/gpu/pick.rs type -->
## Step 21 · src/engine/gpu/instance.rs

Each object row carries placement, color and selection flags for later interaction. Rust field offsets must match the shader byte for byte.
<!-- file: 18 session_viewer/src/engine/gpu/instance.rs type -->
## Step 22 · src/engine/gpu/present.rs

Presentation acquires the frame and submits rendering work. Handle a lost surface before requesting another frame.
<!-- file: 18 session_viewer/src/engine/gpu/present.rs type -->
## Step 23 · src/engine/gpu/surface_outline.rs

Surface masks add outlines around visible coverage. Reuse masks only while camera, geometry and selection are unchanged.
<!-- file: 18 session_viewer/src/engine/gpu/surface_outline.rs type -->
## Step 24 · src/engine/gpu/segments.rs

The segment buffers store strokes and the object rows they belong to. Preserve source IDs when expanding or joining segments.
<!-- file: 18 session_viewer/src/engine/gpu/segments.rs type -->
## Step 25 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes. Later passes must load the attachments written by earlier ones.
<!-- file: 18 session_viewer/src/engine/gpu/render.rs type -->
## Step 26 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 18 session_viewer/src/engine/gpu/mod.rs type -->
## Check

<!-- checkpoint: 18 -->

Expected: Concave boundaries and contact seams stay continuous while genuinely covered edges remain hidden; status: **the status names the selected object**.

![Checkpoint 18 with the supplied teapot: the rim and foot boundaries stay continuous from two camera positions, and the lid-to-body seams stay visible while a neighbouring face passes over their stroke fringe.](screenshots/18-teapot.png)

If it fails:

- A concave boundary breaks: an infinite plane is used outside its triangle.
- A stale silhouette remains: the cached mask is not invalidated by geometry or camera changes.

## What changed

<!-- tree: 18 session_viewer/src/engine -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 18](../lessons/18/index.md).

## Next

[19 · Sheets](19-sheets.md): drawings as one segment batch with lazy metadata.

## Expected viewer result

The full interaction fixture after the finite-triangle visibility changes. Use the teapot examples above to check the rim, foot and lid seams.

[![Full viewer result for 18 finite visibility](screenshots/18.png)](screenshots/18.png)
