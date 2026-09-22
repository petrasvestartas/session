# 04d · Point clouds

A grid of blue points appears beside the mesh, polyline and marker.

![One node, one question: a spacing that projects wider than lod_px descends into the eight children, and one that fits draws the node whole.](illustrations/lod.svg)

## Step 1 · src/engine/gpu/cloud.rs

New file: point cloud positions, colours and source ids on the GPU.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:step-1a"
```

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:step-1b"
```

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:step-1c"
```

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:step-1d"
```

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:step-1e"
```

## Step 2 · src/engine/gpu/lod.rs

New file: the octree walk that picks visible detail.

`lessons/04d/src/engine/gpu/lod.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04d/src/engine/gpu/lod.rs:step-2a"
```

`lessons/04d/src/engine/gpu/lod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/lod.rs:step-2b"
```

`lessons/04d/src/engine/gpu/lod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/lod.rs:step-2c"
```

## Step 3 · src/engine/gpu/splat.rs

New file: the splat pass that projects points, then resolves them into the scene.

`lessons/04d/src/engine/gpu/splat.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3a"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3b"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3c"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3d"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3e"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3f"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3g"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3h"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3i"
```

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:step-3j"
```

## Step 4 · src/shaders/splat.wgsl

New file: the shader that writes the nearest point per pixel.

`lessons/04d/src/shaders/splat.wgsl` · type this, new file, start with these lines

```wgsl
--8<-- "lessons/04d/src/shaders/splat.wgsl:step-4a"
```

`lessons/04d/src/shaders/splat.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04d/src/shaders/splat.wgsl:step-4b"
```

`lessons/04d/src/shaders/splat.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04d/src/shaders/splat.wgsl:step-4c"
```

## Step 5 · src/shaders/splat_resolve.wgsl

New file: the shader that copies point colour and depth into the scene.

`lessons/04d/src/shaders/splat_resolve.wgsl` · 83 lines · type this, new file

```wgsl
--8<-- "lessons/04d/src/shaders/splat_resolve.wgsl"
```

Run `cargo check` in `lessons/04d/`.

## Step 6 · src/engine/pipelines/layouts.rs

Add the cloud bind group layouts.

`lessons/04d/src/engine/pipelines/layouts.rs` · edit · type this

Added above

```rust
/// The bind-group layouts every lane shares.
```

```rust
--8<-- "lessons/04d/src/engine/pipelines/layouts.rs:step-6a"
```

Added below

```rust
    pub segment_rows: wgpu::BindGroupLayout,
```

```rust
--8<-- "lessons/04d/src/engine/pipelines/layouts.rs:step-6b"
```

Added below

```rust
            segment_rows: segment_rows_layout(device),
```

```rust
--8<-- "lessons/04d/src/engine/pipelines/layouts.rs:step-6c"
```

## Step 7 · src/engine/gpu/upload.rs

Add clouds to the upload.

`lessons/04d/src/engine/gpu/upload.rs` · edit · type this

Added below

```rust
use super::arena::ArenaRows;
```

```rust
--8<-- "lessons/04d/src/engine/gpu/upload.rs:step-7a"
```

Added below

```rust
    pub glyph: GlyphRows,
```

```rust
--8<-- "lessons/04d/src/engine/gpu/upload.rs:step-7b"
```

Added below

```rust
            glyph: GlyphRows::default(),
```

```rust
--8<-- "lessons/04d/src/engine/gpu/upload.rs:step-7c"
```

Added below

```rust
        self.glyph.drop_rows();
```

```rust
--8<-- "lessons/04d/src/engine/gpu/upload.rs:step-7d"
```

## Step 8 · src/engine/gpu/mod.rs

Add the cloud lane to the GPU owner, resize and frame.

`lessons/04d/src/engine/gpu/mod.rs` · edit · type this

Replaces the lines from `pub mod frame;` to `use buffers::GpuCtx;` in `lessons/04c/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8a"
```

Added below

