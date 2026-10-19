# 4.2.33 · Measure accepted source fetches

**Chapter 4 · Browser lifetime and recovery · Section 4.2 · GPU diagnostics and recovery** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Time each actual source fetch, including its bounded body read. Keep the samples in the task until the reload ticket is accepted.

## Type

Continue from [Measure upload preparation and GPU completion](34gdf-upload.md). [Save or recover your work](recovery.md).

### 1. `Cargo.toml`

Enable the browser’s native network timing entry.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "Navigator", "Storage", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34gdg-network-01.rs"
```

</details>

### 2. `src/browser_network.rs`

Collect actual body and browser-network measurements without retaining source owners.

Create the file and type:

```rust
--8<-- "journey/code/34gdg-network-02.rs"
```

### 3. `src/lib.rs`

Register network measurement and safe URL handling.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
mod source_fetch;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
mod browser_network;
#[cfg(target_arch = "wasm32")]
mod source_fetch;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34gdg-network-03.rs"
```

</details>

### 4. `src/browser_reload.rs`

Retain measured fetch phases only after the existing ticket accepts the task.

<details>
<summary>Locate the existing block</summary>

```rust
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls { values.push(crate::source_fetch::fetch(&url, &signal).await?); }
            Ok(values)
        }.await;
        let Some((keys, intent)) = shared.borrow_mut().finish(request.ticket, result.is_err()) else { return; };
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut phases = Vec::new();
        let result: Result<Vec<Vec<u8>>, String> = async {
            let mut values = Vec::new();
            for url in request.urls {
                let started = crate::browser_phase::now();
                let value = crate::source_fetch::fetch(&url, &signal).await;
                phases.extend(crate::browser_network::measured(&url, started, value.is_ok(),
                    value.as_ref().map_or(0, |bytes| bytes.len() as u64)));
                values.push(value?);
            }
            Ok(values)
        }.await;
        let Some((keys, intent)) = shared.borrow_mut().finish(request.ticket, result.is_err()) else { return; };
        for phase in phases { let _ = crate::browser_report::phase(phase); }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34gdg-network-04.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Open a file, type `Unload Sources`, then `Reload Sources` and `Report`. The accepted fetch adds its duration and actual body bytes.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for measure accepted source fetches.](../screenshots/journey/34gba-lifecycle.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [4.2.18 · Own diagnostic page transitions](34gba-lifecycle.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Use the browser’s Resource Timing entry when available. Strip credentials, query and fragment from URL identities. Fetch duration includes body consumption; Resource Timing has its own network boundaries and may report zero transfer bytes.

source URL → bounded body read → completed samples → accepted ticket → safe report identity.

![Only accepted network work contributes retained measurements.](../illustrations/journey-34gdg.svg)

Why collect measurements locally until the reload ticket is accepted?

An aborted or superseded fetch can still settle. Its measurements must not enter the current report.

Study estimate, including typing and experiments: 0.75–1.25 hours.

</details>

<details>
<summary>Optional experiment</summary>

Cancel a pending Reload Sources. Its late response must add no fetch phase.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 4.2.33
npm --prefix ../session_tests run course -- save 4.2.33
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The same timing boundary supports HTTP resources; browser Resource Timing may omit private cross-origin sizes. Zero transfer bytes means unavailable or cached, not an empty document.

Native/WASM/GPU checks preserve existing behavior. Chrome checks actual HTTP timing, exact body bytes, safe URLs, rejection and cancellation.

Chrome fetches a real HTTP specimen, compares the browser’s Resource Timing entry and discards a cancelled response.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 4.2.33
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
