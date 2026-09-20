# 04c · Markers
<!-- locator: off -->

An orange point appears below the triangle and fades smoothly at small sizes.

![A sphere is four template corners pushed out by the pixel radius plus half the feather; a free dot is one equilateral triangle whose incircle is the disc.](illustrations/markers.svg)

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · src/engine/gpu/glyphs.rs

The glyph buffers draw free dots and solid vertex markers. A marker retains its parent object row for later picking.
<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=1-49 -->
<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=50-93 -->
<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=94-147 -->
<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=148-208 -->
<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=209-234 -->
<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs type lines=235-290 -->
Download this part from its link to the path shown.
<!-- file: 04c session_viewer/src/engine/gpu/glyphs.rs copy lines=291-319 -->
## Step 2 · src/shaders/sphere.wgsl

Sphere impostors draw solid markers from small screen-space primitives. Write the sphere intersection depth instead of billboard depth.
<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=1-0 -->
<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=1-14 -->
<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=15-69 -->
<!-- file: 04c session_viewer/src/shaders/sphere.wgsl type lines=70-174 -->
## Step 3 · src/shaders/glyph.wgsl

The dot shader draws a marker with an antialiased edge. Keep the size floor and opacity fade together.
<!-- file: 04c session_viewer/src/shaders/glyph.wgsl type lines=1-0 -->
<!-- file: 04c session_viewer/src/shaders/glyph.wgsl type lines=1-11 -->
<!-- file: 04c session_viewer/src/shaders/glyph.wgsl type lines=12-45 -->
<!-- file: 04c session_viewer/src/shaders/glyph.wgsl type lines=46-153 -->
<!-- check: 04c -->
## Step 4 · src/engine/pipelines/mod.rs

Pipeline descriptions keep color, depth and sample-count choices together. The attachment formats must match the render pass.
<!-- file: 04c session_viewer/src/engine/pipelines/mod.rs type -->
## Step 5 · src/engine/pipelines/layouts.rs

Bind-group layouts describe the resources shared by the drawing modules. Binding numbers and shader stages must agree with WGSL.
<!-- file: 04c session_viewer/src/engine/pipelines/layouts.rs type -->
## Step 6 · src/engine/gpu/upload.rs

An upload collects object rows and geometry before sending them to the GPU. Keep local and global offsets distinct when appending.
<!-- file: 04c session_viewer/src/engine/gpu/upload.rs type -->
## Step 7 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources. Create resources before building the bind groups that refer to them.
<!-- file: 04c session_viewer/src/engine/gpu/mod.rs type -->
## Step 8 · src/fixture.rs

Download this file from its link to the path shown.
<!-- file: 04c session_viewer/src/fixture.rs copy -->
## Step 9 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners. Wire initialization and frame updates together so a new module actually runs.
<!-- file: 04c session_viewer/src/lib.rs type -->
## Step 10 · index.html

Download this file from its link to the path shown.
<!-- file: 04c session_viewer/index.html copy -->
## Check

<!-- checkpoint: 04c -->

Expected: An orange point appears below the triangle and fades smoothly at small sizes; status: **Checkpoint 04c · 3 objects**.

![Checkpoint 04c: vertex markers and free dots drawn from the glyph lane.](screenshots/04c.png)

If it fails:

- Dots vanish while zooming out: the half-pixel floor or alpha fade is missing.
- A sphere looks flat: the fragment depth still describes its billboard.

## What changed

<!-- tree: 04c session_viewer/src/engine -->

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: [source at checkpoint 04c](../lessons/04c/index.md).

## Next

[04d · Point clouds](04d-clouds.md): the cloud tables, the LOD walk, and the splat prelude that resolves into the face pass.

## Expected viewer result

Checkpoint 04c: vertex markers and free dots drawn from the glyph lane.

[![Full viewer result for 04c markers](screenshots/04c.png)](screenshots/04c.png)
