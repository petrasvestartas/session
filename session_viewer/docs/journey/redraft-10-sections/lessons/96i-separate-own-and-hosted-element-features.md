# 19.2 · Original Element ancestry and derived feature trees

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 19.2** (Redraft `10.2`) · **10 checkpoints (`96i-separate-own-and-hosted-element-features` → `96r-preserve-exact-command-completions`)** · **Study time: 5–10 h**

> **What You Will Learn & Build:** Identify own attributes and interaction features by original feature type. Check readable names and face labels without changing any geometry. By the final checkpoint (`96r-preserve-exact-command-completions`), you will share the existing dictionary wrapper with the real widget regression and browser typing proof.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 19.2 Pipeline Schematic: Original Element ancestry and derived feature trees</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (10 Checkpoints: 96i-separate-own-and-hosted-element-features → 96r-preserve-exact-command-completions)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">original source</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">bounded feature</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">plan</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">derived display</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">source-preserving</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">archive</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">typed characters</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 19.2:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/feature_plan.rs  ·  src/lib.rs  ·  src/hierarchy.rs  ·  src/layer_edit.rs  ·  src/scene_archive.rs  ·  src/feature_display.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">10 Checkpoints: 96i-separate-own-and-hosted-element-features → 96r-preserve-exact-command-completions — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`96i-separate-own-and-hosted-element-features`](#checkpoint-96i-separate-own-and-hosted-element-features) | Identify own attributes and interaction features by original feature type. Check readable names and face labels without changing any geometry. | `src/feature_plan.rs`, `src/lib.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **2** | [`96j-group-features-by-original-interaction`](#checkpoint-96j-group-features-by-original-interaction) | Walk graph edges in their original order. Group contact and joint pieces under the full interaction type, using the relation name or source Element name, and retain unmatched features. | `src/feature_plan.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **3** | [`96k-separate-derived-rows-from-editable-elements`](#checkpoint-96k-separate-derived-rows-from-editable-elements) | Mark generated display rows separately from original objects. Nested child Elements remain editable even inside attributes, and saved documents omit derived geometry. | `src/hierarchy.rs`, `src/layer_edit.rs`, `src/scene_archive.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **4** | [`96l-attach-a-bounded-derived-feature-tree`](#checkpoint-96l-attach-a-bounded-derived-feature-tree) | Build read-only attributes, interaction and piece rows under the owning Element. Validate the complete tree before replacing its saved structure. | `src/layer_edit.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **5** | [`96m-project-original-feature-outlines`](#checkpoint-96m-project-original-feature-outlines) | Create black display paths from original feature outlines, apply the host placement once, and keep attributes Off and interactions On by default. | `src/feature_display.rs`, `src/lib.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **6** | [`96n-connect-imported-interaction-hierarchies`](#checkpoint-96n-connect-imported-interaction-hierarchies) | Import a document atomically, classify its editable child Elements, then attach feature projections from its original graph and retained Element payloads. | `src/scene_document.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **7** | [`96o-reuse-projections-for-created-elements`](#checkpoint-96o-reuse-projections-for-created-elements) | Created Elements and imported Elements now share feature display ownership. Visibility commands stop at independent child Elements and collect each feature piece once. | `src/element_adopt.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **8** | [`96p-build-an-imported-interaction-specimen`](#checkpoint-96p-build-an-imported-interaction-specimen) | Build a small original document with a host, an editable nested child, and a connector. Its contact, unnamed joint and orphan feature make the hierarchy inspectable without a large floor model. | `src/interaction_example.rs`, `src/lib.rs`, `examples/sample.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **9** | [`96q-prove-imported-feature-ownership-and-roundtrips`](#checkpoint-96q-prove-imported-feature-ownership-and-roundtrips) | Verify that feature projections follow their owner once, preserve original payloads, survive Delete/Undo, and rebuild after Save/Open without serializing display copies. Reject invalid outlines without changing the current document. | `src/interaction_bridge_tests.rs`, `src/lib.rs` | `original source → bounded feature plan → derived display → source-preserving archive` |
| **10** | [`96r-preserve-exact-command-completions`](#checkpoint-96r-preserve-exact-command-completions) | Share the existing dictionary wrapper with the real widget regression and browser typing proof. | `src/command_vocabulary.rs`, `src/panel.rs` | `typed characters → command vocabulary → exact match first → egui submission` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-96i-separate-own-and-hosted-element-features"></a>

### Step 1 of 10 · Separate own attributes from hosted features (`96i-separate-own-and-hosted-element-features`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/10 Visual Trace (96i-separate-own-and-hosted-element-features): Separate own attributes from hosted features</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/feature_plan.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Identify own attributes and interaction features by original feature type. Check readable names and face labels without changing any geometry.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/feature_plan.rs` (*Give this operation its own small owner.*) · `src/lib.rs` (*Register the new owner without changing the browser entry point.*)

An Element stores both its own attributes and features produced by its relationships.

Start with a small classifier over the original feature records. Contact, joint, cut and drill features belong to interactions; other types remain own attributes. Names and face indices become labels, while the original outlines remain untouched.

> **Self-Check Question:** *Why return feature indices instead of cloned geometry?*  
> **Answer:** Indices keep the grouping plan tied to the original Element. Display preparation can later borrow exactly those records, without treating a label or a display path as source data.

- **What You Should See:** Feature classification retains each original type, readable name and source label without mutating its geometry.
- **Hands-On Lab Experiment:** Try an empty feature name, a name equal to its type, and face index −1. Explain which text the label should omit.

<details>
<summary>Implementation 96i-separate-own-and-hosted-element-features: Separate own attributes from hosted features</summary>

Identify own attributes and interaction features by original feature type. Check readable names and face labels without changing any geometry.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/feature_plan.rs`

Give this operation its own small owner.

Create the file and type:

```rust
--8<-- "journey/code/96i-separate-own-and-hosted-element-features-01.rs"
```

### 2. `src/lib.rs`

Register the new owner without changing the browser entry point.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod inventory_tests;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod feature_plan;

#[cfg(test)]
mod inventory_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96i-separate-own-and-hosted-element-features-02.rs"
```

</details>

</details>

<a id="checkpoint-96j-group-features-by-original-interaction"></a>

### Step 2 of 10 · Group pieces by their original interaction (`96j-group-features-by-original-interaction`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/10 Visual Trace (96j-group-features-by-original-interaction): Group pieces by their original interaction</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/feature_plan.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Walk graph edges in their original order. Group contact and joint pieces under the full interaction type, using the relation name or source Element name, and retain unmatched features.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/feature_plan.rs` (*Claim each feature once in graph order; retain unknown and orphaned interaction features.*)

The graph determines who contributed a feature.

Visit incident edges by their recorded index, then each interaction in stored order. Exact feature/relation GUIDs identify contacts; InteractionFeature relations also claim source-prefixed feature GUIDs. Claim each feature once. Use the full interaction type and either its name or the source Element name, then retain unmatched features as explicit groups. Check has_guid before reading a feature GUID: guid() lazily assigns an ID and must not change an original merely to display it.

> **Self-Check Question:** *What happens to a feature with no matching relation?*  
> **Answer:** It remains visible in an unclaimed group using its own type, name and face label. Display grouping does not invent an interaction or assign a missing source GUID.

- **What You Should See:** Each claimed feature contributes to one ordered interaction group; unmatched features retain their own group.
- **Hands-On Lab Experiment:** Add two relations that could claim the same feature. Confirm that original edge/relation order chooses one group and that the feature is not duplicated.

<details>
<summary>Implementation 96j-group-features-by-original-interaction: Group pieces by their original interaction</summary>

Walk graph edges in their original order. Group contact and joint pieces under the full interaction type, using the relation name or source Element name, and retain unmatched features.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/feature_plan.rs`

Claim each feature once in graph order; retain unknown and orphaned interaction features.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96j-group-features-by-original-interaction-01.rs" skip=8
```

</details>

<a id="checkpoint-96k-separate-derived-rows-from-editable-elements"></a>

### Step 3 of 10 · Keep nested Elements editable and projections unsaved (`96k-separate-derived-rows-from-editable-elements`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/10 Visual Trace (96k-separate-derived-rows-from-editable-elements): Keep nested Elements editable and projections unsaved</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/hierarchy.rs, src/layer_edit.rs, src/scene_archive.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Mark generated display rows separately from original objects. Nested child Elements remain editable even inside attributes, and saved documents omit derived geometry.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/hierarchy.rs` (*Record display provenance explicitly on each tree node.*) · `src/layer_edit.rs` (*Keep user-created groups and source objects in the saved tree.*) · `src/scene_archive.rs` (*Only original tree objects enter each archive namespace.*)

A generated path can look like an ordinary polyline while belonging entirely to its host Element.

Add an explicit derived flag to hierarchy nodes. Original nodes remain serializable; generated subtrees are omitted from both the saved tree and the archive object inventory. When classifying an Element subtree, restart editability at each independent child Element, including a child nested beneath an attributes group.

> **Self-Check Question:** *Why filter both the hierarchy and loose-object inventory?*  
> **Answer:** Filtering only the tree would leave generated paths unassigned. The archive fallback would then save them as loose source objects and duplicate them on the next Open.

- **What You Should See:** The archive retains editable child Elements and excludes their generated display rows.
- **Hands-On Lab Experiment:** Trace a child Element beneath attributes. Its own object must remain editable, while its feature descendants remain read-only.

<details>
<summary>Implementation 96k-separate-derived-rows-from-editable-elements: Keep nested Elements editable and projections unsaved</summary>

Mark generated display rows separately from original objects. Nested child Elements remain editable even inside attributes, and saved documents omit derived geometry.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/hierarchy.rs`

Record display provenance explicitly on each tree node. Imported tree nodes initially belong to the source.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
            if exit {nodes[parent.unwrap()].end=nodes.len();continue;}
            let object=lookup.get(node.name.as_str()).copied();let readonly=readonly || inside && matches!(node.name.as_str(),"attributes"|"interactions");
            let index=nodes.len();let source=proto::TreeNode {guid:if node.guid.is_empty() {uuid::Uuid::new_v4().to_string()} else {node.guid.clone()},name:node.name.clone(),parent_guid:node.parent_guid.clone(),color:node.color.clone(),children:Vec::new()};
            nodes.push(Node {placement:Default::default(),portable_guid:uuid::Uuid::new_v4().to_string(),source,object:object.map(|v|v.0),label:object.map_or_else(||node.name.clone(),|v|v.1.to_owned()),parent,depth,end:index+1,readonly});
            stack.push((node,Some(index),depth,inside,readonly,true));
            stack.extend(node.children.iter().rev().map(|child|(child,Some(index),depth+1,inside||object.is_some(),readonly,false)));
        }
```

</details>

Replace it with:

```rust
            if exit {nodes[parent.unwrap()].end=nodes.len();continue;}
            let object=lookup.get(node.name.as_str()).copied();let readonly=readonly || inside && matches!(node.name.as_str(),"attributes"|"interactions");
            let index=nodes.len();let source=proto::TreeNode {guid:if node.guid.is_empty() {uuid::Uuid::new_v4().to_string()} else {node.guid.clone()},name:node.name.clone(),parent_guid:node.parent_guid.clone(),color:node.color.clone(),children:Vec::new()};
            nodes.push(Node {derived:false,placement:Default::default(),portable_guid:uuid::Uuid::new_v4().to_string(),source,object:object.map(|v|v.0),label:object.map_or_else(||node.name.clone(),|v|v.1.to_owned()),parent,depth,end:index+1,readonly});
            stack.push((node,Some(index),depth,inside,readonly,true));
            stack.extend(node.children.iter().rev().map(|child|(child,Some(index),depth+1,inside||object.is_some(),readonly,false)));
        }
```

<details>
<summary>Locate this block</summary>

```rust
}

#[derive(Clone)]
pub struct Node {pub placement:crate::nested_placement::Node,pub portable_guid:String, pub source:proto::TreeNode,pub object:Option<ObjectId>,pub label:String,pub parent:Option<usize>,pub depth:usize,pub end:usize,pub readonly:bool }
#[derive(Clone)]
pub struct Document {pub header:Option<Rc<crate::origin::Origin>>,pub key:u32,pub label:String,pub nodes:Vec<Node>}
#[derive(Clone,Default)]
```

</details>

Replace it with:

```rust
}

#[derive(Clone)]
pub struct Node {pub derived:bool,pub placement:crate::nested_placement::Node,pub portable_guid:String, pub source:proto::TreeNode,pub object:Option<ObjectId>,pub label:String,pub parent:Option<usize>,pub depth:usize,pub end:usize,pub readonly:bool }
#[derive(Clone)]
pub struct Document {pub header:Option<Rc<crate::origin::Origin>>,pub key:u32,pub label:String,pub nodes:Vec<Node>}
#[derive(Clone,Default)]
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::scene::{Scene,ObjectId};
use session_rust::{proto,Xform};
use std::{collections::{HashMap,HashSet},rc::Rc};

pub const MAX_NODES:usize=4096;
pub const MAX_DEPTH:usize=64;

pub fn validate(root:&proto::TreeNode,guids:&HashSet<&str>)->Result<(), &'static str> {
    let mut stack=vec![(root,0)];let mut seen=HashSet::new();let mut count=0;
    while let Some((node,depth))=stack.pop() {
        count+=1;if count>MAX_NODES || depth>MAX_DEPTH {return Err("Tree exceeds this checkpoint's node or depth limit");}
        if guids.contains(node.name.as_str()) && !seen.insert(node.name.as_str()) {return Err("A source object occurs twice in the tree");}
        stack.extend(node.children.iter().rev().map(|child|(child,depth+1)));
    }
    if seen!=*guids {return Err("The tree must contain every source mesh once");} Ok(())
}

pub fn world(root:Option<&proto::TreeNode>,xforms:&HashMap<String,Xform>,guid:&str)->Xform {
    let mut stack=root.into_iter().map(|node|(node,Xform::identity())).collect::<Vec<_>>();
    while let Some((node,parent))=stack.pop() {
        let local=xforms.get(&node.name).cloned().unwrap_or_else(Xform::identity);let matrix=&parent*&local;
        if node.name==guid {return matrix;}
        stack.extend(node.children.iter().rev().map(|child|(child,matrix.clone())));
    }
    xforms.get(guid).cloned().unwrap_or_else(Xform::identity)
}

#[derive(Clone)]
pub struct Node {pub derived:bool,pub placement:crate::nested_placement::Node,pub portable_guid:String, pub source:proto::TreeNode,pub object:Option<ObjectId>,pub label:String,pub parent:Option<usize>,pub depth:usize,pub end:usize,pub readonly:bool }
#[derive(Clone)]
pub struct Document {pub header:Option<Rc<crate::origin::Origin>>,pub key:u32,pub label:String,pub nodes:Vec<Node>}
#[derive(Clone,Default)]
pub struct Forest { pub documents:Vec<Rc<Document>>,readonly_ids:Rc<HashSet<ObjectId>> }

impl Forest {
    pub fn add(&mut self,key:u32,label:String,root:Option<proto::TreeNode>,objects:&[(String,ObjectId,String)]) {
        let fallback=proto::TreeNode {name:label.clone(),children:objects.iter().map(|(guid,_,_)|proto::TreeNode {name:guid.clone(),..Default::default()}).collect(),..Default::default()};
        let root=root.unwrap_or(fallback);let lookup=objects.iter().map(|(guid,id,name)|(guid.as_str(),(*id,name.as_str()))).collect::<HashMap<_,_>>();
        let mut nodes:Vec<Node>=Vec::new();let mut stack=vec![(&root,None::<usize>,0,false,false,false)];
        while let Some((node,parent,depth,inside,readonly,exit))=stack.pop() {
            if exit {nodes[parent.unwrap()].end=nodes.len();continue;}
            let object=lookup.get(node.name.as_str()).copied();let readonly=readonly || inside && matches!(node.name.as_str(),"attributes"|"interactions");
            let index=nodes.len();let source=proto::TreeNode {guid:if node.guid.is_empty() {uuid::Uuid::new_v4().to_string()} else {node.guid.clone()},name:node.name.clone(),parent_guid:node.parent_guid.clone(),color:node.color.clone(),children:Vec::new()};
            nodes.push(Node {derived:false,placement:Default::default(),portable_guid:uuid::Uuid::new_v4().to_string(),source,object:object.map(|v|v.0),label:object.map_or_else(||node.name.clone(),|v|v.1.to_owned()),parent,depth,end:index+1,readonly});
            stack.push((node,Some(index),depth,inside,readonly,true));
            stack.extend(node.children.iter().rev().map(|child|(child,Some(index),depth+1,inside||object.is_some(),readonly,false)));
        }
        Rc::make_mut(&mut self.readonly_ids).extend(nodes.iter().filter(|node|node.readonly).filter_map(|node|node.object));
        self.documents.push(Rc::new(Document {header:None,key,label,nodes}));
    }

    pub(crate) fn original_placements(&mut self,key:u32,source:&session_rust::Session,place:&Xform,world:&HashMap<String,Xform>) {
        let doc=Rc::make_mut(self.documents.iter_mut().find(|doc|doc.key==key).unwrap());
        for i in 0..doc.nodes.len() {
            let node=&doc.nodes[i];let parent=node.parent.map_or_else(Xform::identity,|p|doc.nodes[p].placement.world.clone());let local=source.xforms.get(&node.source.name).cloned().unwrap_or_else(Xform::identity);let local=if node.parent.is_none() {place*&local} else {local};let matrix=world.get(&node.source.name).cloned().unwrap_or_else(||&parent*&local);doc.nodes[i].placement=crate::nested_placement::Node {local,world:matrix,parent};
        }
    }

    pub(crate) fn push(&mut self,document:Document) {
        Rc::make_mut(&mut self.readonly_ids).extend(document.nodes.iter().filter(|node|node.readonly).filter_map(|node|node.object));
        self.documents.push(Rc::new(document));
    }

    pub fn colours(&self,key:u32)->HashMap<ObjectId,[u8;3]> {
        let mut out=HashMap::new();let Some(doc)=self.documents.iter().find(|doc|doc.key==key) else {return out;};
        let mut inherited=vec![None;doc.nodes.len()];
        for (index,node) in doc.nodes.iter().enumerate() {
            let colour=node.source.color.as_ref().map(|colour|[colour.r,colour.g,colour.b].map(|value|(value.clamp(0.,1.)*255.).round() as u8)).or_else(||node.parent.and_then(|parent|inherited[parent]));
            inherited[index]=colour;if let (Some(id),Some(colour))=(node.object,colour) {out.insert(id,colour);}
        }
        out
    }

    pub(crate) fn replace(&mut self,index:usize,document:Document) {
        self.documents[index]=Rc::new(document);
        self.readonly_ids=Rc::new(self.documents.iter().flat_map(|doc|doc.nodes.iter()).filter(|node|node.readonly).filter_map(|node|node.object).collect());
    }

    pub fn readonly(&self,id:ObjectId)->bool {self.readonly_ids.contains(&id)}
    pub fn descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {
        let mut out=ids.to_vec();
        for doc in &self.documents {for (index,node) in doc.nodes.iter().enumerate() {if node.object.is_some_and(|id|ids.contains(&id)) {out.extend(doc.nodes[index+1..node.end].iter().filter_map(|n|n.object));}}}
        out.sort_unstable_by_key(|id|id.value());out.dedup();out
    }

    pub fn snapshot(&self,scene:&Scene,xforms:&mut Vec<proto::XformEntry>)->Result<(Vec<proto::TreeNode>,HashSet<ObjectId>), &'static str> {
        let mut roots=Vec::new();let mut placed=HashSet::new();
        for doc in &self.documents {if let Some(root)=save_node(doc,0,scene,&Xform::identity(),"",xforms,&mut placed)? {roots.push(root);}}
        Ok((roots,placed))
    }
}

fn save_node(doc:&Document,index:usize,scene:&Scene,parent:&Xform,parent_guid:&str,xforms:&mut Vec<proto::XformEntry>,placed:&mut HashSet<ObjectId>)->Result<Option<proto::TreeNode>, &'static str> {
    let original=&doc.nodes[index];let mut node=original.source.clone();node.guid=original.portable_guid.clone();node.parent_guid=parent_guid.to_owned();let mut matrix=parent*&original.placement.local;let mut local=original.placement.local.clone();
    if let Some(id)=original.object {
        let Some(guid)=scene.row_guid(id) else {return Ok(None);};
        node.name=guid.to_owned();matrix=scene.placement(id).ok_or("Saved source placement is missing")?;local=crate::nested_placement::saved_local(&original.placement,parent,&matrix)?;
        placed.insert(id);
    }
    if local.m!=Xform::identity().m {
        let guid=node.name.clone();if let Some(previous)=xforms.iter().find(|entry|entry.guid==guid) {if previous.xform.as_ref()!=Some(&local.to_proto()) {return Err("Different saved tree placements share a name");}}
        else {xforms.push(proto::XformEntry {guid,xform:Some(local.to_proto())});}
    }
    for child in index+1..original.end {if doc.nodes[child].parent==Some(index) {if let Some(child)=save_node(doc,child,scene,&matrix,&node.guid,xforms,placed)? {node.children.push(child);}}}
    Ok(Some(node))
}

#[derive(Clone)]
pub struct Row {pub key:String,pub label:String,pub depth:usize,pub end:usize,pub ids:Vec<ObjectId>,pub readonly:bool}
pub fn rows(scene:&Scene)->Vec<Row> {
    let mut rows=Vec::new();let mut placed=HashSet::new();
    for doc in &scene.forest.documents {
        let mut visible=Vec::new();let mut index=0;
        while index<doc.nodes.len() {
            let node=&doc.nodes[index];
            if node.object.is_some_and(|id|!scene.contains(id)) {index=node.end;continue;}
            visible.push(index);index+=1;
        }
        let start=rows.len();
        for &index in &visible {
            let node=&doc.nodes[index];let ids=doc.nodes[index..node.end].iter().filter_map(|n|n.object).filter(|id|scene.contains(*id)).collect::<Vec<_>>();
            placed.extend(node.object);let end=start+visible.partition_point(|i|*i<node.end);
            rows.push(Row {key:format!("{}/{}",doc.key,node.source.guid),label:if index==0 {doc.label.clone()} else {node.label.clone()},depth:node.depth,end,ids,readonly:node.readonly});
        }
    }
    for row in scene.objects().iter().filter(|r|!placed.contains(&r.id)) {let index=rows.len();rows.push(Row {key:format!("created/{}",row.id.value()),label:row.metadata.name.clone(),depth:0,end:index+1,ids:vec![row.id],readonly:false});}
    let other=scene.lines().iter().map(|r|(r.id,r.prepared.source.name.as_str())).chain(scene.paths().iter().map(|r|(r.id,r.prepared.source.name()))).chain(scene.points().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.texts().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.clouds().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.streams().iter().map(|r|(r.id,r.prepared.source.name.as_str())));
    for (id,label) in other.filter(|(id,_)|!placed.contains(id)) {let index=rows.len();rows.push(Row {key:format!("created/{}",id.value()),label:label.to_owned(),depth:0,end:index+1,ids:vec![id],readonly:false});}
    rows
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-01.rs"
```

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-02.rs"
```

</details>

### 2. `src/layer_edit.rs`

Keep user-created groups and source objects in the saved tree.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    let crate::group_plan::Plan{owner,document:doc,name,parent,members}=crate::group_plan::Plan::new(scene,ids,name)?;
    let parent_node=&doc.nodes[parent];let parent_world=parent_node.placement.world.clone();let back=parent_world.inverse().ok_or("The layer is degenerate")?;
    let source=session_rust::proto::TreeNode{guid:uuid::Uuid::new_v4().to_string(),name:name.clone(),..Default::default()};let group_guid=source.guid.clone();
    let mut group=Branch{node:Node{placement:crate::nested_placement::Node{local:Default::default(),world:parent_world.clone(),parent:parent_world.clone()},portable_guid:group_guid.clone(),source,object:None,label:name.clone(),parent:None,depth:0,end:1,readonly:false},children:Vec::new()};let mut tree=branch(&doc,0);
    for id in members{let mut member=take(&mut tree,id).ok_or("Object tree is gone")?;let world=scene.placement(id).ok_or("Source placement is missing")?;member.node.placement=crate::nested_placement::Node{local:&back*&world,world,parent:parent_world.clone()};group.children.push(member);}
    find(&mut tree,&parent_node.source.guid).ok_or("Shared layer is gone")?.children.push(group);let mut nodes=Vec::new();flatten(tree.clone(),None,0,&mut nodes);if nodes.iter().any(|node|node.depth>crate::hierarchy::MAX_DEPTH){return Err("Tree exceeds its depth limit")}
    replace(scene,owner,tree);Rc::make_mut(&mut scene.groups).insert((doc.key,group_guid));Ok(name)
```

</details>

Replace it with:

```rust
    let crate::group_plan::Plan{owner,document:doc,name,parent,members}=crate::group_plan::Plan::new(scene,ids,name)?;
    let parent_node=&doc.nodes[parent];let parent_world=parent_node.placement.world.clone();let back=parent_world.inverse().ok_or("The layer is degenerate")?;
    let source=session_rust::proto::TreeNode{guid:uuid::Uuid::new_v4().to_string(),name:name.clone(),..Default::default()};let group_guid=source.guid.clone();
    let mut group=Branch{node:Node{derived:false,placement:crate::nested_placement::Node{local:Default::default(),world:parent_world.clone(),parent:parent_world.clone()},portable_guid:group_guid.clone(),source,object:None,label:name.clone(),parent:None,depth:0,end:1,readonly:false},children:Vec::new()};let mut tree=branch(&doc,0);
    for id in members{let mut member=take(&mut tree,id).ok_or("Object tree is gone")?;let world=scene.placement(id).ok_or("Source placement is missing")?;member.node.placement=crate::nested_placement::Node{local:&back*&world,world,parent:parent_world.clone()};group.children.push(member);}
    find(&mut tree,&parent_node.source.guid).ok_or("Shared layer is gone")?.children.push(group);let mut nodes=Vec::new();flatten(tree.clone(),None,0,&mut nodes);if nodes.iter().any(|node|node.depth>crate::hierarchy::MAX_DEPTH){return Err("Tree exceeds its depth limit")}
    replace(scene,owner,tree);Rc::make_mut(&mut scene.groups).insert((doc.key,group_guid));Ok(name)
```

<details>
<summary>Locate this block</summary>

```rust
    if doc.nodes[parent].depth>=crate::hierarchy::MAX_DEPTH {return Err("Tree exceeds its depth limit");}
    let label=(1..).map(|i|format!("Layer {i:02}")).find(|name|!doc.nodes.iter().any(|node|node.label==*name||node.source.name==*name)).unwrap();
    let source=session_rust::proto::TreeNode {guid:uuid::Uuid::new_v4().to_string(),name:label.clone(),..Default::default()};
    let child=Branch {node:Node {placement:Default::default(),portable_guid:source.guid.clone(),source,object:None,label,parent:None,depth:0,end:1,readonly:false},children:Vec::new()};
    let mut tree=branch(doc,0);find(&mut tree,&doc.nodes[parent].source.guid).unwrap().children.push(child);replace(scene,owner,tree);Ok(())
}
```

</details>

Replace it with:

```rust
    if doc.nodes[parent].depth>=crate::hierarchy::MAX_DEPTH {return Err("Tree exceeds its depth limit");}
    let label=(1..).map(|i|format!("Layer {i:02}")).find(|name|!doc.nodes.iter().any(|node|node.label==*name||node.source.name==*name)).unwrap();
    let source=session_rust::proto::TreeNode {guid:uuid::Uuid::new_v4().to_string(),name:label.clone(),..Default::default()};
    let child=Branch {node:Node {derived:false,placement:Default::default(),portable_guid:source.guid.clone(),source,object:None,label,parent:None,depth:0,end:1,readonly:false},children:Vec::new()};
    let mut tree=branch(doc,0);find(&mut tree,&doc.nodes[parent].source.guid).unwrap().children.push(child);replace(scene,owner,tree);Ok(())
}
```

<details>
<summary>Locate this block</summary>

```rust
    if doc.nodes.len()>=crate::hierarchy::MAX_NODES||doc.nodes[index].depth>=crate::hierarchy::MAX_DEPTH {return Err("Current layer exceeds its tree limit");}
    let parent=doc.nodes[index].source.guid.clone();let mut tree=branch(doc,0);
    let source=session_rust::proto::TreeNode {guid:uuid::Uuid::new_v4().to_string(),name:guid,..Default::default()};
    find(&mut tree,&parent).unwrap().children.push(Branch {node:Node {placement:Default::default(),portable_guid:source.guid.clone(),source,object:Some(id),label,parent:None,depth:0,end:1,readonly:false},children:Vec::new()});
    replace(scene,owner,tree);Ok(())
}
```

</details>

Replace it with:

```rust
    if doc.nodes.len()>=crate::hierarchy::MAX_NODES||doc.nodes[index].depth>=crate::hierarchy::MAX_DEPTH {return Err("Current layer exceeds its tree limit");}
    let parent=doc.nodes[index].source.guid.clone();let mut tree=branch(doc,0);
    let source=session_rust::proto::TreeNode {guid:uuid::Uuid::new_v4().to_string(),name:guid,..Default::default()};
    find(&mut tree,&parent).unwrap().children.push(Branch {node:Node {derived:false,placement:Default::default(),portable_guid:source.guid.clone(),source,object:Some(id),label,parent:None,depth:0,end:1,readonly:false},children:Vec::new()});
    replace(scene,owner,tree);Ok(())
}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{hierarchy::{Document,Node},scene::Scene};
use std::rc::Rc;

pub enum Edit {Current,New {sublayer:bool},Rename(String),Delete,Duplicate,Move(Vec<crate::scene::ObjectId>),Copy(Vec<crate::scene::ObjectId>)}

// Temporary branches make subtree edits atomic while the saved scene continues sharing its original source and display allocations.
#[derive(Clone)]
struct Branch {node:Node,children:Vec<Branch>}

fn branch(doc:&Document,index:usize)->Branch {
    Branch {node:doc.nodes[index].clone(),children:(index+1..doc.nodes[index].end).filter(|child|doc.nodes[*child].parent==Some(index)).map(|child|branch(doc,child)).collect()}
}

fn find<'a>(root:&'a mut Branch,guid:&str)->Option<&'a mut Branch> {
    if root.node.source.guid==guid {return Some(root);}
    root.children.iter_mut().find_map(|child|find(child,guid))
}

fn flatten(root:Branch,parent:Option<usize>,depth:usize,nodes:&mut Vec<Node>) {
    let index=nodes.len();let mut node=root.node;node.parent=parent;node.depth=depth;
    node.source.parent_guid=parent.map_or_else(String::new,|parent|nodes[parent].source.guid.clone());
    nodes.push(node);
    for child in root.children {flatten(child,Some(index),depth+1,nodes);}
    nodes[index].end=nodes.len();
}

pub fn locate(scene:&Scene,key:&str)->Result<(usize,usize), &'static str> {locate_group(scene,key,true)}

fn locate_group(scene:&Scene,key:&str,editable:bool)->Result<(usize,usize), &'static str> {
    for (owner,doc) in scene.forest.documents.iter().enumerate() {
        for (index,node) in doc.nodes.iter().enumerate() {
            if format!("{}/{}",doc.key,node.source.guid)!=key {continue;}
            let mut ancestor=Some(index);
            while let Some(index)=ancestor {
                if doc.nodes[index].object.is_some() {return Err("Objects and their attributes are not layers");}
                ancestor=doc.nodes[index].parent;
            }
            if editable && doc.nodes.iter().filter_map(|node|node.object).any(|id|scene.objects().iter().any(|row|row.id==id&&row.geometry().is_none())) {return Err("Reload editable sources before editing layers");}
            return Ok((owner,index));
        }
    }
    Err("This layer is gone")
}

pub fn holds_current(scene:&Scene,key:&str)->bool {
    let Some(current)=current_key(scene) else {return false;};
    let rows=crate::hierarchy::rows(scene);
    rows.iter().position(|row|row.key==key).is_some_and(|index|rows[index..rows[index].end].iter().any(|row|row.key==current))
}

pub fn current_key(scene:&Scene)->Option<String> {
    if let Some(key)=scene.view.current.as_ref() && locate_group(scene,key,false).is_ok() {return Some(key.clone());}
    let doc=scene.forest.documents.iter().find(|doc|Some(doc.key)==scene.created)?;
    Some(format!("{}/{}",doc.key,doc.nodes[0].source.guid))
}

fn replace(scene:&mut Scene,owner:usize,tree:Branch) {
    let old=&scene.forest.documents[owner];let mut document=Document {header:old.header.clone(),key:old.key,label:old.label.clone(),nodes:Vec::new()};
    flatten(tree,None,0,&mut document.nodes);scene.forest.replace(owner,document);Rc::make_mut(&mut scene.view).refresh(&scene.forest);
}

pub(crate) fn attach_generated(scene:&mut Scene,id:crate::scene::ObjectId,guid:String,label:String)->Result<(), &'static str> {
    if current_key(scene).is_none() {
        scene.forest.add(id.value(),"Created".into(),None,&[]);scene.created=Some(id.value());
    }
    let key=current_key(scene).unwrap();let (owner,index)=locate(scene,&key)?;let doc=&scene.forest.documents[owner];
    if doc.nodes.len()>=crate::hierarchy::MAX_NODES||doc.nodes[index].depth>=crate::hierarchy::MAX_DEPTH {return Err("Current layer exceeds its tree limit");}
    let parent=doc.nodes[index].source.guid.clone();let mut tree=branch(doc,0);
    let source=session_rust::proto::TreeNode {guid:uuid::Uuid::new_v4().to_string(),name:guid,..Default::default()};
    find(&mut tree,&parent).unwrap().children.push(Branch {node:Node {derived:false,placement:Default::default(),portable_guid:source.guid.clone(),source,object:Some(id),label,parent:None,depth:0,end:1,readonly:false},children:Vec::new()});
    replace(scene,owner,tree);Ok(())
}

fn take(root:&mut Branch,id:crate::scene::ObjectId)->Option<Branch> {
    if let Some(index)=root.children.iter().position(|child|child.node.object==Some(id)) {return Some(root.children.remove(index));}
    root.children.iter_mut().find_map(|child|take(child,id))
}

fn copy_branch(scene:&mut Scene,mut tree:Branch,all:bool)->Result<Branch,&'static str> {
    let group_doc=scene.forest.documents.iter().find(|d|scene.groups.contains(&(d.key,tree.node.source.guid.clone()))).map(|d|d.key);
    tree.node.source.guid=uuid::Uuid::new_v4().to_string();tree.node.portable_guid=tree.node.source.guid.clone();
    if let Some(key)=group_doc {Rc::make_mut(&mut scene.groups).insert((key,tree.node.source.guid.clone()));}
    if let Some(id)=tree.node.object {let copy=scene.copy_row(id,all)?;tree.node.object=Some(copy);tree.node.source.name=scene.row_guid(copy).unwrap().to_owned();}
    tree.children=tree.children.into_iter().map(|child|copy_branch(scene,child,all)).collect::<Result<_,_>>()?;Ok(tree)
}

fn transfer(scene:&mut Scene,key:&str,ids:Vec<crate::scene::ObjectId>,copy:bool)->Result<(), &'static str> {
    let (owner,index)=locate(scene,key)?;let target=scene.forest.documents[owner].nodes[index].source.guid.clone();
    if ids.is_empty() {return Err("Select objects first");}
    if ids.iter().any(|id|scene.pick_id(id.value())!=Some(*id)) {return Err("Choose visible unlocked editable objects");}
    let chosen=ids.into_iter().collect::<std::collections::HashSet<_>>();let mut trees=scene.forest.documents.iter().map(|doc|branch(doc,0)).collect::<Vec<_>>();let mut moved=Vec::new();
    for (doc_index,doc) in scene.forest.documents.iter().enumerate() {
        for (index,node) in doc.nodes.iter().enumerate() {
            let Some(id)=node.object.filter(|id|chosen.contains(id)) else {continue;};let mut parent=node.parent;let mut follows=false;
            while let Some(index)=parent {follows|=doc.nodes[index].object.is_some_and(|id|chosen.contains(&id));parent=doc.nodes[index].parent;}
            if follows&&!copy {continue;}
            if !copy&&doc_index==owner&&node.parent.is_some_and(|parent|doc.nodes[parent].source.guid==target) {continue;}
            let mut tree=if copy {branch(doc,index)} else {take(&mut trees[doc_index],id).ok_or("Object tree is gone")?};
            if copy {tree.children.clear();}
            moved.push(tree);
        }
    }
    if moved.is_empty() {return Err("The objects are already on this layer");}
    let mut target_nodes=Vec::new();flatten(trees[owner].clone(),None,0,&mut target_nodes);let count=target_nodes.len()+moved.iter().map(|tree|{let mut nodes=Vec::new();flatten(tree.clone(),None,0,&mut nodes);nodes.len()}).sum::<usize>();
    if count>crate::hierarchy::MAX_NODES {return Err("Target layer exceeds its node limit");}
    for tree in moved {let tree=if copy {copy_branch(scene,tree,false)?} else {tree};find(&mut trees[owner],&target).unwrap().children.push(tree);}
    for (owner,tree) in trees.into_iter().enumerate() {let mut nodes=Vec::new();flatten(tree.clone(),None,0,&mut nodes);if nodes.iter().any(|node|node.depth>crate::hierarchy::MAX_DEPTH) {return Err("Target layer exceeds its depth limit");}replace(scene,owner,tree);}
    Ok(())
}

fn new_layer(scene:&mut Scene,key:&str,sublayer:bool)->Result<(), &'static str> {
    let (owner,index)=locate(scene,key)?;let doc=&scene.forest.documents[owner];let node=&doc.nodes[index];
    if doc.nodes.len()>=crate::hierarchy::MAX_NODES {return Err("Tree exceeds its node limit");}
    let parent=if sublayer {index} else {node.parent.unwrap_or(index)};
    if doc.nodes[parent].depth>=crate::hierarchy::MAX_DEPTH {return Err("Tree exceeds its depth limit");}
    let label=(1..).map(|i|format!("Layer {i:02}")).find(|name|!doc.nodes.iter().any(|node|node.label==*name||node.source.name==*name)).unwrap();
    let source=session_rust::proto::TreeNode {guid:uuid::Uuid::new_v4().to_string(),name:label.clone(),..Default::default()};
    let child=Branch {node:Node {derived:false,placement:Default::default(),portable_guid:source.guid.clone(),source,object:None,label,parent:None,depth:0,end:1,readonly:false},children:Vec::new()};
    let mut tree=branch(doc,0);find(&mut tree,&doc.nodes[parent].source.guid).unwrap().children.push(child);replace(scene,owner,tree);Ok(())
}

fn rename(scene:&mut Scene,key:&str,label:&str)->Result<(), &'static str> {
    let (owner,index)=locate(scene,key)?;let doc=&scene.forest.documents[owner];let node=&doc.nodes[index];
    if node.parent.is_none() {return Err("The root layer cannot be renamed");}
    let label=label.trim();
    if label.is_empty()||label.contains('/') {return Err("A layer name must be non-empty and without /");}
    if label==node.label {return Err("The layer already has that name");}
    if doc.nodes.iter().any(|other|other.label==label||other.source.name==label) {return Err("This name is already used in the document");}
    let mut tree=branch(doc,0);let node=&mut find(&mut tree,&node.source.guid).unwrap().node;node.label=label.to_owned();node.source.name=label.to_owned();replace(scene,owner,tree);Ok(())
}

fn delete(scene:&mut Scene,key:&str)->Result<(), &'static str> {
    let (owner,index)=locate(scene,key)?;let doc=&scene.forest.documents[owner];let node=&doc.nodes[index];
    let parent=node.parent.ok_or("The root layer cannot be deleted")?;
    if holds_current(scene,key) {return Err("The current layer cannot be deleted");}
    let ids=doc.nodes[index..node.end].iter().filter_map(|node|node.object).collect::<Vec<_>>();let mut tree=branch(doc,0);
    find(&mut tree,&doc.nodes[parent].source.guid).unwrap().children.retain(|child|child.node.source.guid!=node.source.guid);
    for id in ids {scene.remove(id);}replace(scene,owner,tree);Ok(())
}

fn duplicate(scene:&mut Scene,key:&str)->Result<(), &'static str> {
    let (owner,index)=locate(scene,key)?;let doc=&scene.forest.documents[owner];let node=&doc.nodes[index];
    let parent=node.parent.ok_or("The root layer cannot be duplicated")?;
    if doc.nodes.len()+node.end-index>crate::hierarchy::MAX_NODES {return Err("Tree exceeds its node limit");}
    let label=(1..).map(|i|format!("{} copy {i:02}",node.label)).find(|name|!doc.nodes.iter().any(|node|node.label==*name||node.source.name==*name)).unwrap();
    let parent=doc.nodes[parent].source.guid.clone();let source=branch(doc,index);let mut tree=branch(doc,0);let mut copy=copy_branch(scene,source,true)?;copy.node.label=label.clone();copy.node.source.name=label;
    find(&mut tree,&parent).unwrap().children.push(copy);replace(scene,owner,tree);Ok(())
}

pub fn apply(scene:&mut Scene,key:&str,edit:Edit)->Result<(), &'static str> {
    match edit {
        Edit::Current=>{locate(scene,key)?;Rc::make_mut(&mut scene.view).current=Some(key.to_owned());Ok(())},
        Edit::New {sublayer}=>new_layer(scene,key,sublayer),
        Edit::Rename(label)=>rename(scene,key,&label),
        Edit::Delete=>delete(scene,key),
        Edit::Duplicate=>duplicate(scene,key),
        Edit::Move(ids)=>transfer(scene,key,ids,false),
        Edit::Copy(ids)=>transfer(scene,key,ids,true),
    }
}

pub(crate) fn copy_objects(scene:&mut Scene,ids:&[crate::scene::ObjectId],delta:&session_rust::Xform)->Result<Vec<crate::scene::ObjectId>, &'static str> {
    if ids.is_empty(){return Err("Select objects before Copy");}
    if !crate::placement::valid(&delta.m){return Err("Copy transform is invalid");}
    let mut chosen=ids.to_vec();chosen.sort_unstable_by_key(|id|id.value());chosen.dedup();
    if chosen.iter().any(|id|scene.pick_id(id.value())!=Some(*id)){return Err("Copy needs visible unlocked editable objects");}
    let mut trees=scene.forest.documents.iter().map(|doc|branch(doc,0)).collect::<Vec<_>>();let mut sources=Vec::new();
    for (owner,doc) in scene.forest.documents.iter().enumerate(){for(index,node)in doc.nodes.iter().enumerate(){let Some(id)=node.object.filter(|id|chosen.contains(id))else{continue};let parent=node.parent.ok_or("Object has no parent")?;let mut tree=branch(doc,index);tree.children.retain(|child|child.node.object.is_none()&&child.node.source.name=="attributes");sources.push((owner,doc.nodes[parent].source.guid.clone(),tree,scene.placement(id).ok_or("Copy placement is missing")?));}}
    if sources.len()!=chosen.len(){return Err("Copy source tree is missing");}
    for(owner,doc)in scene.forest.documents.iter().enumerate(){let extra=sources.iter().filter(|source|source.0==owner).map(|source|{let mut nodes=Vec::new();flatten(source.2.clone(),None,0,&mut nodes);nodes.len()}).sum::<usize>();if doc.nodes.len()+extra>crate::hierarchy::MAX_NODES{return Err("Copy exceeds the tree limit");}}
    let mut copies=Vec::new();
    fn place_copy(scene:&mut Scene,tree:&Branch,delta:&session_rust::Xform)->Result<(),&'static str>{if let Some(id)=tree.node.object{let model=scene.placement(id).ok_or("Copy placement is missing")?;scene.place_following(id,delta*&model)?;}for child in &tree.children{place_copy(scene,child,delta)?;}Ok(())}
    for(owner,parent,tree,_model)in sources{let copy=copy_branch(scene,tree,false)?;let id=copy.node.object.ok_or("Copy owner is missing")?;place_copy(scene,&copy,delta)?;copies.push(id);find(&mut trees[owner],&parent).ok_or("Copy parent is missing")?.children.push(copy);}
    for(owner,tree)in trees.into_iter().enumerate(){replace(scene,owner,tree);}Ok(copies)
}

pub fn group(scene:&mut Scene,ids:&[crate::scene::ObjectId],name:&str)->Result<String,&'static str>{
    let crate::group_plan::Plan{owner,document:doc,name,parent,members}=crate::group_plan::Plan::new(scene,ids,name)?;
    let parent_node=&doc.nodes[parent];let parent_world=parent_node.placement.world.clone();let back=parent_world.inverse().ok_or("The layer is degenerate")?;
    let source=session_rust::proto::TreeNode{guid:uuid::Uuid::new_v4().to_string(),name:name.clone(),..Default::default()};let group_guid=source.guid.clone();
    let mut group=Branch{node:Node{derived:false,placement:crate::nested_placement::Node{local:Default::default(),world:parent_world.clone(),parent:parent_world.clone()},portable_guid:group_guid.clone(),source,object:None,label:name.clone(),parent:None,depth:0,end:1,readonly:false},children:Vec::new()};let mut tree=branch(&doc,0);
    for id in members{let mut member=take(&mut tree,id).ok_or("Object tree is gone")?;let world=scene.placement(id).ok_or("Source placement is missing")?;member.node.placement=crate::nested_placement::Node{local:&back*&world,world,parent:parent_world.clone()};group.children.push(member);}
    find(&mut tree,&parent_node.source.guid).ok_or("Shared layer is gone")?.children.push(group);let mut nodes=Vec::new();flatten(tree.clone(),None,0,&mut nodes);if nodes.iter().any(|node|node.depth>crate::hierarchy::MAX_DEPTH){return Err("Tree exceeds its depth limit")}
    replace(scene,owner,tree);Rc::make_mut(&mut scene.groups).insert((doc.key,group_guid));Ok(name)
}

pub fn attach_attributes(scene:&mut Scene,parent:crate::scene::ObjectId,ids:Vec<crate::scene::ObjectId>,name:&str)->Result<(),&'static str>{
 let(owner,index)=scene.forest.documents.iter().enumerate().find_map(|(i,d)|d.nodes.iter().position(|n|n.object==Some(parent)).map(|n|(i,n))).ok_or("Element hierarchy owner is missing")?;
 let doc=&scene.forest.documents[owner];let guid=doc.nodes[index].source.guid.clone();let mut root=branch(doc,0);let mut children=Vec::new();
 for id in ids{let mut child=take(&mut root,id).ok_or("Element attribute must share its document")?;child.node.readonly=true;children.push(child);}
 let parent=find(&mut root,&guid).ok_or("Element hierarchy owner is missing")?;let mut node=parent.node.clone();node.object=None;node.readonly=true;node.label=name.into();node.source=session_rust::proto::TreeNode{guid:uuid::Uuid::new_v4().to_string(),name:name.into(),..Default::default()};node.portable_guid=node.source.guid.clone();node.placement=Default::default();parent.children.push(Branch{node,children});
 let mut count=0;let mut max=0;fn size(b:&Branch,depth:usize,count:&mut usize,max:&mut usize){*count+=1;*max=(*max).max(depth);for c in &b.children{size(c,depth+1,count,max)}}size(&root,0,&mut count,&mut max);if count>crate::hierarchy::MAX_NODES||max>crate::hierarchy::MAX_DEPTH{return Err("Element attributes exceed hierarchy limits")}replace(scene,owner,root);Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-03.rs"
```

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-04.rs"
```

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-05.rs"
```

</details>

### 3. `src/hierarchy.rs`

Reset read-only inheritance at each real child Element; retain it for feature pieces. Skip projected subtrees when saving originals.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
}

fn save_node(doc:&Document,index:usize,scene:&Scene,parent:&Xform,parent_guid:&str,xforms:&mut Vec<proto::XformEntry>,placed:&mut HashSet<ObjectId>)->Result<Option<proto::TreeNode>, &'static str> {
    let original=&doc.nodes[index];let mut node=original.source.clone();node.guid=original.portable_guid.clone();node.parent_guid=parent_guid.to_owned();let mut matrix=parent*&original.placement.local;let mut local=original.placement.local.clone();
    if let Some(id)=original.object {
        let Some(guid)=scene.row_guid(id) else {return Ok(None);};
        node.name=guid.to_owned();matrix=scene.placement(id).ok_or("Saved source placement is missing")?;local=crate::nested_placement::saved_local(&original.placement,parent,&matrix)?;
```

</details>

Replace it with:

```rust
}

fn save_node(doc:&Document,index:usize,scene:&Scene,parent:&Xform,parent_guid:&str,xforms:&mut Vec<proto::XformEntry>,placed:&mut HashSet<ObjectId>)->Result<Option<proto::TreeNode>, &'static str> {
    let original=&doc.nodes[index];if original.derived {return Ok(None);}
    let mut node=original.source.clone();node.guid=original.portable_guid.clone();node.parent_guid=parent_guid.to_owned();let mut matrix=parent*&original.placement.local;let mut local=original.placement.local.clone();
    if let Some(id)=original.object {
        let Some(guid)=scene.row_guid(id) else {return Ok(None);};
        node.name=guid.to_owned();matrix=scene.placement(id).ok_or("Saved source placement is missing")?;local=crate::nested_placement::saved_local(&original.placement,parent,&matrix)?;
```

<details>
<summary>Locate this block</summary>

```rust
        self.readonly_ids=Rc::new(self.documents.iter().flat_map(|doc|doc.nodes.iter()).filter(|node|node.readonly).filter_map(|node|node.object).collect());
    }

    pub fn readonly(&self,id:ObjectId)->bool {self.readonly_ids.contains(&id)}
    pub fn descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {
        let mut out=ids.to_vec();
```

</details>

Replace it with:

```rust
        self.readonly_ids=Rc::new(self.documents.iter().flat_map(|doc|doc.nodes.iter()).filter(|node|node.readonly).filter_map(|node|node.object).collect());
    }

    pub fn classify_elements(&mut self, key: u32, elements: &HashSet<ObjectId>) {
        let owner = self.documents.iter().position(|doc| doc.key == key).unwrap();
        let mut doc = (*self.documents[owner]).clone();
        let mut inside = vec![false; doc.nodes.len()];
        for i in 0..doc.nodes.len() {
            let parent = doc.nodes[i].parent;
            let element = doc.nodes[i].object.is_some_and(|id| elements.contains(&id));
            let inherited = parent.is_some_and(|p| inside[p]);
            let held = parent.is_some_and(|p| doc.nodes[p].readonly);
            if element || inherited {
                doc.nodes[i].readonly = !element && (held
                    || matches!(doc.nodes[i].source.name.as_str(), "attributes" | "interactions"));
            }
            inside[i] = element || inherited;
        }
        self.replace(owner, doc);
    }

    pub fn derived(&self, id: ObjectId) -> bool {
        self.documents.iter().any(|doc| doc.nodes.iter()
            .any(|node| node.object == Some(id) && node.derived))
    }

    pub fn readonly(&self,id:ObjectId)->bool {self.readonly_ids.contains(&id)}
    pub fn descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {
        let mut out=ids.to_vec();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::scene::{Scene,ObjectId};
use session_rust::{proto,Xform};
use std::{collections::{HashMap,HashSet},rc::Rc};

pub const MAX_NODES:usize=4096;
pub const MAX_DEPTH:usize=64;

pub fn validate(root:&proto::TreeNode,guids:&HashSet<&str>)->Result<(), &'static str> {
    let mut stack=vec![(root,0)];let mut seen=HashSet::new();let mut count=0;
    while let Some((node,depth))=stack.pop() {
        count+=1;if count>MAX_NODES || depth>MAX_DEPTH {return Err("Tree exceeds this checkpoint's node or depth limit");}
        if guids.contains(node.name.as_str()) && !seen.insert(node.name.as_str()) {return Err("A source object occurs twice in the tree");}
        stack.extend(node.children.iter().rev().map(|child|(child,depth+1)));
    }
    if seen!=*guids {return Err("The tree must contain every source mesh once");} Ok(())
}

pub fn world(root:Option<&proto::TreeNode>,xforms:&HashMap<String,Xform>,guid:&str)->Xform {
    let mut stack=root.into_iter().map(|node|(node,Xform::identity())).collect::<Vec<_>>();
    while let Some((node,parent))=stack.pop() {
        let local=xforms.get(&node.name).cloned().unwrap_or_else(Xform::identity);let matrix=&parent*&local;
        if node.name==guid {return matrix;}
        stack.extend(node.children.iter().rev().map(|child|(child,matrix.clone())));
    }
    xforms.get(guid).cloned().unwrap_or_else(Xform::identity)
}

#[derive(Clone)]
pub struct Node {pub derived:bool,pub placement:crate::nested_placement::Node,pub portable_guid:String, pub source:proto::TreeNode,pub object:Option<ObjectId>,pub label:String,pub parent:Option<usize>,pub depth:usize,pub end:usize,pub readonly:bool }
#[derive(Clone)]
pub struct Document {pub header:Option<Rc<crate::origin::Origin>>,pub key:u32,pub label:String,pub nodes:Vec<Node>}
#[derive(Clone,Default)]
pub struct Forest { pub documents:Vec<Rc<Document>>,readonly_ids:Rc<HashSet<ObjectId>> }

impl Forest {
    pub fn add(&mut self,key:u32,label:String,root:Option<proto::TreeNode>,objects:&[(String,ObjectId,String)]) {
        let fallback=proto::TreeNode {name:label.clone(),children:objects.iter().map(|(guid,_,_)|proto::TreeNode {name:guid.clone(),..Default::default()}).collect(),..Default::default()};
        let root=root.unwrap_or(fallback);let lookup=objects.iter().map(|(guid,id,name)|(guid.as_str(),(*id,name.as_str()))).collect::<HashMap<_,_>>();
        let mut nodes:Vec<Node>=Vec::new();let mut stack=vec![(&root,None::<usize>,0,false,false,false)];
        while let Some((node,parent,depth,inside,readonly,exit))=stack.pop() {
            if exit {nodes[parent.unwrap()].end=nodes.len();continue;}
            let object=lookup.get(node.name.as_str()).copied();let readonly=readonly || inside && matches!(node.name.as_str(),"attributes"|"interactions");
            let index=nodes.len();let source=proto::TreeNode {guid:if node.guid.is_empty() {uuid::Uuid::new_v4().to_string()} else {node.guid.clone()},name:node.name.clone(),parent_guid:node.parent_guid.clone(),color:node.color.clone(),children:Vec::new()};
            nodes.push(Node {derived:false,placement:Default::default(),portable_guid:uuid::Uuid::new_v4().to_string(),source,object:object.map(|v|v.0),label:object.map_or_else(||node.name.clone(),|v|v.1.to_owned()),parent,depth,end:index+1,readonly});
            stack.push((node,Some(index),depth,inside,readonly,true));
            stack.extend(node.children.iter().rev().map(|child|(child,Some(index),depth+1,inside||object.is_some(),readonly,false)));
        }
        Rc::make_mut(&mut self.readonly_ids).extend(nodes.iter().filter(|node|node.readonly).filter_map(|node|node.object));
        self.documents.push(Rc::new(Document {header:None,key,label,nodes}));
    }

    pub(crate) fn original_placements(&mut self,key:u32,source:&session_rust::Session,place:&Xform,world:&HashMap<String,Xform>) {
        let doc=Rc::make_mut(self.documents.iter_mut().find(|doc|doc.key==key).unwrap());
        for i in 0..doc.nodes.len() {
            let node=&doc.nodes[i];let parent=node.parent.map_or_else(Xform::identity,|p|doc.nodes[p].placement.world.clone());let local=source.xforms.get(&node.source.name).cloned().unwrap_or_else(Xform::identity);let local=if node.parent.is_none() {place*&local} else {local};let matrix=world.get(&node.source.name).cloned().unwrap_or_else(||&parent*&local);doc.nodes[i].placement=crate::nested_placement::Node {local,world:matrix,parent};
        }
    }

    pub(crate) fn push(&mut self,document:Document) {
        Rc::make_mut(&mut self.readonly_ids).extend(document.nodes.iter().filter(|node|node.readonly).filter_map(|node|node.object));
        self.documents.push(Rc::new(document));
    }

    pub fn colours(&self,key:u32)->HashMap<ObjectId,[u8;3]> {
        let mut out=HashMap::new();let Some(doc)=self.documents.iter().find(|doc|doc.key==key) else {return out;};
        let mut inherited=vec![None;doc.nodes.len()];
        for (index,node) in doc.nodes.iter().enumerate() {
            let colour=node.source.color.as_ref().map(|colour|[colour.r,colour.g,colour.b].map(|value|(value.clamp(0.,1.)*255.).round() as u8)).or_else(||node.parent.and_then(|parent|inherited[parent]));
            inherited[index]=colour;if let (Some(id),Some(colour))=(node.object,colour) {out.insert(id,colour);}
        }
        out
    }

    pub(crate) fn replace(&mut self,index:usize,document:Document) {
        self.documents[index]=Rc::new(document);
        self.readonly_ids=Rc::new(self.documents.iter().flat_map(|doc|doc.nodes.iter()).filter(|node|node.readonly).filter_map(|node|node.object).collect());
    }

    pub fn classify_elements(&mut self, key: u32, elements: &HashSet<ObjectId>) {
        let owner = self.documents.iter().position(|doc| doc.key == key).unwrap();
        let mut doc = (*self.documents[owner]).clone();
        let mut inside = vec![false; doc.nodes.len()];
        for i in 0..doc.nodes.len() {
            let parent = doc.nodes[i].parent;
            let element = doc.nodes[i].object.is_some_and(|id| elements.contains(&id));
            let inherited = parent.is_some_and(|p| inside[p]);
            let held = parent.is_some_and(|p| doc.nodes[p].readonly);
            if element || inherited {
                doc.nodes[i].readonly = !element && (held
                    || matches!(doc.nodes[i].source.name.as_str(), "attributes" | "interactions"));
            }
            inside[i] = element || inherited;
        }
        self.replace(owner, doc);
    }

    pub fn derived(&self, id: ObjectId) -> bool {
        self.documents.iter().any(|doc| doc.nodes.iter()
            .any(|node| node.object == Some(id) && node.derived))
    }

    pub fn readonly(&self,id:ObjectId)->bool {self.readonly_ids.contains(&id)}
    pub fn descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {
        let mut out=ids.to_vec();
        for doc in &self.documents {for (index,node) in doc.nodes.iter().enumerate() {if node.object.is_some_and(|id|ids.contains(&id)) {out.extend(doc.nodes[index+1..node.end].iter().filter_map(|n|n.object));}}}
        out.sort_unstable_by_key(|id|id.value());out.dedup();out
    }

    pub fn snapshot(&self,scene:&Scene,xforms:&mut Vec<proto::XformEntry>)->Result<(Vec<proto::TreeNode>,HashSet<ObjectId>), &'static str> {
        let mut roots=Vec::new();let mut placed=HashSet::new();
        for doc in &self.documents {if let Some(root)=save_node(doc,0,scene,&Xform::identity(),"",xforms,&mut placed)? {roots.push(root);}}
        Ok((roots,placed))
    }
}

