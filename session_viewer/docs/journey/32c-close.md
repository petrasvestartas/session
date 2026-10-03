# 32c · Close the document without resetting the view

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 20–40 minutes.** 41 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Drop active rows and both history branches while preserving the local ID counter and camera.

**Follow:** Close action → scene storage release and history reset → empty renderer synchronization.

Close is different from Delete and Open Replace. Delete is an undoable edit. Replacement is one transaction that lets Undo recover the previous document. Close ends that document history entirely.

![Close drops active and history owners, then empty synchronization releases GPU rows.](../illustrations/journey-32c.svg)

Scene::clear drops its row-vector storage and extra-row marker without resetting next_id. Replace reuses that clearing operation inside its existing transaction. Action::Close calls it directly, clears both history branches and clears selection. It returns a scene change so the normal renderer synchronization can release old GPU rows.

The tests create both old document and Redo roots, close, and require empty rows, no surviving history and a later object ID that was never used by the old document. They also check that camera and background settings remain unchanged and repeated close is harmless.

The native GPU proof closes the editor first, then synchronizes an empty scene. Kernel sources/documents are released when the editor roots disappear; CPU displays held by GPU geometry remain until those draw rows are dropped. The browser command is connected in the next lesson.

## Type the change

Continue from [Count each shared GPU buffer once](32b-gpu.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32c-close` (from `session_viewer`).

### 1. `src/scene.rs`

Release row storage while preserving issued identities; reuse this inside atomic replacement.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.objects.clear();
        self.extra = None;
        self.import(loaded)
    }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32c-close-01.rs"
```

### 2. `src/editor.rs`

Give closing a typed editor action.

<details>
<summary>Locate the existing block</summary>

```rust
    Replace(Vec<u8>),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32c-close-02.rs"
```

### 3. `src/editor.rs`

Close outside undoable document edits and preserve application view settings.

<details>
<summary>Locate the existing block</summary>

```rust
            Action::Replace(bytes) => {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32c-close-03.rs"
```

### 4. `src/editor.rs`

Check source release, both history branches, identity reuse and view preservation.

<details>
<summary>Locate the existing block</summary>

```rust
    #[test]
    fn resetting_the_view_preserves_its_current_aspect() {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/32c-close-04.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the close checks below. Closing empties active rows, Undo and Redo while preserving camera and ID issuance. The typed Close command follows next.

**Verified checkpoint in Chrome.**

![Actual browser result: Close the document without resetting the view.](../screenshots/journey/32c-close-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Compare calling Delete three times with calling Close once. Predict which sources history retains in each case, and whether Undo can restore the old document.

</details>

## Explain the change

Why does Close keep next_id even though it drops every row?

<details>
<summary>Compare your explanation</summary>

Old asynchronous work may still refer to an issued local identity. Reusing those identities would make it ambiguous which document row was meant. Empty the row storage and history, clear selection, but keep the monotonically issued counter. Camera and background are application settings and remain outside document close.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32c-close
npm --prefix ../session_tests run course -- save 32c-close
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

Closing, replacing, unloading source-only data and recovering a released source have different contracts. The following source-unload/reload lessons preserve visible display rows instead of emptying them.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32c-close
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
