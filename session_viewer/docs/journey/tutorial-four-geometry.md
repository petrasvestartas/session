# Tutorial 4 · Scene rows, `112 B` instances and `f64` anchor rebasing

## Goal

Define the pure-Rust `GpuSink` upload boundary, pack `112`-byte `Instance` records alongside `f64` camera-anchored `16`-byte translations, and synchronize `Scene` object rows by `(doc, guid)` with `O(1)` tombstones.

![All five geometry lanes in one scene: BRep teapot, Mesh box, NURBS arch, 3D Arrow, and Point markers](../screenshots/tutorials/t04-lanes.png) ![Dense Stanford Bunny mesh uploaded into shared octahedral-normal GPU arena buffers](../screenshots/tutorials/t04-bunny.png)

**What we achieve:** The pure-Rust `viewer-core::rows` and `Scene` row-table layer (`112 B` `Instance` + rebased `[f32; 4]` translation slabs + `O(1)` `FLAG_DEAD | FLAG_HIDDEN` tombstones) verified standalone in `?tutorial=rows` before mesh/BRep and stroke walkers attach in [Tutorial 4.1](tutorial-four-meshes.md) and [Tutorial 4.2](tutorial-four-strokes.md).

## How it works

World translations rebase against the `f64` camera anchor (`translations[i] = f32(T_i − anchor)`) separately from the `112`-byte `Instance` rotation/scale matrix (`rows/instance.rs`), while `scene_sync` keys every uploaded range by `(doc, guid)` and retires deleted objects to tombstones so `Ctrl+Z` restores rows in `O(1)` with zero vertex re-upload.

![Split model + rebased translation placement, five kernel walkers into shared arenas, and O(1) tombstones with exact byte strides](../screenshots/tutorials/explain-04-geometry.png)

## Compilable checkpoint slice

Tutorial 4's row-table core compiles standalone as the `rows` assembly (`80` files, `?tutorial=rows`).

Lines marked `+` show the exact module declarations unlocked between `camera` (`50` files) and `rows` (`80` files):

```rust
// viewer-core/src/lib.rs (camera -> rows)
  pub mod camera;
+ pub mod rows;
+ pub mod sink;
+ pub use crate as app;
+ pub use sink as core;
+ pub use sink::GpuSink;

// src/lib.rs (camera -> rows)
  /// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
  macro_rules! shader {
      ($name:literal) => {
          include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
      };
  }
  mod engine;
+ pub use viewer_core as core;
+ pub use viewer_core::rows;
  pub use viewer_core::camera;
  pub mod command_dock;
  pub mod app;
  #[cfg(all(target_arch = "wasm32", feature = "tutorial"))]
  #[path = "../examples/course/entry.rs"]
  mod course;

// src/engine/mod.rs (camera -> rows)
+ pub mod diagnostics;
  pub mod gpu;
+ pub mod performance;
+ pub mod pipelines;

// src/engine/gpu/mod.rs (camera -> rows)
+ pub mod buffers; // register:buffers
  pub mod device; // register:device
+ pub mod hull; // register:hull
+ pub mod instance; // register:instance
+ pub mod objects;
  pub mod ui; // register:ui

// examples/course/registration.rs (camera -> rows)
  // --8<-- [start:course-registration]
  #[path = "canvas.rs"]
  mod canvas;
  #[path = "frame.rs"]
  mod frame;
  #[path = "painting.rs"]
  mod painting;
  #[path = "mathematics.rs"]
  mod mathematics;
  #[path = "dock.rs"]
  mod dock;
  #[path = "camera.rs"]
  mod camera;
+ #[path = "rows.rs"]
+ mod rows;
  type Start = fn() -> Result<(), wasm_bindgen::JsValue>;
  pub(super) const STARTS: &[(&str, Start)] = &[
      ("frame", frame::start),
      ("painting", painting::start),
      ("mathematics", mathematics::start),
      ("dock", dock::start),
      ("camera", camera::start),
+     ("rows", rows::start),
  ];
  // --8<-- [end:course-registration]
```

## Core code walkthrough

### `112`-byte `Instance` layout & bit flags (`viewer-core/src/rows/instance.rs:4:56`)

```rust
--8<-- "session_viewer/viewer-core/src/rows/instance.rs:4:56"
```

`Instance` (lines `6–18`, compile-time asserted to `112` bytes) holds only rotation/scale in `model: [f32; 16]` (`64 B`), `color` (`16 B`), `flags` + `ao_radius` + `spacing` + `_pad` (`16 B`), and `effect` (`16 B`). World translation lives in a parallel `translations: array<vec4<f32>>` storage buffer rebased on the CPU as `f32(T_world − anchor)` so `f64` CAD coordinates millions of millimetres from the origin never suffer `f32` mantissa jitter.

