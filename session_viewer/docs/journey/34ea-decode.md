# 34ea · Admit only supported bounded saved JSON

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 27–53 minutes.** 47 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Reject oversized, malformed or unsupported saved JSON before it can become a diagnostic candidate.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** raw text byte limit → JSON fields/version → typed shape validator → candidate for recency.

**Before you finish, explain:** Why inspect raw JSON fields before deserializing the report?

The decoder is the complete format-admission boundary. It rejects text above one MiB before parsing, inspects top-level JSON keys and rejects unsupported fields, then converts to Report and applies the shape validator. No partial report reaches a storage consumer.

The byte limit bounds work before allocation-heavy parsing. The schema check prevents silent loss of unknown telemetry. The typed validator enforces limits and failure/outcome consistency after conversion. Timestamp interpretation and recency remain the next separate policy.

Native tests use actual JSON at the exact byte limit and one byte beyond it, reject malformed/versioned/inconsistent/unsupported values, and bound arrays and multibyte text. A valid first failure round-trips. Chrome retains the existing report-download and loss acceptance; localStorage adoption is still not connected.

![Bound raw stored JSON](../illustrations/journey-34ea.svg)

## Type the change

Continue [Check the bounded shape of a diagnostic report](34e-schema.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34ea-decode`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/report_store.rs`

Bound raw input and reject unsupported JSON fields before adopting a valid typed report.

Create the file and type:

```rust
--8<-- "journey/code/34ea-decode-01.rs"
```

### 2. `src/report_store_tests.rs`

Prove actual stored JSON admission and byte/text/schema boundaries.

Create the file and type:

```rust
--8<-- "journey/code/34ea-decode-02.rs"
```

### 3. `src/lib.rs`

Expose the decoder and its native acceptance checks.

Find this exact block:

```rust
pub mod diagnostic;
```

Replace that block with:

```rust
--8<-- "journey/code/34ea-decode-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Import sample.pb, move its selected beam, then type Diagnostic Report to download viewer-diagnostic.json. Save still downloads viewer.session. Run the native tests for this lesson’s report policy. Finish the healthy proof view with Move 0,-0.05,0 and Fit. Chrome also verifies an actual GPU-loss report and startup-failure download, followed by a healthy reload. Saved previous reports and unsaved-edit recovery are not connected yet.

**Actual Chrome screenshot.**

Native tests validate actual saved JSON boundaries and unsupported schema. Chrome checks inherited actual diagnostics and GPU-loss acceptance; browser storage adoption comes later.

![Actual browser result: Admit only supported bounded saved JSON.](../screenshots/journey/34ea-decode-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Remove the raw-field whitelist, add an unknown telemetry field and explain why a later typed export could lose it. Compare the exact one-MiB whitespace-padded JSON with the one-byte-larger value.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The typed deserializer can ignore unknown fields. Rejecting unsupported fields first prevents a later export from silently stripping newer telemetry.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34ea-decode
npm --prefix ../session_tests run course -- save 34ea-decode
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Saved reports need byte/schema/shape admission before recency and UI use. This endpoint adds that decoder; selection, real storage, telemetry and bounded recovery follow.

[Validation status and course release](release.md).
