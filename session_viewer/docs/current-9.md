# current-9 · Keep source dragging live and build one layer tree

One layer tree controls visibility, locking and color while shell dragging updates retained surface samples.

## Step 1 · src/app/command.rs
Show command syntax and a usable example while the reader types.

`lessons/current-9/src/app/command.rs` · edit · type this

Added after the line `Escape,` in `lessons/current-8/src/app/command.rs`

```rust
--8<-- "lessons/current-9/src/app/command.rs:step-1"
```

## Step 2 · src/app/deform.rs
Validate changed surface boundaries before accepting a shell deformation.

`lessons/current-9/src/app/deform.rs` · edit · type this

Replaces the line `for curve in &mut next.m_curves_3d {` in `lessons/current-8/src/app/deform.rs`

```rust
--8<-- "lessons/current-9/src/app/deform.rs:step-2a"
```

Replaces the line `fn validate_boundaries(brep: &session_rust::BRep) -> Resu…` in `lessons/current-8/src/app/deform.rs`

```rust
--8<-- "lessons/current-9/src/app/deform.rs:step-2b"
```

Replaces the line `validate_boundaries(&next).unwrap();` in `lessons/current-8/src/app/deform.rs`

```rust
--8<-- "lessons/current-9/src/app/deform.rs:step-2c"
```

## Step 3 · src/app/edit.rs
Try an in-place source preview before falling back to a complete geometry rebuild.

`lessons/current-9/src/app/edit.rs` · edit · type this

Added after the line `let (doc, guid) = self.writable(row).ok_or("Source is not…` in `lessons/current-8/src/app/edit.rs`

```rust
--8<-- "lessons/current-9/src/app/edit.rs:step-3"
```

## Step 4 · src/app/feedback.rs
Carry status and layer information from the scene into the interface.

`lessons/current-9/src/app/feedback.rs` · edit · type this

Replaces the line `#[derive(Clone)]` in `lessons/current-8/src/app/feedback.rs`

```rust
--8<-- "lessons/current-9/src/app/feedback.rs:step-4"
```

## Step 5 · src/app/hierarchy.rs
Index document trees and graph endpoints into bounded sets of render rows.

`lessons/current-9/src/app/hierarchy.rs` · edit · type this

Added after the line `use crate::app::scene::Scene;` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5a"
```

Replaces the line `for (doc, file) in scene.docs.iter().enumerate() {` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5b"
```

Added after the line `let mut seen = HashSet::new();` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5c"
```

Replaces the line `if let Some(row) = row {` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5d"
```

Replaces the lines from `if self.rows.len() == self.nodes[start].rows.start {` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5e"
```

Added after the line `}` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5f"
```

Added after the line `use session_rust::Point;` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5g"
```

Added after the line `assert_eq!(index.targets(child), vec![0]);` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5h"
```

Replaces the line `assert_eq!(index.rows.len(), count);` in `lessons/current-8/src/app/hierarchy.rs`

```rust
--8<-- "lessons/current-9/src/app/hierarchy.rs:step-5i"
```

## Step 6 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data.

`lessons/current-9/src/app/inspection.rs` · edit · type this

Replaces the line `let snapshot = serde_json::json!({` in `lessons/current-8/src/app/inspection.rs`

```rust
--8<-- "lessons/current-9/src/app/inspection.rs:step-6a"
```

Added after the line `});` in `lessons/current-8/src/app/inspection.rs`

```rust
--8<-- "lessons/current-9/src/app/inspection.rs:step-6b"
```

## Step 7 · src/app/mod.rs
Declare the new application modules so their files join the crate.

`lessons/current-9/src/app/mod.rs` · edit · type this

Added after the line `pub mod ui;` in `lessons/current-8/src/app/mod.rs`

```rust
--8<-- "lessons/current-9/src/app/mod.rs:step-7"
```

## Step 8 · src/app/scene.rs
Retain per-object upload ranges, surface preview samples, locks and color overrides.

`lessons/current-9/src/app/scene.rs` · edit · type this

Added after the line `pub hidden: HashSet<(usize, Rc<str>)>,` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8a"
```

Added after the line `bases: Bases,` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8b"
```

Added after the line `impl Scene {` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8c"
```

Added after the line `hidden: HashSet::new(),` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8d"
```

Added after the line `bases: Bases::default(),` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8e"
```

Added after the line `self.hidden.clear();` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8f"
```

Added after the line `self.bases = Bases::default();` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8g"
```

Added after the line `self.bases.ribbon += self.tables.seg.ribbons.len() as u32;` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8h"
```

Added after the line `self.tables.obj.rows.push(ObjectRow::new(place, flags));` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8i"
```

Added after the line `};` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8j"
```