### Shared WGSL `@group(0..=2)` bindings & colour override (`src/shaders/scene.wgsl:1:63`)

```wgsl
--8<-- "session_viewer/src/shaders/scene.wgsl:1:63"
```

Every geometry shader inherits `@group(0)` (`mvp`), `@group(1)` (`80 B` `LineUniform` + `ClipUniform`), and `@group(2)` (`instances` + `translations`). In `object_color` (lines `59–63`), `FLAG_COLOR` (`256u`) switches between multiplying authored vertex RGB by `inst.color.rgb` vs replacing it outright with the layer/override colour, while `FLAG_ALPHA` (`262144u`) applies per-object opacity without re-uploading a single vertex.

## Prerequisites and files

Requires [Tutorial 3](tutorial-three-rendering.md). Followed immediately by [Tutorial 4.1 (Mesh & BRep faces)](tutorial-four-meshes.md) and [Tutorial 4.2 (Curve ribbons, arrows & glyphs)](tutorial-four-strokes.md).

**Core files to type inline (7 files):**

1. `viewer-core/src/sink.rs`
2. `viewer-core/src/rows/instance.rs`
3. `viewer-core/src/rows/objects.rs`
4. `src/engine/gpu/instance/layout.rs`
5. `src/engine/gpu/objects/table.rs`
6. `viewer-core/src/scene.rs`
7. `examples/course/rows.rs`

**Reference catalog & test files (29 files, folded below):** `viewer-core/src/rows/mod.rs`, `viewer-core/src/rows/encode.rs`, `viewer-core/src/rows/gizmo.rs`, `viewer-core/src/rows/hull.rs`, `viewer-core/src/rows/lanes.rs`, `viewer-core/src/rows/patch.rs`, `viewer-core/src/rows/upload.rs`, `src/engine/gpu/sink.rs`, `src/engine/gpu/buffers.rs`, `src/engine/gpu/upload.rs`, `src/engine/gpu/instance.rs`, `src/engine/gpu/instance/validation.rs`, `src/engine/gpu/objects.rs`, `src/engine/gpu/objects/lane.rs`, `src/engine/gpu/objects/ink.rs`, `src/engine/gpu/objects/gpu_tests.rs`, `src/engine/gpu/hull.rs`, `src/engine/gpu/patch.rs`, `viewer-core/src/knobs.rs`, `viewer-core/src/interactions.rs`, `viewer-core/src/scene_rows.rs`, `viewer-core/src/scene_sync.rs`, `viewer-core/src/scene_sync/allocation.rs`, `viewer-core/src/scene_sync/compaction.rs`, `viewer-core/src/scene_sync/nodes.rs`, `viewer-core/src/scene_sync/notes.rs`, `viewer-core/src/scene_sync/tombs.rs`, `viewer-core/src/scene_sync/testing.rs`, `viewer-core/src/scene_sync/tests.rs`.

## Build

### Step 1 · `viewer-core/src/sink.rs`

`viewer-core/src/sink.rs` (51 lines) defines `trait GpuSink`. It provides `canvas_size()`, `geometry_revision()`, `object_row()`.

```rust
--8<-- "session_viewer/viewer-core/src/sink.rs"
```

### Step 2 · `viewer-core/src/rows/instance.rs`

`viewer-core/src/rows/instance.rs` (93 lines) defines `struct Instance`, `placeholder()`, `FLAG_SELECTED`, `FLAG_HIDDEN`. It connects `Xform` within `rows` across native and `wasm32` targets.

```rust
--8<-- "session_viewer/viewer-core/src/rows/instance.rs"
```

### Step 3 · `viewer-core/src/rows/objects.rs`

`viewer-core/src/rows/objects.rs` (50 lines) defines `struct ObjectRow`, `struct ObjectRows`, `world_box()`, `placed_bounds()`. It connects `hull`, `session_rust` within `rows` across native and `wasm32` targets.

```rust
--8<-- "session_viewer/viewer-core/src/rows/objects.rs"
```

### Step 4 · `src/engine/gpu/instance/layout.rs`

`src/engine/gpu/instance/layout.rs` defines the `#[repr(C)]` `Pod` `Instance` struct pinned to `112` bytes: `model` (`64 B` rotation + scale), `color` (`16 B`), `flags`, `ao_radius`, `spacing`, the `u32` word following `spacing` carries the packed edge colour, and `effect` (`16 B`) stores `[reveal, point_scale, 0.0, 0.0]`.

```rust
--8<-- "session_viewer/src/engine/gpu/instance/layout.rs"
```

