# 34c · Read live diagnostic context outside the GPU runtime

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 24–48 minutes.** 51 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Start a bounded page report and read real browser context independently of the renderer’s lifetime.

**Follow:** page begins → sanitized context → independent report slot → current context → JSON snapshot.

Populate the report from the page before requesting a GPU. One thread-local `RefCell<Option<Report>>` owns metadata independently of the drawing runtime. `RefCell` checks short shared/mutable borrows at runtime.

`context` reads the real viewport, canvas, density and browser details. Build the page value from origin and path, excluding query and fragment. `Reflect` reads WebGPU availability without generated WebGPU bindings.

`start` creates a fresh run ID and UTC start time. `observe` combines `performance.now()` timing with a UTC timestamp. `diagnostic_snapshot` refreshes context and serializes the current report. Its debug export lets you inspect the JSON in the console; it does not add a feature button.

Readiness and fatal observations are wired in the next lesson.

![Report lifetime beyond GPU runtime](../illustrations/journey-34c.svg)

## Type the change

Continue from [Keep recent events without losing the first failure](34ba-events.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-34c-browser` (from `session_viewer`).

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

In the debug console, inspect `window.wasmBindings.diagnostic_snapshot()`. Its viewport and canvas dimensions must match this browser. Resize and read again: context should refresh without changing the scene.

**Verified checkpoint in Chrome.**

![Actual browser result: Read live diagnostic context outside the GPU runtime.](../screenshots/journey/34c-browser-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Put the report inside Runtime instead, trigger the actual device-loss check, and explain why it would disappear with the evidence it was meant to retain. Compare a URL containing a query parameter with the report’s page field.

</details>

## Explain the change

Why does the report live outside the Runtime that GPU failure disposes?

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

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Production diagnostic context lives outside the renderer. This browser bridge establishes that independent report lifetime and real metadata; current/fatal downloads, adapter/phases/resources, bounded storage and recovery remain next.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome compares real browser/canvas dimensions, density, identity, sanitized URL, timestamps and run identifiers with the JSON. Reading a snapshot leaves GPU counters unchanged. It then changes the test viewport and verifies fresh context, reloads the same test tab to require a new run ID and reproduces the final proof scene. Readiness/failure observations and the typed download command are connected in the next checkpoint; this report still says running.

Chrome verifies real diagnostic context, sanitized URL, fresh dimensions, snapshot reads without GPU writes and a new run identifier on same-tab reload. Readiness/failure recording and downloads follow next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34c-browser
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
