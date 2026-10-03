# 34f · Read previous reports from real browser storage

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 21–41 minutes.** 52 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Read admitted saved reports using stable tab identity, the real browser timestamp parser and bounded scanning.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** sessionStorage tab → localStorage candidates → byte/schema/shape admission → Date.parse recency → latest eligible evidence.

**Before you finish, explain:** Why keep tab identity separate from the new run’s storage key?

Store owns a browser storage handle, a stable tab identifier and a fresh run key. It owns no source document, renderer, listener or timer. Session storage retains tab identity across reloads; unavailable storage falls back to a fresh identifier without rejecting viewer startup.

Reading examines at most 256 storage keys and 32 keys in our course namespace. Each value passes the one-MiB decoder and typed shape validator before selection. These limits bound examination when edited storage contains many entries. Values outside that examination window are not adopted. The later writer keeps the normal owned set to three reports. No key is removed by this reader.

The real browser Date parser now supplies the clock port taught in the preceding lessons. Eligible failures rank by their first fatal time; interrupted running reports rank by heartbeat. A newer heartbeat cannot make an older fatal event outrank a newer failure. Malformed, unsupported, healthy, active-other-tab and invalid-time values do not become previous evidence.

A debug-only WebAssembly probe exercises this owner against real sessionStorage and localStorage in headed Chrome. It proves stable tab identity, distinct run keys, correct ranking and storage denial without changing current report, document drawing or GPU geometry counters. These seeded values belong only to the test page’s course namespace. Live persistence and a visible previous-report command are connected after bounded writes are complete.

![Read validated storage evidence](../illustrations/journey-34f.svg)

## Type the change

Continue [Prove saved-run exclusions before adopting storage](34ec-proof.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34f-storage`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/report_storage.rs`

Read bounded validated candidates from actual browser storage and rank with the real Date parser; retain only metadata and storage handles.

Create the file and type:

```rust
--8<-- "journey/code/34f-storage-01.rs"
```

### 2. `src/lib.rs`

Build the storage owner only for the browser target and expose its debug acceptance probe.

Find this exact block:

```rust
mod browser_report;
```

Replace that block with:

```rust
--8<-- "journey/code/34f-storage-02.rs"
```

### 3. `Cargo.toml`

Enable the pinned browser Storage bindings without changing package versions.

Find this exact block:

```toml
"Location", "Navigator",
```

Replace that block with:

```toml
--8<-- "journey/code/34f-storage-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run this lesson’s Rust checks and viewer. Current Diagnostic Report and Save still work as before. Chrome’s debug probe reads real seeded localStorage, ranks actual UTC timestamps and tests denied storage. The viewer’s live report is not persisted yet; bounded writes and command retrieval follow. Finish the healthy proof view with Move 0.05,0,0 and Fit.

**Actual Chrome screenshot.**

Headed Chrome uses real Web Storage and UTC timestamp parsing to test candidate selection, stable tab identity, independent run keys and denied storage. Current viewer reports are still not persisted at this checkpoint.

![Actual browser result: Read previous reports from real browser storage.](../screenshots/journey/34f-storage-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Change selection to rank lastSeen, then compare the two saved failures whose heartbeat order disagrees with failure order. Explain which evidence should win and why. Deny browser storage and explain why current diagnostics must remain usable.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The tab identity survives a reload so interruption policy can recognize its old run. A fresh storage key identifies each run independently; reusing the tab as the run key would overwrite the previous evidence.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34f-storage
npm --prefix ../session_tests run course -- save 34f-storage
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The production reader uses browser storage and stable tab identity. This endpoint connects the native admission/recency policy to actual storage reads. Bounded writes, live report adoption and the previous-report command follow.

[Validation status and course release](release.md).
