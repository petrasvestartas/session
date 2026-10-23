# 12.2 · Locate original arrays without downloading their bodies

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Lesson 12.2** (Redraft `7.2`) · **10 checkpoints (`67-supply-checked-array-layout-readers` → `67si-validate-complete-octrees-before-range-streaming`)** · **Study time: 5–10 h**

> **What You Will Learn & Build:** Bound scalar headers and byte addresses. By the final checkpoint (`67si-validate-complete-octrees-before-range-streaming`), you will validate complete octrees before range streaming.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 12.2 Pipeline Schematic: Locate original arrays without downloading their bodies</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (10 Checkpoints: 67-supply-checked-array-layout-readers → 67si-validate-complete-octrees-before-range-streaming)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="192" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="110" y="108" text-anchor="middle" font-size="11" fill="#111111">original file bytes</text>
    <line x1="202" y1="108" x2="222" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="224" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="234" y1="90" x2="398" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="316" y="108" text-anchor="middle" font-size="11" fill="#111111">checked source layout</text>
    <line x1="408" y1="108" x2="428" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="430" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="522" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="440" y1="90" x2="604" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="522" y="108" text-anchor="middle" font-size="11" fill="#111111">existing GPU array owners</text>
    <line x1="614" y1="108" x2="634" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="636" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="728" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="646" y1="90" x2="810" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="728" y="108" text-anchor="middle" font-size="11" fill="#111111">later range streaming</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 12.2:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/stream.rs  ·  src/lib.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">10 Checkpoints: 67-supply-checked-array-layout-readers → 67si-validate-complete-octrees-before-range-streaming — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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

