# 13 · Source controls

F10 shows original curve and surface controls, and clicking a control highlights it.

![A resident prefix is a fraction of the cloud, so a click walks every intersecting octree node whether or not it was downloaded, and accumulates the answer one bounded page at a time against the depth the frame already has.](illustrations/cloud-pick.svg)

## Step 1 · src/app/selection.rs

Selection keeps original edge, face and control IDs under their parent object.

`lessons/13/src/app/selection.rs` · edit · type this

Added at the top of `lessons/12/src/app/selection.rs`

```rust
--8<-- "lessons/13/src/app/selection.rs:step-1a"
```

`lessons/13/src/app/selection.rs` · edit · type this

Replaces the 24 lines from `}` of `lessons/12/src/app/selection.rs`

```rust
--8<-- "lessons/13/src/app/selection.rs:step-1b"
```

## Step 2 · src/app/fetch.rs

Fetch helpers retrieve source bytes and report the failing stage.

`lessons/13/src/app/fetch.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/13/src/app/fetch.rs:step-2a"
```

`lessons/13/src/app/fetch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/app/fetch.rs:step-2b"
```

Copy this part from the lesson folder to the path shown.

`lessons/13/src/app/fetch.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/13/src/app/fetch.rs:step-2c"
```

Copy this part from the lesson folder to the path shown.

`lessons/13/src/app/fetch.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/13/src/app/fetch.rs:step-2d"
```

## Step 3 · src/app/cloud_query.rs

Cloud queries resolve original points beyond the display prefix.

`lessons/13/src/app/cloud_query.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3a"
```

`lessons/13/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3b"
```

`lessons/13/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3c"
```

`lessons/13/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3d"
```

`lessons/13/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3e"
```

`lessons/13/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3f"
```

Copy this part from the lesson folder to the path shown.

`lessons/13/src/app/cloud_query.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3g"
```

Copy this part from the lesson folder to the path shown.

`lessons/13/src/app/cloud_query.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/13/src/app/cloud_query.rs:step-3h"
```

Run `cargo check` in `lessons/13/`.

## Step 4 · src/app/stream.rs

Copy this file from the lesson folder to the path shown.

`lessons/13/src/app/stream.rs` · edit · copy the file

Replaces `fn is_empty` in `lessons/12/src/app/stream.rs`

```rust
--8<-- "lessons/13/src/app/stream.rs:step-4"
```

## Step 5 · src/engine/gpu/pick.rs

Picking reads an object and subobject ID asynchronously.

`lessons/13/src/engine/gpu/pick.rs` · edit · type this

Added after the `Edge,` line in `enum PickMode` of `lessons/12/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/13/src/engine/gpu/pick.rs:step-5a"
```

Added after the `radius: u32,` line in `struct Picker` of `lessons/12/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/13/src/engine/gpu/pick.rs:step-5b"
```

Added after the `radius: PICK_RADIUS,` line in `fn new` of `lessons/12/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/13/src/engine/gpu/pick.rs:step-5c"
```

Replaces the 2 lines from `self.generation = self.generation.wrapping_ad…` in `fn cancel` of `lessons/12/src/engine/gpu/pick.rs`

```rust
--8<-- "lessons/13/src/engine/gpu/pick.rs:step-5d"
```

## Step 6 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes.

`lessons/13/src/engine/gpu/render.rs` · edit · type this

Added after the `draws += self.selection_outline.draw(pass);` line in `fn scene_list` of `lessons/12/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/13/src/engine/gpu/render.rs:step-6a"
```

Added after the `};` line in `fn scene_list` of `lessons/12/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/13/src/engine/gpu/render.rs:step-6b"
```

Added after the `}` line in `fn scene_list` of `lessons/12/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/13/src/engine/gpu/render.rs:step-6c"
```

## Step 7 · src/state.rs

State coordinates input, selection and frame requests.

`lessons/13/src/state.rs` · edit · type this

Replaces the 4 lines from `use crate::app::selection::SelectionMode;` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7a"
```

Replaces the 4 lines from `requested: PickMode,` in `struct State` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7b"
```

Replaces the 4 lines from `requested: PickMode::Object,` in `fn new` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7c"
```

`lessons/13/src/state.rs` · edit · type this

Added after the `self.selection = SelectionMode::Object;` line in `fn clear` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7d"
```