### Step 5 · `src/engine/gpu/objects/table.rs`

`src/engine/gpu/objects/table.rs` (`InstanceTable` / `ObjectRows`) mirrors CPU rows alongside `instances` (`N × 112 B`) and `translations` (`N × 16 B`), keeping any gizmo widget row last and uploading single-row edits via `write_at`.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/table.rs"
```

### Step 6 · `viewer-core/src/scene.rs`

`src/app/scene.rs` wraps `Rc<Session>` inside `FileDoc` (name, file `Xform`, `point_size`, shell flag) so two placements can share one kernel `Session` while `(doc_idx, GUID)` maps each placed object to its own GPU display row.

```rust
--8<-- "session_viewer/viewer-core/src/scene.rs"
```

### Step 7 · `examples/course/rows.rs`

`examples/course/rows.rs` runs the Tutorial 4 row-table checkpoint (`?tutorial=rows`), uploading `112`-byte `Instance` records and camera-anchored `16`-byte translations through `InstanceTable` and validating the 6 shared WGSL headers. Trap: subtract the `f64` camera anchor before casting world coordinates to `f32`, or objects far from the origin jitter during orbit.

```rust
--8<-- "session_viewer/examples/course/rows.rs"
```

## Reference catalog and test files (29 files)

The 7 core files above implement the complete mechanism for this lesson. Expand this reference section whenever you want to inspect or copy the remaining 29 sibling verbs, adapters, and unit-test suites (`Step 8`–`Step 36`).

### Step 8 · `viewer-core/src/rows/mod.rs`

`viewer-core/src/rows/mod.rs` (20 lines) declares and re-exports module tree `encode`, `gizmo`, `hull`, `instance`, `lanes`, `objects`. It keeps `mod` deterministic across native `cargo xtest` and `wasm32-unknown-unknown` builds.

```rust
--8<-- "session_viewer/viewer-core/src/rows/mod.rs"
```

### Step 9 · `viewer-core/src/rows/encode.rs`

`viewer-core/src/rows/encode.rs` (78 lines) defines `struct Pen`, `encode_width()`, `pack_rgba()`, `oct16()`, `BLACK`. It provides `quant8()`, `sign_not_zero()`, `quant_snorm8()`, `FACING_UNKNOWN`.

```rust
--8<-- "session_viewer/viewer-core/src/rows/encode.rs"
```

### Step 10 · `viewer-core/src/rows/gizmo.rs`

`viewer-core/src/rows/gizmo.rs` (14 lines) defines `ARM`, `BALL_AT`. It keeps `gizmo` deterministic across native `cargo xtest` and `wasm32-unknown-unknown` builds.

```rust
--8<-- "session_viewer/viewer-core/src/rows/gizmo.rs"
```

### Step 11 · `viewer-core/src/rows/hull.rs`

`viewer-core/src/rows/hull.rs` (392 lines) defines `type Hull`, `struct Points`, `of()`, `extreme_points()`, `placed_box()`. It provides `volume()`, `sub()`, `dot()`, `ABOVE`; and locked by `cargo xtest` (`a_turned_beam_is_boxed_by_its_extreme_points`, `every_direction_peaks_on_a_kept_point`, `a_round_set_gives_up` (+1 more)).

```rust
--8<-- "session_viewer/viewer-core/src/rows/hull.rs"
```

### Step 12 · `viewer-core/src/rows/lanes.rs`

`viewer-core/src/rows/lanes.rs` (557 lines) defines `struct Pick`, `enum PickMode`, `struct FaceSource`, `drop_rows()`, `is_silhouette()`. It provides `plated()`, `lane_rows_by_type()`, `FACE_TAG`, `SILHOUETTE_SAG`; and locked by `cargo xtest` (`lane_rows_by_type`).

```rust
--8<-- "session_viewer/viewer-core/src/rows/lanes.rs"
```

### Step 13 · `viewer-core/src/rows/patch.rs`

`viewer-core/src/rows/patch.rs` (224 lines) defines `struct RowSpec`, `enum LaneId`, `struct Counts`, `all()`, `stride()`. It provides `vector_rows_in()`, `vector_merge()`, `plane_rows_in()`, `ROW_REGISTRY`; and exposes cumulative checkpoint seams `register:markers`, `register:meshes`, `register:strokes`.

```rust
--8<-- "session_viewer/viewer-core/src/rows/patch.rs"
```

### Step 14 · `viewer-core/src/rows/upload.rs`

`viewer-core/src/rows/upload.rs` (173 lines) defines `struct Upload`, `drop_uploaded()`, `merge()`, `shift_vertices()`. It provides `merge_arena()`; and exposes cumulative checkpoint seams `register:clouds`, `register:markers`, `register:meshes`, `register:strokes`.

```rust
--8<-- "session_viewer/viewer-core/src/rows/upload.rs"
```

### Step 15 · `src/engine/gpu/sink.rs`

`src/engine/gpu/sink.rs` (188 lines) implements `sink` (`canvas_size()`, `geometry_revision()`, `object_row()`) inside `gpu`. It connects `Gpu`, `rows`, `session_rust`, `GpuSink` within `gpu` across native and `wasm32` targets.

```rust
--8<-- "session_viewer/src/engine/gpu/sink.rs"
```

### Step 16 · `src/engine/gpu/buffers.rs`

`src/engine/gpu/buffers.rs` provides `GpuCtx`, `Template`, and `GrowBuf` (`append`, `reserve`, `WRITE_CHUNK`-bounded `fill`, `fits` against device/storage limits, `reset`, `release`, and GPU-copied `packed`), rebuilding bind groups whenever growth swaps an underlying `wgpu::Buffer`.

```rust
--8<-- "session_viewer/src/engine/gpu/buffers.rs"
```

### Step 17 · `src/engine/gpu/upload.rs`

Stages CPU lane vectors, rebases vertex indices and valid face IDs on merge, and frees host vectors via `drop_uploaded` after GPU transfer.

```rust
--8<-- "session_viewer/src/engine/gpu/upload.rs"
```

### Step 18 · `src/engine/gpu/instance.rs`

`src/engine/gpu/instance.rs` (6 lines) declares and re-exports module tree `layout`, `validation`. It keeps `instance` deterministic across native `cargo xtest` and `wasm32-unknown-unknown` builds.

```rust
--8<-- "session_viewer/src/engine/gpu/instance.rs"
```

### Step 19 · `src/engine/gpu/instance/validation.rs`

Parses WGSL struct declarations and verifies exact field order and `112 B` / `80 B` byte sizes against Rust `Instance` and `LineUniform`.

```rust
--8<-- "session_viewer/src/engine/gpu/instance/validation.rs"
```

### Step 20 · `src/engine/gpu/objects.rs`

`src/engine/gpu/objects.rs` (9 lines) declares and re-exports module tree `gpu_tests`, `ink`, `lane`, `table`. It keeps `objects` deterministic across native `cargo xtest` and `wasm32-unknown-unknown` builds.

```rust
--8<-- "session_viewer/src/engine/gpu/objects.rs"
```

### Step 21 · `src/engine/gpu/objects/lane.rs`

`src/engine/gpu/objects/lane.rs` (17 lines) implements `lane` (`on_reset()`, `on_release()`, `bytes()`) inside `objects`. It connects `InstanceTable`, `GpuCtx`, `Layouts` within `objects` across native and `wasm32` targets.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/lane.rs"
```