```rust
    pub glyphs: glyphs::GlyphLane,
```

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8b"
```

Added below

```rust
        let glyphs = glyphs::GlyphLane::new(&ctx, &layouts, target);
```

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8c"
```

Replaces the `fn set_scene` lines in `lessons/04c/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8d"
```

Added at the end of `fn resize` in `lessons/04c/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8e"
```

Replaces the line `self.objects.rebase_anchor(&self.ctx, origin, distance, now)` in `lessons/04c/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8f"
```

Replaces the `///` line above `fn render` in `lessons/04c/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8g"
```

Added below

```rust
        let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
```

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8h"
```

Added below

```rust
            self.arena.draw_faces(&mut pass, &basic);
```

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:step-8i"
```

## Step 9 · src/fixture.rs

Copy the test scene: it now has a point grid.
Copy this file from the lesson folder to the path shown.

`lessons/04d/src/fixture.rs` · edit · copy the file

Replaces the line `use crate::engine::gpu::{CylinderSegment, GlyphPoint, Ins…` in `lessons/04c/src/fixture.rs`

```rust
--8<-- "lessons/04d/src/fixture.rs:step-9a"
```

Replaces the line `for _ in 0..3 {` in `lessons/04c/src/fixture.rs`

```rust
--8<-- "lessons/04d/src/fixture.rs:step-9b"
```

Added after the last marker, before `upload` is returned, in `lessons/04c/src/fixture.rs`

```rust
--8<-- "lessons/04d/src/fixture.rs:step-9c"
```

## Step 10 · src/lib.rs

Report the point count from the entry point.

`lessons/04d/src/lib.rs` · edit · type this

Replaces the `///` line above `pub async fn create` in `lessons/04c/src/lib.rs`

```rust
--8<-- "lessons/04d/src/lib.rs:step-10a"
```

Replaces the line `"meshVertices":self.gpu.arena.vert_count(),"segments":sel…` in `lessons/04c/src/lib.rs`

```rust
--8<-- "lessons/04d/src/lib.rs:step-10b"
```

## Step 11 · index.html

Copy the page: the status now reports cloud points.
Copy this file from the lesson folder to the path shown.

`lessons/04d/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 04c</title>` in `lessons/04c/index.html`

```html
--8<-- "lessons/04d/index.html:step-11a"
```

Replaces the line `<output id="status">Starting checkpoint 04c</output>` in `lessons/04c/index.html`

```html
--8<-- "lessons/04d/index.html:step-11b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/04c/index.html`

```html
--8<-- "lessons/04d/index.html:step-11c"
```

## Check

Run `trunk serve` in `lessons/04d/` and open <http://127.0.0.1:8770/>.

Expected: A grid of blue points appears beside the mesh, polyline and marker; status: **Checkpoint 04 · 4 objects**.

![Checkpoint 04d: a point cloud through the splat prelude and resolve, beside the mesh, stroke and marker lanes.](screenshots/04d.png)

If it fails:

- A cloud paints over a solid: the resolve does not write the winning point depth.
- Points change size on a high-DPI screen: CSS pixels and framebuffer pixels are mixed.

## What changed

```text
lessons/04d/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── buffers.rs
│   ├── cloud.rs  +
│   ├── frame.rs
│   ├── glyphs.rs
│   ├── instance.rs
│   ├── lod.rs  +
│   ├── mod.rs  ~
│   ├── objects.rs
│   ├── segments.rs
│   ├── splat.rs  +
│   ├── targets.rs
│   ├── text_outline.rs
│   ├── upload.rs  ~
│   └── view.rs
├── pipelines/
│   ├── layouts.rs  ~
│   └── mod.rs
└── mod.rs
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/04d/`.

## Next

[05 · Depth and visible ink](05-visibility.md)

## Expected viewer result

Checkpoint 04d: a point cloud through the splat prelude and resolve, beside the mesh, stroke and marker lanes.

[![Full viewer result for 04d clouds](screenshots/04d.png)](screenshots/04d.png)
