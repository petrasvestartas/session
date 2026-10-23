# 10.1 · Explicit fonts and source label placement

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 10.1** (Redraft `6.6`) · **3 checkpoints (`60-fonts` → `60b-coverage`)**

> **Learning Goal:** Pin scene-text shaping and supply the viewer fonts.

## Visual WebGPU & Architecture Schematic

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1px solid #cbd5e1;border-radius:10px;background:#f8fafc;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#475569"/>
    </marker>
    <marker id="arrow-blue" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <marker id="arrow-green" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#059669"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#e2e8f0" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Chapter 10 WebGPU Text & Shared Arena Pipeline: Glyph Atlas Packing, World/Billboard Text & Matrix Shader</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How crisp typography (text_plane.wgsl / text.wgsl) and procedural cell digits (matrix.wgsl) render in 3D world space.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="235" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="135" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. Shelf-Packed Glyph Atlas</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Rasterize needed Unicode codepoints</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• Pack into R8Unorm GPU Atlas Texture</text>
    <text x="28" y="142" font-size="10" fill="#1e3a8a">  with 1px padding (no bilinear bleed!)</text>
    <text x="28" y="160" font-size="10.5" fill="#1e3a8a">• queue.write_texture dirty rects only</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">• Layout baseline, kerning &amp; line</text>
    <text x="28" y="193" font-size="10.5" fill="#1e3a8a">  wraps into instanced glyph quads</text>

    <line x1="253" y1="138" x2="283" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="285" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="417" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. World-Plane vs Screen Billboard</text>
    <text x="297" y="108" font-size="10.5" font-weight="600" fill="#c2410c">• text_plane.wgsl (3D World Text):</text>
    <text x="297" y="123" font-size="10" fill="#7c2d12">  q_world = origin + u*plane_x + v*plane_y</text>
    <text x="297" y="138" font-size="10" fill="#7c2d12">  Foreshortens naturally with 3D perspective</text>
    <text x="297" y="158" font-size="10.5" font-weight="600" fill="#c2410c">• text.wgsl (Screen-Aligned Label):</text>
    <text x="297" y="173" font-size="10" fill="#7c2d12">  Expands along camera Right/Up vectors;</text>
    <text x="297" y="188" font-size="10" fill="#7c2d12">  optional background plate + 3D leader line</text>

    <line x1="550" y1="138" x2="580" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="582" y="68" width="238" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="701" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. Whole-Text Pick &amp; Matrix (Sec 10)</text>
    <text x="594" y="108" font-size="10.5" fill="#064e3b">• Clicking any glyph or plate writes</text>
    <text x="594" y="124" font-size="10.5" font-weight="600" fill="#047857">  the parent Text owner_id to Rg32Uint</text>
    <text x="594" y="144" font-size="10.5" fill="#064e3b">• Later in Sec 10 (matrix.wgsl):</text>
    <text x="594" y="160" font-size="10" fill="#064e3b">  16-word header + 3 words/cell renders</text>
    <text x="594" y="175" font-size="10" fill="#064e3b">  procedural 3-decimal digits (0 blank)</text>
    <text x="594" y="190" font-size="10" fill="#064e3b">  + plate-less world title &amp; yellow select</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
pinned shaping renderer → fixed font assets → subset coverage → reproducible glyph layout.
original words → label metrics and identity → placement kind → shaped run.
bundled font bytes → explicit font database → Unicode cmap → coverage decision.
```

- Pin scene-text shaping and supply the viewer fonts.
- Keep source words metrics and placement together.
- Load explicit fonts and inspect missing coverage.
- **Why it works (Should a CAD label depend on whichever fonts happen to be installed?):** No. Explicit bundled fonts make its glyph coverage and layout reproducible. Main and symbol fonts handle the viewer’s strings; whole fonts are available when the subsets lack a character.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **Rust / WebGPU** | Shelf-Packed `R8Unorm` Glyph Atlas + 1px Padding | Rasterizes Unicode glyphs on demand into a single-channel GPU texture with a 1-pixel transparent gutter so bilinear filtering never bleeds neighbor glyphs. |
| **WGSL (`text_plane.wgsl` / `text.wgsl`)** | World-Plane Quad vs. Screen-Aligned Billboard Quad | `text_plane.wgsl` expands glyph quads along 3D plane axes `(plane_x, plane_y)` that foreshorten in perspective; `text.wgsl` expands along camera `(Right, Up)` vectors. |
| **WGSL (`matrix.wgsl`)** | Procedural Matrix Cell Digits (16-word Header + 3 words/cell) | Draws heatmap cells and procedural 3-decimal digits directly in the fragment shader (leaving `0` cells blank), with a plate-less world title and yellow selection border. |

**Production source (`session_viewer/src`):** [engine/text.rs](../../../src/engine/text.rs), [engine/gpu/text.rs](../../../src/engine/gpu/text.rs), [engine/gpu/text_plane.rs](../../../src/engine/gpu/text_plane.rs), [engine/gpu/text_plate.rs](../../../src/engine/gpu/text_plate.rs), [app/scene_text.rs](../../../src/app/scene_text.rs), [app/fonts.rs](../../../src/app/fonts.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-60-fonts"></a>

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

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 60b-coverage
npm --prefix ../session_tests run course -- check 60b-coverage
npm --prefix ../session_tests run course -- save 60b-coverage
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

- **Try this:** Compare the viewer’s Lithuanian, German and CAD symbol strings with Fixed Ω cube. Verify the subset miss, then load the whole fonts and shape a nonzero glyph.

Continue with [Validated shaping and revision-based reuse](60ba-validation.md).