Added after the line `}` in `lessons/current-8/src/app/scene.rs`

```rust
--8<-- "lessons/current-9/src/app/scene.rs:step-8k"
```

## Step 9 · src/app/session_io.rs
Store visibility, locks and color overrides with each saved document.

`lessons/current-9/src/app/session_io.rs` · edit · type this

Added after the line `hidden: Vec<(usize, String)>,` in `lessons/current-8/src/app/session_io.rs`

```rust
--8<-- "lessons/current-9/src/app/session_io.rs:step-9a"
```

Added after the line `hidden.sort();` in `lessons/current-8/src/app/session_io.rs`

```rust
--8<-- "lessons/current-9/src/app/session_io.rs:step-9b"
```

Added after the line `hidden,` in `lessons/current-8/src/app/session_io.rs`

```rust
--8<-- "lessons/current-9/src/app/session_io.rs:step-9c"
```

Added after the line `.map(|(doc, id)| (doc, Rc::from(id)))` in `lessons/current-8/src/app/session_io.rs`

```rust
--8<-- "lessons/current-9/src/app/session_io.rs:step-9d"
```

Added after the line `scene.hidden.insert(scene.identity_of(1).unwrap());` in `lessons/current-8/src/app/session_io.rs`

```rust
--8<-- "lessons/current-9/src/app/session_io.rs:step-9e"
```

## Step 10 · src/app/surface_preview.rs
Retain UV samples and boundary endpoints, reevaluate changed surfaces, then patch their existing ranges.

`lessons/current-9/src/app/surface_preview.rs` · 268 lines · type this, new file

```rust
--8<-- "lessons/current-9/src/app/surface_preview.rs"
```

## Step 11 · src/app/ui.rs
Draw one layer tree with bulbs, locks, color controls and command hints.

`lessons/current-9/src/app/ui.rs` · edit · type this

Replaces the line `let snapshot = MODEL.with_borrow(|model| serde_json::json…` in `lessons/current-8/src/app/ui.rs`

```rust
--8<-- "lessons/current-9/src/app/ui.rs:step-11a"
```

Replaces the 8 lines from `let mut at = 0;` in `lessons/current-8/src/app/ui.rs`

```rust
--8<-- "lessons/current-9/src/app/ui.rs:step-11b"
```

Replaces the 4 lines from `fn layer_button(ui: &mut egui::Ui, row: &LayerRow) -> egu…` in `lessons/current-8/src/app/ui.rs`

```rust
--8<-- "lessons/current-9/src/app/ui.rs:step-11c"
```

Replaces the line `let height = if model.command_open { 160.0 } else { 76.0 };` in `lessons/current-8/src/app/ui.rs`

```rust
--8<-- "lessons/current-9/src/app/ui.rs:step-11d"
```

Added after the line `ui.separator();` in `lessons/current-8/src/app/ui.rs`

```rust
--8<-- "lessons/current-9/src/app/ui.rs:step-11e"
```

Replaces the line `.hint_text("Type a command"),` in `lessons/current-8/src/app/ui.rs`

```rust
--8<-- "lessons/current-9/src/app/ui.rs:step-11f"
```

