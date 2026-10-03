# 28a · Retain the imported mesh behind each row

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 16–32 minutes.** 34 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Insert imported source geometry and its prepared display together.

**Follow:** Decoded session → prepared source/display pair → Object source geometry → history snapshot.

Make `Loaded` carry `PreparedMesh` values and retain the source geometry behind each imported row. Preparation still finishes before the scene changes.

The kernel session already owns meshes through `Rc`; clone those handles rather than copying geometry. Keep the session too, because it owns file context such as tree and document name.

During insertion, moving `prepared.display` transfers that field while leaving the other fields available for transfer. History wraps the whole import in one transaction.

Imported rows now have `Some` source geometry. Older demos and generated boxes temporarily have `None`; the next lesson closes that migration gap.

![A retained source session supplies prepared meshes; each imported row keeps its source geometry and GUID.](../illustrations/journey-28a.svg)

## Type the change

Continue from [Prepare a display from an owned source mesh](28-record.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-28a-imported` (from `session_viewer`).

### 1. `src/document.rs`

Load source/display pairs rather than display arrays alone.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::mesh::Mesh;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28a-imported-01.rs"
```

### 2. `src/document.rs`

Retain the prepared geometry alongside its display.

<details>
<summary>Locate the existing block</summary>

```rust
    pub meshes: Vec<(String, Mesh)>,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28a-imported-02.rs"
```

### 3. `src/document.rs`

Share the session’s existing kernel mesh owner, then attach retained file provenance.

<details>
<summary>Locate the existing block</summary>

```rust
        meshes.push((mesh.guid().to_owned(), Mesh::from_kernel(mesh)?));
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28a-imported-03.rs"
```

### 4. `src/scene.rs`

During migration, distinguish rows with source geometry from older demo rows.

<details>
<summary>Locate the existing block</summary>

```rust
    pub mesh: Rc<Mesh>,
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28a-imported-04.rs"
```

### 5. `src/scene.rs`

Keep the existing insertion path while imported rows begin retaining source geometry.

<details>
<summary>Locate the existing block</summary>

```rust
        self.objects.push(Object { id, mesh: Rc::new(mesh), model: session_rust::Xform::identity(), source: None });
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28a-imported-05.rs"
```

### 6. `src/scene.rs`

Keep each prepared display and source on the same stable object record.

<details>
<summary>Locate the existing block</summary>

```rust
        for (guid, mesh) in loaded.meshes {
            self.insert(mesh)?;
            self.objects.last_mut().unwrap().source = Some(crate::document::Source {
                document: Rc::clone(&loaded.document), guid,
            });
        }
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28a-imported-06.rs"
```

### 7. `src/imported_source_tests.rs`

Prove import shares the session’s existing source allocation even when earlier display rows disappear.

Create the file and type:

```rust
--8<-- "journey/code/28a-imported-07.rs"
```

### 8. `src/lib.rs`

Run the source allocation check at this intermediate checkpoint.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod prepared_tests;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28a-imported-08.rs"
```

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Open `sample.pb`. Run the import checks below: every imported row must retain both its original source mesh and its prepared display. Failed preparation must add no rows.

**Verified checkpoint in Chrome.**

![Actual browser result: Retain the imported mesh behind each row.](../screenshots/journey/28a-imported-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

In the import test, remove both original demo IDs before inspecting the imported object. Its row number changes while its ObjectId, source GUID and geometry allocation remain stable. Restore the test before checking your files.

</details>

## Explain the change

If rows move after deletion, how can the object still find its source?

<details>
<summary>Compare your explanation</summary>

Source identity uses a retained session and GUID, not the row index. The object also directly owns the original kernel mesh through Rc. Its display can move to another row without changing either source owner.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 28a-imported
npm --prefix ../session_tests run course -- save 28a-imported
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The production scene keeps source identity through row replacement and compaction. This lesson connects source ownership to our smaller row table; generated objects adopt the same path next.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Retain the imported mesh behind each row. These commands run in the actual dock; kernel ownership is checked separately in Rust.

[Full validation scope](release.md).

</details>
