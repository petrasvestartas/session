# current-5 · Build the nested session and graph panel

The layer panel expands nested groups and selects or hides their descendant objects.

## Step 1 · index.html
Copy this file from the lesson folder to the path shown.

`lessons/current-5/index.html` · edit · copy the file

Replaces the line `style="position: fixed; top: 12px; left: 12px; min-width:…` in `lessons/current-4/index.html`

```html
--8<-- "lessons/current-5/index.html:step-1"
```

## Step 2 · src/app/feedback.rs
Carry status and layer information from the scene into the interface.

`lessons/current-5/src/app/feedback.rs` · edit · type this

Replaces the line `"display:block;width:100%;text-align:left;border:0;backgr…` in `lessons/current-4/src/app/feedback.rs`

```rust
--8<-- "lessons/current-5/src/app/feedback.rs:step-2"
```

## Step 3 · src/app/hierarchy.rs
Index document trees and graph endpoints into bounded sets of render rows.

`lessons/current-5/src/app/hierarchy.rs` · 338 lines · type this, new file

```rust
--8<-- "lessons/current-5/src/app/hierarchy.rs"
```

## Step 4 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data.

`lessons/current-5/src/app/inspection.rs` · edit · type this

Added after the line `"selected": parent,` in `lessons/current-4/src/app/inspection.rs`

```rust
--8<-- "lessons/current-5/src/app/inspection.rs:step-4"
```

## Step 5 · src/app/layers.rs
Count visible and hidden members from the layer membership sets.

`lessons/current-5/src/app/layers.rs` · edit · type this

Added after the line `pub fn rows(scene: &Scene) -> Vec<Row> {` in `lessons/current-4/src/app/layers.rs`

```rust
--8<-- "lessons/current-5/src/app/layers.rs:step-5a"
```

Delete the `fn all_hidden` block from `lessons/current-4/src/app/layers.rs`.

Added after the line `#[test]` in `lessons/current-4/src/app/layers.rs`

```rust
--8<-- "lessons/current-5/src/app/layers.rs:step-5c"
```

## Step 6 · src/app/mod.rs
Declare the new application modules so their files join the crate.

`lessons/current-5/src/app/mod.rs` · edit · type this

Added after the line `pub mod gizmo;` in `lessons/current-4/src/app/mod.rs`

```rust
--8<-- "lessons/current-5/src/app/mod.rs:step-6"
```

## Step 7 · src/app/scene.rs
Map source trees and graph endpoints to row identities, and track revisions as documents change.

`lessons/current-5/src/app/scene.rs` · edit · type this

Added after the line `pub(crate) created_doc: Option<usize>,` in `lessons/current-4/src/app/scene.rs`

```rust
--8<-- "lessons/current-5/src/app/scene.rs:step-7a"
```

Added after the line `created_doc: None,` in `lessons/current-4/src/app/scene.rs`

```rust
--8<-- "lessons/current-5/src/app/scene.rs:step-7b"
```

Added after the line `fn reset_rows(&mut self) {` in `lessons/current-4/src/app/scene.rs`

```rust
--8<-- "lessons/current-5/src/app/scene.rs:step-7c"
```

Added after the line `pub(super) fn push_row(&mut self, owner: usize, guid: &st…` in `lessons/current-4/src/app/scene.rs`

```rust
--8<-- "lessons/current-5/src/app/scene.rs:step-7d"
```

Added after the line `pub fn add_file(&mut self, doc: FileDoc) {` in `lessons/current-4/src/app/scene.rs`

```rust
--8<-- "lessons/current-5/src/app/scene.rs:step-7e"
```

## Step 8 · src/lib.rs
Connect browser events, scene changes and drawing through the application state.

`lessons/current-5/src/lib.rs` · edit · type this

Replaces the 3 lines from `if let Some(layer) = app::layers::Layer::from_key(&key) {` in `lessons/current-4/src/lib.rs`

```rust
--8<-- "lessons/current-5/src/lib.rs:step-8"
```

