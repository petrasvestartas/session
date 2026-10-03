# 34c · Read live diagnostic context outside the GPU runtime

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 24–48 minutes.** 51 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Start a bounded page report and read real browser context independently of the renderer’s lifetime.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** page begins → sanitized context → independent report slot → current context → JSON snapshot.

**Before you finish, explain:** Why does the report live outside the Runtime that GPU failure disposes?

The report now has a browser producer. One thread-local slot owns one small Report for this page. It contains metadata and bounded observations, never the editor, source document or GPU device. This is an intentional lifetime beyond viewer disposal. No timer or extra browser listener is added here.

context reads the actual page origin/path, browser user agent, secure-context and WebGPU availability, CSS viewport, drawing buffer and display density. The query string and fragment are excluded from the page value. Typed web-sys access requires Location, Navigator and Performance in the existing manifest; Reflect checks the WebGPU property without requiring unstable generated WebGPU bindings.

start assigns a fresh run identifier and UTC timestamp before the GPU request. Stable tab identity across reloads is still part of the later storage policy. observe uses performance.now for relative timing and Date for readable wall time, then passes values through our existing bounded recorder. diagnostic_snapshot refreshes context and produces actual JSON. Its debug-only WebAssembly export lets Chrome inspect that same production function; it adds no visible feature control.

Chrome compares real browser/canvas dimensions, density, identity, sanitized URL, timestamps and run identifiers with the JSON. Reading a snapshot leaves GPU counters unchanged. It then changes the test viewport and verifies fresh context, reloads the same test tab to require a new run ID and reproduces the final proof scene. Readiness/failure observations and the typed download command are connected in the next checkpoint; this report still says running.

![Report lifetime beyond GPU runtime](../illustrations/journey-34c.svg)

## Type the change

Continue [Keep recent events without losing the first failure](34ba-events.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34c-browser`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/browser_report.rs`

Own one bounded report for the page and read actual browser metadata without keeping any viewer or GPU owner.

Create the file and type:

```rust
--8<-- "journey/code/34c-browser-01.rs"
```

### 2. `Cargo.toml`

Enable typed browser address, identity and monotonic-clock access.

Find this exact block:

```toml
"PageTransitionEvent", "Event",
```

Replace that block with:

```toml
--8<-- "journey/code/34c-browser-02.rs"
```

### 3. `src/lib.rs`

Compile the browser report bridge only for WebAssembly.

Find this exact block:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_runtime;
```

Replace that block with:

```rust
--8<-- "journey/code/34c-browser-03.rs"
```

### 4. `src/browser.rs`

Start diagnostic context before awaiting the GPU connection; failure observation and downloads are connected next.

Find this exact block:

```rust
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
```

Replace that block with:

```rust
--8<-- "journey/code/34c-browser-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Import sample.pb, Select Next, and use Move or Orbit Up. Device loss is normally external; the browser acceptance calls the real GPUDevice.destroy() through a test-only observer. Failure shows Cannot draw, and old input cannot submit another frame. Reload the SAME page to start a new viewer; unsaved placement is not recovered. In a working run, finish with Orbit Up and Fit. Run the native tests to inspect the report’s JSON header; the Diagnostic Report command is introduced after browser wiring. Finish the healthy proof view with Orbit Right and Fit. The native tests exercise bounded diagnostics; live download and storage are still upcoming. In a debug build, window.wasmBindings.diagnostic_snapshot() returns the current header and empty event window. It remains a metadata read, not a command or GPU submission. Finish with Move 0.05,0,0 and Fit.

**Actual Chrome screenshot.**

Chrome verifies real diagnostic context, sanitized URL, fresh dimensions, snapshot reads without GPU writes and a new run identifier on same-tab reload. Readiness/failure recording and downloads follow next.

![Actual browser result: Read live diagnostic context outside the GPU runtime.](../screenshots/journey/34c-browser-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Put the report inside Runtime instead, trigger the actual device-loss check, and explain why it would disappear with the evidence it was meant to retain. Compare a URL containing a query parameter with the report’s page field.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A failed Runtime releases input, source and GPU owners. Its report must remain readable afterward, so a separate bounded metadata slot owns it without retaining those resources.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34c-browser
npm --prefix ../session_tests run course -- save 34c-browser
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production diagnostic context lives outside the renderer. This browser bridge establishes that independent report lifetime and real metadata; current/fatal downloads, adapter/phases/resources, bounded storage and recovery remain next.

[Validation status and course release](release.md).
