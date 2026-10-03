# 32gg · Read a source response within its byte limit

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 24–47 minutes.** 40 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Read a source response within its byte limit.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Recorded URL → fetch with AbortSignal → HTTP/length checks → bounded stream chunks.

**Before you finish, explain:** Why is checking only Content-Length insufficient?

Enable the browser fetch, response, stream-reader and abort types. `source_fetch::fetch` uses a borrowed URL and AbortSignal, checks the HTTP status and optional length, then reads the response body as a stream. The current tutorial limit remains 4 MiB per source; it is not a measurement of browser staging or total process memory.

`JsFuture::from` lets Rust await a browser promise. `dyn_into` checks that the returned JavaScript value has the expected browser type. Each reader reply contains `done` and a byte-array `value`; `Reflect::get` reads those fields. Check remaining capacity before converting and appending a chunk.

On read failure, cancel the reader and release its lock. On success, release the lock and return original bytes. Version/schema checks still belong to source preparation, before adoption. This helper is compiled here; the browser command will exercise it after the flight and delivery boundaries exist.

![Read a source response within its byte limit](../illustrations/journey-32gg.svg)

## Type the change

Continue [Prove cancelled work releases its source owners](32gfa-ownership.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gg-fetch`. A save keeps your own work; it does not fill in the next lesson.

### 1. `Cargo.toml`

Enable only the browser types used by streaming and cancellation.

Find this exact block:

```toml
"Url", "HtmlAnchorElement"]
```

Replace that block with:

```toml
--8<-- "journey/code/32gg-fetch-01.toml"
```

### 2. `src/lib.rs`

Keep browser promises and streams out of the native editor.

Find this exact block:

```rust
#[cfg(target_arch = "wasm32")]
mod browser;
```

Replace that block with:

```rust
--8<-- "journey/code/32gg-fetch-02.rs"
```

### 3. `src/source_fetch.rs`

Read original byte chunks without exceeding the per-source limit.

Create the file and type:

```rust
--8<-- "journey/code/32gg-fetch-03.rs"
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

Build the streaming reader for WebAssembly. Continue using Unload Sources in the browser; the helper is not a user command yet.

**Actual Chrome screenshot.**

The browser checks existing command-only unloading and retained drawing. The new infrastructure is compiled here; the Reload Sources command is connected in the following command lesson.

![Actual browser result: Read a source response within its byte limit.](../screenshots/journey/32gg-fetch-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Remove the incremental limit in a scratch copy. Explain what a response without a length header could allocate before the final check.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The header can be missing or wrong. Check it before reading when present, then enforce the same limit on every received chunk. Never append a chunk that would exceed the per-source byte budget.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gg-fetch
npm --prefix ../session_tests run course -- save 32gg-fetch
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Later publication lessons extend fetching to large published sources and their version policy. This immutable File/Blob path preserves exact source bytes and a bounded read for the current mesh subset.

[Validation status and course release](release.md).
