# 34e · Check the bounded shape of a diagnostic report

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 30–60 minutes.** 52 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Validate a typed report’s limits and failure/outcome invariants before the later storage decoder adopts it.

**Follow:** typed report → bounded metadata and observations → consistent failure/outcome → valid shape.

Validate a report before trusting it. `valid` bounds identifiers, timestamp text and events; it requires sensible finite dimensions/timing and agreement between Failed and the retained first fatal event.

The first fatal event may be older than the recent-event window. A recent fatal event still requires a retained failure. Validation must accept that distinction.

Use `while` when trimming a deserialized queue: removing one event is insufficient if its input already exceeded the limit. Repairing that queue does not validate the rest of the report. Byte limits, raw JSON fields and timestamp interpretation follow.

![Validate typed report shape](../illustrations/journey-34e.svg)

## Type the change

Continue from [Download diagnostics through the real command line](34d-download.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-34e-schema` (from `session_viewer`).

### 1. `src/diagnostic.rs`

Validate shape, limits and failure/outcome consistency before trusting a decoded stored value.

Find this exact block:

```rust
    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

Replace that block with:

```rust
--8<-- "journey/code/34e-schema-01.rs"
```

### 2. `src/diagnostic.rs`

Keep recording bounded even if a caller bypasses admission and directly deserializes an oversized queue.

Find this exact block:

```rust
        if self.events.len() == 24 { self.events.pop_front(); }
```

Replace that block with:

```rust
--8<-- "journey/code/34e-schema-02.rs"
```

### 3. `src/diagnostic_shape_tests.rs`

Verify typed metadata, bounded queues and consistent first-failure state.

Create the file and type:

```rust
--8<-- "journey/code/34e-schema-03.rs"
```

### 4. `src/lib.rs`

Include the native shape-invariant checks.

Find this exact block:

```rust
pub mod diagnostic;
```

Replace that block with:

```rust
--8<-- "journey/code/34e-schema-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the report-policy checks below. Invalid dimensions, excess events or inconsistent fatal evidence must be rejected. A valid first failure may survive outside the rotated event window.

**Verified checkpoint in Chrome.**

![Actual browser result: Check the bounded shape of a diagnostic report.](../screenshots/journey/34e-schema-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Directly deserialize 25 recent events, inspect valid(), then record one new observation and inspect it again. Explain why queue repair cannot validate the rest of the stored format.

</details>

## Explain the change

Why must a deserialized report satisfy our shape invariants before it can be used?

<details>
<summary>Compare your explanation</summary>

Deserialization alone does not enforce our version, metadata limits, bounded event window or consistent first-failure/outcome state. Those checks are separate from JSON syntax and later timestamp recency.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34e-schema
npm --prefix ../session_tests run course -- save 34e-schema
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The saved-report reader must validate its metadata before use. This endpoint establishes typed shape invariants; raw JSON admission, recency, real storage, telemetry and recovery follow.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome verifies inherited real diagnostic downloads and GPU-loss/restart behavior. Browser storage is connected only after the complete admission and recency checks.

Chrome checks inherited actual diagnostic downloads and failure/restart behavior. Native tests establish typed report shape; raw JSON admission and browser storage follow.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34e-schema
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
