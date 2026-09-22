# 04b · Strokes

A yellow polyline appears beside the blue triangle and keeps its width while zooming.

![Six vertices place a camera-facing quad around the projected axis, and band_area integrates one pixel box against the capsule so coverage is an area rather than a distance ramp.](illustrations/ribbon.svg)

## Step 1 · src/engine/gpu/segments.rs

New file: stroke segments and the object rows they belong to.

`lessons/04b/src/engine/gpu/segments.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:step-1a"
```

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:step-1b"
```

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:step-1c"
```

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:step-1d"
```

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:step-1e"
```

`lessons/04b/src/engine/gpu/segments.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:step-1f"
```

Copy this part from the lesson folder to the path shown.

`lessons/04b/src/engine/gpu/segments.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/04b/src/engine/gpu/segments.rs:step-1g"
```

## Step 2 · src/shaders/ink_visibility.wgsl

New file: the test that hides a stroke sample behind a face.

`lessons/04b/src/shaders/ink_visibility.wgsl` · 51 lines · type this, new file

```wgsl
--8<-- "lessons/04b/src/shaders/ink_visibility.wgsl"
```

## Step 3 · src/shaders/ribbon.wgsl

New file: the stroke shader, a segment expanded into a screen ribbon.

`lessons/04b/src/shaders/ribbon.wgsl` · type this, new file, start with these lines

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:step-3b"
```

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:step-3c"
```

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:step-3d"
```

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:step-3e"
```

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:step-3f"
```

`lessons/04b/src/shaders/ribbon.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04b/src/shaders/ribbon.wgsl:step-3g"
```

Run `cargo check` in `lessons/04b/`.

## Step 4 · src/engine/pipelines/mod.rs

Add the stroke pipeline.

`lessons/04b/src/engine/pipelines/mod.rs` · edit · type this

Added above

```rust
/// The pipeline layout for `groups`, in slot order.
```

```rust
--8<-- "lessons/04b/src/engine/pipelines/mod.rs:step-4"
```

## Step 5 · src/engine/pipelines/layouts.rs

Add the stroke bind group layout.

`lessons/04b/src/engine/pipelines/layouts.rs` · edit · type this

Replaces the `struct Layouts` lines in `lessons/04a/src/engine/pipelines/layouts.rs`

```rust
--8<-- "lessons/04b/src/engine/pipelines/layouts.rs:step-5a"
```

Added below

```rust
            ink_instance: ink_instance_layout(device),
```

```rust
--8<-- "lessons/04b/src/engine/pipelines/layouts.rs:step-5b"
```

## Step 6 · src/engine/gpu/upload.rs

Add strokes to the upload.

`lessons/04b/src/engine/gpu/upload.rs` · edit · type this

Added below

```rust
use super::objects::ObjectRows;
```

```rust
--8<-- "lessons/04b/src/engine/gpu/upload.rs:step-6a"
```

Added below

```rust
    pub arena: ArenaRows,
```

```rust
--8<-- "lessons/04b/src/engine/gpu/upload.rs:step-6b"
```

Added below

```rust
            arena: ArenaRows::default(),
```

```rust
--8<-- "lessons/04b/src/engine/gpu/upload.rs:step-6c"
```

Added below

```rust
        self.arena.drop_rows();
```

```rust
--8<-- "lessons/04b/src/engine/gpu/upload.rs:step-6d"
```

## Step 7 · src/engine/gpu/mod.rs

Add the stroke lane to the GPU owner and the frame.

`lessons/04b/src/engine/gpu/mod.rs` · edit · type this

Added below

```rust
pub mod objects;
```

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:step-7a"
```

Added below

```rust
pub use objects::{ObjectRow, Rebase};
```

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:step-7b"
```

Added below

```rust
    pub arena: arena::ArenaLane,
```

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:step-7c"
```

Added below

```rust
        let arena = arena::ArenaLane::new(&ctx, &layouts, target);
```

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:step-7d"
```

Added below

```rust
            arena,
```

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:step-7e"
```

Added below

```rust
        self.arena.append(&self.ctx, &up.arena);
```

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:step-7f"
```

Replaces the line `self.arena.draw_print(&mut pass, &basic);` in `lessons/04a/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/04b/src/engine/gpu/mod.rs:step-7g"
```

## Step 8 · src/fixture.rs

Copy the test scene: it now has strokes.
Copy this file from the lesson folder to the path shown.

`lessons/04b/src/fixture.rs` · edit · copy the file

Replaces the line `use crate::engine::gpu::{Instance, ObjectRow, Upload};` in `lessons/04a/src/fixture.rs`

```rust
--8<-- "lessons/04b/src/fixture.rs:step-8a"
```

Replaces the line `for _ in 0..1 {` in `lessons/04a/src/fixture.rs`

```rust
--8<-- "lessons/04b/src/fixture.rs:step-8b"
```

Added below

```rust
    upload.arena.idx.extend([0, 1, 2]);
```

```rust
--8<-- "lessons/04b/src/fixture.rs:step-8c"
```

## Step 9 · src/lib.rs

Report the stroke count from the entry point.

`lessons/04b/src/lib.rs` · edit · type this

Replaces the line `"meshVertices":self.gpu.arena.vert_count()}).to_string())` in `lessons/04a/src/lib.rs`

```rust
--8<-- "lessons/04b/src/lib.rs:step-9"
```

## Step 10 · index.html

Copy the page: the status now reports segments.
Copy this file from the lesson folder to the path shown.

`lessons/04b/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 04a</title>` in `lessons/04a/index.html`

```html
--8<-- "lessons/04b/index.html:step-10a"
```

Replaces the line `<output id="status">Starting checkpoint 04a</output>` in `lessons/04a/index.html`

```html
--8<-- "lessons/04b/index.html:step-10b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/04a/index.html`

```html
--8<-- "lessons/04b/index.html:step-10c"
```

## Check

Run `trunk serve` in `lessons/04b/` and open <http://127.0.0.1:8770/>.

Expected: A yellow polyline appears beside the blue triangle and keeps its width while zooming; status: **Checkpoint 04b · 2 objects**.

![Checkpoint 04b: strokes expanded on the GPU into screen-space ribbons beside the mesh.](screenshots/04b.png)

If it fails:

- Lines change width with distance: the offset is applied before the perspective divide.
- A line disappears near the camera: a segment endpoint crosses the near plane without clipping.

## What changed

```text
lessons/04b/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── buffers.rs
│   ├── frame.rs
│   ├── instance.rs
│   ├── mod.rs  ~
│   ├── objects.rs
│   ├── segments.rs  +
│   ├── targets.rs
│   ├── text_outline.rs
│   ├── upload.rs  ~
│   └── view.rs
├── pipelines/
│   ├── layouts.rs  ~
│   └── mod.rs  ~
└── mod.rs
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/04b/`.

## Next

[04c · Markers](04c-markers.md): vertex markers on a quad template and free dots as SDF triangles.

## Expected viewer result

Checkpoint 04b: strokes expanded on the GPU into screen-space ribbons beside the mesh.

[![Full viewer result for 04b strokes](screenshots/04b.png)](screenshots/04b.png)
