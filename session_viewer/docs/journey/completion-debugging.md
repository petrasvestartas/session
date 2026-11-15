# Explain a failed frame

Use a diagnostic report, a controlled scene and a rendered image to distinguish loading, scene, shader and device failures.

## Prerequisites and ownership

Complete rendering, the application lifecycle, loading and input. This application chapter teaches the diagnostic adapters in `engine/diagnostics.rs`, the native harness in `selftest.rs` and its `profile`, `oracle`, `lifecycle` and `stroke_witness` modules. Frame timing remains owned by the renderer's `engine/performance.rs`; the browser diagnostic page receives those reports. The harnesses inspect the production pipeline rather than substituting a tutorial renderer.

## Build

Connect diagnostic reporting to the completed runtime and use its existing harnesses to isolate a controlled failure.

### Retain enough context to explain an error

`diagnostics::diagnostic` calls the browser's `viewerDiagnostic` hook when available. The native path has no browser global and does nothing. The optional hook must not become a new requirement for rendering. `phase` records a named load duration, byte count and source path with the query removed, including successful phases.

A useful failure report binds the source revision, fixture, route, viewport, device ratio, adapter, event sequence and output. A console message without those inputs cannot establish that another machine ran the same case. Keep the original error and image before changing a test or source. A timeout, blank canvas and shader validation error point to different boundaries.

Start with the simplest distinction: did loading reach readiness, did the scene contain geometry, and did the GPU produce pixels? Inspect the report and visible result together. Empty overlap diagnostics do not establish that every glyph is inside the viewport, and a nonempty screenshot does not establish that it contains the intended object.

### Separate CPU construction from GPU rendering

`selftest::profile` times the geometry walk by object kind before GPU drawing. Its BRep measurements separate meshing, edge chains and orientation work. A slow walk suggests different work from a slow draw; reducing shader complexity cannot remove time spent constructing triangles on the CPU.

`selftest::render_scene` loads actual documents, creates a headless `Gpu`, frames the scene, settles display refinement and renders offscreen. Its environment controls select a camera, viewport, clipping, labels or repeated orbit frames. Record those controls with the result. Refinement is bounded by row count and fails if it cannot settle; capturing an unfinished frame would conflate refinement with a rendering regression.

```rust
--8<-- "session_viewer/src/selftest.rs:selftest-pixels"
```

The exported image contains RGB bytes in PPM format; alpha is intentionally omitted. Readback and encoding are separate from judgment. A file written successfully still needs inspection or an image assertion appropriate to the feature.

### Use an independent question as an oracle

`selftest::oracle` asks a CPU ray-visibility question for samples along strokes against placed triangles. It compares clearly visible samples with nearby rendered ink. Its clearance avoids deciding ambiguous coplanar cases using the GPU's own tolerance. This checks hidden-line visibility without merely repeating the shader's output as the expected answer.

`selftest::lifecycle` compares color and object/subobject ID images through resize, rewalk, edits/undo and reload. Each nonzero ID must still resolve to scene identity. Matching color alone could hide a broken selection map; matching IDs alone could hide a rendering defect.

`selftest::stroke_witness` exposes controlled rendering inputs and returns pixels, identity, shader sources and device/queue ownership. The sheet witness explicitly requires a verified software adapter. The renderer's cloud witness serves the corresponding cloud checks. A software witness is reproducible pipeline evidence; it does not demonstrate a hardware driver's behavior or a phone's speed.

### Follow the failing shader value

For a device-specific failure, keep the same object and reduce the shader question. A temporary probe can encode one intermediate value as a simple color or square. Compare values before and inside a function boundary before changing the geometry algorithm. Remove the diagnostic path after establishing the cause.

