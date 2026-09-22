# Make control dragging respect object placement

Drag polyline and curve controls on placed objects without a jump on release.

![Running viewer: Make control dragging respect object placement.](screenshots/extensions-controls.png)

## Starting point

Copy checkpoint 21 and check it builds; the finished steps are in `lessons/controls-1/` … `lessons/controls-3/`.

```bash
cp -r docs/lessons/21 docs/lessons/my-controls
cd docs/lessons/my-controls
cargo check -j4 --lib
```

## Step 1 · Convert a world target back into source coordinates

Convert the world target back to the object's local coordinates before the kernel stores it.

### `src/app/edit.rs`

`lessons/controls-1/src/app/edit.rs` · type this, added at the start of `fn set_control_point`

```rust
--8<-- "lessons/controls-1/src/app/edit.rs:step-1"
```

`lessons/controls-1/src/app/edit.rs` · type this, added at the end of the `mod tests` block

```rust
--8<-- "lessons/controls-1/src/app/edit.rs:step-1b"
```

### Check step 1

Run `cargo check -j4 --lib`.

## Step 2 · Keep hit tests in world space and previews local

Hit-test in world space, snap to placed neighbours, and preview in local space.

### `src/state/edit.rs`

`lessons/controls-2/src/state/edit.rs` · type this, added after the `plane: CPlane,` line

```rust
--8<-- "lessons/controls-2/src/state/edit.rs:step-2"
```

`lessons/controls-2/src/state/edit.rs` · type this, replaces the `let at = self.controls.points[index].position;` line block

```rust
--8<-- "lessons/controls-2/src/state/edit.rs:step-2b"
```

`lessons/controls-2/src/state/edit.rs` · type this, added after the `plane: CPlane::facing(&forward),` line

```rust
--8<-- "lessons/controls-2/src/state/edit.rs:step-2c"
```

`lessons/controls-2/src/state/edit.rs` · type this, added before the `let index = active.index;` line

```rust
--8<-- "lessons/controls-2/src/state/edit.rs:step-2d"
```

`lessons/controls-2/src/state/edit.rs` · type this, added before the `let Some(point) = self.control_target(&active, x, y) else {` line

```rust
--8<-- "lessons/controls-2/src/state/edit.rs:step-2e"
```

`lessons/controls-2/src/state/edit.rs` · type this, replaces the `let (from, dir) = self.camera.ray((x, y), self.viewport())?;` line block

```rust
--8<-- "lessons/controls-2/src/state/edit.rs:step-2f"
```

`lessons/controls-2/src/state/edit.rs` · type this, replaces the `control.position[2],` line block

```rust
--8<-- "lessons/controls-2/src/state/edit.rs:step-2g"
```

### Check step 2

Run `cargo check -j4 --lib`.

## Step 3 · Restore previews on cancellation and failure

On cancel or failure, rebuild the controls from the source and restore the saved placement.

### `src/state.rs`

`lessons/controls-3/src/state.rs` · type this, added at the start of `fn clear`

```rust
--8<-- "lessons/controls-3/src/state.rs:step-3"
```

`lessons/controls-3/src/state.rs` · type this, added at the start of `fn select`

```rust
--8<-- "lessons/controls-3/src/state.rs:step-3b"
```

### `src/state/edit.rs`

`lessons/controls-3/src/state/edit.rs` · type this, added after the `let Some(active) = self.dragging.take() else {` line

```rust
--8<-- "lessons/controls-3/src/state/edit.rs:step-3"
```

`lessons/controls-3/src/state/edit.rs` · type this, replaces the `if self.control_drag.take().is_some() {` line block

```rust
--8<-- "lessons/controls-3/src/state/edit.rs:step-3b"
```

### Check step 3

Run `cargo check -j4 --lib`.

## Check

Run `cargo xtest -j4 --lib app::edit`, then `trunk serve --port 8780` and open <http://localhost:8780/?data=off&inspect=1>.

For the screenshot scene, copy the [nested fixture](extensions/nested.pb) and its [manifest](extensions/nested.yaml) into `assets/` as `extension-nested.pb` / `.yaml` and open <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.

## Try

Select the placed polyline, press F10, drag a control and release: no jump. Escape mid-drag puts it back.

## Finished code

step 1 in `lessons/controls-1/`, step 2 in `lessons/controls-2/`, step 3 in `lessons/controls-3/`.

## Expected viewer result

The placed polyline with its controls, one of them just dragged.

[![Full viewer result for extend controls tutorial](screenshots/extensions-controls.png)](screenshots/extensions-controls.png)
