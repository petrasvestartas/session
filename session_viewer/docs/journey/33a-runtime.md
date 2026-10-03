# 33a · Dispose the viewer without leaving pending work alive

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 19–38 minutes.** 50 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Bind real viewer input to owned callbacks and cancel pending file/source authority when the page hides.

**Follow:** Live runtime → pagehide → next microtask → cancel read/fetch → detach eighteen listeners → release captures.

The main viewer now transfers update into Listeners and records all canvas, document and window bindings. A thread-local Option<Runtime> keeps that owner alive after run returns. install replaces any earlier owner; stop takes the current owner out of the slot and consumes it. The slot’s mutable borrow has ended before Runtime is dropped.

Runtime’s Drop first cancels the read gate and abortable source flight. It then allows the listener owner to detach handlers and release the callback, editor, renderer and other captured values. Cancelling releases pending source keys even when an asynchronous task remains alive. This is ownership cleanup; it does not claim browser or driver memory is reclaimed at the same instant.

pagehide schedules stop through spawn_local and returns immediately. Its task runs on the next microtask so the event callback is no longer active when it is freed. Document Close retains its existing meaning and can still be followed by Open. Page exit disposes the viewer itself.

![Dispose runtime ownership](../illustrations/journey-33a.svg)

## Type the change

Continue from [Own browser listeners instead of forgetting callbacks](33-owner.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-33a-runtime` (from `session_viewer`).

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

In the debug console, dispatch `window.dispatchEvent(new Event("pagehide"))`. On the next task, `window.wasmBindings.runtime_running()` must be false. Reload the same page to restart.

**Verified checkpoint in Chrome.**

![Actual browser result: Dispose the viewer without leaving pending work alive.](../screenshots/journey/33a-runtime-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Skip read/fetch cancellation in Runtime::drop, hold a source response and dispose the viewer. Explain which source URL or late-read acceptance checks reveal retained or revived authority.

</details>

## Explain the change

Why is disposal deferred until the next microtask instead of dropping the runtime inside its own event callback?

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

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Browser listener, document and asynchronous request ownership end together. GPU-loss recovery can now dispose this runtime before starting another one.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Chrome checks the actual main callback’s eighteen removals, an aborted held source request, source URL release, no new GPU submissions after disposal, ignored input and late replies, and cancellation of a held file read before URL adoption. Each restart reloads the same test page; no feature buttons are added. The inherited command/navigation/precision/failure checks remain active.

Chrome verifies real viewer page-exit disposal, eighteen listener removals, source abort/URL release, no later submissions, ignored input/completions, and cancelled file URL adoption; ordinary document Close still permits reopening.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 33a-runtime
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
