# 33b · Keep a cached viewer ready for Back navigation

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 3–5 minutes.** 3 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Distinguish a reusable cached page from a final exit so Back navigation cannot restore an already-disposed viewer.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** pagehide → cancel drag → persisted? → keep owner for cached return; otherwise deferred disposal.

**Before you finish, explain:** Does every pagehide mean the page and its WebAssembly instance are going away forever?

The [HTML lifecycle contract](https://html.spec.whatwg.org/dev/nav-history-apis.html#the-pagetransitionevent-interface) distinguishes final departure from a page that may return from the back/forward cache. A cached page reuses its existing document and scripts. Disposing its callbacks without restarting them would leave that restored viewer unable to accept commands.

The handler cancels an unfinished camera gesture, then checks the actual PageTransitionEvent. A persisted transition keeps the runtime. A final exit still schedules the disposal proved in the previous checkpoint. A plain synthetic Event has no persisted flag and retains the previous final-exit behavior. No new listener or feature control is needed.

Chrome dispatches persisted hide/show events, verifies unchanged drawing, placement, selection and camera, and then uses real commands and a drag to prove the retained viewer works. It also navigates the SAME test tab away and Back: if Chrome caches the WebGPU page, its document token and drawing must survive; if the browser performs a fresh load, a newly running viewer is required. The test logs which path actually occurred, rather than claiming cache eligibility from a synthetic event. Final-exit and delayed source/file disposal checks remain inherited.

![Retain a reusable page](../illustrations/journey-33b.svg)

## Type the change

Continue [Dispose the viewer without leaving pending work alive](33a-runtime.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-33b-cache`. A save keeps your own work; it does not fill in the next lesson.

### 1. `Cargo.toml`

Enable the browser event type that tells us whether the page may return from its cache.

Find this exact block:

```toml
"AddEventListenerOptions", "Event",
```

Replace that block with:

```toml
--8<-- "journey/code/33b-cache-01.rs"
```

### 2. `src/browser.rs`

Cancel an unfinished drag, retain the cached page’s live owner, and dispose only a final page exit.

Find this exact block:

```rust
        if event.type_() == "pagehide" {
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
```

Replace that block with:

```rust
--8<-- "journey/code/33b-cache-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open sample.pb with Open Replace, Select Next, Move 0.35,0,0.25, View Isometric and Fit. Unload Sources, then type Move 0.25,0,0.15 without Reload Sources. The command restores its source and moves once; Undo restores the placement. Repeat with Delete and Undo, then Unload Sources and Save. Save downloads the editable document without creating an Undo step. The sample now includes an original double that differs from its f32 display value. Inspect the actual Save download with the precision checker. Type Orbit Up and Fit for the final proof view. Run the combined acceptance checks. Failed restoration must leave the visible scene unchanged and sources cold. Type Orbit Right and Fit for the final proof view. Finish with Move 0,-0.5,0 and Fit to separate the moved post from the beam; the final proof view avoids coplanar overlapping faces. Type Orbit Right and Fit. Run the debug listener probe and expect [2,2,0]: two live calls, no calls after Drop, and no surviving captured owner. In the browser developer console run Array.from(window.wasmBindings.listener_probe()). The probe is available in this debug checkpoint; release builds omit it. Reload to restart after a lifecycle disposal experiment. In the debug console inspect window.wasmBindings.runtime_running(); dispatch window.dispatchEvent(new Event("pagehide")), then check it on the next task: false. Ordinary document Close still leaves the viewer running. After restarting, finish with View Isometric, Orbit Right and Fit for the final proof view. For a cached transition, dispatch new PageTransitionEvent("pagehide", {persisted:true}) and then pageshow with the same flag. runtime_running stays true; commands still work. Finish with Move 0.1,0,0 and Fit.

**Actual Chrome screenshot.**

Chrome checks persisted transition handling, cancelled drag, retained state and working commands, then a real same-tab away/Back navigation. The test records whether Chrome reused the page or loaded it fresh; final-exit disposal remains verified.

![Actual browser result: Keep a cached viewer ready for Back navigation.](../screenshots/journey/33b-cache-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Remove the persisted check and run the retained-runtime assertions. Explain why a fresh normal load could conceal the broken cached-return path.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

No. PageTransitionEvent.persisted marks a page that may be reused on Back navigation. Retain its runtime in that case; a final exit still disposes the viewer.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 33b-cache
npm --prefix ../session_tests run course -- save 33b-cache
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

A final disposal and a temporarily cached document have different ownership lifetimes. Device-loss recovery remains the next subject.

[Validation status and course release](release.md).