### Step 22 · `src/engine/gpu/objects/ink.rs`

Builds `@group(2)` ink bind groups combining object rows, translations, `1×`/`4×` depth and primitive-ID views, projected triangles and tile lists.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/ink.rs"
```

### Step 23 · `src/engine/gpu/objects/gpu_tests.rs`

Adapter-gated headless GPU test verifying neighbour row isolation, scale-updated `ao_radius` and widget-last ordering.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/gpu_tests.rs"
```

### Step 24 · `src/engine/gpu/hull.rs`

`src/engine/gpu/hull.rs` (2 lines) implements `hull` (module state) inside `gpu`. It keeps `hull` deterministic across native `cargo xtest` and `wasm32-unknown-unknown` builds.

```rust
--8<-- "session_viewer/src/engine/gpu/hull.rs"
```

### Step 25 · `src/engine/gpu/patch.rs`

Tracks per-lane `Counts` and `Span` offsets (including CPU source-ID mirrors) so edits test in-place fit across all lanes simultaneously.

```rust
--8<-- "session_viewer/src/engine/gpu/patch.rs"
```

### Step 26 · `viewer-core/src/knobs.rs`

Latches diagnostic `VIEWER_*` environment switches once via `OnceLock` for profiling, edge/dot suppression, coplanar edges and BRep refinement.

```rust
--8<-- "session_viewer/viewer-core/src/knobs.rs"
```

### Step 27 · `viewer-core/src/interactions.rs`

Separates contact, joint, cut and drill edge interactions from owned element features in deterministic graph-edge creation order.

```rust
--8<-- "session_viewer/viewer-core/src/interactions.rs"
```

### Step 28 · `viewer-core/src/scene_rows.rs`

