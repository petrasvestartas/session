# 12 · maintained viewer shell and picking

The seven-object fixture supports object and source-edge selection.

![A pointer release becomes a scissored ID window, an asynchronous bounded readback, a Scene lookup and a selected flag; stale generations are dropped.](illustrations/picking.svg)

Copy each file from the lesson folder to the path shown.

Copy from `lessons/12/` (tooling this checkpoint needs but the course does not teach):

- `lessons/12/assets/pb/interaction.pb.json`
- `lessons/12/src/selftest/lifecycle.rs`
- `lessons/12/assets/pb/interaction.pb` (binary)

## Step 1 · src/engine/gpu/device.rs

New file: open the GPU as in lesson 01, now with adapter choice, a larger buffer limit and stored errors.

`lessons/12/src/engine/gpu/device.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/engine/gpu/device.rs:step-1a"
```

`lessons/12/src/engine/gpu/device.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/device.rs:step-1b"
```

`lessons/12/src/engine/gpu/device.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/device.rs:step-1c"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/engine/gpu/device.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/device.rs:step-1d"
```

## Step 2 · src/engine/gpu/present.rs

New file: draw a frame to the canvas, run a pick-only frame, or render off-screen for native tests.

`lessons/12/src/engine/gpu/present.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/engine/gpu/present.rs:step-2a"
```

`lessons/12/src/engine/gpu/present.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/present.rs:step-2b"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/engine/gpu/present.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/present.rs:step-2c"
```

## Step 3 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes.

`lessons/12/src/engine/gpu/render.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/engine/gpu/render.rs:step-3a"
```

`lessons/12/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/render.rs:step-3b"
```

## Step 4 · src/app/input.rs

Input routes gestures and keyboard actions to State.

`lessons/12/src/app/input.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/app/input.rs:step-4a"
```

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:step-4b"
```

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:step-4c"
```

`lessons/12/src/app/input.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:step-4d"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/app/input.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/app/input.rs:step-4e"
```

## Step 5 · src/app/touch.rs

New file: one finger orbits, two fingers pan and pinch, a tap picks and a double tap fits.

`lessons/12/src/app/touch.rs` · copy the file, new file, start with these lines

```rust
--8<-- "lessons/12/src/app/touch.rs:step-5a"
```

`lessons/12/src/app/touch.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/touch.rs:step-5b"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/app/touch.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/app/touch.rs:step-5c"
```

## Step 6 · src/app/scene.rs

The scene owns source documents and maps their identities to GPU rows.

`lessons/12/src/app/scene.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6a"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6b"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6c"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6d"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6e"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6f"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6g"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6h"
```

`lessons/12/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/scene.rs:step-6i"
```

## Step 7 · src/app/selection.rs

New file: what inside the picked object is selected, nothing or one edge.

`lessons/12/src/app/selection.rs` · 31 lines · type this, new file

```rust
--8<-- "lessons/12/src/app/selection.rs"
```

## Step 8 · src/app/walk/cloud.rs

New file: walk a point cloud, or one streamed slice of it, into point rows, octree nodes and one draw.

`lessons/12/src/app/walk/cloud.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/app/walk/cloud.rs:step-8a"
```

`lessons/12/src/app/walk/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/walk/cloud.rs:step-8b"
```

`lessons/12/src/app/walk/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/walk/cloud.rs:step-8c"
```

`lessons/12/src/app/walk/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/walk/cloud.rs:step-8d"
```

`lessons/12/src/app/walk/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/app/walk/cloud.rs:step-8e"
```

## Step 9 · src/app/walk/frames.rs

New file: draw a plane as a 1 m square and a box as its 12 edges.

`lessons/12/src/app/walk/frames.rs` · 86 lines · type this, new file

```rust
--8<-- "lessons/12/src/app/walk/frames.rs"
```

## Step 10 · src/app/walk/points.rs

New file: draw a point as one dot.

`lessons/12/src/app/walk/points.rs` · 22 lines · type this, new file

```rust
--8<-- "lessons/12/src/app/walk/points.rs"
```

## Step 11 · src/app/stream.rs

Streaming reads bounded chunks and keeps stable source addresses.

`lessons/12/src/app/stream.rs` · 38 lines · type this, new file

```rust
--8<-- "lessons/12/src/app/stream.rs"
```

## Step 12 · src/app/feedback.rs

New file: status messages, and the error panel with its reload button.

`lessons/12/src/app/feedback.rs` · 29 lines · type this, new file

```rust
--8<-- "lessons/12/src/app/feedback.rs"
```

## Step 13 · src/app/inspection.rs

Copy the file: with ?inspect=1, a JSON snapshot of counts and memory for the browser tests.

