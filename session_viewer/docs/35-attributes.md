# 35 · Attributes On|Off and a phone keyboard
`Attributes On` draws an element's outlines, axes and sections inside its own row, and a hidden input lets a phone type into the command line.

## Step 1 · Cargo.toml
web-sys gains the three DOM types the hidden input needs.

`lessons/35/Cargo.toml` · edit · type this

Added after the line `"HtmlElement",` in `lessons/34/Cargo.toml`

```toml
--8<-- "lessons/35/Cargo.toml:step-1"
```

## Step 2 · index.html
A 1×1 invisible `<input>` is the element a phone keyboard can attach to.

`lessons/35/index.html` · edit · type this

Added after the line `<canvas id="canvas" tabindex="0" role="application" aria-…` in `lessons/34/index.html`

```html
--8<-- "lessons/35/index.html:step-2"
```

## Step 3 · src/app/mod.rs
The agent module is declared.

`lessons/35/src/app/mod.rs` · edit · type this

Added after the line `#[cfg(target_arch = "wasm32")]` in `lessons/34/src/app/mod.rs`

```rust
--8<-- "lessons/35/src/app/mod.rs:step-3"
```

## Step 4 · src/app/agent.rs
New file: the agent focuses the hidden input and replays its value into the command field.

`lessons/35/src/app/agent.rs` · new file · type this

```rust
--8<-- "lessons/35/src/app/agent.rs:step-4"
```

## Step 5 · src/app/command.rs
`Attributes`, `Attributes On`, `Attributes Off` are parsed and listed.

`lessons/35/src/app/command.rs` · edit · type this

Added after the line `Layers(Option<bool>),` in `lessons/34/src/app/command.rs`

```rust
--8<-- "lessons/35/src/app/command.rs:step-5a"
```

Added after the line `"layers" => "Layers (On Off): show or hide the layer panel",` in `lessons/34/src/app/command.rs`

```rust
--8<-- "lessons/35/src/app/command.rs:step-5b"
```

Added after the line `_ => Err("Layers (On Off)".into()),` in `lessons/34/src/app/command.rs`

```rust
--8<-- "lessons/35/src/app/command.rs:step-5c"
```

Added after the line `"layers" => &["Layers On", "Layers Off"],` in `lessons/34/src/app/command.rs`

```rust
--8<-- "lessons/35/src/app/command.rs:step-5d"
```

Replaces the 3 lines from `"Arctic", "Controls", "Curve", "Delete", "Edge", "Escape"…` in `lessons/34/src/app/command.rs`

```rust
--8<-- "lessons/35/src/app/command.rs:step-5e"
```

Added after the line `assert_eq!(parse("Layers OFF"), Ok(Command::Layers(Some(f…` in `lessons/34/src/app/command.rs`

```rust
--8<-- "lessons/35/src/app/command.rs:step-5f"
```

## Step 6 · src/app/scene.rs
The scene keeps an `attributes` flag and skips wood's baked attribute groups, which never get a row.

`lessons/35/src/app/scene.rs` · edit · type this

Added after the line `pub selected: Option<u32>,` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6a"
```

Added after the line `selected: None,` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6b"
```

Added after the line `self.guid_to_row.reserve(count);` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6c"
```

Replaces the line `if !is_drawable(geom) {` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6d"
```

Added after the line `row,` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6e"
```

Added after the line `}` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6f"
```

