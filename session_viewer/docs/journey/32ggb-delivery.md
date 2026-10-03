# 32ggb · Deliver only the current completed reload batch

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 14–28 minutes.** 30 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Deliver only the current completed reload batch.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Flight begin → await URL reads → finish ticket once → scoped Rust reply → viewer event.

**Before you finish, explain:** Why is the restored source data kept in a Rust delivery slot rather than event.detail?

Add a `Delivery` slot and spawn one local asynchronous task. Fetch each requested URL with the captured signal. A failure stops the batch; successful bytes are kept in request order. No Scene changes while any read is pending.

After awaiting, ask the shared owner to finish this ticket. A cancelled or replaced request returns immediately, before reporting an error or dispatching a viewer event. For the current request, zip the accepted keys with its bytes and place that Result in the Rust slot for synchronous dispatch. Clear any unclaimed result after dispatch.

The asynchronous task holds the shared flight, plain strings, signal and delivery slot. Cancellation empties the flight keys, so a delayed task does not keep the old Origin alive. Event callbacks will take the accepted slot and call the atomic hydration API; the command is connected next.

![Deliver only the current completed reload batch](../illustrations/journey-32ggb.svg)

## Type the change

Continue [Pair reload ownership with a browser abort controller](32gga-flight.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32ggb-delivery`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/browser_reload.rs`

Spawn bounded source reads and deliver only one current owned completion.

Find this exact block:

```rust
pub type Shared = Rc<RefCell<Flight>>;
```

Replace that block with:

```rust
--8<-- "journey/code/32ggb-delivery-01.rs"
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

Build the complete request/fetch/delivery path. It remains dormant until the command-line wiring in the next endpoint.

**Actual Chrome screenshot.**

The browser checks existing command-only unloading and retained drawing. The new infrastructure is compiled here; the Reload Sources command is connected in the following command lesson.

![Actual browser result: Deliver only the current completed reload batch.](../screenshots/journey/32ggb-delivery-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Dispatch viewer-reload from the browser console without an accepted slot. Predict why it cannot restore any source.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The slot carries the accepted release keys with their bytes. The event only wakes the viewer; an external event without accepted Rust data cannot fabricate a source completion.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32ggb-delivery
npm --prefix ../session_tests run course -- save 32ggb-delivery
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The accepted File bridge and the source-reload bridge use the same principle: retain native ownership through one accepted handoff and do not let generic browser events become source authority.

[Validation status and course release](release.md).
