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