Added after the line `}` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6g"
```

Added after the line `row,` in `lessons/34/src/app/scene.rs`

```rust
--8<-- "lessons/35/src/app/scene.rs:step-6h"
```

## Step 7 · src/app/walk/mod.rs
With `attributes` on, the walker draws each element's features into the element's own row.

`lessons/35/src/app/walk/mod.rs` · edit · type this

Added after the line `use session_rust::AABB;` in `lessons/34/src/app/walk/mod.rs`

```rust
--8<-- "lessons/35/src/app/walk/mod.rs:step-7a"
```

Added after the line `pub row: u32,` in `lessons/34/src/app/walk/mod.rs`

```rust
--8<-- "lessons/35/src/app/walk/mod.rs:step-7b"
```

Added after the line `faces: false,` in `lessons/34/src/app/walk/mod.rs`

```rust
--8<-- "lessons/35/src/app/walk/mod.rs:step-7c"
```

Replaces the line `Geometry::Element(e) => match e.geometry() {` in `lessons/34/src/app/walk/mod.rs`

```rust
--8<-- "lessons/35/src/app/walk/mod.rs:step-7d"
```

## Step 8 · src/app/walk/brep.rs
Brep walkers pass `attributes: false`.

`lessons/35/src/app/walk/brep.rs` · edit · type this

Added after the line `row: 5,` in `lessons/34/src/app/walk/brep.rs`

```rust
--8<-- "lessons/35/src/app/walk/brep.rs:step-8a"
```

Added after the line `row: 7,` in `lessons/34/src/app/walk/brep.rs`

```rust
--8<-- "lessons/35/src/app/walk/brep.rs:step-8b"
```

## Step 9 · src/app/walk/brep_orient.rs
The orient walker passes `attributes: false`.

`lessons/35/src/app/walk/brep_orient.rs` · edit · type this

Added after the line `row: 0,` in `lessons/34/src/app/walk/brep_orient.rs`

```rust
--8<-- "lessons/35/src/app/walk/brep_orient.rs:step-9"
```

## Step 10 · src/app/walk/mesh_ink.rs
Mesh ink passes `attributes: false`.

`lessons/35/src/app/walk/mesh_ink.rs` · edit · type this

Added after the line `row: 0,` in `lessons/34/src/app/walk/mesh_ink.rs`

```rust
--8<-- "lessons/35/src/app/walk/mesh_ink.rs:step-10a"
```

Added after the line `row: 7,` in `lessons/34/src/app/walk/mesh_ink.rs`

```rust
--8<-- "lessons/35/src/app/walk/mesh_ink.rs:step-10b"
```

## Step 11 · src/app/mesh_preview.rs
The mesh preview passes `attributes: false`.

`lessons/35/src/app/mesh_preview.rs` · edit · type this

Added after the line `row: 0,` in `lessons/34/src/app/mesh_preview.rs`

```rust
--8<-- "lessons/35/src/app/mesh_preview.rs:step-11"
```

## Step 12 · src/app/surface_preview.rs
The surface preview passes `attributes: false`.

`lessons/35/src/app/surface_preview.rs` · edit · type this

Added after the line `row: 0,` in `lessons/34/src/app/surface_preview.rs`

```rust
--8<-- "lessons/35/src/app/surface_preview.rs:step-12a"
```

Added after the line `row: 0,` in `lessons/34/src/app/surface_preview.rs`

```rust
--8<-- "lessons/35/src/app/surface_preview.rs:step-12b"
```

Added after the line `row: 3,` in `lessons/34/src/app/surface_preview.rs`

```rust
--8<-- "lessons/35/src/app/surface_preview.rs:step-12c"
```

## Step 13 · src/app/ui.rs
The UI remembers the command rect and replays the agent's typing into the field.

`lessons/35/src/app/ui.rs` · edit · type this

Replaces the 2 lines from `completion_rect: Option<egui::Rect>,` in `lessons/34/src/app/ui.rs`

```rust
--8<-- "lessons/35/src/app/ui.rs:step-13a"
```

Added after the line `touches: std::collections::HashSet<u64>,` in `lessons/34/src/app/ui.rs`

```rust
--8<-- "lessons/35/src/app/ui.rs:step-13b"
```

Added after the line `touches: std::collections::HashSet::new(),` in `lessons/34/src/app/ui.rs`

```rust
--8<-- "lessons/35/src/app/ui.rs:step-13c"
```

Added after the line `(consumed || escape, response.repaint || escape)` in `lessons/34/src/app/ui.rs`

```rust
--8<-- "lessons/35/src/app/ui.rs:step-13d"
```

Added after the line `self.publish();` in `lessons/34/src/app/ui.rs`

```rust
--8<-- "lessons/35/src/app/ui.rs:step-13e"
```

## Step 14 · src/state.rs
`show_attributes` toggles the flag and rebuilds the rows.

`lessons/35/src/state.rs` · edit · type this

Added after the line `self.touch();` in `lessons/34/src/state.rs`

```rust
--8<-- "lessons/35/src/state.rs:step-14"
```

## Step 15 · src/state/edit.rs
The `Attributes` command reports On or Off.

`lessons/35/src/state/edit.rs` · edit · type this

Added after the line `}` in `lessons/34/src/state/edit.rs`

```rust
--8<-- "lessons/35/src/state/edit.rs:step-15"
```

## Step 16 · src/lib.rs
Agent events reach the UI through the message loop.

`lessons/35/src/lib.rs` · edit · type this

Added after the line `CancelPointer,` in `lessons/34/src/lib.rs`

```rust
--8<-- "lessons/35/src/lib.rs:step-16a"
```

Added after the line `pointer_cancellation: Option<app::input::PointerCancellat…` in `lessons/34/src/lib.rs`

```rust
--8<-- "lessons/35/src/lib.rs:step-16b"
```

Added after the line `pointer_cancellation: None,` in `lessons/34/src/lib.rs`

```rust
--8<-- "lessons/35/src/lib.rs:step-16c"
```

Replaces the line `match app::input::PointerCancellation::new(canvas, proxy.…` in `lessons/34/src/lib.rs`

```rust
--8<-- "lessons/35/src/lib.rs:step-16d"
```

Added after the line `self.input.cancel();` in `lessons/34/src/lib.rs`

```rust
--8<-- "lessons/35/src/lib.rs:step-16e"
```

## Check

Run `trunk serve` in `lessons/35/` and open <http://127.0.0.1:8770/>.

Type `Attributes On`: outlines and axes appear inside each element and move with it; on a phone, tap the command line and the keyboard opens.

## Next

[36 · Translucent faces and the Opacity command](36-translucent-faces.md)
