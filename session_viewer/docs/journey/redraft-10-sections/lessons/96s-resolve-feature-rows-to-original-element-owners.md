# 19.3 · Original owners across selection, copying and display

**Section 10: Shared GPU Arenas, Floating-Origin Precision, Multisampling, and Production Parity** · **Lesson 19.3** (Redraft `10.3`) · **8 checkpoints (`96s-resolve-feature-rows-to-original-element-owners` → `96z-preserve-source-feature-visibility-and-pixel-styles`)** · **Study time: 4–8 h**

> **What You Will Learn & Build:** Trace each derived feature back to the nearest original Element without crossing an independent child Element. By the final checkpoint (`96z-preserve-source-feature-visibility-and-pixel-styles`), you will project only visible original features, with two-pixel lines and twelve-pixel point discs.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 19.3 Pipeline Schematic: Original owners across selection, copying and display</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (8 Checkpoints: 96s-resolve-feature-rows-to-original-element-owners → 96z-preserve-source-feature-visibility-and-pixel-styles)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">feature row</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">nearest original</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">Element</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">editing selection</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">and derived</text>
    <text x="419" y="140" text-anchor="middle" font-size="11" fill="#111111">highlight</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">source feature</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">visibility</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">coincident-point</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">classification</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 19.3:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/feature_owner.rs  ·  src/lib.rs  ·  src/tree_panel.rs  ·  src/editor.rs  ·  src/feature_owner_tests.rs  ·  src/camera.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">8 Checkpoints: 96s-resolve-feature-rows-to-original-element-owners → 96z-preserve-source-feature-visibility-and-pixel-styles — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`96s-resolve-feature-rows-to-original-element-owners`](#checkpoint-96s-resolve-feature-rows-to-original-element-owners) | Trace each derived feature back to the nearest original Element without crossing an independent child Element. | `src/feature_owner.rs`, `src/lib.rs` | `feature row → nearest original Element → editing selection and derived highlight` |
