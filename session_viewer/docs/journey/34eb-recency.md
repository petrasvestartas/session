# 34eb · Choose a recent failure without blaming active tabs

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 25–49 minutes.** 45 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Select recent failed or interrupted runs using actual failure time, valid chronology and tab identity.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** validated report → timestamp parser → ordered finite times → failure/heartbeat freshness → ranking timestamp.

**Before you finish, explain:** Why does a failed report’s fresh heartbeat not prove the failure itself was recent?

The production notice had a concrete counterexample: an old failed tab kept refreshing its heartbeat and looked like a recent failure. A real Chrome regression failed before the corrected selector and passed afterward. The course now teaches the same distinction in a pure policy.

A supplied timestamp parser separates browser date parsing from deterministic native policy tests. The policy validates finite current/start/heartbeat times and their ordering. A failed run is eligible only when its first fatal time lies between start and heartbeat and is less than two hours old. Its ranking timestamp is the failure time, not the heartbeat.

A running report is eligible when its heartbeat is under two hours old and it belongs to this tab, or another tab whose heartbeat is more than two minutes old. A current other tab is not interruption evidence. Ready and closed reports do not qualify. An interruption is not proof of a crash.

Native tests use a deterministic clock port for heartbeat/failure boundaries, future and ordering cases. The next checkpoint proves invalid clocks and healthy-run exclusions. Browser Date.parse and saved-value selection follow that proof; there is no previous-report notice or storage adoption here yet. Chrome retains all existing real diagnostics/loss/download checks.

![Failure time and heartbeat have different meanings](../illustrations/journey-34eb.svg)

## Type the change

Continue [Admit only supported bounded saved JSON](34ea-decode.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34eb-recency`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/report_recency.rs`

Interpret a validated report’s chronology and return an eligible timestamp for ranking, using an injected clock parser.

Create the file and type:

```rust
--8<-- "journey/code/34eb-recency-01.rs"
```

### 2. `src/report_recency_tests.rs`

Cover the original stale-heartbeat regression, active tabs, temporal cutoffs and failure-time ranking.

Create the file and type:

```rust
--8<-- "journey/code/34eb-recency-02.rs"
```

### 3. `src/lib.rs`

Expose native selection policy before binding it to browser storage.

Find this exact block:

```rust
pub mod report_store;
```

Replace that block with:

```rust
--8<-- "journey/code/34eb-recency-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Import sample.pb, move its selected beam, then type Diagnostic Report to download viewer-diagnostic.json. Save still downloads viewer.session. Run the native tests for this lesson’s report policy. Finish the healthy proof view with Move 0.05,0,0 and Fit. Chrome also verifies an actual GPU-loss report and startup-failure download, followed by a healthy reload. Saved previous reports and unsaved-edit recovery are not connected yet.

**Actual Chrome screenshot.**

Native tests verify failure/heartbeat recency and ranking, including the original stale-failure case. Chrome retains real diagnostics/loss acceptance. Additional clock exclusions and browser storage follow.

![Actual browser result: Choose a recent failure without blaming active tabs.](../screenshots/journey/34eb-recency-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Rank failures by lastSeen instead of the first fatal timestamp. Explain why an old tab with a running heartbeat could displace a newer real failure. Then treat all running tabs as interrupted and identify the false notice.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A still-open failed tab can keep updating lastSeen for days. Failure freshness must use the first fatal timestamp; heartbeat age is used separately for interrupted running tabs.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34eb-recency
npm --prefix ../session_tests run course -- save 34eb-recency
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production previous-report notices now date failure independently of heartbeat and reject invalid/future chronology. The course policy establishes candidate eligibility; browser storage, lifecycle telemetry and bounded recovery follow.

[Validation status and course release](release.md).
