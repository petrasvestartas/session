# current-6 · Build the egui panel and command interface

White egui windows display the command input and nested layer panel over the scene.

## Step 1 · Cargo.toml
Copy this file from the lesson folder to the path shown.

`lessons/current-6/Cargo.toml` · edit · copy the file

Added after the line `wgpu = "29.0"` in `lessons/current-5/Cargo.toml`

```toml
--8<-- "lessons/current-6/Cargo.toml:step-1a"
```

Delete the 2 lines from `"HtmlInputElement",` in `lessons/current-5/Cargo.toml`.

## Step 2 · index.html
Copy this file from the lesson folder to the path shown.

`lessons/current-6/index.html` · edit · copy the file

Added after the line `}` in `lessons/current-5/index.html`

```rust
--8<-- "lessons/current-6/index.html:step-2a"
```

Delete the 9 lines from `<!-- The layers panel: one element, filled from Rust with…` in `lessons/current-5/index.html`.

## Step 3 · src/app/feedback.rs
Carry status and layer information from the scene into the interface.

`lessons/current-6/src/app/feedback.rs` · edit · type this

Added after the line `}` in `lessons/current-5/src/app/feedback.rs`

```rust
--8<-- "lessons/current-6/src/app/feedback.rs:step-3a"
```

Replaces the 7 lines from `pub fn command_line(open: bool) -> Option<web_sys::HtmlIn…` in `lessons/current-5/src/app/feedback.rs`

```rust
--8<-- "lessons/current-6/src/app/feedback.rs:step-3b"
```

Added after the line `pub fn command_line(_open: bool) {}` in `lessons/current-5/src/app/feedback.rs`

```rust
--8<-- "lessons/current-6/src/app/feedback.rs:step-3c"
```

Replaces the 28 lines from `let Some(document) = web_sys::window().and_then(|w| w.doc…` in `lessons/current-5/src/app/feedback.rs`

```rust
--8<-- "lessons/current-6/src/app/feedback.rs:step-3d"
```

## Step 4 · src/app/input.rs
Send command-window visibility through the UI model and return Escape to scene input.

`lessons/current-6/src/app/input.rs` · edit · type this

Delete the 96 lines from `pub struct CommandKeys {` in `lessons/current-5/src/app/input.rs`.

## Step 5 · src/app/mod.rs
Declare the new application modules so their files join the crate.

`lessons/current-6/src/app/mod.rs` · edit · type this

Added after the line `pub mod touch;` in `lessons/current-5/src/app/mod.rs`

```rust
--8<-- "lessons/current-6/src/app/mod.rs:step-5"
```

## Step 6 · src/app/ui.rs
Build the command and layer windows, collect their actions, then apply them after the UI borrow ends.

`lessons/current-6/src/app/ui.rs` · 334 lines · type this, new file

```rust
--8<-- "lessons/current-6/src/app/ui.rs"
```

## Step 7 · src/engine/gpu/mod.rs
Add the new GPU resources, initialize them and include their allocations in the counters.

`lessons/current-6/src/engine/gpu/mod.rs` · edit · type this

Added after the line `mod triangle_tiles;` in `lessons/current-5/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/current-6/src/engine/gpu/mod.rs:step-7a"
```

Added after the line `pub widget: widget::Widget,` in `lessons/current-5/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/current-6/src/engine/gpu/mod.rs:step-7b"
```

Added after the line `widget,` in `lessons/current-5/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/current-6/src/engine/gpu/mod.rs:step-7d"
```

## Step 8 · src/engine/gpu/render.rs
Place the new drawing work into the frame sequence.

`lessons/current-6/src/engine/gpu/render.rs` · edit · type this

Added after the line `draws += self.widget.draw(encoder, view, &self.targets);` in `lessons/current-5/src/engine/gpu/render.rs`

```rust
--8<-- "lessons/current-6/src/engine/gpu/render.rs:step-8"
```

## Step 9 · src/engine/gpu/ui.rs
Upload egui textures and triangles, render their clipped ranges, and release requested textures.

`lessons/current-6/src/engine/gpu/ui.rs` · 103 lines · type this, new file

```rust
--8<-- "lessons/current-6/src/engine/gpu/ui.rs"
```

## Step 10 · src/lib.rs
Connect browser events, scene changes and drawing through the application state.

`lessons/current-6/src/lib.rs` · edit · type this

Delete the 2 lines from `Command(String),` in `lessons/current-5/src/lib.rs`.

Replaces the 2 lines from `command_keys: Option<app::input::CommandKeys>,` in `lessons/current-5/src/lib.rs`

```rust
--8<-- "lessons/current-6/src/lib.rs:step-10b"
```

Replaces the 2 lines from `command_keys: None,` in `lessons/current-5/src/lib.rs`

```rust
--8<-- "lessons/current-6/src/lib.rs:step-10c"
```

Added after the line `}` in `lessons/current-5/src/lib.rs`

```rust
--8<-- "lessons/current-6/src/lib.rs:step-10d"
```

Delete the 12 lines from `if let Some(input) = app::feedback::command_line(false) {` in `lessons/current-5/src/lib.rs`.

Delete the 10 lines from `Msg::Command(line) => {` in `lessons/current-5/src/lib.rs`.

Added after the line `let Some(state) = &mut self.state else { return };` in `lessons/current-5/src/lib.rs`

```rust
--8<-- "lessons/current-6/src/lib.rs:step-10g"
```

Added after the line `} else {` in `lessons/current-5/src/lib.rs`

```rust
--8<-- "lessons/current-6/src/lib.rs:step-10h"
```

## Step 11 · src/state.rs
Expose a frame request for interface changes.

`lessons/current-6/src/state.rs` · edit · type this

Added after the line `self.request_selection(x, y, false, false);` in `lessons/current-5/src/state.rs`

```rust
--8<-- "lessons/current-6/src/state.rs:step-11"
```

## Check

Run `trunk serve` in `lessons/current-6/` and open <http://127.0.0.1:8770/>.

Expected: a command runs in a white egui window and reports **geometry updated** above the input.

![Full viewer result for current 6](screenshots/extensions-command-create.png)

If it fails:

- Typing triggers scene shortcuts: egui input is also forwarded to the scene.
- A UI update causes a borrow panic: its action runs inside the model borrow.

## What changed

```text
lessons/current-6/src/
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
│   ├── hierarchy.rs
│   ├── input.rs  ~
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   ├── ui.rs  +
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
│   │   ├── mod.rs  ~
│   │   ├── objects.rs
│   │   ├── pick.rs
│   │   ├── present.rs
│   │   ├── render.rs  ~
│   │   ├── segments.rs
│   │   ├── splat.rs
│   │   ├── surface_outline.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── triangle_tiles.rs
│   │   ├── ui.rs  +
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
│   ├── edit.rs
│   ├── panel.rs
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs  ~
└── state.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: browser event → egui model → deferred action → GPU UI overlay.
Every file at this point: `lessons/current-6/`.

## Next

[Continue with current-7](current-7.md).

## Expected viewer result

The white egui command area shows a completed line command and its feedback, with the created geometry visible in the full viewer. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb). The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 6](screenshots/extensions-command-create.png)](screenshots/extensions-command-create.png)
