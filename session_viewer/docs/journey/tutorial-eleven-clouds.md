# Tutorial 11 · Point clouds, octree LOD and sheets

## Goal

Traverse precomputed `LodNode` point-cloud octrees with 8-corner clip-space frustum culling and `2.0 px` screen-spacing cutoffs, shade splats with Eye-Dome Lighting (EDL), stream `250k`-point chunks, and query cloud/sheet elements lazily.

## Diagram

![Octree frustum culling and `lod_px = 2.0` cutoffs (`lod.rs`), screen-disc splats + EDL resolve, `250k` streamed pages, and cancellable queries (native wgpu readback of stage deck)](../../examples/stage/tutorials-interaction/11-clouds.png)

Stage deck source: [`11-clouds.yaml`](../../examples/stage/tutorials-interaction/11-clouds.yaml).

## Prerequisites and files

Requires [Tutorial 10](tutorial-ten-display.md). Owns `engine/gpu/{cloud,lod,splat,splat/cloud_witness*}.rs`, `shaders/{splat,splat_resolve}.wgsl`, `walk/{cloud*,sheet}.rs`, and `{viewer-core,app,state}/{cloud_query,sheet_query}.rs`.

Owned files in build order (`17` files):

1. `src/engine/gpu/cloud.rs`
2. `src/engine/gpu/lod.rs`
3. `src/engine/gpu/splat.rs`
4. `src/shaders/splat.wgsl`
5. `src/shaders/splat_resolve.wgsl`
6. `viewer-core/src/walk/cloud.rs`
7. `viewer-core/src/walk/cloud/resident.rs`
8. `viewer-core/src/walk/cloud/stream.rs`
9. `viewer-core/src/cloud_query.rs`
10. `src/app/cloud_query.rs`
11. `src/state/cloud_query.rs`
12. `viewer-core/src/walk/sheet.rs`
13. `viewer-core/src/sheet_query.rs`
14. `src/app/sheet_query.rs`
15. `src/state/sheet_query.rs`
16. `src/engine/gpu/splat/cloud_witness.rs`
17. `src/engine/gpu/splat/cloud_witness/lod.rs`

## Build

### Step 1 · `src/engine/gpu/cloud.rs`

Maps source point intervals to GPU `Chunk` offsets (`row_of` -> `None` when non-resident) with independent octree and normal bases. Spanning `323` lines, this module stays free of duplicate state by exposing `Chunk`, `row_of`, `Cloud`, `PointBufs` without introducing UI/GPU coupling cycles. Spanning `323` lines, this module stays free of duplicate state by exposing `Chunk`, `row_of`, `Cloud`, `PointBufs` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/cloud.rs"
```

### Step 2 · `src/engine/gpu/lod.rs`

Traverses resident octree nodes against projected pixel spacing, camera distance and scale, clamping selected spans to uploaded residency. Spanning `315` lines, this module stays free of duplicate state by exposing `Range`, `Projection`, `LodWalk`, `select` and verified by `small_clouds_and_disabled_lod_draw_single_full_run` (+1 more tests). Spanning `315` lines, this module stays free of duplicate state by exposing `Range`, `Projection`, `LodWalk`, `select` and verified by `small_clouds_and_disabled_lod_draw_single_full_run` (+1 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/lod.rs"
```

### Step 3 · `src/engine/gpu/splat.rs`

Emits `160 B` (`40`-word) splat draw records cached by camera/LOD/revision key into dedicated `1×` colour and reverse-Z depth targets. Spanning `602` lines, this module stays free of duplicate state by exposing `MAX_RECORDS`, `SplatRecord`, `RecordCx`, `Splat` without introducing UI/GPU coupling cycles. Spanning `602` lines, this module stays free of duplicate state by exposing `MAX_RECORDS`, `SplatRecord`, `RecordCx`, `Splat` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/splat.rs"
```

### Step 4 · `src/shaders/splat.wgsl`

2. **`src/shaders/splat.wgsl`** and **`src/shaders/splat_resolve.wgsl`** binary-search `40`-word octree draw runs and fold nearest-point `sdepth` + `scolor` into the scene depth buffer with logarithmic Eye-Dome Lighting: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `259`-line shader defines `@vertex fn vs_point`, `@fragment fn fs_point`, `@fragment fn fs_point_id`, `@vertex fn vs_point_aa` after `#include` expansion of `clip.wgsl`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `259`-line shader defines `@vertex fn vs_point`, `@fragment fn fs_point`, `@fragment fn fs_point_id`, `@vertex fn vs_point_aa` after `#include` expansion of `clip.wgsl`.

```wgsl
--8<-- "session_viewer/src/shaders/splat.wgsl"
```

