# 34g · Refresh heartbeat without rewriting failure evidence

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 17–33 minutes.** 36 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Refresh actual last-seen metadata independently of first failure, event history and GPU ownership.

**Follow:** real UTC/context → bounded heartbeat update → release report borrow → bounded metadata persistence.

Refresh lastSeen without recording an event. A periodic heartbeat would otherwise fill the 24-event window and push out useful observations.

`Report::heartbeat` validates timestamp text before updating that single field. Running, Ready, Closed and Failed keep their outcome, events and first fatal reason.

The browser wrapper supplies current UTC/context and persists after ending the report borrow. It receives no renderer or document. This lesson supplies the operation; the next owns its timer.

![Refresh activity without rewriting failure](../illustrations/journey-34g.svg)

## Type the change

Continue from [Preserve unsupported telemetry while pruning](34fc-retention.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-34g-heartbeat` (from `session_viewer`).

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

In the debug console, call `window.wasmBindings.heartbeat()`. Download `Diagnostic Report`: lastSeen advances, while outcome, events and the first failure stay unchanged.

**Verified checkpoint in Chrome.**

![Actual browser result: Refresh heartbeat without rewriting failure evidence.](../screenshots/journey/34g-heartbeat-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Implement heartbeat through record("fatal", ...), then predict which failure/outcome assertion fails. Implement it as a recent observation instead and explain why frequent beats would evict useful events.

</details>

## Explain the change

Why must a heartbeat not append a fatal event or rewrite the first failure time?

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

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Production updates saved heartbeat on its periodic timer. The course now defines the metadata-only operation; owned periodic scheduling, lifecycle/error observers, detailed load telemetry and bounded recovery follow.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Headed Chrome invokes this real wrapper through a debug export, reads the corresponding localStorage value and checks unchanged drawing, selection, camera, history and geometry counters. After destroying an actual GPU device it advances the heartbeat again without new GPU work, then proves the first fatal timestamp/message and events are unchanged. It reloads the same test page to restore the healthy proof view.

Chrome updates and reads real stored heartbeat metadata, preserves scene/history/events and first failure, and proves heartbeat after actual GPU loss performs no GPU work. Periodic scheduling follows next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34g-heartbeat
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
