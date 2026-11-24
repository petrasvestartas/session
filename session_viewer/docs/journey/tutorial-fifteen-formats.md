# Tutorial 15 · File formats, bounded protobuf decode, `.session` archives and live reload

## Goal

Parse YAML/JSON/TOML manifests with identical semantics, pre-validate untrusted `.pb` wire bounds (`<= 512 MiB`) before kernel allocation, yield every `25,000` objects on the browser thread, persist canonical `.session` archives (including camera/display state and `.obj` export), and hot-reload via ETag/SSE.

![Three-asset YAML manifest placing Lion point cloud, CAD pedestal, and upright Bunny in one scene](../screenshots/tutorials/t15-manifest.png) ![Restored .session archive recovering saved camera, custom object colours, and live section cut](../screenshots/tutorials/t15-session.png)

**What we achieve:** Safe wire-validated `.pb` protobuf decoding, multi-document placed YAML manifests, lossless `.session` workspace save/restore, and standard CAD mesh/point-cloud import/export (`OBJ`, `STL`, `PLY`).

## How it works

`validate.rs` bounds-checks protobuf varints and finite `f64` coordinates before heap allocation, `stream.rs` yields to the browser every `25,000` objects so the UI stays responsive, and `.session` archives round-trip full scene geometry alongside camera, layer, colour, and clip-plane state.

![URL `route` -> `Manifest` -> `RangeGate` (`4` slots) -> `validate` (`<= 512 MiB`) -> `decode` (`25k` yields) -> canonical `session_io` and `ETag`/SSE live reload (1600x900 Session Viewer stage plate)](../../examples/stage/tutorials-systems/15-formats.png)

Interactive stage source: [`15-formats.yaml`](../../examples/stage/tutorials-systems/15-formats.yaml).

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

`src/app/yielding.rs` pauses async decoding every `25,000` converted objects by awaiting a zero-millisecond browser timer Promise (`yield_now`), keeping `wasm32` single-threaded without holding any `RefCell` borrow across `.await`.

```rust
--8<-- "session_viewer/src/app/yielding.rs"
```

### Step 2 · `viewer-core/src/validate.rs`

`src/app/validate.rs` checks complete varints (`validate::varint`) and enclosing message bounds (`Reader::field`) so unknown valid fields skip cleanly while truncated tags fail, then validates decoded `prost` mesh vertex indices, cloud column lengths and NURBS control/knot invariants before constructing `session_rust` objects.

```rust
--8<-- "session_viewer/viewer-core/src/validate.rs"
```

### Step 3 · `viewer-core/src/decode.rs`

`viewer-core/src/decode.rs` (1050 lines) defines `trait ByteSource`, `enum Body`, `size()`, `session_from_bytes()`, `session_from_bytes_yielding()`. It verified by `cargo xtest` (`window_decode_keeps_node_colors`, `window_decode_matches_the_source`, `window_decode_keeps_interactions_on_their_edges` (+4 more)).

```rust
--8<-- "session_viewer/viewer-core/src/decode.rs"
```

### Step 4 · `src/app/decode.rs`

`src/app/decode.rs` (67 lines) defines `struct JsBytes`, `enum Body`, `size()`, `session_from_bytes()`, `session_from_body()`. It keeps browser event routing and scene mutations synchronized without redundant redraws.

```rust
--8<-- "session_viewer/src/app/decode.rs"
```

### Step 5 · `viewer-core/src/manifest.rs`

Parses YAML scene manifests (`files`, `xform` overriding `at`, `point_size`, authored `text`, `view` and `grid`) independently of kernel `.pb` bytes.

```rust
--8<-- "session_viewer/viewer-core/src/manifest.rs"
```

### Step 6 · `src/app/route.rs`

Validates scene query names (rejecting `..`, leading `/` and URL schemes) and resolves relative paths against same-origin (`data=off`) or configured data bases.

```rust
--8<-- "session_viewer/src/app/route.rs"
```

### Step 7 · `src/app/fetch.rs`

`src/app/fetch.rs` takes one-shot ownership of pre-started `window.__viewerEarly` Promises/AbortControllers (separating `8,192 B` head probes from full GETs), leaves large payloads in JS `Uint8Array`s until needed, resets stall timers per chunk, and caps whole-file bodies at `512 MiB`.

```rust
--8<-- "session_viewer/src/app/fetch.rs"
```

### Step 8 · `viewer-core/src/range_gate.rs`

Caps concurrent HTTP `Range` reads at `3` permits in FIFO `Future` order, handing slots forward on `Drop` and waking waiters when a scene closes.

```rust
--8<-- "session_viewer/viewer-core/src/range_gate.rs"
```

### Step 9 · `src/app/loader.rs`