`lessons/12/src/app/inspection.rs` · 113 lines · copy the file, new file

```rust
--8<-- "lessons/12/src/app/inspection.rs"
```

## Step 14 · src/app/loader.rs

The loader stages manifest and geometry work before publishing it.

`lessons/12/src/app/loader.rs` · 83 lines · type this, new file

```rust
--8<-- "lessons/12/src/app/loader.rs"
```

Run `cargo check` in `lessons/12/`.

## Step 15 · src/engine/gpu/pick.rs

Picking reads an object and subobject ID asynchronously.

`lessons/12/src/engine/gpu/pick.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15a"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15b"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15c"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15d"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15e"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15f"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15g"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15h"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15i"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15j"
```

`lessons/12/src/engine/gpu/pick.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15k"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/engine/gpu/pick.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/pick.rs:step-15l"
```

## Step 16 · src/engine/gpu/render.rs

The frame encoder orders face, ink, picking and overlay passes.

`lessons/12/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/render.rs:step-16"
```

## Step 17 · src/engine/gpu/selection_outline.rs

A selection mask draws a border around visible selected geometry.

`lessons/12/src/engine/gpu/selection_outline.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/engine/gpu/selection_outline.rs:step-17a"
```

`lessons/12/src/engine/gpu/selection_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/selection_outline.rs:step-17b"
```

`lessons/12/src/engine/gpu/selection_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/selection_outline.rs:step-17c"
```

`lessons/12/src/engine/gpu/selection_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/selection_outline.rs:step-17d"
```

`lessons/12/src/engine/gpu/selection_outline.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/selection_outline.rs:step-17e"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/engine/gpu/selection_outline.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/engine/gpu/selection_outline.rs:step-17f"
```

## Step 18 · src/shaders/selection_outline.wgsl

The selection outline expands the selected coverage mask.

`lessons/12/src/shaders/selection_outline.wgsl` · 39 lines · type this, new file

```wgsl
--8<-- "lessons/12/src/shaders/selection_outline.wgsl"
```

Run `cargo check` in `lessons/12/`.

## Step 19 · src/state.rs

State coordinates input, selection and frame requests.

`lessons/12/src/state.rs` · type this, new file, start with these lines

```rust
--8<-- "lessons/12/src/state.rs:step-19a"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19b"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19c"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19d"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19e"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19f"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19g"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19h"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19i"
```

`lessons/12/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19j"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/state.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/state.rs:step-19k"
```

## Step 20 · src/engine/gpu/mod.rs

The GPU owner connects buffers, pipelines and frame resources.

`lessons/12/src/engine/gpu/mod.rs` · edit · type this

Replaces `mod frame` in `lessons/11/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/12/src/engine/gpu/mod.rs:step-20a"
```

Replaces the 12 lines from `view: view::View::from_env(),` in `fn new` of `lessons/11/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/12/src/engine/gpu/mod.rs:step-20b"
```

Replaces the 10 lines from `self.retarget(false);` in `fn set_scene` of `lessons/11/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/12/src/engine/gpu/mod.rs:step-20c"
```

```rust
--8<-- "lessons/12/src/engine/gpu/mod.rs:step-20d"
```

Replaces the `self.text.retarget(&self.ctx, target);` line in `fn retarget` of `lessons/11/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/12/src/engine/gpu/mod.rs:step-20e"
```

Replaces `fn rebase_anchor` in `lessons/11/src/engine/gpu/mod.rs`

```rust
--8<-- "lessons/12/src/engine/gpu/mod.rs:step-20f"
```

## Step 21 · src/app/mod.rs

The application module connects source loading and interaction helpers.

`lessons/12/src/app/mod.rs` · edit · type this

Replaces `mod knobs` in `lessons/11/src/app/mod.rs`

```rust
--8<-- "lessons/12/src/app/mod.rs:step-21"
```

## Step 22 · src/app/walk/mod.rs

The geometry walk dispatches source types into their render buffers.

`lessons/12/src/app/walk/mod.rs` · edit · type this

Added at the top of `lessons/11/src/app/walk/mod.rs`

```rust
--8<-- "lessons/12/src/app/walk/mod.rs:step-22a"
```

`lessons/12/src/app/walk/mod.rs` · edit · type this

Added after the `}` line of `lessons/11/src/app/walk/mod.rs`

```rust
--8<-- "lessons/12/src/app/walk/mod.rs:step-22b"
```

## Step 23 · src/app/route.rs

Route helpers read viewer options from the page URL.

`lessons/12/src/app/route.rs` · edit · type this

Replaces `fn query` in `lessons/11/src/app/route.rs`

```rust
--8<-- "lessons/12/src/app/route.rs:step-23"
```

## Step 24 · src/lib.rs

