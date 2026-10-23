# 13.4 · Original sheet entity metadata

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Lesson 13.4** (Redraft `7.8`) · **1 checkpoint (`70za-read-original-sheet-metadata-records`)**

> **Learning Goal:** Describe the original sheet side-table layout and entity identity fields.

## Visual WebGPU & Architecture Schematic

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 7 WebGPU Streaming & Instancing: HTTP Range Requests, Shared Source Meshes & 2D Drawing Sheets</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How massive multi-gigabyte scenes stream out-of-core via HTTP 206 Partial Content and draw 1,000+ placements from 1 GPU buffer.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. HTTP 206 Range Streaming (Ch 12)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Fetch compact Manifest + World Bounds</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Prioritize visible frustum chunks</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• Range: bytes=start-end (bounded memory)</text>
    <text x="28" y="162" font-size="10.5" fill="#1e3a8a">• Evict cold off-screen chunks under</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">  memory budget without losing metadata</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="295" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. GPU Hardware Instancing (Ch 13)</text>
    <text x="307" y="108" font-size="10.5" fill="#7c2d12">• 1 Shared Definition Buffer (e.g. Bolt)</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#c2410c">• N Instance Rows (@builtin(instance_index)):</text>
    <text x="307" y="142" font-size="10" fill="#7c2d12">  Per-instance mat4x4 transform, tint,</text>
    <text x="307" y="157" font-size="10" fill="#7c2d12">  and distinct instance_owner_id</text>
    <text x="307" y="176" font-size="10.5" fill="#7c2d12">• draw_indexed(0..idx_cnt, 0, 0..N_inst)</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="592" y="68" width="228" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. 2D Drawing Sheets &amp; Blocks</text>
    <text x="604" y="108" font-size="10.5" fill="#064e3b">• Paper-space Sheet frames &amp; title</text>
    <text x="604" y="124" font-size="10.5" fill="#064e3b">  blocks (`SheetInit / SheetRows`)</text>
    <text x="604" y="144" font-size="10.5" font-weight="600" fill="#047857">• Calibrated mm/inch pen widths</text>
    <text x="604" y="162" font-size="10.5" fill="#064e3b">• Switch smoothly between 3D Model</text>
    <text x="604" y="178" font-size="10.5" fill="#064e3b">  view and 2D Sheet layouts</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
original table bytes → revision-bound entity → cancellable lookup.
```

- Describe the original sheet side-table layout and entity identity fields.
- **Why it works (Why retain the revision beside the record count?):** The table header and later range reads must describe the same source revision. A count from an older table cannot safely identify records in a newer file.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **HTTP / Fetch** | `Range: bytes=start-end` -> `206 Partial Content` | Fetches only the exact byte slice of an octree node or sheet column needed for the current camera view. |
| **Protobuf Wire** | Varint tag + length prefix scanner | Skips over multi-megabyte coordinate arrays by reading their byte range offsets without allocating their contents. |
| **Rust (Residency)** | `budget.reserve(bytes)?` -> LRU eviction | Guarantees resident + in-flight pages never exceed the memory cap; coarse root pages stay pinned so the scene is never blank. |
| **GPU Instancing** | Shared definition buffer + per-instance record | Draws N placed copies of a complex CAD component using 1 shared vertex/index allocation and N compact instance records. |

**Production source (`session_viewer/src`):** [app/scene_instances.rs](../../../../src/app/scene_instances.rs), [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs), [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs), [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs), [app/hierarchy.rs](../../../../src/app/hierarchy.rs), [app/sheet_query.rs](../../../../src/app/sheet_query.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-70za-read-original-sheet-metadata-records"></a>

<details>
<summary>Implementation 70za-read-original-sheet-metadata-records: Read original sheet metadata records</summary>

Describe the original sheet side-table layout and entity identity fields.

Data flow: original table bytes → revision-bound entity → cancellable lookup.

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

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 70za-read-original-sheet-metadata-records
npm --prefix ../session_tests run course -- check 70za-read-original-sheet-metadata-records
npm --prefix ../session_tests run course -- save 70za-read-original-sheet-metadata-records
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

The earlier drawing remains visible while the sheet metadata helpers are prepared.

- **Try this:** Trace one header, one record and one JSON entity. Compare missing identity keys with malformed bytes.
