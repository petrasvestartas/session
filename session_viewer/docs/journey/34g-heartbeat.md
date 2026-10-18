# 4.2.15 · Refresh heartbeat without rewriting failure evidence

**Chapter 4 · Browser lifetime and recovery · Section 4.2 · GPU diagnostics and recovery** · [Contents](contents.md)

**Typing: 8–15 minutes.** [Estimate](typing-load.md).

Refresh lastSeen without recording an event. A periodic heartbeat would otherwise fill the 24-event window and push out useful observations.

## Type

Continue from [Preserve unsupported telemetry while pruning](34fc-retention.md). [Save or recover your work](recovery.md).

### 1. `src/diagnostic.rs`

Advance heartbeat independently of observations, outcome and the first fatal timestamp; invalid timestamp text leaves the report unchanged.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn heartbeat(&mut self, time: String) -> Result<(), &'static str> {
        if time.is_empty() || time.chars().count() > 64 { return Err("Invalid heartbeat timestamp"); }
        self.last_seen = time; Ok(())
    }

    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34g-heartbeat-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/heartbeat_tests.rs`

Optional authoring fixture. Verify all run outcomes, first-failure/event retention and atomic timestamp-text bounds without a browser clock dependency.

Copy this supplied file:

```text
--8<-- "journey/code/34g-heartbeat-02.rs"
```

</details>

### 3. `src/lib.rs`

Include native heartbeat state proofs.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod diagnostic;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod heartbeat_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34g-heartbeat-03.rs"
```

</details>

### 4. `src/browser_report.rs`

Refresh actual browser context and UTC heartbeat, release its borrow, then persist metadata without invoking renderer or recording synthetic observations.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn download() -> Result<(), JsValue> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn heartbeat() -> Result<(), JsValue> {
    let context = context()?; let time = js_sys::Date::new_0().to_iso_string().into();
    REPORT.with(|slot| {
        let mut slot = slot.borrow_mut(); let report = slot.as_mut().ok_or("No report")?;
        report.heartbeat(time).map_err(JsValue::from_str)?; report.context = context; Ok::<_, JsValue>(())
    })?;
    let _ = persist(); Ok(())
}

pub fn download() -> Result<(), JsValue> {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/34g-heartbeat-04.rs"
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

In the debug console, call `window.wasmBindings.heartbeat()`. Download `Report`: lastSeen advances, while outcome, events and the first failure stay unchanged.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for refresh heartbeat without rewriting failure evidence.](../screenshots/journey/34g-heartbeat.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

`Report::heartbeat` validates timestamp text before updating that single field. Running, Ready, Closed and Failed keep their outcome, events and first fatal reason.

The browser wrapper supplies current UTC/context and persists after ending the report borrow. It receives no renderer or document. This lesson supplies the operation; the next owns its timer.

real UTC/context → bounded heartbeat update → release report borrow → bounded metadata persistence.

![Refresh activity without rewriting failure](../illustrations/journey-34g.svg)

Why must a heartbeat not append a fatal event or rewrite the first failure time?

A heartbeat proves recent observation of a page, not a new failure. Rewriting failure time would recreate the stale-warning bug, while synthetic heartbeat events would rotate meaningful observations out of the recent window.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Implement heartbeat through record("fatal", ...), then predict which failure/outcome assertion fails. Implement it as a recent observation instead and explain why frequent beats would evict useful events.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 4.2.15
npm --prefix ../session_tests run course -- save 4.2.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production updates saved heartbeat on its periodic timer. The course now defines the metadata-only operation; owned periodic scheduling, lifecycle/error observers, detailed load telemetry and bounded recovery follow.

Headed Chrome invokes this real wrapper through a debug export, reads the corresponding localStorage value and checks unchanged drawing, selection, camera, history and geometry counters. After destroying an actual GPU device it advances the heartbeat again without new GPU work, then proves the first fatal timestamp/message and events are unchanged. It reloads the same test page to restore the healthy proof view.

Chrome updates and reads real stored heartbeat metadata, preserves scene/history/events and first failure, and proves heartbeat after actual GPU loss performs no GPU work. Periodic scheduling follows next.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 4.2.15
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
