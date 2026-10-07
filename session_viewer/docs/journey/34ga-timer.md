# 34ga · Own the periodic diagnostic heartbeat

**Typing: 16–31 minutes.** [Estimate](typing-load.md).

Own the browser interval and its Rust closure together. `Timer` stores the Window, native handle and Closure. Drop clears the interval before releasing the callback environment.

## Type

Continue from [Refresh heartbeat without rewriting failure evidence](34g-heartbeat.md). [Save or recover your work](recovery.md).

### 1. `src/timer.rs`

Own the actual interval and callback together; clear its native timer before Rust releases the closure.

Create the file and type:

```rust
--8<-- "journey/code/34ga-timer-01.rs"
```

### 2. `src/lib.rs`

Keep browser interval ownership out of the native editor model.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod report_storage;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-02.rs"
```

### 3. `src/browser_report.rs`

Retain metadata scheduling independently of the GPU runtime lifetime.

<details>
<summary>Locate the existing block</summary>

```rust
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-03.rs"
```

### 4. `src/browser_report.rs`

Install periodic metadata refresh at run startup; report scheduler unavailability without rejecting drawing or current downloads.

<details>
<summary>Locate the existing block</summary>

```rust
    let _ = persist(); Ok(())
}

pub fn observe
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-04.rs"
```

### 5. `src/browser_report.rs`

Replace one owned interval at a time, cancel outside the slot borrow, and keep the callback metadata-only.

<details>
<summary>Locate the existing block</summary>

```rust
fn persist() -> bool {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-05.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Download `Report`, wait at least 15 seconds, then download it again. lastSeen should advance without adding an event. The timer updates metadata only.

**Verified checkpoint in Chrome.**

![Actual browser result: Own the periodic diagnostic heartbeat.](../screenshots/journey/34ga-timer-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

Keep one timer outside the GPU runtime and schedule a metadata heartbeat every 15000 milliseconds. Its callback captures no renderer or document, so diagnostics can continue after device loss.

Stop takes the owner out of the `RefCell` before dropping it. Replacement stops the old timer first; failed registration leaves no old timer running. Never dispose the timer from inside its active callback. Browser suspension/final-exit handling follows.

metadata startup → Timer owns native handle and Closure → heartbeat persists metadata → clear interval before callback release.

![Own interval and callback together](../illustrations/journey-34ga.svg)

Why must clearing the interval happen before dropping its Rust Closure?

JavaScript holds the callback function while the interval is registered. Dropping the Rust environment first could leave a callback that invokes released state. The owner clears the native interval in Drop before field cleanup releases the closure.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Call start_periodic twice and inspect the cleared handles. Make registration throw and explain why the previous owner must not remain running. Never drop the owner from inside its own active callback.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 34ga-timer
npm --prefix ../session_tests run course -- save 34ga-timer
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production periodically saves diagnostics. The course now has explicit callback ownership and real scheduled persistence independent of GPU loss. Automatic lifecycle/error observations, full load telemetry and bounded recovery follow.

The test observes the real 15000-millisecond registration, then substitutes faster browser delivery to verify callbacks without a 15-second wait. During inherited acceptance it gates delivery, preserving the preceding checkpoints’ exact snapshot comparisons; custom acceptance enables real callback invocation afterward. This is semantic scheduling/ownership evidence, not a phone performance measurement. It proves lastSeen refresh with unchanged events/outcome, zero GPU calls inside each callback, continued metadata after real device destruction, first-failure preservation, exact cancellation and replacement, and ready startup when registration is denied.

Chrome observes actual 15-second interval registration and accelerates test delivery. It verifies owned callback replacement/cancellation, independent post-loss metadata, unchanged failure/events, zero callback GPU work and usable startup/downloads when scheduling fails.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 34ga-timer
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
