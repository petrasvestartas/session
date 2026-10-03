# 34g · Refresh heartbeat without rewriting failure evidence

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 17–33 minutes.** 36 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Refresh actual last-seen metadata independently of first failure, event history and GPU ownership.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** real UTC/context → bounded heartbeat update → release report borrow → bounded metadata persistence.

**Before you finish, explain:** Why must a heartbeat not append a fatal event or rewrite the first failure time?

A heartbeat updates one run value: lastSeen. Its timestamp text must be nonempty and at most 64 Unicode scalar values. Native tests preserve every other field across running, ready, closed and failed outcomes, including the first failure and all recent observations. Invalid text is refused before mutation. Browser Date parsing and recency remain separate policies.

The browser wrapper supplies an actual UTC timestamp and current viewport/canvas/density context, then persists a cloned report after releasing the report borrow. It has no renderer, source document or GPU argument and creates no observation. Repeated beats cannot fill the24-event window or change a failed outcome to ready.

Headed Chrome invokes this real wrapper through a debug export, reads the corresponding localStorage value and checks unchanged drawing, selection, camera, history and geometry counters. After destroying an actual GPU device it advances the heartbeat again without new GPU work, then proves the first fatal timestamp/message and events are unchanged. It reloads the same test page to restore the healthy proof view.

This endpoint supplies the operation; it deliberately installs no timer. The next checkpoint owns a real periodic callback and verifies cancellation rather than forgetting a Closure. Lifecycle suspension/resume and final-exit handling follow that owner.

![Refresh activity without rewriting failure](../illustrations/journey-34g.svg)

## Type the change

Continue [Preserve unsupported telemetry while pruning](34fc-retention.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34g-heartbeat`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/diagnostic.rs`

Advance heartbeat independently of observations, outcome and the first fatal timestamp; invalid timestamp text leaves the report unchanged.

Find this exact block:

```rust
    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

Replace that block with:

```rust
--8<-- "journey/code/34g-heartbeat-01.rs"
```

### 2. `src/heartbeat_tests.rs`

Verify all run outcomes, first-failure/event retention and atomic timestamp-text bounds without a browser clock dependency.

Create the file and type:

```rust
--8<-- "journey/code/34g-heartbeat-02.rs"
```

### 3. `src/lib.rs`

Include native heartbeat state proofs.

Find this exact block:

```rust
pub mod diagnostic;
```

Replace that block with:

```rust
--8<-- "journey/code/34g-heartbeat-03.rs"
```

### 4. `src/browser_report.rs`

Refresh actual browser context and UTC heartbeat, release its borrow, then persist metadata without invoking renderer or recording synthetic observations.

Find this exact block:

```rust
pub fn download() -> Result<(), JsValue> {
```

Replace that block with:

```rust
--8<-- "journey/code/34g-heartbeat-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run native state checks and use the viewer normally. Chrome calls the real browser heartbeat entry point, reads the stored JSON and verifies lastSeen/context refresh without scene or event changes. It also refreshes a report after real GPU destruction and preserves the first fatal evidence. No interval is installed yet; the next checkpoint owns and schedules periodic heartbeats. Finish the healthy proof view with Move 0.05,0,0 and Fit.

**Actual Chrome screenshot.**

Chrome updates and reads real stored heartbeat metadata, preserves scene/history/events and first failure, and proves heartbeat after actual GPU loss performs no GPU work. Periodic scheduling follows next.

![Actual browser result: Refresh heartbeat without rewriting failure evidence.](../screenshots/journey/34g-heartbeat-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Implement heartbeat through record("fatal", ...), then predict which failure/outcome assertion fails. Implement it as a recent observation instead and explain why frequent beats would evict useful events.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A heartbeat proves recent observation of a page, not a new failure. Rewriting failure time would recreate the stale-warning bug, while synthetic heartbeat events would rotate meaningful observations out of the recent window.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34g-heartbeat
npm --prefix ../session_tests run course -- save 34g-heartbeat
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production updates saved heartbeat on its periodic timer. The course now defines the metadata-only operation; owned periodic scheduling, lifecycle/error observers, detailed load telemetry and bounded recovery follow.

[Validation status and course release](release.md).
