# 30c · Replace a document as one reversible change

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Add `Action::Replace` for one reversible document replacement. First prepare the entire file with `document::load`; only then enter `History::try_edit` and replace the rows.

`Scene::replace` clears objects and the demo marker, then inserts prepared objects. Keep the ID counter advancing. The transaction restores the old scene if insertion fails partway through.

## Type

Continue from [Cancel reads without accepting their late result](30b-cancel.md). [Save or recover your work](recovery.md).

### 1. `src/scene.rs`

Replace rows while preserving the issued local-ID counter; the caller supplies the transaction.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30c-replace-01.rs"
```

### 2. `src/editor.rs`

Name replacement explicitly instead of giving Import a hidden destructive mode.

<details>
<summary>Locate the existing block</summary>

```rust
    Import(Vec<u8>),
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30c-replace-02.rs"
```

### 3. `src/editor.rs`

Validate every source first, then replace as one undoable transaction.

<details>
<summary>Locate the existing block</summary>

```rust
            Action::Import(bytes) => {
```

</details>

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

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod save_roundtrip_tests;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/30c-replace-05.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the replacement checks below. A valid replacement is one Undo step; an invalid candidate must leave the existing scene unchanged. The browser command is connected next.

**Verified checkpoint in Chrome.**

![Actual browser result: Replace a document as one reversible change.](../screenshots/journey/30c-replace-browser.png)

[Verification scope](release.md).

Run the state checks on your computer from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked --target host-tuple -j4
```

<details>
<summary>Code explanation and diagram</summary>

Repair selection after success. Camera, projection and background remain outside document history. Undo restores the old shared owners; Redo restores the replacement objects.

This step tests the editor operation; browser Open is still append-only until the next lesson. Old objects retained by Undo remain alive and are accounted for later.

Replacement bytes → validated prepared sources → history transaction → new rows with fresh local IDs.

![Prepare the full replacement before the transaction; replace rows with fresh IDs; Undo restores the old shared owners.](../illustrations/journey-30c.svg)

Why must the scene counter survive replacing all its rows?

A replacement must not reuse local IDs held by old selection or delayed work. Clearing rows leaves the increasing counter intact. History restoration also retains the highest issued value, while Undo restores the old objects and shared source allocations.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

In the replacement check, change camera distance and aspect before replacing. Predict why Undo and Redo preserve both values. Explain why setting next_id back to one would make an old selection unsafe.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 30c-replace
npm --prefix ../session_tests run course -- save 30c-replace
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Production replacement stages a new scene before committing and preserves the last valid scene on failure. This flat editor teaches the same atomic boundary; browser cancellation, larger sources and resource release are separate concerns.



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 30c-replace
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
