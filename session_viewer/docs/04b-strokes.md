# 04b · Strokes

## You are building

![Diagram: segment rows reach ribbon.wgsl through group 3, become a screen-space quad, and are shaded by exact coverage tested against the scene depth.](illustrations/04b-01.svg)

Group 3 of the segment pipelines (`Layouts::segment_rows`):

| Binding | Rust buffer | WGSL |
|---|---|---|
| 0 | `SegTable.buf` 40-byte rows | `@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>` |
| 1 | `SegTable.ids` one `u32` per row | `@group(3) @binding(1) var<storage, read> source_edges: array<u32>` |
| 2 | `SegmentLane.selection` uniform | `@group(3) @binding(2) var<uniform> edge_selection: vec4<u32>` |

![Six vertices place a camera-facing quad around the projected axis, and band_area integrates one pixel box against the capsule so coverage is an area rather than a distance ramp.](illustrations/ribbon.svg)

## Starting point

- Checkpoint 04a: one mesh drawn through the arena; the ink pass exists but draws nothing of its own.
- A stroke is a camera-facing quad per segment.
- The vertex stage pulls six vertices by index from the segment table.
- The fragment stage computes exact pixel coverage of a capsule, then asks the physical depth whether the axis is visible.

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · The segment row

![Where this step sits in the viewer: Lanes, with 8 of 11 zones built so far.](illustrations/locator-be21b3fc34.svg){ .locator data-strip="illustrations/strip-445a1edf20.svg" }

- `radius` 0 means the screen-constant pen; `facing` packs two face normals for the solid lane's back-edge cull.

![Diagram: walk · segment endpoints · CylinderSegment\ p0 · p1 · radius · facing · segment table](illustrations/04b-02.svg)

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=1-56 -->

## Step 2 · The lane

![Where this step sits in the viewer: Lanes, with 8 of 11 zones built so far.](illustrations/locator-be21b3fc34.svg){ .locator data-strip="illustrations/strip-445a1edf20.svg" }

- Two tables of the same row: pipes (mesh edges, culled by facing) and ribbons (free linework, always drawn).

![Diagram: SegRows\ pipes · ribbons · SegmentLane · ink pass](illustrations/04b-03.svg)

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=57-115 -->

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=116-180 -->

- Every draw is `RIBBON_VERTS * rows` vertices with no vertex buffer bound; the shader indexes the table.

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=181-240 -->

- The lane reports whether it holds any solid rows: 4x is spent only when hard edges exist on the GPU, and only if the canvas fits the adapter's budget.

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=241-252 -->

- `DepthMode::Always` with blending: the shader decides visibility itself, so no hardware depth test can hide a stroke that lies on a surface.

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=253-275 -->

<span class="zone-mark" data-strip="illustrations/strip-445a1edf20.svg" data-zone="Lanes"></span>

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs copy lines=276-314 -->

## Step 3 · The shared visibility rule

![Where this step sits in the viewer: Shaders, with 8 of 11 zones built so far.](illustrations/locator-53c0d29f7b.svg){ .locator data-strip="illustrations/strip-ef21ae124d.svg" }

- Appended to every ink shader by `ink_module`. It compares the scene depth at the pixel with the axis depth.

![Diagram: scene depth · group 2 · ink_visible · axis depth · ink fragment](illustrations/04b-04.svg)

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ink_visibility.wgsl type -->

## Step 4 · The ribbon shader

![Where this step sits in the viewer: Shaders, with 8 of 11 zones built so far.](illustrations/locator-53c0d29f7b.svg){ .locator data-strip="illustrations/strip-ef21ae124d.svg" }

- Bindings and constants. `LineUniform` is the same block as `triangle.wgsl`.