## Step 12 · src/app/walk/brep.rs
Record surface parameters alongside tessellated vertices for later preview updates.

`lessons/current-9/src/app/walk/brep.rs` · edit · type this

Added after the line `}` in `lessons/current-8/src/app/walk/brep.rs`

```rust
--8<-- "lessons/current-9/src/app/walk/brep.rs:step-12a"
```

Added after the line `}` in `lessons/current-8/src/app/walk/brep.rs`

```rust
--8<-- "lessons/current-9/src/app/walk/brep.rs:step-12b"
```

## Step 13 · src/app/walk/points.rs
Give a newly created point a visible screen-size marker.

`lessons/current-9/src/app/walk/points.rs` · edit · type this

Added after the line `let center = p.to_f32();` in `lessons/current-8/src/app/walk/points.rs`

```rust
--8<-- "lessons/current-9/src/app/walk/points.rs:step-13"
```

## Step 14 · src/engine/gpu/arena.rs
Patch the existing face and vertex ranges during a preview.

`lessons/current-9/src/engine/gpu/arena.rs` · edit · type this

Added after the line `pub face_sources: Vec<super::faces::FaceSource>,` in `lessons/current-8/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/arena.rs:step-14a"
```

Added after the line `drop_rows(&mut self.face_sources);` in `lessons/current-8/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/arena.rs:step-14b"
```

Added after the line `.append(ctx, up, [&self.verts.buf, &self.vids.buf, &self.…` in `lessons/current-8/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/arena.rs:step-14c"
```

## Step 15 · src/engine/gpu/buffers.rs
Allow existing GPU buffers to receive geometry patches.

`lessons/current-9/src/engine/gpu/buffers.rs` · edit · type this

Added after the line `pub fn write_at<T: Pod>(&self, ctx: &GpuCtx, at: u32, dat…` in `lessons/current-8/src/engine/gpu/buffers.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/buffers.rs:step-15"
```

## Step 16 · src/engine/gpu/faces.rs
Keep source-face identities attached to the updated triangle ranges.

`lessons/current-9/src/engine/gpu/faces.rs` · edit · type this

Added after the line `}));` in `lessons/current-8/src/engine/gpu/faces.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/faces.rs:step-16"
```

## Step 17 · src/engine/gpu/glyphs.rs
Patch existing marker ranges during a source preview.

`lessons/current-9/src/engine/gpu/glyphs.rs` · edit · type this

Added after the line `impl GlyphLane {` in `lessons/current-8/src/engine/gpu/glyphs.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/glyphs.rs:step-17"
```

## Step 18 · src/engine/gpu/instance.rs
Add a flag for display color overrides to the instance row.

`lessons/current-9/src/engine/gpu/instance.rs` · edit · type this

Added after the line `pub const FLAG_SINGLE: u32 = 1 << 7;` in `lessons/current-8/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/instance.rs:step-18"
```

## Step 19 · src/engine/gpu/mod.rs
Add the new GPU resources, initialize them and include their allocations in the counters.

`lessons/current-9/src/engine/gpu/mod.rs` · edit · type this

Added after the line `pub mod objects;` in `lessons/current-8/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/mod.rs:step-19a"
```

Added after the line `}` in `lessons/current-8/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/mod.rs:step-19b"
```

## Step 20 · src/engine/gpu/objects.rs
Update geometry bounds and display colors in the existing object row.

`lessons/current-9/src/engine/gpu/objects.rs` · edit · type this

Added after the line `}` in `lessons/current-8/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/objects.rs:step-20"
```

## Step 21 · src/engine/gpu/patch.rs
Count each upload table and record the ranges owned by an object.

`lessons/current-9/src/engine/gpu/patch.rs` · 64 lines · type this, new file

```rust
--8<-- "lessons/current-9/src/engine/gpu/patch.rs"
```

## Step 22 · src/engine/gpu/segments.rs
Patch boundary pipes and other stroke ranges in place.

