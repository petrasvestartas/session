# Build the nested session and graph panel

Browse nested groups in the layers panel, select their objects together, and hide or show whole subtrees.

![Running viewer: Build the nested session and graph panel.](screenshots/extensions-panels.png)

## Starting point

Copy checkpoint 21 and check it builds; the finished steps are in `lessons/panels-1/` … `lessons/panels-3/`.

```bash
cp -r docs/lessons/21 docs/lessons/my-panels
cd docs/lessons/my-panels
cargo check -j4 --lib
```

## Step 1 · Index source identity and subtree ranges

Build the index: every node points at one range of rows, so a parent knows its descendants without copying them.

### `src/app/hierarchy.rs`

`lessons/panels-1/src/app/hierarchy.rs` · type this, new file

```rust
--8<-- "lessons/panels-1/src/app/hierarchy.rs"
```

### `src/app/mod.rs`

`lessons/panels-1/src/app/mod.rs` · type this, added at the start of `mod gizmo`

```rust
--8<-- "lessons/panels-1/src/app/mod.rs:step-1"
```

### `src/app/scene.rs`

`lessons/panels-1/src/app/scene.rs` · type this, added after the `pub last_edited: Option<usize>,` line

```rust
--8<-- "lessons/panels-1/src/app/scene.rs:step-1"
```

`lessons/panels-1/src/app/scene.rs` · type this, added after the `last_edited: None,` line

```rust
--8<-- "lessons/panels-1/src/app/scene.rs:step-1b"
```

`lessons/panels-1/src/app/scene.rs` · type this, added at the start of `fn reset_rows`

```rust
--8<-- "lessons/panels-1/src/app/scene.rs:step-1c"
```

`lessons/panels-1/src/app/scene.rs` · type this, added after the `pub(super) fn push_row(&mut self, owner: usize, guid: &st…` line

```rust
--8<-- "lessons/panels-1/src/app/scene.rs:step-1d"
```

`lessons/panels-1/src/app/scene.rs` · type this, added at the start of `fn add_file`

```rust
--8<-- "lessons/panels-1/src/app/scene.rs:step-1e"
```

### Check step 1

Run `cargo check -j4 --lib`.

## Step 2 · Connect selection, visibility and cleanup

Turn a panel click into a row range, then write the hidden flags and highlights for that range.

### `src/state.rs`

`lessons/panels-2/src/state.rs` · type this, added at the start of `mod edit`

```rust
--8<-- "lessons/panels-2/src/state.rs:step-2"
```

`lessons/panels-2/src/state.rs` · type this, added after the `pub selection: SelectionMode,` line

```rust
--8<-- "lessons/panels-2/src/state.rs:step-2b"
```

`lessons/panels-2/src/state.rs` · type this, added after the `selection: SelectionMode::Object,` line

```rust
--8<-- "lessons/panels-2/src/state.rs:step-2c"
```

`lessons/panels-2/src/state.rs` · type this, added at the start of `fn clear`

```rust
--8<-- "lessons/panels-2/src/state.rs:step-2d"
```

`lessons/panels-2/src/state.rs` · type this, added at the start of `fn select`

```rust
--8<-- "lessons/panels-2/src/state.rs:step-2e"
```

`lessons/panels-2/src/state.rs` · type this, added at the start of `fn hide_selected`

```rust
--8<-- "lessons/panels-2/src/state.rs:step-2f"
```

### `src/state/edit.rs`

`lessons/panels-2/src/state/edit.rs` · type this, replaces the `if !self.scene.delete_row(row) {` line block

```rust
--8<-- "lessons/panels-2/src/state/edit.rs:step-2"
```

`lessons/panels-2/src/state/edit.rs` · type this, added at the start of `fn after_history`

```rust
--8<-- "lessons/panels-2/src/state/edit.rs:step-2b"
```

`lessons/panels-2/src/state/edit.rs` · type this, replaces the `let hidden: Vec<bool> = rows` line block

```rust
--8<-- "lessons/panels-2/src/state/edit.rs:step-2c"
```

`lessons/panels-2/src/state/edit.rs` · type this, replaces the `let rows: Vec<crate::app::feedback::LayerRow> = layers::r…` line block

```rust
--8<-- "lessons/panels-2/src/state/edit.rs:step-2d"
```

`lessons/panels-2/src/state/edit.rs` · type this, added after the `.collect();` line

```rust
--8<-- "lessons/panels-2/src/state/edit.rs:step-2e"
```

### `src/state/panel.rs`

`lessons/panels-2/src/state/panel.rs` · type this, new file

```rust
--8<-- "lessons/panels-2/src/state/panel.rs"
```

### Check step 2

Run `cargo check -j4 --lib`.

## Step 3 · Wire the buttons and bound the visible page

Draw the rows with their buttons, 128 per page, and send their clicks through the existing listener.

### `index.html`

`lessons/panels-3/index.html` · type this, replaces the `<div id="viewer-layers" hidden role="group" aria-label="L…` line block

```html
--8<-- "lessons/panels-3/index.html:step-3"
```

### `src/app/feedback.rs`

`lessons/panels-3/src/app/feedback.rs` · type this, replaces the `"style",` line block

```rust
--8<-- "lessons/panels-3/src/app/feedback.rs:step-3"
```

### `src/app/inspection.rs`

`lessons/panels-3/src/app/inspection.rs` · type this, added after the `"selected": parent,` line

```rust
--8<-- "lessons/panels-3/src/app/inspection.rs:step-3"
```

### `src/app/layers.rs`

`lessons/panels-3/src/app/layers.rs` · type this, replaces the `fn rows` block

```rust
--8<-- "lessons/panels-3/src/app/layers.rs:step-3"
```

`lessons/panels-3/src/app/layers.rs` · type this, replaces the `fn all_hidden` block

```rust
--8<-- "lessons/panels-3/src/app/layers.rs:step-3b"
```

`lessons/panels-3/src/app/layers.rs` · type this, added before `fn a_layer_names_the_rows_it_controls`

```rust
--8<-- "lessons/panels-3/src/app/layers.rs:step-3c"
```

### `src/lib.rs`

`lessons/panels-3/src/lib.rs` · type this, replaces the `Msg::ToggleLayer(key) => {` line block

```rust
--8<-- "lessons/panels-3/src/lib.rs:step-3"
```

### Check step 3

Run `cargo check -j4 --lib`.

## Check

Run `cargo xtest -j4 --lib app::hierarchy`, then `trunk serve --port 8780` and open <http://localhost:8780/?data=off&inspect=1>.

For the screenshot scene, copy the [nested fixture](extensions/nested.pb) and its [manifest](extensions/nested.yaml) into `assets/` as `extension-nested.pb` / `.yaml` and open <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.

## Try

Press L, open Assembly and Nested, select Nested: both beams highlight. Hide it, show it.

## Finished code

step 1 in `lessons/panels-1/`, step 2 in `lessons/panels-2/`, step 3 in `lessons/panels-3/`.

## Expected viewer result

The panel shows Assembly → Nested and both beams are highlighted together.

[![Full viewer result for extend panels tutorial](screenshots/extensions-panels.png)](screenshots/extensions-panels.png)
