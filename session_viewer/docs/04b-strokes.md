# 04b · Strokes

## You are building

```mermaid
flowchart TB
    R["fixture: CylinderSegment rows"] --> S["SegRows<br/>pipes · ribbons"]
    S -- "SegmentLane::append" --> G3["group 3<br/>segments · source_edges · edge_selection"]
    G3 --> V["ribbon.wgsl vs_main<br/>6 verts per segment, no vertex buffer"]
    V --> Q["screen-space quad<br/>+ FILTER_REACH"]
    Q --> FS["fs_main<br/>band_area coverage · ink_visible"]
    D["physical depth<br/>group 2 bindings 2, 3"] --> FS
```

Group 3 of the segment pipelines (`Layouts::segment_rows`):

| Binding | Rust buffer | WGSL |
|---|---|---|
| 0 | `SegTable.buf` 40-byte rows | `@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>` |
| 1 | `SegTable.ids` one `u32` per row | `@group(3) @binding(1) var<storage, read> source_edges: array<u32>` |
| 2 | `SegmentLane.selection` uniform | `@group(3) @binding(2) var<uniform> edge_selection: vec4<u32>` |

![Six vertices place a camera-facing quad around the projected axis, and band_area integrates one pixel box against the capsule so coverage is an area rather than a distance ramp.](illustrations/ribbon.svg)

## Starting point

- Checkpoint 04a: one mesh drawn through the arena; the ink pass exists but draws nothing of its own.
- A stroke is a camera-facing quad per segment. The vertex stage pulls six vertices by index from the segment table; the fragment stage computes exact pixel coverage of a capsule and asks the physical depth whether the axis is visible.

<!-- step-status: start -->

**Does it compile yet?** Yes, after every step of this lesson — `cargo check` was run at the end of each one to make sure. A step that writes a file Rust has not been told about yet compiles without checking any of it, so keep going to the checkpoint: that build is the real test.

<!-- step-status: end -->

## Step 1 · The segment row

- 40 bytes, ends as flat `f32`s: a `vec3` would pad the row to 48.
- `radius` 0 means the screen-constant pen; `facing` packs two face normals for the solid lane's back-edge cull.

```mermaid
flowchart TB
    W["walk · segment endpoints"] --> R["CylinderSegment<br/>a · b · radius · facing"]
    R -- "40 B · storage" --> T["segment table"]
    style R fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=1-56 -->

## Step 2 · The lane

- Two tables of the same row: pipes (mesh edges, culled by facing) and ribbons (free linework, always drawn).

```mermaid
flowchart LR
    SR["SegRows<br/>pipes · ribbons"] -- "append" --> SL["SegmentLane"]
    SL -- "draw_pipes · culled" --> P["ink pass"]
    SL -- "draw_ribbons · always" --> P
    style SL fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=57-115 -->

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=116-180 -->

- Every draw is `RIBBON_VERTS * rows` vertices with no vertex buffer bound; the shader indexes the table.

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=181-240 -->

- The lane reports its solid-lane row count because the MSAA policy reads it: sample count is chosen from how much geometry is on screen.

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=241-252 -->

- `DepthMode::Always` with blending: the shader decides visibility itself, so no hardware depth test can hide a stroke that lies on a surface.

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs type lines=253-275 -->

<!-- file: 04b session_viewer/src/engine/gpu/segments.rs copy lines=276-314 -->

## Step 3 · The shared visibility rule

- Appended to every ink shader by `ink_module`. It compares the scene depth at the pixel with the axis depth.

```mermaid
flowchart LR
    D["scene depth · group 2"] --> V["ink_visible"]
    A["axis depth"] --> V
    V -- "keep / discard" --> F["ink fragment"]
    style V fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04b session_viewer/src/shaders/ink_visibility.wgsl type -->

## Step 4 · The ribbon shader

- Bindings and constants. `LineUniform` is the same block as `triangle.wgsl`.

![A value handed from the vertex shader to the fragment shader is blended perspective-correctly; marked flat it is not blended at all, which is how a stroke's half-width travels.](illustrations/interpolate.svg)

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=1-4 -->

- `band_area` integrates the pixel box against the capsule exactly, so coverage cannot beat with the line's subpixel phase the way a distance ramp does.

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=5-17 -->

- Per-vertex outputs are flat: the half-width at each end goes down as two scalars and is resolved per pixel, because a per-vertex width is projective over a trapezoid.

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=18-25 -->

- Clip against the near plane before any divide; a hand divide behind the eye mirrors the point through the screen centre.

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=26-90 -->

- The fragment: coverage times fade, then `ink_visible` at the closest axis point. `fs_id` and `fs_edge_id` write `(row + 1, segment + 1)` for picking.

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=91-137 -->

- `coverage` is where the exactness lives: `band_area` integrates the pixel box against the capsule instead of sampling a distance, so coverage cannot beat against the line's subpixel phase.

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=138-187 -->

- The ID entries write `(row + 1, segment + 1)`, and the segment half carries a tag bit so a picked ribbon can be told from a picked face in the same channel.

<!-- file: 04b session_viewer/src/shaders/ribbon.wgsl type lines=188-280 -->

<!-- check: 04b -->

## Step 5 · Wire the lane

- `ink_module` compiles a lane shader with the visibility rule appended.

```mermaid
flowchart TB
    U["Upload.seg"] -- "set_scene" --> G["Gpu.segments"]
    L["segment_rows layout"] --> G
    G -- "ink pass · ink_group" --> P["strokes drawn"]
    style G fill:#f0bcdb,stroke:#ce4095,color:#111
