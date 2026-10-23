# 18.6 · Matched rendering comparisons from original sources

**Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion** · **Lesson 18.6** (Redraft `9.10`) · **4 checkpoints (`94-declare-the-rendering-comparison-contract` → `94c-prove-the-original-comparison-and-inspect-differences`)** · **Study time: 2–4 h**

> **What You Will Learn & Build:** Declare the rendering comparison contract. By the final checkpoint (`94c-prove-the-original-comparison-and-inspect-differences`), you will prove the original comparison and inspect differences.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 9 Visual Explanation: @compute Tile Visibility Binning, Clipping Winding Caps & Horizon GTAO</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Screen tiles binning covering triangles in @compute. Center: Winding-number section cap with bore hole. Right: GTAO horizon rays.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="250" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="143" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. @compute Tile Binning (CSR)</text>
    <rect x="68" y="96" width="150" height="90" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <line x1="118" y1="96" x2="118" y2="186" stroke="#111111" stroke-width="0.75"/>
    <line x1="168" y1="96" x2="168" y2="186" stroke="#111111" stroke-width="0.75"/>
    <line x1="68" y1="126" x2="218" y2="126" stroke="#111111" stroke-width="0.75"/>
    <line x1="68" y1="156" x2="218" y2="156" stroke="#111111" stroke-width="0.75"/>
    <polygon points="90,110 195,135 130,175" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <text x="143" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">atomicAdd + Prefix-Sum Tile Offsets</text>

    <line x1="268" y1="154" x2="290" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="292" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="429" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Clipping Plane &amp; Winding Section Cap</text>
    <rect x="344" y="102" width="170" height="84" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
    <circle cx="429" cy="144" r="22" fill="#ffffff" stroke="#111111" stroke-width="2"/>
    <text x="429" y="148" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Winding = 0</text>
    <text x="429" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Non-zero winding fills solid, keeps bore open</text>

    <line x1="567" y1="154" x2="589" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="591" y="64" width="231" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="706" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Horizon GTAO (ssao.wgsl)</text>
    <polyline points="620,175 685,175 706,135 727,175 792,175" fill="none" stroke="#111111" stroke-width="2"/>
    <circle cx="706" cy="175" r="4" fill="#111111"/>
    <line x1="706" y1="175" x2="665" y2="120" stroke="#111111" stroke-width="1.2" marker-end="url(#arrow)"/>
    <line x1="706" y1="175" x2="747" y2="120" stroke="#111111" stroke-width="1.2" marker-end="url(#arrow)"/>
    <text x="706" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">6-Mip Depth Pyramid + Bilateral Blur</text>
    
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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 18.6 Pipeline Schematic: Matched rendering comparisons from original sources</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (4 Checkpoints: 94-declare-the-rendering-comparison-contract → 94c-prove-the-original-comparison-and-inspect-differences)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="144" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="261" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="144" y="108" text-anchor="middle" font-size="11" fill="#111111">same original source</text>
    <line x1="271" y1="108" x2="291" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="293" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="303" y1="90" x2="536" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">declared camera and pipeline</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">differences</text>
    <line x1="546" y1="108" x2="566" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="568" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="578" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="694" y="108" text-anchor="middle" font-size="11" fill="#111111">actual maintained frame</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 18.6:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/comparison.rs  ·  src/lib.rs  ·  Cargo.toml  ·  src/reference_example.rs  ·  src/reference_frame.rs  ·  src/comparison_tests.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">4 Checkpoints: 94-declare-the-rendering-comparison-contract → 94c-prove-the-original-comparison-and-inspect-differences — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **WGSL (`@compute`)** | `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>)` | Runs general-purpose parallel GPU threads (outside vertex/fragment rasterization) to project triangles and prefix-sum tile counts. |
| **WGSL (Atomics)** | `atomicAdd(&tile_counts[tile_id], 1u)` | Safely increments shared GPU buffer counters when multiple triangles overlap the same screen tile concurrently. |
| **Math (Winding Cap)** | `signed_crossings != 0` at clip plane depth | Fills only the solid interior of a clipped B-Rep/mesh while leaving interior bores and open shells hollow. |
| **WGSL (`ssao.wgsl`)** | Multi-level depth pyramid horizon integration | Samples 6 mip levels of linear depth and per-object contact radius to shade crevices and ground contact. |

