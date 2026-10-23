# 10.1 · Explicit fonts and source label placement

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 10.1** (Redraft `6.6`) · **3 checkpoints (`60-fonts` → `60b-coverage`)** · **Study time: 1.5–3 h**

> **What You Will Learn & Build:** Pin scene-text shaping and supply the viewer fonts. By the final checkpoint (`60b-coverage`), you will load explicit fonts and inspect missing coverage.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 10 Visual Explanation: Shelf-Packed R8Unorm Font Atlas, 3D World Text vs. Billboard & Matrix Shader</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Shelf-packed glyph atlas with 1px gutter. Center: Foreshortened 3D plane text vs billboard. Right: Procedural matrix.wgsl.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel A: Shelf-Packed Glyph Atlas Texture -->
    <rect x="18" y="64" width="245" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. R8Unorm Shelf Glyph Atlas</text>
    <rect x="52" y="94" width="176" height="100" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <!-- Shelf 1 -->
    <line x1="52" y1="138" x2="228" y2="138" stroke="#111111" stroke-width="1"/>
    <rect x="58" y="100" width="32" height="32" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="2,2"/>
    <text x="74" y="123" text-anchor="middle" font-size="18" font-weight="700" fill="#111111">A</text>
    <rect x="96" y="100" width="32" height="32" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="2,2"/>
    <text x="112" y="123" text-anchor="middle" font-size="18" font-weight="700" fill="#111111">B</text>
    <rect x="134" y="100" width="28" height="32" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="2,2"/>
    <text x="148" y="123" text-anchor="middle" font-size="18" font-weight="700" fill="#111111">g</text>
    <!-- Shelf 2 -->
    <rect x="58" y="144" width="28" height="32" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="2,2"/>
    <text x="72" y="167" text-anchor="middle" font-size="17" font-weight="700" fill="#111111">0</text>
    <rect x="92" y="144" width="28" height="32" fill="#ffffff" stroke="#111111" stroke-width="1" stroke-dasharray="2,2"/>
    <text x="106" y="167" text-anchor="middle" font-size="17" font-weight="700" fill="#111111">1</text>
    <text x="140" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">1px transparent gutter prevents bleed</text>
    <text x="140" y="225" text-anchor="middle" font-size="9" fill="#111111">queue.write_texture uploads dirty rects</text>

    <line x1="263" y1="154" x2="285" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel B: 3D World-Plane Text vs Screen Billboard -->
    <rect x="287" y="64" width="265" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. World-Plane vs. Billboard Text</text>
    <!-- Tilted 3D World Text Quad -->
    <polygon points="304,168 388,142 406,174 322,200" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="342" y="175" font-size="11" font-weight="700" fill="#111111" transform="rotate(-16 342 175)">WORLD</text>
    <text x="355" y="216" text-anchor="middle" font-size="9" fill="#111111">text_plane.wgsl (3D)</text>
    <!-- Screen-aligned Billboard with Leader Line -->
    <rect x="432" y="102" width="104" height="34" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="484" y="123" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Label #42</text>
    <line x1="464" y1="136" x2="438" y2="178" stroke="#111111" stroke-width="1.5"/>
    <circle cx="438" cy="178" r="3.5" fill="#111111"/>
    <text x="484" y="216" text-anchor="middle" font-size="9" fill="#111111">text.wgsl (Billboard)</text>

    <line x1="552" y1="154" x2="574" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel C: Procedural Matrix Shader (matrix.wgsl) -->
    <rect x="576" y="64" width="246" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="699" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Procedural Matrix (matrix.wgsl)</text>
    <text x="699" y="100" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">Stiffness K (3×3) — Plate-less Title</text>
    <!-- 3x2 Matrix Grid -->
    <rect x="604" y="108" width="190" height="84" fill="#ffffff" stroke="#111111" stroke-width="2"/>
    <line x1="667" y1="108" x2="667" y2="192" stroke="#111111" stroke-width="1"/>
    <line x1="730" y1="108" x2="730" y2="192" stroke="#111111" stroke-width="1"/>
    <line x1="604" y1="150" x2="794" y2="150" stroke="#111111" stroke-width="1"/>
    <text x="635" y="133" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">0.425</text>
    <!-- Zero cell left blank intentionally! -->
    <text x="698" y="133" text-anchor="middle" font-size="9" fill="#111111">(0 blank)</text>
    <text x="762" y="133" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">-1.200</text>
    <rect x="604" y="150" width="63" height="42" fill="url(#hatch)" stroke="#111111" stroke-width="1"/>
    <text x="635" y="175" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">3.142</text>
    <text x="698" y="175" text-anchor="middle" font-size="10" font-weight="700" fill="#111111">0.008</text>
    <text x="762" y="175" text-anchor="middle" font-size="9" fill="#111111">(0 blank)</text>
    <text x="699" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">16-word header + 3 words/cell</text>
    <text x="699" y="226" text-anchor="middle" font-size="9" fill="#111111">Whole-matrix Rg32Uint pick + border</text>
    
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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 10.1 Pipeline Schematic: Explicit fonts and source label placement</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (3 Checkpoints: 60-fonts → 60b-coverage)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">pinned shaping</text>
    <text x="89" y="124" text-anchor="middle" font-size="11" fill="#111111">renderer</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">fixed font assets</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">subset coverage</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">reproducible glyph</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">layout</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">bundled font bytes</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 10.1:</text>
    <text x="235" y="177" font-size="11" fill="#111111">Cargo.toml  ·  index.html  ·  src/text.rs  ·  src/lib.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">3 Checkpoints: 60-fonts → 60b-coverage — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **Rust / WebGPU** | Shelf-Packed `R8Unorm` Glyph Atlas + 1px Padding | Rasterizes Unicode glyphs on demand into a single-channel GPU texture with a 1-pixel transparent gutter so bilinear filtering never bleeds neighbor glyphs. |
