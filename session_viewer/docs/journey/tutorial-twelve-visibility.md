# Tutorial 12 · Transparency, finite-triangle tile visibility and section caps

## Goal

Bin projected scene triangles into screen tiles with prefix-sum offsets and `atomicMax` per-tile depth bounds for fast hidden-line stroke visibility, render global `Opacity` and per-object `Alpha`, and fill section-plane cuts with watertight stencil caps.

## Diagram

![`project_triangles` + `atomicMax` depth bound, `scan_triangle_tiles` prefix sum, `ink_visibility` early-out, and `clip.wgsl` / `cap.wgsl` section caps (native wgpu readback of stage deck)](../../examples/stage/tutorials-systems/12-visibility.png)

Stage deck source: [`12-visibility.yaml`](../../examples/stage/tutorials-systems/12-visibility.yaml).

## Prerequisites and files

Requires [Tutorial 11](tutorial-eleven-clouds.md) and builds on the shared `clip.wgsl` and `ink_visibility.wgsl` headers from [Tutorial 3](tutorial-three-rendering.md).

Owned files in build order (`14` files):

1. `src/engine/gpu/triangle_tiles.rs`
2. `src/shaders/project_triangles.wgsl`
3. `src/shaders/triangle_tiles.wgsl`
4. `src/shaders/scan_triangle_tiles.wgsl`
5. `src/engine/gpu/clip.rs`
6. `src/engine/gpu/clip/pipelines.rs`
7. `src/engine/gpu/clip/tests.rs`
8. `src/engine/gpu/clip/editing_tests.rs`
9. `src/shaders/cap.wgsl`
10. `viewer-core/src/clipping.rs`
11. `src/state/clipping.rs`
12. `viewer-core/src/command/verbs/clipping_plane.rs`
13. `viewer-core/src/command/verbs/opacity.rs`
14. `viewer-core/src/command/verbs/alpha.rs`

## Build

### Step 1 · `src/engine/gpu/triangle_tiles.rs`

