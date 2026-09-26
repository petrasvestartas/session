# 15 · Publication and streamed reads

A point cloud too large to load whole is streamed: an 8 KB probe finds its arrays, then byte-range reads bring 2 million points at a time through the range gate. A click on a streamed cloud reads exact positions back from the file, page by page.

![The file is small fields between huge arrays; the window fetches the small fields once and skips the arrays by length.](illustrations/metadata-window.svg)

## Step 1 · src/app/stream.rs

Where a cloud's and a sheet's arrays sit in the file, and the ETag every read must match.

`lessons/15/src/app/stream.rs` · type this, new file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-fields"
```

## Step 2 · src/app/stream.rs

Parse one field header, and record a sheet's fields that follow its coordinates.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-field-at"
```

## Step 3 · src/app/stream.rs

The cloud's octree table and its checks: equal array lengths, finite cubes, and one parent per child.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-lod"
```

## Step 4 · src/app/stream.rs

Find a cloud's or a sheet's coordinates in the first 8 KB; a file that shows neither loads whole.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-layout"
```

## Step 5 · src/app/stream.rs

Checks for doubles and ranges, and a 64 KiB window that answers many small header reads.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-checks"
```

## Step 6 · src/app/stream.rs

Decode packed arrays: varints, doubles, floats, positions, octahedral normals and colours.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-packed"
```

## Step 7 · src/app/stream.rs

Open the browser-only module: the scene's range gate, and one gated range read retried once.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-web"
```

## Step 8 · src/app/stream.rs

Probe the first 8 KB of a file, and find a cloud's positions and colours from it.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-probe"
```

## Step 9 · src/app/stream.rs

