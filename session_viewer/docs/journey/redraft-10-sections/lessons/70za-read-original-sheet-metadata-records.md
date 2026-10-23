# 13.4 · Original sheet entity metadata

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Lesson 13.4** (Redraft `7.8`) · **1 checkpoint (`70za-read-original-sheet-metadata-records`)** · **Study time: 0.5–1 h**

> **What You Will Learn & Build:** Describe the original sheet side-table layout and entity identity fields.

## 1. Visual WebGPU Explanation & Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="256" viewBox="0 0 840 256" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="248" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 7 Visual Explanation: HTTP 206 Range Streaming, GPU Hardware Instancing & 2D Drawing Sheets</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Fetching frustum-prioritized byte ranges. Center: 1 mesh buffer × N instance transforms. Right: Paper-space sheets.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="245" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. HTTP 206 Byte-Range Stream</text>
    <rect x="34" y="98" width="212" height="30" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <rect x="74" y="98" width="60" height="30" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
    <rect x="164" y="98" width="48" height="30" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="144" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Range: bytes=start-end (Visible First)</text>
    <text x="140" y="164" text-anchor="middle" font-size="9.5" fill="#111111">• Bounded Permit Pool &amp; Page Cache</text>
    <text x="140" y="182" text-anchor="middle" font-size="9.5" fill="#111111">• Evicts cold off-screen chunks safely</text>

    <line x1="263" y1="154" x2="285" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="287" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="424" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. GPU Hardware Instancing (1 → N)</text>
    <rect x="305" y="114" width="68" height="48" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
    <text x="339" y="142" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">1 Source</text>
    <line x1="373" y1="126" x2="425" y2="106" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <line x1="373" y1="138" x2="425" y2="138" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <line x1="373" y1="150" x2="425" y2="170" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <rect x="428" y="92" width="118" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="487" y="108" text-anchor="middle" font-size="9" fill="#111111">Instance #0 (M0, ID0)</text>
    <rect x="428" y="126" width="118" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="487" y="142" text-anchor="middle" font-size="9" fill="#111111">Instance #1 (M1, ID1)</text>
    <rect x="428" y="160" width="118" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="487" y="176" text-anchor="middle" font-size="9" fill="#111111">Instance #N (MN, IDN)</text>
    <text x="424" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">@builtin(instance_index) draw_indexed</text>

    <line x1="562" y1="154" x2="584" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="586" y="64" width="236" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. 2D Drawing Sheet &amp; Title Block</text>
    <rect x="612" y="96" width="184" height="104" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
    <rect x="716" y="168" width="80" height="32" fill="url(#hatch)" stroke="#111111" stroke-width="1.2"/>
    <text x="756" y="188" text-anchor="middle" font-size="8.5" font-weight="700" fill="#111111">Title Block</text>
    <text x="704" y="220" text-anchor="middle" font-size="9.5" fill="#111111">Paper-space mm/inch pen widths</text>
    
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="214" viewBox="0 0 840 214" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="206" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 13.4 Pipeline Schematic: Original sheet entity metadata</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (Checkpoint: 70za-read-original-sheet-metadata-records)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="144" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="261" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="144" y="108" text-anchor="middle" font-size="11" fill="#111111">original table bytes</text>
    <line x1="271" y1="108" x2="291" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="293" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="303" y1="90" x2="536" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">revision-bound entity</text>
    <line x1="546" y1="108" x2="566" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="568" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="578" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="694" y="108" text-anchor="middle" font-size="11" fill="#111111">cancellable lookup</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 13.4:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/sheet_entity.rs  ·  src/lib.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">Checkpoint: 70za-read-original-sheet-metadata-records — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-70za-read-original-sheet-metadata-records"></a>

### Step 1 of 1 · Read original sheet metadata records (`70za-read-original-sheet-metadata-records`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/1 Visual Trace (70za-read-original-sheet-metadata-records): Read original sheet metadata records</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/sheet_entity.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original table bytes</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">revision-bound entity</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">cancellable lookup</text>
    
</svg>
</div>

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
