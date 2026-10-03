# 33a · Dispose the viewer without leaving pending work alive

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–38 minutes.** 50 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Bind real viewer input to owned callbacks and cancel pending file/source authority when the page hides.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Live runtime → pagehide → next microtask → cancel read/fetch → detach eighteen listeners → release captures.

**Before you finish, explain:** Why is disposal deferred until the next microtask instead of dropping the runtime inside its own event callback?

The main viewer now transfers update into Listeners and records all canvas, document and window bindings. A thread-local Option<Runtime> keeps that owner alive after run returns. install replaces any earlier owner; stop takes the current owner out of the slot and consumes it. The slot’s mutable borrow has ended before Runtime is dropped.

Runtime’s Drop first cancels the read gate and abortable source flight. It then allows the listener owner to detach handlers and release the callback, editor, renderer and other captured values. Cancelling releases pending source keys even when an asynchronous task remains alive. This is ownership cleanup; it does not claim browser or driver memory is reclaimed at the same instant.

pagehide schedules stop through spawn_local and returns immediately. Its task runs on the next microtask so the event callback is no longer active when it is freed. Document Close retains its existing meaning and can still be followed by Open. Page exit disposes the viewer itself.

Chrome checks the actual main callback’s eighteen removals, an aborted held source request, source URL release, no new GPU submissions after disposal, ignored input and late replies, and cancellation of a held file read before URL adoption. Each restart reloads the same test page; no feature buttons are added. The inherited command/navigation/precision/failure checks remain active.

![Dispose runtime ownership](../illustrations/journey-33a.svg)

## Type the change

Continue [Own browser listeners instead of forgetting callbacks](33-owner.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-33a-runtime`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/browser_runtime.rs`

Keep the live owner in one runtime slot; Drop cancels read/fetch authority before listeners and captured editor/GPU owners are released.

Create the file and type:

```rust
--8<-- "journey/code/33a-runtime-01.rs"
```

### 2. `src/lib.rs`

Register browser runtime ownership beside the listener owner.

Find this exact block:

```rust
mod listeners;
```

Replace that block with:

```rust
--8<-- "journey/code/33a-runtime-02.rs"
```

### 3. `src/browser.rs`

Retain cancellation handles outside the moving callback. Defer page-exit disposal until the active callback has returned.

Find this exact block:

```rust
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let line
```

Replace that block with:

```rust
--8<-- "journey/code/33a-runtime-03.rs"
```

### 4. `src/browser.rs`

Transfer the event callback into the owner before attaching its targets.

Find this exact block:

```rust
    for name in [
        "pointerdown",
```

Replace that block with:

```rust
--8<-- "journey/code/33a-runtime-04.rs"
```

### 5. `src/browser.rs`

Register all eighteen live bindings through the owner; retain it in the runtime instead of forgetting the Rust callback.

Find this exact block:

```rust
        canvas.add_event_listener_with_callback(name, update.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        update.as_ref().unchecked_ref(),
        &options,
    )?;
    window.add_event_listener_with_callback("resize", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("blur", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("change", update.as_ref().unchecked_ref())?;
    document.add_event_listener_with_callback("cancel", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-file-error", update.as_ref().unchecked_ref())?;
    window.add_event_listener_with_callback("viewer-reload", update.as_ref().unchecked_ref())?;
    // Both event sources retain this one callback for the lifetime of the page.
    update.forget();
```

Replace that block with:

```rust
--8<-- "journey/code/33a-runtime-05.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open sample.pb with Open Replace, Select Next, Move 0.35,0,0.25, View Isometric and Fit. Unload Sources, then type Move 0.25,0,0.15 without Reload Sources. The command restores its source and moves once; Undo restores the placement. Repeat with Delete and Undo, then Unload Sources and Save. Save downloads the editable document without creating an Undo step. The sample now includes an original double that differs from its f32 display value. Inspect the actual Save download with the precision checker. Type Orbit Up and Fit for the final proof view. Run the combined acceptance checks. Failed restoration must leave the visible scene unchanged and sources cold. Type Orbit Right and Fit for the final proof view. Finish with Move 0,-0.5,0 and Fit to separate the moved post from the beam; the final proof view avoids coplanar overlapping faces. Type Orbit Right and Fit. Run the debug listener probe and expect [2,2,0]: two live calls, no calls after Drop, and no surviving captured owner. In the browser developer console run Array.from(window.wasmBindings.listener_probe()). The probe is available in this debug checkpoint; release builds omit it. Reload to restart after a lifecycle disposal experiment. In the debug console inspect window.wasmBindings.runtime_running(); dispatch window.dispatchEvent(new Event("pagehide")), then check it on the next task: false. Ordinary document Close still leaves the viewer running. After restarting, finish with View Isometric, Orbit Right and Fit for the final proof view.

**Actual Chrome screenshot.**

Chrome verifies real viewer page-exit disposal, eighteen listener removals, source abort/URL release, no later submissions, ignored input/completions, and cancelled file URL adoption; ordinary document Close still permits reopening.

![Actual browser result: Dispose the viewer without leaving pending work alive.](../screenshots/journey/33a-runtime-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Skip read/fetch cancellation in Runtime::drop, hold a source response and dispose the viewer. Explain which source URL or late-read acceptance checks reveal retained or revived authority.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The pagehide callback is still executing from the owned closure. Disposal waits for that invocation to return, then cancels work and drops the closure safely. spawn_local schedules the task for the next microtask even when its future is immediately ready.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 33a-runtime
npm --prefix ../session_tests run course -- save 33a-runtime
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Browser listener, document and asynchronous request ownership end together. GPU-loss recovery can now dispose this runtime before starting another one.

[Validation status and course release](release.md).
