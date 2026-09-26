# 25 · Draw a solid, readable gumball

Lesson 21 made the gumball's handles answer the pointer; this lesson draws them. Solid arrows, rings and balls go into a small tile of their own, which is blended over the frame.

![The gumball mesh is uploaded once, drawn into its own tile at 4x, resolved and sampled over the frame.](illustrations/extend-gumball.svg)

## Step 1 · registration lines

The widget is one more lane, so resize, reset and the byte count reach it.

`lessons/25/src/engine/gpu/mod.rs` · type the line tagged `register:gumball`

```rust
--8<-- "lessons/25/src/engine/gpu/mod.rs:lane-list"
```

Copy the other lines tagged `register:gumball`, `register:widget` and `register:widget_mesh` from these files of `lessons/25/`:

- `src/engine/gpu/mod.rs`: the `widget` and `widget_mesh` modules, the `widget` field of `Gpu`, and building it.
- `src/engine/gpu/render.rs`: the widget drawn after everything else, with its own depth.
- `src/engine/gpu/present.rs`: `prepare_widget` with the frame uniforms.
- `src/state.rs`: `upload_gizmo` before each frame.
- `src/app/input.rs`: `hover_gizmo` when the pointer moves.
- `src/state/edit.rs` and `src/state/number_box.rs`: `upload_gizmo` wherever the gizmo moves or changes handle.
- `src/app/inspection.rs`: the widget's placement, highlight and bytes in the snapshot.

## Step 2 · src/engine/gpu/widget_mesh.rs

One gumball vertex, in CSS pixels from the centre with a colour and a handle index, and the shape sizes.

`lessons/25/src/engine/gpu/widget_mesh.rs` · type this, new file

```rust
--8<-- "lessons/25/src/engine/gpu/widget_mesh.rs:widget-vertex"
```

## Step 3 · src/engine/gpu/widget_mesh.rs

The mesh: per axis a lathed arrow, a scale ball and a quarter ring, then the grey hub.

`lessons/25/src/engine/gpu/widget_mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget_mesh.rs:widget-mesh"
```

## Step 4 · src/engine/gpu/widget_mesh.rs

Shape helpers: turn onto an axis, spin a profile, a sphere, and any parametric surface as triangles.

`lessons/25/src/engine/gpu/widget_mesh.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget_mesh.rs:widget-shapes"
```

## Step 5 · src/engine/gpu/widget_mesh.rs

Test: every vertex is finite and within the arm, and all ten handles are present.

`lessons/25/src/engine/gpu/widget_mesh.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget_mesh.rs:widget-mesh-tests"
```

## Step 6 · src/shaders/widget.wgsl

The 96-byte uniform, and the tile texture the composite samples.

`lessons/25/src/shaders/widget.wgsl` · type this, new file

```wgsl
--8<-- "lessons/25/src/shaders/widget.wgsl:widget-uniform"
```

## Step 7 · src/shaders/widget.wgsl

Draw the mesh into the tile, the active handle in orange.

`lessons/25/src/shaders/widget.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/25/src/shaders/widget.wgsl:widget-mesh-shader"
```

## Step 8 · src/shaders/widget.wgsl

Stretch the tile over its rectangle on the canvas and blend it in.

`lessons/25/src/shaders/widget.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/25/src/shaders/widget.wgsl:widget-composite"
```

## Step 9 · src/engine/gpu/widget.rs

The `Widget`: the mesh, its uniform, both pipelines, and a tile made on demand.

`lessons/25/src/engine/gpu/widget.rs` · type this, new file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-struct"
```

## Step 10 · src/engine/gpu/widget.rs

Open `impl Widget`: upload the mesh and build both pipelines, then clear, retarget and count bytes.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-new"
```

## Step 11 · src/engine/gpu/widget.rs

Each frame, find the gumball's screen box, size the tile, and write the matrix that fills it.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-prepare"
```

## Step 12 · src/engine/gpu/widget.rs

Two passes, the mesh into the tile then the tile over the frame; `Drop` frees the buffers.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-draw"
```

## Step 13 · src/engine/gpu/widget.rs

The mesh pipeline, depth-tested, and the tile's format at 4x.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-pipeline"
```

## Step 14 · src/engine/gpu/widget.rs

The tile's colour, depth and resolved textures, and the layout of the texture the composite reads.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-tile"
```

## Step 15 · src/engine/gpu/widget.rs

The composite pipeline: no depth, the tile blended over the frame.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-composite"
```

## Step 16 · src/engine/gpu/widget.rs

The gumball's screen box, from the eight corners of its bounding cube.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-bounds"
```

## Step 17 · src/engine/gpu/widget.rs

The widget as a lane: retarget, reset and the byte count.

`lessons/25/src/engine/gpu/widget.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/widget.rs:widget-lane"
```

## Step 18 · src/engine/gpu/present.rs

A `Gpu` method that places the widget for this frame.

`lessons/25/src/engine/gpu/present.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/engine/gpu/present.rs:prepare-widget"
```

## Step 19 · src/state/edit.rs

Tell the GPU where the gizmo is and which handle lights up, and light the handle under the pointer.

`lessons/25/src/state/edit.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/25/src/state/edit.rs:upload-gizmo"
```

## Step 20 · src/state/edit.rs

GPU test: the widget changes the picture, with red, green and blue arms.

`lessons/25/src/state/edit.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/25/src/state/edit.rs:widget-test"
```

Run `cargo check` in `lessons/25/`.

## Check

`cargo check` compiles, and `cargo xtest --lib widget_mesh` passes. Select an object: red, green and blue arrows, quarter rings and balls sit at its centre, the same size at any zoom, and the handle under the pointer turns orange.
