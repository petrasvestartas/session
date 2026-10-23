# 12.3 · Bounded scheduling, exact replies and cancellation

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Lesson 12.3** (Redraft `7.3`) · **17 checkpoints (`66za-own-original-range-read-permits` → `67m-cancel-range-work-when-its-runtime-closes`)** · **Study time: 8.5–17 h**

> **What You Will Learn & Build:** Write the per-scene gate, queued turn, waiting-read and permit types. By the final checkpoint (`67m-cancel-range-work-when-its-runtime-closes`), you will cancel range work when its runtime closes.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 12.3 Pipeline Schematic: Bounded scheduling, exact replies and cancellation</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (17 Checkpoints: 66za-own-original-range-read-permits → 67m-cancel-range-work-when-its-runtime-closes)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">queued range read</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">permit</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">browser request</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">release</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">Close or device</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">disposal</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 12.3:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/range_gate.rs  ·  src/lib.rs  ·  src/range_rules.rs  ·  src/range_page.rs  ·  Cargo.toml  ·  src/range_fetch.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">17 Checkpoints: 66za-own-original-range-read-permits → 67m-cancel-range-work-when-its-runtime-closes — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`66za-own-original-range-read-permits`](#checkpoint-66za-own-original-range-read-permits) | Write the per-scene gate, queued turn, waiting-read and permit types. | `src/range_gate.rs`, `src/lib.rs` | `queued range read → permit → browser request → release` |
| **2** | [`67a-queue-range-reads-in-arrival-order`](#checkpoint-67a-queue-range-reads-in-arrival-order) | Complete the waiting-read Future and its Drop behavior. | `src/range_gate.rs` | `queued range read → permit → browser request → release` |
| **3** | [`67b-validate-exact-http-range-replies`](#checkpoint-67b-validate-exact-http-range-replies) | Validate exact HTTP range replies. | `src/range_rules.rs`, `src/lib.rs` | `requested interval → HTTP206 → matching Content-Range → accepted bytes` |
| **4** | [`67c-retain-original-point-page-values`](#checkpoint-67c-retain-original-point-page-values) | Retain original point page values. | `src/range_page.rs`, `src/lib.rs` | `original bytes → precise coordinates and IDs → separate display floats` |
| **5** | [`67d-own-cancellable-range-requests`](#checkpoint-67d-own-cancellable-range-requests) | Own cancellable range requests. | `Cargo.toml`, `src/range_fetch.rs`, `src/lib.rs` | `request owner → retained abort handle → cancellation → request release` |
| **6** | [`67e-time-out-stalled-source-reads`](#checkpoint-67e-time-out-stalled-source-reads) | Time out stalled source reads. | `src/range_fetch.rs` | `received chunk → fresh deadline → stall abort → timer cleanup` |
| **7** | [`67f-bound-incoming-range-body-chunks`](#checkpoint-67f-bound-incoming-range-body-chunks) | Bound incoming range body chunks. | `src/range_fetch.rs` | `incoming chunk → remaining budget → retained original bytes` |
| **8** | [`67g-request-and-accept-one-source-interval`](#checkpoint-67g-request-and-accept-one-source-interval) | Request and accept one source interval. | `src/range_fetch.rs` | `checked interval → fetch → bounded body → exact response acceptance` |
| **9** | [`67h-keep-range-revisions-and-background-results`](#checkpoint-67h-keep-range-revisions-and-background-results) | Keep range revisions and background results. | `src/range_fetch.rs` | `source ETag → complete range → retained result → caller ownership` |
| **10** | [`67hza-queue-original-source-range-reads`](#checkpoint-67hza-queue-original-source-range-reads) | Write the browser range-read owner and cached metadata-window read. | `src/stream.rs` | `bounded HTTP read → revision and layout → source-array slice → decoded points` |
| **11** | [`67hzb-probe-original-source-array-layouts`](#checkpoint-67hzb-probe-original-source-array-layouts) | Start owned asynchronous range tasks and inspect the bounded file probe. | `src/stream.rs` | `bounded HTTP read → revision and layout → source-array slice → decoded points` |
| **12** | [`67hzc-read-original-cloud-node-tables`](#checkpoint-67hzc-read-original-cloud-node-tables) | Read original node-table fields through the cached metadata window. | `src/stream.rs` | `bounded HTTP read → revision and layout → source-array slice → decoded points` |
| **13** | [`67i-read-source-arrays-through-the-bounded-queue`](#checkpoint-67i-read-source-arrays-through-the-bounded-queue) | Fetch bounded source-array slices through the queue already written. | `src/stream.rs` | `bounded HTTP read → revision and layout → source-array slice → decoded points` |
| **14** | [`67j-keep-original-ranged-source-identity`](#checkpoint-67j-keep-original-ranged-source-identity) | Keep original ranged source identity. | `src/stream.rs` | `small source header → original GUID → stable remote source identity` |
| **15** | [`67k-prepare-a-revision-bound-remote-source`](#checkpoint-67k-prepare-a-revision-bound-remote-source) | Prepare a revision bound remote source. | `src/range_source.rs`, `src/lib.rs` | `stable URL revision → checked original layout → remote source owner` |
| **16** | [`67l-fetch-precise-coordinates-and-original-ids`](#checkpoint-67l-fetch-precise-coordinates-and-original-ids) | Fetch precise coordinates and original IDs. | `src/range_source.rs` | `source row interval → coordinate and ID ranges → checked original page` |
| **17** | [`67m-cancel-range-work-when-its-runtime-closes`](#checkpoint-67m-cancel-range-work-when-its-runtime-closes) | Cancel range work when its runtime closes. | `src/browser_runtime.rs`, `src/browser.rs` | `Close or device disposal → closed gate → aborted reads → released owners` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-66za-own-original-range-read-permits"></a>

### Step 1 of 17 · Own original range-read permits (`66za-own-original-range-read-permits`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/17 Visual Trace (66za-own-original-range-read-permits): Own original range-read permits</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_gate.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">queued range read</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">permit</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">browser request</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">release</text>
    
</svg>
</div>

- **Goal:** Write the per-scene gate, queued turn, waiting-read and permit types.
- **Data Flow Trace:** `queued range read → permit → browser request → release`
- **Files You Touch:** `src/range_gate.rs` (*Type the whole original helpers introduced today.*) · `src/lib.rs` (*Register the original browser range-read gate.*)

Write the per-scene gate, queued turn, waiting-read and permit types. Bound simultaneous reads, wake queued turns and hand a released permit to the oldest waiting read.

> **Self-Check Question:** *Why does a permit own the gate?*  
> **Answer:** The permit releases its slot when dropped. Its gate must remain alive until that release, even if the caller has already discarded its scene reference.

- **What You Should See:** A retained permit describes one original range read; the existing browser scene is unchanged.
- **Hands-On Lab Experiment:** Trace a permit release with and without an older queued turn. The waiting-read Future is completed in the next lesson before this gate is connected.

<details>
<summary>Implementation 66za-own-original-range-read-permits: Own original range-read permits</summary>

Write the per-scene gate, queued turn, waiting-read and permit types.

Data flow: queued range read → permit → browser request → release.

### 1. `src/range_gate.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/66za-own-original-range-read-permits-1.rs"
```

### 2. `src/lib.rs`

Register the original browser range-read gate.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod stream;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66za-own-original-range-read-permits-2.rs" skip=17
```

</details>

<a id="checkpoint-67a-queue-range-reads-in-arrival-order"></a>

### Step 2 of 17 · Poll and cancel original queued range reads (`67a-queue-range-reads-in-arrival-order`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/17 Visual Trace (67a-queue-range-reads-in-arrival-order): Poll and cancel original queued range reads</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_gate.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">queued range read</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">permit</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">browser request</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">release</text>
    
</svg>
</div>

- **Goal:** Complete the waiting-read Future and its Drop behavior.
- **Data Flow Trace:** `queued range read → permit → browser request → release`
- **Files You Touch:** `src/range_gate.rs` (*Extend this original owner with today’s complete helper.*)

Complete the waiting-read Future and its Drop behavior. Admit free capacity without overtaking older readers, refresh each pending waker and remove cancelled turns or pass on an already granted slot.

> **Self-Check Question:** *Why remove a cancelled waiting turn?*  
> **Answer:** A cancelled read must not retain its queue position or consume a later permit. Dropping a turn already granted a slot must hand that slot onward.

- **What You Should See:** The range gate queues reads in arrival order before its browser range-read connection.
- **Hands-On Lab Experiment:** Inspect the original queue, cancellation and closed-gate fixtures. Trace cancellation before and after a slot is granted.

<details>
<summary>Implementation 67a-queue-range-reads-in-arrival-order: Poll and cancel original queued range reads</summary>

Complete the waiting-read Future and its Drop behavior.

Data flow: queued range read → permit → browser request → release.

### 1. `src/range_gate.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        }
    }
}

/// A read waiting at the gate.
pub struct Enter {
    gate: Rc<RangeGate>,    // the gate
    turn: Option<Rc<Turn>>, // its place in the queue, once queued
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67a-queue-range-reads-in-arrival-order-1.rs" skip=180
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/range_gate.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

/// Range reads a scene keeps in flight; the rest wait their turn.
pub const RANGE_READS: usize = 3;

/// A waiting read's place in the queue.
#[derive(Default)]
struct Turn {
    given: Cell<bool>,             // a slot was handed to it
    waker: RefCell<Option<Waker>>, // wakes its read
}

impl Turn {
    /// Wake the read waiting on this turn.
    fn wake(&self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

/// Lets a few reads run at once, first come first served; a closed gate turns waiting reads away.
pub struct RangeGate {
    limit: usize,                       // slots
    running: Cell<usize>,               // slots taken
    queue: RefCell<VecDeque<Rc<Turn>>>, // waiting reads, oldest first
    closed: Cell<bool>,                 // its scene is gone
}

impl RangeGate {
    /// A gate with `limit` slots.
    pub fn new(limit: usize) -> Rc<Self> {
        Rc::new(Self {
            limit: limit.max(1),
            running: Cell::new(0),
            queue: RefCell::new(VecDeque::new()),
            closed: Cell::new(false),
        })
    }

    /// Wait for a slot; None once the gate closed.
    pub fn enter(self: &Rc<Self>) -> Enter {
        Enter {
            gate: self.clone(),
            turn: None,
        }
    }

    /// Turn every waiting read away.
    pub fn close(&self) {
        self.closed.set(true);

        for turn in self.queue.take() {
            turn.wake();
        }
    }

    /// The gate's scene is gone.
    pub fn is_closed(&self) -> bool {
        self.closed.get()
    }

    /// Reads running and waiting.
    pub fn load(&self) -> (usize, usize) {
        (self.running.get(), self.queue.borrow().len())
    }

    /// A slot came free: hand it to the oldest waiting read.
    fn release(&self) {
        let next = self.queue.borrow_mut().pop_front();

        match next {
            Some(turn) => {
                turn.given.set(true);
                turn.wake();
            }
            None => self.running.set(self.running.get().saturating_sub(1)),
        }
    }
}

/// A read waiting at the gate.
pub struct Enter {
    gate: Rc<RangeGate>,    // the gate
    turn: Option<Rc<Turn>>, // its place in the queue, once queued
}

impl Future for Enter {
    type Output = Option<Permit>;

    /// A slot when one is free and no older read waits.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let gate = this.gate.clone();

        if gate.closed.get() {
            this.turn = None;
            return Poll::Ready(None);
        }

        match &this.turn {
            None if gate.running.get() < gate.limit && gate.queue.borrow().is_empty() => {
                gate.running.set(gate.running.get() + 1);
                Poll::Ready(Some(Permit { gate }))
            }
            None => {
                let turn = Rc::new(Turn::default());
                *turn.waker.borrow_mut() = Some(cx.waker().clone());
                gate.queue.borrow_mut().push_back(turn.clone());
                this.turn = Some(turn);
                Poll::Pending
            }
            Some(turn) if turn.given.get() => {
                this.turn = None;
                Poll::Ready(Some(Permit { gate }))
            }
            Some(turn) => {
                *turn.waker.borrow_mut() = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl Drop for Enter {
    /// A read dropped while waiting leaves the queue, or passes on a slot it was given.
    fn drop(&mut self) {
        let Some(turn) = self.turn.take() else {
            return;
        };

        if turn.given.get() {
            self.gate.release();
        } else {
            self.gate
                .queue
                .borrow_mut()
                .retain(|queued| !Rc::ptr_eq(queued, &turn));
        }
    }
}

/// A slot at the gate, freed when dropped.
pub struct Permit {
    gate: Rc<RangeGate>, // the gate it came from
}

impl Drop for Permit {
    /// Free the slot.
    fn drop(&mut self) {
        self.gate.release();
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67a-queue-range-reads-in-arrival-order-2.rs" skip=4322
```

</details>

</details>

<a id="checkpoint-67b-validate-exact-http-range-replies"></a>

### Step 3 of 17 · Validate exact HTTP range replies (`67b-validate-exact-http-range-replies`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/17 Visual Trace (67b-validate-exact-http-range-replies): Validate exact HTTP range replies</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_rules.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">requested interval</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">HTTP206</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">matching Content-Range</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">accepted bytes</text>
    
</svg>
</div>

- **Goal:** Validate exact HTTP range replies.
- **Data Flow Trace:** `requested interval → HTTP206 → matching Content-Range → accepted bytes`
- **Files You Touch:** `src/range_rules.rs` (*Validate the source interval before issuing or accepting a read.*) · `src/lib.rs` (*Register the source component.*)

Compute checked inclusive HTTP intervals, then match response offsets, lengths and total file size.

> **Self-Check Question:** *May a server return the whole file instead of the requested range?*  
> **Answer:** No. Refuse HTTP200 without downloading its unwanted body. Only the initial probe may stop at an earlier file end.

- **Hands-On Lab Experiment:** Check wrong offsets, missing headers, incomplete chunks, invalid totals and u64 overflow.

<details>
<summary>Implementation 67b-validate-exact-http-range-replies: Validate exact HTTP range replies</summary>

Validate exact HTTP range replies.

Data flow: requested interval → HTTP206 → matching Content-Range → accepted bytes.

### 1. `src/range_rules.rs`

Validate the source interval before issuing or accepting a read.

Create the file and type:

```rust
--8<-- "journey/code/67b-validate-exact-http-range-replies-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod range_gate;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67b-validate-exact-http-range-replies-02.rs" skip=21
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/range_rules.rs`

Copy exact-response, EOF-probe and overflow regression fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    Ok(total)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67b-validate-exact-http-range-replies-03.rs" skip=16
```

</details>

</details>

<a id="checkpoint-67c-retain-original-point-page-values"></a>

### Step 4 of 17 · Retain original point page values (`67c-retain-original-point-page-values`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/17 Visual Trace (67c-retain-original-point-page-values): Retain original point page values</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_page.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original bytes</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">precise coordinates and IDs</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">separate display floats</text>
    
</svg>
</div>

- **Goal:** Retain original point page values.
- **Data Flow Trace:** `original bytes → precise coordinates and IDs → separate display floats`
- **Files You Touch:** `src/range_page.rs` (*Retain original f64 pages and stable IDs separately from derived display values.*) · `src/lib.rs` (*Register the source component.*)

Keep original doubles and IDs beside derived display floats. An absent ID column uses the source row.

> **Self-Check Question:** *Can nearby original doubles become the same display float?*  
> **Answer:** Yes. Preserve both original doubles so picking and measurements can recover the precise source values.

- **Hands-On Lab Experiment:** Compare1000000000.125 and1000000000.375, original ID4294967295 and invalid source pages.

<details>
<summary>Implementation 67c-retain-original-point-page-values: Retain original point page values</summary>

Retain original point page values.

Data flow: original bytes → precise coordinates and IDs → separate display floats.

### 1. `src/range_page.rs`

Retain original f64 pages and stable IDs separately from derived display values.

Create the file and type:

```rust
--8<-- "journey/code/67c-retain-original-point-page-values-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod range_rules;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67c-retain-original-point-page-values-02.rs" skip=22
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/range_page.rs`

Copy source precision, high-ID, incomplete page and invalid-double fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    pub fn bytes(&self)->usize {self.coordinates.capacity()*8+self.display.capacity()*4+self.ids.capacity()*4}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67c-retain-original-point-page-values-03.rs" skip=113
```

</details>

</details>

<a id="checkpoint-67d-own-cancellable-range-requests"></a>

### Step 5 of 17 · Own cancellable range requests (`67d-own-cancellable-range-requests`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/17 Visual Trace (67d-own-cancellable-range-requests): Own cancellable range requests</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: Cargo.toml, src/range_fetch.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">request owner</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">retained abort handle</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">cancellation</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">request release</text>
    
</svg>
</div>

- **Goal:** Own cancellable range requests.
- **Data Flow Trace:** `request owner → retained abort handle → cancellation → request release`
- **Files You Touch:** `Cargo.toml` (*Enable browser CORS request configuration without changing package versions.*) · `src/range_fetch.rs` (*Own active browser request handles and explicit cancellation.*) · `src/lib.rs` (*Register the source component.*)

Assign each request a unique owner and retain its abort controller until completion.

> **Self-Check Question:** *May Close leave active source downloads running?*  
> **Answer:** No. Abort every retained request and let its owner remove its registry entry.

- **Hands-On Lab Experiment:** Trace registry insertion, unique keys and cancellation without retaining the source scene.

<details>
<summary>Implementation 67d-own-cancellable-range-requests: Own cancellable range requests</summary>

Own cancellable range requests.

Data flow: request owner → retained abort handle → cancellation → request release.

### 1. `Cargo.toml`

Enable browser CORS request configuration without changing package versions.

<details>
<summary>Locate the existing block</summary>

```toml
[package]
name = "viewer_journey"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[workspace]

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
prost = "=0.14.4"
sha2 = "=0.10.8"
uuid = { version = "=1.26.1", features = ["v4", "js"] }
js-sys = "=0.3.105"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"
session_rust = { path = "../../../session_rust", default-features = false }
getrandom = { version = "=0.2.17", features = ["js"] }

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", features = ["float_roundtrip"] }

glyphon = "=0.11.0"
anyhow = "=1.0.104"
ttf-parser = {version="=0.25.1",default-features=false}

serde_yaml_ng = "=0.10.0"
toml = { version = "=0.8.23", optional = true }

[features]
json-sessions = []
toml-manifests = ["dep:toml"]

[dev-dependencies]
pollster = "=0.4.0"
log = "=0.4.34"
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
--8<-- "journey/code/67d-own-cancellable-range-requests-cargo.toml"
```

### 2. `src/range_fetch.rs`

Own active browser request handles and explicit cancellation.

Create the file and type:

```rust
--8<-- "journey/code/67d-own-cancellable-range-requests-01.rs"
```

### 3. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod range_page;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67d-own-cancellable-range-requests-02.rs" skip=21
```

</details>

<a id="checkpoint-67e-time-out-stalled-source-reads"></a>

### Step 6 of 17 · Time out stalled source reads (`67e-time-out-stalled-source-reads`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/17 Visual Trace (67e-time-out-stalled-source-reads): Time out stalled source reads</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_fetch.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">received chunk</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">fresh deadline</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">stall abort</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">timer cleanup</text>
    
</svg>
</div>

- **Goal:** Time out stalled source reads.
- **Data Flow Trace:** `received chunk → fresh deadline → stall abort → timer cleanup`
- **Files You Touch:** `src/range_fetch.rs` (*Connect the owned stall timer, progress reset and request cleanup.*)

Restart the deadline when bytes arrive. Abort and clear its timer when the read owner drops.

> **Self-Check Question:** *Do queued reads consume their stall timeout?*  
> **Answer:** No. The range gate admits a read before it creates this timer.

- **Hands-On Lab Experiment:** Trace a30-second stall, network failure and intentional cancellation; only network failures are retryable.

<details>
<summary>Implementation 67e-time-out-stalled-source-reads: Time out stalled source reads</summary>

Time out stalled source reads.

Data flow: received chunk → fresh deadline → stall abort → timer cleanup.

### 1. `src/range_fetch.rs`

Connect the owned stall timer, progress reset and request cleanup.

<details>
<summary>Locate the existing block</summary>

```rust
    callback:Closure<dyn FnMut()>,
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67e-time-out-stalled-source-reads-01.rs" skip=37
```

</details>

<a id="checkpoint-67f-bound-incoming-range-body-chunks"></a>

### Step 7 of 17 · Bound incoming range body chunks (`67f-bound-incoming-range-body-chunks`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 7/17 Visual Trace (67f-bound-incoming-range-body-chunks): Bound incoming range body chunks</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_fetch.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">incoming chunk</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">remaining budget</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">retained original bytes</text>
    
</svg>
</div>

- **Goal:** Bound incoming range body chunks.
- **Data Flow Trace:** `incoming chunk → remaining budget → retained original bytes`
- **Files You Touch:** `src/range_fetch.rs` (*Read bounded body chunks while retaining the checked source request owner.*)

Check every chunk against the requested byte length before allocating its copied bytes.

> **Self-Check Question:** *Can Content-Length replace a chunk-by-chunk limit?*  
> **Answer:** No. Servers may omit it, and encoded bodies can decode to more bytes.

- **Hands-On Lab Experiment:** Trace reader cancellation and release when a response grows beyond its declared range.

<details>
<summary>Implementation 67f-bound-incoming-range-body-chunks: Bound incoming range body chunks</summary>

Bound incoming range body chunks.

Data flow: incoming chunk → remaining budget → retained original bytes.

### 1. `src/range_fetch.rs`

Read bounded body chunks while retaining the checked source request owner.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn retryable(error:&str)->bool {error.starts_with("network error")}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67f-bound-incoming-range-body-chunks-01.rs" skip=73
```

</details>

<a id="checkpoint-67g-request-and-accept-one-source-interval"></a>

### Step 8 of 17 · Request and accept one source interval (`67g-request-and-accept-one-source-interval`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 8/17 Visual Trace (67g-request-and-accept-one-source-interval): Request and accept one source interval</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_fetch.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked interval</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">fetch</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded body</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">exact response acceptance</text>
    
</svg>
</div>

- **Goal:** Request and accept one source interval.
- **Data Flow Trace:** `checked interval → fetch → bounded body → exact response acceptance`
- **Files You Touch:** `src/range_fetch.rs` (*Connect HTTP range headers and exact response acceptance.*)

Issue the Range header and verify HTTP status, offset and length before returning original bytes.

> **Self-Check Question:** *Should a changed ETag silently replace part of a page?*  
> **Answer:** No. The higher-level source reader refuses a revision mismatch before adopting the page.

- **Hands-On Lab Experiment:** Trace unwanted HTTP200, Content-Length overflow, missing Content-Range and released stream locks.

<details>
<summary>Implementation 67g-request-and-accept-one-source-interval: Request and accept one source interval</summary>

Request and accept one source interval.

Data flow: checked interval → fetch → bounded body → exact response acceptance.

### 1. `src/range_fetch.rs`

Connect HTTP range headers and exact response acceptance.

<details>
<summary>Locate the existing block</summary>

```rust
}

async fn body(reader:&web_sys::ReadableStreamDefaultReader,limit:u64,read:&mut Read)->Result<Vec<u8>,String> {
    let mut bytes=Vec::new();loop {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67g-request-and-accept-one-source-interval-01.rs"
```

</details>

<a id="checkpoint-67h-keep-range-revisions-and-background-results"></a>

### Step 9 of 17 · Keep range revisions and background results (`67h-keep-range-revisions-and-background-results`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 9/17 Visual Trace (67h-keep-range-revisions-and-background-results): Keep range revisions and background results</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_fetch.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source ETag</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">complete range</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">retained result</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">caller ownership</text>
    
</svg>
</div>

- **Goal:** Keep range revisions and background results.
- **Data Flow Trace:** `source ETag → complete range → retained result → caller ownership`
- **Files You Touch:** `src/range_fetch.rs` (*Validate source revisions and provide background range reads.*)

Compare every accepted ETag with the source revision and retain a background result until its caller awaits it.

> **Self-Check Question:** *Does starting a Task transfer its page into the scene?*  
> **Answer:** No. The task owns only its result. The caller must still check the current source and runtime before adoption.

- **Hands-On Lab Experiment:** Trace changed validators, failed requests and result release after a dropped caller.

<details>
<summary>Implementation 67h-keep-range-revisions-and-background-results: Keep range revisions and background results</summary>

Keep range revisions and background results.

Data flow: source ETag → complete range → retained result → caller ownership.

### 1. `src/range_fetch.rs`

Validate source revisions and provide background range reads.

<details>
<summary>Locate the existing block</summary>

```rust
    }Ok(bytes)
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67h-keep-range-revisions-and-background-results-01.rs" skip=17
```

</details>

<a id="checkpoint-67hza-queue-original-source-range-reads"></a>

### Step 10 of 17 · Queue original source range reads (`67hza-queue-original-source-range-reads`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 10/17 Visual Trace (67hza-queue-original-source-range-reads): Queue original source range reads</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded HTTP read</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">revision and layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source-array slice</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">decoded points</text>
    
</svg>
</div>

- **Goal:** Write the browser range-read owner and cached metadata-window read.
- **Data Flow Trace:** `bounded HTTP read → revision and layout → source-array slice → decoded points`
- **Files You Touch:** `src/stream.rs` (*Type today’s complete browser source readers; reuse the decoding and queue helpers already written.*)

Write the browser range-read owner and cached metadata-window read. Acquire a permit before starting the stall timer, retry once when allowed and close the old scene queue on reset.

> **Self-Check Question:** *Why start the stall timer after acquiring a permit?*  
> **Answer:** A queued request is waiting for capacity rather than stalled network delivery. Counting that wait would cancel healthy reads.

- **What You Should See:** Original source-array reads can enter the bounded queue without changing the current drawing.
- **Hands-On Lab Experiment:** Trace a queued read, a retryable response and a scene reset.

<details>
<summary>Implementation 67hza-queue-original-source-range-reads: Queue original source range reads</summary>

Write the browser range-read owner and cached metadata-window read.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67hza-queue-original-source-range-reads-1.rs"
```

</details>

<a id="checkpoint-67hzb-probe-original-source-array-layouts"></a>

### Step 11 of 17 · Probe original source array layouts (`67hzb-probe-original-source-array-layouts`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 11/17 Visual Trace (67hzb-probe-original-source-array-layouts): Probe original source array layouts</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded HTTP read</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">revision and layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source-array slice</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">decoded points</text>
    
</svg>
</div>

- **Goal:** Start owned asynchronous range tasks and inspect the bounded file probe.
- **Data Flow Trace:** `bounded HTTP read → revision and layout → source-array slice → decoded points`
- **Files You Touch:** `src/stream.rs` (*Type today’s complete browser source readers; reuse the decoding and queue helpers already written.*)

Start owned asynchronous range tasks and inspect the bounded file probe. Locate the coordinate and colour arrays while checking file bounds, point stride and response status.

> **Self-Check Question:** *Why require a range response?*  
> **Answer:** The layout parser relies on a bounded partial response rather than an unbounded whole-file transfer.

- **What You Should See:** Source-array layout probes are prepared for the bounded reader.
- **Hands-On Lab Experiment:** Trace an empty array, invalid double stride and truncated colour header.

<details>
<summary>Implementation 67hzb-probe-original-source-array-layouts: Probe original source array layouts</summary>

Start owned asynchronous range tasks and inspect the bounded file probe.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67hzb-probe-original-source-array-layouts-1.rs"
```

</details>

<a id="checkpoint-67hzc-read-original-cloud-node-tables"></a>

### Step 12 of 17 · Read original cloud node tables (`67hzc-read-original-cloud-node-tables`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 12/17 Visual Trace (67hzc-read-original-cloud-node-tables): Read original cloud node tables</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded HTTP read</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">revision and layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source-array slice</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">decoded points</text>
    
</svg>
</div>

- **Goal:** Read original node-table fields through the cached metadata window.
- **Data Flow Trace:** `bounded HTTP read → revision and layout → source-array slice → decoded points`
- **Files You Touch:** `src/stream.rs` (*Type today’s complete browser source readers; reuse the decoding and queue helpers already written.*)

Read original node-table fields through the cached metadata window. Reject duplicate fields and oversized tables while retaining point-ID and normal ranges.

> **Self-Check Question:** *Why bound the accumulated node-table bytes?*  
> **Answer:** Many individually valid fields could otherwise require an excessive combined metadata allocation.

- **What You Should See:** Original cloud node tables can be decoded; streamed drawing connects later.
- **Hands-On Lab Experiment:** Trace a duplicate field, absent required field and table-size limit.

<details>
<summary>Implementation 67hzc-read-original-cloud-node-tables: Read original cloud node tables</summary>

Read original node-table fields through the cached metadata window.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67hzc-read-original-cloud-node-tables-1.rs"
```

</details>

<a id="checkpoint-67i-read-source-arrays-through-the-bounded-queue"></a>

### Step 13 of 17 · Fetch original coordinates normals and colours (`67i-read-source-arrays-through-the-bounded-queue`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 13/17 Visual Trace (67i-read-source-arrays-through-the-bounded-queue): Fetch original coordinates normals and colours</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded HTTP read</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">revision and layout</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source-array slice</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">decoded points</text>
    
</svg>
</div>

- **Goal:** Fetch bounded source-array slices through the queue already written.
- **Data Flow Trace:** `bounded HTTP read → revision and layout → source-array slice → decoded points`
- **Files You Touch:** `src/stream.rs` (*Type today’s complete browser source readers; reuse the decoding and queue helpers already written.*)

Fetch bounded source-array slices through the queue already written. Convert coordinate doubles and normal records, and advance the variable-length colour cursor using its consumed bytes.

> **Self-Check Question:** *Why advance colours by consumed bytes?*  
> **Answer:** Packed varints have variable size. The fetched window can contain bytes beyond the requested colours.

- **What You Should See:** The complete bounded source-array reader compiles before browser import and streaming are connected.
- **Hands-On Lab Experiment:** Compare position and normal strides with a short varint colour slice.

<details>
<summary>Implementation 67i-read-source-arrays-through-the-bounded-queue: Fetch original coordinates normals and colours</summary>

Fetch bounded source-array slices through the queue already written.

Data flow: bounded HTTP read → revision and layout → source-array slice → decoded points.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

<details>
<summary>Locate the existing block</summary>

```rust
}

#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67i-read-source-arrays-through-the-bounded-queue-1.rs"
```

</details>

<a id="checkpoint-67j-keep-original-ranged-source-identity"></a>

### Step 14 of 17 · Keep original ranged source identity (`67j-keep-original-ranged-source-identity`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 14/17 Visual Trace (67j-keep-original-ranged-source-identity): Keep original ranged source identity</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/stream.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">small source header</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original GUID</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">stable remote source identity</text>
    
</svg>
</div>

- **Goal:** Keep original ranged source identity.
- **Data Flow Trace:** `small source header → original GUID → stable remote source identity`
- **Files You Touch:** `src/stream.rs` (*Read original source identity independently of display pages.*)

Read the original cloud GUID and name before its coordinate body. Refuse missing or repeated identity fields.

> **Self-Check Question:** *Is an HTTP request ticket an original cloud GUID?*  
> **Answer:** No. Tickets identify transient work; the GUID identifies the original geometry.

- **Hands-On Lab Experiment:** Compare the parsed identity with the actual kernel protobuf and reject truncated headers.

<details>
<summary>Implementation 67j-keep-original-ranged-source-identity: Keep original ranged source identity</summary>

Keep original ranged source identity.

Data flow: small source header → original GUID → stable remote source identity.

### 1. `src/stream.rs`

Read original source identity independently of display pages.

<details>
<summary>Locate the existing block</summary>

```rust
    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67j-keep-original-ranged-source-identity-01.rs" skip=73
```

</details>

<a id="checkpoint-67k-prepare-a-revision-bound-remote-source"></a>

### Step 15 of 17 · Prepare a revision bound remote source (`67k-prepare-a-revision-bound-remote-source`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 15/17 Visual Trace (67k-prepare-a-revision-bound-remote-source): Prepare a revision bound remote source</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_source.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">stable URL revision</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">checked original layout</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">remote source owner</text>
    
</svg>
</div>

- **Goal:** Prepare a revision bound remote source.
- **Data Flow Trace:** `stable URL revision → checked original layout → remote source owner`
- **Files You Touch:** `src/range_source.rs` (*Prepare a revision-bound original remote source descriptor.*) · `src/lib.rs` (*Register the source component.*)

Prepare original array offsets and the complete checked octree before requesting point pages.

> **Self-Check Question:** *Can an unversioned mutable URL safely mix independent ranges?*  
> **Answer:** No. This reader requires an ETag or an immutable content-addressed source URL.

- **Hands-On Lab Experiment:** Trace source-size checks, original GUID/name, valid octree adoption and invalid source refusal.

<details>
<summary>Implementation 67k-prepare-a-revision-bound-remote-source: Prepare a revision bound remote source</summary>

Prepare a revision bound remote source.

Data flow: stable URL revision → checked original layout → remote source owner.

### 1. `src/range_source.rs`

Prepare a revision-bound original remote source descriptor.

Create the file and type:

```rust
--8<-- "journey/code/67k-prepare-a-revision-bound-remote-source-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(target_arch = "wasm32")]
pub mod range_fetch;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/67k-prepare-a-revision-bound-remote-source-02.rs" skip=52
```

</details>

<a id="checkpoint-67l-fetch-precise-coordinates-and-original-ids"></a>

### Step 16 of 17 · Fetch precise coordinates and original IDs (`67l-fetch-precise-coordinates-and-original-ids`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 16/17 Visual Trace (67l-fetch-precise-coordinates-and-original-ids): Fetch precise coordinates and original IDs</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/range_source.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source row interval</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">coordinate and ID ranges</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">checked original page</text>
    
</svg>
</div>

- **Goal:** Fetch precise coordinates and original IDs.
- **Data Flow Trace:** `source row interval → coordinate and ID ranges → checked original page`
- **Files You Touch:** `src/range_source.rs` (*Fetch matching original coordinate and ID pages under the source revision.*)

Read coordinate and ID columns for the same source rows, then decode one complete original page.

> **Self-Check Question:** *May a short ID range become implicit row identities?*  
> **Answer:** No. Only an absent source ID column permits that fallback; a present incomplete column is refused.

- **Hands-On Lab Experiment:** Compare actual kernel protobuf values and high-u32 IDs with each decoded source page.

<details>
<summary>Implementation 67l-fetch-precise-coordinates-and-original-ids: Fetch precise coordinates and original IDs</summary>

Fetch precise coordinates and original IDs.

Data flow: source row interval → coordinate and ID ranges → checked original page.

### 1. `src/range_source.rs`

Fetch matching original coordinate and ID pages under the source revision.

<details>
<summary>Locate the existing block</summary>

```rust
        Ok(Self {guid,name,url,fields,lod})
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67l-fetch-precise-coordinates-and-original-ids-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/range_source.rs`

Copy actual kernel-protobuf identity, precision and page-layout regression fixtures.

<details>
<summary>Locate the existing block</summary>

```text
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/67l-fetch-precise-coordinates-and-original-ids-02.rs" skip=8
```

</details>

</details>

<a id="checkpoint-67m-cancel-range-work-when-its-runtime-closes"></a>

### Step 17 of 17 · Cancel range work when its runtime closes (`67m-cancel-range-work-when-its-runtime-closes`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 17/17 Visual Trace (67m-cancel-range-work-when-its-runtime-closes): Cancel range work when its runtime closes</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/browser_runtime.rs, src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Close or device disposal</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">closed gate</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">aborted reads</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">released owners</text>
    
</svg>
</div>

- **Goal:** Cancel range work when its runtime closes.
- **Data Flow Trace:** `Close or device disposal → closed gate → aborted reads → released owners`
- **Files You Touch:** `src/browser_runtime.rs` (*Cancel queued and active range work with its browser runtime.*) · `src/browser.rs` (*Cancel queued and active range work with its browser runtime.*)

Connect gate replacement and active-request abortion to Close and browser runtime disposal.

> **Self-Check Question:** *Can a cancelled request revive a closed source scene?*  
> **Answer:** No. Its request is aborted; the page adoption in68 also checks the source generation and runtime owner.

- **Hands-On Lab Experiment:** Close with queued and active reads; keep the mixed archive picture unchanged until streamed page display connects.

<details>
<summary>Implementation 67m-cancel-range-work-when-its-runtime-closes: Cancel range work when its runtime closes</summary>

Cancel range work when its runtime closes.

Data flow: Close or device disposal → closed gate → aborted reads → released owners.

### 1. `src/browser_runtime.rs`

Cancel queued and active range work with its browser runtime.

<details>
<summary>Locate the existing block</summary>

```rust
impl Drop for Runtime {
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67m-cancel-range-work-when-its-runtime-closes-01.rs"
```

### 2. `src/browser.rs`

Cancel queued and active range work with its browser runtime.

<details>
<summary>Locate the existing block</summary>

```rust
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { publication_control.cancel();request.borrow_mut().cancel();let _={text_fonts.clear();renderer.clear_text_fonts();renderer.set_text(Vec::new())}; }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/67m-cancel-range-work-when-its-runtime-closes-02.rs"
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 67m-cancel-range-work-when-its-runtime-closes
npm --prefix ../session_tests run course -- check 67m-cancel-range-work-when-its-runtime-closes
npm --prefix ../session_tests run course -- save 67m-cancel-range-work-when-its-runtime-closes
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Close with queued and active reads; keep the mixed archive picture unchanged until streamed page display connects.

Continue with [Charged residency and current streamed pages](../../consolidation/68-reserve-page-bytes-before-reading.md).
