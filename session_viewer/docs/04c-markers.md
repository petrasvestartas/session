# 04c · Markers

An orange point appears below the triangle and fades smoothly at small sizes.

![A sphere is four template corners pushed out by the pixel radius plus half the feather; a free dot is one equilateral triangle whose incircle is the disc.](illustrations/markers.svg)

## Step 1 · src/engine/gpu/glyphs.rs

New file: the lane that draws vertex markers and flat dots, one 48-byte row each.

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:step-1a"
```

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:step-1b"
```

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:step-1c"
```

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:step-1d"
```

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:step-1e"
```

`lessons/04c/src/engine/gpu/glyphs.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:step-1f"
```

Copy this part from the lesson folder to the path shown.

`lessons/04c/src/engine/gpu/glyphs.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/04c/src/engine/gpu/glyphs.rs:step-1g"
```

## Step 2 · src/shaders/sphere.wgsl

New file: the sphere shader, a round marker with real depth.

`lessons/04c/src/shaders/sphere.wgsl` · type this, new file, start with these lines

```wgsl
--8<-- "lessons/04c/src/shaders/sphere.wgsl:step-2b"
```

`lessons/04c/src/shaders/sphere.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/sphere.wgsl:step-2c"
```

`lessons/04c/src/shaders/sphere.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/sphere.wgsl:step-2d"
```

## Step 3 · src/shaders/glyph.wgsl

New file: the dot shader: one triangle per dot, cut to a soft-edged disc.

`lessons/04c/src/shaders/glyph.wgsl` · type this, new file, start with these lines

```wgsl
--8<-- "lessons/04c/src/shaders/glyph.wgsl:step-3b"
```

`lessons/04c/src/shaders/glyph.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/glyph.wgsl:step-3c"
```

`lessons/04c/src/shaders/glyph.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04c/src/shaders/glyph.wgsl:step-3d"
```

Run `cargo check` in `lessons/04c/`.

## Step 4 · src/engine/pipelines/mod.rs

Add the marker pipelines.

`lessons/04c/src/engine/pipelines/mod.rs` · edit · type this

Added above

```rust
/// The mesh vertex slot: the kernel's interleaved `RenderVertex` (pos, normal, colour).
```

```rust
--8<-- "lessons/04c/src/engine/pipelines/mod.rs:step-4a"
```

Added above

```rust
/// Compile one WGSL source into a module; the caller keeps it and shares it across pipelines.
```

```rust
--8<-- "lessons/04c/src/engine/pipelines/mod.rs:step-4b"
```

## Step 5 · src/engine/pipelines/layouts.rs

Add the marker bind group layout.

`lessons/04c/src/engine/pipelines/layouts.rs` · edit · type this

Added above

```rust
/// Segment rows retain source-edge IDs and one shared selection uniform.
```

```rust
--8<-- "lessons/04c/src/engine/pipelines/layouts.rs:step-5a"
```

Added below

```rust
    pub ink_instance: wgpu::BindGroupLayout,
```

```rust
--8<-- "lessons/04c/src/engine/pipelines/layouts.rs:step-5b"
```

Added below

```rust
            ink_instance: ink_instance_layout(device),
```

```rust
--8<-- "lessons/04c/src/engine/pipelines/layouts.rs:step-5c"
```

## Step 6 · src/engine/gpu/upload.rs

Add markers to the upload.

`lessons/04c/src/engine/gpu/upload.rs` · edit · type this

Added below

```rust
use super::arena::ArenaRows;
```

```rust
--8<-- "lessons/04c/src/engine/gpu/upload.rs:step-6a"
```

Added below

```rust
    pub seg: SegRows,
```

```rust
--8<-- "lessons/04c/src/engine/gpu/upload.rs:step-6b"
```

Added below

```rust
            seg: SegRows::default(),
```

```rust
--8<-- "lessons/04c/src/engine/gpu/upload.rs:step-6c"
```

Added below

```rust
        self.seg.drop_rows();
```

```rust
--8<-- "lessons/04c/src/engine/gpu/upload.rs:step-6d"
```

## Step 7 · src/engine/gpu/mod.rs

Add the marker lane to the GPU owner and the frame.

`lessons/04c/src/engine/gpu/mod.rs` · edit · type this

Added below

```rust
pub mod frame;
```

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:step-7a"
```

Added below

```rust
pub use frame::FrameInput;
```

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:step-7b"
```

Added below

```rust
    pub segments: segments::SegmentLane,
```

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:step-7c"
```

Added below

```rust
        let segments = segments::SegmentLane::new(&ctx, &layouts, target);
```

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:step-7d"
```

Added below

```rust
            segments,
```

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:step-7e"
```

Added below

```rust
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
```

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:step-7f"
```

Added below

```rust
            self.segments.draw_ribbons(&mut pass, &ink);
```

```rust
--8<-- "lessons/04c/src/engine/gpu/mod.rs:step-7g"
```

## Step 8 · src/fixture.rs

Copy the test scene: it now has markers.
Copy this file from the lesson folder to the path shown.

`lessons/04c/src/fixture.rs` · edit · copy the file

Replaces the line `use crate::engine::gpu::{CylinderSegment, Instance, Objec…` in `lessons/04b/src/fixture.rs`

```rust
--8<-- "lessons/04c/src/fixture.rs:step-8a"
```

Replaces the line `for _ in 0..2 {` in `lessons/04b/src/fixture.rs`

```rust
--8<-- "lessons/04c/src/fixture.rs:step-8b"
```

Added below

```rust
        });
    }
```

```rust
--8<-- "lessons/04c/src/fixture.rs:step-8c"
```

## Step 9 · src/lib.rs

Report the dot count from the entry point.

`lessons/04c/src/lib.rs` · edit · type this

Replaces the line `"meshVertices":self.gpu.arena.vert_count(),"segments":sel…` in `lessons/04b/src/lib.rs`

```rust
--8<-- "lessons/04c/src/lib.rs:step-9"
```

## Step 10 · index.html

Copy the page: the status now reports dots.
Copy this file from the lesson folder to the path shown.

`lessons/04c/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 04b</title>` in `lessons/04b/index.html`

```html
--8<-- "lessons/04c/index.html:step-10a"
```

Replaces the line `<output id="status">Starting checkpoint 04b</output>` in `lessons/04b/index.html`

```html
--8<-- "lessons/04c/index.html:step-10b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/04b/index.html`

```html
--8<-- "lessons/04c/index.html:step-10c"
```

## Check

Run `trunk serve` in `lessons/04c/` and open <http://127.0.0.1:8770/>.

Expected: An orange point appears below the triangle and fades smoothly at small sizes; status: **Checkpoint 04c · 3 objects**.

![Checkpoint 04c: vertex markers and free dots drawn from the glyph lane.](screenshots/04c.png)

If it fails:

- Dots vanish while zooming out: the half-pixel floor or alpha fade is missing.
- A sphere looks flat: the fragment depth still describes its billboard.

## What changed

```text
lessons/04c/src/engine/
├── gpu/
│   ├── arena.rs
│   ├── buffers.rs
│   ├── frame.rs
│   ├── glyphs.rs  +
│   ├── instance.rs
│   ├── mod.rs  ~
│   ├── objects.rs
│   ├── segments.rs
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

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/04c/`.

## Next

[04d · Point clouds](04d-clouds.md)

## Expected viewer result

Checkpoint 04c: vertex markers and free dots drawn from the glyph lane.

[![Full viewer result for 04c markers](screenshots/04c.png)](screenshots/04c.png)
