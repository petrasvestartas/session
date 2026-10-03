# 30c · Replace a document as one reversible change

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–32 minutes.** 36 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Prepare a whole replacement before changing live rows, then commit it through existing history.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Replacement bytes → validated prepared sources → history transaction → new rows with fresh local IDs.

**Before you finish, explain:** Why must the scene counter survive replacing all its rows?

Append and replace are different document operations. Import adds prepared objects to the current rows. Replace removes the current rows and inserts the new prepared document. Both must use the same validation and history boundary.

Add Action::Replace. First call document::load, which prepares every supported source/display pair. Only after that succeeds does History::try_edit enter Scene::replace. A malformed file therefore cannot clear the old drawing.

![Prepare the full replacement before the transaction; replace rows with fresh IDs; Undo restores the old shared owners.](../illustrations/journey-30c.svg)

Scene::replace clears objects and the extra-demo marker, then imports the already prepared document. It deliberately leaves next_id alone. A transaction restores the old scene if insertion fails halfway through; direct callers must use that transaction boundary too.

The editor’s existing selection repair removes a selected old ID after successful replacement. Camera, projection and background are outside document history and remain unchanged. Undo returns the old rows and source allocations; Redo returns the replacement IDs.

This checkpoint connects replacement to the native editor and tests it. The browser still uses append-only Open today. The next lesson exposes Open Replace and gives the file-read bridge an explicit operation mode.

Old objects retained by Undo are still alive. Replacement alone is not a promise that all old CPU or GPU resources have been released; the resource lessons account for those owners.

## Type the change

Continue [Cancel reads without accepting their late result](30b-cancel.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-30c-replace`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/scene.rs`

Replace rows while preserving the issued local-ID counter; the caller supplies the transaction.

Find this exact block:

```rust
    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
```

Replace that block with:

```rust
--8<-- "journey/code/30c-replace-01.rs"
```

### 2. `src/editor.rs`

Name replacement explicitly instead of giving Import a hidden destructive mode.

Find this exact block:

```rust
    Import(Vec<u8>),
```

Replace that block with:

```rust
--8<-- "journey/code/30c-replace-02.rs"
```

### 3. `src/editor.rs`

Validate every source first, then replace as one undoable transaction.

Find this exact block:

```rust
            Action::Import(bytes) => {
```

Replace that block with:

```rust
--8<-- "journey/code/30c-replace-03.rs"
```

### 4. `src/replace_tests.rs`

Prove replacement/Undo/Redo preserve owners, camera state and non-reused local IDs.

Create the file and type:

```rust
--8<-- "journey/code/30c-replace-04.rs"
```

### 5. `src/lib.rs`

Compile the replacement contract.

Find this exact block:

```rust
#[cfg(test)]
mod save_roundtrip_tests;
```

Replace that block with:

```rust
--8<-- "journey/code/30c-replace-05.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the replacement state check: old local IDs disappear, source allocations return on Undo, replacement IDs return on Redo and camera values stay fixed. In the live viewer, Open the specimen, Example Box, Select Next six times, Move -0.8,-0.6,0.2, View Isometric and Fit. Open Replace becomes available in the next lesson.

**Actual Chrome screenshot.**

![Actual browser result: Replace a document as one reversible change.](../screenshots/journey/30c-replace-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

In the replacement check, change camera distance and aspect before replacing. Predict why Undo and Redo preserve both values. Explain why setting next_id back to one would make an old selection unsafe.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

A replacement must not reuse local IDs held by old selection or delayed work. Clearing rows leaves the increasing counter intact. History restoration also retains the highest issued value, while Undo restores the old objects and shared source allocations.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 30c-replace
npm --prefix ../session_tests run course -- save 30c-replace
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production replacement stages a new scene before committing and preserves the last valid scene on failure. This flat editor teaches the same atomic boundary; browser cancellation, larger sources and resource release are separate concerns.

[Validation status and course release](release.md).
