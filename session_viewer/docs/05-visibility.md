# 05 · Depth and visible ink

A grey box shows its visible red edges and black corners while its faces hide the far edges.

![Reversed depth, and why a thick stroke must transfer the surface depth to its axis before comparing.](illustrations/ink-visibility.svg)

## Step 1 · src/shaders/physical.wgsl

New file: the physical outputs, depth beside colour.

`lessons/05/src/shaders/physical.wgsl` · 22 lines · type this, new file

```wgsl
--8<-- "lessons/05/src/shaders/physical.wgsl"
```

## Step 2 · src/shaders/background.wgsl

New file: the background shader, one fullscreen triangle.

`lessons/05/src/shaders/background.wgsl` · 21 lines · type this, new file

```wgsl
--8<-- "lessons/05/src/shaders/background.wgsl"
```

## Step 3 · src/shaders/grid.wgsl

New file: the grid shader, lines from vertex indices.

`lessons/05/src/shaders/grid.wgsl` · 56 lines · type this, new file

```wgsl
--8<-- "lessons/05/src/shaders/grid.wgsl"
```

## Step 4 · src/engine/gpu/backdrop.rs

New file: the backdrop lane, background and grid.

`lessons/05/src/engine/gpu/backdrop.rs` · 111 lines · type this, new file

```rust
--8<-- "lessons/05/src/engine/gpu/backdrop.rs"
```

Run `cargo check` in `lessons/05/`.

## Step 5 · src/shaders/ink_visibility.wgsl

Replace the ink test: fit the surface under a sample and compare depth.

`lessons/05/src/shaders/ink_visibility.wgsl` · type this, replace the whole file, start with these lines

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:step-5a"
```

`lessons/05/src/shaders/ink_visibility.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:step-5b"
```

`lessons/05/src/shaders/ink_visibility.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:step-5c"
```

`lessons/05/src/shaders/ink_visibility.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:step-5d"
```

`lessons/05/src/shaders/ink_visibility.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:step-5e"
```

## Step 6 · src/shaders/triangle.wgsl

The mesh shader writes physical depth and reads it back for ink.

`lessons/05/src/shaders/triangle.wgsl` · edit · type this

Added below

```wgsl
    @location(5) @interpolate(flat) mirrored: u32,
```

```wgsl
--8<-- "lessons/05/src/shaders/triangle.wgsl:step-6a"
```

Added below

```wgsl
    dead.mirrored = 0u;
```

```wgsl
--8<-- "lessons/05/src/shaders/triangle.wgsl:step-6b"
```

Added below

```wgsl
    o.inst_id = in.inst_id;
```

```wgsl
--8<-- "lessons/05/src/shaders/triangle.wgsl:step-6c"
```

Replaces the `fn fs_id` lines in `lessons/04d/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/05/src/shaders/triangle.wgsl:step-6d"
```

## Step 7 · src/shaders/splat.wgsl

The splat shader writes physical depth too.

`lessons/05/src/shaders/splat.wgsl` · edit · type this

Replaces the `fn fs_point_id` lines in `lessons/04d/src/shaders/splat.wgsl`

```wgsl
--8<-- "lessons/05/src/shaders/splat.wgsl:step-7"
```

## Step 8 · src/shaders/splat_resolve.wgsl

The resolve writes point depth into the scene.

`lessons/05/src/shaders/splat_resolve.wgsl` · edit · type this

Added below

```wgsl
    @location(0) color: vec4<f32>,
```

```wgsl
--8<-- "lessons/05/src/shaders/splat_resolve.wgsl:step-8a"
```

Added below

```wgsl
    o.depth = d;
```

```wgsl
--8<-- "lessons/05/src/shaders/splat_resolve.wgsl:step-8b"
```

## Step 9 · src/shaders/text_outline.wgsl

Outline text takes the same depth test.

`lessons/05/src/shaders/text_outline.wgsl` · edit · type this

Added below

```wgsl
// Keep the exact source object row available to the shared identity pass.
```

```wgsl
--8<-- "lessons/05/src/shaders/text_outline.wgsl:step-9"
```

## Step 10 · src/engine/gpu/targets.rs

Targets gain the depth and gradient textures at 1x and 4x.

`lessons/05/src/engine/gpu/targets.rs` · edit · type this

Added above

```rust
/// The attachments of the frame's render pass and the sample count they were made at.
```

```rust
--8<-- "lessons/05/src/engine/gpu/targets.rs:step-10a"
```

Added after the `depth_msaa` field of `struct Targets` in `lessons/04d/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/targets.rs:step-10b"
```

Replaces the line `Self {` in `lessons/04d/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/targets.rs:step-10c"
```

Replaces the `fn begin_faces` lines in `lessons/04d/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/targets.rs:step-10d"
```

Replaces the lines from `color_attachments: &[Some(wgpu::RenderPassColorAttachment {` to `})],` in `lessons/04d/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/targets.rs:step-10e"
```

Added below

```rust
    texture(ctx, label, spec).create_view(&wgpu::TextureViewDescriptor::default())
}
```

```rust
--8<-- "lessons/05/src/engine/gpu/targets.rs:step-10f"
```

## Step 11 · src/engine/pipelines/mod.rs

Pipelines gain the physical target and the sample count.

`lessons/05/src/engine/pipelines/mod.rs` · edit · type this

Added below

```rust
    pub scene_samples: Option<u32>,
