# Tutorial 4 · Geometry and scene data structures

## Goal

Walk `session_rust` meshes, BReps, curves, vectors, points, and planes into `20`-byte packed vertices, slot-addressed arenas, screen-space ribbons/arrows, and `112`-byte `Instance` rows synchronized incrementally by `(doc, guid)` identity.

## Diagram

![Source walk (`viewer-core::walk`) to `Upload` payloads, shared `20 B` vertex arena, `112 B` `Instance` table, GPU lanes, and `Scene` `(doc, guid)` tombstones (native wgpu readback of stage deck)](../../examples/stage/tutorials-foundations/04-geometry.png)

Stage deck source: [`04-geometry.yaml`](../../examples/stage/tutorials-foundations/04-geometry.yaml).

## Prerequisites and files

Requires [Tutorial 3](tutorial-three-rendering.md). Introduces `viewer-core/src/{sink,rows/*,walk/*,knobs,interactions,scene,scene_rows,scene_sync/*}` and their GPU lane counterparts in `src/engine/gpu/` and `src/shaders/`.

Owned files in build order (`75` files):

1. `viewer-core/src/sink.rs`
2. `viewer-core/src/rows/mod.rs`
3. `viewer-core/src/rows/encode.rs`
4. `viewer-core/src/rows/gizmo.rs`
5. `viewer-core/src/rows/hull.rs`
6. `viewer-core/src/rows/instance.rs`
7. `viewer-core/src/rows/lanes.rs`
8. `viewer-core/src/rows/objects.rs`
9. `viewer-core/src/rows/patch.rs`
10. `viewer-core/src/rows/upload.rs`
11. `src/engine/gpu/sink.rs`
12. `src/engine/gpu/buffers.rs`
13. `src/engine/gpu/upload.rs`
14. `src/engine/gpu/instance.rs`
15. `src/engine/gpu/instance/layout.rs`
16. `src/engine/gpu/instance/validation.rs`
17. `src/engine/gpu/objects.rs`
18. `src/engine/gpu/objects/table.rs`
19. `src/engine/gpu/objects/lane.rs`
20. `src/engine/gpu/objects/ink.rs`
21. `src/engine/gpu/objects/gpu_tests.rs`
22. `src/engine/gpu/hull.rs`
23. `src/engine/gpu/patch.rs`
24. `src/engine/gpu/arena.rs`
25. `src/engine/gpu/slots.rs`
26. `src/shaders/slot_table.wgsl`
27. `src/engine/gpu/faces.rs`
28. `src/shaders/triangle.wgsl`
29. `src/engine/gpu/segments.rs`
30. `src/shaders/ribbon.wgsl`
31. `src/shaders/arrow.wgsl`
32. `src/engine/gpu/vectors.rs`
33. `src/shaders/vector.wgsl`
34. `src/engine/gpu/glyphs.rs`
35. `src/shaders/glyph.wgsl`
36. `src/shaders/sphere.wgsl`
37. `src/engine/gpu/planes.rs`
38. `src/shaders/plane.wgsl`
39. `viewer-core/src/knobs.rs`
40. `viewer-core/src/interactions.rs`
41. `viewer-core/src/walk/mod.rs`
42. `viewer-core/src/walk/encode.rs`
43. `viewer-core/src/walk/bounds.rs`
44. `viewer-core/src/walk/mesh.rs`
45. `viewer-core/src/walk/mesh_topology.rs`
46. `viewer-core/src/walk/mesh_ink.rs`
47. `viewer-core/src/walk/brep.rs`
48. `viewer-core/src/walk/brep_orient.rs`
49. `viewer-core/src/walk/brep_shapes.rs`
50. `viewer-core/src/walk/brep_edges.rs`
51. `viewer-core/src/walk/curves.rs`
52. `viewer-core/src/walk/plane.rs`
53. `viewer-core/src/walk/points.rs`
54. `viewer-core/src/walk/frames.rs`
55. `viewer-core/src/walk/display.rs`
56. `viewer-core/src/walk/display_features.rs`
57. `viewer-core/src/walk/application_tests.rs`
58. `viewer-core/src/walk/brep/application_tests.rs`
59. `viewer-core/src/scene.rs`
60. `viewer-core/src/scene_rows.rs`
61. `viewer-core/src/scene_sync.rs`
62. `viewer-core/src/scene_sync/allocation.rs`
63. `viewer-core/src/scene_sync/compaction.rs`
64. `viewer-core/src/scene_sync/nodes.rs`
65. `viewer-core/src/scene_sync/notes.rs`
66. `viewer-core/src/scene_sync/tombs.rs`
67. `viewer-core/src/scene_sync/testing.rs`
68. `viewer-core/src/scene_sync/tests.rs`
69. `src/engine/gpu/scene_gpu_tests/mod.rs`
70. `src/engine/gpu/scene_gpu_tests/contacts.rs`
71. `src/engine/gpu/scene_gpu_tests/frames.rs`
72. `src/engine/gpu/scene_gpu_tests/instances.rs`
73. `src/engine/gpu/scene_gpu_tests/split.rs`
74. `examples/course/rows.rs`
75. `examples/course/geometry.rs`

## Build

