# 32gga · Pair reload ownership with a browser abort controller

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 12–24 minutes.** 35 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Pair reload ownership with a browser abort controller.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** ReloadJob + AbortController → captured signal → cancel or current finish.

**Before you finish, explain:** Why must an old failed request not abort the newer controller?

Define `Flight` with the native job and one AbortController. `begin` cancels the previous flight, creates the browser controller and gives the fetch task its signal together with URL strings. The controller stays in the current owner.

`cancel` removes keys and takes the controller before calling abort. `finish` checks the ticket through the job first. A matching failure aborts any remaining response work; a successful complete batch simply drops its controller. An older completion cannot reach either path.

`Shared` is an `Rc<RefCell<Flight>>` because one browser event callback and its asynchronous task need the same owner. Borrow only for short synchronous operations. Never keep a RefCell borrow across await: another event must be able to cancel while a fetch is waiting.

![Pair reload ownership with a browser abort controller](../illustrations/journey-32gga.svg)

## Type the change

Continue [Read a source response within its byte limit](32gg-fetch.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gga-flight`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/lib.rs`

Keep the browser flight beside the fetch adapter.

Find this exact block:

```rust
#[cfg(target_arch = "wasm32")]
mod source_fetch;
```

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

When Trunk reloads after a code change, the viewer starts with its generated demo again. From `workspace/journey`, make the sample using the example you already typed:

```sh
REGEN_PROTO=0 cargo run --example sample --locked -j4
```

Type `Open Replace`, choose `sample.pb`, then type `Select Next` twice, `Move 0.35,0,0.25`, `View Isometric` and `Fit`. You now have a moved object from a retained, reloadable file.

Compile the browser flight and compare its ownership with the native job tests. Fetch spawning and result delivery are added next.

**Actual Chrome screenshot.**

The browser checks existing command-only unloading and retained drawing. The new infrastructure is compiled here; the Reload Sources command is connected in the following command lesson.

![Actual browser result: Pair reload ownership with a browser abort controller.](../screenshots/journey/32gga-flight-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Move controller.take before the job ticket check in a scratch copy. Trace how a late failure could remove the newer request controller.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

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

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Abort is an I/O request, not permission to commit. The native ticket and release-key checks remain necessary even when network cancellation behaves correctly.

[Validation status and course release](release.md).
