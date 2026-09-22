# 17 · Source faces, text objects and one silhouette

Source faces and scene text become selectable, and optional outlines surround visible solids.

![A pick renders a 19 x 19 attachment: a 13 x 13 readback window inside a three-texel halo, with origin and frame carrying the canvas into it.](illustrations/pick-window.svg)

## Step 1 · src/engine/gpu/faces.rs

Face buffers preserve source face addresses alongside triangles.

`lessons/17/src/engine/gpu/faces.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/17/src/engine/gpu/faces.rs:step-1a"
```

`lessons/17/src/engine/gpu/faces.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/faces.rs:step-1b"
```

`lessons/17/src/engine/gpu/faces.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/faces.rs:step-1c"
```

`lessons/17/src/engine/gpu/faces.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/faces.rs:step-1d"
```

`lessons/17/src/engine/gpu/faces.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/faces.rs:step-1e"
```

`lessons/17/src/engine/gpu/faces.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/faces.rs:step-1f"
```

## Step 2 · src/shaders/triangle.wgsl

The mesh shader places vertices and shades visible faces.

`lessons/17/src/shaders/triangle.wgsl` · edit · type this

Added after the `@location(6) @interpolate(flat) selected: u32,` line in `struct VsOut` of `lessons/16/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/triangle.wgsl:step-2a"
```

Replaces the 6 lines from `return dead;` in `fn dead_vertex` of `lessons/16/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/triangle.wgsl:step-2b"
```

Replaces the `return o;` line in `fn vs_main` of `lessons/16/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/triangle.wgsl:step-2c"
```

Replaces the `return PhysicalId(vec2<u32>(in.inst_id + 1u,…` line in `fn fs_id` of `lessons/16/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/triangle.wgsl:step-2d"
```

Added after the `}` line of `lessons/16/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/triangle.wgsl:step-2e"
```

Run `cargo check` in `lessons/17/`.

## Step 3 · src/engine/gpu/arena.rs

The arena holds mesh vertices and indices across objects.

`lessons/17/src/engine/gpu/arena.rs` · edit · type this

Added after the `pub idx_text: Vec<u32>,` line in `struct ArenaRows` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3a"
```

Added after the `drop_rows(&mut self.idx_text);` line in `fn drop_rows` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3b"
```

Added after the `selection_mask: wgpu::RenderPipeline,` line in `struct ArenaPipelines` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3c"
```

Added after the `outline_text: OutlineTextLane,` line in `struct ArenaLane` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3d"
```

Added after the `+ self.text.buf.size()` line in `fn allocated_bytes` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3e"
```

Added after the `let pipes = build_pipelines(ctx, l, &shader,…` line in `fn new` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3f"
```

Added after the `self.outline_text.retarget(ctx, l, target);` line in `fn retarget` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3g"
```

Added after the `self.text.append(ctx, &up.idx_text);` line in `fn append` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3h"
```

Added after the `.draw_physical_ids(pass, b, &self.outline_buf…` line in `fn draw_face_ids` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-3i"
```

## Step 4 · src/app/walk/mesh.rs

The mesh walk uploads vertices, indices and source face IDs.

`lessons/17/src/app/walk/mesh.rs` · edit · type this

Delete the `use session_rust::RenderVertex;` line of `lessons/16/src/app/walk/mesh.rs`.

Added after the `}` line in `fn walk_mesh` of `lessons/16/src/app/walk/mesh.rs`

```rust
--8<-- "lessons/17/src/app/walk/mesh.rs:step-4b"
```

Replaces `fn positions` in `lessons/16/src/app/walk/mesh.rs`

```rust
--8<-- "lessons/17/src/app/walk/mesh.rs:step-4c"
```

## Step 5 · src/app/walk/brep.rs

The geometry walk uploads shaded faces and their source boundary edges.

`lessons/17/src/app/walk/brep.rs` · edit · type this

Replaces `fn push_face` in `lessons/16/src/app/walk/brep.rs`

```rust
--8<-- "lessons/17/src/app/walk/brep.rs:step-5a"
```

Replaces the `push_face(arena, &rm, cx, &mut solid);` line in `fn walk_brep` of `lessons/16/src/app/walk/brep.rs`

```rust
--8<-- "lessons/17/src/app/walk/brep.rs:step-5b"
```

## Step 6 · src/app/selection.rs

Selection keeps original edge, face and control IDs under their parent object.

`lessons/17/src/app/selection.rs` · edit · type this

Added after the `},` line in `enum SelectionMode` of `lessons/16/src/app/selection.rs`

```rust
--8<-- "lessons/17/src/app/selection.rs:step-6a"
```

Replaces the `Self::Edge { parent, .. } | Self::Controls {…` line in `fn parent` of `lessons/16/src/app/selection.rs`

```rust
--8<-- "lessons/17/src/app/selection.rs:step-6b"
```

## Step 7 · src/engine/gpu/pick.rs

Picking reads an object and subobject ID asynchronously.

`lessons/17/src/engine/gpu/pick.rs` · edit · type this

Added after the `use super::buffers::GpuCtx;` line of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7a"
```

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7b"
```

Added after the `pub const PICK_RADIUS: u32 = 6;` line of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7c"
```

Added after the `Edge,` line in `enum PickMode` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7d"
```

Added after the `}` line in `impl Window` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7e"
```

