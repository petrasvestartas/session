# 34fa · Retain only three diagnostic runs

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 18–36 minutes.** 31 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Write admitted report metadata to an independent run key and retain a bounded set without disturbing viewer state.

**Follow:** valid matching-tab report → bounded existing namespace → current run write → two older reports → release other owned keys.

Keep the current run and two older reports. Validate the report and its matching tab ID before writing. Repeated writes reuse the current run key.

Prepare the retention list, write the current value, then remove excess course-owned keys. Leave unrelated storage alone. Retention ranks heartbeat age; failure selection still ranks fatal time.

A failed write preserves earlier values. A later deletion failure may leave the new report already stored, so return false without claiming rollback. Refuse oversized reports or namespaces before mutation. Storage denial must never stop the viewer.

![Retain a bounded set of reports](../illustrations/journey-34fa.svg)

## Type the change

Continue from [Read previous reports from real browser storage](34f-storage.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-34fa-retain` (from `session_viewer`).

### 1. `src/report_storage.rs`

Admit metadata, save to this run’s key and retain at most two older valid reports; refuse unbounded namespace examination and tolerate unavailable storage.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn previous(&self) -> Option<Report> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34fa-retain-01.rs"
```

### 2. `src/report_storage.rs`

Verify repeated writes use the same key and reach actual Web Storage; this debug export is not a runtime feature control.

<details>
<summary>Locate the existing block</summary>

```rust
    serde_json::json!({"tab": store.tab, "key": store.key, "previous": store.previous()}).to_string()
}
```

</details>

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

Run the storage-write browser acceptance below. Writing a fourth run must leave the current run and the two newest older reports, preserving unrelated storage keys.

**Verified checkpoint in Chrome.**

![Actual browser result: Retain only three diagnostic runs.](../screenshots/journey/34fa-retain-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Make setItem throw and compare all existing values before and after. Then make removal throw after a successful write; explain why returning false cannot mean the current value was rolled back.

</details>

## Explain the change

Why write the current run before pruning the older keys?

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

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Production diagnostics persist bounded metadata independently of the GPU. The course now has a tested writer; startup/current-event integration and typed previous-report download follow.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Headed Chrome reads the actual saved JSON, tests stable-key repeated writes and a fourth run reducing the set to three, retains the two newest old values, removes malformed owned values and preserves unrelated keys. Storage denial and an excessive namespace return false without mutation. These probes exercise the physical storage boundary before live report persistence is connected.

Actual Web Storage contains the saved JSON and only three retained runs after successful pruning. Chrome checks repeated keys, malformed cleanup, unrelated-key preservation and denied/excessive storage; live persistence follows next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34fa-retain
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