### Step 5 · `src/shaders/splat_resolve.wgsl`

2. **`src/shaders/splat.wgsl`** and **`src/shaders/splat_resolve.wgsl`** binary-search `40`-word octree draw runs and fold nearest-point `sdepth` + `scolor` into the scene depth buffer with logarithmic Eye-Dome Lighting: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `151`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main`, `@fragment fn fs_main_aa` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `151`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_main`, `@fragment fn fs_main_aa` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/splat_resolve.wgsl"
```

### Step 6 · `viewer-core/src/walk/cloud.rs`

Declares the additive module boundary re-exporting `walk_cloud` now and registering `stream.rs` in checkpoint `application`. Spanning `4` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `4` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/cloud.rs"
```

### Step 7 · `viewer-core/src/walk/cloud/resident.rs`

Packs kernel `PointCloud` positions, colours and complete normal arrays (`NO_NORMALS` when absent) with file-level `point_size` override. Spanning `139` lines, this module stays free of duplicate state by exposing `DEFAULT_SPACING`, `walk_cloud` without introducing UI/GPU coupling cycles. Spanning `139` lines, this module stays free of duplicate state by exposing `DEFAULT_SPACING`, `walk_cloud` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/cloud/resident.rs"
```

### Step 8 · `viewer-core/src/walk/cloud/stream.rs`

`src/app/walk/cloud/stream.rs` appends streamed position, colour (black-padded when absent) and complete normal slices into GPU chunks, installing the octree table once on the first slice while preserving `(source_offset, gpu_offset)`. Spanning `107` lines, this module stays free of duplicate state by exposing `StreamRows`, `StreamSlice`, `walk_stream_slice` without introducing UI/GPU coupling cycles. Spanning `107` lines, this module stays free of duplicate state by exposing `StreamRows`, `StreamSlice`, `walk_stream_slice` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/cloud/stream.rs"
```

### Step 9 · `viewer-core/src/cloud_query.rs`

