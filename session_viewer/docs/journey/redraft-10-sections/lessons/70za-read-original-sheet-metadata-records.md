# 13.4 · Original sheet entity metadata

**Steps:** 1 · **Source:** [app/scene_instances.rs](../../../../src/app/scene_instances.rs) · [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs) · [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs) · [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">1 Prototype Mesh Def -&gt; N Instance Matrices M_0..M_k</text>
  <polygon points="64,132 104,64 144,132" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="104" y="150" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Def #0</text>
  <line x1="150" y1="98" x2="214" y2="68" stroke="#2563eb" stroke-width="1.75" marker-end="url(#arrB)"/>
  <line x1="150" y1="98" x2="214" y2="104" stroke="#16a34a" stroke-width="1.75" marker-end="url(#arrG)"/>
  <line x1="150" y1="98" x2="214" y2="140" stroke="#dc2626" stroke-width="1.75" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="256" y="72" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">M_0</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="256" y="108" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">M_1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="256" y="144" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">M_2</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D World Space vs 2D Sheet Paper Space (mm)</text>
  <polygon points="434,138 468,78 502,138" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <polygon points="494,138 528,78 562,138" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <rect x="588" y="54" width="112" height="96" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="598" y="64" width="92" height="56" fill="#f8fafc" stroke="#64748b" stroke-width="1.2"/>
  <rect x="646" y="126" width="44" height="16" fill="#fef08a" stroke="#111111" stroke-width="1"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Instanced Scene + 2D Detail Sheet Viewport</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="832,138 860,82 888,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="888,138 916,82 944,138" fill="#bbf7d0" stroke="#111111" stroke-width="1.75"/>
  <polygon points="944,138 972,82 1000,138" fill="#fee2e2" stroke="#111111" stroke-width="1.75"/>
  <rect x="1008" y="56" width="42" height="92" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
</svg>
</div>

<a id="checkpoint-70za-read-original-sheet-metadata-records"></a>

## 1. `src/sheet_entity.rs`

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
