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

Manages projected-triangle buffers, tile headers and the shared reference pool, polling async overflow readbacks to grow capacity up to ceiling.

```rust
--8<-- "session_viewer/src/engine/gpu/triangle_tiles.rs"
```

### Step 2 · `src/shaders/project_triangles.wgsl`

1. **`src/shaders/project_triangles.wgsl`** runs a `64`-thread compute pass (`PROJECT_ROW_GROUPS` `2D` dispatch above `65,535` groups) that resolves each triangle via `placed_triangle`, clips against the near plane, fits inward edge equations `equations[0..3]`, and chooses a local corner reference on small far triangles so homogeneous depth extrapolation never suffers cancellation.

```wgsl
--8<-- "session_viewer/src/shaders/project_triangles.wgsl"
```

### Step 3 · `src/shaders/triangle_tiles.wgsl`

2. **`src/shaders/triangle_tiles.wgsl`** rasterizes one quad over each projected triangle's tile bounding box, discards uncovered tiles via `covered_tile`, atomically counts hits in `fs_count`, and packs `(primitive, nearest_depth_bound)` pairs in `fs_fill`.

```wgsl
--8<-- "session_viewer/src/shaders/triangle_tiles.wgsl"
```

### Step 4 · `src/shaders/scan_triangle_tiles.wgsl`

3. **`src/shaders/scan_triangle_tiles.wgsl`** executes a `256`-lane Hillis-Steele / Blelloch workgroup prefix sum (`scan_tiles` -> `scan_blocks` -> `finish_offsets`) between `fs_count` and `fs_fill`, saturating at buffer capacity instead of wrapping on overflow.

```wgsl
--8<-- "session_viewer/src/shaders/scan_triangle_tiles.wgsl"
```

### Step 5 · `src/engine/gpu/clip.rs`

Tests placed object bounds against active half-spaces with precision slack and builds per-plane cap resources only for verified closed solids.

```rust
--8<-- "session_viewer/src/engine/gpu/clip.rs"
```

### Step 6 · `src/engine/gpu/clip/pipelines.rs`

`src/engine/gpu/clip/pipelines.rs` (208 lines) defines `cap_source()`, `pick_pipelines()`, `pick_placed_pipelines()`. It maintains explicit buffer capacities, bind-group layouts, and frame-submission ordering for WebGPU.

```rust
--8<-- "session_viewer/src/engine/gpu/clip/pipelines.rs"
```

### Step 7 · `src/engine/gpu/clip/tests.rs`

Parses WGSL via `naga` to verify `ClipUniform` byte offsets against `offset_of!` and tests closed-cube section hatches on GPU.

```rust
--8<-- "session_viewer/src/engine/gpu/clip/tests.rs"
```

### Step 8 · `src/engine/gpu/clip/editing_tests.rs`

`src/engine/gpu/clip/editing_tests.rs` (1146 lines) implements internal state and helper routines for `clip`. It maintains explicit buffer capacities, bind-group layouts, and frame-submission ordering for WebGPU.

```rust
--8<-- "session_viewer/src/engine/gpu/clip/editing_tests.rs"
```

### Step 9 · `src/shaders/cap.wgsl`

1. **`src/shaders/cap.wgsl`** evaluates plane depth `clip_plane_depth`, trims against every other active section plane sample-by-sample, tests whether back-minus-front crossings behind the plane exceed `0.5` (`count_at`), and outputs `45°` `box_cdf`-filtered hatches plus `CAP_PRIMITIVE`.

```wgsl
--8<-- "session_viewer/src/shaders/cap.wgsl"
```

### Step 10 · `viewer-core/src/clipping.rs`

Derives GPU half-space normals by crossing transformed in-plane axes (handling non-uniform scale) and anchors hatch direction to the plane frame.

```rust
--8<-- "session_viewer/viewer-core/src/clipping.rs"
```

### Step 11 · `src/state/clipping.rs`

Reads live GPU row placements so dragging a plane updates the cut immediately, keeping cuts active when the plane border is hidden and verifying closed meshes on first cut.

```rust
--8<-- "session_viewer/src/state/clipping.rs"
```

### Step 12 · `viewer-core/src/command/verbs/clipping_plane.rs`

Creates scene-sized section planes or toggles `On`/`Off`, `Flip` and `Hatch`/`Solid` fill modes within the `6`-plane GPU cap.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/clipping_plane.rs"
```

### Step 13 · `viewer-core/src/command/verbs/opacity.rs`

`viewer-core/src/command/verbs/opacity.rs` implements the `opacity` command verb: executes its typed command specification Validates arguments against `&mut dyn CommandHost` before mutating scene or view state so invalid interactive or CLI input leaves undo history untouched.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/opacity.rs"
```

### Step 14 · `viewer-core/src/command/verbs/alpha.rs`

`viewer-core/src/command/verbs/alpha.rs` implements the `alpha` command verb: executes its typed command specification Validates arguments against `&mut dyn CommandHost` before mutating scene or view state so invalid interactive or CLI input leaves undo history untouched.

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
