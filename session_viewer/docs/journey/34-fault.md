# 34 · Keep the first GPU failure with its device

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–38 minutes.** 50 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Give asynchronous GPU callbacks one shared first-failure value without confusing an older device with its replacement.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** callback clone → lock shared failure → first reason wins → later errors cannot overwrite it.

**Before you finish, explain:** Why must a late error from an old GPU device remain separate from a new device’s failure state?

GPU failures can arrive through different callbacks. The original device-loss reason is useful evidence; a later validation error caused by that loss should not overwrite it. Start with one small value before connecting browser callbacks.

Fault owns an Arc containing a Mutex-protected optional String. Arc clones refer to the same allocation and satisfy the callback ownership requirements. The mutex makes the check-and-store one operation: exactly one competing writer installs its message. remember returns true only for that writer. message returns an owned copy so the lock is released before the caller reports or schedules work. No mutex guard crosses an await.

A replacement device receives a new Fault::default(), not a clone of the old one. same_device uses Arc::ptr_eq to compare owners, not equal error text. This prepares the later cleanup guard against a stale old callback stopping a replacement runtime. Native tests verify both shared identity and independent lifetimes, and run two real competing threads.

This checkpoint adds the signal and its tests only. The browser still uses its previous error handler; the next checkpoint wires actual GPU callbacks and stops further work. Chrome verifies the inherited viewer behavior, not device-loss handling yet.

![One first failure per device](../illustrations/journey-34.svg)

## Type the change

Continue [Keep a cached viewer ready for Back navigation](33b-cache.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-34-fault`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/gpu_fault.rs`

Create the shared first-failure value; callback clones identify the same device lifetime.

Create the file and type:

```rust
--8<-- "journey/code/34-fault-01.rs"
```

### 2. `src/gpu_fault_tests.rs`

Verify first-reason retention, replacement-device isolation and concurrent callback delivery.

Create the file and type:

```rust
--8<-- "journey/code/34-fault-02.rs"
```

### 3. `src/lib.rs`

Expose the signal and include its native tests.

Find this exact block:

```rust
pub mod background;
```

Replace that block with:

```rust
--8<-- "journey/code/34-fault-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open sample.pb with Open Replace, Select Next, Move 0.35,0,0.25, View Isometric and Fit. Unload Sources, then type Move 0.25,0,0.15 without Reload Sources. The command restores its source and moves once; Undo restores the placement. Repeat with Delete and Undo, then Unload Sources and Save. Save downloads the editable document without creating an Undo step. The sample now includes an original double that differs from its f32 display value. Inspect the actual Save download with the precision checker. Type Orbit Up and Fit for the final proof view. Run the combined acceptance checks. Failed restoration must leave the visible scene unchanged and sources cold. Type Orbit Right and Fit for the final proof view. Finish with Move 0,-0.5,0 and Fit to separate the moved post from the beam; the final proof view avoids coplanar overlapping faces. Type Orbit Right and Fit. Run the debug listener probe and expect [2,2,0]: two live calls, no calls after Drop, and no surviving captured owner. In the browser developer console run Array.from(window.wasmBindings.listener_probe()). The probe is available in this debug checkpoint; release builds omit it. Reload to restart after a lifecycle disposal experiment. In the debug console inspect window.wasmBindings.runtime_running(); dispatch window.dispatchEvent(new Event("pagehide")), then check it on the next task: false. Ordinary document Close still leaves the viewer running. After restarting, finish with View Isometric, Orbit Right and Fit for the final proof view. For a cached transition, dispatch new PageTransitionEvent("pagehide", {persisted:true}) and then pageshow with the same flag. runtime_running stays true; commands still work. Finish with Move 0.1,0,0 and Fit. Finish with Orbit Up and Fit. Run cargo test --lib --locked --target x86_64-unknown-linux-gnu -j4 to exercise the first-failure tests.

**Actual Chrome screenshot.**

Chrome verifies the inherited command, source-restoration and page-lifetime behavior and captures Orbit Up. The new first-failure ownership is checked by native Rust tests; GPU callback integration comes next.

![Actual browser result: Keep the first GPU failure with its device.](../screenshots/journey/34-fault-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Change the first.is_some guard so every callback overwrites the stored reason. Run the tests and explain which useful evidence is lost. Then make a replacement by cloning the old Fault and explain why its identity test fails.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Each new device gets a fresh Arc allocation. Clones share that allocation; same_device compares allocation identity so an old callback cannot be mistaken for the replacement.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 34-fault
npm --prefix ../session_tests run course -- save 34-fault
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The production device module records the first GPU failure in shared state. This value prepares callback integration, stopped submissions, diagnostics and bounded recovery; those are still upcoming.

[Validation status and course release](release.md).
