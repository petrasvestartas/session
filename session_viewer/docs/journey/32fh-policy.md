# 32fh · Protect sources that cannot be unloaded faithfully

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 21–41 minutes.** 35 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Require a reload location, original kernel allocation and unchanged row metadata.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Require a reload location, original kernel allocation and unchanged row metadata..

**Before you finish, explain:** Why does one modified history row protect every row of that origin?

A release policy must preserve data that cannot be restored. Generated rows have no imported source. Imports without a location cannot be read again. A new kernel allocation or edited source metadata must not silently revert to the original file.

can_unload checks a located Loaded source, its original GUID, name/visibility/locking metadata, and the exact kernel allocation retained by its imported Session. The allocation check distinguishes original data from a modified replacement even when its values happen to compare equal. Model placement is separate row state, so Move does not make an otherwise original source ineligible.

Add borrowed mutable row access for the forthcoming residency pass, and Scene::unload as the operation on a previously validated set of origins. It changes only editable ownership to Released. The editor-wide origin validation comes next; no dock command exists yet.

Native policy tests protect generated/unlocated sources, changed flags and replacement kernel owners. The whole-origin check in the next lesson will inspect active, Undo and Redo roots before any mutation.

![Require a reload location, original kernel allocation and unchanged row metadata.](../illustrations/journey-32fh.svg)

## Type the change

Continue [Separate loaded and released editable ownership](32fg-state.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32fh-policy`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/scene.rs`

Only original, located imported sources can be restored by their original file.

Find this exact block:

```rust
    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
```

Replace that block with:

```rust
--8<-- "journey/code/32fh-policy-01.rs"
```

### 2. `src/scene.rs`

Apply a validated origin set without touching retained display/model/metadata owners.

Find this exact block:

```rust
    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
```

Replace that block with:

```rust
--8<-- "journey/code/32fh-policy-02.rs"
```

### 3. `src/scene.rs`

Prove the conditions that protect data not faithfully reloadable from the original file.

Find this exact block:

```rust
    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
```

Replace that block with:

```rust
--8<-- "journey/code/32fh-policy-03.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run policy tests, then Move an imported specimen with the old browser behavior. The new policy is not a document edit or a new history entry.

**Actual Chrome screenshot.**

![Actual browser result: Protect sources that cannot be unloaded faithfully.](../screenshots/journey/32fh-policy-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Change only placement in a fixture, then change a source flag. Explain why the first remains reloadable while the second must be protected at this endpoint.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

They share one reload source. Reloading its original bytes cannot reproduce modified kernel data in that snapshot. Protect the whole origin whenever any retained row no longer matches the original source contract.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32fh-policy
npm --prefix ../session_tests run course -- save 32fh-policy
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production avoids unloading edited source documents. Our scene-snapshot history requires checking every retained origin row before releasing their shared sources.

[Validation status and course release](release.md).