fn save_node(doc:&Document,index:usize,scene:&Scene,parent:&Xform,parent_guid:&str,xforms:&mut Vec<proto::XformEntry>,placed:&mut HashSet<ObjectId>)->Result<Option<proto::TreeNode>, &'static str> {
    let original=&doc.nodes[index];if original.derived {return Ok(None);}
    let mut node=original.source.clone();node.guid=original.portable_guid.clone();node.parent_guid=parent_guid.to_owned();let mut matrix=parent*&original.placement.local;let mut local=original.placement.local.clone();
    if let Some(id)=original.object {
        let Some(guid)=scene.row_guid(id) else {return Ok(None);};
        node.name=guid.to_owned();matrix=scene.placement(id).ok_or("Saved source placement is missing")?;local=crate::nested_placement::saved_local(&original.placement,parent,&matrix)?;
        placed.insert(id);
    }
    if local.m!=Xform::identity().m {
        let guid=node.name.clone();if let Some(previous)=xforms.iter().find(|entry|entry.guid==guid) {if previous.xform.as_ref()!=Some(&local.to_proto()) {return Err("Different saved tree placements share a name");}}
        else {xforms.push(proto::XformEntry {guid,xform:Some(local.to_proto())});}
    }
    for child in index+1..original.end {if doc.nodes[child].parent==Some(index) {if let Some(child)=save_node(doc,child,scene,&matrix,&node.guid,xforms,placed)? {node.children.push(child);}}}
    Ok(Some(node))
}

