# 32 · Finish the command workspace and soft ambient lighting
A command field below the model controls grouped selection, responsive surface editing, independent colors and soft ambient shadows.

## Step 1 · index.html
Copy this file from the lesson folder to the path shown.

`lessons/32/index.html` · edit · copy the file

Replaces the line `background: #000;` in `lessons/31/index.html`

```html
--8<-- "lessons/32/index.html:step-1a"
```

Replaces the line `top: 0;` in `lessons/31/index.html`

```html
--8<-- "lessons/32/index.html:step-1b"
```

Replaces the line `<canvas id="canvas" tabindex="0" role="application" aria-…` in `lessons/31/index.html`

```html
--8<-- "lessons/32/index.html:step-1c"
```

## Step 2 · src/app/deform.rs
Expose mesh vertex keys and retain shared-vertex behavior for component moves.

`lessons/32/src/app/deform.rs` · edit · type this

Replaces the line `fn mesh_keys(mesh: &Mesh, target: Target) -> Result<Vec<u…` in `lessons/31/src/app/deform.rs`

```rust
--8<-- "lessons/32/src/app/deform.rs:step-2a"
```

Replaces the 2 lines from `mesh.triangulation.clear();` in `lessons/31/src/app/deform.rs`

```rust
--8<-- "lessons/32/src/app/deform.rs:step-2b"
```

## Step 3 · src/app/feedback.rs
Carry status and layer information from the scene into the interface.

`lessons/32/src/app/feedback.rs` · edit · type this

Replaces the line `#[derive(Clone, Default)]` in `lessons/31/src/app/feedback.rs`

```rust
--8<-- "lessons/32/src/app/feedback.rs:step-3"
```

## Step 4 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data.

`lessons/32/src/app/inspection.rs` · edit · type this

Added after the line `});` in `lessons/31/src/app/inspection.rs`

```rust
--8<-- "lessons/32/src/app/inspection.rs:step-4"
```

## Step 5 · src/app/mesh_preview.rs
Cache source-to-render vertex mappings and update the affected neighborhood during a drag.

`lessons/32/src/app/mesh_preview.rs` · 319 lines · type this, new file

```rust
--8<-- "lessons/32/src/app/mesh_preview.rs"
```

## Step 6 · src/app/mod.rs
Declare the new application modules so their files join the crate.

`lessons/32/src/app/mod.rs` · edit · type this

Added after the line `pub mod manifest;` in `lessons/31/src/app/mod.rs`

```rust
--8<-- "lessons/32/src/app/mod.rs:step-6"
```

## Step 7 · src/app/scene.rs
Retain mesh preview caches and separate face and edge overrides by source identity.

`lessons/32/src/app/scene.rs` · edit · type this

Added after the line `pub colors: HashMap<(usize, Rc<str>), [u8; 3]>,` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7a"
```

Added after the line `surface_previews: Vec<Option<crate::app::surface_preview:…` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7b"
```

Added after the line `colors: HashMap::new(),` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7c"
```

Added after the line `surface_previews: Vec::new(),` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7d"
```

Added after the line `self.colors.clear();` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7e"
```

Added after the line `self.surface_previews.clear();` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7f"
```

Added after the line `}` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7g"
```

Added after the line `self.surface_previews.push(None);` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7h"
```

Added after the line `);` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7i"
```

Added after the line `}` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7j"
```

Added after the line `.sum::<usize>()` in `lessons/31/src/app/scene.rs`

```rust
--8<-- "lessons/32/src/app/scene.rs:step-7k"
```

## Step 8 · src/app/session_io.rs
Save and restore the two independent display color channels.

`lessons/32/src/app/session_io.rs` · edit · type this

Added after the line `colors: Vec<(usize, String, [u8; 3])>,` in `lessons/31/src/app/session_io.rs`

```rust
--8<-- "lessons/32/src/app/session_io.rs:step-8a"
```

Added after the line `colors,` in `lessons/31/src/app/session_io.rs`

```rust
--8<-- "lessons/32/src/app/session_io.rs:step-8b"
```

