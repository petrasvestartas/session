# 18 · Finite-triangle visibility

Ink hides behind a face only where a triangle really covers it, not behind that triangle's plane extended past its edges. Every triangle is projected once per camera and binned into small screen tiles, so an ink pixel tests only the few triangles of its own tile.

![The depth plane continues beyond the finite triangle; only a finite nearer hit can hide the axis.](illustrations/finite-triangle.svg)

## Step 1 · src/shaders/slot_table.wgsl

Turn a triangle id into the triangle and the row it draws with; with no instances every id is an arena triangle.

`lessons/18/src/shaders/slot_table.wgsl` · type this, new file

```wgsl
--8<-- "lessons/18/src/shaders/slot_table.wgsl:slot-table"
```

## Step 2 · src/shaders/project_triangles.wgsl

The camera, view size, object rows and mesh buffers the projection reads, and the table it writes.

`lessons/18/src/shaders/project_triangles.wgsl` · type this, new file

```wgsl
--8<-- "lessons/18/src/shaders/project_triangles.wgsl:project-inputs"
```

## Step 3 · src/shaders/project_triangles.wgsl

One invocation per triangle id stores its edge equations, depth slope, nearest depth and screen box.

`lessons/18/src/shaders/project_triangles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/project_triangles.wgsl:project-main"
```

## Step 4 · src/shaders/project_triangles.wgsl

Clip a triangle to the near plane into at most four screen corners; hidden and cut-away triangles give none.

`lessons/18/src/shaders/project_triangles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/project_triangles.wgsl:project-polygon"
```

## Step 5 · src/shaders/project_triangles.wgsl

Paste in the slot table, the clipping test and the projected triangle record.

`lessons/18/src/shaders/project_triangles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/project_triangles.wgsl:project-includes"
```

## Step 6 · src/shaders/triangle_tiles.wgsl

The projected triangles, the tile records with their atomic counters, and what the vertex stage hands on.

`lessons/18/src/shaders/triangle_tiles.wgsl` · type this, new file

```wgsl
--8<-- "lessons/18/src/shaders/triangle_tiles.wgsl:tiles-inputs"
```

## Step 7 · src/shaders/triangle_tiles.wgsl

One quad per triangle over the tiles its screen box touches, in a target with one pixel per tile.

`lessons/18/src/shaders/triangle_tiles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/triangle_tiles.wgsl:tiles-vertex"
```

## Step 8 · src/shaders/triangle_tiles.wgsl

Discard a tile the triangle misses entirely, otherwise return that tile's record.

`lessons/18/src/shaders/triangle_tiles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/triangle_tiles.wgsl:tiles-cover"
```

## Step 9 · src/shaders/triangle_tiles.wgsl

The first binning pass counts one triangle for every tile it covers.

`lessons/18/src/shaders/triangle_tiles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/triangle_tiles.wgsl:tiles-count"
```

## Step 10 · src/shaders/triangle_tiles.wgsl

The second pass writes the triangle and its nearest depth in this tile, or marks the tile overflowed.

`lessons/18/src/shaders/triangle_tiles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/triangle_tiles.wgsl:tiles-fill"
```

## Step 11 · src/shaders/scan_triangle_tiles.wgsl

The tile records and the workgroup's scratch array for the prefix sum.

`lessons/18/src/shaders/scan_triangle_tiles.wgsl` · type this, new file

```wgsl
--8<-- "lessons/18/src/shaders/scan_triangle_tiles.wgsl:scan-inputs"
```

## Step 12 · src/shaders/scan_triangle_tiles.wgsl

A prefix sum over 256 values inside one workgroup, capped at the buffer size instead of wrapping.

`lessons/18/src/shaders/scan_triangle_tiles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/scan_triangle_tiles.wgsl:scan-prefix"
```

## Step 13 · src/shaders/scan_triangle_tiles.wgsl

Three passes turn the tile counts into list starts in the pool and report the words needed.

`lessons/18/src/shaders/scan_triangle_tiles.wgsl` · type this, append at the end of the file

```wgsl
--8<-- "lessons/18/src/shaders/scan_triangle_tiles.wgsl:scan-passes"
```

## Step 14 · src/engine/gpu/triangle_tiles.rs

The tile grid for a canvas: tiles start at 4 px and double until at most 262,144 fit.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, new file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tile-layout"
```

## Step 15 · src/engine/gpu/triangle_tiles.rs

Read back, a few frames later, how many pool words the last scan needed.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:pool-report"
```

## Step 16 · src/engine/gpu/triangle_tiles.rs

Size the dispatch, the projected table and the pool, which doubles on overflow and stops at a ceiling.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:pool-sizing"
```

## Step 17 · src/engine/gpu/triangle_tiles.rs

The projection key, the tile pipelines, and TriangleTiles, which owns the tables.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tile-struct"
```