#[derive(Clone)]
pub struct Row {pub key:String,pub label:String,pub depth:usize,pub end:usize,pub ids:Vec<ObjectId>,pub readonly:bool}
pub fn rows(scene:&Scene)->Vec<Row> {
    let mut rows=Vec::new();let mut placed=HashSet::new();
    for doc in &scene.forest.documents {
        let mut visible=Vec::new();let mut index=0;
        while index<doc.nodes.len() {
            let node=&doc.nodes[index];
            if node.object.is_some_and(|id|!scene.contains(id)) {index=node.end;continue;}
            visible.push(index);index+=1;
        }
        let start=rows.len();
        for &index in &visible {
            let node=&doc.nodes[index];let ids=doc.nodes[index..node.end].iter().filter_map(|n|n.object).filter(|id|scene.contains(*id)).collect::<Vec<_>>();
            placed.extend(node.object);let end=start+visible.partition_point(|i|*i<node.end);
            rows.push(Row {key:format!("{}/{}",doc.key,node.source.guid),label:if index==0 {doc.label.clone()} else {node.label.clone()},depth:node.depth,end,ids,readonly:node.readonly});
        }
    }
    for row in scene.objects().iter().filter(|r|!placed.contains(&r.id)) {let index=rows.len();rows.push(Row {key:format!("created/{}",row.id.value()),label:row.metadata.name.clone(),depth:0,end:index+1,ids:vec![row.id],readonly:false});}
    let other=scene.lines().iter().map(|r|(r.id,r.prepared.source.name.as_str())).chain(scene.paths().iter().map(|r|(r.id,r.prepared.source.name()))).chain(scene.points().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.texts().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.clouds().iter().map(|r|(r.id,r.prepared.source.name.as_str()))).chain(scene.streams().iter().map(|r|(r.id,r.prepared.source.name.as_str())));
    for (id,label) in other.filter(|(id,_)|!placed.contains(id)) {let index=rows.len();rows.push(Row {key:format!("created/{}",id.value()),label:label.to_owned(),depth:0,end:index+1,ids:vec![id],readonly:false});}
    rows
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-06.rs"
```

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-07.rs"
```

</details>

### 4. `src/scene_archive.rs`

Only original tree objects enter each archive namespace. Do not reclassify derived paths as loose original objects.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{archive::{Bundle,Metadata,Document},scene::{ObjectId,Scene}};
use prost::Message;
use std::{collections::HashSet,rc::Rc};

// A namespace is one source document; imported files can reuse original GUIDs while their placements and presentation remain independent.
pub(crate) struct Namespace {pub name:String,pub doc:Option<Rc<crate::hierarchy::Document>>,pub ids:HashSet<ObjectId>}
impl Namespace {pub fn ordered(&self)->Vec<ObjectId> {let mut ids=self.ids.iter().copied().collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());ids}}
fn inventory(scene:&Scene)->Vec<Namespace> {
    let mut out=scene.forest.documents.iter().map(|doc|Namespace {name:doc.label.clone(),doc:Some(doc.clone()),ids:doc.nodes.iter().filter_map(|node|node.object).filter(|id|scene.contains(*id)).collect()}).collect::<Vec<_>>();
    let all=scene.objects().iter().map(|row|row.id).chain(scene.lines().iter().map(|row|row.id)).chain(scene.paths().iter().map(|row|row.id)).chain(scene.points().iter().map(|row|row.id)).chain(scene.clouds().iter().map(|row|row.id)).chain(scene.texts().iter().map(|row|row.id)).chain(scene.sheets().iter().map(|row|row.id)).collect::<HashSet<_>>();
    let assigned=out.iter().flat_map(|namespace|namespace.ids.iter().copied()).collect::<HashSet<_>>();let ids=all.difference(&assigned).copied().collect::<HashSet<_>>();if !ids.is_empty() {out.push(Namespace {name:"Created".into(),doc:None,ids});}out
}

