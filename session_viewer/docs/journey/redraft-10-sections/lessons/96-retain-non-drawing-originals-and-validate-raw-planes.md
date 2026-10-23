# 19.1 · Complete source metadata and the actual command contract

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 19.1** (Redraft `10.1`) · **9 checkpoints (`96-retain-non-drawing-originals-and-validate-raw-planes` → `96h-prove-current-source-grammar-and-explicit-differences`)** · **Study time: 4.5–9 h**

> **What You Will Learn & Build:** Retain non drawing originals and validate raw planes. By the final checkpoint (`96h-prove-current-source-grammar-and-explicit-differences`), you will prove current source grammar and explicit differences.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 10 Visual Explanation: Shared GPU Storage Arena, f64 Floating Origin, Reversed-Z, 4x MSAA & Matrix Shader</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Growable storage arena + vertex pulling. Center: Camera-relative f64→f32 floating origin & Reversed-Z. Right: 4x MSAA + Matrix.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="255" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="145" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Shared GPU Storage Arena</text>
    <rect x="34" y="100" width="222" height="32" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <rect x="34" y="100" width="65" height="32" fill="url(#hatch)" stroke="#111111" stroke-width="1.2"/>
    <rect x="145" y="100" width="55" height="32" fill="url(#hatch)" stroke="#111111" stroke-width="1.2"/>
    <text x="122" y="120" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Free Span</text>
    <text x="145" y="156" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Vertex Pulling (@builtin(vertex_index))</text>
    <text x="145" y="176" text-anchor="middle" font-size="9.5" fill="#111111">8-byte Draw Slots -&gt; ObjectRecord</text>
    <text x="145" y="194" text-anchor="middle" font-size="9.5" fill="#111111">slot_table.wgsl resolves owner IDs</text>

    <line x1="273" y1="154" x2="295" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="297" y="64" width="265" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="429" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. f64 Floating Origin &amp; Reversed-Z</text>
    <rect x="313" y="98" width="233" height="54" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
    <text x="429" y="116" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">rel_f32 = (world_f64 - anchor_f64) as f32</text>
    <text x="429" y="134" text-anchor="middle" font-size="9.5" fill="#111111">Eliminates f32 vertex jitter at 10^12 m!</text>
    <rect x="313" y="162" width="233" height="64" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="429" y="180" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Reversed-Z Depth (GreaterEqual)</text>
    <text x="429" y="198" text-anchor="middle" font-size="9.5" fill="#111111">Near Plane = 1.0  →  Far Plane = 0.0</text>
    <text x="429" y="214" text-anchor="middle" font-size="9" fill="#111111">Preserves live camera across .pb reload</text>

    <line x1="562" y1="154" x2="584" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="586" y="64" width="236" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. 4x MSAA &amp; Matrix Shader</text>
    <rect x="602" y="98" width="204" height="128" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="118" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Coherent 1x / 4x Sample Targets</text>
    <text x="614" y="138" font-size="9.5" fill="#111111">• Color + Depth32Float at 4x MSAA</text>
    <text x="614" y="156" font-size="9.5" fill="#111111">• Rg32Uint Pick Target stays 1x</text>
    <text x="614" y="176" font-size="9.5" font-weight="700" fill="#111111">• matrix.wgsl procedural cells:</text>
    <text x="614" y="192" font-size="9" fill="#111111">  3-decimal digits, 0 blank, plate-less</text>
    <text x="614" y="207" font-size="9" fill="#111111">  world title &amp; yellow selected border</text>
    
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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 19.1 Pipeline Schematic: Complete source metadata and the actual command contract</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (9 Checkpoints: 96-retain-non-drawing-originals-and-validate-raw-planes → 96h-prove-current-source-grammar-and-explicit-differences)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="144" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="261" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="144" y="108" text-anchor="middle" font-size="11" fill="#111111">audit original records and current</text>
    <text x="144" y="124" text-anchor="middle" font-size="11" fill="#111111">source</text>
    <line x1="271" y1="108" x2="291" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="293" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="303" y1="90" x2="536" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">preserve exact metadata</text>
    <line x1="546" y1="108" x2="566" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="568" y="66" width="253" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="578" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="694" y="108" text-anchor="middle" font-size="11" fill="#111111">verify real registry outcomes</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 19.1:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/source_metadata.rs  ·  src/lib.rs  ·  src/definition_merge.rs  ·  src/origin.rs  ·  src/document_source.rs  ·  src/source_validate.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">9 Checkpoints: 96-retain-non-drawing-originals-and-validate-raw-planes → 96h-prove-current-source-grammar-and-explicit-differences — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **WGSL (Vertex Pull)** | `@group(0) @binding(b) var<storage, read> arena_verts: array<u32>;` | Fetches packed vertex positions, normals, and colors directly from a single growable storage buffer using `@builtin(vertex_index)`. |
| **Rust/WGSL (`f64`->`f32`)** | `let rel: Vec3 = (world_f64 - anchor_f64).as_vec3();` | Subtracts the camera-area `f64` anchor on the CPU before converting to `f32`, preventing 32-bit float mantissa exhaustion far from `(0,0,0)`. |
| **WGSL (`matrix.wgsl`)** | 16-word header + 3 words/cell procedural digits | Renders matrix heatmaps and 3-decimal numeric digits procedurally inside the fragment shader with zero texture uploads per cell. |
| **Rust (`ink_ranges.rs`)** | Merged contiguous `Range<u32>` instance draws | Toggles point/line/edge/CAD-wire family visibility without rebuilding GPU buffers or invalidating original owner indices. |