`lessons/current-9/src/engine/gpu/segments.rs` · edit · type this

Added after the line `}` in `lessons/current-8/src/engine/gpu/segments.rs`

```rust
--8<-- "lessons/current-9/src/engine/gpu/segments.rs:step-22"
```

## Step 23 · src/shaders/glyph.wgsl
Apply object color overrides to marker colors.

`lessons/current-9/src/shaders/glyph.wgsl` · edit · type this

Replaces the line `var color = g.color * inst.color;` in `lessons/current-8/src/shaders/glyph.wgsl`

```wgsl
--8<-- "lessons/current-9/src/shaders/glyph.wgsl:step-23"
```

## Step 24 · src/shaders/ribbon.wgsl
Apply object color overrides to strokes.

`lessons/current-9/src/shaders/ribbon.wgsl` · edit · type this

Replaces the line `var color = unpack4x8unorm(seg.color) * inst.color;` in `lessons/current-8/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/current-9/src/shaders/ribbon.wgsl:step-24"
```

## Step 25 · src/shaders/scene.wgsl
Choose between the authored color and the object override using the instance flag.

`lessons/current-9/src/shaders/scene.wgsl` · edit · type this

Added after the line `const FLAG_SINGLE: u32 = 128u;` in `lessons/current-8/src/shaders/scene.wgsl`

```wgsl
--8<-- "lessons/current-9/src/shaders/scene.wgsl:step-25"
```

## Step 26 · src/shaders/sphere.wgsl
Apply object color overrides to sphere markers.

`lessons/current-9/src/shaders/sphere.wgsl` · edit · type this

Replaces the line `var color = g.color * inst.color;` in `lessons/current-8/src/shaders/sphere.wgsl`

```wgsl
--8<-- "lessons/current-9/src/shaders/sphere.wgsl:step-26"
```

## Step 27 · src/shaders/splat.wgsl
Apply object color overrides to cloud splats.

`lessons/current-9/src/shaders/splat.wgsl` · edit · type this

Added after the line `var rgba = unpack4x8unorm(colors[i]) * tint;` in `lessons/current-8/src/shaders/splat.wgsl`

```wgsl
--8<-- "lessons/current-9/src/shaders/splat.wgsl:step-27"
```

## Step 28 · src/shaders/triangle.wgsl
Apply object color overrides to triangle surfaces.

`lessons/current-9/src/shaders/triangle.wgsl` · edit · type this

Replaces the line `var color = in.color.rgb * inst.color.rgb;` in `lessons/current-8/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/current-9/src/shaders/triangle.wgsl:step-28"
```

## Step 29 · src/state.rs
Exclude locked rows from picking and clear stale preview state after scene changes.

`lessons/current-9/src/state.rs` · edit · type this

Added after the line `pub fn select(&mut self, row: Option<u32>) {` in `lessons/current-8/src/state.rs`

```rust
--8<-- "lessons/current-9/src/state.rs:step-29a"
```

Added after the line `fn apply_pick(&mut self, pick: Option<Pick>) {` in `lessons/current-8/src/state.rs`

```rust
--8<-- "lessons/current-9/src/state.rs:step-29b"
```

## Step 30 · src/state/edit.rs
Update cached surface samples during dragging and restore source rendering on cancellation.

`lessons/current-9/src/state/edit.rs` · edit · type this

Delete the 4 lines from `if active.target.is_some() {` in `lessons/current-8/src/state/edit.rs`.

Replaces the line `self.scene.rebuild(&mut self.gpu);` in `lessons/current-8/src/state/edit.rs`

```rust
--8<-- "lessons/current-9/src/state/edit.rs:step-30b"
```

Replaces the line `self.scene.rebuild(&mut self.gpu);` in `lessons/current-8/src/state/edit.rs`

```rust
--8<-- "lessons/current-9/src/state/edit.rs:step-30c"
```

