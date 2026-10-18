# 4.2.11 · Read previous reports from real browser storage

**Chapter 4 · Browser lifetime and recovery · Section 4.2 · GPU diagnostics and recovery** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

Read previous reports from browser storage. `Store` keeps a storage handle, this tab’s stable ID and the new run key. Session storage preserves the tab ID across reloads; unavailable storage uses a fresh ID without stopping startup.

## Type

Continue from [Prove saved-run exclusions before adopting storage](34ec-proof.md). [Save or recover your work](recovery.md).

### 1. `src/report_storage.rs`

Read bounded validated candidates from actual browser storage and rank with the real Date parser; retain only metadata and storage handles.

Create the file and type:

```rust
--8<-- "journey/code/34f-storage-01.rs"
```

### 2. `src/lib.rs`

Build the storage owner only for the browser target and expose its debug acceptance probe.

<details>
<summary>Locate the existing block</summary>

```rust
mod browser_report;
```

</details>

After that block, type:

```rust
#[cfg(target_arch = "wasm32")]
pub mod report_storage;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34f-storage-02.rs"
```

</details>

### 3. `Cargo.toml`

Enable the pinned browser Storage bindings without changing package versions.

<details>
<summary>Locate the existing block</summary>

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "Location", "Navigator", "Storage", "Performance", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34f-storage-03.rs"
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

In the debug console, run `JSON.parse(window.wasmBindings.previous_storage_probe()).tab`. Reload and run it again: the tab ID stays the same.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read previous reports from real browser storage.](../screenshots/journey/34f-storage.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

Scan at most 256 storage keys and 32 keys in the course namespace. Pass each candidate through bounded decoding and the recency policy. Browser `Date.parse` supplies actual timestamp interpretation.

Rank failures by first fatal time and unfinished runs by heartbeat. Reading removes nothing. This lesson introduces the reader; bounded writes and the previous-report command follow.

sessionStorage tab → localStorage candidates → byte/schema/shape admission → Date.parse recency → latest eligible evidence.

![Read validated storage evidence](../illustrations/journey-34f.svg)

Why keep tab identity separate from the new run’s storage key?

The tab identity survives a reload so interruption policy can recognize its old run. A fresh storage key identifies each run independently; reusing the tab as the run key would overwrite the previous evidence.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change selection to rank lastSeen, then compare the two saved failures whose heartbeat order disagrees with failure order. Explain which evidence should win and why. Deny browser storage and explain why current diagnostics must remain usable.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 4.2.11
npm --prefix ../session_tests run course -- save 4.2.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The production reader uses browser storage and stable tab identity. This endpoint connects the native admission/recency policy to actual storage reads. Bounded writes, live report adoption and the previous-report command follow.

Additional verification: Run the storage browser acceptance in the expandable notes below. The reader must choose eligible evidence from real browser storage and retain the tab ID across reloads. Live persistence follows next.

Headed Chrome uses real Web Storage and UTC timestamp parsing to test candidate selection, stable tab identity, independent run keys and denied storage. Current viewer reports are still not persisted at this checkpoint.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 4.2.11
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