Overlaps `8 KiB` protobuf probes with GPU setup, tracks separate load and stream generations, and decrements active-load counts via an RAII `Loading` drop guard.

```rust
--8<-- "session_viewer/src/app/loader.rs"
```

### Step 10 · `src/state/hydrate.rs`

`src/state/hydrate.rs` distinguishes read-only streamed shells from releasable documents, queues pending `Save`/edit `Resume` actions, and re-fetches source `.pb` bytes validated against the current document index and release token.

```rust
--8<-- "session_viewer/src/state/hydrate.rs"
```

### Step 11 · `viewer-core/src/stream.rs`

`viewer-core/src/stream.rs` (839 lines) defines `struct CloudFields`, `struct SheetFields`, `struct Field`, `field_at()`, `set()`. It verified by `cargo xtest` (`normals_pack_three_doubles_per_point`, `metadata_window_checks_ranges_without_copying_or_following_large_skips`, `lod_parallel_arrays_bounds_ranges_and_child_graph_are_validated` (+5 more)).

```rust
--8<-- "session_viewer/viewer-core/src/stream.rs"
```

### Step 12 · `src/app/stream.rs`

`src/app/stream.rs` (422 lines) defines `reset_range_gate()`, `read_window()`, `start_range()`. It keeps browser event routing and scene mutations synchronized without redundant redraws.

```rust
--8<-- "session_viewer/src/app/stream.rs"
```

### Step 13 · `src/app/live.rs`

`src/app/live.rs` manages an `EventSource` change relay plus polling fallback, marking a recheck flag without trusting relay bytes and detaching DOM listeners cleanly on `Drop`.

```rust
--8<-- "session_viewer/src/app/live.rs"
```

### Step 14 · `viewer-core/src/live_revision.rs`

`src/app/live_revision.rs` (`Revisions`) tracks HTTP `ETag` validators separately from received content hashes and stages complete replacements so unchanged polls or failed mid-reload fetches never reset the active camera or scene.

```rust
--8<-- "session_viewer/viewer-core/src/live_revision.rs"
```

### Step 15 · `viewer-core/src/scene_release.rs`

`src/app/scene_release.rs` drops resident `Session` geometry on unmodified display-only documents while keeping tree nodes, names, shape summaries, GPU rows and a unique release token for later identity-checked re-hydration.

```rust
--8<-- "session_viewer/viewer-core/src/scene_release.rs"
```

### Step 16 · `viewer-core/src/session_io.rs`

`viewer-core/src/session_io.rs` (967 lines) defines `struct SavedCamera`, `struct SavedView`, `struct OpenedSession`, `from_camera()`, `apply_to()`. It verified by `cargo xtest` (`lamps_load_from_and_save_to_is_visible`, `locks_load_from_and_save_to_is_locked`, `a_created_text_survives_save_and_open` (+3 more)).

```rust
--8<-- "session_viewer/viewer-core/src/session_io.rs"
```

### Step 17 · `src/app/session_io.rs`

`src/app/session_io.rs` (105 lines) defines `download()`, `download_text()`, `pick()`. It keeps browser event routing and scene mutations synchronized without redundant redraws.

```rust
--8<-- "session_viewer/src/app/session_io.rs"
```

### Step 18 · `viewer-core/src/command/verbs/open.rs`

`viewer-core/src/command/verbs/open.rs` implements the `open` command verb: executes its typed command specification Validates arguments against `&mut dyn CommandHost` before mutating scene or view state so invalid interactive or CLI input leaves undo history untouched.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/open.rs"
```

### Step 19 · `viewer-core/src/command/verbs/save.rs`

`viewer-core/src/command/verbs/save.rs` implements the `save` command verb: executes its typed command specification Validates arguments against `&mut dyn CommandHost` before mutating scene or view state so invalid interactive or CLI input leaves undo history untouched.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/save.rs"
```

### Step 20 · `viewer-core/src/command/verbs/import.rs`

`viewer-core/src/command/verbs/import.rs` implements the `import` command verb: executes its typed command specification Validates arguments against `&mut dyn CommandHost` before mutating scene or view state so invalid interactive or CLI input leaves undo history untouched.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/import.rs"
```

### Step 21 · `viewer-core/src/command/verbs/export.rs`

`viewer-core/src/command/verbs/export.rs` implements the `export` command verb: executes its typed command specification Validates arguments against `&mut dyn CommandHost` before mutating scene or view state so invalid interactive or CLI input leaves undo history untouched.

```rust
--8<-- "session_viewer/viewer-core/src/command/verbs/export.rs"
```

### Step 22 · `assets/view_local.yaml`

`assets/view_local.yaml` lists default local scene URLs, display names, `xform`/`at` instance placements and `point_size` overrides resolved relative to the active manifest route.

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