Added after the `targets: Option<IdTargets>,` line in `struct Picker` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7f"
```

Added after the `targets: None,` line in `fn new` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7g"
```

Added after the `}` line in `impl Picker` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7h"
```

Replaces the 2 lines from `size: (u32, u32),` in `fn begin_pass` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7i"
```

Replaces the 3 lines from `) {` in `impl Picker` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7j"
```

Replaces the 2 lines from `x: win.x,` in `fn copy_window` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7k"
```

Replaces the `let key = (sub == 0, distance, object, sub);` line in `fn nearest_hit` of `lessons/16/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/pick.rs:step-7l"
```

## Step 8 · src/app/input.rs

Input routes gestures and keyboard actions to State.

`lessons/17/src/app/input.rs` · edit · type this

Added after the `use crate::camera::View;` line of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-8a"
```

Added after the `ctrl: bool,` line in `struct Input` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-8b"
```

Added after the `ctrl: false,` line in `fn new` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-8c"
```

Added after the `}` line in `fn key` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-8d"
```

Added after the `self.orbiting = *btn == ElementState::Pressed;` line in `fn mouse` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-8e"
```

Added after the `self.panning = *btn == ElementState::Pressed;` line in `fn mouse` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-8f"
```

## Step 9 · src/state.rs

State coordinates input, selection and frame requests.

`lessons/17/src/state.rs` · edit · type this

Replaces the 2 lines from `use crate::engine::text::{TextLabel, TextPlac…` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-9a"
```

Added after the `pub needs_frame: bool,` line in `struct State` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-9b"
```

Added after the `needs_frame: true,` line in `fn new` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-9c"
```

Replaces the `self.scene.texts = texts;` line in `fn set_texts` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-9d"
```

Added after the `self.selection = SelectionMode::Object;` line in `fn clear` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-9e"
```

Replaces the `self.request_selection(x, y, false);` line in `fn request_pick` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-9f"
```

Added after the `self.selection = SelectionMode::Object;` line in `fn select` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-9g"
```

## Step 10 · src/engine/text.rs

Text layout retains shaped glyph positions for rendering.

`lessons/17/src/engine/text.rs` · edit · type this

Replaces the 4 lines from `#[derive(Clone, Debug, PartialEq)]` of `lessons/16/src/engine/text.rs`

```rust
--8<-- "lessons/17/src/engine/text.rs:step-10a"
```

Added after the `pub clip: Option<[f32; 4]>,` line in `struct TextLabel` of `lessons/16/src/engine/text.rs`

```rust
--8<-- "lessons/17/src/engine/text.rs:step-10b"
```

Added after the `TextLabel {` line in `fn label` of `lessons/16/src/engine/text.rs`

```rust
--8<-- "lessons/17/src/engine/text.rs:step-10c"
```

## Step 11 · src/app/manifest.rs

The manifest describes scene files and their placements.

`lessons/17/src/app/manifest.rs` · edit · type this

Added after the `pub height: f64,` line in `struct TextItem` of `lessons/16/src/app/manifest.rs`

```rust
--8<-- "lessons/17/src/app/manifest.rs:step-11"
```

## Step 12 · src/app/scene_text.rs

Scene text assigns labels their own source rows.

`lessons/17/src/app/scene_text.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/17/src/app/scene_text.rs:step-12a"
```

`lessons/17/src/app/scene_text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:step-12b"
```

