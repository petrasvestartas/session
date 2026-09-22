# current-7 · Finish the shared editing wiring

The floating interface, nested panel and control editing work together before the workspace becomes docked.

## Step 1 · src/app/edit.rs
Refuse unsupported controls and display-only documents, then cover both cases with tests.

`lessons/current-7/src/app/edit.rs` · edit · type this

Added after the line `pub fn delete_row(&mut self, row: u32) -> bool {` in `lessons/current-6/src/app/edit.rs`

```rust
--8<-- "lessons/current-7/src/app/edit.rs:step-1a"
```

Delete the 15 lines from `fn a_kind_with_no_control_points_is_refused() {` in `lessons/current-6/src/app/edit.rs`.

Added after the line `}` in `lessons/current-6/src/app/edit.rs`

```rust
--8<-- "lessons/current-7/src/app/edit.rs:step-1c"
```

## Step 2 · src/app/inspection.rs
Expose the new selection and resource state to the browser inspection data.

`lessons/current-7/src/app/inspection.rs` · edit · type this

Added after the line `"draw_calls": state.gpu.performance.draws,` in `lessons/current-6/src/app/inspection.rs`

```rust
--8<-- "lessons/current-7/src/app/inspection.rs:step-2a"
```

Added after the line `"gpu_texture_estimate_bytes": textures,` in `lessons/current-6/src/app/inspection.rs`

```rust
--8<-- "lessons/current-7/src/app/inspection.rs:step-2b"
```

## Step 3 · src/app/mod.rs
Declare the new application modules so their files join the crate.

`lessons/current-7/src/app/mod.rs` · edit · type this

Delete the 2 lines from `#[cfg(target_arch = "wasm32")]` in `lessons/current-6/src/app/mod.rs`.

Added after the line `pub mod inspection;` in `lessons/current-6/src/app/mod.rs`

```rust
--8<-- "lessons/current-7/src/app/mod.rs:step-3b"
```

## Step 4 · src/state/edit.rs
Reconnect command visibility and preserve streamed-source guards around Undo and Redo.

`lessons/current-7/src/state/edit.rs` · edit · type this

Added after the line `pub fn undo(&mut self) {` in `lessons/current-6/src/state/edit.rs`

```rust
--8<-- "lessons/current-7/src/state/edit.rs:step-5a"
```

Added after the line `pub fn redo(&mut self) {` in `lessons/current-6/src/state/edit.rs`

```rust
--8<-- "lessons/current-7/src/state/edit.rs:step-5b"
```

## Check

Run `trunk serve` in `lessons/current-7/` and open <http://127.0.0.1:8770/>.

Expected: the egui windows and placed control edits work together, and a successful geometry command reports **geometry updated**.

![Full viewer result for current 7](screenshots/extensions-command-create.png)

If it fails:

- Undo loses streamed data: a history rebuild accepts an incomplete source.
- A new helper is unresolved: its module declaration is missing.

## What changed

```text
lessons/current-7/src/
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
│   ├── edit.rs  ~
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── hierarchy.rs
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
│   ├── scene.rs
│   ├── scene_text.rs
│   ├── selection.rs
│   ├── sheet_query.rs
│   ├── snap.rs
│   ├── stream.rs
│   ├── touch.rs
│   ├── ui.rs
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
│   ├── panel.rs
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs
```

`+` new in this lesson · `~` changed in this lesson

Data flow: UI action → shared edit helpers → source history → refreshed display.
Every file at this point: `lessons/current-7/`.

## Next

[Continue with current-8](current-8.md).

## Expected viewer result

Checkpoint 7 completes the original floating-window interface. The next chapter adds the docked workspace, source subobject edits, touch gumball and Save/Open. This is a maintained-viewer reference; its bottom command dock and toolbar are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 7](screenshots/extensions-command-create.png)](screenshots/extensions-command-create.png)