Added after the `self.gpu.logical_size = self.logical_size();` line in `fn resize` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7e"
```

Added after the `pub fn touch(&mut self) {` line in `impl State` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7f"
```

Added after the `self.selection = SelectionMode::Object;` line in `fn select` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7g"
```

Added after the `self.scene.selected = row;` line in `fn select` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7h"
```

`lessons/13/src/state.rs` · edit · type this

Added after the `fn apply_pick(&mut self, pick: Option<Pick>) {` line in `impl State` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7i"
```

Added after the `self.status(&format!("Edge {edge} selected"));` line in `fn apply_pick` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7j"
```

`lessons/13/src/state.rs` · edit · type this

Added after the `self.gpu.logical_size = logical;` line in `fn render` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7k"
```

Added after the `crate::app::feedback::error(&message);` line in `fn render` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7l"
```

Replaces the 5 lines from `}` in `fn render` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7m"
```

Replaces the `if self.dirty {` line in `fn render` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7n"
```

`lessons/13/src/state.rs` · edit · type this

Replaces the 5 lines from `self.gpu.pick.cancel();` in `fn request_selection` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7o"
```

`lessons/13/src/state.rs` · edit · type this

Added after the `}` line in `impl State` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7p"
```

`lessons/13/src/state.rs` · edit · type this

Added after the `}` line in `impl State` of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7q"
```

`lessons/13/src/state.rs` · edit · type this

Replaces `fn update_label` in `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7r"
```

Replaces the `/// Center an annotation in the source object…` line of `lessons/12/src/state.rs`

```rust
--8<-- "lessons/13/src/state.rs:step-7s"
```

```rust
--8<-- "lessons/13/src/state.rs:step-7t"
```

## Step 8 · src/app/input.rs

Input routes gestures and keyboard actions to State.

`lessons/13/src/app/input.rs` · edit · type this

Added after the `Key::Named(NamedKey::Escape) => state.escape_…` line in `fn key` of `lessons/12/src/app/input.rs`

```rust
--8<-- "lessons/13/src/app/input.rs:step-8"
```

## Step 9 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners.

`lessons/13/src/lib.rs` · edit · type this

Added after the `CloudChunk(CloudChunk),` line in `enum Msg` of `lessons/12/src/lib.rs`

```rust
--8<-- "lessons/13/src/lib.rs:step-9a"
```

Added after the `Msg::CloudChunk(c) => state.extend_streamed(c…` line in `fn user_event` of `lessons/12/src/lib.rs`

```rust
--8<-- "lessons/13/src/lib.rs:step-9b"
```

## Step 10 · src/app/mod.rs

The application module connects source loading and interaction helpers.

`lessons/13/src/app/mod.rs` · edit · type this

Added at the top of `lessons/12/src/app/mod.rs`

```rust
--8<-- "lessons/13/src/app/mod.rs:step-10a"
```

Added after the `#[cfg(target_arch = "wasm32")]` line of `lessons/12/src/app/mod.rs`

```rust
--8<-- "lessons/13/src/app/mod.rs:step-10b"
```

## Step 11 · src/app/inspection.rs

Copy this file from the lesson folder to the path shown.

`lessons/13/src/app/inspection.rs` · edit · copy the file

Replaces the `"controls": Vec::<serde_json::Value>::new(),` line in `fn publish` of `lessons/12/src/app/inspection.rs`

```rust
--8<-- "lessons/13/src/app/inspection.rs:step-11"
```

## Step 12 · src/app/loader.rs

The loader stages manifest and geometry work before publishing it.

`lessons/13/src/app/loader.rs` · edit · type this

Replaces the 2 lines from `use super::scene::{FileDoc, Scene};` of `lessons/12/src/app/loader.rs`

```rust
--8<-- "lessons/13/src/app/loader.rs:step-12a"
```

Added after the `async fn fixture() -> Result<(), String> {` line of `lessons/12/src/app/loader.rs`

```rust
--8<-- "lessons/13/src/app/loader.rs:step-12b"
```

Added after the `}` line of `lessons/12/src/app/loader.rs`

```rust
--8<-- "lessons/13/src/app/loader.rs:step-12c"
```

## Step 13 · src/app/scene.rs

Copy this file from the lesson folder to the path shown.

`lessons/13/src/app/scene.rs` · edit · copy the file

Added after the `}` line of `lessons/12/src/app/scene.rs`

```rust
--8<-- "lessons/13/src/app/scene.rs:step-13"
```

## Check

Run `trunk serve` in `lessons/13/` and open <http://127.0.0.1:8770/>.

Expected: F10 shows original curve and surface controls, and clicking a control highlights it; status: **Selected Surface { surface: 0, u: 0, v: 1 }**.

![Checkpoint 13, left to right: F10 on the curve shows its three control points and control polygon; F10 on the surface shows the four corners of its control net; a clicked corner turns yellow and the status reads `Selected Surface { surface: 0, u: 0, v: 1 }`; F10 on the mesh shows its original vertices, not the tessellation.](screenshots/13-controls.png)

If it fails:

- Repeated F10 adds markers: enabling controls appends instead of replacing them.
- The surface shows a dense grid: tessellation vertices replace source controls.

## What changed

```text
lessons/13/src/
├── app/
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── frames.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   └── points.rs
│   ├── cloud_query.rs  +
│   ├── feedback.rs
│   ├── fetch.rs  +
│   ├── input.rs  ~
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── loader.rs  ~
│   ├── mod.rs  ~
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── selection.rs  ~
│   ├── stream.rs  ~
│   └── touch.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs
│   │   ├── objects.rs
│   │   ├── pick.rs  ~
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs
│   │   ├── selection_outline.rs
│   │   ├── splat.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── selection_outline.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   └── triangle.wgsl
├── camera.rs
├── lib.rs  ~
└── state.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/13/`.

## Next

[14 · Loading scenes](14-loading.md): manifests, protobuf documents, validation and safe replacement through the real loader.

## Expected viewer result

Select a curve or surface and press **F10**: its control points and control polygon appear.

[![Full viewer result for 13 controls](screenshots/13.png)](screenshots/13.png)