![A value handed from the vertex shader to the fragment shader is blended perspective-correctly; marked flat it is not blended at all, which is how a stroke's half-width travels.](illustrations/interpolate.svg)

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=1-4 -->


<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=5-17 -->

- The two half-widths travel flat, one scalar per end, and resolve per pixel, because a per-vertex width is projective over a trapezoid.

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=18-25 -->

- Clip against the near plane before any divide; a hand divide behind the eye mirrors the point through the screen centre.

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=26-93 -->

- The fragment: coverage times fade, then `ink_visible` at the closest axis point.

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=94-140 -->

- `band_area` integrates the pixel box against the capsule instead of sampling a distance: the figure above is that integral.

![A stroke's alpha is the exact area of one pixel square inside the capsule, and that area is one trapezoid's CDF evaluated at hw minus d and at hw plus d.](illustrations/band-coverage.svg)

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=141-190 -->

- The ID entries write `(row + 1, segment + 1)`; a tag bit in the segment half tells a picked ribbon from a picked face in the same channel.

<span class="zone-mark" data-strip="illustrations/strip-ef21ae124d.svg" data-zone="Shaders"></span>

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=191-293 -->

<!-- check: 04b -->

## Step 5 · Wire the lane

![Where this step sits in the viewer: Page, Shell, GPU core, with 8 of 11 zones built so far.](illustrations/locator-771239ee6f.svg){ .locator data-strip="illustrations/strip-20ba8a8ce6.svg" }

- `ink_module` compiles a lane shader with the visibility rule appended.

![Diagram: Upload.seg · Gpu.segments · segment_rows layout · strokes drawn](illustrations/04b-05.svg)

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 04b session_viewer/src/engine/pipelines/mod.rs type -->

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 04b session_viewer/src/engine/pipelines/layouts.rs type -->

- Every layout is built once per device and lives here, so a group number is decided in one file.

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 04b session_viewer/src/engine/gpu/upload.rs type -->

- The ink pass binds group 2 through `objects.ink_group`, the variant that carries the physical depth.

<span class="zone-mark" data-strip="illustrations/strip-203427a3dc.svg" data-zone="GPU core"></span>

<!-- file: 04b session_viewer/src/engine/gpu/mod.rs type -->

- A second object row with a three-segment polyline.

<span class="zone-mark" data-strip="illustrations/strip-6e964d1d1f.svg" data-zone="Shell"></span>

<!-- file: 04b session_viewer/src/fixture.rs copy -->

<span class="zone-mark" data-strip="illustrations/strip-6e964d1d1f.svg" data-zone="Shell"></span>

<!-- file: 04b session_viewer/src/lib.rs type -->

- The shell's only change is the status line: every lane reports its own count, and the checkpoint test reads that JSON, not a screenshot.

<span class="zone-mark" data-strip="illustrations/strip-a7bdebbf9f.svg" data-zone="Page"></span>

<!-- file: 04b session_viewer/index.html copy -->

## Check

<!-- checkpoint: 04b -->

Expected:

- The blue triangle plus a thin yellow polyline to its right.
- Status reads **Checkpoint 04b · 2 objects**.
- Zoom in: the line keeps its on-screen width.

![Checkpoint 04b: strokes expanded on the GPU into screen-space ribbons beside the mesh.](screenshots/04b.png)

## What changed

<!-- tree: 04b session_viewer/src/engine -->

- Data flow: `SegRows` → `SegTable` buffers → group 3 → `ribbon.wgsl` → blended ink over the face pass's depth.

**Production equivalent:** `src/engine/gpu/segments.rs`, `src/shaders/ribbon.wgsl`. Production keeps the visibility rule in `src/shaders/ink_visibility.wgsl`.

## Try

- Append `?thickness=4`: every stroke widens on screen while the geometry stays put — the pen is applied in `ribbon.wgsl`, not in the vertex data.
- Zoom far out: the strokes keep their pixel width. A world-space width would vanish; a screen-space pen does not.
- Set `?thickness=0.2`: `floor_hairline` holds the stroke at half a pixel and `hairline_fade` floors its alpha at `HAIRLINE_MIN_ALPHA`, so it thins instead of disappearing. (`?aa=` feathers markers and dots, not ribbons — `ribbon.wgsl` never reads `line.feather`.)

## Questions and answers

**The segment row ends in flat `f32`s instead of two `vec3`s. What would the `vec3`s cost?**

*How to work it out.* Apply lesson 03's alignment rule: a `vec3` holds 12 bytes but aligns to 16. Count the row both ways, then ask what the `vec3` form buys when the shader reads the components individually anyway.

*The answer.* Eight bytes a row, 40 to 48, for no benefit. Predicting this rather than discovering it is the point: the same rule set `Instance` at 96 and will set the marker row at 48.

**Strokes draw with `DepthMode::Always` and blending. Why not simply depth-test them?**

*How to work it out.* A stroke sits on the edge of its own face, at that face's depth. A depth test between two fragments at the same depth is a per-pixel coin flip decided by float rounding, and it changes as the camera moves.

*The answer.* Hardware depth testing at equal depth produces stitching, so the shader decides visibility itself: `ink_visible` compares the scene depth at the pixel against the depth of the closest point on the stroke's axis, read straight out of the depth the face pass wrote. One shared file keeps every ink lane answering the question the same way.

**The half-width at each end travels as a flat scalar, resolved per pixel. What breaks if you interpolate a width per vertex instead?**

*How to work it out.* A stroke going away from the camera is a trapezoid on screen, wide near, narrow far. Interpolation across it is perspective-correct for *positions*, but a width is not a position — it is a screen-space quantity derived from one.

*The answer.* The width comes out wrong in the middle and wobbles as the camera moves. Sending both ends flat and computing the width per pixel is exact — hence `@interpolate(flat)` on the outputs.

**Why must the segment be clipped against the near plane before any divide?**

*How to work it out.* Write out the divide: `x/w`, `y/w`. When `w` is negative both signs flip, so the point appears mirrored through the screen centre instead of absent.

*The answer.* A line crossing behind the eye would swing across the canvas rather than disappear. Clip first, divide second: the bug otherwise appears only when you walk the camera into geometry.

**What you should be able to do now**

Say in two sentences why a stroke keeps its pixel width when you zoom out, and where that decision is applied. Correct: the vertex stage expands the segment into a quad whose half-width is computed in *screen* space from `line.thickness`, so world distance never enters it — the pen is applied in `ribbon.wgsl`, not in the vertex data. Then predict the CPU alternative: rebuilding and re-uploading the geometry on every camera move.

## Next

[04c · Markers](04c-markers.md): vertex markers on a quad template and free dots as SDF triangles.