The crate entry point connects the camera, scene and GPU owners.

`lessons/12/src/lib.rs` · type this, replace the whole file, start with these lines

```rust
--8<-- "lessons/12/src/lib.rs:step-24a"
```

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:step-24b"
```

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:step-24c"
```

`lessons/12/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:step-24d"
```

Copy this part from the lesson folder to the path shown.

`lessons/12/src/lib.rs` · copy the file, append at the end of the file

```rust
--8<-- "lessons/12/src/lib.rs:step-24e"
```

## Step 25 · index.html

Copy this file from the lesson folder to the path shown.

`lessons/12/index.html` · edit · copy the file

Replaces the 109 lines from `<!doctype html>` of `lessons/11/index.html`

```html
--8<-- "lessons/12/index.html:step-25"
```

## Step 26 · assets/view_local.yaml

Copy this file from the lesson folder to the path shown.

`lessons/12/assets/view_local.yaml` · 3 lines · copy the file, new file

```yaml
--8<-- "lessons/12/assets/view_local.yaml"
```

## Step 27 · src/fixture.rs

Remove this file; its replacement is now part of the rendering modules.

Delete `src/fixture.rs` (it exists in `lessons/11/`, not in `lessons/12/`).

Run `cargo check` in `lessons/12/`.

## Check

Run `trunk serve` in `lessons/12/` and open <http://127.0.0.1:8770/>.

Expected: The seven-object fixture supports object and source-edge selection; status: **7 objects**.

![Checkpoint 12: the seven-object interaction fixture in the production shell, nothing selected.](screenshots/12.png)

If it fails:

- The highlight and GUID disagree: the row-to-identity map is wrong.
- Orbiting selects an old object: a stale asynchronous pick is accepted.

## What changed

```text
lessons/12/src/
├── app/
│   ├── walk/
│   │   ├── bounds.rs
│   │   ├── brep.rs
│   │   ├── brep_edges.rs
│   │   ├── brep_orient.rs
│   │   ├── cloud.rs  +
│   │   ├── curves.rs
│   │   ├── encode.rs
│   │   ├── frames.rs  +
│   │   ├── mesh.rs
│   │   ├── mesh_ink.rs
│   │   ├── mesh_topology.rs
│   │   ├── mod.rs  ~
│   │   └── points.rs  +
│   ├── feedback.rs  +
│   ├── input.rs  +
│   ├── inspection.rs  +
│   ├── knobs.rs
│   ├── loader.rs  +
│   ├── mod.rs  ~
│   ├── route.rs  ~
│   ├── scene.rs  +
│   ├── selection.rs  +
│   ├── stream.rs  +
│   └── touch.rs  +
├── engine/
│   ├── gpu/
│   │   ├── arena.rs
│   │   ├── backdrop.rs
│   │   ├── buffers.rs
│   │   ├── cloud.rs
│   │   ├── device.rs  +
│   │   ├── frame.rs
│   │   ├── glyphs.rs
│   │   ├── instance.rs
│   │   ├── lod.rs
│   │   ├── mod.rs  ~
│   │   ├── objects.rs
│   │   ├── pick.rs  +
│   │   ├── present.rs  +
│   │   ├── render.rs  +
│   │   ├── segments.rs
│   │   ├── selection_outline.rs  +
│   │   ├── splat.rs
│   │   ├── targets.rs
│   │   ├── text.rs
│   │   ├── text_outline.rs
│   │   ├── text_plane.rs
│   │   ├── text_plate.rs
│   │   ├── upload.rs
│   │   └── view.rs
│   ├── pipelines/
│   │   ├── layouts.rs
│   │   └── mod.rs
│   ├── mod.rs  ~
│   ├── performance.rs
│   └── text.rs
├── shaders/
│   ├── background.wgsl
│   ├── glyph.wgsl
│   ├── grid.wgsl
│   ├── ink_visibility.wgsl
│   ├── normals.wgsl
│   ├── physical.wgsl
│   ├── ribbon.wgsl
│   ├── scene.wgsl
│   ├── selection_outline.wgsl  +
│   ├── sphere.wgsl
│   ├── splat.wgsl
│   ├── splat_resolve.wgsl
│   ├── text_outline.wgsl
│   ├── text_plane.wgsl
│   ├── text_plate.wgsl
│   └── triangle.wgsl
├── camera.rs
├── lib.rs  ~
└── state.rs  +
```

`+` new in this lesson · `~` changed in this lesson

Every file at this point: `lessons/12/`.

## Next

[13 · Source controls](13-controls.md)

## Expected viewer result

Checkpoint 12: the seven-object interaction fixture in the production shell, nothing selected.

[![Full viewer result for 12 picking](screenshots/12.png)](screenshots/12.png)