## Step 9 · src/state.rs
Own the hierarchy index, refresh its labels and keep group selection separate from one selected object.

`lessons/current-5/src/state.rs` · edit · type this

Added after the line `pub mod edit;` in `lessons/current-4/src/state.rs`

```rust
--8<-- "lessons/current-5/src/state.rs:step-9a"
```

Added after the line `pub selection: SelectionMode,` in `lessons/current-4/src/state.rs`

```rust
--8<-- "lessons/current-5/src/state.rs:step-9b"
```

Added after the line `selection: SelectionMode::Object,` in `lessons/current-4/src/state.rs`

```rust
--8<-- "lessons/current-5/src/state.rs:step-9c"
```

Added after the line `self.cancel_gesture();` in `lessons/current-4/src/state.rs`

```rust
--8<-- "lessons/current-5/src/state.rs:step-9d"
```

Replaces the 2 lines from `self.cancel_gesture();` in `lessons/current-4/src/state.rs`

```rust
--8<-- "lessons/current-5/src/state.rs:step-9e"
```

Added after the line `pub fn hide_selected(&mut self) {` in `lessons/current-4/src/state.rs`

```rust
--8<-- "lessons/current-5/src/state.rs:step-9f"
```

## Step 10 · src/state/edit.rs
Apply visibility and deletion through the hierarchy, then refresh the affected rows.

`lessons/current-5/src/state/edit.rs` · edit · type this

Added after the line `if !self.scene.delete_row(row) {` in `lessons/current-4/src/state/edit.rs`

```rust
--8<-- "lessons/current-5/src/state/edit.rs:step-10a"
```

Added after the line `fn after_history(&mut self) {` in `lessons/current-4/src/state/edit.rs`

```rust
--8<-- "lessons/current-5/src/state/edit.rs:step-10b"
```

Replaces the 3 lines from `let hidden: Vec<bool> = rows` in `lessons/current-4/src/state/edit.rs`

```rust
--8<-- "lessons/current-5/src/state/edit.rs:step-10c"
```

Replaces the line `let rows: Vec<crate::app::feedback::LayerRow> = layers::r…` in `lessons/current-4/src/state/edit.rs`

```rust
--8<-- "lessons/current-5/src/state/edit.rs:step-10d"
```

Added after the line `.collect();` in `lessons/current-4/src/state/edit.rs`

```rust
--8<-- "lessons/current-5/src/state/edit.rs:step-10e"
```

## Step 11 · src/state/panel.rs
Turn panel actions into recursive selection and visibility updates.

`lessons/current-5/src/state/panel.rs` · 183 lines · type this, new file

```rust
--8<-- "lessons/current-5/src/state/panel.rs"
```

## Check

Run `trunk serve` in `lessons/current-5/` and open <http://127.0.0.1:8770/>.

Expected: expanding a group exposes its children, selecting it highlights its members, and the status area shows no error.

![Full viewer result for current 5](screenshots/extensions-panels.png)

If it fails:

- A parent selects stale objects: the hierarchy was not refreshed after a revision.
- An oversized tree loses members: index limits are not reported before truncation.

## What changed

```text
lessons/current-5/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
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
│   │   ├── points.rs
│   │   └── sheet.rs
│   ├── cloud_query.rs
│   ├── command.rs
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── edit.rs
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── hierarchy.rs  +
│   ├── input.rs
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── layers.rs  ~
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs
│   │   ├── objects.rs
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── upload.rs
│   │   ├── view.rs
│   │   ├── widget.rs
│   │   └── widget_mesh.rs
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
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── panel.rs  +
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs  ~
└── state.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source hierarchy → descendant row ranges → selection and visibility.
Every file at this point: `lessons/current-5/`.

## Next

[Continue with current-6](current-6.md).

## Expected viewer result

The expanded Session layers panel shows Assembly → Nested; selecting Nested highlights its two beams together. This maintained-viewer capture uses egui styling; this checkpoint uses DOM buttons with the same hierarchy and selection behavior. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb). The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 5](screenshots/extensions-panels.png)](screenshots/extensions-panels.png)