Added after the line `.map(|(doc, id)| (doc, Rc::from(id)))` in `lessons/31/src/app/session_io.rs`

```rust
--8<-- "lessons/32/src/app/session_io.rs:step-8c"
```

Added after the line `.insert(scene.identity_of(1).unwrap(), [240, 80, 30]);` in `lessons/31/src/app/session_io.rs`

```rust
--8<-- "lessons/32/src/app/session_io.rs:step-8d"
```

Added after the line `assert_eq!(restored.colors, scene.colors);` in `lessons/31/src/app/session_io.rs`

```rust
--8<-- "lessons/32/src/app/session_io.rs:step-8e"
```

## Step 9 · src/app/splitting.rs
Carry face and edge overrides onto split results.

`lessons/32/src/app/splitting.rs` · edit · type this

Added after the line `let color = self.colors.get(&(doc, Rc::clone(&guid))).cop…` in `lessons/31/src/app/splitting.rs`

```rust
--8<-- "lessons/32/src/app/splitting.rs:step-9a"
```

Replaces the line `self.colors.insert((doc, Rc::from(id)), color);` in `lessons/31/src/app/splitting.rs`

```rust
--8<-- "lessons/32/src/app/splitting.rs:step-9b"
```

## Step 10 · src/app/ui.rs
Place the command input below its output window with full-width dividers and inline options.

`lessons/32/src/app/ui.rs` · edit · type this

Added after the line `use winit::window::Window;` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10a"
```

Replaces the 4 lines from `pub fn new(window: &Window, logical_width: f64) -> Self {` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10b"
```

Added after the line `let mut consumed = response.consumed;` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10c"
```

Replaces the line `WindowEvent::MouseWheel { .. } => consumed = !self.scene_…` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10d"
```

Replaces the line `self.ui_drag = !self.scene_rect.contains(self.pointer);` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10e"
```

Added after the line `}` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10f"
```