**Authoritative production source (`session_viewer/src`):** [engine/gpu/ssao.rs](../../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../../src/app/command/tool/elements.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`94-declare-the-rendering-comparison-contract`](#checkpoint-94-declare-the-rendering-comparison-contract) | Declare the rendering comparison contract. | `src/comparison.rs`, `src/lib.rs` | `same original source → declared camera and pipeline differences → actual maintained frame` |
| **2** | [`94a-link-the-maintained-native-rendering-reference`](#checkpoint-94a-link-the-maintained-native-rendering-reference) | Link the maintained native rendering reference. | `Cargo.toml` | `same original source → declared camera and pipeline differences → actual maintained frame` |
| **3** | [`94b-read-actual-maintained-frames-without-source-mutation`](#checkpoint-94b-read-actual-maintained-frames-without-source-mutation) | Read actual maintained frames without source mutation. | `src/reference_example.rs`, `Cargo.toml`, `src/reference_frame.rs`, `src/lib.rs` | `same original source → declared camera and pipeline differences → actual maintained frame` |
| **4** | [`94c-prove-the-original-comparison-and-inspect-differences`](#checkpoint-94c-prove-the-original-comparison-and-inspect-differences) | Prove the original comparison and inspect differences. | `src/comparison_tests.rs`, `src/lib.rs`, `index.html` | `same original source → declared camera and pipeline differences → actual maintained frame` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-94-declare-the-rendering-comparison-contract"></a>

### Step 1 of 4 · Declare the rendering comparison contract (`94-declare-the-rendering-comparison-contract`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/4 Visual Trace (94-declare-the-rendering-comparison-contract): Declare the rendering comparison contract</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/comparison.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">same original source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">declared camera and pipeline</text>
    <text x="420" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">differences</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">actual maintained frame</text>
    
</svg>
</div>

- **Goal:** Declare the rendering comparison contract.
- **Data Flow Trace:** `same original source → declared camera and pipeline differences → actual maintained frame`
- **Files You Touch:** `src/comparison.rs` (*Own one original comparison specimen and explicit maintained-renderer differences.*) · `src/lib.rs` (*Register the comparison owner.*)

Declare the rendering comparison contract.

> **Self-Check Question:** *Can two similar screenshots prove equal viewer behavior?*  
> **Answer:** No. Use the same original source and declare units, camera, viewport, pixel density, settings and source revision. Then distinguish source/picking invariants from deliberate image differences.

- **Hands-On Lab Experiment:** Read the camera, surface, ink, ambient and default rows; follow each real maintained owner. Build the four-solid pedestal once and load those exact protobuf bytes into both viewers.

<details>
<summary>Implementation 94-declare-the-rendering-comparison-contract: Declare the rendering comparison contract</summary>

Declare the rendering comparison contract.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

### 1. `src/comparison.rs`

Own one original comparison specimen and explicit maintained-renderer differences.

Create the file and type:

```rust
--8<-- "journey/code/94-declare-the-rendering-comparison-contract-01.rs"
```

### 2. `src/lib.rs`

Register the comparison owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod opacity_command_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/94-declare-the-rendering-comparison-contract-02.rs" skip=40
```

</details>

<a id="checkpoint-94a-link-the-maintained-native-rendering-reference"></a>

### Step 2 of 4 · Link the maintained native rendering reference (`94a-link-the-maintained-native-rendering-reference`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/4 Visual Trace (94a-link-the-maintained-native-rendering-reference): Link the maintained native rendering reference</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: Cargo.toml</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">same original source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">declared camera and pipeline</text>
    <text x="420" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">differences</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">actual maintained frame</text>
    
</svg>
</div>

- **Goal:** Link the maintained native rendering reference.
- **Data Flow Trace:** `same original source → declared camera and pipeline differences → actual maintained frame`
- **Files You Touch:** `Cargo.toml` (*Link the exact maintained viewer from the reconstructed checkpoint.*)

Link the maintained native rendering reference.

> **Self-Check Question:** *Should this chapter copy another version of the production GPU pipeline?*  
> **Answer:** Use the maintained library as a native reference. The WebAssembly teaching application keeps its own earlier renderer; the native comparison executes the actual current shader and pipeline implementation.

- **Hands-On Lab Experiment:** Inspect the optional authoring reference and its recorded camera and environment settings. Compare the supplied reference image with the browser result using the declared viewport and settings. The earlier Fit and maintained corner-fitting camera differ; this comparison does not require running a native program.

<details>
<summary>Implementation 94a-link-the-maintained-native-rendering-reference: Link the maintained native rendering reference</summary>

Link the maintained native rendering reference.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

### 1. `Cargo.toml`

Link the exact maintained viewer from the reconstructed checkpoint.

<details>
<summary>Locate the existing block</summary>

```toml
pollster = "=0.4.0"
log = "=0.4.34"
```

</details>

After that block, type:

```toml
--8<-- "journey/code/94a-link-the-maintained-native-rendering-reference-01.rs" skip=36
```

</details>

<a id="checkpoint-94b-read-actual-maintained-frames-without-source-mutation"></a>

### Step 3 of 4 · Read actual maintained frames without source mutation (`94b-read-actual-maintained-frames-without-source-mutation`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/4 Visual Trace (94b-read-actual-maintained-frames-without-source-mutation): Read actual maintained frames without source mutation</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/reference_example.rs, Cargo.toml, src/reference_frame.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">same original source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">declared camera and pipeline</text>
    <text x="420" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">differences</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">actual maintained frame</text>
    
</svg>
</div>

- **Goal:** Read actual maintained frames without source mutation.
- **Data Flow Trace:** `same original source → declared camera and pipeline differences → actual maintained frame`
- **Files You Touch:** `src/reference_example.rs` (*Copy the supplied maintained-frame command line example.*) · `Cargo.toml` (*Expose the maintained reference as a native example.*) · `src/reference_frame.rs` (*Supply the optional native comparison wrapper; the browser application does not call it.*) · `src/lib.rs` (*Keep the maintained reference native-only.*)

Read the recorded maintained frame using the same original data. The headless PPM wrapper is an optional authoring fixture; continue running your viewer in the browser.

> **Self-Check Question:** *Does an output filename prove a valid rendered image?*  
> **Answer:** Check the exact framebuffer dimensions, P6 header and byte count, then inspect rendered face coverage. A missing or truncated frame fails. Compare the input file bytes before and after rendering.

- **Hands-On Lab Experiment:** Use a 640 by 480 browser viewport. Inspect the source file, supplied maintained frame and declared default differences.

<details>
<summary>Implementation 94b-read-actual-maintained-frames-without-source-mutation: Read actual maintained frames without source mutation</summary>

Read actual maintained frames without source mutation.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 1. `src/reference_example.rs`

Copy the supplied maintained-frame command line example.

Copy this supplied file:

```text
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-01.rs"
```

</details>

### 2. `Cargo.toml`

Expose the maintained reference as a native example.

<details>
<summary>Locate the existing block</summary>

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
current_viewer = { package = "session_viewer", path = "../.." }
```

</details>

After that block, type:

```toml
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-02.rs" skip=121
```

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 3. `src/reference_frame.rs`

Supply the optional native comparison wrapper; the browser application does not call it.

Copy this supplied file:

```text
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-03.rs"
```

</details>

### 4. `src/lib.rs`

Keep the maintained reference native-only.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod comparison;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-04.rs" skip=21
```

</details>

<a id="checkpoint-94c-prove-the-original-comparison-and-inspect-differences"></a>

### Step 4 of 4 · Prove the original comparison and inspect differences (`94c-prove-the-original-comparison-and-inspect-differences`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/4 Visual Trace (94c-prove-the-original-comparison-and-inspect-differences): Prove the original comparison and inspect differences</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/comparison_tests.rs, src/lib.rs, index.html</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">same original source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">declared camera and pipeline</text>
    <text x="420" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">differences</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">actual maintained frame</text>
    
</svg>
</div>

- **Goal:** Prove the original comparison and inspect differences.
- **Data Flow Trace:** `same original source → declared camera and pipeline differences → actual maintained frame`
- **Files You Touch:** `src/comparison_tests.rs` (*Copy supplied actual source and maintained-frame fixtures.*) · `src/lib.rs` (*Register supplied comparison fixtures.*) · `index.html` (*Package the actual original comparison and later metadata protobuf fixtures for browser acceptance.*)

Prove the original comparison and inspect differences.

> **Self-Check Question:** *Should a visual difference be fixed before finding its cause?*  
> **Answer:** First normalize settings and inspect IDs/source ownership. Flat shading, grid, opacity, camera fit and MSAA can explain differences. Investigate missing originals, incorrect picking or broken boundaries independently.

- **What You Should See:** The teaching renderer draws the same four original solids used by the maintained native reference.
- **Hands-On Lab Experiment:** Inspect the supplied original-source and actual maintained GPU fixtures. Inspect the teaching frame, then compare the maintained reference under the same declared settings.

<details>
<summary>Implementation 94c-prove-the-original-comparison-and-inspect-differences: Prove the original comparison and inspect differences</summary>

Prove the original comparison and inspect differences.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/comparison_tests.rs`

Copy supplied actual source and maintained-frame fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register supplied comparison fixtures.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(not(target_arch="wasm32"))]
pub mod reference_frame;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-02.rs" skip=59
```

</details>

### 3. `index.html`

Package the actual original comparison and later metadata protobuf fixtures for browser acceptance.

<details>
<summary>Locate the existing block</summary>

```html
  <link data-trunk rel="copy-dir" href="assets/text" data-target-path="text">
```

</details>

After that block, type:

```html
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-package-original-fixtures.html" skip=78
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 94c-prove-the-original-comparison-and-inspect-differences
npm --prefix ../session_tests run course -- check 94c-prove-the-original-comparison-and-inspect-differences
npm --prefix ../session_tests run course -- save 94c-prove-the-original-comparison-and-inspect-differences
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

The teaching renderer draws the same four original solids used by the maintained native reference.

- **Final Lesson Experiment:** Inspect the supplied original-source and actual maintained GPU fixtures. Inspect the teaching frame, then compare the maintained reference under the same declared settings.

Continue with [Measured frame costs and dependency-aware cache reports](../../consolidation/95-record-comparable-large-scene-conditions.md).
