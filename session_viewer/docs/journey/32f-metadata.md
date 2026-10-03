# 32f · Keep row metadata separate from editable geometry

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 28–56 minutes.** 62 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Retain names, original source GUIDs and visibility/locking flags without keeping kernel owners alive.

**In the whole viewer:** Connect the browser, drawing and input, then extend the same project. [See the destination](../journey.md#the-destination).

**Follow:** Prepared source → independent row metadata → shared scene snapshots → source ownership proof.

**Before you finish, explain:** Why is a row’s original source GUID separate from its saved object GUID?

Source unloading keeps the picture while dropping eligible editable kernel data. Before dropping any owner, give each row the small descriptive values it will still need. The display mesh, placement and local ObjectId already live on the row. Names and flags currently live inside the kernel mesh.

![A scene row keeps metadata and its derived display independently of editable kernel owners.](../illustrations/journey-32f.svg)

RowMetadata copies the original source GUID, name, is_visible and is_locked when Scene inserts a prepared mesh. These are owned strings and booleans, with no Rc to the kernel mesh or imported Session. The row shares an Rc<RowMetadata> across history snapshots: Move changes placement, so it does not need to copy or alter metadata. Later attribute edits will replace or copy this metadata deliberately.

Keep source_guid distinct from Object.guid. Repeated imports can have the same source GUID but need different saved identities. This is the distinction introduced in lesson29; it now remains available without borrowing a kernel mesh.

The native check retains only metadata and Weak observers, closes the editor, and proves the old kernel value is gone while the name and flags remain readable. Another check follows duplicate imports, Move and Undo. Chrome inspects a hidden diagnostic attribute to compare the same metadata through import and history; no interface button or teaching panel is added.

This endpoint does not unload a source yet. Both source and display still exist. Visibility and locking are preserved as values; their full drawing/editing policy is taught later in the course.

## Type the change

Continue [Prove release does not retain the old document](32e-release.md). From `session_viewer`, save your files with `npm --prefix ../session_tests run course -- save before-32f-metadata`. A save keeps your own work; it does not fill in the next lesson.

### 1. `src/lib.rs`

Give independently retained row metadata its own small module.

Find this exact block:

```rust
pub mod prepared;
```

Replace that block with:

```rust
--8<-- "journey/code/32f-metadata-01.rs"
```

### 2. `src/row_metadata.rs`

Copy source identity and flags without copying or retaining kernel payload; test independent lifetime and history.

Create the file and type:

```rust
--8<-- "journey/code/32f-metadata-02.rs"
```

### 3. `src/scene.rs`

Retain descriptive values independently of the editable source.

Find this exact block:

```rust
    pub guid: String,
```

Replace that block with:

```rust
--8<-- "journey/code/32f-metadata-03.rs"
```

### 4. `src/scene.rs`

Capture metadata before transferring prepared source owners into the row.

Find this exact block:

```rust
        self.objects.push(Object { id, guid, mesh: Rc::new(prepared.display), geometry: prepared.geometry,
```

Replace that block with:

```rust
--8<-- "journey/code/32f-metadata-04.rs"
```

### 5. `src/browser.rs`

Expose row values through hidden inspection for actual Chrome import/history checks.

Find this exact block:

```rust
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
```

Replace that block with:

```rust
--8<-- "journey/code/32f-metadata-05.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Import the same file twice. Compare original source GUIDs with saved row GUIDs, then Move and Undo. The metadata must stay unchanged. Run the native ownership check to prove that retained metadata alone cannot pin a closed source.

**Actual Chrome screenshot.**

![Actual browser result: Keep row metadata separate from editable geometry.](../screenshots/journey/32f-metadata-browser.png)

*Captured from this checkpoint’s browser bundle after its browser checks passed. [Check scope and environment](release.md).*

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

## Try one small experiment

Replace one row’s metadata Rc in a test with newly owned values. Predict why an older scene snapshot must keep the original values. Do not mutate a shared metadata allocation behind history.

## Explain it in your own words

Trace the values through the files without reading the answer first. If you lose the connection, stop at the last value you can follow.

<details>
<summary>Compare your explanation</summary>

The original source GUID locates the mesh in the imported document. The saved object GUID distinguishes duplicate imports in this viewer. Forking the saved identity must not change the original identity used to restore a released source.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 32f-metadata
npm --prefix ../session_tests run course -- save 32f-metadata
```

The comparison spots typing differences; it does not prove behaviour. Keep three notes: what I changed; the values I followed; the question I still have. [Recover a checkpoint](recovery.md) if an experiment gets tangled.

## Where this grows

Production source release retains shape/name/tree information while its editable Session is unloaded. This row boundary prepares that same lifetime distinction; release across active/history roots and guarded rehydration remain required next steps.

[Validation status and course release](release.md).
