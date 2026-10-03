# 32 · Find the owners retained by history

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 15–30 minutes.** 38 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Inspect undo/redo roots and release both branches explicitly.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Scene snapshots → shared source/display owners → history reset → ownership tests.

**Before you finish, explain:** Why can deleting every visible row leave its source alive?

Deleting a row removes it from drawing, but Undo can bring it back. That requires an owner somewhere. Our History keeps scene snapshots whose rows clone Rc owners rather than copying mesh payloads.

![Active rows and undo/redo snapshots can retain the same source allocation.](../illustrations/journey-32.svg)

Add a borrowed iterator over both history branches. It lets accounting inspect the roots without cloning their owners. clear replaces History with its empty default, dropping the snapshots and their vector storage. Ordinary edits still keep history; this explicit reset will be used by document close.

The first test removes a row, confirms history retains its source/display values, then clears history and confirms those values are gone. The second leaves an extra object only in Redo and proves clearing both branches prevents it from returning.

[Weak::upgrade](https://doc.rust-lang.org/std/rc/struct.Weak.html#method.upgrade) lets a check observe whether a value still has an owner. A failed upgrade proves the managed value is gone; the Weak control allocation itself remains until its weak references are dropped. It is not a measurement of process memory.

The browser still behaves as before at this endpoint. We are making the ownership roots inspectable before wiring a destructive document operation into commands.

## Type the change

Continue [Update only changed object settings](31c-incremental.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32-history`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/history.rs`

Expose borrowed snapshot roots and an explicit reset of both branches.

Find this exact block:

```rust
    pub fn edit(&mut self, scene: &mut Scene, action: impl FnOnce(&mut Scene)) {
```

Replace that block with:

```rust
--8<-- "journey/code/32-history-01.rs"
```

### 2. `src/history.rs`

Prove deletion differs from dropping the owners retained by Undo and Redo.

Find this exact block:

```rust
    #[test]
    fn undo_restores_identity_and_shares_the_original_mesh() {
```

Replace that block with:

```rust
--8<-- "journey/code/32-history-02.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j4
CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the state checks. Open Replace the specimen, Move a post and Undo it; ordinary history must still work. No Close command exists yet.

**Actual Chrome screenshot.**

![Actual browser result: Find the owners retained by history.](../screenshots/journey/32-history-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
cargo test --lib --locked -j4
```

## Try one small experiment

Keep a cloned Rc of a removed display in a test scope. Predict why clearing history will no longer make its Weak upgrade fail until that extra owner is dropped.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

Undo and Redo hold scene snapshots. Their rows share the original source and display owners so history can restore them. Removing visible rows does not remove those snapshot owners. Closing a document must drop both history branches as well as the active rows.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32-history
npm --prefix ../session_tests run course -- save 32-history
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

History ownership matters for source release, document replacement and large scenes. Production source unloading also retains display and tree metadata; its later lessons are distinct from closing the document.

[Validation status and course release](release.md).
