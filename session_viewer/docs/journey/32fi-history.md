# 32fi · Unload sources across active and history roots

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–32 minutes.** 32 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Release a validated whole-origin set without clearing Undo/Redo or changing placements.

**Follow:** Release a validated whole-origin set without clearing Undo/Redo or changing placements..

Plan before mutating. Collect distinct eligible origins from all retained scene roots, then inspect every row of those origins across active, Undo and Redo roots. Remove an origin from the plan if any retained row is protected. An empty plan returns an error rather than pretending that anything was unloaded.

Only then issue a checked release epoch and convert every matching row through Scene::unload. This residency pass is not an undoable document change: it must preserve all existing history roots and their placements. Close keeps the issued epoch counter, so a later reopened document cannot reuse an old completion token.

Move and Delete currently return an explicit reload-required error for cold rows. Camera, bounds, picking and history still use retained display/model state. The reload lessons will restore sources and replay the requested edit; these interim errors are not the completed production edit flow. Save already refuses unavailable editable geometry.

The next endpoint adds the typed ownership/history check for a moved imported post. Existing loaded-source and policy checks remain current here, and the native GPU fixture exercises unloading, Weak source/document expiration and retained display/GPU owners. No Unload Sources dock command is exposed until the next endpoint.

![Release a validated whole-origin set without clearing Undo/Redo or changing placements.](../illustrations/journey-32fi.svg)

## Type the change

Continue from [Protect sources that cannot be unloaded faithfully](32fh-policy.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-32fi-history` (from `session_viewer`).

### 1. `src/history.rs`

Visit both history branches without cloning or dropping their rows.

Find this exact block:

```rust
    pub fn clear(&mut self) {
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-01.rs"
```

### 2. `src/editor.rs`

Add a residency operation distinct from document close.

Find this exact block:

```rust
    Close,
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-02.rs"
```

### 3. `src/editor.rs`

Issue epochs independently of document history and Close.

Find this exact block:

```rust
    history: History,
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-03.rs"
```

### 4. `src/editor.rs`

Start the first viewer-owned release sequence without wraparound.

Find this exact block:

```rust
            history: History::default(),
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-04.rs"
```

### 5. `src/editor.rs`

Validate every retained origin row before issuing an epoch or changing any owner.

Find this exact block:

```rust
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-05.rs"
```

### 6. `src/editor.rs`

Keep source unloading out of document Undo transactions.

Find this exact block:

```rust
            Action::Close => {
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-06.rs"
```

### 7. `src/scene.rs`

Refuse an edit until its original editable source has been restored.

Find this exact block:

```rust
        object.model = model;
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-07.rs"
```

### 8. `src/editor.rs`

Keep selection/history unchanged when a cold edit requires hydration.

Find this exact block:

```rust
            Action::Delete => {
                if let Some(id) = self.selected.take() {
```

Replace that block with:

```rust
--8<-- "journey/code/32fi-history-08.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the history-release checks below. Releasing an origin must visit matching active, Undo and Redo rows while retaining their display and placement.

**Verified checkpoint in Chrome.**

![Actual browser result: Unload sources across active and history roots.](../screenshots/journey/32fi-history-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Keep an external clone of Source in a fixture and explain why the editor can release its own roots while that external owner still keeps the kernel alive.

</details>

## Explain the change

Why is unloading only the current Scene insufficient?

<details>
<summary>Compare your explanation</summary>

Its Undo and Redo rows retain the same kernel owners. They would keep the source alive and later restore a different residency. Convert every retained row of a validated origin together while keeping its separate placement and display owner.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32fi-history
npm --prefix ../session_tests run course -- save 32fi-history
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

This models source residency across our shared Scene snapshots without clearing history. Guarded hydration and automatic edit replay remain required before claiming complete source-release parity.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>



[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 32fi-history
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
