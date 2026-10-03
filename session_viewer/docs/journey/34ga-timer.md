# 34ga · Own the periodic diagnostic heartbeat

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–32 minutes.** 41 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Schedule a real 15-second metadata heartbeat independently of GPU lifetime and cancel its callback through an owner.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** metadata startup → Timer owns native handle and Closure → heartbeat persists metadata → clear interval before callback release.

**Before you finish, explain:** Why must clearing the interval happen before dropping its Rust Closure?

Timer owns a native interval handle, its Window and the Rust Closure backing its JavaScript callback. Registration must succeed before the owner is returned. If registration fails, the unregistered closure drops normally. Drop clears the interval before normal field cleanup releases the callback environment.

The report module stores one Timer independently of the GPU runtime. Startup replaces prior scheduling, then installs a 15000-millisecond callback that captures no device, renderer, document or source owner. It calls the metadata heartbeat operation only. Losing the GPU does not cancel diagnostics or rewrite the original fatal time. A timer cannot guarantee delivery while a browser suspends a page; lifecycle handling is the next responsibility.

Explicit stop takes the owner outside the RefCell borrow before dropping it. Repeated stop is harmless. Replacement clears the previous handle before a new registration; a failed replacement leaves no old timer running. The callback itself never disposes its own owner. These functions are ordinary Rust interfaces; debug exports let Chrome test actual lifecycle behavior without feature buttons.

The test observes the real 15000-millisecond registration, then substitutes faster browser delivery to verify callbacks without a 15-second wait. During inherited acceptance it gates delivery, preserving the preceding checkpoints’ exact snapshot comparisons; custom acceptance enables real callback invocation afterward. This is semantic scheduling/ownership evidence, not a phone performance measurement. It proves lastSeen refresh with unchanged events/outcome, zero GPU calls inside each callback, continued metadata after real device destruction, first-failure preservation, exact cancellation and replacement, and ready startup when registration is denied.

Final-exit and cached-page pause/resume still need automatic hooks. The next checkpoint connects them to this owner.

![Own interval and callback together](../illustrations/journey-34ga.svg)

## Type the change

Continue [Refresh heartbeat without rewriting failure evidence](34g-heartbeat.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34ga-timer`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/timer.rs`

Own the actual interval and callback together; clear its native timer before Rust releases the closure.

Create the file and type:

```rust
--8<-- "journey/code/34ga-timer-01.rs"
```

### 2. `src/lib.rs`

Keep browser interval ownership out of the native editor model.

Find this exact block:

```rust
pub mod report_storage;
```

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-02.rs"
```

### 3. `src/browser_report.rs`

Retain metadata scheduling independently of the GPU runtime lifetime.

Find this exact block:

```rust
    static PREVIOUS: RefCell<Option<Report>> = const { RefCell::new(None) };
```

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-03.rs"
```

### 4. `src/browser_report.rs`

Install periodic metadata refresh at run startup; report scheduler unavailability without rejecting drawing or current downloads.

Find this exact block:

```rust
    let _ = persist(); Ok(())
}

pub fn observe
```

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-04.rs"
```

### 5. `src/browser_report.rs`

Replace one owned interval at a time, cancel outside the slot borrow, and keep the callback metadata-only.

Find this exact block:

```rust
fn persist() -> bool {
```

Replace that block with:

```rust
--8<-- "journey/code/34ga-timer-05.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Use the viewer normally: diagnostics now refresh every 15 seconds, including after GPU failure. Chrome observes the actual interval registration and accelerates delivery only in the test to prove metadata refresh, replacement/cancellation and no GPU work. Scheduler denial leaves the viewer ready and current downloads usable. Finish the proof view with Move 0,-0.05,0 and Fit. Lifecycle suspension and final-exit cleanup follow next.

**Actual Chrome screenshot.**

Chrome observes actual 15-second interval registration and accelerates test delivery. It verifies owned callback replacement/cancellation, independent post-loss metadata, unchanged failure/events, zero callback GPU work and usable startup/downloads when scheduling fails.

![Actual browser result: Own the periodic diagnostic heartbeat.](../screenshots/journey/34ga-timer-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Call start_periodic twice and inspect the cleared handles. Make registration throw and explain why the previous owner must not remain running. Never drop the owner from inside its own active callback.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

JavaScript holds the callback function while the interval is registered. Dropping the Rust environment first could leave a callback that invokes released state. The owner clears the native interval in Drop before field cleanup releases the closure.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34ga-timer
npm --prefix ../session_tests run course -- save 34ga-timer
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production periodically saves diagnostics. The course now has explicit callback ownership and real scheduled persistence independent of GPU loss. Automatic lifecycle/error observations, full load telemetry and bounded recovery follow.

[Validation status and course release](release.md).
