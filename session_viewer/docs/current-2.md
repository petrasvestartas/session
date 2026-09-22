# current-2 · Create, trim, extend and explode

Typed commands create points and curves, then trim, extend or explode the selected source geometry.

## Step 1 · src/app/command.rs
Add the modeling command variant, parse its verbs and validate their arguments before dispatch.

`lessons/current-2/src/app/command.rs` · edit · type this

Replaces the line `#[derive(Clone, Copy, Debug, PartialEq)]` in `lessons/current-1/src/app/command.rs`

```rust
--8<-- "lessons/current-2/src/app/command.rs:step-1a"
```

Added after the line `pub fn parse(line: &str) -> Result<Command, String> {` in `lessons/current-1/src/app/command.rs`

```rust
--8<-- "lessons/current-2/src/app/command.rs:step-1b"
```

Added after the line `let rest: Vec<&str> = words.collect();` in `lessons/current-1/src/app/command.rs`

```rust
--8<-- "lessons/current-2/src/app/command.rs:step-1c"
```

Added after the line `#[test]` in `lessons/current-1/src/app/command.rs`

```rust
--8<-- "lessons/current-2/src/app/command.rs:step-1d"
```

Added after the line `}` in `lessons/current-1/src/app/command.rs`

```rust
--8<-- "lessons/current-2/src/app/command.rs:step-1e"
```

## Step 2 · src/app/mod.rs
Declare the new application modules so their files join the crate.

`lessons/current-2/src/app/mod.rs` · edit · type this

Added after the line `pub mod manifest;` in `lessons/current-1/src/app/mod.rs`

```rust
--8<-- "lessons/current-2/src/app/mod.rs:step-2"
```

## Step 3 · src/app/modeling.rs
Create points and curves, or trim, extend and explode selected geometry in a document transaction.

`lessons/current-2/src/app/modeling.rs` · 315 lines · type this, new file

```rust
--8<-- "lessons/current-2/src/app/modeling.rs"
```

## Step 4 · src/app/scene.rs
Make the scene helpers available to the modeling module.

`lessons/current-2/src/app/scene.rs` · edit · type this

Added after the line `pub last_edited: Option<usize>,` in `lessons/current-1/src/app/scene.rs`

```rust
--8<-- "lessons/current-2/src/app/scene.rs:step-4a"
```

Added after the line `last_edited: None,` in `lessons/current-1/src/app/scene.rs`

```rust
--8<-- "lessons/current-2/src/app/scene.rs:step-4b"
```

## Step 5 · src/state/edit.rs
Dispatch a modeling command, rebuild its display and report the result.

`lessons/current-2/src/state/edit.rs` · edit · type this

Added after the line `pub fn run_command(&mut self, line: &str) -> Result<Strin…` in `lessons/current-1/src/state/edit.rs`

```rust
--8<-- "lessons/current-2/src/state/edit.rs:step-5a"
```

Added after the line `match command {` in `lessons/current-1/src/state/edit.rs`

```rust
--8<-- "lessons/current-2/src/state/edit.rs:step-5b"
```

Replaces the line `self.delete_selected();` in `lessons/current-1/src/state/edit.rs`

```rust
--8<-- "lessons/current-2/src/state/edit.rs:step-5c"
```

## Check

Run `trunk serve` in `lessons/current-2/` and open <http://127.0.0.1:8770/>.

Expected: a Point or Line command adds selected geometry and reports **geometry updated**.

![Full viewer result for current 2](screenshots/extensions-modeling.png)

If it fails:

- A malformed command changes the scene: argument validation happens after mutation.
- Explode needs several Undo actions: each piece is committed separately.

## What changed

```text
lessons/current-2/src/
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
│   ├── command.rs  ~
│   ├── coords.rs
│   ├── cplane.rs
│   ├── decode.rs
│   ├── edit.rs
│   ├── feedback.rs
│   ├── fetch.rs
│   ├── gizmo.rs
│   ├── input.rs
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs  ~
│   ├── modeling.rs  +
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
│   └── triangle_tiles.wgsl
├── state/
│   ├── cloud_query.rs
│   ├── edit.rs  ~
│   ├── sheet_query.rs
│   └── text.rs
├── camera.rs
├── lib.rs
└── state.rs
```

`+` new in this lesson · `~` changed in this lesson

Data flow: command text → validated modeling operation → source transaction → existing draw paths.
Every file at this point: `lessons/current-2/`.

## Next

[Continue with current-3](current-3.md).

## Expected viewer result

A newly created line has been trimmed to its middle 60% and selected. Compare its shortened extent with the other geometry in the full viewer. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb). At this checkpoint the command field still uses the original DOM interface, and the handles are drawn with strokes. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 2](screenshots/extensions-modeling.png)](screenshots/extensions-modeling.png)
