# 32gia · Load only the sources the requested edit needs

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 30–60 minutes.** 68 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Select source requests from the original edit target, with Save covering the active document.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Intent → original current row → cold origin + epoch; Save → unique active cold origins.

**Before you finish, explain:** Why should Save ignore a released source that exists only in Undo history?

Move and Delete follow the captured ObjectId, even if selection now belongs to a different import. A loaded target needs no source request. A missing target fails before any fetch begins. For a cold row, ReloadKey::of returns its exact origin and release epoch.

Save reuses Editor::reload_keys: it gathers each active cold origin once, even when several rows share it. It excludes loaded rows and sources held only by Undo/Redo history. Three native tests distinguish two imports, later selection, deduplication, history-only sources and missing or loaded targets.

The method borrows Editor only while collecting keys. The returned keys temporarily retain the origins needed by a future ReloadJob; the Intent itself still holds values only. Browser interception and replay are not connected yet. Chrome verifies the existing explicit reload and a subsequent normal Move.

![Choose requested source scope](../illustrations/journey-32gia.svg)

## Type the change

Continue [Capture the requested edit before waiting](32gi-capture.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32gia-scope`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/edit_intent.rs`

Bring the editor and existing release-key type into this module.

Find this exact block:

```rust
use crate::{editor::Action, scene::ObjectId};
```

Replace that block with:

```rust
--8<-- "journey/code/32gia-scope-01.rs"
```

### 2. `src/edit_intent.rs`

Resolve dependencies from the captured target or active Save scene.

Find this exact block:

```rust
impl Intent {
```

Replace that block with:

```rust
--8<-- "journey/code/32gia-scope-02.rs"
```

### 3. `src/lib.rs`

Register the independent source-scope tests.

Find this exact block:

```rust
pub mod edit_intent;
```

Replace that block with:

```rust
--8<-- "journey/code/32gia-scope-03.rs"
```

### 4. `src/edit_scope_tests.rs`

Distinguish target imports from later selection, active Save sources from history, and missing targets from loaded ones.

Create the file and type:

```rust
--8<-- "journey/code/32gia-scope-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

From your project, run `REGEN_PROTO=0 cargo run --example sample --locked -j4`. Type Open Replace, choose sample.pb, Select Next twice, Move 0.35,0,0.25, View Isometric, Orbit Right, Orbit Up, Move 1.92,0,0.93, Move 0.25,0,-0.15 and Fit. Unload Sources and Reload Sources must retain this drawing. Automatic Move/Delete/Save reload is still pending. Then type Move 0,0.25,0 and Fit. Inspect the native scope tests to compare original-target requests with Save requests.

**Actual Chrome screenshot.**

Chrome checks the inherited explicit source reload and this placed drawing. Original-target source selection, active-document Save scope and refusal cases are proved natively; automatic browser replay is still pending.

![Actual browser result: Load only the sources the requested edit needs.](../screenshots/journey/32gia-scope-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

In the first scope test, point selection at a row from the second import. Explain why Move and Delete still request the first import, while Save requests both.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Save serializes the active scene. History-only rows are not in that file, so fetching their source delays Save without supplying any required data. Undo can request that source later when its rows become active.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32gia-scope
npm --prefix ../session_tests run course -- save 32gia-scope
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

The operation determines its source dependencies. Current selection, history size and the number of displayed rows do not determine a request’s authority.

[Validation status and course release](release.md).
