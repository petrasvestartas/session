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

### Choose an assertion that detects the rendering defect

The element-walk tests compare the same source with features disabled and enabled. Visible features must add ribbons and dots to the element's own row, extend its bounds, and exclude hidden features. A contact polygon must add its filled triangles and black boundary using that same identity. These are CPU upload assertions: they establish which primitives and IDs reach rendering, not their final pixel visibility.

The rational-border regression walks a cylindrical NURBS surface and checks that every border sample belongs to its shaded face vertices, every pipe endpoint names the corresponding vertex, and source edge IDs remain intact. It also compares the source protobuf before and after walking. Moving all controls by half a unit must move preview pipe endpoints by exactly that z offset while preserving x and y. This connects tessellation, picking identity and editable UV samples without accepting a merely similar outline.

The plane-frame image test adds an opaque lid: gray grid pixels must largely disappear while colored axis-arrow pixels remain. Halving plane size must approximately halve its visible column span. A separate authored-width test measures the center-column ink for widths 1, 3 and 15 with a two-pixel tolerance. The plane test is explicitly ignored by default; the width test returns early if no adapter exists, so a successful test process alone cannot prove those pixels were checked.

The clipping uniform mirror test parses and validates WGSL with Naga, then compares the shader member offsets and total struct span with Rust `offset_of` and `size_of`. It checks shared clipping flag values too. A shader can parse successfully while reading the wrong bytes from a Rust buffer; this test makes the host/shader layout agreement explicit rather than inferring it from compilation.

The clipping image tests build controlled solids and inspect projected pixel neighborhoods. Cutting a closed cube must show hatch in the section, keep the retained top face plain, and restore the complete top when clipping is removed. Other fixtures distinguish open meshes, overlapping solids and hidden internal geometry. These tests are ignored by default and their helper can return without an adapter; acceptance requires evidence that the intended GPU cases actually executed, together with their selected fill and view settings.

The ambient-occlusion regression renders contact geometry with the effect off and on at one and four samples in perspective and orthographic views. It requires contact darkening, rejects a halo on distant exposed ground and compares the first shaded frame with the settled frame. Resource checks account for the effect's owned allocations. The test is ignored until a native adapter is explicitly supplied, so normal unit-test success does not establish its image result.

The outline regression compares selected and unselected CAD strokes over curved silhouettes at multiple views, sample counts and device ratios. Pixels that were nearly black must remain black when yellow selected edges are drawn, and total silhouette coverage must remain nontrivial. This catches edges painting over the border rather than only checking that both passes submitted. These native GPU tests require explicit execution beyond the default ignored set.

The grid witness requires the Microsoft CPU DX12 adapter and compares actual images at the origin, a trillion-unit offset and the origin again. Anchored camera matrices must match, while fractional grid offsets remain represented. The near/far and return images must have zero changed pixels. This is a precise rebasing invariant under the recorded WARP configuration, not a claim about every hardware driver.

Font coverage tests enumerate command names, aliases, options and hints against the bundled subsets, then check representative accented text and UI symbols. A specimen intentionally outside the subset is shaped after replacing the fonts with the complete font files, and every resulting glyph ID must be nonzero. This proves local character coverage and replacement behavior; it does not exercise asynchronous browser font delivery or compare glyph edge quality.

### Identify the adapter behind a cloud image

The cloud rendering witness requires positive viewport dimensions, density one or two, one or four samples and a finite nonnegative EDL setting. It verifies the CPU DX12 Microsoft Basic Render Driver before rendering an isolated cloud. The source is decoded and uploaded through the scene path; the fixture then checks its expected matrix, cloud and splat counts. Its result retains adapter identity, actual parameters, uploaded CPU bytes and shader source. Such a result is evidence for that software adapter and fixture, not physical GPU or phone acceptance.

### Inspect LOD selection separately from rendering

The cloud LOD witness prepares production cloud rows, cloud metadata and bounds on the CPU, then invokes the actual `LodWalk::select` with a fixed eye and viewport plus explicit projection, placement and pixel threshold. It rejects a resident prefix larger than the source and reports each selected range's first point, count, spacing and tile. These ranges explain which resident points the selector chose. Its metadata explicitly excludes GPU acceptance: correct ranges cannot establish shader output, depth, EDL or the appearance of the rendered points.

### Distinguish geometry readiness from font completion

The browser diagnostics script keeps a bounded report with startup time, tab identity, viewport, adapter, phase timings and recent events. It retains the first fatal error, writes guarded local-storage snapshots and provides a JSON download even if storage is unavailable. A recent unfinished heartbeat suggests interruption; it does not prove a browser crash. The report changes to ready on the explicit geometry-on-screen milestone. A rendered text-only scene therefore cannot use that milestone as a general font-completion oracle. Compare its label metrics and actual font responses instead. Closing or restoring a page updates lifecycle state without erasing the original failure.

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