Separates `(doc_idx, GUID)` row identity (plus text/free/sink/tomb sentinels) from `Footprint` (`One`, `Many`, `Cloud`, `None`) lane spans.

```rust
--8<-- "session_viewer/viewer-core/src/scene_rows.rs"
```

### Step 29 · `viewer-core/src/scene_sync.rs`

Drains change notes, deduplicates identities, expands `SUBTREE` descendants and emits ordered `Staged` GPU updates (`retire`, `bury`, `patch`, `geometry`, `place`).

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync.rs"
```

### Step 30 · `viewer-core/src/scene_sync/allocation.rs`

Reuses an identity's grave when all lane counts match, pads drag previews by ~`50%`, and routes retired tails to the hidden sink row.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/allocation.rs"
```

### Step 31 · `viewer-core/src/scene_sync/compaction.rs`

`viewer-core/src/scene_sync/compaction.rs` (174 lines) defines `compaction_due()`, `purge_step()`, `cloud_compaction_due()`. It exposes cumulative checkpoint seams `register:editing`, `register:instancing`.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/compaction.rs"
```

### Step 32 · `viewer-core/src/scene_sync/nodes.rs`

Resolves affected tree nodes via weak caches and batched per-document searches, multiplying file `Xform` by root-to-node transforms.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/nodes.rs"
```

### Step 33 · `viewer-core/src/scene_sync/notes.rs`

Translates forward and reverse kernel transaction ops into `GEOMETRY`, `PLACE`, `PRESENCE` and `SUBTREE` invalidation flags without rebuilding untouched objects.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/notes.rs"
```

### Step 34 · `viewer-core/src/scene_sync/tombs.rs`

Retains up to `256 MiB` of deleted lane spans keyed by weak kernel tomb pointers so `Undo` unburies rows without re-tessellating geometry.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/tombs.rs"
```

### Step 35 · `viewer-core/src/scene_sync/testing.rs`

Models `settle` upload bookkeeping on the CPU and compares incremental edits bit-for-bit against a fresh multi-document walk (`verify`).

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/testing.rs"
```

### Step 36 · `viewer-core/src/scene_sync/tests.rs`

Runs `150`-step seeded random edit sequences (including a `4,096 B` tomb cap and cache on/off parity) against the fresh-scene oracle.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/tests.rs"
```

## Integration

`viewer-core/src/lib.rs` exports `rows`, `sink` (`GpuSink`), `knobs`, `interactions`, `walk`, and `scene`, while `src/engine/gpu/sink.rs` implements `GpuSink` for `Gpu`. `Scene::upload_to` pushes `Upload` batches across that trait boundary without `viewer-core` ever importing `wgpu`.

## Run and observe

Verify the `112`-byte `Instance` layout, `f64` camera-anchor rebasing, and `O(1)` tombstone slot updates, or launch `?tutorial=rows`:

```sh
cargo xtest -p viewer-core a_move_goes_into_the_translation_not_the_matrix
cargo xtest -p viewer-core edits_write_only_their_own_slots
trunk serve --features tutorial --open /?tutorial=rows
```

Translating a `50,000`-triangle object updates only `16` bytes in `translations` (`0` vertex or model-matrix uploads), and deleting/undoing an object toggles `FLAG_HIDDEN` in `O(1)`.

## Predict and try

Predict how many bytes upload to the GPU when you translate one `50,000`-triangle mesh by `[100, 0, 0]`: `0` vertex bytes and `0` model-matrix bytes—`InstanceTable` updates only that row's `16`-byte `[f32; 4]` camera-anchored translation entry (`a_move_goes_into_the_translation_not_the_matrix`).

## Question and answer

**Why does deleting an object park its GPU slots in `tombs.rs` instead of immediately compacting `Arena` and `SegTable`?**

??? tip "Think first — click to reveal how to work it out and the answer"
    **How to work it out:** Consider pressing `Delete` on a `200,000`-triangle solid and immediately pressing `Ctrl+Z` (`Undo`): immediate compaction would shift every subsequent object's buffer range and discard the uploaded triangles.

    **The answer:** Parking deleted ranges in `tombs.rs` (`MAX_TOMBS` cap) hides the row in `O(1)` via `FLAG_HIDDEN` so `Undo` restores the exact row ID and GPU vertices with zero re-walk or upload; `compaction.rs` reclaims dead arena space only when tomb bytes cross the high-water budget or redo history drops unreachable graves.

## Validation

Verified via `cargo xtest -p viewer-core scene::` (`46` scene/sync tests), `a_move_goes_into_the_translation_not_the_matrix`, `edits_write_only_their_own_slots`, and `frustum_outcode_culls_offscreen_boxes_and_keeps_visible_ones`.