**Authoritative production source (`session_viewer/src`):** [app/manifest.rs](../../../../src/app/manifest.rs), [app/session_io.rs](../../../../src/app/session_io.rs), [app/stream.rs](../../../../src/app/stream.rs), [app/range_gate.rs](../../../../src/app/range_gate.rs), [app/fetch.rs](../../../../src/app/fetch.rs), [app/loader.rs](../../../../src/app/loader.rs), [app/live.rs](../../../../src/app/live.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`67-supply-checked-array-layout-readers`](#checkpoint-67-supply-checked-array-layout-readers) | Bound scalar headers and byte addresses. | `src/stream.rs`, `src/lib.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **2** | [`67sa-locate-original-cloud-and-sheet-array-fields`](#checkpoint-67sa-locate-original-cloud-and-sheet-array-fields) | Locate original cloud and sheet array fields. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **3** | [`67sb-convert-bounded-doubles-and-cache-metadata-windows`](#checkpoint-67sb-convert-bounded-doubles-and-cache-metadata-windows) | Convert bounded doubles and cache metadata windows. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **4** | [`67sc-decode-packed-arrays-for-existing-gpu-owners`](#checkpoint-67sc-decode-packed-arrays-for-existing-gpu-owners) | Decode packed arrays for existing GPU owners. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **5** | [`67sd-descend-bounded-original-protobuf-messages`](#checkpoint-67sd-descend-bounded-original-protobuf-messages) | Descend bounded original protobuf messages. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **6** | [`67se-find-coordinate-arrays-without-downloading-their-bodies`](#checkpoint-67se-find-coordinate-arrays-without-downloading-their-bodies) | Find coordinate arrays without downloading their bodies. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **7** | [`67sf-validate-sheet-side-array-lengths`](#checkpoint-67sf-validate-sheet-side-array-lengths) | Validate sheet side-array lengths. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **8** | [`67sg-collect-original-octree-node-columns`](#checkpoint-67sg-collect-original-octree-node-columns) | Collect original octree node columns. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **9** | [`67sh-check-octree-bounds-and-unique-child-ownership`](#checkpoint-67sh-check-octree-bounds-and-unique-child-ownership) | Check octree bounds and unique child ownership. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |
| **10** | [`67si-validate-complete-octrees-before-range-streaming`](#checkpoint-67si-validate-complete-octrees-before-range-streaming) | Validate complete octrees before range streaming. | `src/stream.rs` | `original file bytes → checked source layout → existing GPU array owners → later range streaming` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-67-supply-checked-array-layout-readers"></a>

### Step 1 of 10 · Bound scalar headers and byte addresses (`67-supply-checked-array-layout-readers`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/10 Visual Trace (67-supply-checked-array-layout-readers): Bound scalar headers and byte addresses</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Bound scalar headers and byte addresses.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Bound scalar headers and byte addresses.*) · `src/lib.rs` (*Register the source component.*)

Reuse the varint decoder from 66b. A protobuf scalar has a wire-dependent byte length; byte addresses stay u64 until a bounded slice requires usize. checked_add must precede the end comparison. This is the boundary between an external file layout and later GPU-ready arrays.

> **Self-Check Question:** *Why check addition before comparing the message end?*  
> **Answer:** An overflowing byte address could wrap into a small, apparently valid offset.

- **What You Should See:** The cumulative browser application compiles. This parser preparation preserves the current drawing; actual range requests and streamed pages connect in the following lessons.
- **Hands-On Lab Experiment:** Trace a body at u64::MAX-2 with length 4; predict the rejection before evaluating the end bound.

<details>
<summary>Implementation 67-supply-checked-array-layout-readers: Bound scalar headers and byte addresses</summary>

Bound scalar headers and byte addresses.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Bound scalar headers and byte addresses.

Create the file and type:

```rust
--8<-- "journey/code/67-supply-checked-array-layout-readers-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
pub mod browser_publication;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67-supply-checked-array-layout-readers-02.rs" skip=60
```

</details>

<a id="checkpoint-67sa-locate-original-cloud-and-sheet-array-fields"></a>

### Step 2 of 10 · Locate original cloud and sheet array fields (`67sa-locate-original-cloud-and-sheet-array-fields`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/10 Visual Trace (67sa-locate-original-cloud-and-sheet-array-fields): Locate original cloud and sheet array fields</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Locate original cloud and sheet array fields.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Locate original cloud and sheet array fields.*)

CloudFields and SheetFields retain offsets into the original file rather than owned coordinate arrays.

Field records the parsed tag, body start and next address. Parse scalar fields with the helper from the preceding lesson; length-delimited bodies require a second checked address addition. ETags remain attached to the original layout so later ranges can reject mixed revisions.

> **Self-Check Question:** *Does a field body offset point at its length prefix?*  
> **Answer:** No. The body begins after both the tag and the length varint.

- **Hands-On Lab Experiment:** Draw the tag, length prefix and body for a three-byte packed array. Mark body and next separately.

<details>
<summary>Implementation 67sa-locate-original-cloud-and-sheet-array-fields: Locate original cloud and sheet array fields</summary>

Locate original cloud and sheet array fields.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Locate original cloud and sheet array fields.

<details>
<summary>Locate the existing block</summary>

```rust
fn skip_scalar(b: &[u8], i: usize, wire: u32) -> Option<usize> {
    match wire {
        0 => Some(varint(b, i)?.1),
        1 => Some(8),
        5 => Some(4),
        _ => None,
    }
}

/// True when the value fits an f32.
fn finite_float(value: f64) -> bool {
    value.is_finite() && (value as f32).is_finite()
}

/// `at + length` when it stays within `end`.
fn body_end(at: u64, length: u64, end: u64) -> Option<u64> {
    let next = at.checked_add(length)?;

    if next <= end { Some(next) } else { None }
}
```

</details>

After that block, type:

```rust

/// Byte positions of a cloud's arrays in its file.
#[derive(Clone, Debug)]
pub struct CloudFields {
    pub end: u64,                 // end of the cloud message
    pub coords_at: u64,           // start of the positions
    pub coords_len: u64,          // their length
    pub colors_at: u64,           // start of the colours
    pub colors_len: u64,          // their length
    pub normals_at: u64,          // start of the normals, 0 = none
    pub normals_len: u64,         // their length
    pub count: u32,               // points in the cloud
    pub ids_at: u64,              // start of the original ids, 0 = none
    pub ids_len: u64,             // their length
    pub revision: Option<String>, // file ETag every read must match
}

/// Byte positions of a sheet's arrays in its file; length 0 = absent.
#[derive(Clone, Debug, Default)]
pub struct SheetFields {
    pub end: u64,                 // end of the sheet message
    pub coords_at: u64,           // start of the segment ends
    pub coords_len: u64,          // their length
    pub colors_at: u64,           // start of the colours
    pub colors_len: u64,          // their length
    pub widths_at: u64,           // start of the pen widths
    pub widths_len: u64,          // their length
    pub ids_at: u64,              // start of the entity ids
    pub ids_len: u64,             // their length
    pub count: u32,               // segments in the sheet
    pub entities: u32,            // records in the side table
    pub meta: String,             // side table file name, empty = none
    pub revision: Option<String>, // file ETag every read must match
}

/// One protobuf field header.
#[derive(Clone, Copy)]
pub struct Field {
    pub field: u32, // field number
    pub wire: u32,  // wire type
    pub value: u64, // the varint value, or the body length
    pub body: u64,  // where the body starts
    pub next: u64,  // where the next field starts
}

/// Parse the field header at `at`; None when it runs past `end`.
pub fn field_at(header: &[u8], at: u64, end: u64) -> Option<Field> {
    let (tag, used) = varint(header, 0)?;
    let (field, wire) = (u32::try_from(tag >> 3).ok()?, (tag & 7) as u32);

    if field == 0 {
        return None;
    }

    let body = body_end(at, used as u64, end)?;

    if wire != 2 {
        let (value, _) = if wire == 0 {
            varint(header, used)?
        } else {
            (0, 0)
        };
        let skip = skip_scalar(header, used, wire)?;
        return Some(Field {
            field,
            wire,
            value,
            body,
            next: body_end(body, skip as u64, end)?,
        });
    }

    let (length, extra) = varint(header, used)?;
    let body = body_end(body, extra as u64, end)?;
    Some(Field {
        field,
        wire,
        value: length,
        body,
        next: body_end(body, length, end)?,
    })
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sa-locate-original-cloud-and-sheet-array-fields-01.rs"
```

</details>

</details>

<a id="checkpoint-67sb-convert-bounded-doubles-and-cache-metadata-windows"></a>

### Step 3 of 10 · Convert bounded doubles and cache metadata windows (`67sb-convert-bounded-doubles-and-cache-metadata-windows`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/10 Visual Trace (67sb-convert-bounded-doubles-and-cache-metadata-windows): Convert bounded doubles and cache metadata windows</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Convert bounded doubles and cache metadata windows.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Convert bounded doubles and cache metadata windows.*)

Keep original source precision separate from the f32 upload representation.

Require the exact declared byte count before converting doubles, then reject non-finite values in either representation. MetadataWindow serves a borrowed slice only when the complete range is cached. Its read length batches nearby small fields within 64 KiB and the enclosing message; it does not allocate a geometry-sized header.

> **Self-Check Question:** *Why reject a finite f64 that becomes infinite as f32?*  
> **Answer:** The source value is valid as f64 but cannot be represented in the GPU coordinate format.

- **Hands-On Lab Experiment:** Compare a request inside a cached window with one crossing its end. Trace a finite 1 e 300 source coordinate through the f32 check.

<details>
<summary>Implementation 67sb-convert-bounded-doubles-and-cache-metadata-windows: Convert bounded doubles and cache metadata windows</summary>

Convert bounded doubles and cache metadata windows.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Convert bounded doubles and cache metadata windows.

<details>
<summary>Locate the existing block</summary>

```rust
        } else {
            (0, 0)
        };
        let skip = skip_scalar(header, used, wire)?;
        return Some(Field {
            field,
            wire,
            value,
            body,
            next: body_end(body, skip as u64, end)?,
        });
    }

    let (length, extra) = varint(header, used)?;
    let body = body_end(body, extra as u64, end)?;
    Some(Field {
        field,
        wire,
        value: length,
        body,
        next: body_end(body, length, end)?,
    })
}
```

</details>

After that block, type:

```rust

/// `count` xyz triples as f32; None when short or not finite.
#[cfg(any(target_arch = "wasm32", test))]
fn checked_positions(raw: &[u8], count: u32) -> Option<Vec<f32>> {
    checked_doubles(raw, u64::from(count).checked_mul(3)?)
}

/// Exactly `n` doubles as f32; None when short or not finite.
#[cfg(any(target_arch = "wasm32", test))]
fn checked_doubles(raw: &[u8], n: u64) -> Option<Vec<f32>> {
    if raw.len() as u64 != n.checked_mul(8)? {
        return None;
    }

    let mut out = Vec::with_capacity(raw.len() / 8);

    for bytes in raw.chunks_exact(8) {
        let value = f64::from_le_bytes(bytes.try_into().ok()?);

        if !finite_float(value) {
            return None;
        }

        out.push(value as f32);
    }

    Some(out)
}

/// True when a range is small enough to read.
#[cfg(any(target_arch = "wasm32", test))]
fn bounded_range(at: u64, length: u64) -> bool {
    length <= 64 * 1024 * 1024 && at.checked_add(length).is_some()
}

/// A cached slice of the file's header bytes.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Default)]
struct MetadataWindow {
    at: u64,        // file position of `bytes[0]`
    bytes: Vec<u8>, // the cached bytes
}

#[cfg(any(target_arch = "wasm32", test))]
impl MetadataWindow {
    /// The cached bytes at `at`, if all present.
    fn slice(&self, at: u64, length: u64) -> Option<&[u8]> {
        let start = usize::try_from(at.checked_sub(self.at)?).ok()?;
        let length = usize::try_from(length).ok()?;
        self.bytes.get(start..start.checked_add(length)?)
    }

    /// How much to read at once: at least `length`, up to 64 KiB.
    fn read_length(at: u64, length: u64, end: u64) -> Option<u64> {
        if !bounded_range(at, length) {
            return None;
        }

        body_end(at, length, end)?;
        Some(length.max(64 * 1024).min(end - at))
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sb-convert-bounded-doubles-and-cache-metadata-windows-01.rs"
```

</details>

</details>

<a id="checkpoint-67sc-decode-packed-arrays-for-existing-gpu-owners"></a>

### Step 4 of 10 · Decode packed arrays for existing GPU owners (`67sc-decode-packed-arrays-for-existing-gpu-owners`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/10 Visual Trace (67sc-decode-packed-arrays-for-existing-gpu-owners): Decode packed arrays for existing GPU owners</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Decode packed arrays for existing GPU owners.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Decode packed arrays for existing GPU owners.*)

Read fixed-width arrays with explicit little-endian conversion and varints with the shared decoder.

These low-level readers are used after layout and length validation; they are not independent whole-file validators. Streamed positions become f32 upload triples while original bytes remain in the source. Normals reuse cloud_normals::oct16, and colours pack four byte channels into one u32.

> **Self-Check Question:** *Does decoding normals require another normal encoding algorithm?*  
> **Answer:** No. Reuse the octahedral encoder already written for the cloud owner.

- **Hands-On Lab Experiment:** Decode little-endian bytes for 1.0 and predict the packed value of RGBA[255,0,0,255]. Compare a zero normal with a valid axis normal.

<details>
<summary>Implementation 67sc-decode-packed-arrays-for-existing-gpu-owners: Decode packed arrays for existing GPU owners</summary>

Decode packed arrays for existing GPU owners.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Decode packed arrays for existing GPU owners.

<details>
<summary>Locate the existing block</summary>

```rust
    fn slice(&self, at: u64, length: u64) -> Option<&[u8]> {
        let start = usize::try_from(at.checked_sub(self.at)?).ok()?;
        let length = usize::try_from(length).ok()?;
        self.bytes.get(start..start.checked_add(length)?)
    }

    /// How much to read at once: at least `length`, up to 64 KiB.
    fn read_length(at: u64, length: u64, end: u64) -> Option<u64> {
        if !bounded_range(at, length) {
            return None;
        }

        body_end(at, length, end)?;
        Some(length.max(64 * 1024).min(end - at))
    }
}
```

</details>

After that block, type:

```rust

/// A packed `int32` (varint) array in full.
pub fn packed_i32(raw: &[u8]) -> Vec<i32> {
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < raw.len() {
        let Some((v, n)) = varint(raw, i) else { break };
        out.push(v as i32);
        i += n;
    }

    out
}

/// A packed `double` array in full.
pub fn packed_f64(raw: &[u8]) -> Vec<f64> {
    let mut out = Vec::with_capacity(raw.len() / 8);

    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()));
    }

    out
}

/// A packed `fixed32` array in full.
pub fn packed_u32(raw: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity(raw.len() / 4);

    for c in raw.chunks_exact(4) {
        out.push(u32::from_le_bytes(c.try_into().unwrap()));
    }

    out
}

/// A packed `float` array in full; a non-finite value becomes 0.
pub fn packed_f32(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 4);

    for c in raw.chunks_exact(4) {
        let v = f32::from_le_bytes(c.try_into().unwrap());
        out.push(if v.is_finite() { v } else { 0.0 });
    }

    out
}

/// An already-fetched coords slice as f32 triples.
pub fn positions_from(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 8);

    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()) as f32);
    }

    out
}

/// Packed doubles, three per point, as octahedral normals; a degenerate one becomes 0.
pub fn normals_from(raw: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity(raw.len() / 24);

    for c in raw.chunks_exact(24) {
        let n = [
            f64::from_le_bytes(c[0..8].try_into().unwrap()),
            f64::from_le_bytes(c[8..16].try_into().unwrap()),
            f64::from_le_bytes(c[16..24].try_into().unwrap()),
        ];
        out.push(crate::cloud_normals::oct16(&n).unwrap_or(0));
    }

    out
}

/// `count` colours from packed varints, and where they ended.
pub fn colors_from(raw: &[u8], count: u32) -> Option<(Vec<u32>, usize)> {
    let mut out = Vec::with_capacity(count as usize);
    let mut i = 0usize;

    for _ in 0..count {
        let mut rgba = [255u8; 4];

        for k in &mut rgba {
            let (v, n) = varint(raw, i)?;
            i += n;
            *k = (v & 255) as u8;
        }

        out.push(u32::from_le_bytes(rgba));
    }

    Some((out, i))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sc-decode-packed-arrays-for-existing-gpu-owners-01.rs"
```

</details>

</details>

<a id="checkpoint-67sd-descend-bounded-original-protobuf-messages"></a>

### Step 5 of 10 · Descend bounded original protobuf messages (`67sd-descend-bounded-original-protobuf-messages`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/10 Visual Trace (67sd-descend-bounded-original-protobuf-messages): Descend bounded original protobuf messages</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Descend bounded original protobuf messages.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Descend bounded original protobuf messages.*)

Walk nested length-delimited messages using parent-end constraints.

Small names can precede the target; a larger skipped body stops this prefix-only fast path. Reaching a target nested message requires its end to agree with the declared parent end. Failure means this specialized layout is unavailable, not that arbitrary source data can be accepted.

> **Self-Check Question:** *Why refuse a large field before the expected geometry message?*  
> **Answer:** A bounded prefix reader cannot skip an arbitrary geometry body while claiming it has inspected the layout.

- **Hands-On Lab Experiment:** Trace a nested message whose declared end exceeds its parent, then a small name before the expected field.

<details>
<summary>Implementation 67sd-descend-bounded-original-protobuf-messages: Descend bounded original protobuf messages</summary>

Descend bounded original protobuf messages.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Descend bounded original protobuf messages.

<details>
<summary>Locate the existing block</summary>

```rust
    }

    out
}

/// `count` colours from packed varints, and where they ended.
pub fn colors_from(raw: &[u8], count: u32) -> Option<(Vec<u32>, usize)> {
    let mut out = Vec::with_capacity(count as usize);
    let mut i = 0usize;

    for _ in 0..count {
        let mut rgba = [255u8; 4];

        for k in &mut rgba {
            let (v, n) = varint(raw, i)?;
            i += n;
            *k = (v & 255) as u8;
        }

        out.push(u32::from_le_bytes(rgba));
    }

    Some((out, i))
}
```

</details>

After that block, type:

```rust

/// A skipped field longer than this is geometry, not a name.
const NAME_BYTES: u64 = 256;

/// Enter field `want` of the message at `at`; returns its end.
fn descend_message(head: &[u8], at: &mut usize, parent_end: Option<u64>, want: u32) -> Option<u64> {
    loop {
        let (tag, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        let (field, wire) = (u32::try_from(tag >> 3).ok()?, tag & 7);

        if field == 0 || wire != 2 {
            return None;
        }

        let (length, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        let next = (*at as u64).checked_add(length)?;

        if let Some(end) = parent_end
            && next > end
        {
            return None;
        }

        if field == want {
            if let Some(end) = parent_end
                && end != next
            {
                return None;
            }

            return Some(next);
        }

        if length > NAME_BYTES {
            return None;
        }

        *at = usize::try_from(next).ok()?;
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sd-descend-bounded-original-protobuf-messages-01.rs"
```

</details>

</details>

<a id="checkpoint-67se-find-coordinate-arrays-without-downloading-their-bodies"></a>

### Step 6 of 10 · Find coordinate arrays without downloading their bodies (`67se-find-coordinate-arrays-without-downloading-their-bodies`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/10 Visual Trace (67se-find-coordinate-arrays-without-downloading-their-bodies): Find coordinate arrays without downloading their bodies</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Find coordinate arrays without downloading their bodies.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Find coordinate arrays without downloading their bodies.*)

After entering the cloud or sheet message, scan only the permitted leading fields.

The coordinates field supplies its source offset and length without reading the entire body. cloud_layout and sheet_layout use the same scanner; plain chooses the whole-file path when neither bounded specialization matches. A truncated varint cannot produce a guessed offset.

> **Self-Check Question:** *How can a short prefix locate a much larger coordinate array?*  
> **Answer:** The field header declares the array length; its body does not need to be present in the prefix.

- **Hands-On Lab Experiment:** Sketch a short prefix declaring a megabyte coordinate body. Identify which bytes must be downloaded now and which remain for later range requests.

<details>
<summary>Implementation 67se-find-coordinate-arrays-without-downloading-their-bodies: Find coordinate arrays without downloading their bodies</summary>

Find coordinate arrays without downloading their bodies.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Find coordinate arrays without downloading their bodies.

<details>
<summary>Locate the existing block</summary>

```rust
        *at = (*at).checked_add(used)?;
        let next = (*at as u64).checked_add(length)?;

        if let Some(end) = parent_end
            && next > end
        {
            return None;
        }

        if field == want {
            if let Some(end) = parent_end
                && end != next
            {
                return None;
            }

            return Some(next);
        }

        if length > NAME_BYTES {
            return None;
        }

        *at = usize::try_from(next).ok()?;
    }
}
```

</details>

After that block, type:

```rust

/// The `coords` field, which only small names may precede.
fn first_array(head: &[u8], mut at: usize, end: u64) -> Option<(u64, u64, u64)> {
    while (at as u64) < end {
        let (tag, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        let (field, wire) = (u32::try_from(tag >> 3).ok()?, (tag & 7) as u32);

        if field == 0 {
            return None;
        }

        if wire != 2 {
            at = at.checked_add(skip_scalar(head, at, wire)?)?;

            if at as u64 > end {
                return None;
            }

            continue;
        }

        let (length, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        let next = (at as u64).checked_add(length)?;

        if next > end {
            return None;
        }

        if field == 3 {
            return Some((at as u64, length, end));
        }

        if field > 3 || length > NAME_BYTES {
            return None;
        }

        at = usize::try_from(next).ok()?;
    }

    None
}

/// Start and length of a single cloud's `coords`; None otherwise.
pub fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}

/// Start, length of `coords` and end of the cloud message.
fn cloud_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 8)?;
    first_array(head, at, end)
}

/// Start, length of `coords` and end of the sheet message.
pub fn sheet_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 17)?;
    first_array(head, at, end)
}

/// True for a file whose first bytes show neither a cloud nor a sheet: it loads whole.
pub fn plain(head: &[u8]) -> bool {
    cloud_layout(head).is_none() && sheet_layout(head).is_none()
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67se-find-coordinate-arrays-without-downloading-their-bodies-01.rs"
```

</details>

</details>

<a id="checkpoint-67sf-validate-sheet-side-array-lengths"></a>

### Step 7 of 10 · Validate sheet side-array lengths (`67sf-validate-sheet-side-array-lengths`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 7/10 Visual Trace (67sf-validate-sheet-side-array-lengths): Validate sheet side-array lengths</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Validate sheet side-array lengths.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Validate sheet side-array lengths.*)

SheetFields::set records each optional side array once.

Colours, widths and IDs must match the declared segment count, while metadata names remain bounded UTF-8. Reject duplicate known fields and wrong wire types; skip unknown fields to preserve forward compatibility. Actual sheet GPU drawing is connected at 71, using these original entity IDs.

> **Self-Check Question:** *Why must a colour field match the segment count?*  
> **Answer:** Its fixed32 values index the same original segments as the coordinate array.

- **Hands-On Lab Experiment:** For two segments, predict the accepted colour byte count. Try a duplicate colour field and an invalid UTF-8 metadata name.

<details>
<summary>Implementation 67sf-validate-sheet-side-array-lengths: Validate sheet side-array lengths</summary>

Validate sheet side-array lengths.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Validate sheet side-array lengths.

<details>
<summary>Locate the existing block</summary>

```rust
/// Start, length of `coords` and end of the sheet message.
pub fn sheet_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 17)?;
    first_array(head, at, end)
}

/// True for a file whose first bytes show neither a cloud nor a sheet: it loads whole.
pub fn plain(head: &[u8]) -> bool {
    cloud_layout(head).is_none() && sheet_layout(head).is_none()
}
```

</details>

After that block, type:

```rust


impl SheetFields {
    /// Bytes per segment: six doubles.
    pub const SEGMENT_BYTES: u64 = 48;

    /// Record one field found after `coords`; false when it is wrong.
    pub fn set(&mut self, f: &Field, body: &[u8]) -> bool {
        let per_segment = u64::from(self.count) * 4;

        match (f.field, f.wire) {
            (4, 2) if self.colors_len == 0 && f.value == per_segment => {
                (self.colors_at, self.colors_len) = (f.body, f.value)
            }
            (5, 2) if self.widths_len == 0 && f.value == per_segment => {
                (self.widths_at, self.widths_len) = (f.body, f.value)
            }
            (15, 2) if self.ids_len == 0 && f.value == per_segment => {
                (self.ids_at, self.ids_len) = (f.body, f.value)
            }
            (6, 0) => return f.value == u64::from(self.count),
            (7, 0) => match u32::try_from(f.value) {
                Ok(entities) => self.entities = entities,
                Err(_) => return false,
            },
            (8, 2) if f.value <= NAME_BYTES => match std::str::from_utf8(body) {
                Ok(meta) => self.meta = meta.to_string(),
                Err(_) => return false,
            },
            (1..=8, _) | (15, _) => return false,
            _ => {}
        }

        true
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sf-validate-sheet-side-array-lengths-01.rs"
```

</details>

</details>

<a id="checkpoint-67sg-collect-original-octree-node-columns"></a>

### Step 8 of 10 · Collect original octree node columns (`67sg-collect-original-octree-node-columns`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 8/10 Visual Trace (67sg-collect-original-octree-node-columns): Collect original octree node columns</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Collect original octree node columns.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Collect original octree node columns.*)

Store the original octree as parallel columns.

Validate fixed-width alignment and every varint before adopting a field, then use the already-written packed readers. set_field only fills columns; it does not establish topology. The next two lessons verify node bounds and connected parent relationships before streamed level selection uses them.

> **Self-Check Question:** *Why validate a packed varint column before decoding it?*  
> **Answer:** The permissive packed reader can stop at malformed input; the column must first be known complete.

- **Hands-On Lab Experiment:** List the seven node columns. Explain why a truncated final varint must reject the column rather than silently drop a node.

<details>
<summary>Implementation 67sg-collect-original-octree-node-columns: Collect original octree node columns</summary>

Collect original octree node columns.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Collect original octree node columns.

<details>
<summary>Locate the existing block</summary>

```rust
            (6, 0) => return f.value == u64::from(self.count),
            (7, 0) => match u32::try_from(f.value) {
                Ok(entities) => self.entities = entities,
                Err(_) => return false,
            },
            (8, 2) if f.value <= NAME_BYTES => match std::str::from_utf8(body) {
                Ok(meta) => self.meta = meta.to_string(),
                Err(_) => return false,
            },
            (1..=8, _) | (15, _) => return false,
            _ => {}
        }

        true
    }
}
```

</details>

After that block, type:

```rust

/// A cloud's octree node table.
#[derive(Clone, Default)]
pub struct CloudLod {
    pub min: Vec<f64>,      // cube corner, three per node
    pub size: Vec<f64>,     // cube size per node
    pub spacing: Vec<f64>,  // point spacing per node
    pub level: Vec<i32>,    // depth per node
    pub first: Vec<i32>,    // first point per node
    pub count: Vec<i32>,    // points per node
    pub children: Vec<i32>, // eight child indices per node, -1 = none
}

impl CloudLod {
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.size.len()
    }

    /// Fill one array from a packed field; false when unknown.
    pub fn set_field(&mut self, field: usize, raw: &[u8]) -> bool {
        if (8..=10).contains(&field) && !raw.len().is_multiple_of(8) {
            return false;
        }

        if (11..=14).contains(&field) {
            let mut at = 0;

            while at < raw.len() {
                let Some((_, bytes)) = varint(raw, at) else {
                    return false;
                };
                at += bytes;
            }
        }

        match field {
            8 => self.min = packed_f64(raw),
            9 => self.size = packed_f64(raw),
            10 => self.spacing = packed_f64(raw),
            11 => self.level = packed_i32(raw),
            12 => self.first = packed_i32(raw),
            13 => self.count = packed_i32(raw),
            14 => self.children = packed_i32(raw),
            _ => return false,
        }

        true
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sg-collect-original-octree-node-columns-01.rs"
```

</details>

</details>

<a id="checkpoint-67sh-check-octree-bounds-and-unique-child-ownership"></a>

### Step 9 of 10 · Check octree bounds and unique child ownership (`67sh-check-octree-bounds-and-unique-child-ownership`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 9/10 Visual Trace (67sh-check-octree-bounds-and-unique-child-ownership): Check octree bounds and unique child ownership</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Check octree bounds and unique child ownership.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Check octree bounds and unique child ownership.*)

A node must have representable finite bounds, nonnegative size and spacing, and a point interval inside the original cloud.

Child indices must be valid, unclaimed and exactly one level below their parent. Strictly increasing levels prevent cycles; the full-table check in the next lesson also rejects detached nodes. These checks protect later GPU page selection without rewriting original topology.

> **Self-Check Question:** *Can two octree parents claim the same child?*  
> **Answer:** No. The shared parent bitmap rejects a second claim and the root cannot be a child.

- **Hands-On Lab Experiment:** Trace a duplicate child claim and a child with the wrong level. Compare an interval ending exactly at the cloud count with one ending past it.

<details>
<summary>Implementation 67sh-check-octree-bounds-and-unique-child-ownership: Check octree bounds and unique child ownership</summary>

Check octree bounds and unique child ownership.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Check octree bounds and unique child ownership.

<details>
<summary>Locate the existing block</summary>

```rust
                    return false;
                };
                at += bytes;
            }
        }

        match field {
            8 => self.min = packed_f64(raw),
            9 => self.size = packed_f64(raw),
            10 => self.spacing = packed_f64(raw),
            11 => self.level = packed_i32(raw),
            12 => self.first = packed_i32(raw),
            13 => self.count = packed_i32(raw),
            14 => self.children = packed_i32(raw),
            _ => return false,
        }

        true
    }
}
```

</details>

After that block, type:

```rust

impl CloudLod {
    /// True when one node's values are sound.
    fn valid_node(&self, node: usize, total: u32) -> bool {
        let size = self.size[node];
        let spacing = self.spacing[node];

        if !finite_float(size)
            || size < 0.0
            || !finite_float(spacing)
            || spacing < 0.0
            || self.level[node] < 0
        {
            return false;
        }

        for &min in &self.min[node * 3..node * 3 + 3] {
            if !finite_float(min) || !finite_float(min + size) {
                return false;
            }
        }

        let (Ok(first), Ok(count)) = (
            u32::try_from(self.first[node]),
            u32::try_from(self.count[node]),
        ) else {
            return false;
        };
        let Some(end) = first.checked_add(count) else {
            return false;
        };
        end <= total
    }

    /// True when a node's children are one level down and unclaimed.
    fn valid_children(&self, node: usize, parents: &mut [bool]) -> bool {
        for &child in &self.children[node * 8..node * 8 + 8] {
            if child == -1 {
                continue;
            }

            let Ok(child) = usize::try_from(child) else {
                return false;
            };

            if child == 0
                || child >= parents.len()
                || parents[child]
                || self.level[node].checked_add(1) != Some(self.level[child])
            {
                return false;
            }

            parents[child] = true;
        }

        true
    }

    /// True when the file carried no octree.
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67sh-check-octree-bounds-and-unique-child-ownership-01.rs"
```

</details>

</details>

<a id="checkpoint-67si-validate-complete-octrees-before-range-streaming"></a>

### Step 10 of 10 · Validate complete octrees before range streaming (`67si-validate-complete-octrees-before-range-streaming`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 10/10 Visual Trace (67si-validate-complete-octrees-before-range-streaming): Validate complete octrees before range streaming</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original file bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked source layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing GPU array owners</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">later range streaming</text>
    
</svg>
</div>

- **Goal:** Validate complete octrees before range streaming.
- **Data Flow Trace:** `original file bytes → checked source layout → existing GPU array owners → later range streaming`
- **Files You Touch:** `src/stream.rs` (*Validate complete octrees before range streaming.*)

Check column lengths with checked multiplication before indexing.

Then validate each node and its children with one shared parent bitmap. A final parent check rejects disconnected nodes. The supplied fixtures exercise malformed headers, finite conversion, original IDs, sheet fields and hostile topology; browser range requests connect in the following lessons.

> **Self-Check Question:** *Do valid individual nodes prove the table is connected?*  
> **Answer:** No. Every non-root node must also have exactly one valid parent.

- **Hands-On Lab Experiment:** Predict outcomes for an empty tree, a single valid root and an otherwise valid detached node. Continue to the range-request lessons using this same module.

<details>
<summary>Implementation 67si-validate-complete-octrees-before-range-streaming: Validate complete octrees before range streaming</summary>

Validate complete octrees before range streaming.

Data flow: original file bytes → checked source layout → existing GPU array owners → later range streaming.

### 1. `src/stream.rs`

Validate complete octrees before range streaming.

<details>
<summary>Locate the existing block</summary>

```rust
            let Ok(child) = usize::try_from(child) else {
                return false;
            };

            if child == 0
                || child >= parents.len()
                || parents[child]
                || self.level[node].checked_add(1) != Some(self.level[child])
            {
                return false;
            }

            parents[child] = true;
        }

        true
    }

    /// True when the file carried no octree.
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}
```

</details>

After that block, type:

```rust

impl CloudLod {
    /// True when every node and child index is sound.
    pub fn valid(&self, total: u32) -> bool {
        let n = self.len();
        let Some(triples) = n.checked_mul(3) else {
            return false;
        };
        let Some(children) = n.checked_mul(8) else {
            return false;
        };

        if n == 0
            || self.min.len() != triples
            || self.spacing.len() != n
            || self.level.len() != n
            || self.first.len() != n
            || self.count.len() != n
            || self.children.len() != children
            || self.level[0] != 0
        {
            return false;
        }

        let mut parents = vec![false; n];

        for node in 0..n {
            if !self.valid_node(node, total) || !self.valid_children(node, &mut parents) {
                return false;
            }
        }

        !parents[1..].contains(&false)
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/67si-validate-complete-octrees-before-range-streaming-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/stream.rs`

Supply independent malformed-layout and original-source value fixtures.

<details>
<summary>Locate the existing block</summary>

```text
          || self.level.len() != n
            || self.first.len() != n
            || self.count.len() != n
            || self.children.len() != children
            || self.level[0] != 0
        {
            return false;
        }

        let mut parents = vec![false; n];

        for node in 0..n {
            if !self.valid_node(node, total) || !self.valid_children(node, &mut parents) {
                return false;
            }
        }

        !parents[1..].contains(&false)
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67si-validate-complete-octrees-before-range-streaming-02.rs" skip=500
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 67si-validate-complete-octrees-before-range-streaming
npm --prefix ../session_tests run course -- check 67si-validate-complete-octrees-before-range-streaming
npm --prefix ../session_tests run course -- save 67si-validate-complete-octrees-before-range-streaming
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Predict outcomes for an empty tree, a single valid root and an otherwise valid detached node. Continue to the range-request lessons using this same module.

Continue with [Bounded scheduling, exact replies and cancellation](../../consolidation/66za-own-original-range-read-permits.md).