**Authoritative production source (`session_viewer/src`):** [app/session_io.rs](../../../../src/app/session_io.rs), [app/command/mod.rs](../../../../src/app/command/mod.rs), [app/hierarchy.rs](../../../../src/app/hierarchy.rs), [app/scene.rs](../../../../src/app/scene.rs), [app/selection.rs](../../../../src/app/selection.rs), [app/deform.rs](../../../../src/app/deform.rs), [app/walk/mod.rs](../../../../src/app/walk/mod.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`96-retain-non-drawing-originals-and-validate-raw-planes`](#checkpoint-96-retain-non-drawing-originals-and-validate-raw-planes) | Retain non drawing originals and validate raw planes. | `src/source_metadata.rs`, `src/lib.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **2** | [`96a-merge-unused-definitions-with-displayed-instances`](#checkpoint-96a-merge-unused-definitions-with-displayed-instances) | Merge unused definitions with displayed instances. | `src/definition_merge.rs`, `src/lib.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **3** | [`96b-preserve-component-and-interaction-metadata-in-archives`](#checkpoint-96b-preserve-component-and-interaction-metadata-in-archives) | Preserve component and interaction metadata in archives. | `src/origin.rs`, `src/document_source.rs`, `src/source_validate.rs`, `src/archive_document.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **4** | [`96c-keep-current-element-and-frame-definition-families`](#checkpoint-96c-keep-current-element-and-frame-definition-families) | Keep current Element and frame definition families. | `src/instance_plan.rs`, `src/instance_definition.rs`, `src/instance_snapshot.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **5** | [`96d-observe-the-real-current-registry-and-parser`](#checkpoint-96d-observe-the-real-current-registry-and-parser) | Observe the real current registry and parser. | `src/registry_audit.rs`, `src/lib.rs`, `src/inventory_example.rs`, `Cargo.toml` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **6** | [`96e-verify-the-complete-declared-current-source-closure`](#checkpoint-96e-verify-the-complete-declared-current-source-closure) | Verify the complete declared current source closure. | `src/current_inventory.json`, `src/source_audit.rs`, `src/lib.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **7** | [`96f-report-retained-original-metadata-in-the-real-viewer`](#checkpoint-96f-report-retained-original-metadata-in-the-real-viewer) | Report retained original metadata in the real viewer. | `src/browser.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **8** | [`96g-prove-exact-metadata-and-definition-round-trips`](#checkpoint-96g-prove-exact-metadata-and-definition-round-trips) | Prove exact metadata and definition round trips. | `src/metadata_tests.rs`, `src/lib.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |
| **9** | [`96h-prove-current-source-grammar-and-explicit-differences`](#checkpoint-96h-prove-current-source-grammar-and-explicit-differences) | Prove current source grammar and explicit differences. | `src/current_registry.json`, `src/inventory_tests.rs`, `src/lib.rs` | `audit original records and current source → preserve exact metadata → verify real registry outcomes` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-96-retain-non-drawing-originals-and-validate-raw-planes"></a>

### Step 1 of 9 · Retain non drawing originals and validate raw planes (`96-retain-non-drawing-originals-and-validate-raw-planes`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/9 Visual Trace (96-retain-non-drawing-originals-and-validate-raw-planes): Retain non drawing originals and validate raw planes</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/source_metadata.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Retain non drawing originals and validate raw planes.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/source_metadata.rs` (*Own exact non drawing source metadata and pre-kernel plane validation.*) · `src/lib.rs` (*Register the completed audit owner.*)

Retain non drawing originals and validate raw planes.

> **Self-Check Question:** *Can a non drawing record be discarded during Save?*  
> **Answer:** Its component JSON, unknown interaction state and unused definitions still belong to the document. Retain their exact original records. Validate all twelve raw plane doubles before kernel defaults can replace malformed data.

- **Hands-On Lab Experiment:** Inspect the component-only header and unused-definition selection. Compare retained serialized metadata size with actual heap/GPU memory; they are different measurements.

<details>
<summary>Implementation 96-retain-non-drawing-originals-and-validate-raw-planes: Retain non drawing originals and validate raw planes</summary>

Retain non drawing originals and validate raw planes.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/source_metadata.rs`

Own exact non drawing source metadata and pre-kernel plane validation.

Create the file and type:

```rust
--8<-- "journey/code/96-retain-non-drawing-originals-and-validate-raw-planes-01.rs"
```

### 2. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod bench_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96-retain-non-drawing-originals-and-validate-raw-planes-02.rs" skip=30
```

</details>

<a id="checkpoint-96a-merge-unused-definitions-with-displayed-instances"></a>

### Step 2 of 9 · Merge unused definitions with displayed instances (`96a-merge-unused-definitions-with-displayed-instances`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/9 Visual Trace (96a-merge-unused-definitions-with-displayed-instances): Merge unused definitions with displayed instances</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/definition_merge.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Merge unused definitions with displayed instances.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/definition_merge.rs` (*Merge exact retained definitions without losing unused originals.*) · `src/lib.rs` (*Register the completed audit owner.*)

Merge unused definitions with displayed instances.

> **Self-Check Question:** *Does saving only displayed instance definitions preserve the whole document?*  
> **Answer:** Keep the unused originals too. Merge by original family and GUID, sort deterministically and reject conflicting payloads instead of choosing one silently.

- **Hands-On Lab Experiment:** Compare a used definition, an unused definition and two different originals sharing one GUID.

<details>
<summary>Implementation 96a-merge-unused-definitions-with-displayed-instances: Merge unused definitions with displayed instances</summary>

Merge unused definitions with displayed instances.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/definition_merge.rs`

Merge exact retained definitions without losing unused originals.

Create the file and type:

```rust
--8<-- "journey/code/96a-merge-unused-definitions-with-displayed-instances-01.rs"
```

### 2. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod source_metadata;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96a-merge-unused-definitions-with-displayed-instances-02.rs" skip=26
```

</details>

<a id="checkpoint-96b-preserve-component-and-interaction-metadata-in-archives"></a>

### Step 3 of 9 · Preserve component and interaction metadata in archives (`96b-preserve-component-and-interaction-metadata-in-archives`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/9 Visual Trace (96b-preserve-component-and-interaction-metadata-in-archives): Preserve component and interaction metadata in archives</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/origin.rs, src/document_source.rs, src/source_validate.rs, src/archive_document.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Preserve component and interaction metadata in archives.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/origin.rs` (*Connect exact metadata retention, raw validation and complete archive reconstruction.*) · `src/document_source.rs` (*Connect exact metadata retention, raw validation and complete archive reconstruction.*) · `src/source_validate.rs` (*Connect exact metadata retention, raw validation and complete archive reconstruction.*) · `src/archive_document.rs` (*Connect exact metadata retention, raw validation and complete archive reconstruction.*)

Preserve component and interaction metadata in archives.

> **Self-Check Question:** *Must component metadata become a selectable mesh?*  
> **Answer:** Keep it as original document metadata. Preserve its exact JSON text and interaction bytes in retained headers, then restore them beside the current drawable originals when saving.

- **Hands-On Lab Experiment:** Import the supplied four-solid/component/unknown-interaction fixture, Save/Open twice and compare original metadata. Original header accounting includes the serialized retained records.

<details>
<summary>Implementation 96b-preserve-component-and-interaction-metadata-in-archives: Preserve component and interaction metadata in archives</summary>

Preserve component and interaction metadata in archives.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/origin.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

<details>
<summary>Locate the existing block</summary>

```rust
        let header = proto::Session { name: message.name.clone(), guid: message.guid.clone(),
            tree: message.tree.clone(), graph: message.graph.clone(),
            bvh_boxes: message.bvh_boxes.clone(), xforms: message.xforms.clone(), ..Default::default() };
        Self { id: uuid::Uuid::new_v4(), header, version: FileVersion::of(bytes), location: None }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-01.rs"
```

### 2. `src/document_source.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String> {
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    let origin=Rc::new(crate::origin::Origin::new(&message,bytes));
    let document=Rc::new(Session::from_proto(message).map_err(|e|e.to_string())?);
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    if !source.interactions.is_empty() {return Err("Interactions arrive with element features".into());}
    let objects=source.objects.as_ref().ok_or("Session has no objects")?;
    if !objects.components.is_empty() {return Err("This subject cannot open assemblies yet".into());}
    Ok(())
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{proto,Session,Xform};
use prost::Message;
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) struct Source {pub document:Rc<Session>,pub origin:Rc<crate::origin::Origin>,pub tree:Option<proto::TreeNode>,pub world:HashMap<String,Xform>,pub sheets:Vec<Rc<crate::sheet_source::Prepared>>}
pub(crate) fn prepare(bytes:&[u8],place:&Xform,point_px:f32)->Result<Source,String> {
    if bytes.len()>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
    let message=proto::Session::decode(bytes).map_err(|e|e.to_string())?;
    crate::source_validate::session(&message)?;
    supported(&message)?;
    if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size".into());}
    let tree=message.tree.as_ref().and_then(|tree|tree.root.clone());
    let mut guids=HashSet::new();let objects=message.objects.as_ref().ok_or("Session has no objects")?;
    macro_rules! identity {($field:ident)=>{for object in &objects.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Source GUIDs must be distinct within a document".into());}}};}
    identity!(components);identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);identity!(instances);identity!(sheets);
    if let Some(root)=tree.as_ref() {crate::hierarchy::validate(root,&guids).map_err(str::to_owned)?;}
    let mut placed=HashSet::new();for entry in &message.xforms {if !placed.insert(entry.guid.as_str())||!crate::placement::valid(&entry.xform.as_ref().ok_or("Missing placement matrix")?.matrix) {return Err("Invalid source placement".into());}}
    let sheets=objects.sheets.iter().cloned().map(|sheet|crate::sheet_source::Prepared::new(Rc::new(sheet)).map(Rc::new).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
    let origin=Rc::new(crate::origin::Origin::new(&message,bytes));
    let document=Rc::new(Session::from_proto(message).map_err(|e|e.to_string())?);
    let world=crate::nested_placement::prepare(&document,place).map_err(str::to_owned)?;
    Ok(Source {document,origin,tree,world,sheets})
}

fn supported(source:&proto::Session)->Result<(),String> {
    crate::instance_plan::validate(source)?;
    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-02.rs"
```

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-03.rs"
```

</details>

### 3. `src/source_validate.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {if !crate::plane_original::valid(&session_rust::Plane::from_proto(item.clone())) {return Err("Invalid original plane frame".into())}}
        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
```

</details>

Replace it with:

```rust
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len()+objects.components.len()+objects.sheets.len()+objects.instances.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {crate::source_metadata::raw_plane(item)?;}
        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
```

<details>
<summary>Complete updated block reference</summary>

```rust
use prost::Message;
use session_rust::proto;

/// Source objects a scene may hold.
pub const MAX_OBJECTS: usize = 2_000_000;

/// Why a scene with more objects is refused.
pub const TOO_MANY: &str = "scene exceeds two million source objects";

/// Deepest tree level a scene may have.
pub const MAX_DEPTH: usize = 64;

/// Why a scene with a deeper tree is refused.
pub const TOO_DEEP: &str = "scene hierarchy exceeds 64 levels";

/// Every value must be finite.
fn finite(values: &[f64], label: &str) -> Result<(), String> {
    for value in values {
        if !value.is_finite() {
            return Err(format!("{label}: non-finite value"));
        }
    }

    Ok(())
}

/// A finite array of whole xyz triples.
fn triples(values: &[f64], label: &str) -> Result<(), String> {
    if !values.len().is_multiple_of(3) {
        return Err(format!("{label}: incomplete coordinate triple"));
    }

    finite(values, label)
}

/// A protobuf decode error as a message.
fn decode_error(error: prost::DecodeError) -> String {
    format!("invalid element geometry: {error}")
}

/// The varint at `i` and its byte length.
pub fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;

    loop {
        let byte = *b.get(i)?;

        if shift == 63 && byte > 1 {
            return None;
        }

        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;

        if byte & 0x80 == 0 {
            return Some((v, i - start));
        }

        shift += 7;

        if shift > 63 {
            return None;
        }
    }
}

/// The highest control index a surface reads, None on overflow.
fn addressed_surface_controls(
    rows: usize,
    columns: usize,
    stride_u: usize,
    stride_v: usize,
    size: usize,
) -> Option<usize> {
    (rows - 1)
        .checked_mul(stride_u)?
        .checked_add((columns - 1).checked_mul(stride_v)?)?
        .checked_add(size)
}

/// Check order, count and knots; returns the knot count.
fn axis(order: i32, count: i32, knots: &[f64]) -> Result<usize, String> {
    if !(2..=64).contains(&order) || count < order || count > 1_000_000 {
        return Err("invalid or oversized NURBS order/control count".into());
    }

    if knots.len() != (order + count - 2) as usize {
        return Err("NURBS knot count does not match its order and controls".into());
    }

    finite(knots, "NURBS knots")?;

    for pair in knots.windows(2) {
        if pair[0] > pair[1] {
            return Err("NURBS knots are not monotonic".into());
        }
    }

    if knots[(order - 2) as usize] >= knots[(count - 1) as usize] {
        return Err("NURBS parameter domain is empty".into());
    }

    Ok(count as usize)
}

/// A JSON parse error as a message.
fn json_error(error: serde_json::Error) -> String {
    format!("invalid session JSON: {error}")
}

/// A count field, defaulted and capped.
fn json_count(
    source: &serde_json::Value,
    key: &str,
    default: u64,
    maximum: u64,
) -> Result<usize, String> {
    let count = match source.get(key) {
        Some(value) => match value.as_u64() {
            Some(value) => value,
            None => return Err(format!("invalid JSON {key}")),
        },
        None => default,
    };

    if count > maximum {
        return Err(format!("JSON {key} exceeds its supported count"));
    }

    Ok(count as usize)
}

/// Check one JSON knot vector.
fn json_axis(
    source: &serde_json::Value,
    order_key: &str,
    count: usize,
    knots_key: &str,
) -> Result<(), String> {
    let order = json_count(source, order_key, 4, 64)?;
    let values = source[knots_key]
        .as_array()
        .ok_or("missing JSON NURBS knots")?;
    let mut knots = Vec::with_capacity(values.len());

    for value in values {
        knots.push(value.as_f64().ok_or("invalid JSON NURBS knot")?);
    }

    axis(order as i32, count as i32, &knots)?;
    Ok(())
}

/// Check one JSON curve or surface's controls against its counts.
fn json_controls(source: &serde_json::Value) -> Result<(), String> {
    let controls = source["control_points"]
        .as_array()
        .ok_or("JSON controls must be an array")?;
    let dimension = json_count(source, "dimension", 3, 3)?;

    if dimension < 2 {
        return Err("invalid JSON control dimension".into());
    }

    let size = dimension + usize::from(source["is_rational"].as_bool().unwrap_or(false));

    if source.get("cv_count_u").is_some() {
        let rows = json_count(source, "cv_count_u", 0, 1_000_000)?;
        let columns = json_count(source, "cv_count_v", 0, 1_000_000)?;
        let total = rows as u64 * columns as u64;

        if total > 4_000_000 || controls.len() as u64 != total * size as u64 {
            return Err("JSON surface controls disagree with their declared counts".into());
        }

        json_axis(source, "order_u", rows, "nurbsknots_u")?;
        json_axis(source, "order_v", columns, "nurbsknots_v")?;
    } else {
        let count = json_count(source, "cv_count", 0, 1_000_000)?;
        let stride = json_count(source, "cv_stride", size as u64, 4)?;

        if count != controls.len() || stride != size {
            return Err("JSON curve controls disagree with their declared count or stride".into());
        }

        for control in controls {
            if control.as_array().map(Vec::len) != Some(size) {
                return Err("incomplete JSON curve control".into());
            }
        }

        json_axis(source, "order", count, "nurbsknots")?;
    }

    Ok(())
}

/// Check every NURBS record in session JSON before it is parsed.
pub fn json(text: &str) -> Result<(), String> {
    let source: serde_json::Value = serde_json::from_str(text).map_err(json_error)?;
    let mut pending = vec![(&source, 0usize)];

    while let Some((value, depth)) = pending.pop() {
        if depth > 64 {
            return Err("session JSON exceeds 64 nested levels".into());
        }

        match value {
            serde_json::Value::Object(fields) => {
                if fields.contains_key("control_points")
                    && (fields.contains_key("cv_count") || fields.contains_key("cv_count_u"))
                {
                    json_controls(value)?;
                }

                for child in fields.values() {
                    pending.push((child, depth + 1));
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    pending.push((child, depth + 1));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/// Check one protobuf point.
pub fn point(source: &proto::Point) -> Result<(), String> {
    finite(&[source.x, source.y, source.z, source.width], "point")
}

/// Check one protobuf line.
pub fn line(source: &proto::Line) -> Result<(), String> {
    if source.coords.len() != 6 {
        return Err("line needs exactly two endpoint triples".into());
    }

    finite(&source.coords, "line coordinates")?;
    finite(&source.dash, "line dashes")
}

/// Check one protobuf polyline.
pub fn polyline(source: &proto::Polyline) -> Result<(), String> {
    triples(&source.coords, "polyline coordinates")
}

/// Check one object placement.
pub fn xform(entry: &proto::XformEntry) -> Result<(), String> {
    let Some(transform) = &entry.xform else {
        return Err("missing object transform".into());
    };
    finite(&transform.matrix, "object transform")?;

    if transform.matrix.len() != 16
        || transform.matrix[3] != 0.0
        || transform.matrix[7] != 0.0
        || transform.matrix[11] != 0.0
        || transform.matrix[15] != 1.0
    {
        return Err("object placement must be a sixteen-value affine matrix".into());
    }

    Ok(())
}

/// Nothing to check.
pub fn any<T>(_: &T) -> Result<(), String> {
    Ok(())
}

/// Every index must name a vertex.
fn mesh_indices(source: &proto::Mesh, indices: &[u64]) -> Result<(), String> {
    for index in indices {
        if !source.vertices.contains_key(index) {
            return Err(format!("mesh references missing vertex {index}"));
        }
    }

    Ok(())
}

/// Check one protobuf mesh.
pub fn mesh(source: &proto::Mesh) -> Result<(), String> {
    for (key, vertex) in &source.vertices {
        if *key > u32::MAX as u64 {
            return Err("mesh vertex key exceeds the browser index range".into());
        }

        finite(&[vertex.x, vertex.y, vertex.z], "mesh vertex")?;

        for value in vertex.attributes.values() {
            if !value.is_finite() {
                return Err("mesh vertex has a non-finite attribute".into());
            }
        }
    }

    for face in source.faces.values() {
        if face.vertices.len() < 3 {
            return Err("mesh face has fewer than three vertices".into());
        }

        mesh_indices(source, &face.vertices)?;

        for hole in &face.holes {
            mesh_indices(source, &hole.vertices)?;
        }
    }

    for (face, triangles) in &source.triangulation {
        if !source.faces.contains_key(face) || !triangles.vertices.len().is_multiple_of(3) {
            return Err("invalid mesh triangulation record".into());
        }

        mesh_indices(source, &triangles.vertices)?;
    }

    Ok(())
}

/// Check one protobuf curve.
pub fn curve(source: &proto::NurbsCurve) -> Result<(), String> {
    if !(2..=3).contains(&source.dimension) {
        return Err("NURBS curve dimension must be 2 or 3".into());
    }

    let count = axis(source.order, source.cv_count, &source.nurbsknots)?;
    let size = source.dimension as usize + usize::from(source.is_rational);

    if source.cvs.len() != count * size {
        return Err("NURBS curve control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS curve controls")
}

/// Check one protobuf surface.
pub fn surface(source: &proto::NurbsSurface) -> Result<(), String> {
    if source.dimension != 3 {
        return Err("NURBS surface dimension must be 3".into());
    }

    let rows = axis(source.order_u, source.cv_count_u, &source.nurbsknots_u)?;
    let columns = axis(source.order_v, source.cv_count_v, &source.nurbsknots_v)?;

    if rows.saturating_mul(columns) > 4_000_000 {
        return Err("surface exceeds four million controls".into());
    }

    let size = 3 + usize::from(source.is_rational);
    let stride_u = if source.cv_stride_u > 0 {
        source.cv_stride_u as usize
    } else {
        columns * size
    };
    let stride_v = if source.cv_stride_v > 0 {
        source.cv_stride_v as usize
    } else {
        size
    };
    let addressed = addressed_surface_controls(rows, columns, stride_u, stride_v, size);

    if !matches!(addressed, Some(length) if length <= source.cvs.len()) {
        return Err("NURBS surface control storage is incomplete".into());
    }

    finite(&source.cvs, "NURBS surface controls")?;

    if let Some(cached) = &source.cached_mesh {
        mesh(cached)?;
    }

    Ok(())
}

/// Check one protobuf point cloud.
pub fn cloud(source: &proto::PointCloud) -> Result<(), String> {
    triples(&source.coords, "point cloud")?;
    finite(&source.normals, "point normals")?;
    let points = source.coords.len() / 3;

    if (!source.normals.is_empty() && source.normals.len() != source.coords.len())
        || (!source.colors.is_empty() && source.colors.len() != points * 4)
        || (!source.point_ids.is_empty() && source.point_ids.len() != points)
    {
        return Err("point-cloud attribute counts disagree".into());
    }

    let nodes = source.lod_size.len();

    if nodes > 0 {
        if source.lod_min.len() != nodes * 3
            || source.lod_spacing.len() != nodes
            || source.lod_level.len() != nodes
            || source.lod_first.len() != nodes
            || source.lod_count.len() != nodes
            || source.lod_children.len() != nodes * 8
        {
            return Err("point-cloud octree arrays disagree".into());
        }

        finite(&source.lod_min, "octree bounds")?;
        finite(&source.lod_size, "octree sizes")?;

        for node in 0..nodes {
            if source.lod_first[node] < 0
                || source.lod_count[node] < 0
                || source.lod_first[node] as u64 + source.lod_count[node] as u64 > points as u64
            {
                return Err("octree range exceeds its source points".into());
            }
        }

        for child in &source.lod_children {
            if *child < -1 || *child >= nodes as i32 {
                return Err("octree references an absent child".into());
            }
        }
    }

    Ok(())
}

/// Check one protobuf BRep.
pub fn brep(source: &proto::BRep) -> Result<(), String> {
    for item in &source.curves_2d {
        curve(item)?;
    }

    for item in &source.curves_3d {
        curve(item)?;
    }

    for item in &source.surfaces {
        surface(item)?;
    }

    for vertex in &source.vertices {
        if let Some(point) = &vertex.point {
            finite(&[point.x, point.y, point.z], "BRep vertex")?;
        }
    }

    Ok(())
}

/// Check one protobuf element's mesh or BRep.
pub fn element(source: &proto::Element) -> Result<(), String> {
    match source.geometry_type.as_str() {
        "Mesh" => {
            mesh(&proto::Mesh::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        "BRep" => {
            brep(&proto::BRep::decode(source.geometry_data.as_slice()).map_err(decode_error)?)
        }
        _ => Ok(()),
    }
}

/// Check every definition record, and the definitions with `instances` within the cap.
pub fn definitions(d: &proto::Objects, instances: usize) -> Result<(), String> {
    let count = d.points.len()
        + d.lines.len()
        + d.polylines.len()
        + d.meshes.len()
        + d.pointclouds.len()
        + d.nurbscurves.len()
        + d.nurbssurfaces.len()
        + d.breps.len()
        + d.elements.len();

    if count + instances > MAX_OBJECTS {
        return Err(TOO_MANY.into());
    }

    for item in &d.points {
        point(item)?;
    }

    for item in &d.lines {
        line(item)?;
    }

    for item in &d.polylines {
        polyline(item)?;
    }

    for item in &d.meshes {
        mesh(item)?;
    }

    for item in &d.pointclouds {
        cloud(item)?;
    }

    for item in &d.nurbscurves {
        curve(item)?;
    }

    for item in &d.nurbssurfaces {
        surface(item)?;
    }

    for item in &d.breps {
        brep(item)?;
    }

    for item in &d.elements {
        element(item)?;
    }

    Ok(())
}

/// Check every record of a protobuf session.
pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.planes.len()+objects.bboxes.len()+objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len()+objects.components.len()+objects.sheets.len()+objects.instances.len();

        if count > MAX_OBJECTS {
            return Err(TOO_MANY.into());
        }

        for item in &objects.planes {crate::source_metadata::raw_plane(item)?;}
        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
            point(item)?;
        }

        for item in &objects.lines {
            line(item)?;
        }

        for item in &objects.polylines {
            polyline(item)?;
        }

        for item in &objects.meshes {
            mesh(item)?;
        }

        for item in &objects.pointclouds {
            cloud(item)?;
        }

        for item in &objects.nurbscurves {
            curve(item)?;
        }

        for item in &objects.nurbssurfaces {
            surface(item)?;
        }

        for item in &objects.breps {
            brep(item)?;
        }

        for item in &objects.elements {
            element(item)?;
        }
    }

    for entry in &source.xforms {
        xform(entry)?;
    }

    if let Some(tree) = &source.tree
        && let Some(root) = &tree.root
    {
        let mut pending = vec![(root, 0usize)];

        while let Some((node, depth)) = pending.pop() {
            if depth > MAX_DEPTH {
                return Err(TOO_DEEP.into());
            }

            for child in &node.children {
                pending.push((child, depth + 1));
            }
        }
    }

    Ok(())
}

/// Check every geometry of a loaded session.
pub fn retained(source: &session_rust::Session) -> Result<(), String> {
    for item in &source.objects.meshes {
        mesh(&item.to_proto())?;
    }

    for item in &source.objects.pointclouds {
        cloud(&item.to_proto())?;
    }

    for item in &source.objects.nurbscurves {
        curve(&item.to_proto())?;
    }

    for item in &source.objects.nurbssurfaces {
        surface(&item.to_proto())?;
    }

    for item in &source.objects.breps {
        brep(&item.to_proto())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Huge or wrong counts are refused.
    #[test]
    fn hostile_counts_do_not_reach_kernel_allocations() {
        let source = proto::NurbsCurve {
            dimension: 3,
            order: 2,
            cv_count: i32::MAX,
            ..Default::default()
        };
        assert!(curve(&source).is_err());
        let source = proto::NurbsSurface {
            dimension: 3,
            order_u: 2,
            order_v: 2,
            cv_count_u: -1,
            cv_count_v: 2,
            ..Default::default()
        };
        assert!(surface(&source).is_err());
    }

    /// Bad indices and short cloud arrays are refused.
    #[test]
    fn missing_indices_and_partial_cloud_attributes_are_errors() {
        let mut source = proto::Mesh::default();
        source.faces.insert(
            0,
            proto::FaceData {
                vertices: vec![0, 1, 2],
                ..Default::default()
            },
        );
        assert!(mesh(&source).is_err());
        let cloud_source = proto::PointCloud {
            coords: vec![0., 0., 0.],
            point_ids: vec![7, 8],
            ..Default::default()
        };
        assert!(cloud(&cloud_source).is_err());
    }

    /// JSON counts must match the controls given.
    #[test]
    fn json_declared_controls_are_checked_before_constructor_allocation() {
        assert!(
            json(r#"{"control_points":[],"cv_count":18446744073709551615,"dimension":3}"#).is_err()
        );
        assert!(
            json(r#"{"control_points":[],"cv_count_u":1000000,"cv_count_v":1000000}"#).is_err()
        );
        assert!(json(r#"{"control_points":[[0,0,0],[1,0,0]],"cv_count":2,"dimension":3,"order":2,"nurbsknots":[0,1]}"#).is_ok());
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-04.rs"
```

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-05.rs"
```

</details>

### 4. `src/archive_document.rs`

Connect exact metadata retention, raw validation and complete archive reconstruction.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{scene::Scene,scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap,HashSet};

pub(crate) fn prepare(scene:&Scene,namespace:&Namespace)->Result<(proto::Session,HashMap<String,String>,f32),String> {
    let names=crate::archive_identity::aliases(scene,namespace)?;let mut objects=crate::geometry_snapshot::objects(scene,&namespace.ids)?;crate::archive_identity::objects(&mut objects,&names);
    let mut xforms=Vec::new();let (mut roots,placed)=if let Some(doc)=namespace.doc.as_ref() {let mut forest=crate::hierarchy::Forest::default();forest.documents.push(doc.clone());forest.snapshot(scene,&mut xforms).map_err(str::to_owned)?} else {(vec![],HashSet::new())};
    for id in namespace.ordered() {
        let current=scene.row_guid(id).ok_or("Saved source owner is missing")?;let guid=names.get(current).cloned().unwrap_or_else(||current.into());
        if !placed.contains(&id) {roots.push(proto::TreeNode {name:guid.clone(),..Default::default()});let model=scene.placement(id).ok_or("Saved source placement is missing")?;if model.m!=session_rust::Xform::identity().m {xforms.push(proto::XformEntry {guid,xform:Some(model.to_proto())});}}
    }
    for root in &mut roots {crate::archive_identity::tree(root,&names);}for entry in &mut xforms {if let Some(guid)=names.get(&entry.guid) {entry.guid=guid.clone();}}
    let mut source=namespace.doc.as_ref().and_then(|doc|doc.header.as_ref()).map_or_else(proto::Session::default,|header|header.header.clone());if source.name.is_empty() {source.name=namespace.name.clone();}
    if let Some(doc)=namespace.doc.as_ref(){source.graph=crate::graph_edit::saved(scene,doc,&names);}
    let root=if namespace.doc.is_some()&&roots.len()==1 {roots.pop().unwrap()} else {proto::TreeNode {name:source.name.clone(),children:roots,..Default::default()}};
    source.definitions=crate::instance_snapshot::definitions(scene,&namespace.ids)?;source.objects=Some(objects);source.tree=Some(proto::Tree {root:Some(root),..Default::default()});source.xforms=xforms;source.bvh_boxes.clear();
    let mut clouds=scene.clouds().iter().filter(|row|namespace.ids.contains(&row.id));let point_px=clouds.next().map_or(0.,|row|row.point_px);
    if clouds.any(|row|row.point_px!=point_px) {return Err("Cloud placements in one saved document need the same point size override".into());}Ok((source,names,point_px))
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{scene::Scene,scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap,HashSet};

pub(crate) fn prepare(scene:&Scene,namespace:&Namespace)->Result<(proto::Session,HashMap<String,String>,f32),String> {
        let names=crate::archive_identity::aliases(scene,namespace)?;let mut objects=crate::geometry_snapshot::objects(scene,&namespace.ids)?;crate::archive_identity::objects(&mut objects,&names);
    let mut xforms=Vec::new();let (mut roots,placed)=if let Some(doc)=namespace.doc.as_ref() {let mut forest=crate::hierarchy::Forest::default();forest.documents.push(doc.clone());forest.snapshot(scene,&mut xforms).map_err(str::to_owned)?} else {(vec![],HashSet::new())};
    for id in namespace.ordered() {
        let current=scene.row_guid(id).ok_or("Saved source owner is missing")?;let guid=names.get(current).cloned().unwrap_or_else(||current.into());
        if !placed.contains(&id) {roots.push(proto::TreeNode {name:guid.clone(),..Default::default()});let model=scene.placement(id).ok_or("Saved source placement is missing")?;if model.m!=session_rust::Xform::identity().m {xforms.push(proto::XformEntry {guid,xform:Some(model.to_proto())});}}
    }
    for root in &mut roots {crate::archive_identity::tree(root,&names);}for entry in &mut xforms {if let Some(guid)=names.get(&entry.guid) {entry.guid=guid.clone();}}
    let mut source=namespace.doc.as_ref().and_then(|doc|doc.header.as_ref()).map_or_else(proto::Session::default,|header|header.header.clone());if source.name.is_empty() {source.name=namespace.name.clone();}
    if let Some(doc)=namespace.doc.as_ref(){source.graph=crate::graph_edit::saved(scene,doc,&names);}
    let root=if namespace.doc.is_some()&&roots.len()==1 {roots.pop().unwrap()} else {proto::TreeNode {name:source.name.clone(),children:roots,..Default::default()}};
    if let Some(metadata)=source.objects.as_ref(){objects.components=metadata.components.clone();}
    source.definitions=crate::definition_merge::merge(source.definitions.as_ref(),crate::instance_snapshot::definitions(scene,&namespace.ids)?)?;source.objects=Some(objects);source.tree=Some(proto::Tree {root:Some(root),..Default::default()});source.xforms=xforms;source.bvh_boxes.clear();
    let mut clouds=scene.clouds().iter().filter(|row|namespace.ids.contains(&row.id));let point_px=clouds.next().map_or(0.,|row|row.point_px);
    if clouds.any(|row|row.point_px!=point_px) {return Err("Cloud placements in one saved document need the same point size override".into());}Ok((source,names,point_px))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-06.rs"
```

```text
--8<-- "journey/code/96b-preserve-component-and-interaction-metadata-in-archives-07.rs"
```

</details>

</details>

<a id="checkpoint-96c-keep-current-element-and-frame-definition-families"></a>

### Step 4 of 9 · Keep current Element and frame definition families (`96c-keep-current-element-and-frame-definition-families`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/9 Visual Trace (96c-keep-current-element-and-frame-definition-families): Keep current Element and frame definition families</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/instance_plan.rs, src/instance_definition.rs, src/instance_snapshot.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Keep current Element and frame definition families.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/instance_plan.rs` (*Retain supported original definition families and per-instance feature records.*) · `src/instance_definition.rs` (*Retain supported original definition families and per-instance feature records.*) · `src/instance_snapshot.rs` (*Retain supported original definition families and per-instance feature records.*)

Keep current Element and frame definition families.

> **Self-Check Question:** *Can an instance of an Element be flattened into a mesh for Save?*  
> **Answer:** Retain its typed Element definition and its own instance-feature records. Plane and box definitions retain their original families too. Derived display does not replace those records.

- **Hands-On Lab Experiment:** Inspect current instance originals, shared templates, exact definition payloads and feature records. Use the maintained destination for its complete derived instance-feature display.

<details>
<summary>Implementation 96c-keep-current-element-and-frame-definition-families: Keep current Element and frame definition families</summary>

Keep current Element and frame definition families.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/instance_plan.rs`

Retain supported original definition families and per-instance feature records.

<details>
<summary>Locate the existing block</summary>

```rust
use session_rust::{proto,Geometry,InstanceRef,Session};
use std::{collections::HashSet,rc::Rc};

pub fn validate(message:&proto::Session)->Result<(),String> {
    let instances=message.objects.as_ref().map_or(&[][..],|objects|objects.instances.as_slice());
    let Some(definitions)=message.definitions.as_ref() else {return if instances.is_empty() {Ok(())} else {Err("Instances have no original definitions".into())};};
    crate::source_validate::definitions(definitions,instances.len())?;
    if !definitions.planes.is_empty()||!definitions.bboxes.is_empty()||!definitions.elements.is_empty()||!definitions.components.is_empty()||!definitions.sheets.is_empty()||!definitions.instances.is_empty() {return Err("Unsupported original definition family".into());}
    let mut guids=HashSet::new();
    macro_rules! identity {($field:ident)=>{for object in &definitions.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Definition GUIDs must be distinct".into());}}};}
    identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);
    let mut refs=HashSet::new();
    for instance in instances {
        if instance.guid.is_empty()||!refs.insert(instance.guid.as_str())||!guids.contains(instance.definition_guid.as_str()) {return Err("Invalid original instance reference".into());}
        if instance.flags&InstanceRef::FLAG_COLOR!=0 {
            let color=instance.color.as_ref().ok_or("Instance colour flag has no original colour")?;
            if [color.r,color.g,color.b,color.a].iter().any(|value|!value.is_finite()||!(0.0..=1.0).contains(value)) {return Err("Invalid original instance colour".into());}
        }
        if !instance.features.is_empty() {return Err("Instance features connect with element features".into());}
    }
    Ok(())
}

#[derive(Clone)]
pub struct Owner {pub document:Rc<Session>,pub instance:Rc<InstanceRef>,pub definition:Geometry}
impl Owner {
    pub fn new(document:Rc<Session>,instance:Rc<InstanceRef>)->Result<Self,&'static str> {
        let definition=document.definition_lookup.get(&instance.definition_guid).ok_or("Missing original instance definition")?.clone();
        Ok(Self {document,instance,definition})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn message()->proto::Session {
        let mesh=crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.3,0.8]).unwrap().geometry;
        proto::Session {objects:Some(proto::Objects {instances:vec![proto::InstanceRef {guid:"first".into(),name:"First".into(),definition_guid:mesh.guid().into(),..Default::default()},proto::InstanceRef {guid:"second".into(),name:"Second".into(),definition_guid:mesh.guid().into(),..Default::default()}],..Default::default()}),definitions:Some(proto::Objects {meshes:vec![mesh.to_proto()],..Default::default()}),..Default::default()}
    }
    #[test]
    fn checked_original_references_share_definition_identity_without_copying_geometry() {
        let message=message();validate(&message).unwrap();let document=Rc::new(Session::from_proto(message).unwrap());let first=Owner::new(document.clone(),document.objects.instances[0].clone()).unwrap();let second=Owner::new(document.clone(),document.objects.instances[1].clone()).unwrap();let (Geometry::Mesh(a),Geometry::Mesh(b))=(&first.definition,&second.definition) else {panic!("Original mesh definition")};assert!(Rc::ptr_eq(a,b));assert_eq!(first.instance.guid(),"first");assert_eq!(second.instance.guid(),"second");let weak=Rc::downgrade(a);drop(document);drop(first);assert!(weak.upgrade().is_some());drop(second);assert!(weak.upgrade().is_none());
    }
    #[test]
    fn missing_duplicate_and_unsupported_original_definitions_are_refused() {
        let mut source=message();source.definitions=None;assert!(validate(&source).is_err());let mut source=message();source.objects.as_mut().unwrap().instances[0].definition_guid="missing".into();assert!(validate(&source).is_err());let mut source=message();let duplicate=source.definitions.as_ref().unwrap().meshes[0].clone();source.definitions.as_mut().unwrap().meshes.push(duplicate);assert!(validate(&source).is_err());let mut source=message();source.definitions.as_mut().unwrap().instances.push(Default::default());assert!(validate(&source).is_err());
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{proto,Geometry,InstanceRef,Session};
use std::{collections::HashSet,rc::Rc};

pub fn validate(message:&proto::Session)->Result<(),String> {
    let instances=message.objects.as_ref().map_or(&[][..],|objects|objects.instances.as_slice());
    let Some(definitions)=message.definitions.as_ref() else {return if instances.is_empty() {Ok(())} else {Err("Instances have no original definitions".into())};};
    crate::source_validate::definitions(definitions,instances.len())?;
    if !definitions.components.is_empty()||!definitions.sheets.is_empty()||!definitions.instances.is_empty() {return Err("Unsupported original definition family".into());}
    let mut guids=HashSet::new();
    macro_rules! identity {($field:ident)=>{for object in &definitions.$field {if object.guid.is_empty()||!guids.insert(object.guid.as_str()) {return Err("Definition GUIDs must be distinct".into());}}};}
    identity!(elements);identity!(planes);identity!(bboxes);identity!(meshes);identity!(points);identity!(lines);identity!(polylines);identity!(nurbscurves);identity!(nurbssurfaces);identity!(breps);identity!(pointclouds);
    let mut refs=HashSet::new();
    for instance in instances {
        if instance.guid.is_empty()||!refs.insert(instance.guid.as_str())||!guids.contains(instance.definition_guid.as_str()) {return Err("Invalid original instance reference".into());}
        if instance.flags&InstanceRef::FLAG_COLOR!=0 {
            let color=instance.color.as_ref().ok_or("Instance colour flag has no original colour")?;
            if [color.r,color.g,color.b,color.a].iter().any(|value|!value.is_finite()||!(0.0..=1.0).contains(value)) {return Err("Invalid original instance colour".into());}
        }
    }
    Ok(())
}

#[derive(Clone)]
pub struct Owner {pub document:Rc<Session>,pub instance:Rc<InstanceRef>,pub definition:Geometry}
impl Owner {
    pub fn new(document:Rc<Session>,instance:Rc<InstanceRef>)->Result<Self,&'static str> {
        let definition=document.definition_lookup.get(&instance.definition_guid).ok_or("Missing original instance definition")?.clone();
        Ok(Self {document,instance,definition})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn message()->proto::Session {
        let mesh=crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.3,0.8]).unwrap().geometry;
        proto::Session {objects:Some(proto::Objects {instances:vec![proto::InstanceRef {guid:"first".into(),name:"First".into(),definition_guid:mesh.guid().into(),..Default::default()},proto::InstanceRef {guid:"second".into(),name:"Second".into(),definition_guid:mesh.guid().into(),..Default::default()}],..Default::default()}),definitions:Some(proto::Objects {meshes:vec![mesh.to_proto()],..Default::default()}),..Default::default()}
    }
    #[test]
    fn checked_original_references_share_definition_identity_without_copying_geometry() {
        let message=message();validate(&message).unwrap();let document=Rc::new(Session::from_proto(message).unwrap());let first=Owner::new(document.clone(),document.objects.instances[0].clone()).unwrap();let second=Owner::new(document.clone(),document.objects.instances[1].clone()).unwrap();let (Geometry::Mesh(a),Geometry::Mesh(b))=(&first.definition,&second.definition) else {panic!("Original mesh definition")};assert!(Rc::ptr_eq(a,b));assert_eq!(first.instance.guid(),"first");assert_eq!(second.instance.guid(),"second");let weak=Rc::downgrade(a);drop(document);drop(first);assert!(weak.upgrade().is_some());drop(second);assert!(weak.upgrade().is_none());
    }
    #[test]
    fn missing_duplicate_and_unsupported_original_definitions_are_refused() {
        let mut source=message();source.definitions=None;assert!(validate(&source).is_err());let mut source=message();source.objects.as_mut().unwrap().instances[0].definition_guid="missing".into();assert!(validate(&source).is_err());let mut source=message();let duplicate=source.definitions.as_ref().unwrap().meshes[0].clone();source.definitions.as_mut().unwrap().meshes.push(duplicate);assert!(validate(&source).is_err());let mut source=message();source.definitions.as_mut().unwrap().instances.push(Default::default());assert!(validate(&source).is_err());
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-01.rs"
```

```text
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-02.rs"
```

</details>

### 2. `src/instance_definition.rs`

Retain supported original definition families and per-instance feature records.

<details>
<summary>Locate the existing block</summary>

```rust
    let mut scene=Scene::empty();
    match &owner.definition {
        Geometry::Mesh(source)=>{scene.insert(crate::prepared::PreparedMesh::from_shared(source.clone())?)?;},
        Geometry::BRep(source)=>{scene.insert_prepared_cad(shapes.prepare(source.clone())?)?;},
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-03.rs"
```

### 3. `src/instance_snapshot.rs`

Retain supported original definition families and per-instance feature records.

<details>
<summary>Locate the existing block</summary>

```rust
    for (_,owner) in scene.instances.iter().filter(|(id,_)|ids.contains(id)) {
        match &owner.definition {
            Geometry::Mesh(source)=>keep!(meshes,source),Geometry::BRep(source)=>keep!(breps,source),Geometry::NurbsSurface(source)=>keep!(nurbssurfaces,source),Geometry::NurbsCurve(source)=>keep!(nurbscurves,source),Geometry::Polyline(source)=>keep!(polylines,source),Geometry::Line(source)=>keep!(lines,source),Geometry::Point(source)=>keep!(points,source),Geometry::PointCloud(source)=>keep!(pointclouds,source),_=>return Err("Unsupported saved original definition".into()),
        }
    }
    macro_rules! order {($field:ident)=>{out.$field.sort_by(|a,b|a.guid.cmp(&b.guid));};}
    order!(meshes);order!(breps);order!(nurbssurfaces);order!(nurbscurves);order!(polylines);order!(lines);order!(points);order!(pointclouds);
    Ok((!records.is_empty()).then_some(out))
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96c-keep-current-element-and-frame-definition-families-04.rs"
```

</details>

<a id="checkpoint-96d-observe-the-real-current-registry-and-parser"></a>

### Step 5 of 9 · Observe the real current registry and parser (`96d-observe-the-real-current-registry-and-parser`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/9 Visual Trace (96d-observe-the-real-current-registry-and-parser): Observe the real current registry and parser</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/registry_audit.rs, src/lib.rs, src/inventory_example.rs, Cargo.toml</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Observe the real current registry and parser.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/registry_audit.rs` (*Observe the compiled maintained command registry directly.*) · `src/lib.rs` (*Register the completed audit owner.*) · `src/inventory_example.rs` (*Copy the supplied native registry observation example.*) · `Cargo.toml` (*Expose the maintained registry observation as a native example.*)

Observe the real current registry and parser.

> **Self-Check Question:** *Does a literal search find the complete runtime option list?*  
> **Answer:** Read every real Spec and call its parser for names, static options and aliases. Record canonical spelling, prompts, arity, selection/draft flags and actual errors. A parsed action still needs a real execution context.

- **Hands-On Lab Experiment:** Read the supplied runtime registry and grammar observations. Compare its 115 recorded static options with the earlier 149 literal strings, then inspect pending bare forms and Controls/Opacity differences. The authoring inventory helper is optional.

<details>
<summary>Implementation 96d-observe-the-real-current-registry-and-parser: Observe the real current registry and parser</summary>

Observe the real current registry and parser.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/registry_audit.rs`

Observe the compiled maintained command registry directly.

Create the file and type:

```rust
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-01.rs"
```

### 2. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod definition_merge;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-02.rs" skip=27
```

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 3. `src/inventory_example.rs`

Copy the supplied native registry observation example.

Copy this supplied file:

```text
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-03.rs"
```

</details>

### 4. `Cargo.toml`

Expose the maintained registry observation as a native example.

<details>
<summary>Locate the existing block</summary>

```toml
name = "reference"
path = "src/reference_example.rs"
```

</details>

After that block, type:

```toml
--8<-- "journey/code/96d-observe-the-real-current-registry-and-parser-04.rs" skip=53
```

</details>

<a id="checkpoint-96e-verify-the-complete-declared-current-source-closure"></a>

### Step 6 of 9 · Verify the complete declared current source closure (`96e-verify-the-complete-declared-current-source-closure`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/9 Visual Trace (96e-verify-the-complete-declared-current-source-closure): Verify the complete declared current source closure</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/current_inventory.json, src/source_audit.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Verify the complete declared current source closure.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/current_inventory.json` (*Copy the supplied full current Rust source/build closure and ownership snapshot.*) · `src/source_audit.rs` (*Verify actual source bytes and the complete declared file set.*) · `src/lib.rs` (*Register the completed audit owner.*)

Verify the complete declared current source closure.

> **Self-Check Question:** *Can a matching Git commit prove matching source contents?*  
> **Answer:** Hash the actual files, including uncommitted edits, and require the complete declared file set. Record the scope and local heads; do not imply that excluded C++/assets or remote commits were checked.

- **Hands-On Lab Experiment:** Read the supplied 428-file inventory, follow its lesson ownership and change one scratch record hash to see the audit fail.

<details>
<summary>Implementation 96e-verify-the-complete-declared-current-source-closure: Verify the complete declared current source closure</summary>

Verify the complete declared current source closure.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 1. `src/current_inventory.json`

Copy the supplied full current Rust source/build closure and ownership snapshot.

Copy this supplied file:

```text
--8<-- "journey/code/96e-verify-the-complete-declared-current-source-closure-01.rs"
```

</details>

### 2. `src/source_audit.rs`

Verify actual source bytes and the complete declared file set.

Create the file and type:

```rust
--8<-- "journey/code/96e-verify-the-complete-declared-current-source-closure-02.rs"
```

### 3. `src/lib.rs`

Register the completed audit owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod registry_audit;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96e-verify-the-complete-declared-current-source-closure-03.rs" skip=25
```

</details>

<a id="checkpoint-96f-report-retained-original-metadata-in-the-real-viewer"></a>

### Step 7 of 9 · Report retained original metadata in the real viewer (`96f-report-retained-original-metadata-in-the-real-viewer`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 7/9 Visual Trace (96f-report-retained-original-metadata-in-the-real-viewer): Report retained original metadata in the real viewer</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/browser.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Report retained original metadata in the real viewer.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/browser.rs` (*Expose truthful serialized original metadata accounting in the teaching inspector.*)

Report retained original metadata in the real viewer.

> **Self-Check Question:** *Does a byte counter measure driver memory?*  
> **Answer:** This counter reports unique retained origins and their serialized header size. Keep it distinct from allocated CPU/GPU bytes and process/driver memory.

- **Hands-On Lab Experiment:** Import and Close the metadata fixture and inspect the actual canvas report. Compare shared source origins across snapshots without counting them twice.

<details>
<summary>Implementation 96f-report-retained-original-metadata-in-the-real-viewer: Report retained original metadata in the real viewer</summary>

Report retained original metadata in the real viewer.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

### 1. `src/browser.rs`

Expose truthful serialized original metadata accounting in the teaching inspector.

<details>
<summary>Locate the existing block</summary>

```rust
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-groups",&serde_json::json!(editor.scene.groups.iter().collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-graph",&serde_json::json!(crate::graph_edit::rows(&editor.scene)).to_string())?;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96f-report-retained-original-metadata-in-the-real-viewer-01.rs"
```

</details>

<a id="checkpoint-96g-prove-exact-metadata-and-definition-round-trips"></a>

### Step 8 of 9 · Prove exact metadata and definition round trips (`96g-prove-exact-metadata-and-definition-round-trips`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 8/9 Visual Trace (96g-prove-exact-metadata-and-definition-round-trips): Prove exact metadata and definition round trips</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/metadata_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Prove exact metadata and definition round trips.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/metadata_tests.rs` (*Copy supplied exact archive, original-family and malformed-input fixtures.*) · `src/lib.rs` (*Register supplied metadata fixtures.*)

Prove exact metadata and definition round trips.

> **Self-Check Question:** *What catches an apparently successful but incomplete Save?*  
> **Answer:** Compare the original records after reopening. Include raw JSON formatting, opaque interaction bytes, unused typed definitions, instance-feature records and invalid planes.

- **What You Should See:** The archive implementation is unchanged. Optional cases cover retained custom metadata, unknown interactions and unused original definitions.
- **Hands-On Lab Experiment:** Save an imported original document, reopen it and compare its editable source families. Read the supplied archive cases to identify metadata that has no visible drawing. Do not infer preservation of invisible fields from the screenshot alone.

<details>
<summary>Implementation 96g-prove-exact-metadata-and-definition-round-trips: Prove exact metadata and definition round trips</summary>

Prove exact metadata and definition round trips.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/metadata_tests.rs`

Copy supplied exact archive, original-family and malformed-input fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96g-prove-exact-metadata-and-definition-round-trips-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register supplied metadata fixtures.

<details>
<summary>Locate the existing block</summary>

```text

pub mod source_audit;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96g-prove-exact-metadata-and-definition-round-trips-02.rs" skip=23
```

</details>

</details>

<a id="checkpoint-96h-prove-current-source-grammar-and-explicit-differences"></a>

### Step 9 of 9 · Prove current source grammar and explicit differences (`96h-prove-current-source-grammar-and-explicit-differences`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 9/9 Visual Trace (96h-prove-current-source-grammar-and-explicit-differences): Prove current source grammar and explicit differences</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/current_registry.json, src/inventory_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">audit original records and current</text>
    <text x="146" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">preserve exact metadata</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">verify real registry outcomes</text>
    
</svg>
</div>

- **Goal:** Prove current source grammar and explicit differences.
- **Data Flow Trace:** `audit original records and current source → preserve exact metadata → verify real registry outcomes`
- **Files You Touch:** `src/current_registry.json` (*Copy real observed current parser outcomes, prompts, aliases and action flags.*) · `src/inventory_tests.rs` (*Copy supplied complete source/registry and explicit grammar/shortcut fixtures.*) · `src/lib.rs` (*Register supplied complete source and registry fixtures.*)

Prove current source grammar and explicit differences.

> **Self-Check Question:** *Does one green parser test cover the current viewer?*  
> **Answer:** Check every recorded name, static option and alias against the compiled registry, every source hash/file against this checkout, and selected deliberate differences explicitly. Runtime browser actions remain separate acceptance.

- **What You Should See:** The original comparison scene remains intact while source and registry fixtures verify the complete declared maintained Rust inventory.
- **Hands-On Lab Experiment:** Read the dated source inventory and grammar observations beside the explicit implementation differences. Choose one listed command and execute it on an original selection in the browser. Distinguish that observed action from the optional source-hash and parser fixtures; complete viewer parity remains open.

<details>
<summary>Implementation 96h-prove-current-source-grammar-and-explicit-differences: Prove current source grammar and explicit differences</summary>

Prove current source grammar and explicit differences.

Data flow: audit original records and current source → preserve exact metadata → verify real registry outcomes.

<details>
<summary>Supplied data</summary>

Copy this supplied data once; it is excluded from the typing estimate.

### 1. `src/current_registry.json`

Copy real observed current parser outcomes, prompts, aliases and action flags.

Copy this supplied file:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/inventory_tests.rs`

Copy supplied complete source/registry and explicit grammar/shortcut fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/lib.rs`

Register supplied complete source and registry fixtures.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(test)]
mod metadata_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96h-prove-current-source-grammar-and-explicit-differences-03.rs" skip=33
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 96h-prove-current-source-grammar-and-explicit-differences
npm --prefix ../session_tests run course -- check 96h-prove-current-source-grammar-and-explicit-differences
npm --prefix ../session_tests run course -- save 96h-prove-current-source-grammar-and-explicit-differences
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

The original comparison scene remains intact while source and registry fixtures verify the complete declared maintained Rust inventory.

- **Final Lesson Experiment:** Read the dated source inventory and grammar observations beside the explicit implementation differences. Choose one listed command and execute it on an original selection in the browser. Distinguish that observed action from the optional source-hash and parser fixtures; complete viewer parity remains open.

Continue with [Original Element ancestry and derived feature trees](../../consolidation/96i-separate-own-and-hosted-element-features.md).
