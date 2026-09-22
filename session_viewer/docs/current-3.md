# current-3 · Make control dragging respect object placement

Control points follow a placed object correctly during dragging and return to their source positions on cancellation.

## Step 1 · src/app/edit.rs
Convert the world-space control target through the inverse object placement, then test the result under translation and scale.

`lessons/current-3/src/app/edit.rs` · edit · type this

Added after the line `pub fn set_control_point(&mut self, row: u32, index: usiz…` in `lessons/current-2/src/app/edit.rs`

```rust
--8<-- "lessons/current-3/src/app/edit.rs:step-1a"
```

Added after the line `}` in `lessons/current-2/src/app/edit.rs`

```rust
--8<-- "lessons/current-3/src/app/edit.rs:step-1b"
```

## Step 2 · src/state.rs
Cancel an active gesture before clearing the scene or changing selection.

`lessons/current-3/src/state.rs` · edit · type this

Added after the line `pub fn clear(&mut self) {` in `lessons/current-2/src/state.rs`

```rust
--8<-- "lessons/current-3/src/state.rs:step-2a"
```

Added after the line `pub fn select(&mut self, row: Option<u32>) {` in `lessons/current-2/src/state.rs`

```rust
--8<-- "lessons/current-3/src/state.rs:step-2b"
```

## Step 3 · src/state/edit.rs
Restore the grab placement before committing, restore controls on cancellation, and retain the control origin for placed previews.

`lessons/current-3/src/state/edit.rs` · edit · type this

Added after the line `};` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3a"
```

Replaces the line `if self.control_drag.take().is_some() {` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3b"
```

Added after the line `plane: CPlane,` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3c"
```

Replaces the line `let Some((sx, sy)) = self.project(at) else {` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3d"
```

Added after the line `plane: CPlane::facing(&forward),` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3e"
```

Added after the line `};` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3f"
```

Added after the line `};` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3g"
```

Replaces the 5 lines from `let origin = {` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3h"
```

Replaces the line `),` in `lessons/current-2/src/state/edit.rs`

```rust
--8<-- "lessons/current-3/src/state/edit.rs:step-3i"
```

## Check

Run `trunk serve` in `lessons/current-3/` and open <http://127.0.0.1:8770/>.

Expected: a placed control moves in world space, Escape restores it, and the status area shows no error.

![Full viewer result for current 3](screenshots/extensions-controls.png)

If it fails:

- A placed control jumps while dragging: the world target is stored as a local coordinate.
- Escape leaves a displaced control: cancellation does not restore source-derived controls.

## What changed

```text
lessons/current-3/src/
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
│   ├── input.rs
│   ├── inspection.rs
│   ├── knobs.rs
│   ├── layers.rs
│   ├── live.rs
│   ├── loader.rs
│   ├── manifest.rs
│   ├── mod.rs
│   ├── modeling.rs
│   ├── route.rs
│   ├── scene.rs
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
└── state.rs  ~
```

`+` new in this lesson · `~` changed in this lesson

Data flow: pointer target → inverse placement → local control → one source edit.
Every file at this point: `lessons/current-3/`.

## Next

[Continue with current-4](current-4.md).

## Expected viewer result

The placed polyline remains in the scene with its source controls visible after a control has been dragged. Check the released control position and control polygon against the surrounding geometry. The capture uses the maintained viewer and the [nested fixture](extensions/nested.pb). At this checkpoint the gumball still uses strokes; the next chapter adds solid handles. The bottom dock, right Layers panel and left toolbar visible in this maintained-viewer reference are added in [checkpoint 8](current-8.md).

[![Full viewer result for current 3](screenshots/extensions-controls.png)](screenshots/extensions-controls.png)