Added after the line `Command::Model(command) => {` in `lessons/current-8/src/state/edit.rs`

```rust
--8<-- "lessons/current-9/src/state/edit.rs:step-30d"
```

Replaces the 9 lines from `let mut rows: Vec<crate::app::feedback::LayerRow> = layer…` in `lessons/current-8/src/state/edit.rs`

```rust
--8<-- "lessons/current-9/src/state/edit.rs:step-30e"
```

Added after the line `}` in `lessons/current-8/src/state/edit.rs`

```rust
--8<-- "lessons/current-9/src/state/edit.rs:step-30f"
```

## Step 31 · src/state/panel.rs
Route bulbs, locks and swatches through the selected tree node and its descendants.

`lessons/current-9/src/state/panel.rs` · edit · type this

Added after the line `self.toggle_layer(layer);` in `lessons/current-8/src/state/panel.rs`

```rust
--8<-- "lessons/current-9/src/state/panel.rs:step-31a"
```

Replaces the 5 lines from `if self` in `lessons/current-8/src/state/panel.rs`

```rust
--8<-- "lessons/current-9/src/state/panel.rs:step-31b"
```

Added after the line `self.select(Some(row));` in `lessons/current-8/src/state/panel.rs`

```rust
--8<-- "lessons/current-9/src/state/panel.rs:step-31c"
```

Replaces the 7 lines from `let indent = "  ".repeat(node.depth.min(16));` in `lessons/current-8/src/state/panel.rs`

```rust
--8<-- "lessons/current-9/src/state/panel.rs:step-31d"
```

Added after the line `hidden: false,` in `lessons/current-8/src/state/panel.rs`

```rust
--8<-- "lessons/current-9/src/state/panel.rs:step-31e"
```

Added after the line `hidden: false,` in `lessons/current-8/src/state/panel.rs`

```rust
--8<-- "lessons/current-9/src/state/panel.rs:step-31f"
```

## Check

Run `trunk serve` in `lessons/current-9/` and open <http://127.0.0.1:8770/>.

Expected: a point is visible after Fit, layer locks prevent selection, and a successful modeling command reports **geometry updated**.

![Full viewer result for current 9](screenshots/extensions-layers-desktop.png)

If it fails:

- The entire scene rebuilds while dragging: cached UV samples are not used.
- Locks vanish after Open: only visibility is serialized.

## What changed

```text
lessons/current-9/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
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
│   │   ├── points.rs  ~
│   │   └── sheet.rs
│   ├── cloud_query.rs
│   ├── command.rs  ~
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── deform.rs  ~
│   ├── edit.rs  ~
│   ├── feedback.rs  ~
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── hierarchy.rs  ~
│   ├── input.rs
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── session_io.rs  ~
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── surface_preview.rs  +
│   ├── touch.rs
│   ├── ui.rs  ~
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs
│   │   ├── buffers.rs  ~
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs  ~
│   │   ├── frame.rs
│   │   ├── glyphs.rs  ~
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── patch.rs  +
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs
│   │   ├── segments.rs  ~
│   │   ├── splat.rs
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── ui.rs
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
│   ├── glyph.wgsl  ~
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl  ~
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl  ~
│   ├── sphere.wgsl  ~
│   ├── splat.wgsl  ~
│   ├── splat_resolve.wgsl
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl  ~
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── panel.rs  ~
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: source surface samples → preview ranges → GPU patches; layer actions → retained settings.
Every file at this point: `lessons/current-9/`.

## Next

[Continue with current-10](current-10.md).

## Expected viewer result

The completed viewer has one right-hand layer tree. Bulbs control visibility, locks prevent selection, and swatches change object and child colors. The command dock spans the bottom and displays syntax hints. Source-shell dragging updates the existing preview throughout the gesture; Save/Open retains geometry, visibility, locks and colors.

[![Full viewer result for current 9](screenshots/extensions-layers-desktop.png)](screenshots/extensions-layers-desktop.png)
