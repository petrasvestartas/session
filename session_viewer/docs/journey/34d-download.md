# 34d · Download diagnostics through the real command line

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 13–26 minutes.** 26 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Download a current ready-run report with Diagnostic Report and attempt one independent first-failure download after GPU disposal.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** first scene presented → ready report → typed Diagnostic Report → JSON Blob → actual download → delayed URL cleanup.

**Before you finish, explain:** Why can the failed viewer download a report after GPU input and drawing have stopped?

The metadata producer becomes a real viewer command. The first scene presentation records the geometry-on-screen milestone and changes an otherwise healthy report to ready. Diagnostic Report serializes the current context and invokes the existing download owner with a JSON filename. Save still writes viewer.session and retains its exact original-source contract. Neither command consumes scene Undo.

The serializer releases its report-slot borrow before anchor.click can invoke browser behavior. The hidden download anchor is temporary, and the existing10-second URL cleanup owns only the Blob URL. No feature button or permanent GPU reference is introduced. This command does not claim that diagnostic counters are total browser memory or actual-phone timing.

A matching GPU failure first disposes the runtime, then records the fatal reason and attempts one automatic report download. Startup failure is recorded too when report initialization succeeded. Some browsers can block automatic downloads; this checkpoint preserves the report in memory only. Persisted recovery and a later typed previous-report download are the next responsibility. Failure diagnostics do not restore unsaved edits.

Chrome types Diagnostic Report and reads the actual JSON download. It verifies ready context and milestone timing, unchanged placement/camera/history/GPU counters, then confirms Move Undo still works. It destroys the real GPUDevice, captures the actual failed download, checks the first reason, unchanged message after repeated input and no further GPU work. A startup adapter rejection produces a failed report without a runtime. Normal same-tab reload restarts the working viewer.

![Command and fatal report downloads](../illustrations/journey-34d.svg)

## Type the change

Continue [Read live diagnostic context outside the GPU runtime](34c-browser.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34d-download`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/file_output.rs`

Reuse the existing owned Blob/anchor cleanup for a named report while keeping Save’s filename unchanged.

Find this exact block:

```rust
pub fn download(bytes: &[u8]) -> Result<(), JsValue> {
    let window
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-01.rs"
```

### 2. `src/file_output.rs`

Choose the diagnostic JSON filename without changing the document download contract.

Find this exact block:

```rust
    anchor.set_download("viewer.session");
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-02.rs"
```

### 3. `src/browser_report.rs`

Download a current metadata snapshot and attempt one automatic download for the first fatal observation.

Find this exact block:

```rust
#[cfg_attr(debug_assertions, wasm_bindgen::prelude::wasm_bindgen)]
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-03.rs"
```

### 4. `src/browser.rs`

Keep the matching device’s failure evidence after its GPU runtime is disposed.

Find this exact block:

```rust
        if crate::browser_runtime::stop_if(&fault) { report(&format!("Cannot draw: {message}")); }
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-04.rs"
```

### 5. `src/browser.rs`

Expose diagnostics through the actual command dock without an HTML feature button.

Find this exact block:

```rust
            "Save",
            "Example Box",
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-05.rs"
```

### 6. `src/browser.rs`

Mark the report ready only after the initial scene frame is presented.

Find this exact block:

```rust
        )?;
    }
    let input_canvas = canvas.clone();
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-06.rs"
```

### 7. `src/browser.rs`

Handle the typed report command separately from document Save and edit history.

Find this exact block:

```rust
            } else if line == "save" {
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-07.rs"
```

### 8. `src/lib.rs`

Capture startup failure after diagnostic initialization, even when no GPU runtime could be installed.

Find this exact block:

```rust
            browser::report(&format!("Cannot draw: {error:?}"));
```

Replace that block with:

```rust
--8<-- "journey/code/34d-download-08.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Import and move sample.pb. Type Diagnostic Report to download viewer-diagnostic.json, containing a ready outcome and actual browser context. Save still downloads viewer.session. On a recorded GPU or startup failure the viewer attempts one JSON report download, then shows failure feedback. Reload starts a fresh run; no saved previous report or unsaved-edit recovery is connected yet. Finish the proof view with Move 0,-0.05,0 and Fit.

**Actual Chrome screenshot.**

Chrome reads actual typed ready-report and real GPU-loss/startup-failure JSON downloads, checks scene/history/camera/GPU invariants and late-input blocking, then reloads the same test page for its final working picture.

![Actual browser result: Download diagnostics through the real command line.](../screenshots/journey/34d-download-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Keep the report inside the GPU Runtime and explain why the fatal download would lose it. Then compare the actual document Save and diagnostic downloads and explain why they cannot be used interchangeably.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The independent report owns metadata only. JSON serialization and the existing Blob/anchor download need no GPU resources, so disposal can stop the renderer without destroying its evidence.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34d-download
npm --prefix ../session_tests run course -- save 34d-download
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production exposes Diagnostic Report through the command dock and attempts an automatic first-failure report download. This endpoint establishes current/fatal downloads. Persisted reports, detailed adapter/load/resource telemetry and bounded automatic recovery remain future work.

[Validation status and course release](release.md).
