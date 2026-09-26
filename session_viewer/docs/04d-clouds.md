# 04d · Point clouds

A point cloud of millions of points is drawn as discs into a texture of its own, then copied into the scene in one pass. An octree picks, per frame, only the points close enough to matter.

![A node whose spacing projects wider than lod_px descends into its eight children; one that fits draws whole.](illustrations/lod.svg)

## Step 1 · src/engine/gpu/cloud.rs

A cloud arrives in batches: the batch, the octree node, where points live on the GPU, and one upload's rows.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, new file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:cloud-rows"
```

## Step 2 · src/engine/gpu/cloud.rs

Open `impl CloudLane`: three growing buffers for positions, colours and normals, and a size check against the device limit.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:cloud-lane"
```

## Step 3 · src/engine/gpu/cloud.rs

Append a batch: grow to exactly the points announced, then open a cloud or extend one.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:cloud-append"
```

## Step 4 · src/engine/gpu/cloud.rs

Instant undo: kill or bury a cloud without touching its points, and compact the buffers later.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:cloud-undo"
```

## Step 5 · src/engine/gpu/cloud.rs

Find the cloud of a GPU row, lend the buffers, reset and release; the impl block closes.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:cloud-lookup"
```

## Step 6 · src/engine/gpu/cloud.rs

The `Lane` trait: this lane has buffers only, so it needs no retarget.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:cloud-trait"
```

## Step 7 · src/engine/gpu/cloud.rs

A second `impl CloudRows` block merges two uploads, shifting each batch's first rows.

`lessons/04d/src/engine/gpu/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/cloud.rs:cloud-merge"
```

## Step 8 · src/engine/gpu/lod.rs

Walk a cloud's octree each frame and keep the nodes whose points are close enough on screen.

`lessons/04d/src/engine/gpu/lod.rs` · type this, new file

```rust
--8<-- "lessons/04d/src/engine/gpu/lod.rs:lod-walk"
```

## Step 9 · src/engine/gpu/lod.rs

Point spacing on screen in pixels, and the disc radius factor handed to the shader.

`lessons/04d/src/engine/gpu/lod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/lod.rs:lod-spacing"
```

## Step 10 · src/engine/gpu/splat.rs

The 160-byte record of one run of points, and the key that skips a pass when nothing changed.

`lessons/04d/src/engine/gpu/splat.rs` · type this, new file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-record"
```

## Step 11 · src/engine/gpu/splat.rs

The point depth and colour textures, made when the first cloud arrives.

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-targets"
```

## Step 12 · src/engine/gpu/splat.rs

The splat drawer and its constructor: record buffer, bind group, point and resolve pipelines.

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-new"
```

## Step 13 · src/engine/gpu/splat.rs

Small setters: edited cloud, highlighted point, retarget, resize, rebind, release.

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-state"
```

## Step 14 · src/engine/gpu/splat.rs

The point pass into its own textures, the resolve into the scene, and the pick-id draw.

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-passes"
```

## Step 15 · src/engine/gpu/splat.rs

Build one record per run: walk the octree, clip each run to its uploaded chunks; the impl closes.

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-records"
```

## Step 16 · src/engine/gpu/splat.rs

Open the point pass, bind its buffers, and build the point and resolve pipelines.

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-pipelines"
```

## Step 17 · src/engine/gpu/splat.rs

The `Lane` trait, so release drops the point textures too.

`lessons/04d/src/engine/gpu/splat.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/splat.rs:splat-trait"
```

## Step 18 · src/shaders/splat.wgsl

The cloud uniform, the record table and the three point buffers.

`lessons/04d/src/shaders/splat.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04d/src/shaders/splat.wgsl:splat-bindings"
```

## Step 19 · src/shaders/splat.wgsl

Find a point's record by binary search, then project, clip, size, light and colour it.

`lessons/04d/src/shaders/splat.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04d/src/shaders/splat.wgsl:splat-project"
```

## Step 20 · src/shaders/splat.wgsl

Draw each point as a pixel-aligned square cut to a disc, in colour or as a pick id.

`lessons/04d/src/shaders/splat.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04d/src/shaders/splat.wgsl:splat-point"
```

## Step 21 · src/shaders/splat_resolve.wgsl

The resolve reads both point textures, drawn by one triangle that covers the screen.

`lessons/04d/src/shaders/splat_resolve.wgsl` · type this, new file

```wgsl
--8<-- "lessons/04d/src/shaders/splat_resolve.wgsl:resolve-bindings"
```

## Step 22 · src/shaders/splat_resolve.wgsl

Copy each point pixel into the scene with its depth, darkened at depth steps by eye-dome lighting.

`lessons/04d/src/shaders/splat_resolve.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/04d/src/shaders/splat_resolve.wgsl:resolve-shade"
```

## Step 23 · src/engine/gpu/mod.rs

Append a cloud upload, log the scene, rebind the point buffers, and count live points.

`lessons/04d/src/engine/gpu/mod.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/mod.rs:clouds"
```

## Step 24 · src/engine/gpu/render.rs

The point pass each frame, fed the camera, size and LOD settings.

`lessons/04d/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/render.rs:point-pass"
```

## Step 25 · src/engine/gpu/hull.rs

The extreme points of one upload, so a turned object keeps an exact bounding box.

`lessons/04d/src/engine/gpu/hull.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/04d/src/engine/gpu/hull.rs:hull-of"
```

## Step 26 · registration lines

Copy the lines tagged `register:cloud`, `register:clouds`, `register:lod` and `register:splat` from these files of `lessons/04d/`:

- `src/engine/gpu/mod.rs`: the modules, the cloud and splat fields, the dead point count, their creation, append and release.
- `src/engine/gpu/present.rs`, `upload.rs`, `render.rs`: the point count, the cloud rows of an upload, the point pass and the resolve draw.

Run `cargo check` in `lessons/04d/`.

## Check

Run `cargo check` in `lessons/04d/`: the lesson adds no test of its own; the clouds are drawn and picked from lesson 06 on, when the scene walk fills `CloudRows`.