`src/app/cloud_query.rs` narrows out-of-core click candidates via octree LOD bounds (falling back to full scan if incomplete), streams `<= 65,536`-point pages through GPU pick ranking, and reads exact stored `u32` source IDs. Spanning `503` lines, this module stays free of duplicate state by exposing `PAGE_POINTS`, `original_id`, `QueryView`, `project` and verified by `original_fixed32_ids_preserve_all_bits_and_reject_partial_ranges` (+5 more tests). Spanning `503` lines, this module stays free of duplicate state by exposing `PAGE_POINTS`, `original_id`, `QueryView`, `project` and verified by `original_fixed32_ids_preserve_all_bits_and_reject_partial_ranges` (+5 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/cloud_query.rs"
```

### Step 10 · `src/app/cloud_query.rs`

`src/app/cloud_query.rs` narrows out-of-core click candidates via octree LOD bounds (falling back to full scan if incomplete), streams `<= 65,536`-point pages through GPU pick ranking, and reads exact stored `u32` source IDs. Spanning `174` lines, this module stays free of duplicate state by exposing `fetch_page`, `resolve_id` without introducing UI/GPU coupling cycles. Spanning `174` lines, this module stays free of duplicate state by exposing `fetch_page`, `resolve_id` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/cloud_query.rs"
```

### Step 11 · `src/state/cloud_query.rs`

Assigns a monotonic generation to streamed-cloud control picks, ranking `65,536`-point pages on the GPU and discarding results if camera or selection changes. Spanning `262` lines, this module stays free of duplicate state by exposing `cloud_query_awaiting_gpu`, `streamed_slot`, `cancel_cloud_query`, `start_cloud_query` without introducing UI/GPU coupling cycles. Spanning `262` lines, this module stays free of duplicate state by exposing `cloud_query_awaiting_gpu`, `streamed_slot`, `cancel_cloud_query`, `start_cloud_query` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/cloud_query.rs"
```

### Step 12 · `viewer-core/src/walk/sheet.rs`

Walks flat `6`-float segment slices into the dedicated sheet lane with scene-unit half-widths and preserved `(source_offset, gpu_row)` spans. Spanning `109` lines, this module stays free of duplicate state by exposing `SheetRows`, `SheetSlice`, `walk_sheet_slice` and verified by `sheet_slice_pads_short_columns_and_reports_its_box`. Spanning `109` lines, this module stays free of duplicate state by exposing `SheetRows`, `SheetSlice`, `walk_sheet_slice` and verified by `sheet_slice_pads_short_columns_and_reports_its_box`.

```rust
--8<-- "session_viewer/viewer-core/src/walk/sheet.rs"
```

### Step 13 · `viewer-core/src/sheet_query.rs`

`src/app/sheet_query.rs` validates the `SHM1` side-table header, fixed-width `(offset, len)` directory and bounded UTF-8 JSON payloads pinned to the table's `ETag` revision. Spanning `161` lines, this module stays free of duplicate state by exposing `HEAD_BYTES`, `RECORD_BYTES`, `MAX_BLOB`, `EntityMeta` and verified by `side_table_head_records_and_blobs_parse_from_a_hand_built_buffer` (+1 more tests). Spanning `161` lines, this module stays free of duplicate state by exposing `HEAD_BYTES`, `RECORD_BYTES`, `MAX_BLOB`, `EntityMeta` and verified by `side_table_head_records_and_blobs_parse_from_a_hand_built_buffer` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/sheet_query.rs"
```

### Step 14 · `src/app/sheet_query.rs`

`src/app/sheet_query.rs` validates the `SHM1` side-table header, fixed-width `(offset, len)` directory and bounded UTF-8 JSON payloads pinned to the table's `ETag` revision. Spanning `96` lines, this module stays free of duplicate state by exposing `fetch_entity`, `read_entity` without introducing UI/GPU coupling cycles. Spanning `96` lines, this module stays free of duplicate state by exposing `fetch_entity`, `read_entity` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/sheet_query.rs"
```

### Step 15 · `src/state/sheet_query.rs`

Highlights picked sheet entities immediately on the parent row and resolves optional `.meta` side-table names under a generation token. Spanning `86` lines, this module stays free of duplicate state by exposing `apply_sheet_pick`, `sheet_entity` without introducing UI/GPU coupling cycles. Spanning `86` lines, this module stays free of duplicate state by exposing `apply_sheet_pick`, `sheet_entity` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/sheet_query.rs"
```

### Step 16 · `src/engine/gpu/splat/cloud_witness.rs`

Verifies the CPU DX12 `Microsoft Basic Render Driver` (`WARP`) before capturing cloud splat/EDL pixels, uploaded byte counts and shader text. Spanning `141` lines, this module stays free of duplicate state by exposing `Frame`, `render` without introducing UI/GPU coupling cycles. Spanning `141` lines, this module stays free of duplicate state by exposing `Frame`, `render` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/splat/cloud_witness.rs"
```

### Step 17 · `src/engine/gpu/splat/cloud_witness/lod.rs`

Runs `LodWalk::select` on CPU cloud metadata to inspect chosen octree `(first, count, spacing, tile)` spans independently of shaders. Spanning `85` lines, this module stays free of duplicate state by exposing `Prepared`, `metadata`, `select` without introducing UI/GPU coupling cycles. Spanning `85` lines, this module stays free of duplicate state by exposing `Prepared`, `metadata`, `select` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/splat/cloud_witness/lod.rs"
```

## Integration

`splat.rs` activates `lod_px = 2.0` automatically for clouds with `>= 2,000,000` points (overridable via `?lod=`), logs a one-time warning if draw runs hit `MAX_RECORDS`, and invalidates cached draw records whenever row visibility or camera frustum changes.

## Run and observe

Run `cargo xtest -p session_viewer engine::gpu::lod::tests`, `segments::tests::sheet_chunks_map_global_ribbon_rows_to_segments_and_ids`, and `cargo xtest -p viewer-core cloud_query::tests` + `sheet_query::tests` + `walk::sheet::tests`: all pass.

## Predict and try

Predict what happens when you zoom out on a `20,000,000`-point lidar scan with `lod_px = 2.0` vs `lod_px = 0.0`: `lod::select_runs` halts recursion at coarse octree nodes as soon as projected point spacing drops below `2.0 px` and rejects off-screen octants via 8-corner clip outcodes, cutting splat vertex invocations by orders of magnitude (`octree_lod_splits_coarse_nodes_propagates_spacing_and_culls_outside_frustum`).

## Question and answer

**How does `splat_resolve.wgsl` give depth cues and surface relief on uncoloured or normal-free lidar point clouds without computing k-nearest-neighbour normals?**

**How to work it out:** Examine the inputs available in `splat_resolve.wgsl` after `splat.wgsl` rasterizes camera-facing point discs into the colour and linear/reverse-Z depth buffers.

**The answer:** Eye-Dome Lighting (EDL) samples neighboring texels' log-depths around each splat pixel and darkens silhouettes where depth jumps abruptly toward the background, synthesizing crisp ambient-like contour shading in a single screen-space pass with zero normal vectors stored per point.

## Validation

Verified via `lod::tests` (`2/2` octree split, spacing propagation, and frustum culling tests), `cloud_query::tests` (`5/5` cancellation and cube-eligibility tests), and `sheet_query::tests` (`2/2`).