```

<!-- file: 04b session_viewer/src/engine/pipelines/mod.rs type -->

<!-- file: 04b session_viewer/src/engine/pipelines/layouts.rs type -->

<!-- file: 04b session_viewer/src/engine/gpu/upload.rs type -->

- The ink pass binds group 2 through `objects.ink_group`, the variant that carries the physical depth.

<!-- file: 04b session_viewer/src/engine/gpu/mod.rs type -->

- A second object row with a three-segment polyline.

<!-- file: 04b session_viewer/src/fixture.rs copy -->

<!-- file: 04b session_viewer/src/lib.rs type -->

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

- Append `?thickness=4` to the URL: every stroke widens on screen while the geometry stays put, because the pen is applied in `ribbon.wgsl`, not in the vertex data.
- Zoom far out: the strokes keep their pixel width. A world-space width would vanish; a screen-space pen does not.
- Set `?thickness=0.2`: the stroke thins to a hairline and keeps a floor of alpha rather than disappearing, because `band_area` integrates the pixel box exactly instead of ramping a distance. (`?aa=` feathers markers and dots, not ribbons — the ribbon shader never reads `line.feather`.)

## Questions and answers

**The segment row ends in flat `f32`s instead of two `vec3`s. What would the `vec3`s cost?**

*How to work it out.* Apply the alignment rule from lesson 03: a `vec3` aligns to 16 even though it holds 12 bytes. Lay the row out both ways and count — then ask what the `vec3` form actually buys, given that the shader reads the components individually anyway.

*The answer.* Eight bytes a row, taking it from 40 to 48, for no benefit. Being able to predict this rather than discover it is the point: the same rule set `Instance` at 96 and will set the marker row at 48.

**Strokes draw with `DepthMode::Always` and blending. Why not simply depth-test them?**

*How to work it out.* Ask where a stroke usually sits: on the edge of the face it belongs to, at the same depth as that face. Now ask what a depth test does with two fragments at the same depth — it is a coin flip decided by float rounding, per pixel, and it changes as the camera moves.

*The answer.* Hardware depth testing at equal depth produces stitching, so the shader decides visibility itself: `ink_visible` compares the scene depth at the pixel against the depth of the closest point on the stroke's axis, using the gradient the face pass wrote. Putting that rule in one shared file is what keeps every ink lane answering the question the same way.

**The half-width at each end travels as a flat scalar, resolved per pixel. What breaks if you interpolate a width per vertex instead?**

*How to work it out.* Draw a stroke going away from the camera: on screen it is a trapezoid, wide at the near end, narrow at the far end. Interpolation across a trapezoid is perspective-correct for *positions*, but a width is not a position — it is a screen-space quantity derived from one.

*The answer.* The width comes out wrong in the middle and wobbles as the camera moves. Sending both ends flat and computing the width per pixel from them is exact, which is why the outputs are marked `@interpolate(flat)`.

**Why must the segment be clipped against the near plane before any divide?**

*How to work it out.* Write out the divide: `x/w`, `y/w`. Ask what happens when `w` is negative — both signs flip, so the point appears mirrored through the screen centre instead of being absent.

*The answer.* A line crossing behind the eye would swing across the canvas rather than disappear. Clip first, divide second. It costs nothing and it is the difference between a correct viewer and one with a bizarre bug that appears only when you walk the camera into geometry.

**What you should be able to do now**

Say in two sentences why a stroke keeps its pixel width when you zoom out, and where that decision is applied. Correct: the vertex stage expands the segment into a quad whose half-width is computed in *screen* space from `line.thickness`, so world distance never enters it — the pen is applied in `ribbon.wgsl`, not in the vertex data. Then predict the CPU alternative: you would have to rebuild and re-upload the geometry on every camera move.

## Next

[04c · Markers](04c-markers.md): vertex markers on a quad template and free dots as SDF triangles.
