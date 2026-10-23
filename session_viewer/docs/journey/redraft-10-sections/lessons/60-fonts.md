# 10.1 · Explicit fonts and source label placement

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 10.1** (Redraft `6.6`) · **3 checkpoints (`60-fonts` → `60b-coverage`)** · **Study time: 1.5–3 h**

> **What You Will Learn & Build:** Pin scene-text shaping and supply the viewer fonts. By the final checkpoint (`60b-coverage`), you will load explicit fonts and inspect missing coverage.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 10.1 Execution Flow: Explicit fonts and source label placement</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (3 Checkpoints: 60-fonts → 60b-coverage)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">pinned shaping</text>
    <text x="89" y="124" text-anchor="middle" font-size="11" fill="#111111">renderer</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">fixed font assets</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">subset coverage</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">reproducible glyph</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">layout</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">bundled font bytes</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 10.1:</text>
    <text x="215" y="181" font-size="11" fill="#111111">Cargo.toml  ·  index.html  ·  src/text.rs  ·  src/lib.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">3 Checkpoints: 60-fonts → 60b-coverage — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 10 WebGPU Text & Shared Arena Pipeline: Glyph Atlas Packing, World/Billboard Text & Matrix Shader</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How crisp typography (text_plane.wgsl / text.wgsl) and procedural cell digits (matrix.wgsl) render in 3D world space.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="235" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="135" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Shelf-Packed Glyph Atlas</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Rasterize needed Unicode codepoints</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Pack into R8Unorm GPU Atlas Texture</text>
    <text x="28" y="142" font-size="10" fill="#111111">  with 1px padding (no bilinear bleed!)</text>
    <text x="28" y="160" font-size="10.5" fill="#111111">• queue.write_texture dirty rects only</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">• Layout baseline, kerning &amp; line</text>
    <text x="28" y="193" font-size="10.5" fill="#111111">  wraps into instanced glyph quads</text>

    <line x1="253" y1="138" x2="283" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="285" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="417" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. World-Plane vs Screen Billboard</text>
    <text x="297" y="108" font-size="10.5" font-weight="600" fill="#111111">• text_plane.wgsl (3D World Text):</text>
    <text x="297" y="123" font-size="10" fill="#111111">  q_world = origin + u*plane_x + v*plane_y</text>
    <text x="297" y="138" font-size="10" fill="#111111">  Foreshortens naturally with 3D perspective</text>
    <text x="297" y="158" font-size="10.5" font-weight="600" fill="#111111">• text.wgsl (Screen-Aligned Label):</text>
    <text x="297" y="173" font-size="10" fill="#111111">  Expands along camera Right/Up vectors;</text>
    <text x="297" y="188" font-size="10" fill="#111111">  optional background plate + 3D leader line</text>

    <line x1="550" y1="138" x2="580" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="582" y="68" width="238" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="701" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Whole-Text Pick &amp; Matrix (Sec 10)</text>
    <text x="594" y="108" font-size="10.5" fill="#111111">• Clicking any glyph or plate writes</text>
    <text x="594" y="124" font-size="10.5" font-weight="600" fill="#111111">  the parent Text owner_id to Rg32Uint</text>
    <text x="594" y="144" font-size="10.5" fill="#111111">• Later in Sec 10 (matrix.wgsl):</text>
    <text x="594" y="160" font-size="10" fill="#111111">  16-word header + 3 words/cell renders</text>
    <text x="594" y="175" font-size="10" fill="#111111">  procedural 3-decimal digits (0 blank)</text>
    <text x="594" y="190" font-size="10" fill="#111111">  + plate-less world title &amp; yellow select</text>
    
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

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-60-fonts"></a>

### Step 1 of 3 · Pin scene-text shaping and supply the viewer fonts (`60-fonts`)

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

- **Final Lesson Experiment:** Compare the viewer’s Lithuanian, German and CAD symbol strings with Fixed Ω cube. Verify the subset miss, then load the whole fonts and shape a nonzero glyph.

Continue with [Validated shaping and revision-based reuse](../../consolidation/60ba-validation.md).
