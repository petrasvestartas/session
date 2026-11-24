# Tutorial 15 · File formats, bounded protobuf decode, `.session` archives and live reload

## Goal

Parse YAML/JSON/TOML manifests with identical semantics, pre-validate untrusted `.pb` wire bounds (`<= 512 MiB`) before kernel allocation, yield every `25,000` objects on the browser thread, persist canonical `.session` archives (including camera/display state and `.obj` export), and hot-reload via ETag/SSE.

## Diagram

![URL `route` -> `Manifest` -> `RangeGate` (`4` slots) -> `validate` (`<= 512 MiB`) -> `decode` (`25k` yields) -> canonical `session_io` and `ETag`/SSE live reload (native wgpu readback of stage deck)](../../examples/stage/tutorials-systems/15-formats.png)

Stage deck source: [`15-formats.yaml`](../../examples/stage/tutorials-systems/15-formats.yaml).

## Prerequisites and files

Requires [Tutorial 14](tutorial-fourteen-fastmcp.md). Owns `yielding`, `{viewer-core,app}/{validate,decode,manifest,range_gate,stream,live_revision,scene_release,session_io}`, `app/{route,fetch,loader,live}`, `state/hydrate`, 4 I/O verbs, `assets/view_local.yaml`, and `examples/course/decoding.rs`.

Owned files in build order (`23` files):

1. `src/app/yielding.rs`
2. `viewer-core/src/validate.rs`
3. `viewer-core/src/decode.rs`
4. `src/app/decode.rs`
5. `viewer-core/src/manifest.rs`
6. `src/app/route.rs`
7. `src/app/fetch.rs`
8. `viewer-core/src/range_gate.rs`
9. `src/app/loader.rs`
10. `src/state/hydrate.rs`
11. `viewer-core/src/stream.rs`
12. `src/app/stream.rs`
13. `src/app/live.rs`
14. `viewer-core/src/live_revision.rs`
15. `viewer-core/src/scene_release.rs`
16. `viewer-core/src/session_io.rs`
17. `src/app/session_io.rs`
18. `viewer-core/src/command/verbs/open.rs`
19. `viewer-core/src/command/verbs/save.rs`
20. `viewer-core/src/command/verbs/import.rs`
21. `viewer-core/src/command/verbs/export.rs`
22. `assets/view_local.yaml`
23. `examples/course/decoding.rs`

## Build

### Step 1 · `src/app/yielding.rs`

`src/app/yielding.rs` pauses async decoding every `25,000` converted objects by awaiting a zero-millisecond browser timer Promise (`yield_now`), keeping `wasm32` single-threaded without holding any `RefCell` borrow across `.await`. Spanning `21` lines, this module stays free of duplicate state by exposing `sleep_ms`, `next_tick` without introducing UI/GPU coupling cycles. Spanning `21` lines, this module stays free of duplicate state by exposing `sleep_ms`, `next_tick` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/yielding.rs"
```

### Step 2 · `viewer-core/src/validate.rs`

`src/app/validate.rs` checks complete varints (`validate::varint`) and enclosing message bounds (`Reader::field`) so unknown valid fields skip cleanly while truncated tags fail, then validates decoded `prost` mesh vertex indices, cloud column lengths and NURBS control/knot invariants before constructing `session_rust` objects. Spanning `684` lines, this module stays free of duplicate state by exposing `MAX_OBJECTS`, `TOO_MANY`, `MAX_DEPTH`, `TOO_DEEP` and verified by `hostile_counts_do_not_reach_kernel_allocations` (+2 more tests). Spanning `684` lines, this module stays free of duplicate state by exposing `MAX_OBJECTS`, `TOO_MANY`, `MAX_DEPTH`, `TOO_DEEP` and verified by `hostile_counts_do_not_reach_kernel_allocations` (+2 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/validate.rs"
```

### Step 3 · `viewer-core/src/decode.rs`