The maintained arrow interface passes fields individually because the tested Android driver corrupted `end` and `heads` when a packed row crossed the function boundary. Preserve that ABI in the owning [rendering subject](completion-geometry.md#the-arrow-function-boundary). The [probe procedure](../debugging.md#probe-a-phone-only-shader-failure) records how to investigate such failures; there is no supported permanent probe query switch.

### Measure interaction and resting quality separately

The performance controller distinguishes wheel bursts and drag frames from resting frames. It uses a recent median and hysteresis before changing interaction tiers, then restores resting quality. Timing an isolated CPU submission is not the same as waiting for GPU completion. Record the actual adapter, whether the run used software rendering, and whether timing includes startup, compilation or refinement.

The browser host uses completion pacing to avoid accumulating obsolete frames. A continuously moving camera and a quiet settled frame therefore answer different performance questions. Consult [phone performance](../phone-performance.md) for the current resource and driver boundaries; desktop emulation cannot establish phone memory pressure or touch-keyboard behavior.

### Compare incremental edits with a fresh scene

The scene-sync fixture combines nested placed groups, an object parent with a placed child, curve and mesh geometry, and an element with baked features. Two placements share one kernel session, alongside another document and read-only streamed shells. This exposes identity, inherited-transform and ownership mistakes that a single flat object cannot. The common check synchronizes changes, settles the CPU upload model, then compares the result with a fresh walk. A seeded integer generator makes edit sequences reproducible.

The CPU `settle` helper models upload bookkeeping without creating a GPU: it retires and buries rows, patches edge ownership, drains new tables, and applies geometry, unbury and placement updates. `verify` rebuilds the same documents and compares editable identities, lane counts, exact placement and bounds bits, hulls, spacing, faces and flags. It masks only the sheet-classification bit when the original load classification differs from the fresh walk. Pipe owners must resolve or belong to a batch or tomb; retained and dead counts must fit uploaded storage. This catches synchronization errors, but cannot prove GPU uploads or pixels.

The hierarchy regression creates a point, deletes it and undoes the deletion, refreshing the panel after each synchronized edit. It asserts that the same row disappears and returns. A separate comparison checks that a group action targets exactly the tree descendants mapped to scene rows. Thus a correctly rendered hierarchy is insufficient if its action targets are stale.

The editing regressions make retained storage observable: deleting a 100-point cloud buries its row and charges its point bytes to the tomb budget; undo stages an unbury rather than uploading points again. Setting the budget one byte below that allocation forces release. A compaction case then proves that undo can recreate the same identity after retained rows are gone. Multi-document deletion must form one history step, so one Undo restores all affected identities.

The command regressions check allocation behavior as well as final geometry. A 200-frame growing-curve preview may request new rows at most 16 times, bounds dead storage by the allocation sizes, and must return to its original span on cancellation. The edit-cost test times 40 create/delete/undo cycles in scenes of 1,000 and 100,000 objects and permits the larger case less than four times the smaller duration plus 50 milliseconds. This is a regression threshold for incremental work, not a device-independent latency promise.

The randomized synchronization tests apply 150 edits for each fixed seed and compare with the fresh-scene oracle after every step, reporting the seed and step on failure. A second set uses a 4,096-byte tomb cap to exercise releases and reconstruction. Cache-disabled and cache-enabled sequences must produce the same document rows and lane spans while the cache reduces searches. Other sequences preserve hidden and color overrides through compaction and confirm that editing ordinary objects leaves streamed shells unchanged. These CPU assertions remain distinct from the explicitly ignored GPU tests in the same module.

### Report feedback without replacing another message

The feedback adapter mirrors status to the browser status element and command model, while native execution logs it. An empty status may restore a recovered-run notice; download progress replaces only an empty message or its own previous progress, preserving unrelated feedback. Fatal errors emit a diagnostic and reveal the error panel. Panel updates copy presentation models, command history records canonical text, and keyboard requests delegate to the browser input owner. These adapters expose state; a successful call alone does not prove that a user saw or acted on the message.

### Distinguish measured payload from total memory

With `inspect=1`, the inspection adapter publishes JSON on the canvas as `data-viewer-inspection`. It connects visible state to object identities, camera matrices, selection, tool state, lane counts, retained allocations and timing counters. Buffer capacity, estimated texture bytes and known source payload are reported separately; private egui and glyphon resources are explicitly outside the totals. Tests must pair these snapshots with the action and image: an allocated ribbon count can remain unchanged after deletion because retained storage is not the live-object count.

The source-memory cache retains weak session references and recounts when document session pointers change. It counts each shared session and geometry value once, adding vector and string capacities, exposed slice lengths, occupied map entries and known value sizes. Dead collection slots remain counted; history bytes are reported separately. Allocator overhead, spare map capacity and private nested caches are not available to this walker, so `known_bytes` is a scoped payload estimate rather than process RSS or total heap use.

## Run and observe

Run the cumulative application with the comparison fixture and `inspect=1`. In browser developer tools, evaluate `window.viewerDiagnostics.read()` after readiness, then orbit and fit the scene. Save the report beside a screenshot and note the actual adapter. The visible geometry should respond while the report retains startup/load context without a failure.

Use the loading chapter's blocked-font experiment next. The result should be a contextual error, not a successful readiness report. Restore the request before continuing. Keep both the failing and successful observations with their different inputs.

## Predict and try

Predict whether an identical screenshot proves that picking still works after a scene reload. Reload, then select a known object. Color can remain identical even if stale IDs no longer map to the right scene row. The lifecycle harness therefore compares identity images and resolution as well as color; the browser exercise checks the user-visible consequence.

## Question and answer

Why is a software-rendered witness insufficient to close an Android shader failure?

It executes the same pipeline through a different compiler and driver. It can constrain the failing inputs and detect ordinary logic errors, but the affected hardware must exercise the compatibility boundary before its acceptance is established.

## Validation

The current cumulative application compiles and links the native test modules and has browser reports for scene, command and stage checks. This chapter does not claim a fresh full native lifecycle/oracle run or physical-device acceptance. Those results must be recorded independently with their inputs and adapter, rather than inferred from successful browser screenshots.
