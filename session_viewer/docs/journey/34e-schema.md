# 34e · Check the bounded shape of a diagnostic report

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Validate a report before trusting it. `valid` bounds identifiers, timestamp text and events; it requires sensible finite dimensions/timing and agreement between Failed and the retained first fatal event.

The first fatal event may be older than the recent-event window. A recent fatal event still requires a retained failure. Validation must accept that distinction.

## Type

Continue from [Download diagnostics through the real command line](34d-download.md). [Save or recover your work](recovery.md).

### 1. `src/diagnostic.rs`

Validate shape, limits and failure/outcome consistency before trusting a decoded stored value.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34e-schema-01.rs"
```

### 2. `src/diagnostic.rs`

Keep recording bounded even if a caller bypasses admission and directly deserializes an oversized queue.

<details>
<summary>Locate the existing block</summary>

```rust
        if self.events.len() == 24 { self.events.pop_front(); }
```

</details>

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

<details>
<summary>Locate the existing block</summary>

```rust
pub mod diagnostic;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34e-schema-04.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the report-policy checks below. Invalid dimensions, excess events or inconsistent fatal evidence must be rejected. A valid first failure may survive outside the rotated event window.

**Verified checkpoint in Chrome.**

![Actual browser result: Check the bounded shape of a diagnostic report.](../screenshots/journey/34e-schema-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Use `while` when trimming a deserialized queue: removing one event is insufficient if its input already exceeded the limit. Repairing that queue does not validate the rest of the report. Byte limits, raw JSON fields and timestamp interpretation follow.

typed report → bounded metadata and observations → consistent failure/outcome → valid shape.

![Validate typed report shape](../illustrations/journey-34e.svg)

Why must a deserialized report satisfy our shape invariants before it can be used?

Deserialization alone does not enforce our version, metadata limits, bounded event window or consistent first-failure/outcome state. Those checks are separate from JSON syntax and later timestamp recency.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Directly deserialize 25 recent events, inspect valid(), then record one new observation and inspect it again. Explain why queue repair cannot validate the rest of the stored format.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 34e-schema
npm --prefix ../session_tests run course -- save 34e-schema
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The saved-report reader must validate its metadata before use. This endpoint establishes typed shape invariants; raw JSON admission, recency, real storage, telemetry and recovery follow.

Chrome verifies inherited real diagnostic downloads and GPU-loss/restart behavior. Browser storage is connected only after the complete admission and recency checks.

Chrome checks inherited actual diagnostic downloads and failure/restart behavior. Native tests establish typed report shape; raw JSON admission and browser storage follow.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34e-schema
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
