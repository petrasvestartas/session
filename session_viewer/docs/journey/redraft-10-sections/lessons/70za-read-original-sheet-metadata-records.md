# 13.4 · Original sheet entity metadata

**Goal:** Describe the original sheet side-table layout and entity identity fields. · **Steps:** 1 · **Source:** [app/scene_instances.rs](../../../../src/app/scene_instances.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs) · [app/hierarchy.rs](../../../../src/app/hierarchy.rs) · [app/sheet_query.rs](../../../../src/app/sheet_query.rs) (`src/app/instances.rs,src/app/sheets.rs,src/engine/gpu/instances.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 13.4 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/sheet_entity.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1 Coding Step</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/instances.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/sheets.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Read sheet metadata recor…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Shared Block Definition DAG</text>
  <rect x="644" y="48" width="112" height="28" rx="5" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Def #0 (1 Mesh)</text>
  <line x1="674" y1="76" x2="634" y2="106" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <line x1="726" y1="76" x2="766" y2="106" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="594" y="110" width="92" height="28" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="128" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Inst A (M0)</text>
  <rect x="714" y="110" width="92" height="28" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="760" y="128" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Inst B (M1)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Original sheet entity metad…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. 2D Drawing Sheet &amp; Views</text>
  <rect x="888" y="48" width="176" height="92" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="898" y="56" width="96" height="54" rx="2" fill="#f8fafc" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="946" y="86" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Detail View</text>
  <rect x="984" y="116" width="72" height="18" rx="2" fill="#fef9c3" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1020" y="129" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#111111">Title Block</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Read sheet metadata recor…</text>
</svg>
</div>

<a id="checkpoint-70za-read-original-sheet-metadata-records"></a>

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/sheet_entity.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original table bytes</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">revision-bound entity</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/instances.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cancellable lookup</text>
</svg>
</div>

**What this code does:** Describe the original sheet side-table layout and entity identity fields.

## 1. `src/sheet_entity.rs`

Type the whole original metadata or lookup helpers using their shared owner.

Create the file and type:

```rust
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
        raw[4..8]
            .try_into()
            .expect("exact head checked above"),
    ))
}

/// One record as (offset, length).
pub fn record(raw: &[u8]) -> Result<(u64, u64), String> {
    if raw.len() != RECORD_BYTES as usize {
        return Err("Side table record must contain exactly 16 bytes".to_string());
    }

    Ok((
        u64::from_le_bytes(
            raw[..8]
                .try_into()
                .expect("exact record checked above"),
        ),
        u64::from_le_bytes(
            raw[8..]
                .try_into()
                .expect("exact record checked above"),
        ),
    ))
}

/// Byte position of record `id`; `record_at(count)` starts the blobs.
pub fn record_at(id: u32) -> u64 {
    HEAD_BYTES + RECORD_BYTES * u64::from(id)
}

/// An entity from its JSON; missing keys default.
pub fn entity_from(raw: &[u8]) -> Result<EntityMeta, String> {
    serde_json::from_slice(raw)
        .map_err(|error| format!("Entity record is not valid JSON: {error}"))
}
```

## 2. `src/lib.rs`

Compile this shared original metadata module.

**Locate this existing block:**

```rust
mod nested_placement_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod nested_placement_tests;

pub mod sheet_entity;

#[cfg(target_arch = "wasm32")]
```

## Build & Run

![Recorded visual output for Lesson 13.4 (70za-read-original-sheet-metadata-records)](../../../screenshots/journey/70za-read-original-sheet-metadata-records.png)

```sh
npm --prefix ../session_tests run course -- dependencies 13.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 13.4
npm --prefix ../session_tests run course -- save 13.4
```

Continue with [14.1 · Original drawings, entity queries and construction frames](../../consolidation/71-read-original-sheet-entity-metadata.md).
