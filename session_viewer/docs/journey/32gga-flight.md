# 32gga · Pair reload ownership with a browser abort controller

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 12–24 minutes.** 35 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Pair reload ownership with a browser abort controller.

**Follow:** ReloadJob + AbortController → captured signal → cancel or current finish.

Define `Flight` with the native job and one AbortController. `begin` cancels the previous flight, creates the browser controller and gives the fetch task its signal together with URL strings. The controller stays in the current owner.

`cancel` removes keys and takes the controller before calling abort. `finish` checks the ticket through the job first. A matching failure aborts any remaining response work; a successful complete batch simply drops its controller. An older completion cannot reach either path.

`Shared` is an `Rc<RefCell<Flight>>` because one browser event callback and its asynchronous task need the same owner. Borrow only for short synchronous operations. Never keep a RefCell borrow across await: another event must be able to cancel while a fetch is waiting.

![Pair reload ownership with a browser abort controller](../illustrations/journey-32gga.svg)

## Type the change

Continue from [Read a source response within its byte limit](32gg-fetch.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32gga-flight` (from `session_viewer`).

### 1. `src/lib.rs`

Keep the browser flight beside the fetch adapter.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
mod source_fetch;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32gga-flight-01.rs"
```

### 2. `src/browser_reload.rs`

Cancel keys and I/O together without letting old results touch a new flight.

Create the file and type:

```rust
--8<-- "journey/code/32gga-flight-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the checks below and build the browser bundle. The flight now pairs its ticket with an AbortController; cancelling it removes authority and aborts its fetch.

**Verified checkpoint in Chrome.**

![Actual browser result: Pair reload ownership with a browser abort controller.](../screenshots/journey/32gga-flight-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Move controller.take before the job ticket check in a scratch copy. Trace how a late failure could remove the newer request controller.

</details>

## Explain the change

Why must an old failed request not abort the newer controller?

<details>
<summary>Compare your explanation</summary>

Only a matching current ticket may finish and take the current controller. A late result first fails the job ticket check and cannot touch newer work.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gga-flight
npm --prefix ../session_tests run course -- save 32gga-flight
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Abort is an I/O request, not permission to commit. The native ticket and release-key checks remain necessary even when network cancellation behaves correctly.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

The browser checks existing command-only unloading and retained drawing. The new infrastructure is compiled here; the Reload Sources command is connected in the following command lesson.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32gga-flight
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