`lessons/17/src/app/scene_text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:step-12c"
```

`lessons/17/src/app/scene_text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/app/scene_text.rs:step-12d"
```

## Step 13 · src/app/scene.rs

The scene owns source documents and maps their identities to GPU rows.

`lessons/17/src/app/scene.rs` · edit · type this

Added at the top of `lessons/16/src/app/scene.rs`

```rust
--8<-- "lessons/17/src/app/scene.rs:step-13a"
```

Replaces the `pub texts: Vec<super::manifest::TextItem>,` line in `struct Scene` of `lessons/16/src/app/scene.rs`

```rust
--8<-- "lessons/17/src/app/scene.rs:step-13b"
```

Added after the `let docs = std::mem::take(&mut self.docs);` line in `fn rebuild` of `lessons/16/src/app/scene.rs`

```rust
--8<-- "lessons/17/src/app/scene.rs:step-13c"
```

Replaces the 10 lines from `self.add_file(FileDoc {` in `fn rebuild` of `lessons/16/src/app/scene.rs`

```rust
--8<-- "lessons/17/src/app/scene.rs:step-13d"
```

Added after the `pub fn object_name(&self, row: u32) -> &str {` line in `impl Scene` of `lessons/16/src/app/scene.rs`

```rust
--8<-- "lessons/17/src/app/scene.rs:step-13e"
```

## Step 14 · src/state/text.rs

State updates annotations when selection changes.

`lessons/17/src/state/text.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/17/src/state/text.rs:step-14a"
```

`lessons/17/src/state/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/text.rs:step-14b"
```

`lessons/17/src/state/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/text.rs:step-14c"
```

`lessons/17/src/state/text.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/text.rs:step-14d"
```

## Step 15 · src/state/cloud_query.rs

State coordinates asynchronous source-point queries.

`lessons/17/src/state/cloud_query.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/17/src/state/cloud_query.rs:step-15a"
```

`lessons/17/src/state/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/cloud_query.rs:step-15b"
```

`lessons/17/src/state/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/cloud_query.rs:step-15c"
```

`lessons/17/src/state/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/state/cloud_query.rs:step-15d"
```

## Step 16 · src/state.rs

State coordinates input, selection and frame requests.

`lessons/17/src/state.rs` · edit · type this

Added after the `self.gpu.set_hidden(row, true);` line in `fn hide_selected` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16a"
```

Added after the `self.scene.hidden.clear();` line in `fn show_all` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16b"
```

Replaces the `PickMode::Edge => {` line in `fn apply_pick` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16c"
```

Added after the `self.status(&format!("Edge {edge} selected"));` line in `fn apply_pick` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16d"
```

Added after the `if let Some(message) = failure {` line in `fn render` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16e"
```

Added after the `self.last_frame_ms = now_ms;` line in `fn render` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16f"
```

