# 14.1.1 · Own cancellable sheet-entity lookups

**Chapter 14 · Coordinates and geometry tools · Section 14.1 · Sheet queries and construction planes** · [Contents](contents.md)

**Typing: 9–18 minutes.** [Estimate](typing-load.md).

Complete the original lookup owner with its query id, sheet row and entity index. Share its cancellation flag with the pending reader and cancel when the owner drops. Keep the result and table revision together. The following lookup bridge connects these prepared helpers.

## Type

Continue from [Read original sheet metadata records](70za-read-original-sheet-metadata-records.md). [Save or recover your work](recovery.md).

### 1. `src/sheet_entity.rs`

Extend this original owner with the day’s complete input helpers.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn entity_from(raw: &[u8]) -> Result<EntityMeta, String> {
    serde_json::from_slice(raw).map_err(|error| format!("Entity record is not valid JSON: {error}"))
}
```

</details>

After that block, type:

```rust
/// One entity lookup in flight.
pub struct Query {
    pub id: u64,                   // lookup number
    pub row: u32,                  // the sheet's object row
    pub entity: u32,               // entity index in the sheet
    pub cancelled: Rc<Cell<bool>>, // set when a newer lookup replaces this
}

impl Query {
    /// A new lookup.
    pub fn new(id: u64, row: u32, entity: u32) -> Self {
        Self {
            id,
            row,
            entity,
            cancelled: Rc::new(Cell::new(false)),
        }
    }
}

impl Drop for Query {
    /// Cancel the lookup.
    fn drop(&mut self) {
        self.cancelled.set(true);
    }
}

/// The answer to one lookup.
pub struct Resolved {
    pub query: u64,                                       // which lookup
    pub result: Result<(EntityMeta, SheetTable), String>, // the entity and the table head
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/71-read-original-sheet-entity-metadata-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/sheet_entity.rs`

Supply only the unchanged original metadata and cancellation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use serde::Deserialize;
use std::cell::Cell;
use std::rc::Rc;

/// Bytes of the table head: `SHM1` then a u32 record count.
pub const HEAD_BYTES: u64 = 8;

/// Bytes of one record: u64 offset, u64 length.
pub const RECORD_BYTES: u64 = 16;

/// Largest entity record accepted.
pub const MAX_BLOB: u64 = 64 * 1024;

/// One sheet entity's identity, from its JSON record.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct EntityMeta {
    #[serde(default)]
    pub guid: String, // source object id
    #[serde(default)]
    pub name: String, // display name
    #[serde(default)]
    pub kind: String, // wall, door, ...
    #[serde(default)]
    pub width: f32, // pen width
    #[serde(default)]
    pub color: Vec<f32>, // pen colour
}

/// The table head, cached per sheet.
#[derive(Clone, Debug)]
pub struct SheetTable {
    pub count: u32,               // records in the table
    pub revision: Option<String>, // ETag every read must match
}

/// The record count from a table head.
pub fn table_count(raw: &[u8]) -> Result<u32, String> {
    if raw.len() != HEAD_BYTES as usize || &raw[..4] != b"SHM1" {
        return Err("Side table head is not SHM1".to_string());
    }

    Ok(u32::from_le_bytes(
        raw[4..8].try_into().expect("exact head checked above"),
    ))
}

/// One record as (offset, length).
pub fn record(raw: &[u8]) -> Result<(u64, u64), String> {
    if raw.len() != RECORD_BYTES as usize {
        return Err("Side table record must contain exactly 16 bytes".to_string());
    }

    Ok((
        u64::from_le_bytes(raw[..8].try_into().expect("exact record checked above")),
        u64::from_le_bytes(raw[8..].try_into().expect("exact record checked above")),
    ))
}

/// Byte position of record `id`; `record_at(count)` starts the blobs.
pub fn record_at(id: u32) -> u64 {
    HEAD_BYTES + RECORD_BYTES * u64::from(id)
}

/// An entity from its JSON; missing keys default.
pub fn entity_from(raw: &[u8]) -> Result<EntityMeta, String> {
    serde_json::from_slice(raw).map_err(|error| format!("Entity record is not valid JSON: {error}"))
}

/// One entity lookup in flight.
pub struct Query {
    pub id: u64,                   // lookup number
    pub row: u32,                  // the sheet's object row
    pub entity: u32,               // entity index in the sheet
    pub cancelled: Rc<Cell<bool>>, // set when a newer lookup replaces this
}

impl Query {
    /// A new lookup.
    pub fn new(id: u64, row: u32, entity: u32) -> Self {
        Self {
            id,
            row,
            entity,
            cancelled: Rc::new(Cell::new(false)),
        }
    }
}

impl Drop for Query {
    /// Cancel the lookup.
    fn drop(&mut self) {
        self.cancelled.set(true);
    }
}

/// The answer to one lookup.
pub struct Resolved {
    pub query: u64,                                       // which lookup
    pub result: Result<(EntityMeta, SheetTable), String>, // the entity and the table head
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/71-read-original-sheet-entity-metadata-2.rs" skip=2967
```

</details>

<details>
<summary>Supplied example data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 3. `src/sheet_example.rs`

Supply the existing sheet scene and side-table data specimen.

Copy this supplied file:

```text
--8<-- "journey/code/71-read-original-sheet-entity-metadata-3.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied sheet data specimen.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod sheet_entity;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/71-read-original-sheet-entity-metadata-4.rs" skip=23
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.1.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The rotated and mirrored original references remain visible while original drawing columns, shared segment batches and selected-entity metadata connect. The final checkpoint opens the original plan.

### Recorded visual result

**Previous native render — current verification pending.**

The blue rotated and selected yellow mirrored references retain their original source definition.

![Native renderer output for own cancellable sheet-entity lookups.](../screenshots/journey/70za-read-original-sheet-metadata-records.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.2.6 · Read original sheet metadata records](70za-read-original-sheet-metadata-records.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original table bytes → revision-bound entity → cancellable lookup.



Why share the cancellation flag instead of retaining the query owner?

The reader can observe cancellation without keeping an obsolete lookup alive. Dropping the owner signals that its result is no longer wanted.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a replacement lookup through the old owner’s Drop and the held cancellation flag.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.1.1
npm --prefix ../session_tests run course -- save 14.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited nested-reference exercise remains authored and unrun. The final sheet contract covers complete archive reopening, repeated checked-range placements, metadata only after selection, bounded reads and Close during a lookup. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.1.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
