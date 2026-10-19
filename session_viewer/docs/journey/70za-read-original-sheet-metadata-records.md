# 13.2.6 · Read original sheet metadata records

**Chapter 13 · Shared definitions and drawing sheets · Section 13.2 · Feature ancestry and sheet metadata** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Describe the original sheet side-table layout and entity identity fields. Check the eight-byte header and sixteen-byte record before reading their little-endian integers. Parse the JSON entity with its original defaults and retain the table revision for the following lookup.

## Type

Continue from [Exercise rotated and mirrored original references](70d-exercise-rotated-and-mirrored-original-references.md). [Save or recover your work](recovery.md).

### 1. `src/sheet_entity.rs`

Type the whole original metadata or lookup helpers using their shared owner.

Create the file and type:

```rust
--8<-- "journey/code/70za-read-original-sheet-metadata-records-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original metadata module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod nested_placement_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/70za-read-original-sheet-metadata-records-2.rs" skip=41
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 13.2.6
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The earlier drawing remains visible while the sheet metadata helpers are prepared.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read original sheet metadata records.](../screenshots/journey/70za-read-original-sheet-metadata-records.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original table bytes → revision-bound entity → cancellable lookup.



Why retain the revision beside the record count?

The table header and later range reads must describe the same source revision. A count from an older table cannot safely identify records in a newer file.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one header, one record and one JSON entity. Compare missing identity keys with malformed bytes.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 13.2.6
npm --prefix ../session_tests run course -- save 13.2.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original metadata helpers are preparatory until the existing sheet lookup bridge connects them. The earlier drawing stays unchanged.



[Full validation scope](release.md).

</details>