Added after the `}` line in `impl State` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16g"
```

Replaces the `let mode = if edge {` line in `fn request_selection` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16h"
```

Added after the `self.selection.enable_controls(Some(parent),…` line in `fn enable_controls` of `lessons/16/src/state.rs`

```rust
--8<-- "lessons/17/src/state.rs:step-16i"
```

## Step 17 · src/engine/gpu/objects.rs

The object table stores GPU rows separately from source identity.

`lessons/17/src/engine/gpu/objects.rs` · edit · type this

Added after the `}` line in `impl InstanceTable` of `lessons/16/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/objects.rs:step-17"
```

## Step 18 · src/engine/gpu/text_plate.rs

Text plates draw a backing around shaped labels.

`lessons/17/src/engine/gpu/text_plate.rs` · edit · type this

Replaces the 78 lines from `}` of `lessons/16/src/engine/gpu/text_plate.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plate.rs:step-18a"
```

Added after the `self.vertices.reset();` line in `impl Plates` of `lessons/16/src/engine/gpu/text_plate.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plate.rs:step-18b"
```

Replaces `fn pipeline` in `lessons/16/src/engine/gpu/text_plate.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plate.rs:step-18c"
```

Replaces the `depth_compare: Some(wgpu::CompareFunction::Al…` line in `fn pipeline` of `lessons/16/src/engine/gpu/text_plate.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plate.rs:step-18d"
```

## Step 19 · src/shaders/text_plate.wgsl

Text plates draw rounded backing shapes behind labels.

`lessons/17/src/shaders/text_plate.wgsl` · edit · type this

Replaces the 23 lines from `}` of `lessons/16/src/shaders/text_plate.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/text_plate.wgsl:step-19"
```

## Step 20 · src/engine/gpu/text_plane.rs

Plane text projects labels through their scene placement.

`lessons/17/src/engine/gpu/text_plane.rs` · edit · type this

Added after the `pipeline: wgpu::RenderPipeline,` line in `struct CachedPlane` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20a"
```

Replaces the 8 lines from `let pipeline = pipeline(ctx, target, &layout);` in `impl Planes` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20b"
```

Replaces the `self.pipeline = pipeline(ctx, target, &self.l…` line in `impl Planes` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20c"
```

Replaces the `pass.set_pipeline(&self.pipeline);` line in `impl Planes` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20d"
```

Replaces the 4 lines from `bounds[0] -= 2;` in `fn rasterize` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20e"
```

Replaces the `vertices: &mut Vec<[f32; 14]>,` line in `fn append_quad` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20f"
```

Replaces the `let value = f32::from(label.color[index]) / 2…` line in `fn append_quad` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20g"
```

Replaces the 26 lines from `clip[0], clip[1], clip[2], clip[3], u, v, col…` in `fn append_quad` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20h"
```

Added after the `let mut label = TextLabel {` line in `fn fixed_plane_obeys_solid_depth_orientation_cache_and_release` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20i"
```

Added after the `assert_eq!(gpu.text.stats.world_plane_rasteri…` line in `fn fixed_plane_obeys_solid_depth_orientation_cache_and_release` of `lessons/16/src/engine/gpu/text_plane.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text_plane.rs:step-20j"
```

## Step 21 · src/shaders/text_plane.wgsl

Plane labels project shaped glyphs onto their scene plane.

`lessons/17/src/shaders/text_plane.wgsl` · edit · type this

Replaces the 12 lines from `}` of `lessons/16/src/shaders/text_plane.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/text_plane.wgsl:step-21a"
```

Replaces the 3 lines from `return vec4<f32>(in.color.rgb * coverage, in.…` in `fn fs_main` of `lessons/16/src/shaders/text_plane.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/text_plane.wgsl:step-21b"
```

## Step 22 · src/engine/gpu/text.rs

The GPU text owner coordinates glyphs and label backgrounds.

`lessons/17/src/engine/gpu/text.rs` · edit · type this

Replaces the `if let Some(rectangle) = center_nameplate(run…` line in `fn prepare` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22a"
```

Replaces the 4 lines from `run.label.color[0],` in `fn prepare` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22b"
```

Replaces `fn draw` in `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22c"
```

Replaces the `draws += self.plates.draw(pass);` line in `fn draw` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22d"
```

Replaces `fn center_nameplate` in `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22e"
```

Replaces the 2 lines from `placed.left -= width * scale * 0.5;` in `fn center_nameplate` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22f"
```

`lessons/17/src/engine/gpu/text.rs` · edit · type this

Replaces the 7 lines from `placed.left - padding[0] * scale,` in `fn center_nameplate` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22g"
```

Added after the `let label = TextLabel {` line in `fn logical_to_physical_scale_is_applied_once` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22h"
```

Added after the `let mut label = TextLabel {` line in `fn scene_anchor_preserves_rebased_depth_and_culls_near_plane` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22i"
```

Added after the `let mut label = TextLabel {` line in `fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22j"
```

Replaces the `let rectangle = center_nameplate(run, &mut pl…` line in `fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22k"
```

Replaces the 3 lines from `center_nameplate(run, &mut placed, &frame, 2.0)` in `fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22l"
```

Added after the `let label = TextLabel {` line in `fn nameplate_has_black_background_white_ink_centering_and_clean_release` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22m"
```

Replaces the `gpu.text.set_labels(vec![label]).unwrap();` line in `fn nameplate_has_black_background_white_ink_centering_and_clean_release` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22n"
```

Added after the `);` line in `fn nameplate_has_black_background_white_ink_centering_and_clean_release` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22o"
```

Replaces the `assert_eq!(gpu.text.stats.nameplate_capacity_…` line in `fn nameplate_has_black_background_white_ink_centering_and_clean_release` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22p"
```

Added after the `let mut label = TextLabel {` line in `fn actual_glyph_coverage_obeys_depth_clip_motion_and_release` of `lessons/16/src/engine/gpu/text.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22q"
```

Replaces this block of `lessons/16/src/engine/gpu/text.rs`:

```rust
        let mut label = TextLabel {
            id: 1,
```

```rust
--8<-- "lessons/17/src/engine/gpu/text.rs:step-22r"
```

## Step 23 · src/app/inspection.rs

Inspection reports retained resources and source information.

`lessons/17/src/app/inspection.rs` · edit · type this

Added after the `"samples": state.gpu.targets.samples,` line in `fn publish` of `lessons/16/src/app/inspection.rs`

```rust
--8<-- "lessons/17/src/app/inspection.rs:step-23"
```

## Step 24 · src/engine/gpu/surface_outline.rs

Surface masks add outlines around visible coverage.

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24a"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24b"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24c"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24d"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24e"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24f"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24g"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24h"
```

`lessons/17/src/engine/gpu/surface_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24i"
```

Copy this part from the lesson folder to the path shown.

`lessons/17/src/engine/gpu/surface_outline.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/17/src/engine/gpu/surface_outline.rs:step-24j"
```

## Step 25 · src/shaders/surface_outline.wgsl

The outline shader expands visible coverage into a narrow border.

`lessons/17/src/shaders/surface_outline.wgsl` · 109 lines · type this, new file

```wgsl
--8<-- "lessons/17/src/shaders/surface_outline.wgsl"
```

## Step 26 · src/engine/gpu/arena.rs

The arena holds mesh vertices and indices across objects.

`lessons/17/src/engine/gpu/arena.rs` · edit · type this

Added after the `}` line in `impl ArenaLane` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-26a"
```

Added after the `pub fn release(&mut self, ctx: &GpuCtx) {` line in `impl ArenaLane` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-26b"
```

Added after the `),` line in `fn build_pipelines` of `lessons/16/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/arena.rs:step-26c"
```

## Step 27 · src/engine/gpu/view.rs

View settings control display features without changing source geometry.

`lessons/17/src/engine/gpu/view.rs` · edit · type this

Added after the `pub show_mesh_edges: bool,` line in `struct View` of `lessons/16/src/engine/gpu/view.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/view.rs:step-27a"
```

Replaces the 5 lines from `markers: knob("BENCH_NO_MARKERS", "nomarkers"…` in `fn from_env` of `lessons/16/src/engine/gpu/view.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/view.rs:step-27b"
```

Added after the `}` line in `fn from_env` of `lessons/16/src/engine/gpu/view.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/view.rs:step-27c"
```

## Step 28 · src/app/input.rs

Input routes gestures and keyboard actions to State.

`lessons/17/src/app/input.rs` · edit · type this

Added after the `WindowEvent::CursorMoved { position, .. } => {` line in `fn mouse` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-28"
```

## Step 29 · src/app/inspection.rs

Inspection reports retained resources and source information.

`lessons/17/src/app/inspection.rs` · edit · type this

Added after the `"id": run.label.id,` line in `fn text_labels` of `lessons/16/src/app/inspection.rs`

```rust
--8<-- "lessons/17/src/app/inspection.rs:step-29"
```

## Step 30 · src/app/walk/curves.rs

Curve sampling builds connected strokes from source geometry.

`lessons/17/src/app/walk/curves.rs` · edit · type this

Added after the `pub(super) fn push_polyline(seg: &mut SegRows…` line of `lessons/16/src/app/walk/curves.rs`

```rust
--8<-- "lessons/17/src/app/walk/curves.rs:step-30a"
```

Added after the `}` line of `lessons/16/src/app/walk/curves.rs`

```rust
--8<-- "lessons/17/src/app/walk/curves.rs:step-30b"
```

## Step 31 · src/app/walk/brep_edges.rs

Boundary chains share samples between adjacent faces.

`lessons/17/src/app/walk/brep_edges.rs` · edit · type this

Added after the `let fm = &ep.fms[chain.face];` line in `fn push_edge_pipes` of `lessons/16/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/17/src/app/walk/brep_edges.rs:step-31a"
```

Added after the `}` line in `fn push_edge_pipes` of `lessons/16/src/app/walk/brep_edges.rs`

```rust
--8<-- "lessons/17/src/app/walk/brep_edges.rs:step-31b"
```

## Step 32 · src/engine/gpu/segments.rs

The segment buffers store strokes and the object rows they belong to.

`lessons/17/src/engine/gpu/segments.rs` · edit · type this

Added after the `};` line of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32a"
```

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32b"
```

Added after the `pub pipe_ids: Vec<u32>,` line in `struct SegRows` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32c"
```

Replaces the 2 lines from `drop_rows(&mut self.ribbons);` in `fn drop_rows` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32d"
```

Replaces the `std::mem::size_of::<CylinderSegment>() as u64,` line in `fn new` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32e"
```

`lessons/17/src/engine/gpu/segments.rs` · edit · type this

Added after the `ribbon: wgpu::RenderPipeline,` line in `struct SegPipelines` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32f"
```

Added after the `selection: wgpu::Buffer,` line in `struct SegmentLane` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32g"
```

Added after the `selection,` line in `fn new` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32h"
```

Replaces the 7 lines from `let pipes_changed = self.pipes.buf.append(ctx…` in `fn append` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32i"
```

Replaces `fn set_edge` in `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32j"
```

Added after the `);` line in `fn set_edge` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32k"
```

Added after the `pub fn reset(&mut self) {` line in `impl SegmentLane` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32l"
```

Added after the `pub fn release(&mut self, ctx: &GpuCtx, l: &L…` line in `impl SegmentLane` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32m"
```

`lessons/17/src/engine/gpu/segments.rs` · edit · type this

Added after the `SegPipelines {` line in `fn build_pipelines` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32n"
```

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32o"
```

Replaces the 11 lines from `];` in `fn cylinder_segment_mirror` of `lessons/16/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/segments.rs:step-32p"
```

## Step 33 · src/engine/gpu/instance.rs

Each object row carries placement, color and selection flags for later interaction.

`lessons/17/src/engine/gpu/instance.rs` · edit · type this

Replaces the `use crate::engine::gpu::segments::CylinderSeg…` line in `fn shader_validation_and_layouts` of `lessons/16/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/instance.rs:step-33a"
```

Replaces the `"CylinderSegment" => (` line in `fn shader_validation_and_layouts` of `lessons/16/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/instance.rs:step-33b"
```

Replaces the 2 lines from `],` in `fn shader_validation_and_layouts` of `lessons/16/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/instance.rs:step-33c"
```

Added after the `offset_of!(LineUniform, backface),` line in `fn shader_validation_and_layouts` of `lessons/16/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/instance.rs:step-33d"
```

Replaces the 25 lines from `];` in `fn line_uniform_mirror` of `lessons/16/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/instance.rs:step-33e"
```

## Step 34 · src/shaders/ribbon.wgsl

The stroke shader expands segments into screen-space ribbons.

`lessons/17/src/shaders/ribbon.wgsl` · edit · type this

Replaces `struct CylinderSegment` in `lessons/16/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/ribbon.wgsl:step-34a"
```

Replaces the 3 lines from `}` of `lessons/16/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/ribbon.wgsl:step-34b"
```

Added after the `@location(10) @interpolate(flat) source_edge:…` line in `struct VsOut` of `lessons/16/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/ribbon.wgsl:step-34c"
```

`lessons/17/src/shaders/ribbon.wgsl` · edit · type this

Replaces the 6 lines from `@vertex` of `lessons/16/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/ribbon.wgsl:step-34d"
```

Replaces the 5 lines from `let raw0 = half_width_px(seg.radius, e0.w);` in `fn vs_main` of `lessons/16/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/ribbon.wgsl:step-34e"
```

`lessons/17/src/shaders/ribbon.wgsl` · edit · type this

Replaces the `if ((inst.flags & FLAG_SELECTED) != 0u || (ed…` line in `fn vs_main` of `lessons/16/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/ribbon.wgsl:step-34f"
```

Replaces the 5 lines from `return o;` in `fn vs_main` of `lessons/16/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/ribbon.wgsl:step-34g"
```

Copy each file from the lesson folder to the path shown.

Copy from `lessons/17/` (tooling this checkpoint needs but the course does not teach):

- `lessons/17/examples/mk_mixed_solids.rs`
- `lessons/17/examples/mk_selection_overlap.rs`
- `lessons/17/examples/mk_stroke_joins.rs`
- `lessons/17/src/selftest.rs`
- `lessons/17/src/text_quality.rs`
- `lessons/17/tests/interaction.cjs`
- `lessons/17/tests/selection-overlap.py`
- `lessons/17/tests/stroke-joins.py`
- `lessons/17/tests/world-text.cjs`

## Step 35 · src/shaders/splat.wgsl

Point projection writes the nearest visible cloud samples.

`lessons/17/src/shaders/splat.wgsl` · edit · type this

Added after the `edl: f32,` line in `struct CloudUniform` of `lessons/16/src/shaders/splat.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/splat.wgsl:step-35a"
```

Replaces the 3 lines from `s.r = clamp(bitcast<f32>(table[base + 23u]) *…` in `fn project` of `lessons/16/src/shaders/splat.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/splat.wgsl:step-35b"
```

## Step 36 · src/shaders/splat_resolve.wgsl

The resolve writes point color and depth into the scene.

`lessons/17/src/shaders/splat_resolve.wgsl` · edit · type this

Added after the `edl: f32,` line in `struct CloudUniform` of `lessons/16/src/shaders/splat_resolve.wgsl`

```wgsl
--8<-- "lessons/17/src/shaders/splat_resolve.wgsl:step-36"
```

## Step 37 · src/app/input.rs

Input routes gestures and keyboard actions to State.

`lessons/17/src/app/input.rs` · edit · type this

Replaces the 7 lines from `false` in `fn mouse` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-37a"
```

Added after the `Act::Tap(at) => {` line in `fn mouse` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-37b"
```

Added after the `self.ctrl = false;` line in `fn cancel` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-37c"
```

Added after the `self.ctrl,` line in `fn left` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-37d"
```

Replaces the 12 lines from `#[cfg(target_arch = "wasm32")]` of `lessons/16/src/app/input.rs`

```rust
--8<-- "lessons/17/src/app/input.rs:step-37e"
```

## Step 38 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners.

`lessons/17/src/lib.rs` · edit · type this

Replaces the 2 lines from `let win = web_sys::window()?;` in `fn desired_canvas_size` of `lessons/16/src/lib.rs`

```rust
--8<-- "lessons/17/src/lib.rs:step-38"
```

## Step 39 · src/engine/gpu/targets.rs

Targets own the depth and color attachments for a frame.

`lessons/17/src/engine/gpu/targets.rs` · edit · type this

Added after the `const MSAA_PIXELS_DISCRETE: u32 = 9_000_000;` line of `lessons/16/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/targets.rs:step-39a"
```

Replaces `fn samples_for` in `lessons/16/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/targets.rs:step-39b"
```

Replaces the 4 lines from `Targets::samples_for(true, 3840 * 2160, Some(…` in `fn msaa_follows_the_adapter` of `lessons/16/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/targets.rs:step-39c"
```

Replaces the 3 lines from `assert_eq!(Targets::samples_for(true, 2560 *…` in `fn the_browser_arm_is_not_the_integrated_one` of `lessons/16/src/engine/gpu/targets.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/targets.rs:step-39d"
```

## Step 40 · src/app/route.rs

Route helpers read viewer options from the page URL.

`lessons/17/src/app/route.rs` · edit · type this

Added after the `}` line of `lessons/16/src/app/route.rs`

```rust
--8<-- "lessons/17/src/app/route.rs:step-40"
```

## Step 41 · src/app/feedback.rs

Feedback publishes status and panel information from the same application state.

`lessons/17/src/app/feedback.rs` · edit · type this

Added after the `pub fn status(message: &str) {` line of `lessons/16/src/app/feedback.rs`

```rust
--8<-- "lessons/17/src/app/feedback.rs:step-41"
```

## Step 42 · src/state.rs

State coordinates input, selection and frame requests.

`lessons/17/src/state.rs` · edit · type this

Delete `fn cloud_query_awaiting_gpu` from `lessons/16/src/state.rs`.

Delete `fn label_center` from `lessons/16/src/state.rs`.

## Step 43 · src/engine/gpu/selection_outline.rs

Remove this file; its replacement is now part of the rendering modules.

Delete `src/engine/gpu/selection_outline.rs` (it exists in `lessons/16/`, not in `lessons/17/`).

## Step 44 · src/shaders/selection_outline.wgsl

Remove this file; its replacement is now part of the rendering modules.

Delete `src/shaders/selection_outline.wgsl` (it exists in `lessons/16/`, not in `lessons/17/`).

## Step 45 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources.

`lessons/17/src/engine/gpu/mod.rs` · edit · type this

Added after the `pub mod device;` line of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45a"
```

Added after the `pub mod splat;` line in `mod splat` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45b"
```

Replaces the `pub selection_outline: selection_outline::Sel…` line in `struct Gpu` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45c"
```

Replaces the `let (outline_buffers, outline_textures) = sel…` line in `fn allocated_bytes` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45d"
```

Replaces the `let selection_outline = selection_outline::Se…` line in `fn build` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45e"
```

Added after the `selection_outline,` line in `fn build` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45f"
```

Added after the `self.selection_outline.retarget(&self.ctx, ta…` line in `fn retarget` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45g"
```

Added after the `self.msaa_budget(),` line in `fn msaa_now` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45h"
```

Replaces the 7 lines from `self.arena.reset();` in `fn reset` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45i"
```

Replaces this block of `lessons/16/src/engine/gpu/mod.rs`:

```rust
        self.selection_outline.reset();
        self.pick.cancel();
```

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45j"
```

Added after the `pub fn set_selected(&mut self, row: u32, on:…` line in `impl Gpu` of `lessons/16/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/mod.rs:step-45k"
```

## Step 46 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes.

`lessons/17/src/engine/gpu/render.rs` · edit · type this

Replaces the 9 lines from `self.arena.face_count() > 0,` in `fn encode_frame` of `lessons/16/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/render.rs:step-46a"
```

Replaces the 19 lines from `let basic = Binds {` in `fn scene_list` of `lessons/16/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/render.rs:step-46b"
```

Delete the 59 lines from `let window = match at {` in `fn scene_list` of `lessons/16/src/engine/gpu/render.rs`.

Replaces the 59 lines from `let window = match at {` in `fn scene_list` of `lessons/16/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/render.rs:step-46d"
```

Replaces the 14 lines from `mvp: &self.frame.mvp_group,` in `fn scene_list` of `lessons/16/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/render.rs:step-46e"
```

Replaces the 4 lines from `}` in `fn scene_list` of `lessons/16/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/17/src/engine/gpu/render.rs:step-46f"
```

## Check

Run `trunk serve` in `lessons/17/` and open <http://127.0.0.1:8770/>.

Expected: Source faces and scene text become selectable, and optional outlines surround visible solids; status: **Face N selected**.

![Checkpoint 17. Left: Ctrl + Shift + click inside the mesh selects one source face, the rest of the object stays grey. Middle: the selected BRep with its silhouette after pressing O, one black border of uniform width around the yellow fill. Right: without the silhouette only the yellow strokes remain.](screenshots/17-face-silhouette.png)

If it fails:

- A face click selects an edge: edges intentionally win where both are hit.
- Text stays visible after hiding: its row is missing from the scene visibility path.
- Joints show dark dots: connected segments overlap instead of sharing a join partition.

## What changed

```text
lessons/17/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
│   │   ├── brep_edges.rs  ~
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs  ~
│   │   ├── encode.rs
│   │   ├── frames.rs
│   │   ├── mesh.rs  ~
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   └── points.rs
│   ├── cloud_query.rs
│   ├── decode.rs
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── input.rs  ~
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs  ~
│   ├── mod.rs
│   ├── route.rs  ~
│   ├── scene.rs  ~
│   ├── scene_text.rs  +
│   ├── selection.rs  ~
│   ├── stream.rs
│   ├── touch.rs
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs  +
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── pick.rs  ~
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs  ~
│   │   ├── splat.rs
│   │   ├── surface_outline.rs  +
│   │   ├── targets.rs  ~
│   │   ├── text.rs  ~
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs  ~
│   │   ├── text_plate.rs  ~
│   │   ├── upload.rs
│   │   └── view.rs  ~
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs  ~
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── ribbon.wgsl  ~
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl  ~
│   ├── splat_resolve.wgsl  ~
│   ├── surface_outline.wgsl  +
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl  ~
│   ├── text_plate.wgsl  ~
│   └── triangle.wgsl  ~
├── state/
│   ├── cloud_query.rs  +
│   └── text.rs  +
├── camera.rs
├── lib.rs  ~
└── state.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/17/`.

## Next

[18 · Finite-triangle visibility](18-finite-visibility.md): why a neighbouring triangle's plane can hide a visible seam, and the tile index that fixes it.

## Expected viewer result

Select a face, read the authored text, and see silhouettes on the teapot.

[![Full viewer result for 17 source presentation](screenshots/17.png)](screenshots/17.png)
