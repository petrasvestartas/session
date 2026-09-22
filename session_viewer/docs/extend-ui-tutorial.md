# Build the egui panel and command interface

Replace the DOM controls with an egui interface drawn after the scene: a command line and a layers window.

![CPU/GPU flow and resource lifetime](illustrations/extend-ui.svg)

## Starting point

Copy checkpoint 21 and check it builds; the finished steps are in `lessons/ui-1/`.

```bash
cp -r docs/lessons/21 docs/lessons/my-ui
cd docs/lessons/my-ui
cargo check -j4 --lib
```

## Step 1 · Connect input, UI actions and the final render pass

Add the two UI modules, route winit events to egui first, and draw the interface as the last render pass.

### `Cargo.lock`

Copy `Cargo.lock` from `lessons/ui-1/` over yours.

### `Cargo.toml`

Copy `Cargo.toml` from `lessons/ui-1/` over yours.

### `index.html`

`lessons/ui-1/index.html` · type this, added before the `#no-webgpu {` line

```html
--8<-- "lessons/ui-1/index.html:step-1"
```

`lessons/ui-1/index.html` · type this, replaces the `<div id="viewer-status" role="status" aria-live="polite" …` line block

```html
--8<-- "lessons/ui-1/index.html:step-1b"
```

### `src/app/feedback.rs`

`lessons/ui-1/src/app/feedback.rs` · type this, added before the `log::info!("{message}");` line

```rust
--8<-- "lessons/ui-1/src/app/feedback.rs:step-1"
```

`lessons/ui-1/src/app/feedback.rs` · type this, replaces the `fn command_line` block

```rust
--8<-- "lessons/ui-1/src/app/feedback.rs:step-1b"
```

`lessons/ui-1/src/app/feedback.rs` · type this, replaces the `fn command_text` block

```rust
--8<-- "lessons/ui-1/src/app/feedback.rs:step-1c"
```

`lessons/ui-1/src/app/feedback.rs` · type this, replaces the `fn layers_panel` block

```rust
--8<-- "lessons/ui-1/src/app/feedback.rs:step-1d"
```

### `src/app/input.rs`

Delete the `struct CommandKeys` block from `lessons/21/src/app/input.rs`.

### `src/app/mod.rs`

`lessons/ui-1/src/app/mod.rs` · type this, added at the start of `mod touch`

```rust
--8<-- "lessons/ui-1/src/app/mod.rs:step-1"
```

### `src/app/ui.rs`

`lessons/ui-1/src/app/ui.rs` · type this, new file

```rust
--8<-- "lessons/ui-1/src/app/ui.rs"
```

### `src/engine/gpu/mod.rs`

`lessons/ui-1/src/engine/gpu/mod.rs` · type this, added at the start of `mod triangle_tiles`

```rust
--8<-- "lessons/ui-1/src/engine/gpu/mod.rs:step-1"
```

`lessons/ui-1/src/engine/gpu/mod.rs` · type this, added after the `pub glyphs: GlyphLane,` line

```rust
--8<-- "lessons/ui-1/src/engine/gpu/mod.rs:step-1b"
```

`lessons/ui-1/src/engine/gpu/mod.rs` · type this, added after the `glyphs,` line

```rust
--8<-- "lessons/ui-1/src/engine/gpu/mod.rs:step-1c"
```

### `src/engine/gpu/render.rs`

`lessons/ui-1/src/engine/gpu/render.rs` · type this, added before the `(draws, self.objects.len())` line

```rust
--8<-- "lessons/ui-1/src/engine/gpu/render.rs:step-1"
```

### `src/engine/gpu/ui.rs`

`lessons/ui-1/src/engine/gpu/ui.rs` · type this, new file

```rust
--8<-- "lessons/ui-1/src/engine/gpu/ui.rs"
```

### `src/lib.rs`

`lessons/ui-1/src/lib.rs` · type this, replaces the `CancelPointer,` line block

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1"
```

`lessons/ui-1/src/lib.rs` · type this, replaces the `pointer_cancellation: Option<app::input::PointerCancellat…` line block

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1b"
```

`lessons/ui-1/src/lib.rs` · type this, replaces the `pointer_cancellation: None,` line block

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1c"
```

`lessons/ui-1/src/lib.rs` · type this, replaces the `state.window.request_redraw();` line block

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1d"
```

`lessons/ui-1/src/lib.rs` · type this, replaces the `if let Some(input) = app::feedback::command_line(false) {` line block

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1e"
```

`lessons/ui-1/src/lib.rs` · type this, replaces the `CancelPointer,` line and the two after it

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1f"
```

`lessons/ui-1/src/lib.rs` · type this, added before the `let changed = match event {` line

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1g"
```

`lessons/ui-1/src/lib.rs` · type this, replaces the `} else {` line block

```rust
--8<-- "lessons/ui-1/src/lib.rs:step-1h"
```

### `src/state.rs`

`lessons/ui-1/src/state.rs` · type this, added before `fn touch`

```rust
--8<-- "lessons/ui-1/src/state.rs:step-1"
```

### Check step 1

Run `cargo check -j4 --lib`.

## Check

Run `cargo xtest -j4 --lib app::command`, then `trunk serve --port 8780` and open <http://localhost:8780/?data=off&inspect=1>.

For the screenshot scene, copy the [nested fixture](extensions/nested.pb) and its [manifest](extensions/nested.yaml) into `assets/` as `extension-nested.pb` / `.yaml` and open <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.

## Try

Press `:`, type `move 10,0,0` with an object selected, Enter. Press L for the layers window.

## Finished code

step 1 in `lessons/ui-1/`.

## Expected viewer result

The white command area with a finished command and the geometry it made.

[![Full viewer result for extend ui tutorial](screenshots/extensions-command-create.png)](screenshots/extensions-command-create.png)