pub fn save(scene:&Scene)->Result<Vec<u8>,String> {
    if !scene.streams().is_empty() {return Err("This scene contains streamed sources. Open complete source documents before saving an editable session.".into());}
    let mut namespaces=HashSet::new();let mut documents=Vec::new();let mut bytes_used=0usize;
    let mut metadata=Metadata {created_doc:None,current_layer:None,documents:vec![],hidden:vec![],locked:vec![],colors:vec![],edge_colors:Some(vec![]),texts:vec![],groups:vec![]};
    for (index,namespace) in inventory(scene).iter().enumerate() {
        for id in &namespace.ids {if !namespaces.insert(*id) {return Err("Source owner occurs in two saved documents".into());}}
        let (source,names,point_px)=crate::archive_document::prepare(scene,namespace)?;crate::source_validate::session(&source)?;let bytes=source.encode_to_vec();bytes_used=bytes_used.checked_add(bytes.len()).ok_or("Session size overflow")?;if bytes_used>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
        crate::archive_presentation::append(&mut metadata,index,namespace,scene,&names)?;
        documents.push(bytes);metadata.documents.push(Document {name:namespace.name.clone(),place:session_rust::Xform::identity().m,point_px});
    }
    for row in scene.texts() {let mut label=row.label(false,false).map_err(str::to_owned)?;label.object=None;label.id=row.prepared.source.label.id;metadata.texts.push((row.guid.clone(),label,scene.visible(row.id)));}
    metadata.hidden.sort();metadata.locked.sort();metadata.colors.sort();metadata.edge_colors.as_mut().unwrap().sort();metadata.texts.sort_by(|a,b|a.0.cmp(&b.0));Bundle {documents,metadata}.encode()
}