| **WGSL (`text_plane.wgsl` / `text.wgsl`)** | World-Plane Quad vs. Screen-Aligned Billboard Quad | `text_plane.wgsl` expands glyph quads along 3D plane axes `(plane_x, plane_y)` that foreshorten in perspective; `text.wgsl` expands along camera `(Right, Up)` vectors. |
| **WGSL (`matrix.wgsl`)** | Procedural Matrix Cell Digits (16-word Header + 3 words/cell) | Draws heatmap cells and procedural 3-decimal digits directly in the fragment shader (leaving `0` cells blank), with a plate-less world title and yellow selection border. |

**Authoritative production source (`session_viewer/src`):** [engine/text.rs](../../../../src/engine/text.rs), [engine/gpu/text.rs](../../../../src/engine/gpu/text.rs), [engine/gpu/text_plane.rs](../../../../src/engine/gpu/text_plane.rs), [engine/gpu/text_plate.rs](../../../../src/engine/gpu/text_plate.rs), [app/scene_text.rs](../../../../src/app/scene_text.rs), [app/fonts.rs](../../../../src/app/fonts.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`60-fonts`](#checkpoint-60-fonts) | Pin scene-text shaping and supply the viewer fonts. | `Cargo.toml`, `index.html` | `pinned shaping renderer → fixed font assets → subset coverage → reproducible glyph layout` |
| **2** | [`60a-labels`](#checkpoint-60a-labels) | Keep source words metrics and placement together. | `src/text.rs`, `src/lib.rs` | `original words → label metrics and identity → placement kind → shaped run` |
| **3** | [`60b-coverage`](#checkpoint-60b-coverage) | Load explicit fonts and inspect missing coverage. | `src/text.rs` | `bundled font bytes → explicit font database → Unicode cmap → coverage decision` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-60-fonts"></a>

### Step 1 of 3 · Pin scene-text shaping and supply the viewer fonts (`60-fonts`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/3 Visual Trace (60-fonts): Pin scene-text shaping and supply the viewer fonts</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: Cargo.toml, index.html</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">pinned shaping renderer</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">fixed font assets</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">subset coverage</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">reproducible glyph layout</text>
    
</svg>
</div>

- **Goal:** Pin scene-text shaping and supply the viewer fonts.
- **Data Flow Trace:** `pinned shaping renderer → fixed font assets → subset coverage → reproducible glyph layout`
- **Files You Touch:** `Cargo.toml` (*Pin the production text renderer and explicit glyph coverage parser.*) · `index.html` (*Make the supplied whole fonts available at the current text asset URLs.*)

Use the current Glyphon release compatible with the viewer GPU API and the viewer’s explicit Noto main/symbol fonts. Keep the font licence with the supplied assets.

> **Self-Check Question:** *Should a CAD label depend on whichever fonts happen to be installed?*  
> **Answer:** No. Explicit bundled fonts make its glyph coverage and layout reproducible. Main and symbol fonts handle the viewer’s strings; whole fonts are available when the subsets lack a character.

- **What You Should See:** The blue CAD bore remains visible. Run the supplied text shaping checks; glyph coverage is drawn through the atlas in 61 and scene labels connect in 62.
- **Hands-On Lab Experiment:** Inspect the supplied main and symbol fonts and OFL.txt. Keep the licence beside them when distributing assets, and compare subset coverage with the full fonts.

<details>
<summary>Implementation 60-fonts: Pin scene-text shaping and supply the viewer fonts</summary>

Pin scene-text shaping and supply the viewer fonts.

Data flow: pinned shaping renderer → fixed font assets → subset coverage → reproducible glyph layout.

### 1. `Cargo.toml`

Pin the production text renderer and explicit glyph coverage parser.

<details>
<summary>Locate the existing block</summary>

```toml
serde_json = "1.0"

[dev-dependencies]
pollster = "=0.4.0"
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```toml
--8<-- "journey/code/60-fonts-01.rs"
```

### 2. `index.html`

Make the supplied whole fonts available at the current text asset URLs.

<details>
<summary>Locate the existing block</summary>

```html
  <title>Session Viewer</title>
  <link data-trunk rel="rust">
  <style>
    html, body { width: 100%; height: 100%; margin: 0; overflow: hidden; }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```html
--8<-- "journey/code/60-fonts-02.rs"
```

</details>

<a id="checkpoint-60a-labels"></a>

### Step 2 of 3 · Keep source words metrics and placement together (`60a-labels`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/3 Visual Trace (60a-labels): Keep source words metrics and placement together</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/text.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original words</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">label metrics and identity</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">placement kind</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">shaped run</text>
    
</svg>
</div>

- **Goal:** Keep source words metrics and placement together.
- **Data Flow Trace:** `original words → label metrics and identity → placement kind → shaped run`
- **Files You Touch:** `src/text.rs` (*Define current label/source placement types and explicit bundled font constants.*) · `src/lib.rs` (*Register the source component.*)

Retain original text, source identity, colour and CSS metrics beside an explicit screen, anchor, nameplate, world-plane or billboard placement. World positions and axes remain f64.

> **Self-Check Question:** *Is the source text a list of positioned glyphs?*  
> **Answer:** No. Words and placement are source data. Shaping derives glyphs and clusters, and rendering derives their coverage. Moving or recolouring a label should not rewrite its words.

- **Hands-On Lab Experiment:** Compare CSS font size with a world-plane world height. Keep the original world coordinates unchanged while changing the screen colour or clip rectangle.

<details>
<summary>Implementation 60a-labels: Keep source words metrics and placement together</summary>

Keep source words metrics and placement together.

Data flow: original words → label metrics and identity → placement kind → shaped run.

### 1. `src/text.rs`

Define current label/source placement types and explicit bundled font constants.

Create the file and type:

```rust
--8<-- "journey/code/60a-labels-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod cad_display_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60a-labels-02.rs" skip=36
```

</details>

<a id="checkpoint-60b-coverage"></a>

### Step 3 of 3 · Load explicit fonts and inspect missing coverage (`60b-coverage`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/3 Visual Trace (60b-coverage): Load explicit fonts and inspect missing coverage</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/text.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">bundled font bytes</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">explicit font database</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">Unicode cmap</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">coverage decision</text>
    
</svg>
</div>

- **Goal:** Load explicit fonts and inspect missing coverage.
- **Data Flow Trace:** `bundled font bytes → explicit font database → Unicode cmap → coverage decision`
- **Files You Touch:** `src/text.rs` (*Load shared font bytes and inspect their actual Unicode coverage.*)

Load the current bundled fonts into an explicit font database and read their Unicode character maps. Coverage can request whole fonts before the renderer silently loses a glyph.

> **Self-Check Question:** *Does an absent subset character mean its whole font lacks that character too?*  
> **Answer:** No. The subset deliberately contains fewer glyphs. A coverage miss can require the supplied whole font; it should not be treated as evidence that the original source text is invalid.

- **Hands-On Lab Experiment:** Compare the viewer’s Lithuanian, German and CAD symbol strings with Fixed Ω cube. Verify the subset miss, then load the whole fonts and shape a nonzero glyph.

<details>
<summary>Implementation 60b-coverage: Load explicit fonts and inspect missing coverage</summary>

Load explicit fonts and inspect missing coverage.

Data flow: bundled font bytes → explicit font database → Unicode cmap → coverage decision.

### 1. `src/text.rs`

Load shared font bytes and inspect their actual Unicode coverage.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60b-coverage-01.rs" skip=9
```

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 60b-coverage
npm --prefix ../session_tests run course -- check 60b-coverage
npm --prefix ../session_tests run course -- save 60b-coverage
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 10.1 (60b-coverage)](../../../screenshots/journey/60b-coverage.png)

- **Final Lesson Experiment:** Compare the viewer’s Lithuanian, German and CAD symbol strings with Fixed Ω cube. Verify the subset miss, then load the whole fonts and shape a nonzero glyph.

Continue with [Validated shaping and revision-based reuse](../../consolidation/60ba-validation.md).
