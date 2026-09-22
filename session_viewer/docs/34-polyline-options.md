# 34 · Polyline options, ordered input and a remembered view
The polyline command offers Points, Rectangle and Polygon, clicks and keys keep their order, and loading a scene never overrides a view you already chose.

## Step 1 · src/app/command.rs
`Polyline` alone opens three options instead of running.

`lessons/34/src/app/command.rs` · edit · type this

Replaces the line `"polyline" => "Polyline points… · Example: Polyline 0,0,0…` in `lessons/33/src/app/command.rs`

```rust
--8<-- "lessons/34/src/app/command.rs:step-1a"
```

Added after the line `{` in `lessons/33/src/app/command.rs`

```rust
--8<-- "lessons/34/src/app/command.rs:step-1b"
```

Replaces the line `if (!options(text).is_empty() && words.len() == 1)` in `lessons/33/src/app/command.rs`

```rust
--8<-- "lessons/34/src/app/command.rs:step-1c"
```

Added after the line `assert_eq!(accept("Lin"), ("Line".into(), true));` in `lessons/33/src/app/command.rs`

```rust
--8<-- "lessons/34/src/app/command.rs:step-1d"
```

## Step 2 · src/state/drawing.rs
Replace the whole file: the drawing state handles the three polyline modes.

`lessons/34/src/state/drawing.rs` · replace the whole file · type this

```rust
--8<-- "lessons/34/src/state/drawing.rs:step-2"
```

## Step 3 · src/app/ui.rs
The command panel shows the polyline options and runs events in the order they arrived.

`lessons/34/src/app/ui.rs` · edit · type this

Added after the line `pub drawing_prompt: String,` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3a"
```

Added after the line `let mut consumed = response.consumed;` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3b"
```

Replaces the 3 lines from `consumed = self.ui_drag` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3c"
```

Replaces the line `consumed = in_popup(self.pointer) || !self.scene_rect.con…` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3d"
```

Replaces the 2 lines from `self.ui_drag =` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3e"
```

Replaces the line `MODEL.with_borrow_mut(|model| model.drawing_prompt = stat…` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3f"
```

Replaces the 7 lines from `let mut output = if let Some(pointer) = pointer_input {` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3g"
```

Added after the line `let previous_popup = model.completion_rect.take();` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3h"
```

Added after the line `.show_inside(root, |ui| {` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3i"
```

Replaces the 2 lines from `.min_scrolled_height(54.0)` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3j"
```

Added after the line `);` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3k"
```

Replaces the line `let inline_options = crate::app::command::options(&model.…` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3l"
```

Replaces the line `model.command = format!("{name} ");` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3m"
```

Added after the line `});` in `lessons/33/src/app/ui.rs`

```rust
--8<-- "lessons/34/src/app/ui.rs:step-3n"
```

## Step 4 · src/camera.rs
A `CameraPose` is the chosen view without the scene extent, so it can be compared.

`lessons/34/src/camera.rs` · edit · type this

Added after the line `}` in `lessons/33/src/camera.rs`

```rust
--8<-- "lessons/34/src/camera.rs:step-4a"
```

Added after the line `use super::*;` in `lessons/33/src/camera.rs`

```rust
--8<-- "lessons/34/src/camera.rs:step-4b"
```

## Step 5 · src/state.rs
The pose at load time is remembered and `fit_loaded` only fits when it is unchanged.

`lessons/34/src/state.rs` · edit · type this

Added after the line `pub camera: Camera,` in `lessons/33/src/state.rs`

```rust
--8<-- "lessons/34/src/state.rs:step-5a"
```

Added after the line `log::info!("gpu init {:.0} ms", now_ms() - t0);` in `lessons/33/src/state.rs`

```rust
--8<-- "lessons/34/src/state.rs:step-5b"
```

Added after the line `pub fn clear(&mut self) {` in `lessons/33/src/state.rs`

```rust
--8<-- "lessons/34/src/state.rs:step-5c"
```

Added after the line `self.touch();` in `lessons/33/src/state.rs`

```rust
--8<-- "lessons/34/src/state.rs:step-5d"
```

## Step 6 · src/lib.rs
The loader calls `fit_loaded` instead of `fit_all`.

`lessons/34/src/lib.rs` · edit · type this

Replaces the line `Msg::Fit => state.fit_all(),` in `lessons/33/src/lib.rs`

```rust
--8<-- "lessons/34/src/lib.rs:step-6"
```

## Check

Run `trunk serve` in `lessons/34/` and open <http://127.0.0.1:8770/>.

Type `Polyline`, pick Rectangle, click two corners; then orbit and reload a scene: the view stays where you put it.

## Next

[35 · Attributes On|Off and a phone keyboard](35-attributes.md)
