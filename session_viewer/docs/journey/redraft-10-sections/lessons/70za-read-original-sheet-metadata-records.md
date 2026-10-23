# 13.4 · Original sheet entity metadata

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Lesson 13.4** (Redraft `7.8`) · **1 checkpoint (`70za-read-original-sheet-metadata-records`)** · **Study time: 0.5–1 h**

> **What You Will Learn & Build:** Describe the original sheet side-table layout and entity identity fields.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 13.4 Execution Flow: Original sheet entity metadata</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (Checkpoint: 70za-read-original-sheet-metadata-records)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="144" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="261" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="144" y="108" text-anchor="middle" font-size="11" fill="#111111">original table bytes</text>
    <line x1="271" y1="108" x2="291" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="293" y="68" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="303" y1="92" x2="536" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">revision-bound entity</text>
    <line x1="546" y1="108" x2="566" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="568" y="68" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="578" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="694" y="108" text-anchor="middle" font-size="11" fill="#111111">cancellable lookup</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 13.4:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/sheet_entity.rs  ·  src/lib.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">Checkpoint: 70za-read-original-sheet-metadata-records — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Section 7 WebGPU Streaming & Instancing: HTTP Range Requests, Shared Source Meshes & 2D Drawing Sheets</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How massive multi-gigabyte scenes stream out-of-core via HTTP 206 Partial Content and draw 1,000+ placements from 1 GPU buffer.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. HTTP 206 Range Streaming (Ch 12)</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Fetch compact Manifest + World Bounds</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Prioritize visible frustum chunks</text>
    <text x="28" y="144" font-size="10.5" fill="#111111">• Range: bytes=start-end (bounded memory)</text>
    <text x="28" y="162" font-size="10.5" fill="#111111">• Evict cold off-screen chunks under</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">  memory budget without losing metadata</text>

    <line x1="263" y1="138" x2="293" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="295" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="427" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. GPU Hardware Instancing (Ch 13)</text>
    <text x="307" y="108" font-size="10.5" fill="#111111">• 1 Shared Definition Buffer (e.g. Bolt)</text>
    <text x="307" y="126" font-size="10.5" font-weight="600" fill="#111111">• N Instance Rows (@builtin(instance_index)):</text>
    <text x="307" y="142" font-size="10" fill="#111111">  Per-instance mat4x4 transform, tint,</text>
    <text x="307" y="157" font-size="10" fill="#111111">  and distinct instance_owner_id</text>
    <text x="307" y="176" font-size="10.5" fill="#111111">• draw_indexed(0..idx_cnt, 0, 0..N_inst)</text>

    <line x1="560" y1="138" x2="590" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="592" y="68" width="228" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="706" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. 2D Drawing Sheets &amp; Blocks</text>
    <text x="604" y="108" font-size="10.5" fill="#111111">• Paper-space Sheet frames &amp; title</text>
    <text x="604" y="124" font-size="10.5" fill="#111111">  blocks (SheetInit / SheetRows)</text>
    <text x="604" y="144" font-size="10.5" font-weight="600" fill="#111111">• Calibrated mm/inch pen widths</text>
    <text x="604" y="162" font-size="10.5" fill="#111111">• Switch smoothly between 3D Model</text>
    <text x="604" y="178" font-size="10.5" fill="#111111">  view and 2D Sheet layouts</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **HTTP / Fetch** | `Range: bytes=start-end` -> `206 Partial Content` | Fetches only the exact byte slice of an octree node or sheet column needed for the current camera view. |
| **Protobuf Wire** | Varint tag + length prefix scanner | Skips over multi-megabyte coordinate arrays by reading their byte range offsets without allocating their contents. |
| **Rust (Residency)** | `budget.reserve(bytes)?` -> LRU eviction | Guarantees resident + in-flight pages never exceed the memory cap; coarse root pages stay pinned so the scene is never blank. |
| **GPU Instancing** | Shared definition buffer + per-instance record | Draws N placed copies of a complex CAD component using 1 shared vertex/index allocation and N compact instance records. |

**Authoritative production source (`session_viewer/src`):** [app/scene_instances.rs](../../../../src/app/scene_instances.rs), [engine/gpu/instanced.rs](../../../../src/engine/gpu/instanced.rs), [engine/gpu/slots.rs](../../../../src/engine/gpu/slots.rs), [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs), [app/hierarchy.rs](../../../../src/app/hierarchy.rs), [app/sheet_query.rs](../../../../src/app/sheet_query.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`70za-read-original-sheet-metadata-records`](#checkpoint-70za-read-original-sheet-metadata-records) | Describe the original sheet side-table layout and entity identity fields. | `src/sheet_entity.rs`, `src/lib.rs` | `original table bytes → revision-bound entity → cancellable lookup` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-70za-read-original-sheet-metadata-records"></a>

### Step 1 of 1 · Read original sheet metadata records (`70za-read-original-sheet-metadata-records`)

- **Goal:** Describe the original sheet side-table layout and entity identity fields.
- **Data Flow Trace:** `original table bytes → revision-bound entity → cancellable lookup`
- **Files You Touch:** `src/sheet_entity.rs` (*Type the whole original metadata or lookup helpers using their shared owner.*) · `src/lib.rs` (*Compile this shared original metadata module.*)

Describe the original sheet side-table layout and entity identity fields. Check the eight-byte header and sixteen-byte record before reading their little-endian integers. Parse the JSON entity with its original defaults and retain the table revision for the following lookup.

> **Self-Check Question:** *Why retain the revision beside the record count?*  
> **Answer:** The table header and later range reads must describe the same source revision. A count from an older table cannot safely identify records in a newer file.

- **What You Should See:** The earlier drawing remains visible while the sheet metadata helpers are prepared.
- **Hands-On Lab Experiment:** Trace one header, one record and one JSON entity. Compare missing identity keys with malformed bytes.

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

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 70za-read-original-sheet-metadata-records
npm --prefix ../session_tests run course -- check 70za-read-original-sheet-metadata-records
npm --prefix ../session_tests run course -- save 70za-read-original-sheet-metadata-records
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

The earlier drawing remains visible while the sheet metadata helpers are prepared.

- **Final Lesson Experiment:** Trace one header, one record and one JSON entity. Compare missing identity keys with malformed bytes.