```

```rust
--8<-- "lessons/05/src/engine/pipelines/mod.rs:step-11a"
```

Added below

```rust
            scene_samples: None,
```

```rust
--8<-- "lessons/05/src/engine/pipelines/mod.rs:step-11b"
```

Added above

```rust
    /// The same desc with another depth mode.
```

```rust
--8<-- "lessons/05/src/engine/pipelines/mod.rs:step-11c"
```

Replaces the line `let source = format!("{}\n{}", source, include_str!("../.…` in `lessons/04d/src/engine/pipelines/mod.rs`

```rust
--8<-- "lessons/05/src/engine/pipelines/mod.rs:step-11d"
```

Replaces the line `let targets = [Some(wgpu::ColorTargetState {` in `lessons/04d/src/engine/pipelines/mod.rs`

```rust
--8<-- "lessons/05/src/engine/pipelines/mod.rs:step-11e"
```

## Step 12 · src/engine/pipelines/layouts.rs

Layouts gain the depth and gradient bindings.

`lessons/05/src/engine/pipelines/layouts.rs` · edit · type this

Added before `fn ink_instance_layout` in `lessons/04d/src/engine/pipelines/layouts.rs`

```rust
--8<-- "lessons/05/src/engine/pipelines/layouts.rs:step-12a"
```

Added below

```rust
            scene_depth(3, true),
```

```rust
--8<-- "lessons/05/src/engine/pipelines/layouts.rs:step-12b"
```

## Step 13 · src/engine/gpu/objects.rs

The object table binds the new textures.

`lessons/05/src/engine/gpu/objects.rs` · edit · type this

Added after the `binding: 3` entry in `lessons/04d/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/objects.rs:step-13a"
```

Added below

```rust
        depths: [&wgpu::TextureView; 2],
```

```rust
--8<-- "lessons/05/src/engine/gpu/objects.rs:step-13b"
```

Added above

```rust
            ],
```

```rust
--8<-- "lessons/05/src/engine/gpu/objects.rs:step-13c"
```

## Step 14 · src/engine/gpu/arena.rs

The arena draws a physical pass before the colour pass.

`lessons/05/src/engine/gpu/arena.rs` · edit · type this

Added below

```rust
    id_faces: wgpu::RenderPipeline,
```

```rust
--8<-- "lessons/05/src/engine/gpu/arena.rs:step-14a"
```

Added above

```rust
    /// Sheet fills: same vertex table, depth write off, so a page's exactly coplanar regions
```

```rust
--8<-- "lessons/05/src/engine/gpu/arena.rs:step-14b"
```

Replaces the line `.draw_ids(pass, b, &self.outline_buffers(&self.print))` in `lessons/04d/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/arena.rs:step-14c"
```

Replaces the lines from `faces: build(dev, target, &base.with("triangle", "fs_main…` to `id_faces: build(dev, Target::ID, &base.with("triangle.id"…` in `lessons/04d/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/arena.rs:step-14d"
```

## Step 15 · src/engine/gpu/splat.rs

The splat pass runs against the physical depth.

`lessons/05/src/engine/gpu/splat.rs` · edit · type this

Added below

```rust
        .vertex("vs_point");
```

```rust
--8<-- "lessons/05/src/engine/gpu/splat.rs:step-15a"
```

Added below

```rust
        .with("splat.resolve", "fs_main")
```

```rust
--8<-- "lessons/05/src/engine/gpu/splat.rs:step-15b"
```

## Step 16 · src/engine/gpu/text_outline.rs

Outline text draws in both passes.

`lessons/05/src/engine/gpu/text_outline.rs` · edit · type this

Added below

```rust
    id: wgpu::RenderPipeline,
```

```rust
--8<-- "lessons/05/src/engine/gpu/text_outline.rs:step-16a"
```

Replaces the lines from `let (color, id) = pipelines(ctx, layouts, &shader, target);` to `(self.color, self.id) = pipelines(ctx, layouts, &self.sha…` in `lessons/04d/src/engine/gpu/text_outline.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/text_outline.rs:step-16b"
```

Added above

```rust
    /// Preserve the existing exact object IDs and sheet depth comparison in the picking pass.
```

```rust
--8<-- "lessons/05/src/engine/gpu/text_outline.rs:step-16c"
```

Replaces the line `) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {` in `lessons/04d/src/engine/gpu/text_outline.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/text_outline.rs:step-16d"
```

Replaces the line `(color, id)` in `lessons/04d/src/engine/gpu/text_outline.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/text_outline.rs:step-16e"
```

## Step 17 · src/engine/gpu/mod.rs

The GPU owner runs the physical pass, then every lane over it.

`lessons/05/src/engine/gpu/mod.rs` · edit · type this

Added below

```rust
pub mod arena;
```

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17a"
```

Added below

```rust
pub use segments::CylinderSegment;
```

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17b"
```

Added below

```rust
    pub objects: objects::InstanceTable,
```

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17c"
```

Added below

```rust
        let objects = objects::InstanceTable::new(&ctx, &layouts, &InkScene { targets: &targets });
```

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17d"
```

Added below

```rust
            objects,
```

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17e"
```

Added below

```rust
        self.bounds.union_with(&up.bounds);
```

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17f"
```

Replaces the lines from `self.targets = targets::Targets::new(` to `);` in `lessons/04d/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17g"
```

Added below

```rust
            let mut pass = self.targets.begin_faces(&mut encoder, &target, input.clear);
```

```rust
--8<-- "lessons/05/src/engine/gpu/mod.rs:step-17h"
```

## Step 18 · src/fixture.rs

Copy the test scene: a box with edges behind faces.
Copy this file from the lesson folder to the path shown.

`lessons/05/src/fixture.rs` · edit · copy the file

Replaces the lines from `use crate::engine::gpu::{` to `facing_ext: [0xffffffff; 2],` in `lessons/04d/src/fixture.rs`

```rust
--8<-- "lessons/05/src/fixture.rs:step-18"
```

## Step 19 · src/lib.rs

The entry point reports the sample count.

`lessons/05/src/lib.rs` · edit · type this

Replaces the line `camera.set_view(camera::View::Top);` in `lessons/04d/src/lib.rs`

```rust
--8<-- "lessons/05/src/lib.rs:step-19a"
```

Replaces the line `Ok(serde_json::json!({"stage":4,"objects":self.gpu.object…` in `lessons/04d/src/lib.rs`

```rust
--8<-- "lessons/05/src/lib.rs:step-19b"
```

Added below

```rust
    JsValue::from_str(&error.to_string())
}
```

```rust
--8<-- "lessons/05/src/lib.rs:step-19c"
```

## Step 20 · index.html

Copy the page: the status says checkpoint 05.
Copy this file from the lesson folder to the path shown.

`lessons/05/index.html` · edit · copy the file

Replaces the line `<title>Session checkpoint 04</title>` in `lessons/04d/index.html`

```html
--8<-- "lessons/05/index.html:step-20a"
```

Replaces the line `<output id="status">Starting checkpoint 04</output>` in `lessons/04d/index.html`

```html
--8<-- "lessons/05/index.html:step-20b"
```

Replaces the line `document.getElementById('status').textContent = 'Checkpoi…` in `lessons/04d/index.html`

```html
--8<-- "lessons/05/index.html:step-20c"
```

## Check

Run `trunk serve` in `lessons/05/` and open <http://127.0.0.1:8770/>.

Expected: A grey box shows its visible red edges and black corners while its faces hide the far edges; status: **Checkpoint 05 · 1 objects**.

![Checkpoint 05: hidden lines stay hidden while visible strokes and corners stay readable, over the white backdrop.](screenshots/05.png)

If it fails:

- Every edge disappears: the depth clear and comparison disagree with reversed depth.
- Hidden edges show through: the face pipeline omits its physical depth gradient.

## What changed

```text
lessons/05/src/
├── app/
│   ├── mod.rs
│   └── route.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs  +
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── segments.rs
│   │   ├── splat.rs  ~
│   │   ├── targets.rs  ~
│   │   ├── text_outline.rs  ~
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs  ~
│   │   └── mod.rs  ~
│   └── mod.rs
├── shaders/
│   ├── background.wgsl  +
│   ├── glyph.wgsl
│   ├── grid.wgsl  +
│   ├── ink_visibility.wgsl  ~
│   ├── normals.wgsl
│   ├── physical.wgsl  +
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl  ~
│   ├── splat_resolve.wgsl  ~
│   ├── text_outline.wgsl  ~
│   └── triangle.wgsl  ~
├── camera.rs
├── fixture.rs  ~
└── lib.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source files → retained scene state → GPU buffers → visible result. Every file at this point: `lessons/05/`.

## Next

[06 · CAD face rules](06-cad-contract.md)

## Expected viewer result

Checkpoint 05: hidden lines stay hidden while visible strokes and corners stay readable, over the white backdrop.

[![Full viewer result for 05 visibility](screenshots/05.png)](screenshots/05.png)