Replaces the 2 lines from `let mut tool = None;` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10g"
```

Replaces the line `if model.history.len() == 8 {` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10h"
```

Replaces the line `let snapshot = MODEL.with_borrow(|model| serde_json::json…` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10i"
```

Replaces the line `visuals.panel_fill = egui::Color32::from_gray(247);` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10j"
```

Replaces the line `widget.bg_fill = egui::Color32::WHITE;` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10k"
```

Replaces the 2 lines from `let width = (root.available_width() * 0.25).clamp(180.0, …` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10l"
```

Added after the line `let mut color = row.color.unwrap_or([180, 180, 180]);` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10m"
```

Replaces the line `let height = if model.command_open { 160.0 } else { 104.0 };` in `lessons/31/src/app/ui.rs`

```rust
--8<-- "lessons/32/src/app/ui.rs:step-10n"
```

## Step 11 · src/engine/gpu/glyphs.rs
Update an individual control marker during a mesh preview.

`lessons/32/src/engine/gpu/glyphs.rs` · edit · type this

Added after the line `impl GlyphLane {` in `lessons/31/src/engine/gpu/glyphs.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/glyphs.rs:step-11"
```

## Step 12 · src/engine/gpu/instance.rs
Store the separate edge color and its override flag in the GPU instance data.

`lessons/32/src/engine/gpu/instance.rs` · edit · type this

Added after the line `pub const FLAG_COLOR: u32 = 1 << 8;` in `lessons/31/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/instance.rs:step-12a"
```

Replaces the line `let rust = ["model", "color", "flags", "_pad0", "spacing"];` in `lessons/31/src/engine/gpu/instance.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/instance.rs:step-12b"
```

## Step 13 · src/engine/gpu/mod.rs
Own optional ambient resources and report their buffer and texture sizes.

`lessons/32/src/engine/gpu/mod.rs` · edit · type this

Added after the line `pub mod splat;` in `lessons/31/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/mod.rs:step-13a"
```

Added after the line `pub backdrop: BackdropLane,` in `lessons/31/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/mod.rs:step-13b"
```

Replaces the line `+ outline_buffers;` in `lessons/31/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/mod.rs:step-13c"
```

Replaces the line `+ outline_textures,` in `lessons/31/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/mod.rs:step-13d"
```

Added after the line `backdrop,` in `lessons/31/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/mod.rs:step-13e"
```

Replaces the 2 lines from `pub fn set_object_color(&mut self, row: u32, color: [u8; …` in `lessons/31/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/mod.rs:step-13f"
```

## Step 14 · src/engine/gpu/objects.rs
Update either color channel and advance the geometry revision after a preview change.

`lessons/32/src/engine/gpu/objects.rs` · edit · type this

Added after the line `pub color: [f32; 4],` in `lessons/31/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/objects.rs:step-14a"
```

Added after the line `color: [1.0; 4],` in `lessons/31/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/objects.rs:step-14b"
```

Replaces the line `_pad: 0,` in `lessons/31/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/objects.rs:step-14c"
```

Replaces the line `pub fn set_color(&mut self, ctx: &GpuCtx, row: u32, color…` in `lessons/31/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/objects.rs:step-14d"
```

Added after the line `self.buffer.write_at(ctx, row, std::slice::from_ref(r));` in `lessons/31/src/engine/gpu/objects.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/objects.rs:step-14e"
```

## Step 15 · src/engine/gpu/splat.rs
Invalidate cached point-cloud pixels when object geometry or placement changes.

`lessons/32/src/engine/gpu/splat.rs` · edit · type this

Added after the line `point_count: u32,` in `lessons/31/src/engine/gpu/splat.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/splat.rs:step-15a"
```

Added after the line `point_count,` in `lessons/31/src/engine/gpu/splat.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/splat.rs:step-15b"
```

## Step 16 · src/shaders/glyph.wgsl
Use the correct authored or overridden marker color.

`lessons/32/src/shaders/glyph.wgsl` · edit · type this

Replaces the line `var color = object_color(g.color, inst);` in `lessons/31/src/shaders/glyph.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/glyph.wgsl:step-16"
```

## Step 17 · src/shaders/ribbon.wgsl
Resolve independent edge colors and keep 3D stroke widths constant on screen.

`lessons/32/src/shaders/ribbon.wgsl` · edit · type this

Delete the two `const WIRE_MIN_PENS` and `const TAPER_MIN` lines and the comment above them from `lessons/31/src/shaders/ribbon.wgsl`.

Added after the line `fn half_width_px(radius: f32, w: f32) -> f32 {` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17b"
```

Delete the `fn density_taper` block from `lessons/31/src/shaders/ribbon.wgsl`.

Replaces the lines from `let cad_boundary = (inst.flags & FLAG_SMOOTH) != 0u && so…` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17d"
```

Replaces the 2 lines from `o.hw0 = raw0 * crowd;` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17e"
```

Replaces the line `if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17f"
```

Replaces the line `if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17g"
```

Replaces the line `if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17h"
```

Replaces the 2 lines from `if (alpha <= 0.0 || !ink_visible(in.pos.xy, ink_axis(in),…` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17i"
```

Replaces the line `if (coverage(in) <= 0.0 || !ink_visible(in.pos.xy, ink_ax…` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17j"
```

Replaces the line `if (in.source_edge == 0xffffffffu || coverage(in) <= 0.0 …` in `lessons/31/src/shaders/ribbon.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ribbon.wgsl:step-17k"
```

## Step 18 · src/shaders/scene.wgsl
Resolve face and edge colors independently from their override flags.

`lessons/32/src/shaders/scene.wgsl` · edit · type this

Added after the line `spacing: f32,` in `lessons/31/src/shaders/scene.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/scene.wgsl:step-18a"
```

Added after the line `}` in `lessons/31/src/shaders/scene.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/scene.wgsl:step-18b"
```

## Step 19 · src/shaders/sphere.wgsl
Keep marker colors consistent with the new channel flags.

`lessons/32/src/shaders/sphere.wgsl` · edit · type this

Replaces the line `var color = object_color(g.color, inst);` in `lessons/31/src/shaders/sphere.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/sphere.wgsl:step-19"
```

## Step 20 · src/state/edit.rs
Preview all selected placements together and commit once on release.

`lessons/32/src/state/edit.rs` · edit · type this

Replaces the line `base_local: Xform,` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20a"
```

Replaces the line `let mut origin = box_.center();` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20b"
```

Added after the line `};` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20c"
```

Added after the line `};` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20d"
```

Added after the line `let local = &(&back * &delta) * &place;` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20e"
```

Replaces the line `let place = &delta * &active.base_place;` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20f"
```

Replaces the line `self.gpu.grew_bounds(active.row);` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20g"
```

Replaces the 5 lines from `if let Some(place) = self.scene.set_row_xform(active.row,…` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20h"
```

Replaces the line `if active.target.is_some() {` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20i"
```

Added after the line `}` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20j"
```

Replaces the line `fn pixel_scale(&self) -> f64 {` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20k"
```

Added after the line `self.cancel_gesture();` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20l"
```

Added after the line `match command {` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20m"
```

Replaces the 3 lines from `let Some(place) = self.scene.transform_row(row, &delta, l…` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20n"
```

Added after the line `let index = active.index;` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20o"
```

Added after the line `};` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20p"
```

Added after the line `let free = active.plane.hit(&active.origin, &from, &dir)?;` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20q"
```

Replaces the line `fn project(&self, at: [f64; 3]) -> Option<(f64, f64)> {` in `lessons/31/src/state/edit.rs`

```rust
--8<-- "lessons/32/src/state/edit.rs:step-20r"
```

## Step 21 · src/state/panel.rs
Apply selection and locking recursively through the layer hierarchy.

`lessons/32/src/state/panel.rs` · edit · type this

Replaces the 2 lines from `if let Some((index, hex)) = value.split_once('/')` in `lessons/31/src/state/panel.rs`

```rust
--8<-- "lessons/32/src/state/panel.rs:step-21a"
```

Replaces the line `"select" => {` in `lessons/31/src/state/panel.rs`

```rust
--8<-- "lessons/32/src/state/panel.rs:step-21b"
```

Replaces the 13 lines from `self.select(None);` in `lessons/31/src/state/panel.rs`

```rust
--8<-- "lessons/32/src/state/panel.rs:step-21c"
```

Added after the line `locked,` in `lessons/31/src/state/panel.rs`

```rust
--8<-- "lessons/32/src/state/panel.rs:step-21d"
```

## Step 22 · session_rust/src/color.rs
Use very light grey for default geometry colors throughout the kernel.

`session_rust/src/color.rs` · edit · type this

The kernel's `Color::lightgrey` is now 0.94 grey instead of 0.9; this is the live kernel, already changed:

```rust
--8<-- "session_rust/src/color.rs:171:173"
```

The kernel's default colour is the same 0.94 grey:

```rust
--8<-- "session_rust/src/color.rs:365:367"
```

## Step 23 · src/app/command.rs
Accept a partial command with Enter, then accept its default or arrow-selected option.

`lessons/32/src/app/command.rs` · edit · type this

Added after the line `Fit,` in `lessons/31/src/app/command.rs`

```rust
--8<-- "lessons/32/src/app/command.rs:step-23a"
```

Replaces the 2 lines from `"point" => "Point x,y,z · Example: Point 0,0,0 · Enter cr…` in `lessons/31/src/app/command.rs`

```rust
--8<-- "lessons/32/src/app/command.rs:step-23b"
```

Replaces the line `_ => "Try Point 0,0,0 · Line 0,0,0 100,0,0 · Fit · Undo ·…` in `lessons/31/src/app/command.rs`

```rust
--8<-- "lessons/32/src/app/command.rs:step-23c"
```

Added after the line `match verb.as_str() {` in `lessons/31/src/app/command.rs`

```rust
--8<-- "lessons/32/src/app/command.rs:step-23d"
```

Added after the line `other => Err(format!("no command '{other}'")),` in `lessons/31/src/app/command.rs`

```rust
--8<-- "lessons/32/src/app/command.rs:step-23e"
```

Added after the line `use super::*;` in `lessons/31/src/app/command.rs`

```rust
--8<-- "lessons/32/src/app/command.rs:step-23f"
```

## Step 24 · src/app/edit.rs
Collect selected descendants and transform the selection as one set.

`lessons/32/src/app/edit.rs` · edit · type this

Added after the line `impl Scene {` in `lessons/31/src/app/edit.rs`

```rust
--8<-- "lessons/32/src/app/edit.rs:step-24a"
```

Added after the line `use session_rust::{Point, Session};` in `lessons/31/src/app/edit.rs`

```rust
--8<-- "lessons/32/src/app/edit.rs:step-24b"
```

## Step 25 · src/app/gizmo.rs
Measure the distance from the pointer ray to each handle.

`lessons/32/src/app/gizmo.rs` · edit · type this

Replaces the line `let (_, t) = line_line_parameters(&ray, &line, 1e-9, fals…` in `lessons/31/src/app/gizmo.rs`

```rust
--8<-- "lessons/32/src/app/gizmo.rs:step-25a"
```

Added after the line `use super::*;` in `lessons/31/src/app/gizmo.rs`

```rust
--8<-- "lessons/32/src/app/gizmo.rs:step-25b"
```

## Step 26 · src/app/input.rs
Route pending drawing clicks before selection; use Shift to add objects and G to toggle Arctic lighting.

`lessons/32/src/app/input.rs` · edit · type this

Replaces the 2 lines from `Key::Named(NamedKey::Escape) => state.escape_selection(),` in `lessons/31/src/app/input.rs`

```rust
--8<-- "lessons/32/src/app/input.rs:step-26a"
```

Added after the line `}` in `lessons/31/src/app/input.rs`

```rust
--8<-- "lessons/32/src/app/input.rs:step-26b"
```

Replaces the line `dragging || state.hover_gizmo(position.x, position.y)` in `lessons/31/src/app/input.rs`

```rust
--8<-- "lessons/32/src/app/input.rs:step-26c"
```

Replaces the line `if !self.ctrl && state.begin_control_drag(self.last_curso…` in `lessons/31/src/app/input.rs`

```rust
--8<-- "lessons/32/src/app/input.rs:step-26d"
```

Added after the line `}` in `lessons/31/src/app/input.rs`

```rust
--8<-- "lessons/32/src/app/input.rs:step-26e"
```

## Step 27 · src/app/surface_preview.rs
Reuse the sampled surface grid and update its positions during a gesture.

`lessons/32/src/app/surface_preview.rs` · edit · type this

Replaces the 7 lines from `let pipe_vertices = pipes` in `lessons/31/src/app/surface_preview.rs`

```rust
--8<-- "lessons/32/src/app/surface_preview.rs:step-27a"
```

Added after the line `let b = vertices[normals[1]].normal.map(f64::from);` in `lessons/31/src/app/surface_preview.rs`

```rust
--8<-- "lessons/32/src/app/surface_preview.rs:step-27b"
```

Added after the line `use std::rc::Rc;` in `lessons/31/src/app/surface_preview.rs`

```rust
--8<-- "lessons/32/src/app/surface_preview.rs:step-27c"
```

## Step 28 · src/app/walk/brep.rs
Sample a bounded grid for standalone NURBS surfaces and build their natural boundary curves once.

`lessons/32/src/app/walk/brep.rs` · edit · type this

Replaces the line `use super::mesh::{MeshCx, MeshOpts, mesh_spacing, walk_me…` in `lessons/31/src/app/walk/brep.rs`

```rust
--8<-- "lessons/32/src/app/walk/brep.rs:step-28a"
```

Added after the line `let mut verts = 0;` in `lessons/31/src/app/walk/brep.rs`

```rust
--8<-- "lessons/32/src/app/walk/brep.rs:step-28b"
```

Added after the line `}` in `lessons/31/src/app/walk/brep.rs`

```rust
--8<-- "lessons/32/src/app/walk/brep.rs:step-28c"
```

Replaces the line `walk_brep_edges(ink, b, &chains, (&ep, &mut row.bounds));` in `lessons/31/src/app/walk/brep.rs`

```rust
--8<-- "lessons/32/src/app/walk/brep.rs:step-28d"
```

Added after the line `out: (&EdgePen, &mut AABB),` in `lessons/31/src/app/walk/brep.rs`

```rust
--8<-- "lessons/32/src/app/walk/brep.rs:step-28e"
```

Replaces the lines from `let mut sm = if let Some(mesh) = &s.m_mesh {` in `lessons/31/src/app/walk/brep.rs`

```rust
--8<-- "lessons/32/src/app/walk/brep.rs:step-28f"
```

Added after the line `(arena, seg, glyph, row)` in `lessons/31/src/app/walk/brep.rs`

```rust
--8<-- "lessons/32/src/app/walk/brep.rs:step-28g"
```

## Step 29 · src/app/walk/curves.rs
Remove consecutive duplicate polyline points before constructing strokes.

`lessons/32/src/app/walk/curves.rs` · edit · type this

Added after the line `for w in pts.windows(2) {` in `lessons/31/src/app/walk/curves.rs`

```rust
--8<-- "lessons/32/src/app/walk/curves.rs:step-29"
```

## Step 30 · src/app/walk/encode.rs
Encode ordinary 3D pen widths as screen pixels.

`lessons/32/src/app/walk/encode.rs` · edit · type this

Replaces the lines from `(w as f32) * 0.5` in `lessons/31/src/app/walk/encode.rs`

```rust
--8<-- "lessons/32/src/app/walk/encode.rs:step-30"
```

## Step 31 · src/engine/gpu/frame.rs
Select the soft hemisphere shader when ambient lighting is enabled.

`lessons/32/src/engine/gpu/frame.rs` · edit · type this

Replaces the line `lit: f32::from(cx.view.lit),` in `lessons/31/src/engine/gpu/frame.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/frame.rs:step-31"
```

## Step 32 · src/engine/gpu/render.rs
Shade faces, then apply ambient contact shadows before drawing ink.

`lessons/32/src/engine/gpu/render.rs` · edit · type this

Added after the line `};` in `lessons/31/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/render.rs:step-32a"
```

Replaces the line `let mut draws = self.backdrop.draw_background(pass);` in `lessons/31/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/render.rs:step-32b"
```

## Step 33 · src/engine/gpu/ssao.rs
Cache hemisphere occlusion in two R8 textures at half resolution, capped at 960 pixels on its longest side.

`lessons/32/src/engine/gpu/ssao.rs` · 459 lines · type this, new file

```rust
--8<-- "lessons/32/src/engine/gpu/ssao.rs"
```

## Step 34 · src/engine/gpu/view.rs
Start with ambient lighting disabled.

`lessons/32/src/engine/gpu/view.rs` · edit · type this

Added after the line `pub struct View {` in `lessons/31/src/engine/gpu/view.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/view.rs:step-34a"
```

Added after the line `Self {` in `lessons/31/src/engine/gpu/view.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/view.rs:step-34b"
```

## Step 35 · src/shaders/background.wgsl
Use a white background normally and very light grey with Arctic enabled.

`lessons/32/src/shaders/background.wgsl` · edit · type this

Replaces the line `return PhysicalColor(vec4<f32>(1.0, 1.0, 1.0, 1.0), vec4<…` in `lessons/31/src/shaders/background.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/background.wgsl:step-35"
```

## Step 36 · src/shaders/ssao.wgsl
Reconstruct positions from depth and sample surface occlusion with slightly stronger shading.

`lessons/32/src/shaders/ssao.wgsl` · 176 lines · type this, new file

```wgsl
--8<-- "lessons/32/src/shaders/ssao.wgsl"
```

## Step 37 · src/shaders/triangle.wgsl
Apply the archive viewer’s soft sky and ground shading while preserving authored colors.

`lessons/32/src/shaders/triangle.wgsl` · edit · type this

Replaces the line `let shaded = select(1.0, lit, line.lit > 0.5 && in.print …` in `lessons/31/src/shaders/triangle.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/triangle.wgsl:step-37"
```

## Step 38 · src/state.rs
Retain the selected object set and place one gumball around its combined bounds.

`lessons/32/src/state.rs` · edit · type this

Added after the line `mod cloud_query;` in `lessons/31/src/state.rs`

```rust
--8<-- "lessons/32/src/state.rs:step-38a"
```

Added after the line `pending_split: Option<splitting::Pending>,` in `lessons/31/src/state.rs`

```rust
--8<-- "lessons/32/src/state.rs:step-38b"
```

Added after the line `pending_split: None,` in `lessons/31/src/state.rs`

```rust
--8<-- "lessons/32/src/state.rs:step-38c"
```

Replaces the lines from `let row = match self.scene.selected {` in `lessons/31/src/state.rs`

```rust
--8<-- "lessons/32/src/state.rs:step-38d"
```

Added after the line `self.scene.selected = row;` in `lessons/31/src/state.rs`

```rust
--8<-- "lessons/32/src/state.rs:step-38e"
```

Added after the line `log::info!("pick: nothing");` in `lessons/31/src/state.rs`

```rust
--8<-- "lessons/32/src/state.rs:step-38f"
```

Replaces the 6 lines from `let toggle = if self.scene.selected == Some(hit.row) {` in `lessons/31/src/state.rs`

```rust
--8<-- "lessons/32/src/state.rs:step-38g"
```

## Step 39 · src/engine/gpu/backdrop.rs
Bind the shared lighting uniform before drawing the background.

`lessons/32/src/engine/gpu/backdrop.rs` · edit · type this

Replaces the 3 lines from `use crate::engine::pipelines::{` in `lessons/31/src/engine/gpu/backdrop.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/backdrop.rs:step-39a"
```

Replaces the line `let background_shader = module(` in `lessons/31/src/engine/gpu/backdrop.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/backdrop.rs:step-39b"
```

Replaces the line `let background = build_background(ctx, &background_shader…` in `lessons/31/src/engine/gpu/backdrop.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/backdrop.rs:step-39c"
```

Replaces the line `self.background = build_background(ctx, &self.background_…` in `lessons/31/src/engine/gpu/backdrop.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/backdrop.rs:step-39d"
```

Added after the line `ctx: &GpuCtx,` in `lessons/31/src/engine/gpu/backdrop.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/backdrop.rs:step-39e"
```

## Step 40 · src/state/drawing.rs
Collect clicked or typed points, preview the next span and snap to existing geometry.

`lessons/32/src/state/drawing.rs` · 289 lines · type this, new file

```rust
--8<-- "lessons/32/src/state/drawing.rs"
```

## Step 41 · src/engine/gpu/arena.rs
Retain exact source indices for boundary endpoints until the preview cache captures them.

`lessons/32/src/engine/gpu/arena.rs` · edit · type this

Added after the line `pub face_sources: Vec<super::faces::FaceSource>,` in `lessons/31/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/arena.rs:step-41a"
```

Added after the line `drop_rows(&mut self.surface_samples);` in `lessons/31/src/engine/gpu/arena.rs`

```rust
--8<-- "lessons/32/src/engine/gpu/arena.rs:step-41b"
```

## Step 42 · src/shaders/ink_visibility.wgsl
Test a NURBS boundary against triangles at its actual projected position.

`lessons/32/src/shaders/ink_visibility.wgsl` · edit · type this

Replaces the 2 lines from `fn ink_visible(pixel: vec2<f32>, axis: InkAxis, sample: u…` in `lessons/31/src/shaders/ink_visibility.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ink_visibility.wgsl:step-42a"
```

Replaces the 3 lines from `if (fringe==0u) {` in `lessons/31/src/shaders/ink_visibility.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ink_visibility.wgsl:step-42b"
```

Added after the line `}` in `lessons/31/src/shaders/ink_visibility.wgsl`

```wgsl
--8<-- "lessons/32/src/shaders/ink_visibility.wgsl:step-42c"
```

## Step 43 · supplied files
Copy each file from the lesson folder to the path shown.

Copy from `lessons/32/` (tooling this checkpoint needs but the course does not teach):

- `session_cpp/src/color.cpp` (in the kernel checkout, not in the lesson folder)
- `session_cpp/src/color.h` (in the kernel checkout, not in the lesson folder)
- `session_cpp/src/color_test.cpp` (in the kernel checkout, not in the lesson folder)
- `session_py/src/session_py/color.py` (in the kernel checkout, not in the lesson folder)
- `session_py/src/session_py/color_test.py` (in the kernel checkout, not in the lesson folder)
- `session_rust/src/color_test.rs` (in the kernel checkout, not in the lesson folder)
- `lessons/32/tests/ambient-lighting.cjs`
- `lessons/32/tests/command-workspace.cjs`
- `lessons/32/tests/teapot.cjs`

## Check

Run `trunk serve` in `lessons/32/` and open <http://127.0.0.1:8770/>.

Expected: the output window sits above Command, options stay inline, and Arctic On adds concentrated ground shadows with **SSAO On** in the history.
![The completed command workspace with soft ambient lighting](screenshots/current-workspace.png)

If it fails:

- Typed letters duplicate: a sizing pass processes the same input events twice.
- Suggestions disappear below the window: the popup opens downward instead of above the command field.
- Surface edits show mesh diagonals: the display derives boundaries from tessellation.
- Shadows remain after G turns them off: the optional texture or lighting uniform stays enabled.
## What changed

```text
lessons/32/src/
├── app/
│   ├── inspection/
│   │   └── source_memory.rs
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs  ~
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs
│   │   ├── curves.rs  ~
│   │   ├── encode.rs  ~
│   │   ├── frames.rs
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs
│   │   ├── points.rs
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
│   ├── gizmo.rs  ~
│   ├── hierarchy.rs
│   ├── input.rs  ~
│   ├── inspection.rs  ~
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mesh_preview.rs  +
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs  ~
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── session_io.rs  ~
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── splitting.rs  ~
│   ├── stream.rs
│   ├── surface_preview.rs  ~
│   ├── touch.rs
│   ├── ui.rs  ~
│   └── validate.rs
├── engine/
│   ├── gpu/
│   │   ├── arena.rs  ~
│   │   ├── backdrop.rs  ~
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs
│   │   ├── faces.rs
│   │   ├── frame.rs  ~
│   │   ├── glyphs.rs  ~
│   │   ├── instance.rs  ~
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs  ~
│   │   ├── patch.rs
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs
│   │   ├── splat.rs  ~
│   │   ├── ssao.rs  +
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── ui.rs
│   │   ├── upload.rs
│   │   ├── view.rs  ~
│   │   ├── widget.rs
│   │   └── widget_mesh.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl  ~
│   ├── glyph.wgsl  ~
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl  ~
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── project_triangles.wgsl
│   ├── projected_triangle.wgsl
│   ├── ribbon.wgsl  ~
│   ├── scan_triangle_tiles.wgsl
│   ├── scene.wgsl  ~
│   ├── sphere.wgsl  ~
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── ssao.wgsl  +
│   ├── surface_outline.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   ├── triangle.wgsl  ~
│   ├── triangle_tiles.wgsl
│   └── widget.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── drawing.rs  +
│   ├── edit.rs  ~
│   ├── panel.rs  ~
│   ├── sheet_query.rs
│   ├── splitting.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: command or layer selection → shared selection → cached previews; scene depth → small occlusion texture → smooth contact shadows.
Every file at this point: `lessons/32/`.
## Next
[33 · Contact shadows that follow object size](33-contact-shadows.md)
## Expected viewer result
The command field sits below the viewport with a visible caret, inline completion, a scrollable command list and clickable options. Layers start hidden; Layers On reveals selection highlights and recursive layer controls. Point, Line, Polyline and Curve accept clicks or coordinates with Snap On by default. Shift selects multiple objects, one gumball moves them together, and G toggles Arctic shading and ground shadows. The background is white when Arctic is off. NURBS surfaces display only their true boundaries while editing. Face and edge colors remain independent.

[![The completed command workspace](screenshots/current-workspace.png)](screenshots/current-workspace.png)