### Step 1 · `viewer-core/src/sink.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `51` lines, this module stays free of duplicate state by exposing `GpuSink` without introducing UI/GPU coupling cycles. Spanning `51` lines, this module stays free of duplicate state by exposing `GpuSink` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/sink.rs"
```

### Step 2 · `viewer-core/src/rows/mod.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `20` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `20` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/rows/mod.rs"
```

### Step 3 · `viewer-core/src/rows/encode.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `78` lines, this module stays free of duplicate state by exposing `encode_width`, `pack_rgba`, `oct16`, `BLACK` without introducing UI/GPU coupling cycles. Spanning `78` lines, this module stays free of duplicate state by exposing `encode_width`, `pack_rgba`, `oct16`, `BLACK` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/rows/encode.rs"
```

### Step 4 · `viewer-core/src/rows/gizmo.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `14` lines, this module stays free of duplicate state by exposing `ARM`, `BALL_AT`, `HUB`, `PLANE_MIN` without introducing UI/GPU coupling cycles. Spanning `14` lines, this module stays free of duplicate state by exposing `ARM`, `BALL_AT`, `HUB`, `PLANE_MIN` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/rows/gizmo.rs"
```

### Step 5 · `viewer-core/src/rows/hull.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `392` lines, this module stays free of duplicate state by exposing `Hull`, `Points`, `of`, `extreme_points` and verified by `a_turned_beam_is_boxed_by_its_extreme_points` (+3 more tests). Spanning `392` lines, this module stays free of duplicate state by exposing `Hull`, `Points`, `of`, `extreme_points` and verified by `a_turned_beam_is_boxed_by_its_extreme_points` (+3 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/rows/hull.rs"
```

### Step 6 · `viewer-core/src/rows/instance.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `93` lines, this module stays free of duplicate state by exposing `Instance`, `FLAG_SELECTED`, `FLAG_HIDDEN`, `FLAG_INSIDE` without introducing UI/GPU coupling cycles. Spanning `93` lines, this module stays free of duplicate state by exposing `Instance`, `FLAG_SELECTED`, `FLAG_HIDDEN`, `FLAG_INSIDE` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/rows/instance.rs"
```

### Step 7 · `viewer-core/src/rows/lanes.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `557` lines, this module stays free of duplicate state by exposing `Pick`, `PickMode`, `FACE_TAG`, `FaceSource` and verified by `lane_rows_by_type`. Spanning `557` lines, this module stays free of duplicate state by exposing `Pick`, `PickMode`, `FACE_TAG`, `FaceSource` and verified by `lane_rows_by_type`.

```rust
--8<-- "session_viewer/viewer-core/src/rows/lanes.rs"
```

### Step 8 · `viewer-core/src/rows/objects.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `50` lines, this module stays free of duplicate state by exposing `ObjectRow`, `ObjectRows`, `world_box`, `placed_bounds` without introducing UI/GPU coupling cycles. Spanning `50` lines, this module stays free of duplicate state by exposing `ObjectRow`, `ObjectRows`, `world_box`, `placed_bounds` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/rows/objects.rs"
```

### Step 9 · `viewer-core/src/rows/patch.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `224` lines, this module stays free of duplicate state by exposing `RowSpec`, `ROW_REGISTRY`, `REGISTERED`, `LaneId` without introducing UI/GPU coupling cycles. Spanning `224` lines, this module stays free of duplicate state by exposing `RowSpec`, `ROW_REGISTRY`, `REGISTERED`, `LaneId` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/rows/patch.rs"
```

### Step 10 · `viewer-core/src/rows/upload.rs`

`viewer-core/src/rows/` (`mod.rs`, `instance.rs`, `objects.rs`, `hull.rs`, `lanes.rs`, `patch.rs`, `upload.rs`, `encode.rs`, `gizmo.rs`) and `viewer-core/src/sink.rs` (`GpuSink`) keep all `Pod` staging row layouts, lane strides, preview padding and upload contracts free of `wgpu` types so `viewer-core` compiles and tests natively or on `wasm32` without a GPU adapter. Spanning `173` lines, this module stays free of duplicate state by exposing `Upload`, `drop_uploaded`, `merge`, `shift_vertices` without introducing UI/GPU coupling cycles. Spanning `173` lines, this module stays free of duplicate state by exposing `Upload`, `drop_uploaded`, `merge`, `shift_vertices` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/rows/upload.rs"
```

### Step 11 · `src/engine/gpu/sink.rs`

`src/engine/pipelines/geometry.rs` binds `GpuVertex` (`20 B` stride: `pos: [f32; 3]`, `oct32` normal `u32`, `rgba8` `u32`) to arena pipelines and includes the adapter-gated lazy-compilation test verifying `1×`/`4×` MSAA target reuse without redundant shader builds, while `src/engine/gpu/sink.rs` implements `viewer_core::GpuSink` on `Gpu` so `Scene` flushes staged uploads, patches and compactions without depending on `wgpu`. Spanning `188` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `188` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/sink.rs"
```

### Step 12 · `src/engine/gpu/buffers.rs`

`src/engine/gpu/buffers.rs` provides `GpuCtx`, `Template`, and `GrowBuf` (`append`, `reserve`, `WRITE_CHUNK`-bounded `fill`, `fits` against device/storage limits, `reset`, `release`, and GPU-copied `packed`), rebuilding bind groups whenever growth swaps an underlying `wgpu::Buffer`. Spanning `378` lines, this module stays free of duplicate state by exposing `GpuCtx`, `ROWS`, `WRITE_CHUNK`, `VERTS` and verified by `reserve_is_exact_and_stops_at_the_largest_buffer` (+1 more tests). Spanning `378` lines, this module stays free of duplicate state by exposing `GpuCtx`, `ROWS`, `WRITE_CHUNK`, `VERTS` and verified by `reserve_is_exact_and_stops_at_the_largest_buffer` (+1 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/buffers.rs"
```

### Step 13 · `src/engine/gpu/upload.rs`

Stages CPU lane vectors, rebases vertex indices and valid face IDs on merge, and frees host vectors via `drop_uploaded` after GPU transfer. Spanning `2` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `2` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/upload.rs"
```

### Step 14 · `src/engine/gpu/instance.rs`

`src/engine/gpu/hull.rs`, `src/engine/gpu/instance.rs`, `src/engine/gpu/objects.rs` and `src/engine/pipelines/mod.rs` re-export narrow public types (`Hull`, `Instance`, `ObjectRows`, `Layouts`, `Cache`) that Tutorial 7 extends additively. Spanning `6` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `6` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/instance.rs"
```

### Step 15 · `src/engine/gpu/instance/layout.rs`

`src/engine/gpu/instance/layout.rs` defines the `#[repr(C)]` `Pod` `Instance` struct pinned to `112` bytes: `model` (`64 B` rotation + scale), `color` (`16 B`), `flags`, `ao_radius`, `spacing`, the `u32` word following `spacing` carries the packed edge colour, and `effect` (`16 B`) stores `[reveal, point_scale, 0.0, 0.0]`: Spanning `1` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `1` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/instance/layout.rs"
```

### Step 16 · `src/engine/gpu/instance/validation.rs`

Parses WGSL struct declarations and verifies exact field order and `112 B` / `80 B` byte sizes against Rust `Instance` and `LineUniform`. Spanning `214` lines, this module stays free of duplicate state by exposing `wgsl_fields` and verified by `instance_mirror` (+3 more tests). Spanning `214` lines, this module stays free of duplicate state by exposing `wgsl_fields` and verified by `instance_mirror` (+3 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/instance/validation.rs"
```

### Step 17 · `src/engine/gpu/objects.rs`

`src/engine/gpu/hull.rs`, `src/engine/gpu/instance.rs`, `src/engine/gpu/objects.rs` and `src/engine/pipelines/mod.rs` re-export narrow public types (`Hull`, `Instance`, `ObjectRows`, `Layouts`, `Cache`) that Tutorial 7 extends additively. Spanning `9` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `9` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/objects.rs"
```

### Step 18 · `src/engine/gpu/objects/table.rs`

`src/engine/gpu/objects/table.rs` (`InstanceTable` / `ObjectRows`) mirrors CPU rows alongside `instances` (`N × 112 B`) and `translations` (`N × 16 B`), keeping any gizmo widget row last and uploading single-row edits via `write_at`. Spanning `1185` lines, this module stays free of duplicate state by exposing `Rebase`, `InstanceTable`, `instance_buffer`, `geometry_revision` and verified by `world_box_translates` (+6 more tests). Spanning `1185` lines, this module stays free of duplicate state by exposing `Rebase`, `InstanceTable`, `instance_buffer`, `geometry_revision` and verified by `world_box_translates` (+6 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/objects/table.rs"
```

### Step 19 · `src/engine/gpu/objects/lane.rs`

Adapts `ObjectRows` to the `Lane` reset, release and byte-accounting contract alongside geometry lanes. Spanning `17` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `17` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/lane.rs"
```

### Step 20 · `src/engine/gpu/objects/ink.rs`

Builds `@group(2)` ink bind groups combining object rows, translations, `1×`/`4×` depth and primitive-ID views, projected triangles and tile lists. Spanning `83` lines, this module stays free of duplicate state by exposing `InkScene`, `ink_group`, `rebind_ink`, `pick_group` without introducing UI/GPU coupling cycles. Spanning `83` lines, this module stays free of duplicate state by exposing `InkScene`, `ink_group`, `rebind_ink`, `pick_group` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/ink.rs"
```

### Step 21 · `src/engine/gpu/objects/gpu_tests.rs`

Adapter-gated headless GPU test verifying neighbour row isolation, scale-updated `ao_radius` and widget-last ordering. Spanning `52` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `52` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/objects/gpu_tests.rs"
```

### Step 22 · `src/engine/gpu/hull.rs`

`src/engine/gpu/hull.rs`, `src/engine/gpu/instance.rs`, `src/engine/gpu/objects.rs` and `src/engine/pipelines/mod.rs` re-export narrow public types (`Hull`, `Instance`, `ObjectRows`, `Layouts`, `Cache`) that Tutorial 7 extends additively. Spanning `2` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `2` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/hull.rs"
```

### Step 23 · `src/engine/gpu/patch.rs`

Tracks per-lane `Counts` and `Span` offsets (including CPU source-ID mirrors) so edits test in-place fit across all lanes simultaneously. Spanning `2` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `2` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/patch.rs"
```

### Step 24 · `src/engine/gpu/arena.rs`

Packs vertices into `20 B` (`xyz` `f32`, octahedral normal `u32`, RGBA `u32`) with separate ordinary, printed-fill and text-outline index streams. Spanning `623` lines, this module stays free of duplicate state by exposing `SHADERS`, `GpuVertex`, `layout`, `ArenaLane` and verified by `packed_vertex_is_twenty_bytes` (+2 more tests). Spanning `623` lines, this module stays free of duplicate state by exposing `SHADERS`, `GpuVertex`, `layout`, `ArenaLane` and verified by `packed_vertex_is_twenty_bytes` (+2 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/arena.rs"
```

### Step 25 · `src/engine/gpu/slots.rs`

Maps shared definition meshes to placed instances via `8 B` `(object_row, tri_id_base)` entries (`slot 0 = OWN_ROW`) and a `1024`-wide lookup texture. Spanning `942` lines, this module stays free of duplicate state by exposing `OWN_ROW`, `Slot`, `slot_layout`, `row_slot_layout` and verified by `instance_ids_follow_the_arena` (+4 more tests). Spanning `942` lines, this module stays free of duplicate state by exposing `OWN_ROW`, `Slot`, `slot_layout`, `row_slot_layout` and verified by `instance_ids_follow_the_arena` (+4 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/slots.rs"
```

### Step 26 · `src/shaders/slot_table.wgsl`

Binary-searches `placed_triangle` across the `1024`-texel slot texture so shared definition triangles resolve to each instance's own row and unique triangle ID. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `45`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `45`-line shader defines shared WGSL helper functions and uniform structs compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/slot_table.wgsl"
```

### Step 27 · `src/engine/gpu/faces.rs`

Maps drawn triangles to source faces and binds physical, opaque, clipped, object-ID, face-pick, highlight and mask pipelines. Spanning `383` lines, this module stays free of duplicate state by exposing `Faces`, `prewarm`, `retarget`, `append` without introducing UI/GPU coupling cycles. Spanning `383` lines, this module stays free of duplicate state by exposing `Faces`, `prewarm`, `retarget`, `append` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/faces.rs"
```

### Step 28 · `src/shaders/triangle.wgsl`

1. **`src/shaders/triangle.wgsl`** transforms arena and slot-instanced vertices (`slot_row`), preserves unlit opaque contact fills pulled forward via `depth_layer`, detects mirrored transforms (`det(model) < 0`), and writes `PhysicalColor` (`color` + `2×u16` `primitive`): Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `386`-line shader defines `@vertex fn vs_main`, `@vertex fn vs_triangle`, `@vertex fn vs_face`, `@fragment fn fs_id` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `386`-line shader defines `@vertex fn vs_main`, `@vertex fn vs_triangle`, `@vertex fn vs_face`, `@fragment fn fs_id` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/triangle.wgsl"
```

### Step 29 · `src/engine/gpu/segments.rs`

Expands `40 B` CPU segments into `72 B` `StrokeSegment` records across pipe, ribbon and sheet tables while keeping source entity intervals separate from GPU offsets. Spanning `859` lines, this module stays free of duplicate state by exposing `SHADERS`, `SegChunk`, `SegSheet`, `HEAD_MARK` and verified by `cylinder_segment_mirror` (+3 more tests). Spanning `859` lines, this module stays free of duplicate state by exposing `SHADERS`, `SegChunk`, `SegSheet`, `HEAD_MARK` and verified by `cylinder_segment_mirror` (+3 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/segments.rs"
```

### Step 30 · `src/shaders/ribbon.wgsl`

1. **`src/shaders/ribbon.wgsl`** reads each `72 B` `StrokeSegment`, tests adjacent face normals (`edge_faces_camera`), and bisects consecutive screen directions in `join_plane` so chained curve segments meet at clean mitered corners without overlapping alpha: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `735`-line shader defines `@vertex fn vs_main`, `@vertex fn vs_unselected`, `@vertex fn vs_selected`, `@vertex fn vs_front` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `735`-line shader defines `@vertex fn vs_main`, `@vertex fn vs_unselected`, `@vertex fn vs_selected`, `@vertex fn vs_front` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/ribbon.wgsl"
```

### Step 31 · `src/shaders/arrow.wgsl`

2. **`src/shaders/arrow.wgsl`** and **`src/shaders/vector.wgsl`**: `src/shaders/arrow.wgsl` generates `18` shaft and narrow `HEAD_ASPECT = 0.30` start/end head vertices per vector (landing on the exact endpoint) by accepting scalar/vector arguments individually in `arrow_corner` rather than a packed `VectorRow` struct, avoiding an observed Android WebGPU driver struct-argument corruption bug: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `429`-line shader defines `@fragment fn fs_main`, `@fragment fn fs_id` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `429`-line shader defines `@fragment fn fs_main`, `@fragment fn fs_id` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/arrow.wgsl"
```

### Step 32 · `src/engine/gpu/vectors.rs`

Registers `VectorLane` (`64 B` `VectorRow` stride, `18` generated vertices per arrow) with matching colour and object-ID pipelines. Spanning `950` lines, this module stays free of duplicate state by exposing `SHADERS`, `VectorLane`, `REGISTERED`, `arrow` and verified by `vector_row_mirror` (+4 more tests). Spanning `950` lines, this module stays free of duplicate state by exposing `SHADERS`, `VectorLane`, `REGISTERED`, `arrow` and verified by `vector_row_mirror` (+4 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/vectors.rs"
```

### Step 33 · `src/shaders/vector.wgsl`

2. **`src/shaders/arrow.wgsl`** and **`src/shaders/vector.wgsl`**: `src/shaders/arrow.wgsl` generates `18` shaft and narrow `HEAD_ASPECT = 0.30` start/end head vertices per vector (landing on the exact endpoint) by accepting scalar/vector arguments individually in `arrow_corner` rather than a packed `VectorRow` struct, avoiding an observed Android WebGPU driver struct-argument corruption bug: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `22`-line shader defines `@vertex fn vs_main` after `#include` expansion of `arrow.wgsl`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `22`-line shader defines `@vertex fn vs_main` after `#include` expansion of `arrow.wgsl`.

```wgsl
--8<-- "session_viewer/src/shaders/vector.wgsl"
```

### Step 34 · `src/engine/gpu/glyphs.rs`

Stores `48 B` `GlyphPoint` records (position, signed radius, colour, row and up to `6` packed face normals) for dots and vertex markers. Spanning `333` lines, this module stays free of duplicate state by exposing `SHADERS`, `GlyphLane`, `patch_marker`, `patch` and verified by `glyph_point_mirror`. Spanning `333` lines, this module stays free of duplicate state by exposing `SHADERS`, `GlyphLane`, `patch_marker`, `patch` and verified by `glyph_point_mirror`.

```rust
--8<-- "session_viewer/src/engine/gpu/glyphs.rs"
```

### Step 35 · `src/shaders/glyph.wgsl`

1. **`src/shaders/glyph.wgsl`** circumscribes each `48 B` `GlyphPoint` with one `3`-vertex equilateral triangle `(0, 2), (-1.732, -1), (1.732, -1)` and cuts the anti-aliased circle in the fragment shader, while **`src/shaders/sphere.wgsl`** tapers dense mesh vertex markers down to `TAPER_MIN = 0.15`: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `181`-line shader defines `@fragment fn fs_main`, `@fragment fn fs_id`, `@vertex fn vs_main`, `@vertex fn vs_source` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `181`-line shader defines `@fragment fn fs_main`, `@fragment fn fs_id`, `@vertex fn vs_main`, `@vertex fn vs_source` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/glyph.wgsl"
```

### Step 36 · `src/shaders/sphere.wgsl`

1. **`src/shaders/glyph.wgsl`** circumscribes each `48 B` `GlyphPoint` with one `3`-vertex equilateral triangle `(0, 2), (-1.732, -1), (1.732, -1)` and cuts the anti-aliased circle in the fragment shader, while **`src/shaders/sphere.wgsl`** tapers dense mesh vertex markers down to `TAPER_MIN = 0.15`: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `211`-line shader defines `@vertex fn vs_main`, `@vertex fn vs_front`, `@fragment fn fs_main`, `@fragment fn fs_id` compiled directly through `build.rs` into `OUT_DIR`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `211`-line shader defines `@vertex fn vs_main`, `@vertex fn vs_front`, `@fragment fn fs_main`, `@fragment fn fs_id` compiled directly through `build.rs` into `OUT_DIR`.

```wgsl
--8<-- "session_viewer/src/shaders/sphere.wgsl"
```

### Step 37 · `src/engine/gpu/planes.rs`

Registers `PlaneLane` (`48 B` stride, `1` instanced draw -> `22` grid lines + `3` axis arrows) with atomic `f64` global plane-size scaling. Spanning `254` lines, this module stays free of duplicate state by exposing `SHADERS`, `PlaneLane`, `REGISTERED` and verified by `plane_row_mirror` (+1 more tests). Spanning `254` lines, this module stays free of duplicate state by exposing `SHADERS`, `PlaneLane`, `REGISTERED` and verified by `plane_row_mirror` (+1 more tests).

```rust
--8<-- "session_viewer/src/engine/gpu/planes.rs"
```

### Step 38 · `src/shaders/plane.wgsl`

3. **`src/shaders/plane.wgsl`** synthesizes `22` grid segments (`132` vertices) plus `3` RGB `XYZ` arrows (`54` vertices) per `48 B` `PlaneRow` by calling `arrow_corner`: Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `47`-line shader defines `@vertex fn vs_plane` after `#include` expansion of `arrow.wgsl`. Keep struct field offsets and `@group`/`@binding` indices synchronized with the Rust `bytemuck` mirrors in `cargo xtest`; this `47`-line shader defines `@vertex fn vs_plane` after `#include` expansion of `arrow.wgsl`.

```wgsl
--8<-- "session_viewer/src/shaders/plane.wgsl"
```

### Step 39 · `viewer-core/src/knobs.rs`

Latches diagnostic `VIEWER_*` environment switches once via `OnceLock` for profiling, edge/dot suppression, coplanar edges and BRep refinement. Spanning `60` lines, this module stays free of duplicate state by exposing `profile`, `drop_sessions`, `no_edges`, `no_dots` without introducing UI/GPU coupling cycles. Spanning `60` lines, this module stays free of duplicate state by exposing `profile`, `drop_sessions`, `no_edges`, `no_dots` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/knobs.rs"
```

### Step 40 · `viewer-core/src/interactions.rs`

Separates contact, joint, cut and drill edge interactions from owned element features in deterministic graph-edge creation order. Spanning `202` lines, this module stays free of duplicate state by exposing `Hosted`, `is_interaction`, `hosted`, `own` and verified by `features_group_under_their_interactions`. Spanning `202` lines, this module stays free of duplicate state by exposing `Hosted`, `is_interaction`, `hosted`, `own` and verified by `features_group_under_their_interactions`.

```rust
--8<-- "session_viewer/viewer-core/src/interactions.rs"
```

### Step 41 · `viewer-core/src/walk/mod.rs`

Splits mutable `Upload` borrows via `Walk::solid` and threads `WalkCx` (`row`, `cloud_px`, `arrowhead_pens` override) so converters append faces, edges and dots without cloning tables. Spanning `317` lines, this module stays free of duplicate state by exposing `Walk`, `of`, `WalkCx`, `Row` without introducing UI/GPU coupling cycles. Spanning `317` lines, this module stays free of duplicate state by exposing `Walk`, `of`, `WalkCx`, `Row` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/mod.rs"
```

### Step 42 · `viewer-core/src/walk/encode.rs`

Packs RGBA bytes, maps explicit pixel widths to negative half-widths, and quantizes octahedral normals while reserving `0xffffffff` for unknown facing. Spanning `3` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `3` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/encode.rs"
```

### Step 43 · `viewer-core/src/walk/bounds.rs`

Records pre-append `Baselines`, unions live placed bounds in `file_extent`, and flags `z < 0.001` planar batches as `0.5`-unit drawing sheets. Spanning `118` lines, this module stays free of duplicate state by exposing `Baselines`, `capture`, `file_extent`, `planar_band` without introducing UI/GPU coupling cycles. Spanning `118` lines, this module stays free of duplicate state by exposing `Baselines`, `capture`, `file_extent`, `planar_band` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/bounds.rs"
```

### Step 44 · `viewer-core/src/walk/mesh.rs`

Applies `MeshOpts` (ordinary, smooth sampled, or closed element), estimates spacing from `diag / sqrt(V)`, and drops raw wireframe above `200,000` triangles. Spanning `310` lines, this module stays free of duplicate state by exposing `MESH_RAW_MIN`, `with_wireframe_edges`, `force_wireframe_edges`, `WIREFRAME_BLACK_MIN` without introducing UI/GPU coupling cycles. Spanning `310` lines, this module stays free of duplicate state by exposing `MESH_RAW_MIN`, `with_wireframe_edges`, `force_wireframe_edges`, `WIREFRAME_BLACK_MIN` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/mesh.rs"
```

### Step 45 · `viewer-core/src/walk/mesh_topology.rs`

Maps vertex keys to dense or hash slots, computes Newell polygon normals, and pairs shared edges with hole rings oriented against outer borders. Spanning `270` lines, this module stays free of duplicate state by exposing `SlotMap`, `slot`, `MeshTopo`, `mesh_topology` and verified by `hole_rims_carry_both_faces_without_reordering_source_edges` (+1 more tests). Spanning `270` lines, this module stays free of duplicate state by exposing `SlotMap`, `slot`, `MeshTopo`, `mesh_topology` and verified by `hole_rims_carry_both_faces_without_reordering_source_edges` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/mesh_topology.rs"
```

### Step 46 · `viewer-core/src/walk/mesh_ink.rs`

Suppresses coplanar diagonals, retains borders and creases above `25°`, marks smooth silhouette seams, and switches wireframes above `10,000` edges to black. Spanning `409` lines, this module stays free of duplicate state by exposing `Ink`, `InkCx`, `edges_and_dots` and verified by `box_ink_rows` (+1 more tests). Spanning `409` lines, this module stays free of duplicate state by exposing `Ink`, `InkCx`, `edges_and_dots` and verified by `box_ink_rows` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/mesh_ink.rs"
```

### Step 47 · `viewer-core/src/walk/brep.rs`

Reuses congruent shapes or requests per-face display meshes, retains `(u, v)` samples and face IDs, and sets smooth/open/closed row flags without mutating the BRep. Spanning `915` lines, this module stays free of duplicate state by exposing `QUALITY`, `walk_brep`, `walk_surface` and verified by `repeated_surface_color_metadata_cannot_expand_a_display_candidate` (+8 more tests). Spanning `915` lines, this module stays free of duplicate state by exposing `QUALITY`, `walk_brep`, `walk_surface` and verified by `repeated_surface_color_metadata_cannot_expand_a_display_candidate` (+8 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/brep.rs"
```

### Step 48 · `viewer-core/src/walk/brep_orient.rs`

Traverses shared-edge BFS components on closed shells and flips connected components by signed volume so solid normals point outward reproducibly. Spanning `289` lines, this module stays free of duplicate state by exposing `face_signs` and verified by `kernel_solids_keep_their_normals` (+3 more tests). Spanning `289` lines, this module stays free of duplicate state by exposing `face_signs` and verified by `kernel_solids_keep_their_normals` (+3 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/brep_orient.rs"
```

### Step 49 · `viewer-core/src/walk/brep_shapes.rs`

Scopes thread-local `SharedWalks` to one document walk so congruent shapes replay recorded vertex/edge ranges under distinct placed object rows. Spanning `946` lines, this module stays free of duplicate state by exposing `Frame`, `Marks`, `of`, `Recording` and verified by `a_moved_copy_replays_its_shape` (+1 more tests). Spanning `946` lines, this module stays free of duplicate state by exposing `Frame`, `Marks`, `of`, `Recording` and verified by `a_moved_copy_replays_its_shape` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/brep_shapes.rs"
```

### Step 50 · `viewer-core/src/walk/brep_edges.rs`

Evaluates exact `3D` trim/seam curves, unwraps closed seams, and stores each chord's geometric sag above its facet so curved ink stays visible without global depth bias. Spanning `1736` lines, this module stays free of duplicate state by exposing `EdgeUse`, `iso_chain`, `EdgeChain`, `edge_chains` and verified by `silhouettes_use_flat_triangle_normals_and_exclude_planar_and_boundary_edges` (+17 more tests). Spanning `1736` lines, this module stays free of duplicate state by exposing `EdgeUse`, `iso_chain`, `EdgeChain`, `edge_chains` and verified by `silhouettes_use_flat_triangle_normals_and_exclude_planar_and_boundary_edges` (+17 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/brep_edges.rs"
```

### Step 51 · `viewer-core/src/walk/curves.rs`

Emits chained polyline/NURBS segments with cumulative arc-length dash offsets, `arrowhead_pens`-scaled terminal `VectorRow`s landing on exact endpoints, and marker-aware trims. Spanning `545` lines, this module stays free of duplicate state by exposing `push_polyline`, `push_heads`, `walk_line`, `walk_polyline` and verified by `polyline_heads_aim_along_the_end_segments` (+4 more tests). Spanning `545` lines, this module stays free of duplicate state by exposing `push_polyline`, `push_heads`, `walk_line`, `walk_polyline` and verified by `polyline_heads_aim_along_the_end_segments` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/curves.rs"
```

### Step 52 · `viewer-core/src/walk/plane.rs`

Emits `48 B` `Plane` frame rows (`12`-edge ribbons for `OBB`s) and separates clipping-plane rectangles + cut arrows from orientation grids, triggering solid-mesh verification on first cut. Spanning `199` lines, this module stays free of duplicate state by exposing `NAME`, `is_clipping`, `verify_solids`, `solids_verified` without introducing UI/GPU coupling cycles. Spanning `199` lines, this module stays free of duplicate state by exposing `NAME`, `is_clipping`, `verify_solids`, `solids_verified` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/plane.rs"
```

### Step 53 · `viewer-core/src/walk/points.rs`

Appends one signed-distance dot per `Point` (`-3 px` default radius) while keeping scene bounds restricted to the world point centre. Spanning `23` lines, this module stays free of duplicate state by exposing `walk_point` without introducing UI/GPU coupling cycles. Spanning `23` lines, this module stays free of duplicate state by exposing `walk_point` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/walk/points.rs"
```

### Step 54 · `viewer-core/src/walk/frames.rs`

Converts coordinate frames and vectors into compact `48 B` plane and `64 B` `VectorRow` GPU records instead of tessellated arrow meshes. Spanning `121` lines, this module stays free of duplicate state by exposing `walk_plane`, `walk_obb` and verified by `plane_is_one_row_of_its_frame`. Spanning `121` lines, this module stays free of duplicate state by exposing `walk_plane`, `walk_obb` and verified by `plane_is_one_row_of_its_frame`.

```rust
--8<-- "session_viewer/viewer-core/src/walk/frames.rs"
```

### Step 55 · `viewer-core/src/walk/display.rs`

Quantizes `Quality::for_error` in half-octave steps and subdivides knot spans up to depth `16` and `4,096` segments with a drop-guarded candidate state. Spanning `828` lines, this module stays free of duplicate state by exposing `CURVE_SEGMENTS`, `SURFACE_STEPS`, `OBJECT_BYTES`, `brep_source_bytes` and verified by `inflections_are_bounded_even_when_the_midpoint_is_on_the_chord` (+14 more tests). Spanning `828` lines, this module stays free of duplicate state by exposing `CURVE_SEGMENTS`, `SURFACE_STEPS`, `OBJECT_BYTES`, `brep_source_bytes` and verified by `inflections_are_bounded_even_when_the_midpoint_is_on_the_chord` (+14 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/display.rs"
```

### Step 56 · `viewer-core/src/walk/display_features.rs`

Caps element annotations at `256` features and `4,096` finite points, copying lean display polylines so hidden kernel caches are not retained. Spanning `253` lines, this module stays free of duplicate state by exposing `Cost`, `cost`, `snapshot` and verified by `snapshot_preserves_feature_identity_without_retaining_plane_or_dash` (+2 more tests). Spanning `253` lines, this module stays free of duplicate state by exposing `Cost`, `cost`, `snapshot` and verified by `snapshot_preserves_feature_identity_without_retaining_plane_or_dash` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/display_features.rs"
```

### Step 57 · `viewer-core/src/walk/application_tests.rs`

Verifies element feature/contact polygon ribbons, dots and parent-row ID inheritance when element annotations toggle on/off. Spanning `121` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `interactions_join_the_element_row` (+1 more tests). Spanning `121` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `interactions_join_the_element_row` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/walk/application_tests.rs"
```

### Step 58 · `viewer-core/src/walk/brep/application_tests.rs`

Verifies rational cylindrical NURBS border-to-face vertex sharing, preserved source `.pb` bytes and exact `Δz` surface-preview pipe offsets. Spanning `91` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `exact_rational_border_samples_preserve_faces_source_ids_and_edit_preview`. Spanning `91` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `exact_rational_border_samples_preserve_faces_source_ids_and_edit_preview`.

```rust
--8<-- "session_viewer/viewer-core/src/walk/brep/application_tests.rs"
```

### Step 59 · `viewer-core/src/scene.rs`

`src/app/scene.rs` wraps `Rc<Session>` inside `FileDoc` (name, file `Xform`, `point_size`, shell flag) so two placements can share one kernel `Session` while `(doc_idx, GUID)` maps each placed object to its own GPU display row. Spanning `1704` lines, this module stays free of duplicate state by exposing `READ_ONLY`, `FileDoc`, `Picked`, `PickedPoint` and verified by `duplicate_guids_keep_their_document_and_control_owners` (+8 more tests). Spanning `1704` lines, this module stays free of duplicate state by exposing `READ_ONLY`, `FileDoc`, `Picked`, `PickedPoint` and verified by `duplicate_guids_keep_their_document_and_control_owners` (+8 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene.rs"
```

### Step 60 · `viewer-core/src/scene_rows.rs`

Separates `(doc_idx, GUID)` row identity (plus text/free/sink/tomb sentinels) from `Footprint` (`One`, `Many`, `Cloud`, `None`) lane spans. Spanning `314` lines, this module stays free of duplicate state by exposing `EdgeStep`, `TEXT`, `FREE`, `SINK` and verified by `footprint_is_12_bytes_and_counts_roundtrip` (+1 more tests). Spanning `314` lines, this module stays free of duplicate state by exposing `EdgeStep`, `TEXT`, `FREE`, `SINK` and verified by `footprint_is_12_bytes_and_counts_roundtrip` (+1 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene_rows.rs"
```

### Step 61 · `viewer-core/src/scene_sync.rs`

Drains change notes, deduplicates identities, expands `SUBTREE` descendants and emits ordered `Staged` GPU updates (`retire`, `bury`, `patch`, `geometry`, `place`). Spanning `647` lines, this module stays free of duplicate state by exposing `TOMB_CAP`, `Work`, `has_pending`, `dead_bytes` without introducing UI/GPU coupling cycles. Spanning `647` lines, this module stays free of duplicate state by exposing `TOMB_CAP`, `Work`, `has_pending`, `dead_bytes` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync.rs"
```

### Step 62 · `viewer-core/src/scene_sync/allocation.rs`

Reuses an identity's grave when all lane counts match, pads drag previews by ~`50%`, and routes retired tails to the hidden sink row. Spanning `142` lines, this module stays free of duplicate state by exposing `place_rows`, `append`, `retire`, `kill_lanes` without introducing UI/GPU coupling cycles. Spanning `142` lines, this module stays free of duplicate state by exposing `place_rows`, `append`, `retire`, `kill_lanes` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/allocation.rs"
```

### Step 63 · `viewer-core/src/scene_sync/compaction.rs`

`src/app/scene_sync/compaction.rs` triggers GPU lane compaction once dead bytes reach `16 MiB` and dead + tomb bytes exceed live bytes, rebuilding buffer bind groups and skipping shared `Rc<Session>` purges, while `src/engine/gpu/scene_gpu_tests/` (`mod.rs`, `contacts.rs`, `frames.rs`, `instances.rs`, `split.rs`) verifies offscreen pixel and object-ID parity across edits, undos, instancing and compactions on a native headless GPU adapter. Spanning `174` lines, this module stays free of duplicate state by exposing `compaction_due`, `purge_step`, `cloud_compaction_due`, `compact_clouds` without introducing UI/GPU coupling cycles. Spanning `174` lines, this module stays free of duplicate state by exposing `compaction_due`, `purge_step`, `cloud_compaction_due`, `compact_clouds` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/compaction.rs"
```

### Step 64 · `viewer-core/src/scene_sync/nodes.rs`

Resolves affected tree nodes via weak caches and batched per-document searches, multiplying file `Xform` by root-to-node transforms. Spanning `274` lines, this module stays free of duplicate state by exposing `find_nodes`, `cached`, `cache_hit`, `beside_parent` without introducing UI/GPU coupling cycles. Spanning `274` lines, this module stays free of duplicate state by exposing `find_nodes`, `cached`, `cache_hit`, `beside_parent` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/nodes.rs"
```

### Step 65 · `viewer-core/src/scene_sync/notes.rs`

Translates forward and reverse kernel transaction ops into `GEOMETRY`, `PLACE`, `PRESENCE` and `SUBTREE` invalidation flags without rebuilding untouched objects. Spanning `84` lines, this module stays free of duplicate state by exposing `commit`, `stepped` without introducing UI/GPU coupling cycles. Spanning `84` lines, this module stays free of duplicate state by exposing `commit`, `stepped` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/notes.rs"
```

### Step 66 · `viewer-core/src/scene_sync/tombs.rs`

Retains up to `256 MiB` of deleted lane spans keyed by weak kernel tomb pointers so `Undo` unburies rows without re-tessellating geometry. Spanning `204` lines, this module stays free of duplicate state by exposing `record_of`, `bury`, `revive`, `free_tomb` without introducing UI/GPU coupling cycles. Spanning `204` lines, this module stays free of duplicate state by exposing `record_of`, `bury`, `revive`, `free_tomb` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/tombs.rs"
```

### Step 67 · `viewer-core/src/scene_sync/testing.rs`

Models `settle` upload bookkeeping on the CPU and compares incremental edits bit-for-bit against a fresh multi-document walk (`verify`). Spanning `135` lines, this module stays free of duplicate state by exposing `settle`, `rewalk_cpu`, `verify` without introducing UI/GPU coupling cycles. Spanning `135` lines, this module stays free of duplicate state by exposing `settle`, `rewalk_cpu`, `verify` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/testing.rs"
```

### Step 68 · `viewer-core/src/scene_sync/tests.rs`

Runs `150`-step seeded random edit sequences (including a `4,096 B` tomb cap and cache on/off parity) against the fresh-scene oracle. Spanning `216` lines, this module stays free of duplicate state by exposing `file`, `p`, `arc`, `other` without introducing UI/GPU coupling cycles. Spanning `216` lines, this module stays free of duplicate state by exposing `file`, `p`, `arc`, `other` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/scene_sync/tests.rs"
```

### Step 69 · `src/engine/gpu/scene_gpu_tests/mod.rs`

`src/app/scene_sync/compaction.rs` triggers GPU lane compaction once dead bytes reach `16 MiB` and dead + tomb bytes exceed live bytes, rebuilding buffer bind groups and skipping shared `Rc<Session>` purges, while `src/engine/gpu/scene_gpu_tests/` (`mod.rs`, `contacts.rs`, `frames.rs`, `instances.rs`, `split.rs`) verifies offscreen pixel and object-ID parity across edits, undos, instancing and compactions on a native headless GPU adapter. Spanning `4` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `4` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/scene_gpu_tests/mod.rs"
```

### Step 70 · `src/engine/gpu/scene_gpu_tests/contacts.rs`

`src/app/scene_sync/compaction.rs` triggers GPU lane compaction once dead bytes reach `16 MiB` and dead + tomb bytes exceed live bytes, rebuilding buffer bind groups and skipping shared `Rc<Session>` purges, while `src/engine/gpu/scene_gpu_tests/` (`mod.rs`, `contacts.rs`, `frames.rs`, `instances.rs`, `split.rs`) verifies offscreen pixel and object-ID parity across edits, undos, instancing and compactions on a native headless GPU adapter. Spanning `102` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `102` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/scene_gpu_tests/contacts.rs"
```

### Step 71 · `src/engine/gpu/scene_gpu_tests/frames.rs`

`src/app/scene_sync/compaction.rs` triggers GPU lane compaction once dead bytes reach `16 MiB` and dead + tomb bytes exceed live bytes, rebuilding buffer bind groups and skipping shared `Rc<Session>` purges, while `src/engine/gpu/scene_gpu_tests/` (`mod.rs`, `contacts.rs`, `frames.rs`, `instances.rs`, `split.rs`) verifies offscreen pixel and object-ID parity across edits, undos, instancing and compactions on a native headless GPU adapter. Spanning `222` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `authored_line_widths_reach_the_pixels`. Spanning `222` lines, this module stays free of duplicate state by wiring internal module helpers and verified by `authored_line_widths_reach_the_pixels`.

```rust
--8<-- "session_viewer/src/engine/gpu/scene_gpu_tests/frames.rs"
```

### Step 72 · `src/engine/gpu/scene_gpu_tests/instances.rs`

`src/app/scene_sync/compaction.rs` triggers GPU lane compaction once dead bytes reach `16 MiB` and dead + tomb bytes exceed live bytes, rebuilding buffer bind groups and skipping shared `Rc<Session>` purges, while `src/engine/gpu/scene_gpu_tests/` (`mod.rs`, `contacts.rs`, `frames.rs`, `instances.rs`, `split.rs`) verifies offscreen pixel and object-ID parity across edits, undos, instancing and compactions on a native headless GPU adapter. Spanning `409` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `409` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/scene_gpu_tests/instances.rs"
```

### Step 73 · `src/engine/gpu/scene_gpu_tests/split.rs`

`src/app/scene_sync/compaction.rs` triggers GPU lane compaction once dead bytes reach `16 MiB` and dead + tomb bytes exceed live bytes, rebuilding buffer bind groups and skipping shared `Rc<Session>` purges, while `src/engine/gpu/scene_gpu_tests/` (`mod.rs`, `contacts.rs`, `frames.rs`, `instances.rs`, `split.rs`) verifies offscreen pixel and object-ID parity across edits, undos, instancing and compactions on a native headless GPU adapter. Spanning `758` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles. Spanning `758` lines, this module stays free of duplicate state by wiring internal module helpers without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/engine/gpu/scene_gpu_tests/split.rs"
```

### Step 74 · `examples/course/rows.rs`

`examples/course/rows.rs` runs the Tutorial 4 row-table checkpoint (`?tutorial=rows`), uploading `112`-byte `Instance` records and camera-anchored `16`-byte translations through `InstanceTable` and validating the 6 shared WGSL headers. Trap: subtract the `f64` camera anchor before casting world coordinates to `f32`, or objects far from the origin jitter during orbit.

```rust
--8<-- "session_viewer/examples/course/rows.rs"
```

### Step 75 · `examples/course/geometry.rs`

`examples/course/geometry.rs` runs the Tutorial 4 geometry checkpoint (`?tutorial=geometry`), walking a kernel `Mesh` cube, a `BRep` solid, and an arrowed `NurbsCurve` into GPU lanes (with no 3D matrix lane). Trap: BRep trim loops and crease edges must carry their original face normals into `SegTable` so hidden-line ink visibility agrees with solid shading.

```rust
--8<-- "session_viewer/examples/course/geometry.rs"
```

## Integration

`viewer-core/src/lib.rs` exports `rows`, `sink` (`GpuSink`), `knobs`, `interactions`, `walk`, and `scene`, while `src/engine/gpu/sink.rs` implements `GpuSink` for `Gpu`. `Scene::upload_to` pushes `Upload` batches across that trait boundary without `viewer-core` ever importing `wgpu`.

## Run and observe

Run `cargo xtest -j8 packed_vertex_is_twenty_bytes`, `octahedral_normals_round_trip`, `instance_mirror`, `arrow_is_exactly_as_long_as_its_line`, `teapot_boundary_ink_follows_authored_patch_curves`, and `edits_write_only_their_own_slots`: all pass with worst arrow-tip error `0.010 px` and `0` base-seam pixels.

## Predict and try

Predict how many bytes upload to the GPU when you translate one `50,000`-triangle mesh by `[100, 0, 0]`: `0` vertex bytes and `0` model-matrix bytes—`InstanceTable` updates only that row's `16`-byte `[f32; 4]` camera-anchored translation entry (`a_move_goes_into_the_translation_not_the_matrix`).

## Question and answer

**Why does deleting an object park its GPU slots in `tombs.rs` instead of immediately compacting `Arena` and `SegTable`?**

**How to work it out:** Consider pressing `Delete` on a `200,000`-triangle solid and immediately pressing `Ctrl+Z` (`Undo`): immediate compaction would shift every subsequent object's buffer range and discard the uploaded triangles.

**The answer:** Parking deleted ranges in `tombs.rs` (`MAX_TOMBS` cap) hides the row in `O(1)` via `FLAG_HIDDEN` so `Undo` restores the exact row ID and GPU vertices with zero re-walk or upload; `compaction.rs` reclaims dead arena space only when tomb bytes cross the high-water budget or redo history drops unreachable graves.

## Validation

Verified via `cargo xtest -p viewer-core walk::` (`59` walk tests), `cargo xtest -p viewer-core scene::` (`46` scene/sync tests), and `cargo xtest -p session_viewer engine::gpu::` (`packed_vertex_is_twenty_bytes`, `arrow_is_exactly_as_long_as_its_line`, `frustum_outcode_culls_offscreen_boxes_and_keeps_visible_ones`).
