# 34fa · Retain only three diagnostic runs

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 18–36 minutes.** 31 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Write admitted report metadata to an independent run key and retain a bounded set without disturbing viewer state.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** valid matching-tab report → bounded existing namespace → current run write → two older reports → release other owned keys.

**Before you finish, explain:** Why write the current run before pruning the older keys?

The same Store now owns writes as well as reads. A report must satisfy typed validation and match the store’s tab identity before any mutation. Examination is capped at 256 total keys and 32 keys in the owned namespace; a larger namespace refuses writing without modifying storage. Each serialized report retains the existing one-MiB limit.

The writer prepares the two newest valid older reports by heartbeat, writes the current run, then removes other examined keys in the course namespace. Repeated writes reuse the current key. Malformed owned values are removed after a successful write. Keys outside the course namespace remain untouched. Retention age uses heartbeat; selecting a failed report still uses fatal time, so those responsibilities remain distinct.

Failed writes preserve the existing evidence. If later deletion fails, the method returns false and makes no claim that retention completed; the current saved value may already exist. Private browsing or denied storage must not stop current diagnostics or drawing. No renderer or source data is serialized.

Headed Chrome reads the actual saved JSON, tests stable-key repeated writes and a fourth run reducing the set to three, retains the two newest old values, removes malformed owned values and preserves unrelated keys. Storage denial and an excessive namespace return false without mutation. These probes exercise the physical storage boundary before live report persistence is connected.

![Retain a bounded set of reports](../illustrations/journey-34fa.svg)

## Type the change

Continue [Read previous reports from real browser storage](34f-storage.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34fa-retain`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/report_storage.rs`

Admit metadata, save to this run’s key and retain at most two older valid reports; refuse unbounded namespace examination and tolerate unavailable storage.

Find this exact block:

```rust
    pub fn previous(&self) -> Option<Report> {
```

Replace that block with:

```rust
--8<-- "journey/code/34fa-retain-01.rs"
```

### 2. `src/report_storage.rs`

Verify repeated writes use the same key and reach actual Web Storage; this debug export is not a runtime feature control.

Find this exact block:

```rust
    serde_json::json!({"tab": store.tab, "key": store.key, "previous": store.previous()}).to_string()
}
```

Replace that block with:

```rust
--8<-- "journey/code/34fa-retain-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run this lesson’s state checks and viewer. Current Diagnostic Report and Save still work as before. Chrome’s debug probe writes actual report JSON, proves repeated writes share one key, retains three runs and refuses denied or oversized storage. Live persistence and the previous-report command are connected next. Finish the healthy proof view with Move 0,-0.05,0 and Fit.

**Actual Chrome screenshot.**

Actual Web Storage contains the saved JSON and only three retained runs after successful pruning. Chrome checks repeated keys, malformed cleanup, unrelated-key preservation and denied/excessive storage; live persistence follows next.

![Actual browser result: Retain only three diagnostic runs.](../screenshots/journey/34fa-retain-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Make setItem throw and compare all existing values before and after. Then make removal throw after a successful write; explain why returning false cannot mean the current value was rolled back.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A failed write must leave existing evidence intact. Only after current metadata is saved can the owner remove older values while keeping two prior reports. Deletion failures are reported rather than claimed as successful retention.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34fa-retain
npm --prefix ../session_tests run course -- save 34fa-retain
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production diagnostics persist bounded metadata independently of the GPU. The course now has a tested writer; startup/current-event integration and typed previous-report download follow.

[Validation status and course release](release.md).