`src/app/decode.rs` reads `Body::Bytes` or windowed `1 MiB` `Body::Js` slices with checked offsets, enforces a `512 MiB` whole-file cap, counts at most `2,000,000` geometry records (`Rc`-shared between typed tables and GUID maps), and rebuilds the hierarchy on an explicit `Vec` stack capped at depth `64` plus bidirectional graph edges and local instance transforms. Spanning `1050` lines, this module stays free of duplicate state by exposing `ByteSource`, `Body`, `size`, `session_from_bytes` and verified by `window_decode_keeps_node_colors` (+6 more tests). Spanning `1050` lines, this module stays free of duplicate state by exposing `ByteSource`, `Body`, `size`, `session_from_bytes` and verified by `window_decode_keeps_node_colors` (+6 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/decode.rs"
```

### Step 4 · `src/app/decode.rs`

`src/app/decode.rs` reads `Body::Bytes` or windowed `1 MiB` `Body::Js` slices with checked offsets, enforces a `512 MiB` whole-file cap, counts at most `2,000,000` geometry records (`Rc`-shared between typed tables and GUID maps), and rebuilds the hierarchy on an explicit `Vec` stack capped at depth `64` plus bidirectional graph edges and local instance transforms. Spanning `67` lines, this module stays free of duplicate state by exposing `JsBytes`, `Body`, `size`, `session_from_bytes` without introducing UI/GPU coupling cycles. Spanning `67` lines, this module stays free of duplicate state by exposing `JsBytes`, `Body`, `size`, `session_from_bytes` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/decode.rs"
```

### Step 5 · `viewer-core/src/manifest.rs`

Parses YAML scene manifests (`files`, `xform` overriding `at`, `point_size`, authored `text`, `view` and `grid`) independently of kernel `.pb` bytes. Spanning `457` lines, this module stays free of duplicate state by exposing `Item`, `TextItem`, `Manifest`, `encoded_size` and verified by `yaml_json_and_toml_have_identical_scene_semantics` (+8 more tests). Spanning `457` lines, this module stays free of duplicate state by exposing `Item`, `TextItem`, `Manifest`, `encoded_size` and verified by `yaml_json_and_toml_have_identical_scene_semantics` (+8 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/manifest.rs"
```

### Step 6 · `src/app/route.rs`

Validates scene query names (rejecting `..`, leading `/` and URL schemes) and resolves relative paths against same-origin (`data=off`) or configured data bases. Spanning `224` lines, this module stays free of duplicate state by exposing `DATA_BASE`, `LOCAL_SCENE`, `AUTO_GRID`, `is_local_url` without introducing UI/GPU coupling cycles. Spanning `224` lines, this module stays free of duplicate state by exposing `DATA_BASE`, `LOCAL_SCENE`, `AUTO_GRID`, `is_local_url` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/route.rs"
```

### Step 7 · `src/app/fetch.rs`

`src/app/fetch.rs` takes one-shot ownership of pre-started `window.__viewerEarly` Promises/AbortControllers (separating `8,192 B` head probes from full GETs), leaves large payloads in JS `Uint8Array`s until needed, resets stall timers per chunk, and caps whole-file bodies at `512 MiB`. Spanning `529` lines, this module stays free of duplicate state by exposing `PROBE_BYTES`, `retryable`, `Reply`, `whole` without introducing UI/GPU coupling cycles. Spanning `529` lines, this module stays free of duplicate state by exposing `PROBE_BYTES`, `retryable`, `Reply`, `whole` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/fetch.rs"
```

### Step 8 · `viewer-core/src/range_gate.rs`

Caps concurrent HTTP `Range` reads at `3` permits in FIFO `Future` order, handing slots forward on `Drop` and waking waiters when a scene closes. Spanning `252` lines, this module stays free of duplicate state by exposing `RANGE_READS`, `RangeGate`, `enter`, `close` and verified by `gate_runs_a_few_reads_and_queues_the_rest_in_order` (+3 more tests). Spanning `252` lines, this module stays free of duplicate state by exposing `RANGE_READS`, `RangeGate`, `enter`, `close` and verified by `gate_runs_a_few_reads_and_queues_the_rest_in_order` (+3 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/range_gate.rs"
```

### Step 9 · `src/app/loader.rs`

Overlaps `8 KiB` protobuf probes with GPU setup, tracks separate load and stream generations, and decrements active-load counts via an RAII `Loading` drop guard. Spanning `1053` lines, this module stays free of duplicate state by exposing `post_stage`, `post`, `boot`, `reload_scene` without introducing UI/GPU coupling cycles. Spanning `1053` lines, this module stays free of duplicate state by exposing `post_stage`, `post`, `boot`, `reload_scene` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/loader.rs"
```

### Step 10 · `src/state/hydrate.rs`

`src/state/hydrate.rs` distinguishes read-only streamed shells from releasable documents, queues pending `Save`/edit `Resume` actions, and re-fetches source `.pb` bytes validated against the current document index and release token. Spanning `161` lines, this module stays free of duplicate state by exposing `Resume`, `fetch_wanted`, `resume_after`, `locked_reason` without introducing UI/GPU coupling cycles. Spanning `161` lines, this module stays free of duplicate state by exposing `Resume`, `fetch_wanted`, `resume_after`, `locked_reason` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/state/hydrate.rs"
```

### Step 11 · `viewer-core/src/stream.rs`

`src/app/stream.rs` parses bounded `CloudFields` and `SheetFields` byte ranges (`field_at`), validating element strides, segment counts, packed-`f64` LOD alignment and source `ETag` revisions before issuing HTTP `Range` reads. Spanning `839` lines, this module stays free of duplicate state by exposing `CloudFields`, `SheetFields`, `Field`, `field_at` and verified by `normals_pack_three_doubles_per_point` (+7 more tests). Spanning `839` lines, this module stays free of duplicate state by exposing `CloudFields`, `SheetFields`, `Field`, `field_at` and verified by `normals_pack_three_doubles_per_point` (+7 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/stream.rs"
```

### Step 12 · `src/app/stream.rs`

`src/app/stream.rs` parses bounded `CloudFields` and `SheetFields` byte ranges (`field_at`), validating element strides, segment counts, packed-`f64` LOD alignment and source `ETag` revisions before issuing HTTP `Range` reads. Spanning `422` lines, this module stays free of duplicate state by exposing `reset_range_gate`, `read_window`, `start_range`, `probe` without introducing UI/GPU coupling cycles. Spanning `422` lines, this module stays free of duplicate state by exposing `reset_range_gate`, `read_window`, `start_range`, `probe` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/stream.rs"
```

### Step 13 · `src/app/live.rs`

`src/app/live.rs` manages an `EventSource` change relay plus polling fallback, marking a recheck flag without trusting relay bytes and detaching DOM listeners cleanly on `Drop`. Spanning `473` lines, this module stays free of duplicate state by exposing `DEFAULT_SOURCE`, `LiveSource`, `texts`, `take_opening` without introducing UI/GPU coupling cycles. Spanning `473` lines, this module stays free of duplicate state by exposing `DEFAULT_SOURCE`, `LiveSource`, `texts`, `take_opening` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/live.rs"
```

### Step 14 · `viewer-core/src/live_revision.rs`

`src/app/live_revision.rs` (`Revisions`) tracks HTTP `ETag` validators separately from received content hashes and stages complete replacements so unchanged polls or failed mid-reload fetches never reset the active camera or scene. Spanning `53` lines, this module stays free of duplicate state by exposing `Revisions`, `etag`, `forget`, `changed` and verified by `identical_bytes_survive_strong_weak_missing_and_new_etags`. Spanning `53` lines, this module stays free of duplicate state by exposing `Revisions`, `etag`, `forget`, `changed` and verified by `identical_bytes_survive_strong_weak_missing_and_new_etags`.

```rust
--8<-- "session_viewer/viewer-core/src/live_revision.rs"
```

### Step 15 · `viewer-core/src/scene_release.rs`

`src/app/scene_release.rs` drops resident `Session` geometry on unmodified display-only documents while keeping tree nodes, names, shape summaries, GPU rows and a unique release token for later identity-checked re-hydration. Spanning `509` lines, this module stays free of duplicate state by exposing `release`, `released_doc`, `shape`, `released_object_name` and verified by `release_keeps_rows_and_tree_and_refuses_edits` (+4 more tests). Spanning `509` lines, this module stays free of duplicate state by exposing `release`, `released_doc`, `shape`, `released_object_name` and verified by `release_keeps_rows_and_tree_and_refuses_edits` (+4 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/scene_release.rs"
```

### Step 16 · `viewer-core/src/session_io.rs`

Reads and writes versioned `.session` archives (`<= 512 MiB`) combining stamped document `.pb` payloads with JSON placements, overrides, text and groups. Spanning `967` lines, this module stays free of duplicate state by exposing `SavedCamera`, `from_camera`, `apply_to`, `SavedView` and verified by `lamps_load_from_and_save_to_is_visible` (+5 more tests). Spanning `967` lines, this module stays free of duplicate state by exposing `SavedCamera`, `from_camera`, `apply_to`, `SavedView` and verified by `lamps_load_from_and_save_to_is_visible` (+5 more tests).

```rust
--8<-- "session_viewer/viewer-core/src/session_io.rs"
```

### Step 17 · `src/app/session_io.rs`

Reads and writes versioned `.session` archives (`<= 512 MiB`) combining stamped document `.pb` payloads with JSON placements, overrides, text and groups. Spanning `105` lines, this module stays free of duplicate state by exposing `download`, `download_text`, `pick`, `pick_import` without introducing UI/GPU coupling cycles. Spanning `105` lines, this module stays free of duplicate state by exposing `download`, `download_text`, `pick`, `pick_import` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/src/app/session_io.rs"
```

### Step 18 · `viewer-core/src/command/verbs/open.rs`

Opens the browser file picker on `wasm32` and posts selected `.pb` or `.session` payloads through the async loader. Spanning `21` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `21` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/open.rs"
```

### Step 19 · `viewer-core/src/command/verbs/save.rs`

Hydrates any released documents first, then serializes the `.session` archive and triggers browser download delivery. Spanning `25` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `25` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/save.rs"
```

### Step 20 · `viewer-core/src/command/verbs/import.rs`

`export.rs` writes selected or visible surface/mesh geometry to Wavefront OBJ via the kernel writer; `import.rs` imports local `.pb`, `.obj` or `.session` files. Spanning `25` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `25` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/import.rs"
```

### Step 21 · `viewer-core/src/command/verbs/export.rs`

`export.rs` writes selected or visible surface/mesh geometry to Wavefront OBJ via the kernel writer; `import.rs` imports local `.pb`, `.obj` or `.session` files. Spanning `25` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles. Spanning `25` lines, this module stays free of duplicate state by registering `SPEC` in `verbs!` without introducing UI/GPU coupling cycles.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/export.rs"
```

### Step 22 · `assets/view_local.yaml`

`assets/view_local.yaml` lists default local scene URLs, display names, `xform`/`at` instance placements and `point_size` overrides resolved relative to the active manifest route. Keep this `45`-line configuration synchronized with the workspace toolchain and deterministic build rules. Keep this `45`-line configuration synchronized with the workspace toolchain and deterministic build rules.

```yaml
--8<-- "session_viewer/assets/view_local.yaml"
```

### Step 23 · `examples/course/decoding.rs`

`examples/course/decoding.rs` runs the Tutorial 15 decoding checkpoint (`?tutorial=decoding`), validating wire bounds with `validate.rs` and decoding protobuf `.pb` bytes with `25,000`-object browser yields. Trap: reject oversized count varints and non-finite coordinates in `validate.rs` before constructing any `session_rust` heap container.

```rust
--8<-- "session_viewer/examples/course/decoding.rs"
```

## Integration

`ByteSource` (`viewer-core/src/decode.rs`) decouples protobuf window decoding from browser `Uint8Array` views (`src/app/decode.rs`), while `session_io::save_with_state` canonicalizes `Mesh` protobuf `map<uint64, ...>` entries and persists `SavedCamera` + `SavedView` inside `Metadata` JSON.

## Run and observe

Run `cargo xtest -p viewer-core validate::tests`, `decode::tests`, `manifest::tests`, `session_io::tests`, `stream::tests`, and `range_gate::tests`: hostile count varints are rejected before allocation, YAML/JSON/TOML manifests produce identical scenes, and saved `.session` archives restore objects, layers, lamps, locks, colours, alphas, camera pose, and view modes.

## Predict and try

Predict what happens if two separate processes open the same `Mesh` `.pb` file (`Mesh.vertex` uses Rust `HashMap` with randomized `RandomState`) and save `.session` without `canonicalize_mesh_bytes`: their `.session` files would differ in vertex/face map entry order! Sorting protobuf map tags `3`, `4`, and `17` by ascending `u64` key in `session_io.rs` makes saves 100% deterministic across processes.

## Question and answer

**Why must `scene_release.rs` refuse `Save` on a `display_only: true` document until `state/hydrate.rs` re-fetches its full `.pb` source?**

**How to work it out:** Compare what `Scene` retains after `release_display_only` drops the `Rc<Session>` against what `session_io::save_with_state` writes into `Archive.documents`.

**The answer:** `display_only` releases the heavy `session_rust::Session` source from WASM RAM after uploading `20 B` packed GPU vertices so multi-document scenes stay within the `4 GiB` browser heap. Because GPU display vertices are tessellated and camera-quantized (lacking exact NURBS knots, trim curves, and metadata), saving without first hydrating the original `.pb` bytes via `hydrate.rs` would corrupt source fidelity.

## Validation

Verified via `validate::tests` (`3/3`), `decode::tests` (`7/7`), `manifest::tests` (`6/6`), `range_gate::tests` (`4/4`), `stream::tests` (`7/7`), `live_revision::tests` (`1/1`), `scene::release::editing_tests` (`5/5`), and `session_io::tests` (`6/6`).