| **2** | [`96t-select-and-highlight-original-feature-owners`](#checkpoint-96t-select-and-highlight-original-feature-owners) | Connect feature-row clicks to original-owner selection and the existing editing/history path. | `src/tree_panel.rs`, `src/editor.rs`, `src/feature_owner_tests.rs`, `src/lib.rs` | `feature row → nearest original Element → editing selection and derived highlight` |
| **3** | [`96u-fit-original-box-corners-to-the-free-canvas`](#checkpoint-96u-fit-original-box-corners-to-the-free-canvas) | Replace sphere fitting with the current projected-corner constraint and five-percent margin. | `src/camera.rs`, `src/projected_fit_tests.rs`, `src/lib.rs` | `original bounds → camera-axis offsets → free-rectangle constraints → distance` |
| **4** | [`96v-project-definition-and-per-instance-features`](#checkpoint-96v-project-definition-and-per-instance-features) | Reuse feature projection while keeping shared definitions and separate instance ownership. | `src/instance_features.rs`, `src/lib.rs`, `src/scene_document.rs`, `src/feature_owner.rs` | `shared definition features + per-instance features → display-only projection → instance placement once → original instance selection` |
| **5** | [`96w-detach-copied-original-source-identity`](#checkpoint-96w-detach-copied-original-source-identity) | Detach copied original source identity | `src/scene.rs` | `original source → fresh copy identity → original placement → regenerated display` |
| **6** | [`96x-regenerate-display-for-copied-feature-owners`](#checkpoint-96x-regenerate-display-for-copied-feature-owners) | Regenerate display for copied feature owners | `src/layer_edit.rs`, `src/copy_feature_tests.rs`, `src/lib.rs` | `original source → fresh copy identity → original placement → regenerated display` |
| **7** | [`96y-address-feature-pixels-to-original-cad-owners`](#checkpoint-96y-address-feature-pixels-to-original-cad-owners) | Address feature pixels to original CAD owners | `src/feature_owner.rs`, `src/pick_sources.rs`, `src/feature_pick_tests.rs`, `src/lib.rs` | `original source → fresh copy identity → original placement → regenerated display` |
| **8** | [`96z-preserve-source-feature-visibility-and-pixel-styles`](#checkpoint-96z-preserve-source-feature-visibility-and-pixel-styles) | Project only visible original features, with two-pixel lines and twelve-pixel point discs. | `src/feature_display.rs`, `src/feature_style_tests.rs`, `src/lib.rs`, `src/pick_gpu_tests.rs` | `source feature visibility → coincident-point classification → existing marker/path display → original owner` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-96s-resolve-feature-rows-to-original-element-owners"></a>

### Step 1 of 8 · Resolve feature rows to original Element owners (`96s-resolve-feature-rows-to-original-element-owners`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/8 Visual Trace (96s-resolve-feature-rows-to-original-element-owners): Resolve feature rows to original Element owners</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/feature_owner.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">feature row</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">nearest original Element</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">editing selection and derived</text>
    <text x="694" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">highlight</text>
    
</svg>
</div>

- **Goal:** Trace each derived feature back to the nearest original Element without crossing an independent child Element.
- **Data Flow Trace:** `feature row → nearest original Element → editing selection and derived highlight`
- **Files You Touch:** `src/feature_owner.rs` (*Resolve ownership through bounded hierarchy parent links and collect visible display highlights.*) · `src/lib.rs` (*Compile the owner resolver alongside the existing scene and hierarchy.*)

The import now has read-only display pieces, but the maintained panel can select their owner.

Walk parent links from the clicked row and stop at the first original Element. Check that owner through the ordinary editable-object predicate. A hidden or locked owner therefore cannot be reached through its feature rows.

Use the same walk to collect visible derived paths for highlighting. Stop at an independent child Element, so selecting the host does not highlight that child’s own feature. Selection still contains only editable originals; the wider list is consumed only by the renderer.

> **Self-Check Question:** *Does clicking a projected feature make the display path editable?*  
> **Answer:** No. Resolve the original Element that owns the feature. Selection, camera Fit and editing target that owner; generated paths only contribute visible highlight.

- **What You Should See:** Keep the imported specimen and select feature rows through their original Element owner.
- **Hands-On Lab Experiment:** Trace a host feature and an independent child through the ownership helper. Stop at the nearest original Element. This preparation retains the previous browser selection behavior until the [input connection](../../96t-select-and-highlight-original-feature-owners.md).

<details>
<summary>Implementation 96s-resolve-feature-rows-to-original-element-owners: Resolve feature rows to original Element owners</summary>

Trace each derived feature back to the nearest original Element without crossing an independent child Element.

Data flow: feature row → nearest original Element → editing selection and derived highlight.

### 1. `src/feature_owner.rs`

Resolve ownership through bounded hierarchy parent links and collect visible display highlights.

Create the file and type:

```rust
--8<-- "journey/code/96s-resolve-feature-rows-to-original-element-owners-01.rs"
```

### 2. `src/lib.rs`

Compile the owner resolver alongside the existing scene and hierarchy.

<details>
<summary>Locate the existing block</summary>

```rust
mod command_vocabulary;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96s-resolve-feature-rows-to-original-element-owners-02.rs" skip=24
```

</details>

<a id="checkpoint-96t-select-and-highlight-original-feature-owners"></a>

### Step 2 of 8 · Select and highlight original feature owners (`96t-select-and-highlight-original-feature-owners`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/8 Visual Trace (96t-select-and-highlight-original-feature-owners): Select and highlight original feature owners</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/tree_panel.rs, src/editor.rs, src/feature_owner_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">feature row</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">nearest original Element</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">editing selection and derived</text>
    <text x="694" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">highlight</text>
    
</svg>
</div>

- **Goal:** Connect feature-row clicks to original-owner selection and the existing editing/history path.
- **Data Flow Trace:** `feature row → nearest original Element → editing selection and derived highlight`
- **Files You Touch:** `src/tree_panel.rs` (*Resolve an eligible original owner independently of the displayed feature IDs.*) · `src/editor.rs` (*Extend only renderer highlighting; retain the original editing selection.*) · `src/feature_owner_tests.rs` (*Supply original-owner, child-boundary, hidden/locked and Delete/Undo ownership regressions.*) · `src/lib.rs` (*Run the supplied regressions in the native suite.*)

Keep row IDs for visibility lamps: those IDs still describe the displayed pieces. Give the panel a separate resolved owner for selection. Its selection button and selected state use that owner; clicking a regular child Element continues to use the child’s editable ID.

The editor’s renderer highlight list includes visible derived paths owned by selected Elements, while selected_objects remains unchanged. Delete and Move therefore edit original owners and their descendants once. Undo restores their original identities, and Save continues to omit the generated paths.

Report a preview change after selection and Fit. The browser then refreshes the original-owner selection and generated highlight buffers without recording a source edit. A view-only result would leave the newly selected owner’s GPU highlight stale.

> **Self-Check Question:** *Which original does a connected feature-row click select?*  
> **Answer:** The nearest owning Element from the [resolution helper](../../96s-resolve-feature-rows-to-original-element-owners.md). The existing selection and editing path then handles that original.

- **What You Should See:** Feature-row selection highlights its original owner and uses the same editing/history path.
- **Hands-On Lab Experiment:** Open sample.pb, enable Attributes, expand Layers, and compare clicks on a host feature and on the independent child Element. Hidden and locked owners must not become selected through a feature row.

<details>
<summary>Implementation 96t-select-and-highlight-original-feature-owners: Select and highlight original feature owners</summary>

Connect feature-row clicks to original-owner selection and the existing editing/history path.

Data flow: feature row → nearest original Element → editing selection and derived highlight.

### 1. `src/tree_panel.rs`

Resolve an eligible original owner independently of the displayed feature IDs. Cache the resolved owner with the row’s presentation state. Feature-row selection follows its original owner. Enable eligible feature rows without making their display objects editable. Send the original owner through the existing tree selection/Fit action.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
}

#[derive(Clone,Copy,Default)]
struct RowState {hidden:bool,locked:bool,selectable:bool,current:bool,has_faces:bool,colours:[Option<[u8;3]>;2],group:bool,root:bool,holds_current:bool}

#[cfg(any(test,target_arch="wasm32"))]
fn properties(ui:&mut egui::Ui,key:&str,state:RowState,action:&mut Option<(String,crate::layer_properties::Property)>,menus:&mut Vec<egui::Rect>,controls:&mut Option<Vec<Control>>) {
```

</details>

Replace it with:

```rust
}

#[derive(Clone,Copy,Default)]
struct RowState {owner:Option<ObjectId>,hidden:bool,locked:bool,selectable:bool,current:bool,has_faces:bool,colours:[Option<[u8;3]>;2],group:bool,root:bool,holds_current:bool}

#[cfg(any(test,target_arch="wasm32"))]
fn properties(ui:&mut egui::Ui,key:&str,state:RowState,action:&mut Option<(String,crate::layer_properties::Property)>,menus:&mut Vec<egui::Rect>,controls:&mut Option<Vec<Control>>) {
```

<details>
<summary>Locate this block</summary>

```rust
                    let row=model.rows[index].clone();let state=model.states.get(&row.key).copied().unwrap_or_default();ui.push_id(&row.key,|ui|ui.horizontal(|ui| {
                        ui.add_space(row.depth.min(12) as f32*12.);
                        if row.end>index+1 {let arrow=ui.small_button(if model.expanded.contains(&row.key) {"▾"} else {"▸"});record(controls,&format!("tree/open/{}",row.key),"Expand or collapse",&arrow);if arrow.clicked() {if !model.expanded.insert(row.key.clone()) {model.expanded.remove(&row.key);}}} else {ui.add_space(18.);}
                        let selected=row.ids.iter().any(|id|model.selected.contains(id));
                        let label=if row.label.is_empty() {row.ids.first().map_or("Group".into(),|id|format!("Object {}",id.value()))} else {row.label.clone()};
                        let label_width=(ui.available_width()-90.).max(24.);
                        let button=ui.add_enabled_ui(state.group || !row.readonly && state.selectable,|ui|ui.add_sized([label_width,20.],egui::Button::new(format!("{label} ({})",row.ids.len())).selected(selected).truncate())).inner.on_hover_text(&label).on_disabled_hover_text("Source attribute: read only; edit its parent");
                        record(controls,&format!("tree/select/{}",row.key),&label,&button);
                        if button.clicked() && state.selectable {model.action=Some((row.ids.clone(),ui.input(|input|input.modifiers.shift)));}
                        if state.group {if let Some(menu)=egui::Popup::context_menu(&button).show(|ui| {
                            use crate::layer_edit::Edit;
                            for (name,edit,enabled) in [("Set current",Some(Edit::Current),true),("New layer",Some(Edit::New {sublayer:false}),true),("New sublayer",Some(Edit::New {sublayer:true}),true),("Duplicate layer",Some(Edit::Duplicate),!state.root),("Change object layer",Some(Edit::Move(model.selected.clone())),!model.selected.is_empty()),("Copy object layer",Some(Edit::Copy(model.selected.clone())),!model.selected.is_empty()),("Rename",None,!state.root),("Delete",Some(Edit::Delete),!state.root&&!state.holds_current)] {
```

</details>

Replace it with:

```rust
                    let row=model.rows[index].clone();let state=model.states.get(&row.key).copied().unwrap_or_default();ui.push_id(&row.key,|ui|ui.horizontal(|ui| {
                        ui.add_space(row.depth.min(12) as f32*12.);
                        if row.end>index+1 {let arrow=ui.small_button(if model.expanded.contains(&row.key) {"▾"} else {"▸"});record(controls,&format!("tree/open/{}",row.key),"Expand or collapse",&arrow);if arrow.clicked() {if !model.expanded.insert(row.key.clone()) {model.expanded.remove(&row.key);}}} else {ui.add_space(18.);}
                        let selected=state.owner.map_or_else(||row.ids.iter().any(|id|model.selected.contains(id)),|id|model.selected.contains(&id));
                        let label=if row.label.is_empty() {row.ids.first().map_or("Group".into(),|id|format!("Object {}",id.value()))} else {row.label.clone()};
                        let label_width=(ui.available_width()-90.).max(24.);
                        let button=ui.add_enabled_ui(state.group || state.owner.is_some() || !row.readonly && state.selectable,|ui|ui.add_sized([label_width,20.],egui::Button::new(format!("{label} ({})",row.ids.len())).selected(selected).truncate())).inner.on_hover_text(&label).on_disabled_hover_text("Source attribute: read only; edit its parent");
                        record(controls,&format!("tree/select/{}",row.key),&label,&button);
                        if button.clicked() && state.selectable {model.action=Some((state.owner.map_or_else(||row.ids.clone(),|id|vec![id]),ui.input(|input|input.modifiers.shift)));}
                        if state.group {if let Some(menu)=egui::Popup::context_menu(&button).show(|ui| {
                            use crate::layer_edit::Edit;
                            for (name,edit,enabled) in [("Set current",Some(Edit::Current),true),("New layer",Some(Edit::New {sublayer:false}),true),("New sublayer",Some(Edit::New {sublayer:true}),true),("Duplicate layer",Some(Edit::Duplicate),!state.root),("Change object layer",Some(Edit::Move(model.selected.clone())),!model.selected.is_empty()),("Copy object layer",Some(Edit::Copy(model.selected.clone())),!model.selected.is_empty()),("Rename",None,!state.root),("Delete",Some(Edit::Delete),!state.root&&!state.holds_current)] {
```

<details>
<summary>Locate this block</summary>

```rust
        self.witness=None;
        self.graph=crate::graph_edit::rows(scene);
        for doc in &scene.forest.documents{for(index,node)in doc.nodes.iter().enumerate(){if scene.groups.contains(&(doc.key,node.source.guid.clone()))&&doc.nodes[index+1..node.end].iter().filter_map(|n|n.object).any(|id|selected.contains(&id)){let mut parent=Some(index);while let Some(i)=parent{self.expanded.insert(format!("{}/{}",doc.key,doc.nodes[i].source.guid));parent=doc.nodes[i].parent;}}}}
        self.rows=crate::hierarchy::rows(scene);self.selected=selected;self.states=self.rows.iter().map(|row|(row.key.clone(),RowState {hidden:scene.view.lamps.contains(&row.key)||!row.ids.is_empty()&&row.ids.iter().all(|id|scene.view.hidden.contains(id)),locked:!row.ids.is_empty()&&row.ids.iter().all(|id|scene.view.locked.contains(id)),selectable:row.ids.iter().any(|id|scene.pick_id(id.value()).is_some()),current:crate::layer_edit::current_key(scene).as_ref()==Some(&row.key),has_faces:row.ids.iter().any(|id|scene.objects().iter().any(|mesh|mesh.id==*id)),colours:[row.ids.iter().find_map(|id|scene.view.faces.get(id).copied()),row.ids.iter().find_map(|id|scene.view.edges.get(id).copied())],group:crate::layer_edit::locate(scene,&row.key).is_ok(),root:row.depth==0,holds_current:crate::layer_edit::holds_current(scene,&row.key)})).collect();
    }
    pub fn rename_result(&mut self,error:Option<&str>) {if let Some(error)=error {self.rename_error=error.into();self.rename_focus=true;} else {self.rename=None;self.rename_error.clear();}}
    pub fn all(&mut self) {self.open=true;self.expanded.extend(self.rows.iter().enumerate().filter(|(index,r)|r.end>*index+1).map(|(_,r)|r.key.clone()));self.page=0;}
```

</details>

Replace it with:

```rust
        self.witness=None;
        self.graph=crate::graph_edit::rows(scene);
        for doc in &scene.forest.documents{for(index,node)in doc.nodes.iter().enumerate(){if scene.groups.contains(&(doc.key,node.source.guid.clone()))&&doc.nodes[index+1..node.end].iter().filter_map(|n|n.object).any(|id|selected.contains(&id)){let mut parent=Some(index);while let Some(i)=parent{self.expanded.insert(format!("{}/{}",doc.key,doc.nodes[i].source.guid));parent=doc.nodes[i].parent;}}}}
        self.rows=crate::hierarchy::rows(scene);self.selected=selected;self.states=self.rows.iter().map(|row|(row.key.clone(),RowState {hidden:scene.view.lamps.contains(&row.key)||!row.ids.is_empty()&&row.ids.iter().all(|id|scene.view.hidden.contains(id)),locked:!row.ids.is_empty()&&row.ids.iter().all(|id|scene.view.locked.contains(id)),selectable:crate::feature_owner::row_owner(scene,&row.key).is_some() || row.ids.iter().any(|id|scene.pick_id(id.value()).is_some()),owner:crate::feature_owner::row_owner(scene,&row.key),current:crate::layer_edit::current_key(scene).as_ref()==Some(&row.key),has_faces:row.ids.iter().any(|id|scene.objects().iter().any(|mesh|mesh.id==*id)),colours:[row.ids.iter().find_map(|id|scene.view.faces.get(id).copied()),row.ids.iter().find_map(|id|scene.view.edges.get(id).copied())],group:crate::layer_edit::locate(scene,&row.key).is_ok(),root:row.depth==0,holds_current:crate::layer_edit::holds_current(scene,&row.key)})).collect();
    }
    pub fn rename_result(&mut self,error:Option<&str>) {if let Some(error)=error {self.rename_error=error.into();self.rename_focus=true;} else {self.rename=None;self.rename_error.clear();}}
    pub fn all(&mut self) {self.open=true;self.expanded.extend(self.rows.iter().enumerate().filter(|(index,r)|r.end>*index+1).map(|(_,r)|r.key.clone()));self.page=0;}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{hierarchy::Row,scene::ObjectId};
#[cfg(any(test,target_arch="wasm32"))]
use crate::command_dock::{Control,record};
use std::collections::HashSet;

#[derive(Default)]
pub struct TreePanel {witness:Option<crate::tree_witness::Witness>,pub refreshes:u64,pub graph_open:bool,pub graph:Vec<(String,String,String,String)>,pub edit:Option<(String,crate::layer_edit::Edit)>,pub rename:Option<(String,String)>,pub rename_error:String,rename_focus:bool,pub confirm:Option<(String,usize)>,pub property:Option<(String,crate::layer_properties::Property)>,pub menu_rects:Vec<egui::Rect>,states:std::collections::HashMap<String,RowState>,pub open:bool,pub rows:Vec<Row>,pub selected:Vec<ObjectId>,expanded:HashSet<String>,pub rect:Option<egui::Rect>,pub action:Option<(Vec<ObjectId>,bool)>,page:usize}
impl TreePanel {
    pub fn refresh_cached(&mut self,scene:&crate::scene::Scene,selected:Vec<ObjectId>,epoch:u64){
        if self.witness.as_ref().is_some_and(|w|w.matches(scene,&selected,epoch)){return;}
        self.refresh(scene,selected.clone());self.witness=Some(crate::tree_witness::Witness::new(scene,&selected,epoch));self.refreshes+=1;
    }

    pub fn refresh(&mut self,scene:&crate::scene::Scene,selected:Vec<ObjectId>) {
        self.witness=None;
        self.graph=crate::graph_edit::rows(scene);
        for doc in &scene.forest.documents{for(index,node)in doc.nodes.iter().enumerate(){if scene.groups.contains(&(doc.key,node.source.guid.clone()))&&doc.nodes[index+1..node.end].iter().filter_map(|n|n.object).any(|id|selected.contains(&id)){let mut parent=Some(index);while let Some(i)=parent{self.expanded.insert(format!("{}/{}",doc.key,doc.nodes[i].source.guid));parent=doc.nodes[i].parent;}}}}
        self.rows=crate::hierarchy::rows(scene);self.selected=selected;self.states=self.rows.iter().map(|row|(row.key.clone(),RowState {hidden:scene.view.lamps.contains(&row.key)||!row.ids.is_empty()&&row.ids.iter().all(|id|scene.view.hidden.contains(id)),locked:!row.ids.is_empty()&&row.ids.iter().all(|id|scene.view.locked.contains(id)),selectable:crate::feature_owner::row_owner(scene,&row.key).is_some() || row.ids.iter().any(|id|scene.pick_id(id.value()).is_some()),owner:crate::feature_owner::row_owner(scene,&row.key),current:crate::layer_edit::current_key(scene).as_ref()==Some(&row.key),has_faces:row.ids.iter().any(|id|scene.objects().iter().any(|mesh|mesh.id==*id)),colours:[row.ids.iter().find_map(|id|scene.view.faces.get(id).copied()),row.ids.iter().find_map(|id|scene.view.edges.get(id).copied())],group:crate::layer_edit::locate(scene,&row.key).is_ok(),root:row.depth==0,holds_current:crate::layer_edit::holds_current(scene,&row.key)})).collect();
    }
    pub fn rename_result(&mut self,error:Option<&str>) {if let Some(error)=error {self.rename_error=error.into();self.rename_focus=true;} else {self.rename=None;self.rename_error.clear();}}
    pub fn all(&mut self) {self.open=true;self.expanded.extend(self.rows.iter().enumerate().filter(|(index,r)|r.end>*index+1).map(|(_,r)|r.key.clone()));self.page=0;}
    pub fn visible(&self)->Vec<usize> {
        let mut out=Vec::new();let mut index=0;
        while index<self.rows.len() {out.push(index);let row=&self.rows[index];index=if self.expanded.contains(&row.key) {index+1} else {row.end.max(index+1)};}
        out
    }
}

#[cfg(any(test,target_arch="wasm32"))]
pub fn draw(root:&mut egui::Ui,model:&mut TreePanel,controls:&mut Option<Vec<Control>>) {
    model.rect=None;model.menu_rects.clear();if !model.open {return;}
    let visible=model.visible();let pages=visible.len().div_ceil(128).max(1);model.page=model.page.min(pages-1);
    let response=egui::Area::new("session-tree".into()).anchor(egui::Align2::RIGHT_TOP,egui::vec2(-8.,8.)).order(egui::Order::Foreground).show(root.ctx(),|ui| {
        egui::Frame::group(ui.style()).fill(egui::Color32::from_rgb(248,248,248)).show(ui,|ui| {
            ui.set_width((root.available_width()/3.).clamp(240.,413.33334));ui.set_height((root.max_rect().height()-32.).max(60.));ui.horizontal(|ui| {ui.strong("Layers");let close=ui.button("×");record(controls,"tree/close","Close Layers",&close);if close.clicked() {model.open=false;}});
            dialogs(ui,model,controls);
            ui.horizontal(|ui|{let button=ui.selectable_label(model.graph_open,format!("Graph ({})",model.graph.len()));record(controls,"tree/graph","Graph",&button);if button.clicked(){model.graph_open=!model.graph_open;}});
            if model.graph_open{egui::ScrollArea::vertical().id_salt("graph-table").max_height(150.).show(ui,|ui|{egui::Grid::new("graph-edges").striped(true).show(ui,|ui|{for title in ["Document","From","To","Attribute"]{ui.strong(title);}ui.end_row();for(doc,from,to,attribute)in model.graph.iter().take(128){for word in [doc,from,to,attribute]{ui.label(word);}ui.end_row();}});if model.graph.len()>128{ui.label(format!("Showing first 128 of {} edges",model.graph.len()));}});}
            if pages>1 {ui.horizontal(|ui| {if ui.button("Previous").clicked() {model.page=model.page.saturating_sub(1);}ui.label(format!("{} / {pages}",model.page+1));if ui.button("Next").clicked() {model.page=(model.page+1).min(pages-1);}});}
            let height=(root.max_rect().height()-100.).max(60.);
            egui::ScrollArea::vertical().max_height(height).show(ui,|ui| {
                for &index in visible.iter().skip(model.page*128).take(128) {
                    let row=model.rows[index].clone();let state=model.states.get(&row.key).copied().unwrap_or_default();ui.push_id(&row.key,|ui|ui.horizontal(|ui| {
                        ui.add_space(row.depth.min(12) as f32*12.);
                        if row.end>index+1 {let arrow=ui.small_button(if model.expanded.contains(&row.key) {"▾"} else {"▸"});record(controls,&format!("tree/open/{}",row.key),"Expand or collapse",&arrow);if arrow.clicked() {if !model.expanded.insert(row.key.clone()) {model.expanded.remove(&row.key);}}} else {ui.add_space(18.);}
                        let selected=state.owner.map_or_else(||row.ids.iter().any(|id|model.selected.contains(id)),|id|model.selected.contains(&id));
                        let label=if row.label.is_empty() {row.ids.first().map_or("Group".into(),|id|format!("Object {}",id.value()))} else {row.label.clone()};
                        let label_width=(ui.available_width()-90.).max(24.);
                        let button=ui.add_enabled_ui(state.group || state.owner.is_some() || !row.readonly && state.selectable,|ui|ui.add_sized([label_width,20.],egui::Button::new(format!("{label} ({})",row.ids.len())).selected(selected).truncate())).inner.on_hover_text(&label).on_disabled_hover_text("Source attribute: read only; edit its parent");
                        record(controls,&format!("tree/select/{}",row.key),&label,&button);
                        if button.clicked() && state.selectable {model.action=Some((state.owner.map_or_else(||row.ids.clone(),|id|vec![id]),ui.input(|input|input.modifiers.shift)));}
                        if state.group {if let Some(menu)=egui::Popup::context_menu(&button).show(|ui| {
                            use crate::layer_edit::Edit;
                            for (name,edit,enabled) in [("Set current",Some(Edit::Current),true),("New layer",Some(Edit::New {sublayer:false}),true),("New sublayer",Some(Edit::New {sublayer:true}),true),("Duplicate layer",Some(Edit::Duplicate),!state.root),("Change object layer",Some(Edit::Move(model.selected.clone())),!model.selected.is_empty()),("Copy object layer",Some(Edit::Copy(model.selected.clone())),!model.selected.is_empty()),("Rename",None,!state.root),("Delete",Some(Edit::Delete),!state.root&&!state.holds_current)] {
                                let button=ui.add_enabled(enabled,egui::Button::new(name));record(controls,&format!("layer/menu/{}/{name}",row.key),name,&button);
                                if button.clicked() {
                                    if name=="Rename" {model.rename=Some((row.key.clone(),row.label.clone()));model.rename_error.clear();model.rename_focus=true;}
                                    else if name=="Delete"&&!row.ids.is_empty() {model.confirm=Some((row.key.clone(),row.ids.len()));}
                                    else {model.edit=Some((row.key.clone(),edit.unwrap()));}
                                    ui.close();
                                }
                            }
                        }) {model.menu_rects.push(menu.response.rect);}}
                        properties(ui,&row.key,state,&mut model.property,&mut model.menu_rects,controls);
                    }));
                }
            });
        });
    });model.rect=model.open.then_some(response.response.rect);
}

#[derive(Clone,Copy,Default)]
struct RowState {owner:Option<ObjectId>,hidden:bool,locked:bool,selectable:bool,current:bool,has_faces:bool,colours:[Option<[u8;3]>;2],group:bool,root:bool,holds_current:bool}

#[cfg(any(test,target_arch="wasm32"))]
fn properties(ui:&mut egui::Ui,key:&str,state:RowState,action:&mut Option<(String,crate::layer_properties::Property)>,menus:&mut Vec<egui::Rect>,controls:&mut Option<Vec<Control>>) {
    use crate::layer_properties::Property;
    for (name,label,property,enabled) in [("hide",if state.current {"✓"} else if state.hidden {"○"} else {"●"},Property::Hidden(!state.hidden),!state.current),("lock",if state.locked {"🔒"} else {"🔓"},Property::Locked(!state.locked),true)] {
        let button=ui.add_enabled(enabled,egui::Button::new(label)).on_hover_text(if name=="hide" {"Switch this row’s lamp; preserve the children’s own lamps"} else {"Lock or unlock selection of this row and children"});record(controls,&format!("layer/{name}/{key}"),name,&button);
        if button.clicked() {*action=Some((key.to_owned(),property));}
    }
    let button=ui.button("■").on_hover_text("Object colours; Faces and Edges are independent");record(controls,&format!("layer/colour/{key}"),"Colour",&button);
    if let Some(menu)=egui::Popup::menu(&button).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
        let channel_id=egui::Id::new(("layer-colour-channel",key));
        let mut edge=ui.ctx().data_mut(|data|data.get_temp::<bool>(channel_id).unwrap_or(false))&&state.has_faces;
        if state.has_faces {ui.horizontal(|ui| {for (name,value) in [("Faces",false),("Edges",true)] {let button=ui.selectable_label(edge==value,name);record(controls,&format!("layer/channel/{key}/{name}"),name,&button);if button.clicked() {edge=value;}}});} else {ui.label("Object and child colours");}
        ui.ctx().data_mut(|data|data.insert_temp(channel_id,edge));
        let original=ui.button("Original");record(controls,&format!("layer/original/{key}/{edge}"),"Original",&original);if original.clicked() {*action=Some((key.to_owned(),Property::Colour {edge,rgb:None}));ui.close();}
        if let Some(rgb)=crate::layer_palette::draw(ui,key,edge,state.colours[usize::from(edge)],controls) {*action=Some((key.to_owned(),Property::Colour {edge,rgb:Some(rgb)}));}
    }) {menus.push(menu.response.rect);}
}

#[cfg(any(test,target_arch="wasm32"))]
fn dialogs(ui:&mut egui::Ui,model:&mut TreePanel,controls:&mut Option<Vec<Control>>) {
    if let Some((key,text))=model.rename.as_mut() {
        let id=egui::Id::new("layer-name-input");
        if std::mem::take(&mut model.rename_focus) {ui.memory_mut(|memory|memory.request_focus(id));}
        let cancel=ui.input_mut(|input|input.consume_key(egui::Modifiers::NONE,egui::Key::Escape));
        let mut submit=ui.input_mut(|input|input.consume_key(egui::Modifiers::NONE,egui::Key::Enter));
        ui.horizontal(|ui| {let field=ui.add(egui::TextEdit::singleline(text).id(id).char_limit(128).desired_width(130.));record(controls,"layer/name","Layer name",&field);let apply=ui.button("Rename");record(controls,"layer/name/apply","Rename",&apply);submit|=apply.clicked();});
        if !model.rename_error.is_empty() {ui.colored_label(egui::Color32::from_rgb(170,30,30),&model.rename_error);}
        if submit {model.edit=Some((key.clone(),crate::layer_edit::Edit::Rename(text.clone())));}
        if cancel {model.rename=None;model.rename_error.clear();ui.memory_mut(|memory|memory.surrender_focus(id));}
    }
    if let Some((key,count))=model.confirm.clone() {
        ui.label(format!("Delete this layer and {count} objects?"));
        ui.horizontal(|ui| {for (name,yes) in [("Delete",true),("Cancel",false)] {let button=ui.button(name);record(controls,&format!("layer/delete/{name}"),name,&button);if button.clicked() {if yes {model.edit=Some((key.clone(),crate::layer_edit::Edit::Delete));}model.confirm=None;}}});
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-01.rs"
```

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-02.rs"
```

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-03.rs"
```

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-04.rs"
```

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-05.rs"
```

</details>

### 2. `src/editor.rs`

Extend only renderer highlighting; retain the original editing selection.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(e)=&self.element{return e.ids()}if let Some(trim)=&self.trim{return if trim.phase==crate::trim_draft::Phase::Parts{Vec::new()}else if trim.phase==crate::trim_draft::Phase::Targets{trim.targets.iter().map(|o|o.id).collect()}else{trim.cutters.iter().map(|o|o.id).collect()}}if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||self.selected_objects(),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn highlighted_objects(&self)->Vec<ObjectId>{if let Some(e)=&self.element{return e.ids()}if let Some(trim)=&self.trim{return if trim.phase==crate::trim_draft::Phase::Parts{Vec::new()}else if trim.phase==crate::trim_draft::Phase::Targets{trim.targets.iter().map(|o|o.id).collect()}else{trim.cutters.iter().map(|o|o.id).collect()}}if let Some(split)=&self.split{return std::iter::once(split.target.id).chain(split.cutters.iter().map(|o|o.id)).collect()}if let Some(extend)=self.draft.as_ref().and_then(|d|d.extend.as_ref()){return extend.boundaries.iter().map(|o|o.id).collect()}self.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||crate::feature_owner::highlighted(&self.scene,&self.selected_objects()),|g|{g.progress.picked().iter().filter_map(|row|g.curves.iter().find(|(id,_)|id.value()==*row).map(|(id,_)|*id)).collect()})}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-06.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/feature_owner_tests.rs`

Supply original-owner, child-boundary, hidden/locked and Delete/Undo ownership regressions.

Copy this supplied file:

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-07.rs"
```

</details>

### 4. `src/lib.rs`

Run the supplied regressions in the native suite.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod feature_owner;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-08.rs" skip=23
```

### 5. `src/editor.rs`

Selection and Fit must invalidate the renderer highlight even though original source data did not change.

<details>
<summary>Locate the existing block</summary>

```rust
            Action::TreePick {ids,additive}=>{if self.trimming_picks(){for id in ids{self.apply_action(Action::TrimPick(Some(id)))?;}return Ok(Change::View)}if self.trimming_parts(){return Err("Trim now needs a part click")}if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);return self.apply(Action::FitSelected);},
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            Action::TreePick {ids,additive}=>{if self.trimming_picks(){for id in ids{self.apply_action(Action::TrimPick(Some(id)))?;}return Ok(Change::View)}if self.trimming_parts(){return Err("Trim now needs a part click")}if self.split.is_some(){for id in ids{self.split_pick(Some(id))?;}return Ok(Change::View)}let eligible=ids.into_iter().filter(|id|self.scene.pick_id(id.value())==Some(*id)).collect::<Vec<_>>();if eligible.is_empty() {return Err("This tree row has no visible editable objects");}self.select_objects(eligible,additive);self.apply(Action::FitSelected)?;return Ok(Change::Preview);},
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96t-select-and-highlight-original-feature-owners-09.rs"
```

</details>

</details>

<a id="checkpoint-96u-fit-original-box-corners-to-the-free-canvas"></a>

### Step 3 of 8 · Fit original box corners to the free canvas (`96u-fit-original-box-corners-to-the-free-canvas`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/8 Visual Trace (96u-fit-original-box-corners-to-the-free-canvas): Fit original box corners to the free canvas</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/camera.rs, src/projected_fit_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original bounds</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">camera-axis offsets</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">free-rectangle constraints</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">distance</text>
    
</svg>
</div>

- **Goal:** Replace sphere fitting with the current projected-corner constraint and five-percent margin.
- **Data Flow Trace:** `original bounds → camera-axis offsets → free-rectangle constraints → distance`
- **Files You Touch:** `src/camera.rs` (*Solve the current projected box-corner distance in the existing camera.*) · `src/projected_fit_tests.rs` (*Supply independent visible-rectangle and thin-part regression witnesses.*) · `src/lib.rs` (*Run the supplied fit witnesses with the cumulative source.*)

Reuse the existing camera orientation, panel rectangle and original bounds.

Project each of the eight corner offsets onto right, up and forward. With forward pointing from the eye toward the target, a corner needs distance at least |x|/tx − z and |y|/ty − z. Every centered box corner has an opposite corner, so the maximum over all eight corners is unchanged when using +z. That symmetric form, followed by a five-percent margin, is the maintained Fit rule.

This upgrades the earlier sphere approximation in place. It introduces no second camera and changes no source geometry. Keep radius as the depth/zoom extent. A point box can move the target without inventing a nonzero extent; invalid bounds leave the pose unchanged. Current unit conversion, reversed depth and GPU rebasing remain separate connected changes.

> **Self-Check Question:** *Why can a bounding sphere leave a thin CAD part unnecessarily small?*  
> **Answer:** Its radius fits every direction, although the current view needs only the projected box corners. Solve the horizontal and vertical corner constraints in the camera basis, including depth, and use the largest required distance.

- **What You Should See:** Thin parts fill the available canvas; all box corners remain inside the rectangle left by the panels.
- **Hands-On Lab Experiment:** Open the imported specimen, enable Layers and select a feature row. Fit its original owner, then compare Front, Top and Isometric with a thin Box. Switch projection and Fit again; preserve original dimensions and selection.

<details>
<summary>Implementation 96u-fit-original-box-corners-to-the-free-canvas: Fit original box corners to the free canvas</summary>

Replace sphere fitting with the current projected-corner constraint and five-percent margin.

Data flow: original bounds → camera-axis offsets → free-rectangle constraints → distance.

### 1. `src/camera.rs`

Solve the current projected box-corner distance in the existing camera.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn fit(&mut self, bounds: &crate::bounds::Bounds) {
        let half_y = (HALF_FOV.tan()*(self.frame[3]-self.frame[1])).atan();
        let half_x = (self.aspect * HALF_FOV.tan()*(self.frame[2]-self.frame[0])).atan();
        let radius = bounds.radius();
        self.radius = if radius > 0.0 { radius } else { 1.0 };
        self.target = bounds.centre();
        let opening = match self.projection {
            Projection::Perspective => half_x.min(half_y).sin(),
            Projection::Orthographic => half_y.tan().min(half_x.tan()),
        };
        self.distance = self.radius * 1.1 / opening;
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96u-fit-original-box-corners-to-the-free-canvas-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/projected_fit_tests.rs`

Supply independent visible-rectangle and thin-part regression witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96u-fit-original-box-corners-to-the-free-canvas-02.rs"
```

</details>

### 3. `src/lib.rs`

Run the supplied fit witnesses with the cumulative source.

<details>
<summary>Locate the existing block</summary>

```rust
mod feature_owner_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96u-fit-original-box-corners-to-the-free-canvas-03.rs" skip=25
```

</details>

<a id="checkpoint-96v-project-definition-and-per-instance-features"></a>

### Step 4 of 8 · Project definition and per-instance features (`96v-project-definition-and-per-instance-features`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/8 Visual Trace (96v-project-definition-and-per-instance-features): Project definition and per-instance features</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/instance_features.rs, src/lib.rs, src/scene_document.rs, src/feature_owner.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">shared definition features</text>
    <text x="111" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">+…</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">display-only projection</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">instance placement once</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">original instance</text>
    <text x="726" y="81" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">selection</text>
    
</svg>
</div>

- **Goal:** Reuse feature projection while keeping shared definitions and separate instance ownership.
- **Data Flow Trace:** `shared definition features + per-instance features → display-only projection → instance placement once → original instance selection`
- **Files You Touch:** `src/instance_features.rs` (*Combine feature data in a temporary projection object; reuse the existing path and tree owner.*) · `src/lib.rs` (*Connect the instance adapter to the existing Session ownership types.*) · `src/scene_document.rs` (*Stop inherited feature read-only classification at each original instance.*) · `src/feature_owner.rs` (*Resolve generated paths to the original instance, using the same visible/unlocked pick guard.*) · `src/instance_feature_tests.rs` (*Supply original Session input and independent ownership/archive regression witnesses.*)

A definition owns reusable geometry and its own features.

Each InstanceRef may add features in that definition frame. Build a temporary feature-only Element with the instance GUID, append definition features first and instance features second, and pass it to the projection owner already written at96m. Do not duplicate the mesh or add another GPU display implementation.

Connect this projection after the original instance rows and hierarchy exist. The existing host placement transforms every generated path once. Treat instances as independent ownership boundaries: selecting a generated feature targets the original instance, and its visibility and locking still govern editing. The temporary Element is never inserted or saved; the Session definition and InstanceRef remain the archive sources.

Install the supplied binary specimen using the dependency command below, then open assets/pb/course-instance-features.pb from the learner project in the browser. It contains two placements of one Element definition, a shared axis and a distinct cut per instance. The second instance is locked. These are original Session data, not implementation code.

> **Self-Check Question:** *Why must per-instance features be projected after selecting the instance placement?*  
> **Answer:** They are authored in the definition frame but belong to one InstanceRef. Reusing the projection code places them once under that instance and preserves both shared geometry and independent original ownership.

- **What You Should See:** Both instances show their interaction outlines; enabling attributes reveals the shared definition axis at each placement.
- **Hands-On Lab Experiment:** Open the supplied instance specimen and Fit. Toggle Element Attributes and Element Interactions separately. Select the first instance’s cut row, Delete and Undo; the second placement remains. The locked second instance’s feature row must not become an editable selection. Save and reopen: each outline appears once.

<details>
<summary>Implementation 96v-project-definition-and-per-instance-features: Project definition and per-instance features</summary>

Reuse feature projection while keeping shared definitions and separate instance ownership.

Data flow: shared definition features + per-instance features → display-only projection → instance placement once → original instance selection.

### 1. `src/instance_features.rs`

Combine feature data in a temporary projection object; reuse the existing path and tree owner.

Create the file and type:

```rust
--8<-- "journey/code/96v-project-definition-and-per-instance-features-01.rs"
```

### 2. `src/lib.rs`

Connect the instance adapter to the existing Session ownership types.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod instance_plan;
```

</details>

After that block, type:

```rust
mod instance_features;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96v-project-definition-and-per-instance-features-02.rs"
```

</details>

### 3. `src/scene_document.rs`

Stop inherited feature read-only classification at each original instance. Append instance feature displays after attaching original owners to their imported tree.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                .map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    scene.restore(candidate);Ok(key)
}
```

</details>

Replace it with:

```rust
                .map_err(str::to_owned)?;
        }
    }
    for (_, id, _) in &owners {
        if let Some(owner) = candidate.instances.get(id).cloned() {
            crate::instance_features::append(&mut candidate, *id, &owner).map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    scene.restore(candidate);Ok(key)
}
```

<details>
<summary>Locate this block</summary>

```rust
    candidate.forest.original_placements(key,&source.document,place,&source.world);
    Rc::make_mut(candidate.forest.documents.last_mut().unwrap()).header=Some(source.origin);
    let elements = owners.iter().filter_map(|(guid, id, _)|
        matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_))).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
```

</details>

Replace it with:

```rust
    candidate.forest.original_placements(key,&source.document,place,&source.world);
    Rc::make_mut(candidate.forest.documents.last_mut().unwrap()).header=Some(source.origin);
    let elements = owners.iter().filter_map(|(guid, id, _)|
        (matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_)))
            || candidate.instances.contains_key(id)).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::scene::Scene;
use session_rust::Xform;
use std::rc::Rc;
#[cfg(test)]
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String> {
    let source=crate::document_source::prepare(bytes,place,point_px)?;
    let mut candidate=scene.clone();let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
    candidate.forest.add(key,if name.is_empty() {source.document.name.clone()} else {name.into()},source.tree,&owners);
    candidate.forest.original_placements(key,&source.document,place,&source.world);
    Rc::make_mut(candidate.forest.documents.last_mut().unwrap()).header=Some(source.origin);
    let elements = owners.iter().filter_map(|(guid, id, _)|
        (matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_)))
            || candidate.instances.contains_key(id)).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) = source.document.lookup.get(guid) {
            crate::feature_display::append(&mut candidate, *id, &source.document, element)
                .map_err(str::to_owned)?;
        }
    }
    for (_, id, _) in &owners {
        if let Some(owner) = candidate.instances.get(id).cloned() {
            crate::instance_features::append(&mut candidate, *id, &owner).map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    scene.restore(candidate);Ok(key)
}

pub fn open(scene:&mut Scene,bytes:&[u8])->Result<(),String> {
    let bundle=crate::archive::Bundle::decode(bytes)?;
    let mut candidate=scene.clone();candidate.clear();let mut keys=Vec::new();
    for (source,meta) in bundle.documents.iter().zip(&bundle.metadata.documents) {keys.push(import(&mut candidate,source,&meta.name,&Xform::from_matrix(meta.place),meta.point_px)?);}
    crate::archive_restore_texts::restore(&mut candidate,&bundle.metadata)?;
    crate::archive_restore_flags::restore(&mut candidate,&bundle.metadata,&keys)?;
    scene.restore(candidate);Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_save_open_restores_original_sources_placements_layers_and_presentation() {
        let mut scene=Scene::demo();scene.clear();
        let cad=scene.insert_cad(Rc::new(crate::cad_display::example())).unwrap();
        let surface=scene.insert_surface(Rc::new(crate::surface::example())).unwrap();
        let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(crate::cloud::example())).unwrap()).unwrap();
        let copy=scene.copy_row(cloud,false).unwrap();scene.place_cloud(copy,Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
        scene.cloud_point_size(cloud,5.).unwrap();scene.cloud_point_size(copy,5.).unwrap();
        let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.123456789012345,0.,0.))).unwrap()).unwrap();
        let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-1.,0.,0.,1.,0.,0.))).unwrap()).unwrap();
        let label=crate::scene_label::example(crate::text::TextPlacement::WorldBillboard {world:[0.,0.,0.],world_height:0.1});
        let text=scene.insert_text(crate::scene_label::Prepared::new(Rc::new(label)).unwrap()).unwrap();scene.place_text(text,Xform::translation(0.123456789012345,0.,0.)).unwrap();
        let root=&scene.forest.documents[0];let current=format!("{}/{}",root.key,root.nodes[0].source.guid);
        let view=Rc::make_mut(&mut scene.view);view.current=Some(current);view.hide(&[cad,text],true);view.lock(&[surface,line,text],true);view.faces.insert(cad,[2,5,7]);view.edges.insert(cad,[0;3]);
        let mut restored=Scene::demo();let old_ids=restored.objects().iter().map(|row|row.id.value()).collect::<Vec<_>>();let bytes=crate::scene_archive::save(&scene).unwrap();open(&mut restored,&bytes).unwrap();
        assert_eq!(restored.objects().len(),2);assert_eq!(restored.objects().iter().filter(|row|row.cad.is_some()).count(),1);assert_eq!(restored.objects().iter().filter(|row|row.surface.is_some()).count(),1);
        assert_eq!(restored.clouds().len(),2);assert_eq!(restored.clouds()[1].model.m[12],1_000_000_000.125);assert_eq!(restored.clouds()[0].prepared.source.to_proto().coords,scene.clouds()[0].prepared.source.to_proto().coords);assert!(restored.clouds().iter().all(|row|row.point_px==5.&&row.prepared.source.point_size==3.));
        assert_eq!(restored.points()[0].prepared.source[0],scene.points()[0].prepared.source[0]);assert_eq!(restored.points()[0].prepared.source[0],0.123456789012345);let _=point;
        assert_eq!(restored.texts().len(),1);assert_eq!(restored.texts()[0].label(false,false).unwrap().placement,scene.texts()[0].label(false,false).unwrap().placement);assert!(!restored.visible(restored.texts()[0].id));assert!(restored.view.locked.contains(&restored.texts()[0].id));
        assert!(restored.view.current.is_some());assert!(restored.created.is_some());assert_eq!(restored.view.hidden.len(),2);assert_eq!(restored.view.locked.len(),3);assert_eq!(restored.view.faces[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[2,5,7]);assert_eq!(restored.view.edges[&restored.objects().iter().find(|row|row.cad.is_some()).unwrap().id],[0;3]);assert!(restored.objects().iter().all(|row|!old_ids.contains(&row.id.value())));
        let again=crate::archive::Bundle::decode(&crate::scene_archive::save(&restored).unwrap()).unwrap();assert_eq!(again.documents.len(),2);assert_eq!(again.metadata.hidden.len(),2);assert_eq!(again.metadata.locked.len(),3);
    }
    #[test]
    fn repeated_guid_documents_reopen_with_independent_flags_and_original_meshes() {
        let mut scene=Scene::demo();scene.clear();let raw=crate::specimen::bytes();scene.import(crate::document::load(&raw).unwrap()).unwrap();let n=scene.objects().len();scene.import(crate::document::load(&raw).unwrap()).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;Rc::make_mut(&mut scene.view).hide(&[a],true);Rc::make_mut(&mut scene.view).lock(&[b],true);
        let saved=crate::scene_archive::save(&scene).unwrap();let mut restored=Scene::demo();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);let a=&restored.objects()[0];let b=&restored.objects()[n];assert_ne!(a.guid,b.guid);assert_eq!(a.geometry().unwrap().guid(),b.geometry().unwrap().guid());assert!(!restored.visible(a.id));assert!(restored.visible(b.id));assert!(!restored.view.locked.contains(&a.id));assert!(restored.view.locked.contains(&b.id));
        let document=Rc::downgrade(&a.source().unwrap().document);let ids=restored.objects().iter().take(n).map(|row|row.id).collect::<Vec<_>>();for id in ids {assert!(restored.remove(id));}assert!(document.upgrade().is_none());
    }
    #[test]
    fn rejected_files_and_late_display_preparation_errors_leave_the_scene_unchanged() {
        let mut scene=Scene::demo();let ids=scene.objects().iter().map(|row|row.id).collect::<Vec<_>>();assert!(open(&mut scene,b"broken").is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.tree=None;source.objects.as_mut().unwrap().points.push(session_rust::Point::new(1e100,0.,0.).to_proto());let bytes=source.encode_to_vec();assert!(import(&mut scene,&bytes,"too large",&Xform::identity(),0.).is_err());assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);assert!(scene.points().is_empty());assert!(scene.forest.documents.iter().all(|doc|doc.label!="too large"));
        source.objects.as_mut().unwrap().points.clear();source.definitions=Some(proto::Objects {instances:vec![Default::default()],..Default::default()});assert!(import(&mut scene,&source.encode_to_vec(),"unsupported",&Xform::identity(),0.).unwrap_err().contains("definition"));assert_eq!(scene.objects().iter().map(|row|row.id).collect::<Vec<_>>(),ids);
    }
    #[test]
    fn document_placement_is_composed_without_mutating_original_source_coordinates() {
        let mut scene=Scene::demo();scene.clear();let cloud=crate::cloud::example();let _=cloud.guid();let original=cloud.to_proto();let source=proto::Session {name:"precise cloud".into(),objects:Some(proto::Objects {pointclouds:vec![original.clone()],..Default::default()}),..Default::default()};let raw=source.encode_to_vec();let place=Xform::translation(1_000_000_000.125,0.,0.);import(&mut scene,&raw,"placed",&place,7.).unwrap();assert_eq!(scene.clouds()[0].model.m[12],place.m[12]);assert_eq!(scene.clouds()[0].prepared.source.to_proto(),original);assert_eq!(scene.clouds()[0].point_px,7.);
    }
    #[test]
    fn repeated_roundtrips_keep_tree_depth_current_layer_source_header_and_precise_pose() {
        let mut source=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();source.guid="00000000-0000-0000-0000-000000000123".into();let raw=source.encode_to_vec();let mut scene=Scene::empty();import(&mut scene,&raw,"placed",&Xform::translation(0.123456789012345,0.,0.),0.).unwrap();let doc=&scene.forest.documents[0];let current=format!("{}/{}",doc.key,doc.nodes[0].source.guid);Rc::make_mut(&mut scene.view).current=Some(current);let nodes=doc.nodes.len();let depth=doc.nodes.iter().map(|node|node.depth).max().unwrap();
        for _ in 0..6 {let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();assert_eq!(proto::Session::decode(bundle.documents[0].as_slice()).unwrap().guid,source.guid);open(&mut scene,&saved).unwrap();assert_eq!(scene.forest.documents[0].nodes.len(),nodes);assert_eq!(scene.forest.documents[0].nodes.iter().map(|node|node.depth).max().unwrap(),depth);assert_eq!(scene.objects()[0].model.m[12],0.123456789012345);assert!(scene.view.current.is_some());}
    }
    #[test]
    fn moving_repeated_source_objects_into_one_layer_saves_distinct_editable_records() {
        let raw=crate::specimen::bytes();let mut scene=Scene::empty();import(&mut scene,&raw,"first",&Xform::identity(),0.).unwrap();let n=scene.objects().len();import(&mut scene,&raw,"second",&Xform::identity(),0.).unwrap();let a=scene.objects()[0].id;let b=scene.objects()[n].id;let doc=&scene.forest.documents[0];let key=format!("{}/{}",doc.key,doc.nodes[0].source.guid);crate::layer_edit::apply(&mut scene,&key,crate::layer_edit::Edit::Move(vec![b])).unwrap();
        let saved=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&saved).unwrap();let source=proto::Session::decode(bundle.documents[0].as_slice()).unwrap();let meshes=source.objects.unwrap().meshes;assert_eq!(meshes.len(),n+1);let guids=meshes.iter().map(|mesh|&mesh.guid).collect::<HashSet<_>>();assert_eq!(guids.len(),n+1);assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().guid(),scene.objects().iter().find(|row|row.id==b).unwrap().geometry().unwrap().guid());let mut restored=Scene::empty();open(&mut restored,&saved).unwrap();assert_eq!(restored.objects().len(),2*n);
    }

}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96v-project-definition-and-per-instance-features-03.rs"
```

```text
--8<-- "journey/code/96v-project-definition-and-per-instance-features-04.rs"
```

</details>

### 4. `src/feature_owner.rs`

Resolve generated paths to the original instance, using the same visible/unlocked pick guard.

<details>
<summary>Locate the existing block</summary>

```rust
            if scene.objects().iter().any(|row| row.id == id
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            if scene.instances.contains_key(&id) || scene.objects().iter().any(|row| row.id == id
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96v-project-definition-and-per-instance-features-05.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/instance_feature_tests.rs`

Supply original Session input and independent ownership/archive regression witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96v-project-definition-and-per-instance-features-06.rs"
```

</details>

### 6. `src/lib.rs`

Register the supplied regression fixtures.

<details>
<summary>Locate the existing block</summary>

```rust
mod projected_fit_tests;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod instance_feature_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96v-project-definition-and-per-instance-features-07.rs"
```

</details>

</details>

<a id="checkpoint-96w-detach-copied-original-source-identity"></a>

### Step 5 of 8 · Detach copied original source identity (`96w-detach-copied-original-source-identity`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 5/8 Visual Trace (96w-detach-copied-original-source-identity): Detach copied original source identity</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/scene.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">fresh copy identity</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original placement</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">regenerated display</text>
    
</svg>
</div>

- **Goal:** Detach copied original source identity
- **Data Flow Trace:** `original source → fresh copy identity → original placement → regenerated display`
- **Files You Touch:** `src/scene.rs` (*Detach copied original ownership without exposing mutable scene storage.*)

The row copier already assigns a new display identity. An original Element and an InstanceRef also carry Session GUIDs. Detach only that copied source with Rc::make_mut, then give it the new row GUID. Keep an instance definition shared: changing its GUID would affect every placement.

Keep this helper private inside Scene, beside copy_row. Call it after cloning the InstanceRef owner, while the new row exists. Update source metadata with the same GUID. The old source remains unchanged for Undo and archive comparison. This is a copy-on-write ownership lesson; it adds no new renderer or mesh upload.

> **Self-Check Question:** *Which identity must stay shared when an InstanceRef is copied?*  
> **Answer:** Its definition identity and geometry stay shared. The new InstanceRef and its row have a fresh identity, so saving does not merge two placements into one original.

- **What You Should See:** The browser continues to display original CAD objects and their projected features.
- **Hands-On Lab Experiment:** Copy the first instance in the supplied specimen. Save and reopen: the definition remains shared, and the two original references have different GUIDs. Predict which Rc is detached before checking the supplied fixture.

<details>
<summary>Implementation 96w-detach-copied-original-source-identity: Detach copied original source identity</summary>

Detach copied original source identity

Data flow: original source → fresh copy identity → original placement → regenerated display.

### 1. `src/scene.rs`

Detach copied original ownership without exposing mutable scene storage. Assign fresh original identity after copying the instance reference.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        else if let Some(row)=self.sheets.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.sheets.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        if let Some(owner)=self.instances.get(&id).cloned() {self.instances.insert(copy,owner);}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
        for channel in [&mut view.faces,&mut view.edges] {if let Some(rgb)=channel.get(&id).copied() {channel.insert(copy,rgb);}}
```

</details>

Replace it with:

```rust
        else if let Some(row)=self.sheets.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.sheets.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        if let Some(owner)=self.instances.get(&id).cloned() {self.instances.insert(copy,owner);}
        self.detach_copy_identity(copy);
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
        for channel in [&mut view.faces,&mut view.edges] {if let Some(rgb)=channel.get(&id).copied() {channel.insert(copy,rgb);}}
```

<details>
<summary>Locate this block</summary>

```rust
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
    }

    pub(crate) fn copy_row(&mut self,id:ObjectId,all:bool)->Result<ObjectId,&'static str> {
        if self.objects.iter().any(|row|row.id==id&&row.geometry().is_none()) {return Err("Reload editable sources before copying");}
        if !self.contains(id) {return Err("Object is gone");}
```

</details>

Replace it with:

```rust
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
    }

    fn detach_copy_identity(&mut self, id: ObjectId) {
        let instance = self.instances.contains_key(&id);
        if let Some(row) = self.objects.iter_mut().find(|row| row.id == id) {
            let element = !instance && matches!(row.original, Some(session_rust::Geometry::Element(_)));
            if element {
                if let Some(session_rust::Geometry::Element(source)) = row.original.as_mut() {
                    Rc::make_mut(source).set_guid(row.guid.clone());
                }
            }
            if instance || element { Rc::make_mut(&mut row.metadata).source_guid = row.guid.clone(); }
        }
        let guid = self.row_guid(id).map(str::to_owned);
        if let (Some(guid), Some(owner)) = (guid, self.instances.get_mut(&id)) {
            Rc::make_mut(&mut owner.instance).set_guid(guid);
        }
    }

    pub(crate) fn copy_row(&mut self,id:ObjectId,all:bool)->Result<ObjectId,&'static str> {
        if self.objects.iter().any(|row|row.id==id&&row.geometry().is_none()) {return Err("Reload editable sources before copying");}
        if !self.contains(id) {return Err("Object is gone");}
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{mesh::Mesh, prepared::PreparedMesh};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

impl ObjectId { pub fn value(self) -> u32 { self.0 } }

#[derive(Clone)]
pub struct Object {
    pub original:Option<session_rust::Geometry>,
    pub cad:Option<Rc<crate::cad_display::Prepared>>,
    pub surface:Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
    pub mesh: Rc<Mesh>,
    pub model: session_rust::Xform,
    editable: crate::edit_source::EditSource,
}

impl Object {
    pub fn visible(&self) -> bool { self.metadata.is_visible }

    pub fn geometry(&self) -> Option<&Rc<session_rust::Mesh>> {
        match &self.editable { crate::edit_source::EditSource::Loaded { geometry, .. } => Some(geometry), _ => None }
    }

    pub fn source(&self) -> Option<&crate::document::Source> {
        match &self.editable { crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref(), _ => None }
    }

    pub fn origin(&self) -> Option<&Rc<crate::origin::Origin>> {
        match &self.editable {
            crate::edit_source::EditSource::Loaded { source, .. } => source.as_ref().map(|source| &source.origin),
            crate::edit_source::EditSource::Released { origin, .. } => Some(origin),
        }
    }

    pub fn release_epoch(&self) -> Option<u64> {
        match &self.editable { crate::edit_source::EditSource::Released { epoch, .. } => Some(*epoch), _ => None }
    }

    pub fn can_unload(&self) -> bool {
        let Some(source) = self.source() else { return false; };
        let Some(geometry) = self.geometry() else { return false; };
        source.origin.location.is_some() && self.metadata.source_guid == source.guid
            && geometry.guid() == source.guid && self.metadata.name == geometry.name
            && self.metadata.is_visible == geometry.is_visible && self.metadata.is_locked == geometry.is_locked
            && source.document.objects.meshes.iter().any(|mesh| Rc::ptr_eq(mesh, geometry))
    }

    pub fn world_point(&self, vertex: [f32; 6]) -> [f64; 3] {
        let local = session_rust::Point::new(vertex[0] as f64, vertex[1] as f64, vertex[2] as f64);
        let world = self.model.transform_point(&local);
        [world[0], world[1], world[2]]
    }
}

#[derive(Clone)]
pub struct LineObject {
    pub guid: String,
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
}

impl LineObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [self.model.transform_point(&line.start()), self.model.transform_point(&line.end())]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke { start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32), ..self.prepared.display }
    }
}

#[derive(Clone)]
pub struct PathObject { pub controls_visible: bool, pub id: ObjectId, pub guid: String, pub prepared: crate::chain::PreparedChain, pub model: session_rust::Xform }

impl PathObject {
    pub fn visible(&self) -> bool { self.prepared.source.visible() }

    pub fn segments(&self) -> Vec<crate::chain::Segment> {
        let mut points = Vec::new();
        for p in self.points() {
            let p = std::array::from_fn(|i| p[i] as f32);
            if points.last() != Some(&p) { points.push(p); }
        }
        crate::chain::PreparedChain { points: Rc::new(points), ..self.prepared.clone() }.segments()
    }

    pub fn bounds_points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.bounds_coordinates().into_iter().map(|p|
            self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }

    pub fn points(&self) -> Vec<session_rust::Point> {
        self.prepared.source.coordinates().into_iter().map(|p| self.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]))).collect()
    }


}

pub struct ControlMarker { pub parent: ObjectId, pub control: crate::controls::Control, pub display: crate::marker::Marker }

#[derive(Clone)]
pub struct PointObject { pub id: ObjectId, pub guid: String, pub prepared: crate::marker::PreparedPoint, pub model: session_rust::Xform }

impl PointObject {
    pub fn visible(&self) -> bool { self.prepared.source.is_visible }

    pub fn point(&self) -> session_rust::Point { self.model.transform_point(&self.prepared.source) }
    pub fn marker(&self) -> crate::marker::Marker { crate::marker::Marker { center: self.point().to_f32(), ..self.prepared.display } }
}

#[derive(Clone)]
pub struct Scene {
    pub groups:Rc<std::collections::HashSet<(u32,String)>>,
    pub graphs:Rc<std::collections::HashMap<u32,Rc<session_rust::Graph>>>,
    pub cad_walks:[usize;2],
    pub(crate) instances:std::collections::HashMap<ObjectId,crate::instance_plan::Owner>,
    pub(crate) created:Option<u32>,
    ids:Rc<std::collections::HashSet<ObjectId>>,
    pub view:Rc<crate::view_flags::Flags>,
    pub forest:crate::hierarchy::Forest,
    sheets:Vec<crate::sheet_source::Row>,
    streamed:Vec<crate::range_cloud::Row>,
    clouds:Vec<crate::cloud::Row>,
    texts:Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn pick_id(&self,value:u32)->Option<ObjectId> {
        let id=ObjectId(value);(self.visible(id) && !self.view.locked.contains(&id) && !self.forest.readonly(id)).then_some(id)
    }

    pub fn empty()->Self {
        Self {groups:Default::default(),graphs:Default::default(),cad_walks:[0;2],instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),sheets:Vec::new(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
        let pink = PreparedMesh::triangle([
            [-0.7, -0.6, 0.25], [0.5, -0.6, 0.25], [-0.1, 0.6, 0.25],
        ], [0.9, 0.25, 0.45]).expect("Valid pink triangle");
        let turquoise = PreparedMesh::triangle([
            [-0.4, -0.2, 0.75], [0.8, -0.2, 0.75], [0.2, 0.8, 0.75],
        ], [0.05, 0.7, 0.7]).expect("Valid turquoise triangle");
        let mut scene = Self::empty();
        scene.insert(pink).expect("ID available");
        scene.insert(turquoise).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn visible(&self,id:ObjectId)->bool {self.contains(id)&&!self.view.is_hidden(&id)}

    pub(crate) fn restore_view(&mut self,mut saved:Self) {saved.view=Rc::clone(&self.view);self.restore(saved);Rc::make_mut(&mut self.view).refresh(&self.forest);}

    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for object in self.objects.iter().filter(|row| self.visible(row.id)) {
            for vertex in object.mesh.vertices() {
                let point = object.world_point(*vertex);
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for line in self.lines.iter().filter(|line| self.visible(line.id)) {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        for row in self.paths.iter().filter(|row| self.visible(row.id)) {
            for point in row.bounds_points() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds { Some(bounds) => bounds.include(point), None => bounds = Some(crate::bounds::Bounds::point(point)) }
            }
        }
        for row in self.points.iter().filter(|row| self.visible(row.id)) {
            let p = row.point(); let p = [p[0], p[1], p[2]];
            match &mut bounds { Some(bounds) => bounds.include(p), None => bounds = Some(crate::bounds::Bounds::point(p)) }
        }
        for row in self.texts.iter().filter(|row|self.visible(row.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.sheets.iter().filter(|r|self.visible(r.id)) {if let Some(next)=row.bounds() {if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}}
        for row in self.streamed.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        for row in self.clouds.iter().filter(|r|self.visible(r.id)) {let next=row.bounds();if let Some(bounds)=&mut bounds {bounds.include(next.min);bounds.include(next.max);} else {bounds=Some(next);}}
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
        let mut bounds: Option<crate::bounds::Bounds> = None;
        for id in ids { if let Some(next) = self.selected_bounds(*id) {
            if let Some(b) = &mut bounds { b.include(next.min); b.include(next.max); } else { bounds = Some(next); }
        } } bounds
    }

    pub fn placement(&self, id: ObjectId) -> Option<session_rust::Xform> {
        self.objects.iter().find(|r| r.id==id).map(|r| r.model.clone())
            .or_else(|| self.lines.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.paths.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.points.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.clouds.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.streamed.iter().find(|r|r.id==id).map(|r|r.model.clone()))
            .or_else(|| self.texts.iter().find(|r| r.id==id).map(|r| r.model.clone()))
            .or_else(|| self.sheets.iter().find(|r| r.id==id).map(|r| r.model.clone()))
    }

    pub fn with_descendants(&self,ids:&[ObjectId])->Vec<ObjectId> {self.forest.descendants(ids).into_iter().filter(|id|self.contains(*id)).collect()}

    pub fn place_owner(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_following(id,model)
    }

    pub(crate) fn place_following(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if self.objects.iter().any(|r| r.id==id) { self.place_mesh(id,model) }
        else if self.lines.iter().any(|r| r.id==id) { self.place_line(id,model) }
        else if self.paths.iter().any(|r| r.id==id) { self.place_path(id,model) }
        else if self.clouds.iter().any(|r|r.id==id) {self.place_cloud(id,model)}
        else if self.streamed.iter().any(|r|r.id==id) {self.place_stream(id,model)}
        else if self.sheets.iter().any(|r|r.id==id) {self.place_sheet(id,model)}
        else if self.texts.iter().any(|r|r.id==id) {self.place_text(id,model)}
        else { self.place_point(id,model) }
    }

    pub fn translate(&mut self, id: ObjectId, offset: [f64; 3]) -> Result<(), &'static str> {
        let shift = session_rust::Xform::translation(offset[0], offset[1], offset[2]);
        if let Some(row) = self.objects().iter().find(|r| r.id == id) {
            if row.geometry().is_none() { return Err("Reload editable sources before Move"); }
            let model = &shift * &row.model; return self.place(id, model);
        }
        if let Some(row) = self.lines().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_line(id, model); }
        if let Some(row) = self.paths().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_path(id, model); }
        if let Some(row) = self.points().iter().find(|r| r.id == id) { let model = &shift * &row.model; return self.place_point(id, model); }
        if let Some(row)=self.streamed.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_stream(id,model);}
        if let Some(row)=self.clouds.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_cloud(id,model);}
        if let Some(row)=self.sheets.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_sheet(id,model);}
        if let Some(row)=self.texts.iter().find(|r|r.id==id) {let model=&shift*&row.model;return self.place_text(id,model);}
        Err("Object not found")
    }

    pub fn selected_bounds(&self, id: ObjectId) -> Option<crate::bounds::Bounds> {
        if !self.visible(id) { return None; }
        if let Some(row)=self.clouds.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.streamed.iter().find(|row|row.id==id) {return Some(row.bounds());}
        if let Some(row)=self.texts.iter().find(|row|row.id==id) {return row.bounds();}
        if let Some(row)=self.sheets.iter().find(|row|row.id==id) {return row.bounds();}
        let points=if let Some(row)=self.lines.iter().find(|row|row.id==id){Some(row.endpoints().map(|p|[p[0],p[1],p[2]]).to_vec())}
            else if let Some(row)=self.points.iter().find(|row|row.id==id){let p=row.model.transform_point(&row.prepared.source);Some(vec![[p[0],p[1],p[2]]])}
            else{self.paths.iter().find(|row|row.id==id).map(|row|row.prepared.source.bounds_coordinates().into_iter().map(|p|{let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]}).collect::<Vec<_>>())};
        if let Some(points)=points{let mut bounds=crate::bounds::Bounds::point(*points.first()?);for p in points{bounds.include(p);}return Some(bounds);}
        let object = self.objects.iter().find(|object| object.id == id)?;
        if let Some(session_rust::Geometry::Plane(source))=&object.original{return crate::plane_original::placed(source,&object.model)}
        if let Some(session_rust::Geometry::OBB(source))=&object.original{return Some(crate::bounds_original::placed(source,&object.model))}
        let mut vertices = object.mesh.vertices().iter();
        let first = vertices.next()?;
        let mut bounds = crate::bounds::Bounds::point(object.world_point(*first));
        for vertex in vertices {
            bounds.include(object.world_point(*vertex));
        }
        Some(bounds)
    }

    pub fn line_capacity_bytes(&self) -> usize { self.lines.capacity() * std::mem::size_of::<LineObject>() }

    pub fn cloud_controls(&mut self,parent:Option<ObjectId>) {
        let parent=parent.filter(|id|self.pick_id(id.value()).is_some());
        for row in &mut self.clouds {row.controls_visible=Some(row.id)==parent;}
        for row in &mut self.streamed {row.controls_visible=Some(row.id)==parent;}
    }
    pub fn sheets(&self)->&[crate::sheet_source::Row] {&self.sheets}
    pub fn insert_sheet(&mut self,prepared:Rc<crate::sheet_source::Prepared>)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.original.guid.clone();let used=self.used_guids();while used.contains(&guid) {guid=uuid::Uuid::new_v4().to_string();}let name=prepared.original.name.clone();let url=prepared.streamed.as_ref().map(|stream|stream.source.url.clone());let row=crate::sheet_source::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity(),url};self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);self.sheets.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_sheet(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid sheet placement");}let row=self.sheets.iter_mut().find(|r|r.id==id).ok_or("Sheet is gone")?;let mut candidate=row.clone();candidate.model=model;if candidate.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Sheet placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn sheet_import_url(&mut self,url:&str,previous:&std::collections::HashSet<ObjectId>) {for row in &mut self.sheets {if !previous.contains(&row.id) {row.url=Some(url.into());}}}
    pub fn sheet_url(&mut self,url:&str) {for row in &mut self.sheets {row.url=Some(url.into());}}
    pub fn streams(&self)->&[crate::range_cloud::Row] {&self.streamed}
    pub fn insert_stream(&mut self,prepared:Rc<crate::range_cloud::Prepared>,model:session_rust::Xform,point_px:f32)->Result<ObjectId,&'static str> {
        if !crate::placement::valid(&model.m)||!point_px.is_finite()||point_px<0. {return Err("Invalid streamed source placement or point size");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.streamed.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let row=crate::range_cloud::Row {id,guid:guid.clone(),prepared,model,point_px,controls_visible:false};
        if row.bounds().min.into_iter().chain(row.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}
        let name=row.prepared.source.name.clone();let properties=row.prepared.source.properties;
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,properties.visible,properties.locked);self.streamed.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_stream(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid streamed source placement");}
        let row=self.streamed.iter_mut().find(|row|row.id==id).ok_or("Streamed source is gone")?;let mut candidate=row.clone();candidate.model=model;
        if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!v.is_finite()||!(v as f32).is_finite()) {return Err("Streamed source placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn clouds(&self)->&[crate::cloud::Row] {&self.clouds}
    pub fn insert_cloud(&mut self,prepared:crate::cloud::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid().to_string();while self.clouds.iter().any(|r|r.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}
        let name=prepared.source.name.clone();let visible=prepared.source.is_visible;let locked=prepared.source.is_locked;let row=crate::cloud::Row {point_px:0.,controls_visible:false,id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.clouds.push(row);crate::layer_edit::attach_generated(self,id,guid,name)?;Ok(id)
    }
    pub fn place_cloud(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid cloud placement");}let row=self.clouds.iter_mut().find(|r|r.id==id).ok_or("Cloud object not found")?;let mut candidate=row.clone();candidate.model=model;if candidate.bounds().min.into_iter().chain(candidate.bounds().max).any(|v|!(v as f32).is_finite()) {return Err("Cloud placement exceeds display range");}*row=candidate;Ok(())
    }
    pub fn texts(&self)->&[crate::scene_label::Row] {&self.texts}
    pub(crate) fn texts_mut(&mut self)->&mut [crate::scene_label::Row] {&mut self.texts}
    pub fn insert_text(&mut self,prepared:crate::scene_label::Prepared)->Result<ObjectId,&'static str> {
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);let mut guid=prepared.source.guid.clone();while self.texts.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();let row=crate::scene_label::Row {id,guid:guid.clone(),prepared,model:session_rust::Xform::identity()};row.label(false,true)?;if row.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,row.prepared.source.visible,row.prepared.source.locked);self.texts.push(row);crate::layer_edit::attach_generated(self,id,guid,label)?;Ok(id)
    }
    pub fn place_text(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(),&'static str> {
        if !crate::placement::valid(&model.m) {return Err("Invalid text placement");}let row=self.texts.iter_mut().find(|row|row.id==id).ok_or("Text object not found")?;let mut candidate=row.clone();candidate.model=model;candidate.label(false,true)?;if candidate.bounds().is_some_and(|b|b.min.into_iter().chain(b.max).any(|v|!(v as f32).is_finite())) {return Err("Text bounds exceed the display range");}*row=candidate;Ok(())
    }
    pub fn text_labels(&self,selected:&[ObjectId])->Result<Vec<crate::text::TextLabel>,&'static str> {self.texts.iter().filter(|row|self.visible(row.id)).map(|row|row.label(selected.contains(&row.id),self.pick_id(row.id.value()).is_some())).collect()}
    pub fn points(&self) -> &[PointObject] { &self.points }

    pub fn point_capacity_bytes(&self) -> usize { self.points.capacity() * std::mem::size_of::<PointObject>() }

    pub fn insert_point(&mut self, prepared: crate::marker::PreparedPoint) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.points.iter().any(|row| row.guid == guid) || self.paths.iter().any(|row| row.guid == guid)
            || self.objects.iter().any(|row| row.guid == guid) || self.lines.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let label=prepared.source.name.clone();let node_guid=guid.clone();self.points.push(PointObject { id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_point(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid point placement"); }
        let row = self.points.iter_mut().find(|row| row.id == id).ok_or("Point not found")?;
        let point = model.transform_point(&row.prepared.source);
        if (0..3).any(|i| !(point[i] as f32).is_finite()) { return Err("Point placement exceeds the display range"); }
        row.model = model; Ok(())
    }

    pub fn show_controls(&mut self, visible: bool) {
        for row in &mut self.paths { row.controls_visible = visible && matches!(row.prepared.source, crate::chain::Source::Curve(..)); }
    }

    pub fn control_markers(&self) -> Vec<ControlMarker> {
        let mut markers = Vec::new();
        for row in self.paths.iter().filter(|row| row.controls_visible && self.visible(row.id)) {
            for control in row.prepared.source.controls() {
                let p = control.position().expect("Validated original control");
                let p = row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                let display = crate::marker::Marker { center: p.to_f32(), ..control.marker().expect("Prepared control marker") };
                markers.push(ControlMarker { parent: row.id, control, display });
            }
        }
        markers
    }

    pub fn paths(&self) -> &[PathObject] { &self.paths }

    pub fn path_capacity_bytes(&self) -> usize { self.paths.capacity() * std::mem::size_of::<PathObject>() }

    pub fn insert_path(&mut self, prepared: crate::chain::PreparedChain) -> Result<ObjectId, &'static str> {
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let mut guid = prepared.source.guid().to_owned();
        while self.paths.iter().any(|row| row.guid == guid) || self.objects.iter().any(|row| row.guid == guid)
            || self.lines.iter().any(|row| row.guid == guid) { guid = uuid::Uuid::new_v4().to_string(); }
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.visible(),prepared.source.locked());
        let label=prepared.source.name().to_owned();let node_guid=guid.clone();self.paths.push(PathObject { controls_visible: false, id, guid, prepared, model: session_rust::Xform::identity() });crate::layer_edit::attach_generated(self,id,node_guid,label)?;Ok(id)
    }

    pub fn place_path(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid path placement"); }
        let row = self.paths.iter_mut().find(|row| row.id == id).ok_or("Path not found")?;
        for p in row.prepared.source.coordinates().into_iter().chain(row.prepared.source.bounds_coordinates()) {
            let p = model.transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) { return Err("Path placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn lines(&self) -> &[LineObject] { &self.lines }

    pub fn insert_line(&mut self, prepared: crate::stroke::PreparedLine) -> Result<ObjectId, &'static str> {
        if prepared.source.arrowhead!=session_rust::Arrowhead::NONE{return self.insert_path(crate::chain::PreparedChain::line(prepared.source)?)}
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id); self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.source.is_visible,prepared.source.is_locked);
        let mut guid=prepared.source.guid().to_owned();while self.lines.iter().any(|row|row.guid==guid) {guid=uuid::Uuid::new_v4().to_string();}let label=prepared.source.name.clone();
        self.lines.push(LineObject { guid:guid.clone(), id, prepared, model: session_rust::Xform::identity() });
        crate::layer_edit::attach_generated(self,id,guid,label)?;
        Ok(id)
    }

    pub fn place_line(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) { return Err("Invalid line placement"); }
        let row = self.lines.iter_mut().find(|row| row.id == id).ok_or("Line not found")?;
        for point in [row.prepared.source.start(), row.prepared.source.end()] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) { return Err("Line placement exceeds the display range"); }
        }
        row.model = model; Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub(crate) fn objects_mut(&mut self) -> &mut [Object] { &mut self.objects }

    pub(crate) fn unload(&mut self, origins: &std::collections::HashSet<uuid::Uuid>, epoch: u64) {
        for row in &mut self.objects {
            if let Some(origin) = row.origin().filter(|origin| origins.contains(&origin.id)).cloned() {
                row.editable = crate::edit_source::EditSource::Released { origin, epoch };
            }
        }
    }

    pub(crate) fn hydrate(&mut self, prepared: &crate::rehydrate::PreparedReload) {
        for row in &mut self.objects {
            if !prepared.key.matches(row) { continue; }
            let Some(geometry) = prepared.geometry(&row.metadata.source_guid).cloned() else { continue; };
            let source = crate::document::Source { document: Rc::clone(&prepared.document),
                guid: row.metadata.source_guid.clone(), origin: Rc::clone(&prepared.key.origin) };
            row.editable = crate::edit_source::EditSource::Loaded { geometry, source: Some(source) };
        }
    }

    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) { return Err("Placement must be finite, affine and fit the display range"); }
        let next = self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        let mut guid = prepared.geometry.guid().to_owned();
        while self.objects.iter().any(|object| object.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(id,prepared.geometry.is_visible,prepared.geometry.is_locked);
        let metadata = Rc::new(crate::row_metadata::RowMetadata::from_mesh(&prepared.geometry));
        let generated=prepared.source.is_none();
        let editable = crate::edit_source::EditSource::Loaded { geometry: prepared.geometry, source: prepared.source };
        let label=metadata.name.clone();let node_guid=guid.clone();
        self.objects.push(Object {original:None,cad:None,surface:None, id, guid, metadata, mesh: Rc::new(prepared.display), model: prepared.model, editable });
        if generated {crate::layer_edit::attach_generated(self,id,node_guid,label)?;}
        Ok(id)
    }

    pub fn insert_cad(&mut self,source:Rc<session_rust::BRep>)->Result<ObjectId,&'static str> {
        self.insert_prepared_cad(crate::cad_display::prepare(source)?)
    }

    pub(crate) fn insert_prepared_cad(&mut self,(prepared,mesh):(crate::cad_display::Prepared,crate::prepared::PreparedMesh))->Result<ObjectId,&'static str> {
        let prepared=Rc::new(prepared);let id=self.insert(mesh)?;self.objects.iter_mut().find(|row|row.id==id).unwrap().cad=Some(prepared);Ok(id)
    }

    pub fn insert_surface(&mut self,source:Rc<session_rust::NurbsSurface>)->Result<ObjectId,&'static str> {
        let prepared=Rc::new(crate::surface::Prepared::new(source)?);let mesh=prepared.mesh()?;let id=self.insert(mesh)?;
        self.objects.iter_mut().find(|row|row.id==id).unwrap().surface=Some(prepared);Ok(id)
    }

    fn detach_copy_identity(&mut self, id: ObjectId) {
        let instance = self.instances.contains_key(&id);
        if let Some(row) = self.objects.iter_mut().find(|row| row.id == id) {
            let element = !instance && matches!(row.original, Some(session_rust::Geometry::Element(_)));
            if element {
                if let Some(session_rust::Geometry::Element(source)) = row.original.as_mut() {
                    Rc::make_mut(source).set_guid(row.guid.clone());
                }
            }
            if instance || element { Rc::make_mut(&mut row.metadata).source_guid = row.guid.clone(); }
        }
        let guid = self.row_guid(id).map(str::to_owned);
        if let (Some(guid), Some(owner)) = (guid, self.instances.get_mut(&id)) {
            Rc::make_mut(&mut owner.instance).set_guid(guid);
        }
    }

    pub(crate) fn copy_row(&mut self,id:ObjectId,all:bool)->Result<ObjectId,&'static str> {
        if self.objects.iter().any(|row|row.id==id&&row.geometry().is_none()) {return Err("Reload editable sources before copying");}
        if !self.contains(id) {return Err("Object is gone");}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let copy=ObjectId(self.next_id);let guid=uuid::Uuid::new_v4().to_string();
        if let Some(row)=self.objects.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.objects.push(row);}
        else if let Some(row)=self.lines.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.lines.push(row);}
        else if let Some(row)=self.paths.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.paths.push(row);}
        else if let Some(row)=self.points.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.points.push(row);}
        else if let Some(row)=self.streamed.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.streamed.push(row);}
        else if let Some(row)=self.clouds.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;row.controls_visible=false;self.clouds.push(row);}
        else if let Some(row)=self.sheets.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.sheets.push(row);}
        else if let Some(row)=self.texts.iter().find(|row|row.id==id) {let mut row=row.clone();row.id=copy;row.guid=guid;self.texts.push(row);}
        if let Some(owner)=self.instances.get(&id).cloned() {self.instances.insert(copy,owner);}
        self.detach_copy_identity(copy);
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(copy);
        let view=Rc::make_mut(&mut self.view);if all {view.hide(&[copy],view.is_hidden(&id));view.lock(&[copy],view.locked.contains(&id));}
        for channel in [&mut view.faces,&mut view.edges] {if let Some(rgb)=channel.get(&id).copied() {channel.insert(copy,rgb);}}
        Ok(copy)
    }

    pub(crate) fn row_guid(&self,id:ObjectId)->Option<&str> {
        self.objects.iter().find(|row|row.id==id).map(|row|row.guid.as_str())
            .or_else(||self.paths.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.points.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.lines.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.clouds.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.streamed.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.texts.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
            .or_else(||self.sheets.iter().find(|row|row.id==id).map(|row|row.guid.as_str()))
    }

    pub(crate) fn append_document(&mut self,source:&Scene,name:&str,place:&session_rust::Xform,point_px:f32)->Result<(),&'static str> {
        if !crate::placement::valid(&place.m)||!point_px.is_finite() {return Err("Invalid document placement or point size");}
        let mut candidate=self.clone();let mut ids=source.ids.iter().copied().collect::<Vec<_>>();ids.sort_unstable_by_key(|id|id.value());
        let mut remap=std::collections::HashMap::new();let mut names=std::collections::HashMap::new();let mut used=candidate.ids.iter().filter_map(|id|candidate.row_guid(*id).map(str::to_owned)).collect::<std::collections::HashSet<_>>();
        for old in &ids {
            let id=ObjectId(candidate.reserve_document_key()?);remap.insert(*old,id);Rc::make_mut(&mut candidate.ids).insert(id);
            let mut guid=source.row_guid(*old).ok_or("Original source owner is missing")?.to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}names.insert(*old,guid);
        }
        macro_rules! rows {($field:ident)=>{for row in &source.$field {let mut row=row.clone();let old=row.id;row.id=remap[&old];row.guid=names[&old].clone();candidate.$field.push(row);}};}
        rows!(objects);rows!(lines);rows!(paths);rows!(points);rows!(clouds);rows!(streamed);rows!(texts);rows!(sheets);
        for old in &ids {
            let id=remap[old];if let Some(owner)=source.instances.get(old) {candidate.instances.insert(id,owner.clone());}candidate.place_following(id,place*&source.placement(*old).ok_or("Original placement is missing")?)?;
            let view=Rc::make_mut(&mut candidate.view);view.initial(id,source.visible(*old),source.view.locked.contains(old));
            if let Some(rgb)=source.view.faces.get(old) {view.faces.insert(id,*rgb);}if let Some(rgb)=source.view.edges.get(old) {view.edges.insert(id,*rgb);}
            if source.clouds.iter().any(|row|row.id==*old)&&point_px>0. {candidate.cloud_point_size(id,point_px)?;}
            if source.streamed.iter().any(|row|row.id==*old)&&point_px>0. {candidate.streamed.iter_mut().find(|row|row.id==id).unwrap().point_px=point_px;}
        }
        for doc in &source.forest.documents {
            let mut doc=(**doc).clone();let old_key=doc.key;doc.key=candidate.reserve_document_key()?;
            if !name.is_empty() {doc.label=name.to_owned();}
            let guids=doc.nodes.iter().map(|node|(node.source.guid.clone(),uuid::Uuid::new_v4().to_string())).collect::<std::collections::HashMap<_,_>>();
            for node in &mut doc.nodes {node.object=node.object.map(|id|remap[&id]);node.source.guid=guids[&node.source.guid].clone();node.portable_guid=node.source.guid.clone();node.source.parent_guid=guids.get(&node.source.parent_guid).cloned().unwrap_or_default();}
            for (key,guid) in source.groups.iter().filter(|(key,_)|*key==old_key){let _=key;if let Some(guid)=guids.get(guid){Rc::make_mut(&mut candidate.groups).insert((doc.key,guid.clone()));}}
            let graph_names=ids.iter().filter_map(|old|Some((source.row_guid(*old)?.to_owned(),names[old].clone()))).collect::<std::collections::HashMap<_,_>>();
            if source.graphs.contains_key(&old_key)||doc.header.as_ref().and_then(|h|h.header.graph.as_ref()).is_some(){let graph=crate::graph_edit::transferred(source,source.forest.documents.iter().find(|d|d.key==old_key).unwrap(),&graph_names);Rc::make_mut(&mut candidate.graphs).insert(doc.key,Rc::new(session_rust::Graph::from_proto(graph)));}
            if source.created==Some(old_key)&&candidate.created.is_none() {candidate.created=Some(doc.key);}
            candidate.forest.push(doc);
        }
        self.restore(candidate);Ok(())
    }

    pub fn face_rgba(&self,id:ObjectId)->Option<[f32;4]> {
        if let Some(rgb)=self.view.faces.get(&id) {return Some([rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]);}
        let owner=self.instances.get(&id)?;
        (owner.instance.flags&session_rust::InstanceRef::FLAG_COLOR!=0).then(|| {let c=&owner.instance.color;[c.r as f32,c.g as f32,c.b as f32,c.a as f32]})
    }

    pub(crate) fn used_guids(&self)->std::collections::HashSet<String> {
        self.objects.iter().map(|r|r.guid.clone()).chain(self.lines.iter().map(|r|r.guid.clone())).chain(self.paths.iter().map(|r|r.guid.clone())).chain(self.points.iter().map(|r|r.guid.clone())).chain(self.clouds.iter().map(|r|r.guid.clone())).chain(self.streamed.iter().map(|r|r.guid.clone())).chain(self.texts.iter().map(|r|r.guid.clone())).chain(self.sheets.iter().map(|r|r.guid.clone())).collect()
    }

    // Prepare a single placed row before appending it; repeated instances share geometry without cloning the growing destination scene.
    pub(crate) fn append_instance(&mut self,template:&Scene,owner:crate::instance_plan::Owner,model:session_rust::Xform,guid:String,point_px:f32)->Result<ObjectId,&'static str> {
        let mut placed=template.clone();let original=placed.ids.iter().copied().next().ok_or("Missing definition display owner")?;
        if placed.ids.len()!=1 {return Err("One definition must create one drawable owner");}
        placed.place_following(original,model)?;if !placed.clouds.is_empty()&&point_px>0. {placed.cloud_point_size(original,point_px)?;}
        let next=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;let id=ObjectId(self.next_id);
        let visible=owner.instance.flags&session_rust::InstanceRef::FLAG_HIDDEN==0;let locked=owner.instance.flags&session_rust::InstanceRef::FLAG_LOCKED!=0;
        if let Some(row)=placed.objects.first_mut() {row.metadata=Rc::new(crate::row_metadata::RowMetadata {source_guid:owner.instance.guid().into(),name:owner.instance.name.clone(),is_visible:visible,is_locked:locked});}
        macro_rules! append {($field:ident)=>{if let Some(mut row)=placed.$field.pop() {row.id=id;row.guid=guid.clone();self.$field.push(row);}};}
        append!(objects);append!(lines);append!(paths);append!(points);append!(clouds);
        self.next_id=next;Rc::make_mut(&mut self.ids).insert(id);Rc::make_mut(&mut self.view).initial(id,visible,locked);self.instances.insert(id,owner);Ok(id)
    }

    pub(crate) fn reserve_document_key(&mut self)->Result<u32,&'static str> {
        let key=self.next_id;self.next_id=self.next_id.checked_add(1).ok_or("Object IDs exhausted")?;Ok(key)
    }

    pub(crate) fn cloud_point_size(&mut self,id:ObjectId,px:f32)->Result<(),&'static str> {
        if !px.is_finite() {return Err("Non-finite cloud point size");}
        self.clouds.iter_mut().find(|row|row.id==id).ok_or("Cloud object not found")?.point_px=px;Ok(())
    }

    pub fn clear(&mut self) {
        self.groups=Default::default();self.graphs=Default::default();
        self.cad_walks=[0;2];self.instances.clear();
        self.created=None;
        self.ids=Default::default();
        self.view=Default::default();
        self.forest=Default::default();
        self.clouds=Vec::new();self.streamed=Vec::new();self.sheets=Vec::new();
        self.texts=Vec::new();
        self.points = Vec::new();
        self.paths = Vec::new();
        self.lines = Vec::new();
        self.objects = Vec::new();
        self.extra = None;
    }

    pub fn replace(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        self.clear();
        self.import(loaded)
    }

    pub fn import(&mut self, loaded: crate::document::Loaded) -> Result<(), &'static str> {
        let mut objects=Vec::new();let key=self.next_id;
        for prepared in loaded.meshes {
            let guid=prepared.geometry.guid().to_owned();let name=prepared.geometry.name.clone();
            let id=self.insert(prepared)?;objects.push((guid,id,name));
        }
        self.forest.add(key,loaded.document.name.clone(),loaded.tree,&objects);if let Some(origin)=objects.first().and_then(|(_,id,_)|self.objects.iter().find(|row|row.id==*id)).and_then(|row|row.origin()).cloned() {Rc::make_mut(self.forest.documents.last_mut().unwrap()).header=Some(origin);}let colours=self.forest.colours(key);Rc::make_mut(&mut self.view).faces.extend(colours);Ok(())
    }

    pub fn place(&mut self, id: ObjectId, model: session_rust::Xform) -> Result<(), &'static str> {
        if self.forest.readonly(id) {return Err("Object attributes follow their parent and are read only");}
        self.place_mesh(id,model)
    }

    fn place_mesh(&mut self,id:ObjectId,model:session_rust::Xform)->Result<(), &'static str> {
        if !crate::placement::valid(&model.m)
        {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let object = self.objects.iter_mut().find(|o| o.id == id).ok_or("Object not found")?;
        if object.geometry().is_none() { return Err("Reload editable sources before Move"); }
        object.model = model;
        Ok(())
    }

    pub fn remove(&mut self, id: ObjectId) -> bool {
        crate::graph_edit::remove(self,id);
        self.instances.remove(&id);
        let existed = Rc::make_mut(&mut self.ids).remove(&id);
        self.objects.retain(|row| row.id != id); self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id); self.points.retain(|row| row.id != id);self.texts.retain(|row|row.id!=id);self.clouds.retain(|row|row.id!=id);self.streamed.retain(|row|row.id!=id);self.sheets.retain(|row|row.id!=id);
        if self.extra == Some(id) { self.extra = None; }
        existed
    }

    pub(crate) fn all_ids(&self)->std::collections::HashSet<ObjectId> {(*self.ids).clone()}
    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self,current:Option<ObjectId>)->Option<ObjectId> {
        let mut ids=self.objects.iter().map(|r|r.id).chain(self.lines.iter().map(|r|r.id)).chain(self.paths.iter().map(|r|r.id)).chain(self.points.iter().map(|r|r.id)).chain(self.texts.iter().map(|r|r.id)).chain(self.clouds.iter().map(|r|r.id)).chain(self.streamed.iter().map(|r|r.id)).chain(self.sheets.iter().map(|r|r.id)).filter(|id|self.pick_id(id.value()).is_some()).collect::<Vec<_>>();
        ids.sort_unstable_by_key(|id|id.value());if ids.is_empty() {return None;}
        let start=ids.iter().position(|id|Some(*id)==current).map_or(0,|i|i+1);Some(ids[start%ids.len()])
    }

    pub fn add_box(&mut self) -> Result<ObjectId, &'static str> {
        let mut source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        source.set_objectcolor(session_rust::Color::new(0.65, 0.65, 0.65, 1.0));
        let id = self.insert(PreparedMesh::new(source)?)?;
        self.place(id, session_rust::Xform::translation(0.9, 0.0, 0.4))?;
        Ok(id)
    }

    pub fn toggle_extra(&mut self) {
        if let Some(id) = self.extra.take() {
            self.remove(id);
        } else {
            let extra = PreparedMesh::triangle([
                [-0.9, 0.3, 0.5], [-0.4, 0.3, 0.5], [-0.65, 0.9, 0.5],
            ], [0.2, 0.8, 0.3]).expect("Valid extra triangle");
            self.extra = Some(self.insert(extra).expect("ID available"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unloading_policy_protects_generated_unlocated_and_modified_sources() {
        let mut scene = Scene::demo(); assert!(!scene.objects[0].can_unload());
        let bytes = crate::specimen::bytes();
        scene.replace(crate::document::load(&bytes).unwrap()).unwrap(); assert!(!scene.objects[0].can_unload());
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:policy".into()));
        scene.replace(crate::document::load_at(&bytes, Some(url)).unwrap()).unwrap();
        assert!(scene.objects.iter().all(Object::can_unload));
        let row = &mut scene.objects[0];
        Rc::make_mut(&mut row.metadata).is_locked = true; assert!(!row.can_unload());
        Rc::make_mut(&mut row.metadata).is_locked = row.geometry().unwrap().is_locked;
        assert!(row.can_unload());
        let source = row.source().cloned(); let geometry = Rc::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0));
        row.editable = crate::edit_source::EditSource::Loaded { geometry, source }; assert!(!row.can_unload());
    }

    #[test]
    fn partial_replacement_failure_restores_owners_without_reusing_ids() {
        let mut scene = Scene::demo();
        let before = scene.objects.clone();
        scene.next_id = u32::MAX - 1;
        let loaded = crate::document::load(&crate::specimen::bytes()).unwrap();
        let mut history = crate::history::History::default();
        assert!(history.try_edit(&mut scene, |scene| scene.replace(loaded)).is_err());
        assert_eq!(scene.next_id, u32::MAX);
        assert_eq!(scene.objects.len(), before.len());
        for (object, old) in scene.objects.iter().zip(&before) {
            assert_eq!(object.id, old.id);
            assert!(Rc::ptr_eq(object.geometry().unwrap(), old.geometry().unwrap()));
            assert!(Rc::ptr_eq(&object.mesh, &old.mesh));
        }
        assert!(!history.undo(&mut scene));
    }

    #[test]
    fn removal_moves_a_row_without_changing_its_identity() {
        let mut scene = Scene::demo();
        let first = scene.objects()[0].id;
        let second = scene.objects()[1].id;
        assert!(scene.remove(first));
        assert_eq!(scene.objects()[0].id, second);
        assert_eq!(scene.next(None), Some(second));
        assert!(!scene.contains(first));
        scene.toggle_extra();
        assert_ne!(scene.objects()[1].id, first);
        assert_ne!(scene.objects()[1].id, second);
    }

    #[test]
    fn toggling_the_extra_object_uses_its_id_after_other_removals() {
        let mut scene = Scene::demo();
        scene.toggle_extra();
        let extra = scene.objects()[2].id;
        scene.remove(scene.objects()[0].id);
        scene.toggle_extra();
        assert!(!scene.contains(extra));
        assert_eq!(scene.objects().len(), 1);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "pick_gpu_tests.rs"]
mod pick_gpu_tests;

impl Scene {
    pub fn replace_original(&mut self,id:ObjectId,mut geometry:session_rust::Geometry)->Result<(),&'static str>{
        use session_rust::Geometry;
        if self.pick_id(id.value())!=Some(id){return Err("Edit a visible unlocked original")}
        crate::source_edit::original(self,id).ok_or("Reload an editable original source")?;
        let model=self.placement(id).ok_or("Source placement is missing")?;geometry.set_guid(self.row_guid(id).ok_or("Original source identity is missing")?);
        match geometry{
            Geometry::Plane(source)=>{let prepared=crate::plane_original::prepared(&source)?;let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected a plane source")?;row.mesh=Rc::new(prepared.display);row.original=Some(Geometry::Plane(source));row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};},
            Geometry::OBB(source)=>{let prepared=crate::bounds_original::prepared(&source)?;let bounds=crate::bounds_original::placed(&source,&model);if bounds.min.into_iter().chain(bounds.max).any(|v|!(v as f32).is_finite()){return Err("Edited box exceeds display range")}let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected a box source")?;row.mesh=Rc::new(prepared.display);row.original=Some(Geometry::OBB(source));row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};},
            Geometry::Point(source)=>{let prepared=crate::marker::PreparedPoint::new(source)?;let p=model.transform_point(&prepared.source);if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited point exceeds display range")}if let Some(row)=self.points.iter_mut().find(|r|r.id==id){row.prepared=prepared;}else if let Some(row)=self.clouds.iter().find(|r|r.id==id){let guid=row.guid.clone();self.clouds.retain(|r|r.id!=id);self.points.push(PointObject{id,guid,prepared,model});}else{return Err("Expected a point source")}},
            Geometry::Line(source)if self.lines.iter().any(|r|r.id==id)=>{
                let prepared=crate::chain::PreparedChain::line(source.clone())?;for p in prepared.source.bounds_coordinates(){let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited line exceeds display range")}}
                if source.arrowhead==session_rust::Arrowhead::NONE{self.lines.iter_mut().find(|r|r.id==id).unwrap().prepared=crate::stroke::PreparedLine::new(source)?;}else{let old=self.lines.iter().find(|r|r.id==id).unwrap();let guid=old.guid.clone();self.lines.retain(|r|r.id!=id);self.paths.push(PathObject{controls_visible:false,id,guid,prepared,model});}
            },
            Geometry::Line(_)|Geometry::Polyline(_)|Geometry::NurbsCurve(_)=>{
                let prepared=match geometry{Geometry::Line(source)=>crate::chain::PreparedChain::line(source)?,Geometry::Polyline(source)=>crate::chain::PreparedChain::polyline(source)?,Geometry::NurbsCurve(source)=>crate::chain::PreparedChain::curve(source)?,_=>unreachable!()};
                for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()){let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited path exceeds display range")}}self.paths.iter_mut().find(|r|r.id==id).ok_or("Expected a path source")?.prepared=prepared;
            },
            Geometry::Mesh(_)|Geometry::BRep(_)|Geometry::NurbsSurface(_)=>{
                let(cad,surface,prepared)=match geometry{Geometry::Mesh(source)=>(None,None,crate::prepared::PreparedMesh::new((*source).clone())?),Geometry::BRep(source)=>{let(cad,mesh)=crate::cad_display::prepare(source)?;(Some(Rc::new(cad)),None,mesh)},Geometry::NurbsSurface(source)=>{let s=Rc::new(crate::surface::Prepared::new(source)?);let mesh=s.mesh()?;(None,Some(s),mesh)},_=>unreachable!()};
                for v in prepared.display.vertices(){let p=model.transform_point(&session_rust::Point::new(v[0]as f64,v[1]as f64,v[2]as f64));if(0..3).any(|i|!(p[i]as f32).is_finite()){return Err("Edited object exceeds display range")}}let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected an object source")?;row.mesh=Rc::new(prepared.display);row.cad=cad;row.surface=surface;row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};
            },
            _=>return Err("This original source cannot be replaced"),
        }Ok(())
    }
    pub fn deform_part(&mut self,target:crate::pick_sources::Target,delta:&session_rust::Xform) -> Result<(), &'static str> {
        use crate::pick_sources::Part;
        if !target.valid(self) || !crate::placement::valid(&delta.m) {return Err("Invalid part edit");}
        if matches!(target.part,Part::CloudPoint(_)) {return Err("Cloud point editing connects with source transforms");}
        if self.objects.iter().any(|row|row.id==target.owner&&row.original.is_some()){return Err("Part editing for this original family connects in the complete command inventory")}
        let model=self.placement(target.owner).ok_or("Part owner is missing")?;
        let local=&(&model.inverse().ok_or("Part placement cannot be inverted")?*delta)*&model;
        if self.objects.iter().any(|row|row.id==target.owner&&(row.surface.is_some()||row.cad.is_some())) {
            let original=crate::source_part::geometry(self,target.owner)?;let edited=crate::source_deform::transform(&original,crate::source_part::target(target.part)?,&local).map_err(|_|"CAD edit would invalidate the original source or adjacent trims")?;
            let (cad,surface,prepared)=match edited {
                session_rust::Geometry::BRep(source)=>{let(cad,prepared)=crate::cad_display::prepare(source)?;(Some(Rc::new(cad)),None,prepared)},
                session_rust::Geometry::NurbsSurface(source)=>{let surface=Rc::new(crate::surface::Prepared::new(source)?);let prepared=surface.mesh()?;(None,Some(surface),prepared)},_=>return Err("Unexpected edited CAD source"),
            };
            for vertex in prepared.display.vertices(){let point=model.transform_point(&session_rust::Point::new(vertex[0]as f64,vertex[1]as f64,vertex[2]as f64));if(0..3).any(|i|!(point[i]as f32).is_finite()){return Err("Part edit exceeds the display range")}}
            let row=self.objects.iter_mut().find(|row|row.id==target.owner).unwrap();row.mesh=Rc::new(prepared.display);row.cad=cad;row.surface=surface;row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};return Ok(());
        }
        if let Some(row)=self.objects.iter_mut().find(|r|r.id==target.owner) {
            let mut source=(**row.geometry().ok_or("Reload editable sources before editing a part")?).clone();
            for key in crate::part_preview::keys(&source,target.part)? {
                let point=local.transform_point(&source.vertex_point(key).ok_or("Missing source vertex")?);
                source.vertex.get_mut(&key).ok_or("Missing source vertex")?.set_position(point);
            }
            source.clear_triangle_bvh();
            let prepared=crate::prepared::PreparedMesh::new(source)?;
            for vertex in prepared.display.vertices() {let point=model.transform_point(&session_rust::Point::new(vertex[0] as f64,vertex[1] as f64,vertex[2] as f64));
                if (0..3).any(|i| !(point[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}}
            row.mesh=Rc::new(prepared.display); row.editable=crate::edit_source::EditSource::Loaded {geometry:prepared.geometry,source:None}; return Ok(());
        }
        let row=self.paths.iter_mut().find(|r|r.id==target.owner).ok_or("Part owner is missing")?;
        let (crate::chain::Source::Curve(curve,_),Part::Control(index))=(&row.prepared.source,target.part) else {return Err("Select a curve control point");};
        let mut source=(**curve).clone(); let point=local.transform_point(&source.get_cv(index).ok_or("Missing curve control")?);
        let weight=source.get_cv_4d(index).ok_or("Missing curve control")?.3;
        let set=if source.m_is_rat {source.set_cv_4d(index,point[0]*weight,point[1]*weight,point[2]*weight,weight)} else {source.set_cv_point(index,&point)};
        if !set {return Err("Cannot set curve control");}
        let prepared=crate::chain::PreparedChain::curve(Rc::new(source))?;
        for p in prepared.source.coordinates().into_iter().chain(prepared.source.bounds_coordinates()) {
            let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));
            if (0..3).any(|i| !(p[i] as f32).is_finite()) {return Err("Part edit exceeds the display range");}
        }
        row.prepared=prepared;Ok(())
    }
}

#[cfg(test)]
#[path="cap_pick_tests.rs"]
mod cap_pick_tests;
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96w-detach-copied-original-source-identity-01.rs"
```

```text
--8<-- "journey/code/96w-detach-copied-original-source-identity-02.rs"
```

</details>

</details>

<a id="checkpoint-96x-regenerate-display-for-copied-feature-owners"></a>

### Step 6 of 8 · Regenerate display for copied feature owners (`96x-regenerate-display-for-copied-feature-owners`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 6/8 Visual Trace (96x-regenerate-display-for-copied-feature-owners): Regenerate display for copied feature owners</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/layer_edit.rs, src/copy_feature_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">fresh copy identity</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original placement</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">regenerated display</text>
    
</svg>
</div>

- **Goal:** Regenerate display for copied feature owners
- **Data Flow Trace:** `original source → fresh copy identity → original placement → regenerated display`
- **Files You Touch:** `src/layer_edit.rs` (*Select direct original attributes and reuse existing projection owners.*) · `src/copy_feature_tests.rs` (*Supply independent original-source, placement and archive witnesses.*) · `src/lib.rs` (*Register the supplied copy fixtures.*)

A copied Element keeps its original feature data.

Its derived paths are display objects, so copying those paths would create stale ownership and archive duplicates. Keep only direct original Geometry children in the source attributes group, matching the current viewer. Remove those children’s old tree descendants; their own original feature data remains intact. InstanceRef and Sheet attributes are outside that Geometry lookup.

Copy and place the original rows using the existing tree copier, then attach the new tree. Collect its original owners before appending any generated rows. Reuse feature_display for Elements and instance_features for instances, once per owner. A fresh Element GUID has no existing relationship edges in the source document; the planner therefore labels unlinked interaction features as orphans.

No second projection algorithm is introduced. Presentation defaults and current contact rendering are separate bridges; this stage checks original identities, placement and regeneration.

> **Self-Check Question:** *Why collect copied owners before appending their generated display?*  
> **Answer:** Appending changes the tree. A fixed list of original owners prevents newly generated paths from entering the traversal or being projected again.

- **Hands-On Lab Experiment:** Copy the host in the imported interaction specimen, then Save and reopen three times. Its direct original child comes with it, each owner projects once, and no derived path is serialized. Compare GUID-to-world-matrix maps to detect a transform applied twice.

<details>
<summary>Implementation 96x-regenerate-display-for-copied-feature-owners: Regenerate display for copied feature owners</summary>

Regenerate display for copied feature owners

Data flow: original source → fresh copy identity → original placement → regenerated display.

### 1. `src/layer_edit.rs`

Select direct original attributes and reuse existing projection owners. Exclude generated rows and nested attribute tree descendants before cloning. Regenerate each new original owner after hierarchy and placement adoption.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
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
```

</details>

Replace it with:

```rust
    }
}

fn copy_attributes(scene: &Scene, tree: &mut Branch) {
    tree.children.retain(|group| group.node.object.is_none() && group.node.source.name == "attributes");
    for group in &mut tree.children {
        group.children.retain(|child| child.node.object.is_some_and(|id|
            !child.node.derived && !scene.instances.contains_key(&id)
            && !scene.sheets().iter().any(|row| row.id == id)
            && !scene.texts().iter().any(|row| row.id == id)));
        for child in &mut group.children { child.children.clear(); }
    }
}

fn project_copies(scene: &mut Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
    let detached = session_rust::Session::new("copies");
    let owners = scene.forest.descendants(ids);
    for id in owners {
        if let Some(owner) = scene.instances.get(&id).cloned() {
            crate::instance_features::append(scene, id, &owner)?;
        } else {
            let original = scene.objects().iter().find(|row| row.id == id)
                .and_then(|row| row.original.clone());
            if let Some(session_rust::Geometry::Element(element)) = original {
                crate::feature_display::append(scene, id, &detached, &element)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn copy_objects(scene:&mut Scene,ids:&[crate::scene::ObjectId],delta:&session_rust::Xform)->Result<Vec<crate::scene::ObjectId>, &'static str> {
    if ids.is_empty(){return Err("Select objects before Copy");}
    if !crate::placement::valid(&delta.m){return Err("Copy transform is invalid");}
    let mut chosen=ids.to_vec();chosen.sort_unstable_by_key(|id|id.value());chosen.dedup();
    if chosen.iter().any(|id|scene.pick_id(id.value())!=Some(*id)){return Err("Copy needs visible unlocked editable objects");}
    let mut trees=scene.forest.documents.iter().map(|doc|branch(doc,0)).collect::<Vec<_>>();let mut sources=Vec::new();
    for (owner,doc) in scene.forest.documents.iter().enumerate(){for(index,node)in doc.nodes.iter().enumerate(){let Some(id)=node.object.filter(|id|chosen.contains(id))else{continue};let parent=node.parent.ok_or("Object has no parent")?;let mut tree=branch(doc,index);copy_attributes(scene, &mut tree);sources.push((owner,doc.nodes[parent].source.guid.clone(),tree,scene.placement(id).ok_or("Copy placement is missing")?));}}
    if sources.len()!=chosen.len(){return Err("Copy source tree is missing");}
    for(owner,doc)in scene.forest.documents.iter().enumerate(){let extra=sources.iter().filter(|source|source.0==owner).map(|source|{let mut nodes=Vec::new();flatten(source.2.clone(),None,0,&mut nodes);nodes.len()}).sum::<usize>();if doc.nodes.len()+extra>crate::hierarchy::MAX_NODES{return Err("Copy exceeds the tree limit");}}
    let mut copies=Vec::new();
    fn place_copy(scene:&mut Scene,tree:&Branch,delta:&session_rust::Xform)->Result<(),&'static str>{if let Some(id)=tree.node.object{let model=scene.placement(id).ok_or("Copy placement is missing")?;scene.place_following(id,delta*&model)?;}for child in &tree.children{place_copy(scene,child,delta)?;}Ok(())}
    for(owner,parent,tree,_model)in sources{let copy=copy_branch(scene,tree,false)?;let id=copy.node.object.ok_or("Copy owner is missing")?;place_copy(scene,&copy,delta)?;copies.push(id);find(&mut trees[owner],&parent).ok_or("Copy parent is missing")?.children.push(copy);}
    for(owner,tree)in trees.into_iter().enumerate(){replace(scene,owner,tree);}project_copies(scene, &copies)?;Ok(copies)
}

pub fn group(scene:&mut Scene,ids:&[crate::scene::ObjectId],name:&str)->Result<String,&'static str>{
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

fn copy_attributes(scene: &Scene, tree: &mut Branch) {
    tree.children.retain(|group| group.node.object.is_none() && group.node.source.name == "attributes");
    for group in &mut tree.children {
        group.children.retain(|child| child.node.object.is_some_and(|id|
            !child.node.derived && !scene.instances.contains_key(&id)
            && !scene.sheets().iter().any(|row| row.id == id)
            && !scene.texts().iter().any(|row| row.id == id)));
        for child in &mut group.children { child.children.clear(); }
    }
}

fn project_copies(scene: &mut Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
    let detached = session_rust::Session::new("copies");
    let owners = scene.forest.descendants(ids);
    for id in owners {
        if let Some(owner) = scene.instances.get(&id).cloned() {
            crate::instance_features::append(scene, id, &owner)?;
        } else {
            let original = scene.objects().iter().find(|row| row.id == id)
                .and_then(|row| row.original.clone());
            if let Some(session_rust::Geometry::Element(element)) = original {
                crate::feature_display::append(scene, id, &detached, &element)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn copy_objects(scene:&mut Scene,ids:&[crate::scene::ObjectId],delta:&session_rust::Xform)->Result<Vec<crate::scene::ObjectId>, &'static str> {
    if ids.is_empty(){return Err("Select objects before Copy");}
    if !crate::placement::valid(&delta.m){return Err("Copy transform is invalid");}
    let mut chosen=ids.to_vec();chosen.sort_unstable_by_key(|id|id.value());chosen.dedup();
    if chosen.iter().any(|id|scene.pick_id(id.value())!=Some(*id)){return Err("Copy needs visible unlocked editable objects");}
    let mut trees=scene.forest.documents.iter().map(|doc|branch(doc,0)).collect::<Vec<_>>();let mut sources=Vec::new();
    for (owner,doc) in scene.forest.documents.iter().enumerate(){for(index,node)in doc.nodes.iter().enumerate(){let Some(id)=node.object.filter(|id|chosen.contains(id))else{continue};let parent=node.parent.ok_or("Object has no parent")?;let mut tree=branch(doc,index);copy_attributes(scene, &mut tree);sources.push((owner,doc.nodes[parent].source.guid.clone(),tree,scene.placement(id).ok_or("Copy placement is missing")?));}}
    if sources.len()!=chosen.len(){return Err("Copy source tree is missing");}
    for(owner,doc)in scene.forest.documents.iter().enumerate(){let extra=sources.iter().filter(|source|source.0==owner).map(|source|{let mut nodes=Vec::new();flatten(source.2.clone(),None,0,&mut nodes);nodes.len()}).sum::<usize>();if doc.nodes.len()+extra>crate::hierarchy::MAX_NODES{return Err("Copy exceeds the tree limit");}}
    let mut copies=Vec::new();
    fn place_copy(scene:&mut Scene,tree:&Branch,delta:&session_rust::Xform)->Result<(),&'static str>{if let Some(id)=tree.node.object{let model=scene.placement(id).ok_or("Copy placement is missing")?;scene.place_following(id,delta*&model)?;}for child in &tree.children{place_copy(scene,child,delta)?;}Ok(())}
    for(owner,parent,tree,_model)in sources{let copy=copy_branch(scene,tree,false)?;let id=copy.node.object.ok_or("Copy owner is missing")?;place_copy(scene,&copy,delta)?;copies.push(id);find(&mut trees[owner],&parent).ok_or("Copy parent is missing")?.children.push(copy);}
    for(owner,tree)in trees.into_iter().enumerate(){replace(scene,owner,tree);}project_copies(scene, &copies)?;Ok(copies)
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

pub enum FeatureTree {
    Group(String, Vec<FeatureTree>),
    Piece(String, crate::scene::ObjectId),
}

pub fn attach_features(scene: &mut Scene, id: crate::scene::ObjectId, groups: Vec<FeatureTree>) -> Result<(), &'static str> {
    fn build(scene: &Scene, tree: FeatureTree) -> Result<Branch, &'static str> {
        let (label, object, children) = match tree {
            FeatureTree::Group(label, children) => (label, None, children),
            FeatureTree::Piece(label, id) => (label, Some(id), Vec::new()),
        };
        let name = match object {
            Some(id) => scene.row_guid(id).ok_or("Feature piece is missing")?.to_owned(),
            None => label.clone(),
        };
        let source = session_rust::proto::TreeNode {
            guid: uuid::Uuid::new_v4().to_string(), name, ..Default::default()
        };
        let node = Node { source: source.clone(), portable_guid: source.guid,
            object, label, parent: None, depth: 0, end: 1, readonly: true,
            derived: true, placement: Default::default() };
        let children = children.into_iter().map(|child| build(scene, child)).collect::<Result<_, _>>()?;
        Ok(Branch { node, children })
    }
    let (owner, index) = scene.forest.documents.iter().enumerate().find_map(|(owner, doc)|
        doc.nodes.iter().position(|node| node.object == Some(id)).map(|index| (owner, index)))
        .ok_or("Element tree is missing")?;
    let doc = &scene.forest.documents[owner];
    let guid = doc.nodes[index].source.guid.clone();
    let mut tree = branch(doc, 0);
    let parent = find(&mut tree, &guid).unwrap();
    for group in groups {
        let group = build(scene, group)?;
        if let Some(existing) = parent.children.iter_mut().find(|child|
            child.node.object.is_none() && child.node.source.name == group.node.source.name) {
            existing.children.extend(group.children);
        } else { parent.children.push(group); }
    }
    let mut nodes = Vec::new();
    flatten(tree.clone(), None, 0, &mut nodes);
    if nodes.len() > crate::hierarchy::MAX_NODES
        || nodes.iter().any(|node| node.depth > crate::hierarchy::MAX_DEPTH) {
        return Err("Element features exceed the tree limit");
    }
    replace(scene, owner, tree);
    Ok(())
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96x-regenerate-display-for-copied-feature-owners-01.rs"
```

```text
--8<-- "journey/code/96x-regenerate-display-for-copied-feature-owners-02.rs"
```

```text
--8<-- "journey/code/96x-regenerate-display-for-copied-feature-owners-03.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/copy_feature_tests.rs`

Supply independent original-source, placement and archive witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96x-regenerate-display-for-copied-feature-owners-04.rs"
```

</details>

### 3. `src/lib.rs`

Register the supplied copy fixtures.

<details>
<summary>Locate the existing block</summary>

```rust
mod instance_feature_tests;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod copy_feature_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96x-regenerate-display-for-copied-feature-owners-05.rs"
```

</details>

</details>

<a id="checkpoint-96y-address-feature-pixels-to-original-cad-owners"></a>

### Step 7 of 8 · Address feature pixels to original CAD owners (`96y-address-feature-pixels-to-original-cad-owners`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 7/8 Visual Trace (96y-address-feature-pixels-to-original-cad-owners): Address feature pixels to original CAD owners</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/feature_owner.rs, src/pick_sources.rs, src/feature_pick_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original source</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">fresh copy identity</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original placement</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">regenerated display</text>
    
</svg>
</div>

- **Goal:** Address feature pixels to original CAD owners
- **Data Flow Trace:** `original source → fresh copy identity → original placement → regenerated display`
- **Files You Touch:** `src/feature_owner.rs` (*Reuse the existing original-owner traversal for a displayed GPU primitive.*) · `src/pick_sources.rs` (*Write an original editing ID and suppress invented subpart tags for derived ink.*) · `src/feature_pick_tests.rs` (*Supply independent original-owner address and guard witnesses.*) · `src/lib.rs` (*Register the supplied original-owner address fixture.*) · `src/pick_gpu_tests.rs` (*Supply a real integer GPU feature-pixel readback witness.*)

The integer picking pass must write original editing identities.

A derived feature path is visible geometry, but it is not an editable Session object. Resolve its nearest original Element or InstanceRef with the ownership traversal already taught for tree rows. Original paths keep their existing IDs.

Use this resolver when building the GPU ink-ID arrays. A generated feature gets its owner ID and a zero subpart tag: its tessellated segments are display data, not original PathEdge indices. Object mode therefore selects the original CAD owner; edge and control modes cannot invent a source subpart. Hidden and locked original owners still fail the existing eligibility guard.

This change reuses the integer target, readback and stale-result handling already developed. It adds no second picking pass. The supplied GPU witness clicks a feature line away from its Element mesh and verifies the returned owner ID, then locks the original and repeats the click.

The dependency command also supplies assets/pb/course-feature-pick.pb: two original Elements with feature lines away from their meshes, with the second Element locked. Open it in the browser and use object selection to check the original-owner address directly.

> **Self-Check Question:** *Why is the subpart tag zero for a generated feature outline?*  
> **Answer:** A tessellated display segment has no editable original PathEdge index. Zero identifies only the original object; it prevents the edge-mode resolver from treating generated geometry as a kernel source part.

- **Hands-On Lab Experiment:** Click a visible interaction outline in object mode and Delete, then Undo. The original Element and its features disappear and return together. Try edge mode: it must not expose a generated outline segment as an original editable edge. Lock the owner and repeat.

<details>
<summary>Implementation 96y-address-feature-pixels-to-original-cad-owners: Address feature pixels to original CAD owners</summary>

Address feature pixels to original CAD owners

Data flow: original source → fresh copy identity → original placement → regenerated display.

### 1. `src/feature_owner.rs`

Reuse the existing original-owner traversal for a displayed GPU primitive.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) { return scene.pick_id(id.value()); }
    for doc in &scene.forest.documents {
        if let Some(index) = doc.nodes.iter().position(|node| node.object == Some(id)) {
            return ancestor(scene, doc, index);
        }
    }
    None
}

pub fn row_owner(scene: &Scene, key: &str) -> Option<ObjectId> {
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-01.rs"
```

</details>

### 2. `src/pick_sources.rs`

Write an original editing ID and suppress invented subpart tags for derived ink.

<details>
<summary>Locate the existing block</summary>

```rust
        let id = |owner: ObjectId, tag| [scene.pick_id(owner.value()).map_or(0, ObjectId::value), tag];
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let id = |owner: ObjectId, tag| [crate::feature_owner::display_owner(scene, owner).map_or(0, ObjectId::value),
            if scene.forest.derived(owner) { 0 } else { tag }];
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/feature_pick_tests.rs`

Supply independent original-owner address and guard witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-03.rs"
```

</details>

### 4. `src/lib.rs`

Register the supplied original-owner address fixture.

<details>
<summary>Locate the existing block</summary>

```rust
mod copy_feature_tests;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod feature_pick_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/pick_gpu_tests.rs`

Supply a real integer GPU feature-pixel readback witness.

<details>
<summary>Locate the existing block</summary>

```text
#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96y-address-feature-pixels-to-original-cad-owners-05.rs"
```

</details>

</details>

<a id="checkpoint-96z-preserve-source-feature-visibility-and-pixel-styles"></a>

### Step 8 of 8 · Preserve source feature visibility and pixel styles (`96z-preserve-source-feature-visibility-and-pixel-styles`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 8/8 Visual Trace (96z-preserve-source-feature-visibility-and-pixel-styles): Preserve source feature visibility and pixel styles</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/feature_display.rs, src/feature_style_tests.rs, src/lib.rs, src/pick_gpu_tests.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">source feature visibility</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">coincident-point</text>
    <text x="316" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">classification</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">existing marker/path</text>
    <text x="521" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">display</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">original owner</text>
    
</svg>
</div>

- **Goal:** Project only visible original features, with two-pixel lines and twelve-pixel point discs.
- **Data Flow Trace:** `source feature visibility → coincident-point classification → existing marker/path display → original owner`
- **Files You Touch:** `src/feature_display.rs` (*Respect original feature visibility before generating a display.*) · `src/feature_style_tests.rs` (*Supply original-data style, visibility, placement and ownership witnesses.*) · `src/lib.rs` (*Register the independent source-style fixture.*) · `src/pick_gpu_tests.rs` (*Supply a real GPU marker ownership and CSS-diameter witness at DPR one and two.*)

A feature has source visibility as well as the owner’s Attributes or Interactions switch. Skip invisible features during projection. A global presentation switch cannot revive a source feature marked invisible. Its original Session payload remains available for saving and later regeneration.

Use the existing marker pipeline for an outline whose coordinates coincide after conversion to the renderer’s f32 format. Preserve the original outline color and set a twelve-pixel diameter on the temporary display point. Other outlines use two-pixel strokes without arrowheads. Clone only display inputs: original widths, colors, arrowheads and visibility remain unchanged.

Both paths and markers already resolve to their original owner through the integer picking table. Reuse that route; do not add another shader or readback. Contact polygon fills are developed separately.

A feature outline is display input, not a separately editable original. Its standalone visible and locked flags do not override the feature flag or the original owner. Set those flags only on the temporary display clone; preserve them unchanged in the original payload.

> **Self-Check Question:** *Why must a coincident outline become a marker before path preparation?*  
> **Answer:** The kernel permits coincident feature endpoints to describe a point. Path preparation removes duplicate vertices and requires a segment; the existing marker pipeline represents this source point with a stable screen diameter.

- **What You Should See:** Visible ordinary feature outlines retain their source colors and use the current screen-size conventions without altering original Session data.
- **Hands-On Lab Experiment:** Import an Element with a colored axis, an invisible cut and coincident drill endpoints. Enable Attributes: the axis appears with a two-pixel stroke, the drill remains a twelve-pixel disc, and the invisible cut stays absent. Change zoom and pixel density; predict which dimensions remain fixed in CSS pixels. Save and inspect the original feature widths and arrowheads.

<details>
<summary>Implementation 96z-preserve-source-feature-visibility-and-pixel-styles: Preserve source feature visibility and pixel styles</summary>

Project only visible original features, with two-pixel lines and twelve-pixel point discs.

Data flow: source feature visibility → coincident-point classification → existing marker/path display → original owner.

### 1. `src/feature_display.rs`

Respect original feature visibility before generating a display. Reuse marker and path preparation with source colors and current ordinary feature sizes.

<details>
<summary>Locate the existing block</summary>

```rust
use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
    let model = scene.placement(id).ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let mut source = outline.clone();
                    source.set_guid(uuid::Uuid::new_v4().to_string());
                    source.name = format!("piece {}", part + 1);
                    source.linecolor = session_rust::Color::black();
                    let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                    scene.place_following(piece, model.clone())?;
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(format!("piece {}", part + 1), piece));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(crate::feature_plan::label(feature), outlines));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() { groups.push(FeatureTree::Group("attributes".into(), own)); }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() { hosted.push(FeatureTree::Group(relation.label, children)); }
        }
        if !hosted.is_empty() { groups.push(FeatureTree::Group("interactions".into(), hosted)); }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{scene::{Scene, ObjectId}, layer_edit::FeatureTree};
use session_rust::{Element, Session};
use std::rc::Rc;

pub fn append(scene: &mut Scene, id: ObjectId, session: &Session, element: &Element) -> Result<(), &'static str> {
    let model = scene.placement(id).ok_or("Element placement is missing")?;
    let forest = std::mem::take(&mut scene.forest);
    let created = scene.created.take();
    let current = Rc::make_mut(&mut scene.view).current.take();
    let prepared = (|| {
        let mut groups = Vec::new();
        let mut pieces = |indices: &[usize]| -> Result<Vec<FeatureTree>, &'static str> {
            let mut rows = Vec::new();
            for &index in indices {
                let feature = &element.features[index];
                if !feature.visible { continue; }
                let mut outlines = Vec::new();
                for (part, outline) in feature.outlines.iter().enumerate() {
                    let point = outline.get_point(0).filter(|p| outline.coords.chunks_exact(3)
                        .all(|q| [q[0] as f32, q[1] as f32, q[2] as f32] == p.to_f32()));
                    let piece = if let Some(mut point) = point {
                        point.set_guid(uuid::Uuid::new_v4().to_string());
                        point.name = format!("piece {}", part + 1);
                        point.pointcolor = outline.linecolor.clone();
                        point.width = 12.0;
                        point.is_visible = true;
                        point.is_locked = false;
                        let piece = scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point))?)?;
                        scene.place_point(piece, model.clone())?;
                        piece
                    } else {
                        let mut source = outline.clone();
                        source.set_guid(uuid::Uuid::new_v4().to_string());
                        source.name = format!("piece {}", part + 1);
                        source.width = 2.0;
                        source.is_visible = true;
                        source.is_locked = false;
                        source.arrowhead = session_rust::Arrowhead::NONE;
                        let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                        scene.place_following(piece, model.clone())?;
                        piece
                    };
                    if !crate::feature_plan::interaction(feature) {
                        Rc::make_mut(&mut scene.view).hide(&[piece], true);
                    }
                    outlines.push(FeatureTree::Piece(format!("piece {}", part + 1), piece));
                }
                if !outlines.is_empty() {
                    rows.push(FeatureTree::Group(crate::feature_plan::label(feature), outlines));
                }
            }
            Ok(rows)
        };
        let own = pieces(&crate::feature_plan::own(element))?;
        if !own.is_empty() { groups.push(FeatureTree::Group("attributes".into(), own)); }
        let mut hosted = Vec::new();
        for relation in crate::feature_plan::hosted(session, element) {
            let children = pieces(&relation.features)?;
            if !children.is_empty() { hosted.push(FeatureTree::Group(relation.label, children)); }
        }
        if !hosted.is_empty() { groups.push(FeatureTree::Group("interactions".into(), hosted)); }
        Ok(groups)
    })();
    scene.forest = forest;
    scene.created = created;
    Rc::make_mut(&mut scene.view).current = current;
    crate::layer_edit::attach_features(scene, id, prepared?)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-01.rs"
```

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/feature_style_tests.rs`

Supply original-data style, visibility, placement and ownership witnesses.

Copy this supplied file:

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-03.rs"
```

</details>

### 3. `src/lib.rs`

Register the independent source-style fixture.

<details>
<summary>Locate the existing block</summary>

```rust
mod feature_pick_tests;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod feature_style_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-04.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/pick_gpu_tests.rs`

Supply a real GPU marker ownership and CSS-diameter witness at DPR one and two.

<details>
<summary>Locate the existing block</summary>

```text
#[test]
fn actual_integer_pick_preserves_high_ids_depth_and_locked_occlusion()
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96z-preserve-source-feature-visibility-and-pixel-styles-05.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 96z-preserve-source-feature-visibility-and-pixel-styles
npm --prefix ../session_tests run course -- check 96z-preserve-source-feature-visibility-and-pixel-styles
npm --prefix ../session_tests run course -- save 96z-preserve-source-feature-visibility-and-pixel-styles
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Visible ordinary feature outlines retain their source colors and use the current screen-size conventions without altering original Session data.

- **Final Lesson Experiment:** Import an Element with a colored axis, an invisible cut and coincident drill endpoints. Enable Attributes: the axis appears with a two-pixel stroke, the drill remains a twelve-pixel disc, and the invisible cut stays absent. Change zoom and pixel density; predict which dimensions remain fixed in CSS pixels. Save and inspect the original feature widths and arrowheads.

Continue with [Family-aware original controls and source previews](../../consolidation/96za-address-original-controls-with-family-keys.md).