## Step 18 · src/engine/gpu/triangle_tiles.rs

Open `impl TriangleTiles`: size the buffers for the canvas and triangle count, true when one moved.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tiles-prepare"
```

## Step 19 · src/engine/gpu/triangle_tiles.rs

Project when camera or objects changed, then clear, count, scan and fill the tile lists.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tiles-encode"
```

## Step 20 · src/engine/gpu/triangle_tiles.rs

Free the tables when nothing reads them and count their bytes; the brace closes the impl.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tiles-release"
```

## Step 21 · src/engine/gpu/triangle_tiles.rs

What the arena hands the encoder, and a helper for one buffer binding.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tile-input"
```

## Step 22 · src/engine/gpu/triangle_tiles.rs

The projection and scan compute pipelines, compiled on first use, and the count and fill raster pipelines.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tile-pipelines"
```

## Step 23 · src/engine/gpu/triangle_tiles.rs

Tests: the shaders validate, dispatches stay in limits, the pool grows and stops, and lists rebuild only when needed.

`lessons/18/src/engine/gpu/triangle_tiles.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/triangle_tiles.rs:tile-tests"
```

## Step 24 · src/engine/gpu/arena.rs

The arena owns the tile tables and creates them with its buffers.

`lessons/18/src/engine/gpu/arena.rs` · type the line tagged `register:tiles` at the top of `struct ArenaLane`

```rust
--8<-- "lessons/18/src/engine/gpu/arena.rs:tiles-field"
```

`lessons/18/src/engine/gpu/arena.rs` · type the line tagged `register:tiles` in `ArenaLane::new`

```rust
--8<-- "lessons/18/src/engine/gpu/arena.rs:tiles-new"
```

## Step 25 · src/engine/gpu/arena.rs

A second `impl ArenaLane` block hands its vertex, row and index buffers and the slot table to the tiles.

`lessons/18/src/engine/gpu/arena.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/arena.rs:arena-visibility"
```

## Step 26 · src/engine/gpu/render.rs

Before the faces, build the tables only when strokes read them, drop the lists during a slow drag, and free unread ones.

`lessons/18/src/engine/gpu/render.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/render.rs:tile-passes"
```

## Step 27 · src/engine/gpu/render.rs

Tests: a pick of faces alone reads no tables, and every stroke a pick draws does.

`lessons/18/src/engine/gpu/render.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/render.rs:tile-tests"
```

## Step 28 · src/engine/gpu/instance.rs

Tests: every lane shader validates and its structs, the projected triangle included, match the Rust layout.

`lessons/18/src/engine/gpu/instance.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/18/src/engine/gpu/instance.rs:shader-tests"
```

## Step 29 · src/engine/gpu/surface_outline.rs

A slow drag tests edges against the fitted planes alone, so the outline mask remembers which way it was drawn.

`lessons/18/src/engine/gpu/surface_outline.rs` · type the line tagged `register:tiles` in `struct MaskKey`

```rust
--8<-- "lessons/18/src/engine/gpu/surface_outline.rs:rough-key"
```

`lessons/18/src/engine/gpu/surface_outline.rs` · type the line tagged `register:tiles` where `after_faces` builds the key

```rust
--8<-- "lessons/18/src/engine/gpu/surface_outline.rs:rough-frame"
```

## Step 30 · examples and tests

Copy these files from `lessons/18/`; they are checked, not explained.

- `examples/mk_triangle_visibility.rs`: a seam that a nearby strip must not hide and a wide strip must.
- `tests/triangle-visibility.py`: every exposed seam pixel shows and no hidden ink does.

## Step 31 · registration lines

Copy the lines tagged `register:triangle_tiles` and `register:tiles` from these files of `lessons/18/`:

- `src/engine/gpu/mod.rs`: the `triangle_tiles` module and the tiles in the ink bind group.
- `src/engine/gpu/pass.rs`: the `rough` flag in `Frame`.
- `src/engine/gpu/render.rs`: the tile passes before the faces, `rough` in the frame, the tables before a stroke pick, and the tiles in the ink binds.
- `src/engine/gpu/objects.rs`: the projected triangles and the tile lists in the ink bind group.
- `src/engine/gpu/present.rs`: the pool report mapped after each submit.
- `src/engine/gpu/arena.rs`: the lists invalidated on every append, patch, kill and reset, freed on release, and counted in the bytes.

Run `cargo check` in `lessons/18/`.

## Check

`cargo check` compiles, and `cargo xtest --lib triangle_tiles` passes the pool and grid tests. With `trunk serve` in `lessons/18/`, a mesh edge right beside a nearer face stays visible, and a strip that really covers it hides it.