Read the octree table after the colours, and note where the normals and original ids lie.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-lod-read"
```

## Step 10 · src/app/stream.rs

Fetch the positions, normals and colours of one slice; the brace closes the module.

`lessons/15/src/app/stream.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-arrays"
```

## Step 11 · src/app/stream.rs

Tests: packed arrays, the header window, octree checks, safe offsets, varints and the sheet layout.

`lessons/15/src/app/stream.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/15/src/app/stream.rs:stream-tests"
```

## Step 12 · src/app/walk/cloud.rs

Append one streamed slice to the cloud rows; the first slice also brings the octree.

`lessons/15/src/app/walk/cloud.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/walk/cloud.rs:stream-slice"
```

## Step 13 · src/app/scene.rs

A streamed cloud's first slice, and its slot in the scene.

`lessons/15/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/scene.rs:streamed-types"
```

## Step 14 · src/app/scene.rs

Add a streamed cloud from its first slice and grow it with each later one, within the page's point ceiling.

`lessons/15/src/app/scene.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/scene.rs:streamed-scene"
```

## Step 15 · src/app/scene.rs

Test: the points still expected are what the files hold, never past the page's ceiling.

`lessons/15/src/app/scene.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/15/src/app/scene.rs:stream-tests"
```

## Step 16 · src/app/loader.rs

Start a cloud by range: its first share of points is posted now, or staged for a reload.

`lessons/15/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/loader.rs:stream-start"
```

## Step 17 · src/app/loader.rs

Read a cloud's first slice: fields, octree, then positions, colours and normals.

`lessons/15/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/loader.rs:stream-prefix"
```

## Step 18 · src/app/loader.rs

Keep reading slices in the background until the cloud is complete, the budget is spent or the scene is gone.

`lessons/15/src/app/loader.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/loader.rs:stream-rest"
```

## Step 19 · src/state.rs

State adds a streamed cloud and each later slice, and grows the camera's extent to fit.

`lessons/15/src/state.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/state.rs:stream-state"
```

## Step 20 · src/lib.rs

The `CloudChunk` message, and starting the background reads when a cloud's first slice arrives.

`lessons/15/src/lib.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/lib.rs:cloud-stream"
```

## Step 21 · src/app/cloud_query.rs

The page size of a point query, and a point's original id from its four stored bytes.

`lessons/15/src/app/cloud_query.rs` · type this, new file

```rust
--8<-- "lessons/15/src/app/cloud_query.rs:query-id"
```

## Step 22 · src/app/cloud_query.rs

The click and its camera: project a point to pixels, and keep every octree cube that may reach the click.

`lessons/15/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/cloud_query.rs:query-view"
```

## Step 23 · src/app/cloud_query.rs

Clip-space tests, and helpers for a cube's corners and a pixel box.

`lessons/15/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/cloud_query.rs:query-clip"
```

## Step 24 · src/app/cloud_query.rs

Merge the point ranges of every octree node near the click.

`lessons/15/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/cloud_query.rs:query-ranges"
```

## Step 25 · src/app/cloud_query.rs

One pick, read page by page and cancelled through a shared flag when dropped, and the answers it posts.

`lessons/15/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/cloud_query.rs:query-pages"
```

## Step 26 · src/app/cloud_query.rs

The browser half: read each page, keep the points near the click, then read the winner's id and position.

`lessons/15/src/app/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/app/cloud_query.rs:query-web"
```

## Step 27 · src/app/cloud_query.rs

Tests: ids keep 32 bits, far nodes stay eligible, pages cover every row, and a drop cancels.

`lessons/15/src/app/cloud_query.rs` · copy, append at the end of the file

```rust
--8<-- "lessons/15/src/app/cloud_query.rs:query-tests"
```

## Step 28 · src/state/cloud_query.rs

Open `impl State` in its own file: the waiting flag, the streamed slot of a row, and cancel.

`lessons/15/src/state/cloud_query.rs` · type this, new file

```rust
--8<-- "lessons/15/src/state/cloud_query.rs:query-state"
```

## Step 29 · src/state/cloud_query.rs

Start a query on a click in cloud control mode, and fetch pages until the best point is known.

`lessons/15/src/state/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/state/cloud_query.rs:query-start"
```

## Step 30 · src/state/cloud_query.rs

Draw each arrived page as dots for the GPU to pick, and keep the pick.

`lessons/15/src/state/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/state/cloud_query.rs:query-batch"
```

## Step 31 · src/state/cloud_query.rs

Select the winning point once its id and position arrive; the brace closes the impl.

`lessons/15/src/state/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/state/cloud_query.rs:query-resolved"
```

## Step 32 · src/state/cloud_query.rs

The hooks that hand a query the click and the GPU's answer, and drop it when the readback fails.

`lessons/15/src/state/cloud_query.rs` · type this, append at the end of the file

```rust
--8<-- "lessons/15/src/state/cloud_query.rs:query-hooks"
```

## Step 33 · tests

Copy `tests/publication.py` and `tests/streamed-controls.cjs` from `lessons/15/`; they are checked, not explained.

## Step 34 · registration lines

Copy the lines tagged `register:stream` and `register:cloud_query` from these files of `lessons/15/`:

- `src/app/mod.rs`: the modules `cloud_query` and `stream`.
- `src/app/loader.rs`: reset the range gate on clear, probe each plain `.pb` file, stage and post a streamed cloud, and start it from `stream_item`.
- `src/app/scene.rs`: the streamed clouds and the point ceiling, their start values and clear, and the rows still expected.
- `src/lib.rs`: the stream and query messages and their handlers.
- `src/state.rs`: the `cloud_query` module, cancelling a query when the view or selection changes, and starting one on a click.
- `src/state/features.rs`: the query in flight, its generation counter and its pick hook.

Run `cargo check` in `lessons/15/`.

## Check

`cargo check` compiles, and `cargo xtest --lib stream` and `cargo xtest --lib cloud_query` pass. The local scene looks the same; on `?scene=scenes/view_pointclouds.yaml` the clouds appear 2 million points at a time, and the network panel shows range requests.