Manages projected-triangle buffers, tile headers and the shared reference pool, polling async overflow readbacks to grow capacity up to ceiling. Spanning `1190` lines, this module stays free of duplicate state by exposing `PROJECTED_BYTES`, `TriangleTiles`, `prewarm`, `map_report` and verified by `tile_shaders_validate` (+7 more tests). Spanning `1190` lines, this module stays free of duplicate state by exposing `PROJECTED_BYTES`, `TriangleTiles`, `prewarm`, `map_report` and verified by `tile_shaders_validate` (+7 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/triangle_tiles.rs"
```

### Step 2 · `src/shaders/project_triangles.wgsl`

1. **`src/shaders/project_triangles.wgsl`** runs a `64`-thread compute pass (`PROJECT_ROW_GROUPS` `2D` dispatch above `65,535` groups) that resolves each triangle via `placed_triangle`, clips against the near plane, fits inward edge equations `equations[0..3]`, and chooses a local corner reference on small far triangles so homogeneous depth extrapolation never suffers cancellation: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `239`-line shader defines `@compute fn cs_main` after `#include` expansion of `slot_table.wgsl`, `clip.wgsl`, `projected_triangle.wgsl`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `239`-line shader defines `@compute fn cs_main` after `#include` expansion of `slot_table.wgsl`, `clip.wgsl`, `projected_triangle.wgsl`.

```wgsl
--8<-- "session_viewer/src/shaders/project_triangles.wgsl"
```

### Step 3 · `src/shaders/triangle_tiles.wgsl`

2. **`src/shaders/triangle_tiles.wgsl`** rasterizes one quad over each projected triangle's tile bounding box, discards uncovered tiles via `covered_tile`, atomically counts hits in `fs_count`, and packs `(primitive, nearest_depth_bound)` pairs in `fs_fill`: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `122`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_count`, `@fragment fn fs_fill` after `#include` expansion of `projected_triangle.wgsl`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `122`-line shader defines `@vertex fn vs_main`, `@fragment fn fs_count`, `@fragment fn fs_fill` after `#include` expansion of `projected_triangle.wgsl`.

```wgsl
--8<-- "session_viewer/src/shaders/triangle_tiles.wgsl"
```

### Step 4 · `src/shaders/scan_triangle_tiles.wgsl`

3. **`src/shaders/scan_triangle_tiles.wgsl`** executes a `256`-lane Hillis-Steele / Blelloch workgroup prefix sum (`scan_tiles` -> `scan_blocks` -> `finish_offsets`) between `fs_count` and `fs_fill`, saturating at buffer capacity instead of wrapping on overflow: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `135`-line shader defines `@compute fn scan_tiles`, `@compute fn scan_blocks`, `@compute fn finish_offsets` after `#include` expansion of `projected_triangle.wgsl`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `135`-line shader defines `@compute fn scan_tiles`, `@compute fn scan_blocks`, `@compute fn finish_offsets` after `#include` expansion of `projected_triangle.wgsl`.

```wgsl
--8<-- "session_viewer/src/shaders/scan_triangle_tiles.wgsl"
```

### Step 5 · `src/engine/gpu/clip.rs`

Tests placed object bounds against active half-spaces with precision slack and builds per-plane cap resources only for verified closed solids. Spanning `890` lines, this module stays free of duplicate state by exposing `ClipView`, `Clip`, `find_solids`, `set` without introducing UI/GPU coupling cycles. Spanning `890` lines, this module stays free of duplicate state by exposing `ClipView`, `Clip`, `find_solids`, `set` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/clip.rs"
```

### Step 6 · `src/engine/gpu/clip/pipelines.rs`

Specializes WGSL cap headers for `1×` vs `4×` MSAA count/ID textures and single-sample cap-picking passes. Spanning `208` lines, this module stays free of duplicate state by exposing `cap_source`, `pick_pipelines`, `pick_placed_pipelines`, `count_placed_pipeline` without introducing UI/GPU coupling cycles. Spanning `208` lines, this module stays free of duplicate state by exposing `cap_source`, `pick_pipelines`, `pick_placed_pipelines`, `count_placed_pipeline` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/clip/pipelines.rs"
```

### Step 7 · `src/engine/gpu/clip/tests.rs`

Parses WGSL via `naga` to verify `ClipUniform` byte offsets against `offset_of!` and tests closed-cube section hatches on GPU. Spanning `425` lines, this module stays free of duplicate state by exposing `view` and verified by `clip_uniform_mirror` (+9 more tests). Spanning `425` lines, this module stays free of duplicate state by exposing `view` and verified by `clip_uniform_mirror` (+9 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/clip/tests.rs"
```

### Step 8 · `src/engine/gpu/clip/editing_tests.rs`

Verifies live clipping-plane transforms, open-mesh non-capping and section restoration across interactive edits. Spanning `1146` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `1146` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/clip/editing_tests.rs"
```

### Step 9 · `src/shaders/cap.wgsl`

1. **`src/shaders/cap.wgsl`** evaluates plane depth `clip_plane_depth`, trims against every other active section plane sample-by-sample, tests whether back-minus-front crossings behind the plane exceed `0.5` (`count_at`), and outputs `45°` `box_cdf`-filtered hatches plus `CAP_PRIMITIVE`: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `198`-line shader defines `@vertex fn vs_cap`, `@fragment fn fs_cap`, `@fragment fn fs_cap_selection`, `@fragment fn fs_cap_id` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `198`-line shader defines `@vertex fn vs_cap`, `@fragment fn fs_cap`, `@fragment fn fs_cap_selection`, `@fragment fn fs_cap_id` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/cap.wgsl"
```

### Step 10 · `viewer-core/src/clipping.rs`

Derives GPU half-space normals by crossing transformed in-plane axes (handling non-uniform scale) and anchors hatch direction to the plane frame. Spanning `340` lines, this module stays free of duplicate state by exposing `Mode`, `ALL`, `word`, `points` and verified by `modes_place_the_plane_and_the_cut_side` (+6 more tests). Spanning `340` lines, this module stays free of duplicate state by exposing `Mode`, `ALL`, `word`, `points` and verified by `modes_place_the_plane_and_the_cut_side` (+6 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/clipping.rs"
```

### Step 11 · `src/state/clipping.rs`

Reads live GPU row placements so dragging a plane updates the cut immediately, keeping cuts active when the plane border is hidden and verifying closed meshes on first cut. Spanning `231` lines, this module stays free of duplicate state by exposing `update_clipping`, `clipping_status`, `clipping_half_size`, `pick_clipping_plane` without introducing UI/GPU coupling cycles. Spanning `231` lines, this module stays free of duplicate state by exposing `update_clipping`, `clipping_status`, `clipping_half_size`, `pick_clipping_plane` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/clipping.rs"
```

### Step 12 · `viewer-core/src/command/verbs/clipping_plane.rs`

Creates scene-sized section planes or toggles `On`/`Off`, `Flip` and `Hatch`/`Solid` fill modes within the `6`-plane GPU cap. Spanning `225` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `clipping_plane_lines_parse` (+1 more tests). Spanning `225` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` and verified by `clipping_plane_lines_parse` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/clipping_plane.rs"
```

### Step 13 · `viewer-core/src/command/verbs/opacity.rs`

`arctic.rs` enables ambient clay shading + surface outlines; `outline.rs` toggles borders independently; `opacity.rs` validates finite `[0.0, 1.0]` global face alpha; `alpha.rs` sets or resets per-object transparency overrides on the selection. Spanning `32` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `32` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/opacity.rs"
```

### Step 14 · `viewer-core/src/command/verbs/alpha.rs`

`arctic.rs` enables ambient clay shading + surface outlines; `outline.rs` toggles borders independently; `opacity.rs` validates finite `[0.0, 1.0]` global face alpha; `alpha.rs` sets or resets per-object transparency overrides on the selection. Spanning `35` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `35` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/alpha.rs"
```

## Integration

`triangle_tiles.rs` dynamically doubles its tile-entry pool when `map_report` observes saturation (up to the storage ceiling) and reuses its `R8Unorm` tile target across growths. `Alpha <0..1|Reset>` sets per-object `FLAG_ALPHA` in `InstanceTable` (`has_transparent`), blending translucent faces while keeping opaque strokes readable.

## Run and observe

Run `cargo xtest -p session_viewer triangle_tiles::tests`, `clip::tests`, and `selftest::oracle::edges_behind_another_solid_stay_hidden`: tile shaders validate, prefix-sum dispatches stay within WebGPU workgroup limits, and `188/188` visible edges draw with `0` hidden leaks.

## Predict and try

Predict how much GPU frame time drops on `assets/view_local.yaml` (`127k` triangles, `147k` strokes) when `TileRecord.values[3]` stores `atomicMax` frontmost tile depth for `O(1)` early-out in `ink_visibility.wgsl`: measured resting ink pass time drops from `17.40 ms` to `7.06 ms` (`-59.4%`), and orbit p95 drops from `29.20 ms` to `10.50 ms` (`-64.0%`).

## Question and answer

**Why isn't reading the hardware `Depth32Float` buffer sufficient to decide whether a thick CAD edge stroke is visible or occluded?**

**How to work it out:** Consider a `4 px`-wide silhouette edge along the front corner of a beam: half of the ribbon quad's fragments overhang empty background pixels beside the beam, while the other half step onto the beam's own sloping face.

**The answer:** Sampling `Depth32Float` at an overhanging stroke fragment reads background depth (`0.0`) even when another solid sits directly in front of the stroke's centerline, or self-occludes against the beam's own sloped face. Testing the stroke's projected axis point against finite binned triangles in `ink_visibility.wgsl` (short-circuited by `atomicMax` tile depth when the stroke is strictly in front of the entire tile) keeps silhouette strokes crisp without hidden-edge leaks.

## Validation

Verified via `triangle_tiles::tests` (`9/9` passed), `clip::tests` (`10/10` passed), `clipping::tests` (`6/6` passed), `clipping_plane::tests` (`2/2` passed), and `selftest::oracle` (`close_up_edges_preserve_visibility_across_views_and_samples`, `edges_behind_another_solid_stay_hidden`).