#[cfg(test)]
mod tests {
    use super::*;
    use {session_rust::proto,std::collections::HashMap,std::rc::Rc};
    #[test]
    fn mixed_archive_keeps_original_source_types_copy_ids_and_precise_models() {
        let mut scene=Scene::demo();scene.clear();let cad=scene.insert_cad(Rc::new(crate::cad_display::example())).unwrap();let cloud=Rc::new(crate::cloud::example());let _=cloud.guid();let before=cloud.to_proto();let cloud_id=scene.insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap()).unwrap();let copy=scene.copy_row(cloud_id,false).unwrap();scene.place_cloud(copy,session_rust::Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        Rc::make_mut(&mut scene.view).hide(&[cad],true);let saved=Bundle::decode(&save(&scene).unwrap()).unwrap();let mut found=HashMap::new();
        for (index,bytes) in saved.documents.iter().enumerate() {let source=proto::Session::decode(bytes.as_slice()).unwrap();let objects=source.objects.unwrap();assert!(objects.meshes.is_empty());for c in objects.pointclouds {found.insert(c.guid.clone(),c);}if !objects.breps.is_empty() {assert_eq!(objects.breps[0].is_visible,Some(false));assert!(saved.metadata.hidden.iter().any(|(doc,guid)|*doc==index&&guid==scene.row_guid(cad).unwrap()));}if let Some(entry)=source.xforms.iter().find(|entry|entry.guid==scene.row_guid(copy).unwrap()) {assert_eq!(entry.xform.as_ref().unwrap().matrix[12],1_000_000_000.125);}}
        assert_eq!(found.len(),2);assert_eq!(found[scene.row_guid(cloud_id).unwrap()].coords,before.coords);assert_eq!(found[scene.row_guid(copy).unwrap()].point_ids,before.point_ids);assert_eq!(cloud.to_proto(),before);
    }
    #[test]
    fn repeated_imported_documents_keep_independent_guid_namespaces_and_flags() {
        let mut scene=Scene::demo();scene.clear();let bytes=crate::specimen::bytes();scene.import(crate::document::load(&bytes).unwrap()).unwrap();let per_document=scene.objects().len();scene.import(crate::document::load(&bytes).unwrap()).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[per_document].id;assert_ne!(scene.row_guid(a),scene.row_guid(b));assert_eq!(scene.objects()[0].geometry().unwrap().guid(),scene.objects()[per_document].geometry().unwrap().guid());
        Rc::make_mut(&mut scene.view).hide(&[a],true);Rc::make_mut(&mut scene.view).lock(&[b],true);let bundle=Bundle::decode(&save(&scene).unwrap()).unwrap();assert_eq!(bundle.documents.len(),2);assert_eq!(bundle.metadata.hidden.len(),1);assert_eq!(bundle.metadata.locked.len(),1);assert_ne!(bundle.metadata.hidden[0].0,bundle.metadata.locked[0].0);assert_eq!(bundle.metadata.hidden[0].1,bundle.metadata.locked[0].1);
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{archive::{Bundle,Metadata,Document},scene::{ObjectId,Scene}};
use prost::Message;
use std::{collections::HashSet,rc::Rc};

// A namespace is one source document; imported files can reuse original GUIDs while their placements and presentation remain independent.
pub(crate) struct Namespace {pub name:String,pub doc:Option<Rc<crate::hierarchy::Document>>,pub ids:HashSet<ObjectId>}
impl Namespace {pub fn ordered(&self)->Vec<ObjectId> {let mut ids=self.ids.iter().copied().collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());ids}}
fn inventory(scene:&Scene)->Vec<Namespace> {
    let mut out=scene.forest.documents.iter().map(|doc|Namespace {name:doc.label.clone(),doc:Some(doc.clone()),ids:doc.nodes.iter().filter(|node|!node.derived).filter_map(|node|node.object).filter(|id|scene.contains(*id)).collect()}).collect::<Vec<_>>();
    let all=scene.objects().iter().map(|row|row.id).chain(scene.lines().iter().map(|row|row.id)).chain(scene.paths().iter().map(|row|row.id)).chain(scene.points().iter().map(|row|row.id)).chain(scene.clouds().iter().map(|row|row.id)).chain(scene.texts().iter().map(|row|row.id)).chain(scene.sheets().iter().map(|row|row.id)).collect::<HashSet<_>>();
    let assigned=out.iter().flat_map(|namespace|namespace.ids.iter().copied()).collect::<HashSet<_>>();let ids=all.difference(&assigned).copied().filter(|id|!scene.forest.derived(*id)).collect::<HashSet<_>>();if !ids.is_empty() {out.push(Namespace {name:"Created".into(),doc:None,ids});}out
}

pub fn save(scene:&Scene)->Result<Vec<u8>,String> {
    if !scene.streams().is_empty() {return Err("This scene contains streamed sources. Open complete source documents before saving an editable session.".into());}
    let mut namespaces=HashSet::new();let mut documents=Vec::new();let mut bytes_used=0usize;
    let mut metadata=Metadata {created_doc:None,current_layer:None,documents:vec![],hidden:vec![],locked:vec![],colors:vec![],edge_colors:Some(vec![]),texts:vec![],groups:vec![]};
    for (index,namespace) in inventory(scene).iter().enumerate() {
        for id in &namespace.ids {if !namespaces.insert(*id) {return Err("Source owner occurs in two saved documents".into());}}
        let (source,names,point_px)=crate::archive_document::prepare(scene,namespace)?;crate::source_validate::session(&source)?;let bytes=source.encode_to_vec();bytes_used=bytes_used.checked_add(bytes.len()).ok_or("Session size overflow")?;if bytes_used>crate::archive::LIMIT {return Err("Session exceeds the 512 MiB file limit".into());}
        crate::archive_presentation::append(&mut metadata,index,namespace,scene,&names)?;
        documents.push(bytes);metadata.documents.push(Document {name:namespace.name.clone(),place:session_rust::Xform::identity().m,point_px});
    }
    for row in scene.texts() {let mut label=row.label(false,false).map_err(str::to_owned)?;label.object=None;label.id=row.prepared.source.label.id;metadata.texts.push((row.guid.clone(),label,scene.visible(row.id)));}
    metadata.hidden.sort();metadata.locked.sort();metadata.colors.sort();metadata.edge_colors.as_mut().unwrap().sort();metadata.texts.sort_by(|a,b|a.0.cmp(&b.0));Bundle {documents,metadata}.encode()
}

#[cfg(test)]
mod tests {
    use super::*;
    use {session_rust::proto,std::collections::HashMap,std::rc::Rc};
    #[test]
    fn mixed_archive_keeps_original_source_types_copy_ids_and_precise_models() {
        let mut scene=Scene::demo();scene.clear();let cad=scene.insert_cad(Rc::new(crate::cad_display::example())).unwrap();let cloud=Rc::new(crate::cloud::example());let _=cloud.guid();let before=cloud.to_proto();let cloud_id=scene.insert_cloud(crate::cloud::Prepared::new(cloud.clone()).unwrap()).unwrap();let copy=scene.copy_row(cloud_id,false).unwrap();scene.place_cloud(copy,session_rust::Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        Rc::make_mut(&mut scene.view).hide(&[cad],true);let saved=Bundle::decode(&save(&scene).unwrap()).unwrap();let mut found=HashMap::new();
        for (index,bytes) in saved.documents.iter().enumerate() {let source=proto::Session::decode(bytes.as_slice()).unwrap();let objects=source.objects.unwrap();assert!(objects.meshes.is_empty());for c in objects.pointclouds {found.insert(c.guid.clone(),c);}if !objects.breps.is_empty() {assert_eq!(objects.breps[0].is_visible,Some(false));assert!(saved.metadata.hidden.iter().any(|(doc,guid)|*doc==index&&guid==scene.row_guid(cad).unwrap()));}if let Some(entry)=source.xforms.iter().find(|entry|entry.guid==scene.row_guid(copy).unwrap()) {assert_eq!(entry.xform.as_ref().unwrap().matrix[12],1_000_000_000.125);}}
        assert_eq!(found.len(),2);assert_eq!(found[scene.row_guid(cloud_id).unwrap()].coords,before.coords);assert_eq!(found[scene.row_guid(copy).unwrap()].point_ids,before.point_ids);assert_eq!(cloud.to_proto(),before);
    }
    #[test]
    fn repeated_imported_documents_keep_independent_guid_namespaces_and_flags() {
        let mut scene=Scene::demo();scene.clear();let bytes=crate::specimen::bytes();scene.import(crate::document::load(&bytes).unwrap()).unwrap();let per_document=scene.objects().len();scene.import(crate::document::load(&bytes).unwrap()).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[per_document].id;assert_ne!(scene.row_guid(a),scene.row_guid(b));assert_eq!(scene.objects()[0].geometry().unwrap().guid(),scene.objects()[per_document].geometry().unwrap().guid());
        Rc::make_mut(&mut scene.view).hide(&[a],true);Rc::make_mut(&mut scene.view).lock(&[b],true);let bundle=Bundle::decode(&save(&scene).unwrap()).unwrap();assert_eq!(bundle.documents.len(),2);assert_eq!(bundle.metadata.hidden.len(),1);assert_eq!(bundle.metadata.locked.len(),1);assert_ne!(bundle.metadata.hidden[0].0,bundle.metadata.locked[0].0);assert_eq!(bundle.metadata.hidden[0].1,bundle.metadata.locked[0].1);
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-08.rs"
```

```text
--8<-- "journey/code/96k-separate-derived-rows-from-editable-elements-09.rs"
```

</details>

</details>

<a id="checkpoint-96l-attach-a-bounded-derived-feature-tree"></a>

### Step 4 of 10 · Attach a bounded feature tree to its host (`96l-attach-a-bounded-derived-feature-tree`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/10 Visual Trace (96l-attach-a-bounded-derived-feature-tree): Attach a bounded feature tree to its host</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/layer_edit.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Build read-only attributes, interaction and piece rows under the owning Element. Validate the complete tree before replacing its saved structure.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/layer_edit.rs` (*Reuse the existing branch transaction and node/depth limits for feature rows.*)

Convert the feature plan into temporary branches beneath the owning Element.

Groups contain groups or pieces; every new node carries derived and read-only provenance. Merge into an existing attributes/interactions group when one exists, preserving its original children. Flatten and check the complete node count and depth before replacing the document hierarchy.

> **Self-Check Question:** *Why preserve an existing attributes group?*  
> **Answer:** It may contain original geometry or independent child Elements. Only the new projected descendants are disposable; replacing the whole group would destroy source structure.

- **What You Should See:** The validated hierarchy adds read-only feature groups beneath their owning original Element.
- **Hands-On Lab Experiment:** Compare the original attributes group before and after projection. Its identity and child Element should remain, with the new feature group appended.

<details>
<summary>Implementation 96l-attach-a-bounded-derived-feature-tree: Attach a bounded feature tree to its host</summary>

Build read-only attributes, interaction and piece rows under the owning Element. Validate the complete tree before replacing its saved structure.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/layer_edit.rs`

Reuse the existing branch transaction and node/depth limits for feature rows.

<details>
<summary>Locate the existing block</summary>

```rust
 let mut count=0;let mut max=0;fn size(b:&Branch,depth:usize,count:&mut usize,max:&mut usize){*count+=1;*max=(*max).max(depth);for c in &b.children{size(c,depth+1,count,max)}}size(&root,0,&mut count,&mut max);if count>crate::hierarchy::MAX_NODES||max>crate::hierarchy::MAX_DEPTH{return Err("Element attributes exceed hierarchy limits")}replace(scene,owner,root);Ok(())
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96l-attach-a-bounded-derived-feature-tree-01.rs" skip=371
```

</details>

<a id="checkpoint-96m-project-original-feature-outlines"></a>

### Step 5 of 10 · Project original outlines without changing their coordinates (`96m-project-original-feature-outlines`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/10 Visual Trace (96m-project-original-feature-outlines): Project original outlines without changing their coordinates</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/feature_display.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Create black display paths from original feature outlines, apply the host placement once, and keep attributes Off and interactions On by default.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/feature_display.rs` (*Give this operation its own small owner.*) · `src/lib.rs` (*Register the new owner without changing the browser entry point.*)

Display preparation clones each original outline, gives the display copy its own identity, and prepares a black path.

Apply the host placement once after preparation. Temporarily detach the hierarchy/current-layer routing while allocating pieces so these paths cannot be inserted into a user layer as independent originals. Restore routing before attaching the bounded feature tree. Import and editor history provide the surrounding rollback transaction.

> **Self-Check Question:** *Which transform belongs on a generated piece?*  
> **Answer:** The already resolved host placement. Original outline coordinates stay local, and applying an additional parent placement would move nested features twice.

- **What You Should See:** Placed original outlines produce derived black paths with independent attribute and interaction visibility defaults.
- **Hands-On Lab Experiment:** Use a translated parent and a translated child Element. Compare each feature path’s placement with its own host, then repeat at a large world coordinate.

<details>
<summary>Implementation 96m-project-original-feature-outlines: Project original outlines without changing their coordinates</summary>

Create black display paths from original feature outlines, apply the host placement once, and keep attributes Off and interactions On by default.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/feature_display.rs`

Give this operation its own small owner.

Create the file and type:

```rust
--8<-- "journey/code/96m-project-original-feature-outlines-01.rs"
```

### 2. `src/lib.rs`

Register the new owner without changing the browser entry point.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod inventory_tests;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod feature_display;

#[cfg(test)]
mod inventory_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96m-project-original-feature-outlines-02.rs"
```

</details>

</details>

<a id="checkpoint-96n-connect-imported-interaction-hierarchies"></a>

### Step 6 of 10 · Connect imported Elements to their feature hierarchy (`96n-connect-imported-interaction-hierarchies`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/10 Visual Trace (96n-connect-imported-interaction-hierarchies): Connect imported Elements to their feature hierarchy</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/scene_document.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Import a document atomically, classify its editable child Elements, then attach feature projections from its original graph and retained Element payloads.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/scene_document.rs` (*Connect projections only after original tree placement and source retention are ready.*)

Connect projection only after the candidate import has retained its original header, built its hierarchy and resolved original placements.

Identify original Element owners, classify their editable child boundaries, then append each Element’s feature display using the source Session graph. A failure aborts the candidate; the open scene remains intact.

> **Self-Check Question:** *Why append features to the import candidate?*  
> **Answer:** The original import is already atomic. Invalid outline data or a hierarchy-limit failure can discard the candidate without leaving partially created paths in the user’s drawing.

- **What You Should See:** Accepted imports retain editable children and attach the validated original interaction hierarchy atomically.
- **Hands-On Lab Experiment:** Corrupt one feature outline with a non-finite coordinate. Verify that import returns an error and preserves the existing scene and its paths.

<details>
<summary>Implementation 96n-connect-imported-interaction-hierarchies: Connect imported Elements to their feature hierarchy</summary>

Import a document atomically, classify its editable child Elements, then attach feature projections from its original graph and retained Element payloads.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/scene_document.rs`

Connect projections only after original tree placement and source retention are ready.

<details>
<summary>Locate the existing block</summary>

```rust
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let elements = owners.iter().filter_map(|(guid, id, _)|
        matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_))).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) = source.document.lookup.get(guid) {
            crate::feature_display::append(&mut candidate, *id, &source.document, element)
                .map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96n-connect-imported-interaction-hierarchies-01.rs"
```

</details>

</details>

<a id="checkpoint-96o-reuse-projections-for-created-elements"></a>

### Step 7 of 10 · Use the same projection path for created Elements (`96o-reuse-projections-for-created-elements`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 7/10 Visual Trace (96o-reuse-projections-for-created-elements): Use the same projection path for created Elements</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/element_adopt.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Created Elements and imported Elements now share feature display ownership. Visibility commands stop at independent child Elements and collect each feature piece once.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/element_adopt.rs` (*Replace serialized display copies with the shared feature projection and preserve independent child owners.*)

Use the same feature projection for newly created Elements.

Keep original base-plane attributes as source nodes, then add generated feature pieces through the shared adapter. Visibility collection stops at independent editable child Elements and gathers each read-only piece once, so a parent’s Attributes command cannot hide a child’s geometry.

> **Self-Check Question:** *Why stop visibility traversal at an editable child?*  
> **Answer:** That child owns its geometry and its own attribute/interactions groups. The parent’s attribute switch must not become a second visibility owner for the child object.

- **What You Should See:** Created Elements use the existing projection owner, with independent child visibility preserved.
- **Hands-On Lab Experiment:** Toggle Attributes and Interactions separately on a document with a nested child Element. Confirm that the child remains visible and editable.

<details>
<summary>Implementation 96o-reuse-projections-for-created-elements: Use the same projection path for created Elements</summary>

Created Elements and imported Elements now share feature display ownership. Visibility commands stop at independent child Elements and collect each feature piece once.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/element_adopt.rs`

Replace serialized display copies with the shared feature projection and preserve independent child owners.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn adopt(scene: &mut Scene, made: Vec<Made>, consumed: &[ObjectId]) -> Result<Vec<ObjectId>, &'static str> {
    let mut ids = Vec::new();
    for item in made {
        let id = crate::element_original::insert(scene, item.source.clone())?;
        ids.push(id);
        let mut attributes = Vec::new();
        let mut interactions = Vec::new();
        if let Some(plane) = item.plane {
            attributes.push(crate::plane_original::insert(scene, Rc::new(plane))?);
        }
        // Own axes and sections are attributes; contact, joint, cut and drill geometry comes from interactions.
        for feature in &item.source.features {
            let interaction = matches!(feature.feature_type.as_str(), "contact" | "joint" | "cut" | "drill");
            for outline in &feature.outlines {
                let mut source = outline.clone();
                source.set_guid(uuid::Uuid::new_v4().to_string());
                source.name = format!("{} {}", feature.feature_type, feature.name);
                let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                if interaction { interactions.push(piece); } else { attributes.push(piece); }
            }
        }
        if !attributes.is_empty() {
            Rc::make_mut(&mut scene.view).hide(&attributes, true);
            crate::layer_edit::attach_attributes(scene, id, attributes, "attributes")?;
        }
        if !interactions.is_empty() {
            crate::layer_edit::attach_attributes(scene, id, interactions, "interactions")?;
        }
    }
    for id in scene.with_descendants(consumed) { scene.remove(id); }
    Ok(ids)
}

fn group_ids(scene: &Scene, name: &str) -> Vec<ObjectId> {
    scene.forest.documents.iter().flat_map(|doc| {
        doc.nodes.iter().enumerate().filter(move |(_, node)| node.source.name == name && node.readonly)
            .flat_map(|(index, node)| doc.nodes[index + 1..node.end].iter().filter_map(|node| node.object))
    }).collect()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96o-reuse-projections-for-created-elements-01.rs"
```

</details>

<a id="checkpoint-96p-build-an-imported-interaction-specimen"></a>

### Step 8 of 10 · Build an original interaction specimen (`96p-build-an-imported-interaction-specimen`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 8/10 Visual Trace (96p-build-an-imported-interaction-specimen): Build an original interaction specimen</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/interaction_example.rs, src/lib.rs, examples/sample.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Build a small original document with a host, an editable nested child, and a connector. Its contact, unnamed joint and orphan feature make the hierarchy inspectable without a large floor model.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/interaction_example.rs` (*Author the original geometry, graph and feature associations used by the exercise.*) · `src/lib.rs` (*Expose the specimen to native examples and tests.*) · `examples/sample.rs` (*Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.*)

Build a small original Session that exercises the complete bridge: a named contact, an unnamed feature-producing relation, an unmatched cut, and a nested editable child Element.

The sample keeps unknown interaction payloads and original feature GUIDs. Its Open example uses these exact serialized originals; generated display paths are created only by the viewer.

> **Self-Check Question:** *Why include an unmatched cut and a child inside attributes?*  
> **Answer:** They expose two common shortcuts: silently dropping features that lack graph matches, and marking every descendant of attributes read-only.

- **What You Should See:** Download the [supplied interaction sample.pb](../../fixtures/interaction-96p.pb). Type Open in the browser and choose it. Expand the host: inspect the InteractionContactFace seam, InteractionFeatureSolid connector and unclaimed cut. Keep the child selectable and compare Element Attributes On with Element Interactions Off.
- **Hands-On Lab Experiment:** Open sample.pb, expand Layers All, and follow host → interactions → full relation type/name → feature → piece. Locate the independently editable child under attributes.

<details>
<summary>Implementation 96p-build-an-imported-interaction-specimen: Build an original interaction specimen</summary>

Build a small original document with a host, an editable nested child, and a connector. Its contact, unnamed joint and orphan feature make the hierarchy inspectable without a large floor model.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

### 1. `src/interaction_example.rs`

Author the original geometry, graph and feature associations used by the exercise.

Create the file and type:

```rust
--8<-- "journey/code/96p-build-an-imported-interaction-specimen-01.rs"
```

### 2. `src/lib.rs`

Expose the specimen to native examples and tests.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod inventory_tests;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod interaction_example;

#[cfg(test)]
mod inventory_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96p-build-an-imported-interaction-specimen-02.rs"
```

</details>

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 3. `examples/sample.rs`

Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.

<details>
<summary>Locate the existing block</summary>

```text
fn main() -> std::io::Result<()> {
    std::fs::write("sample.pb", viewer_journey::specimen::precise_bytes())
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96p-build-an-imported-interaction-specimen-03.rs"
```

</details>

</details>

<a id="checkpoint-96q-prove-imported-feature-ownership-and-roundtrips"></a>

### Step 9 of 10 · Prove imported feature ownership and round trips (`96q-prove-imported-feature-ownership-and-roundtrips`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 9/10 Visual Trace (96q-prove-imported-feature-ownership-and-roundtrips): Prove imported feature ownership and round trips</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/interaction_bridge_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bounded feature plan</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">derived display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">source-preserving archive</text>
    
</svg>
</div>

- **Goal:** Verify that feature projections follow their owner once, preserve original payloads, survive Delete/Undo, and rebuild after Save/Open without serializing display copies. Reject invalid outlines without changing the current document.
- **Data Flow Trace:** `original source → bounded feature plan → derived display → source-preserving archive`
- **Files You Touch:** `src/interaction_bridge_tests.rs` (*Run supplied regression fixtures against original source payloads and the connected import path.*) · `src/lib.rs` (*Include the ownership and archive proofs in the native suite.*)

Exercise the connected import through original source comparisons, hierarchy ownership, independent visibility, nested transforms, Delete/Undo, and repeated archives.

Decode Mesh messages before comparing their fields because protobuf map serialization order is not stable. Compare the remaining Element payload exactly, including empty feature GUIDs and original outlines. Generated paths must remain six after every Save/Open and must never appear as saved original polylines.

> **Self-Check Question:** *What makes a successful round-trip stronger than equal object counts?*  
> **Answer:** The saved document must retain the original Element fields and opaque interactions, omit generated source geometry, restore editable child ownership, and regenerate exactly one set of feature pieces.

- **What You Should See:** Inspect the four imported-feature regression tests, then repeat the Open sample.pb exercise from the preceding checkpoint. Keep the contact and child Element visible while switching the independent display controls.
- **Hands-On Lab Experiment:** In Chrome, toggle both Element controls, delete/undo the child, and Save/Close/Open twice. Confirm the same hierarchy without accumulated pieces.

<details>
<summary>Implementation 96q-prove-imported-feature-ownership-and-roundtrips: Prove imported feature ownership and round trips</summary>

Verify that feature projections follow their owner once, preserve original payloads, survive Delete/Undo, and rebuild after Save/Open without serializing display copies. Reject invalid outlines without changing the current document.

Data flow: original source → bounded feature plan → derived display → source-preserving archive.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/interaction_bridge_tests.rs`

Run supplied regression fixtures against original source payloads and the connected import path.

Copy this supplied file:

```text
--8<-- "journey/code/96q-prove-imported-feature-ownership-and-roundtrips-01.rs"
```

</details>

### 2. `src/lib.rs`

Include the ownership and archive proofs in the native suite.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod inventory_tests;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod interaction_bridge_tests;

#[cfg(test)]
mod inventory_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96q-prove-imported-feature-ownership-and-roundtrips-02.rs"
```

</details>

</details>

<a id="checkpoint-96r-preserve-exact-command-completions"></a>

### Step 10 of 10 · Prove exact commands across the expanded vocabulary (`96r-preserve-exact-command-completions`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 10/10 Visual Trace (96r-preserve-exact-command-completions): Prove exact commands across the expanded vocabulary</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/command_vocabulary.rs, src/panel.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">typed characters</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">command vocabulary</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">exact match first</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">egui submission</text>
    
</svg>
</div>

- **Goal:** Share the existing dictionary wrapper with the real widget regression and browser typing proof.
- **Data Flow Trace:** `typed characters → command vocabulary → exact match first → egui submission`
- **Files You Touch:** `src/command_vocabulary.rs` (*Move the already-typed private Commands wrapper from panel.rs into this shared module; retain its existing methods.*) · `src/panel.rs` (*Use the exact-first vocabulary in the actual browser panel.*)

The first vocabulary lesson already preserves exact words.

Move the current Commands wrapper into the shared module unchanged, so the native widget fixture and the browser use the same dictionary. Inspect the supplied character-by-character and Enter regression for the expanded CAD command list. No second completion algorithm is introduced.

> **Self-Check Question:** *Does a larger command registry justify changing a fully typed command?*  
> **Answer:** No. Reuse the exact-word ranking written at03cd, and verify that both inline typing and Enter preserve the expanded command vocabulary.

- **What You Should See:** Type View Isometric and press Enter. The history must contain that exact command. Then type View Isometric Back and verify that the longer command remains available.
- **Hands-On Lab Experiment:** Try Line beside its longer command names, and View Isometric beside View Isometric Back. Check both the fully typed field and the submitted history entry. Check Select Object beside Selection: a space after Select must continue the intended command.

<details>
<summary>Implementation 96r-preserve-exact-command-completions: Prove exact commands across the expanded vocabulary</summary>

Share the existing dictionary wrapper with the real widget regression and browser typing proof.

Data flow: typed characters → command vocabulary → exact match first → egui submission.

### 1. `src/command_vocabulary.rs`

Move the already-typed private Commands wrapper from panel.rs into this shared module; retain its existing methods.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
.sort_by_key(|name| {
        if name.eq_ignore_ascii_case(line) { 0 }
        else if name.get(line.len()..).is_some_and(|rest| rest.starts_with(' ')) { 1 }
        else { 2 }
    });
    choices
}
```

</details>

Move the existing code after that block:

```text
--8<-- "journey/code/96r-preserve-exact-command-completions-move-existing-wrapper.rs" skip=200
```

</details>

### 2. `src/command_vocabulary.rs`

Import the existing dock trait and expose the shared wrapper and its input list to the panel.

<details>
<summary>Locate the existing block</summary>

```rust
struct Commands(Vec<&'static str>);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::command_dock;

pub(crate) struct Commands(pub(crate) Vec<&'static str>);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96r-preserve-exact-command-completions-wire-moved-wrapper.rs"
```

</details>

### 3. `src/panel.rs`

Use the exact-first vocabulary in the actual browser panel.

<details>
<summary>Locate the existing block</summary>

```rust
struct Commands(Vec<&'static str>);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        crate::commands::canonical(line)
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96r-preserve-exact-command-completions-02.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/command_vocabulary.rs`

Feed character-by-character input and Enter through the actual egui command field.

<details>
<summary>Locate the existing block</summary>

```text
        self.completions(line)
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96r-preserve-exact-command-completions-04.rs" skip=40
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 96r-preserve-exact-command-completions
npm --prefix ../session_tests run course -- check 96r-preserve-exact-command-completions
npm --prefix ../session_tests run course -- save 96r-preserve-exact-command-completions
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Type View Isometric and press Enter. The history must contain that exact command. Then type View Isometric Back and verify that the longer command remains available.

- **Final Lesson Experiment:** Try Line beside its longer command names, and View Isometric beside View Isometric Back. Check both the fully typed field and the submitted history entry. Check Select Object beside Selection: a space after Select must continue the intended command.

Continue with [Original owners across selection, copying and display](../../consolidation/96s-resolve-feature-rows-to-original-element-owners.md).
