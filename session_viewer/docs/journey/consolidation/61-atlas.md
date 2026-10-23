# 10.3 · A shared glyph atlas and lazy text drawing

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Checkpoints:** `61-atlas` → `61d-glyphs` (5 steps) · **Production `session_viewer/src` target:** [engine/text.rs](../../../src/engine/text.rs) · [engine/gpu/text.rs](../../../src/engine/gpu/text.rs) · [engine/gpu/text_plane.rs](../../../src/engine/gpu/text_plane.rs) · [engine/gpu/text_plate.rs](../../../src/engine/gpu/text_plate.rs) · [app/scene_text.rs](../../../src/app/scene_text.rs) · [app/fonts.rs](../../../src/app/fonts.rs) (`src/app/fonts.rs,src/shaders/matrix.wgsl,src/engine/gpu/matrices.rs`) · **Goal:** Draw the specimen and prove real coverage reuse.

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 10.3 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/text_atlas.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/text_example.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">61-atlas -&gt; 61d-glyphs</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/fonts.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/matrix.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Own glyph coverage in a s…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. R8Unorm Glyph Atlas (B&amp;W)</text>
  <rect x="616" y="54" width="168" height="84" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="628" y="64" width="34" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="645" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">A</text>
  <rect x="670" y="64" width="34" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="687" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">B</text>
  <rect x="712" y="64" width="34" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="729" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">7</text>
  <text x="700" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1px Padded Shelves</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">A shared glyph atlas and la…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Procedural matrix.wgsl</text>
  <rect x="914" y="54" width="120" height="84" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="914" y="54" width="40" height="28" fill="#fdba74"/><text x="934" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <rect x="954" y="82" width="40" height="28" fill="#f97316"/><text x="974" y="100" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#ffffff">+2.4</text>
  <rect x="994" y="54" width="40" height="28" fill="#93c5fd"/><text x="1014" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">-0.8</text>
  <rect x="994" y="110" width="40" height="28" fill="#fdba74"/><text x="1014" y="128" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <line x1="954" y1="54" x2="954" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="994" y1="54" x2="994" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="82" x2="1034" y2="82" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="110" x2="1034" y2="110" stroke="#111111" stroke-width="1"/>
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Draw the specimen and pro…</text>
</svg>
</div>

## <a id="checkpoint-61-atlas"></a>Step 1 of 5 · Own glyph coverage in a shared GPU atlas (`61-atlas`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/5: 61-atlas</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_atlas.rs, src/lib.…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">explicit shaped fonts</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">glyph coverage atlas</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">text renderer</text>
</svg>
</div>

Create the current Glyphon cache, accurate-colour atlas, viewport, rasteriser and text renderer. Keep this lane lazy so scenes without labels allocate no extra text atlas.

> **Check:** *Must every repeated glyph get another texture allocation?* — No. Glyph coverage belongs in a shared atlas. Positioned glyph instances can reuse that coverage while changing placement or colour. The lane is created only when labels exist.

### 1. `src/text_atlas.rs`

Own the current accurate-colour atlas and cache with separate frame/source revisions.

Create the file and type:

```rust
--8<-- "journey/code/61-atlas-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

After that block, type:

```rust
--8<-- "journey/code/61-atlas-02.rs" skip=15
```

**Result after `61-atlas`:** The blue CAD bore remains visible. Connect the glyph lane and draw Example Text at the final checkpoint of 61.

## <a id="checkpoint-61a-coverage"></a>Step 2 of 5 · Prepare DPI-scaled glyph bounds and cache unchanged frames (`61a-coverage`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/5: 61a-coverage</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_atlas.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">CSS placement and clipping</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">glyph preparation</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">unchanged-frame reuse</text>
</svg>
</div>

Convert CSS placement and clip bounds to physical pixels, prepare actual glyph coverage and reuse an unchanged document/font/viewport/density frame. Accepted font replacement resets stale atlas state.

> **Check:** *Should a density change double the label’s CSS size?* — No. Physical glyph pixels and bounds scale with density while the label keeps its CSS size. The changed density requires preparation and new coverage where needed, not new source words.

### 1. `src/text_atlas.rs`

Prepare real glyph coverage, pixel clips, revision resets and truthful CPU raster-cache counts.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/61a-coverage-01.rs"
```

## <a id="checkpoint-61b-specimen"></a>Step 3 of 5 · Supply a real-font CAD text specimen (`61b-specimen`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/5: 61b-specimen</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_example.rs, src/li…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original specimen words</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fractional position</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">real glyph coverage</text>
</svg>
</div>

Use an explicit fractional CSS position, fixed metrics and viewer-supported ligatures, accents and symbols. This specimen is a presentation overlay and creates no scene geometry row.

> **Check:** *Does this screen specimen become a saved CAD object?* — No. It demonstrates the glyph lane as a presentation overlay. Source-owned scene labels and their placements connect in 62. Its words still shape through the actual bundled fonts.

### 1. `src/text_example.rs`

Provide a current-font specimen that exercises real glyph coverage.

Create the file and type:

```rust
--8<-- "journey/code/61b-specimen-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

After that block, type:

```rust
--8<-- "journey/code/61b-specimen-02.rs" skip=21
```

## <a id="checkpoint-61c-renderer"></a>Step 4 of 5 · Draw lazy text coverage after the scene (`61c-renderer`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/5: 61c-renderer</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">scene and handles</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">colour load pass</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">unchanged geometry and pi…</text>
</svg>
</div>

Create the atlas only for a nonempty label set, render its coverage in a separate load pass and preserve primary geometry/picking. Retain text preparation/draw errors in diagnostics instead of silently dropping them.

> **Check:** *Should enabling the screen text reupload the CAD mesh?* — No. It adds glyph presentation after the existing scene. Original CAD geometry, source ownership and integer pick tables remain unchanged. An empty label set releases the text lane.

### 1. `src/renderer.rs`

Draw a lazy glyph lane independently from primary geometry and expose preparation errors.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder,view);
        self.queue.submit([encoder.finish()]);
    }
}
```

Replace it with:

```rust
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.device,&self.queue,self.size,self.density) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("screen glyphs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.queue.submit([encoder.finish()]);
    }
}
```

**Locate this block:**

```rust
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
```

Replace it with:

```rust
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    pub fn set_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();let lane=lane.get_or_insert_with(||crate::text_atlas::Lane::new(&self.device,&self.queue,self.format));lane.document.set_labels(labels)
    }
    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{let [rasters,bytes]=lane.raster_usage();[lane.document.shape_count,lane.preparations,lane.skips,rasters as u64,bytes as u64,lane.resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
```

**Locate this block:**

```rust
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self { device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

Replace it with:

```rust
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self { text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
```

**Locate this block:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    text:std::cell::RefCell<Option<crate::text_atlas::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
```

**Complete updated block reference:**

```rust
use crate::{gpu_mesh::GpuMesh, scene::{ObjectId, Scene}};

pub struct Renderer {
    text:std::cell::RefCell<Option<crate::text_atlas::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
    meshes: Vec<GpuMesh>,
    geometry: crate::geometry_cache::GeometryCache,
    settings_allocations: usize,
    settings_writes: usize,
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    size: [u32; 2],
    density: f32,
    picker: crate::pick_gpu::Picker,
    sources: crate::pick_sources::Sources,
    faces: crate::face_pick::Faces,
    mesh_ink: crate::stroke_gpu::Lane,
    pick_mode: crate::pick_sources::Mode,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let view_layout = crate::gpu_layout::uniform(&device, 80);
        let model_layout = crate::gpu_layout::uniform(&device, 160);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout)], immediate_size: 0 });
        let mesh_pipeline = |depth| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }, wgpu::VertexBufferLayout {
                    array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface());
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let picker = crate::pick_gpu::Picker::new(&device, &view_layout, &model_layout);
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self { text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, strokes, paths, markers, grid, grid_settings: None, size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode: Default::default() };
        renderer.set_scene(scene, None);
        renderer
    }

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.picker.cancel(); }
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.picker.cancel(); self.density = density; }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [count, bytes, self.strokes.uploads, self.strokes.uploaded_bytes]
    }

    pub fn path_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.paths.usage();
        [count, bytes, self.paths.uploads, self.paths.uploaded_bytes]
    }

    pub fn point_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.markers.usage();
        [count, bytes, self.markers.uploads, self.markers.uploaded_bytes]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.sources = crate::pick_sources::Sources::new(scene).expect("Prepared source topology is bounded");
        self.mesh_ink.set(&self.device, self.sources.mesh_ink.iter().copied()).expect("Prepared source edges");
        self.mesh_ink.set_ids(&self.device, self.sources.mesh_ids.iter().copied()).expect("Source edge IDs match");
        self.markers.set_points(&self.device, scene.points().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut marker=row.marker();if let Some(colour)=scene.view.faces.get(&row.id) {marker.colour=rgba(*colour);}marker}).chain(scene.control_markers().into_iter().map(|c| c.display)))
            .expect("Prepared point placement fits the display range");
        self.paths.set_chains(&self.device, scene.paths().iter().filter(|row| scene.visible(row.id)).flat_map(|row| {let mut segments=row.segments();if let Some(colour)=scene.view.faces.get(&row.id) {for segment in &mut segments {segment.stroke.colour=rgba(*colour);}}segments}))
            .expect("Prepared path placement fits the display range");
        self.strokes.set(&self.device, scene.lines().iter().filter(|row| scene.visible(row.id)).map(|row| {let mut stroke=row.stroke();if let Some(colour)=scene.view.faces.get(&row.id) {stroke.colour=rgba(*colour);}stroke}))
            .expect("Prepared line placement fits the display range");
        self.markers.set_ids(&self.device, self.sources.marker_ids.iter().copied()).expect("Marker owners match");
        self.paths.set_ids(&self.device, self.sources.path_ids.iter().copied()).expect("Path owners match");
        self.strokes.set_ids(&self.device, self.sources.line_ids.iter().copied()).expect("Line owners match");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
                && std::rc::Rc::ptr_eq(&row.geometry.source, &object.mesh));
            let row = if let Some(index) = matching {
                let mut row = old.swap_remove(index);
                let [calls, bytes] = row.update(&self.queue, object,style(scene,object.id,selected==Some(object.id)));
                self.settings_writes += calls;
                self.settings_bytes += bytes;
                row
            } else {
                let geometry = self.geometry.get(&self.device, &object.mesh);
                self.settings_allocations += 1;
                GpuMesh::with_geometry(&self.device, &layout, object,
                    style(scene,object.id,selected==Some(object.id)), geometry)
            };
            self.meshes.push(row);
        }
        drop(old);
        self.geometry.prune();
    }

    pub fn request_pick(&mut self, screen: [f32; 2], transform: [f32; 16]) -> Result<(), &'static str> {
        let window = crate::pick_window::Window::new(screen, self.size, self.density).ok_or("Invalid pick cursor")?;
        self.picker.queue(crate::pick_gpu::Request { window, transform, density: self.density, mode: self.pick_mode })?;
        self.pump_pick(transform); Ok(())
    }

    fn pump_pick(&mut self, transform: [f32; 16]) {
        let Ok(view) = crate::pick_view::View::new(transform, self.size, self.density) else { self.picker.cancel(); return; };
        if self.pick_mode == crate::pick_sources::Mode::Face && !self.faces.ready(self.meshes.len()) {
            self.faces.set(&self.device, &self.meshes, &self.sources.mesh_bases);
        }
        if let Some(request) = self.picker.next(view) { self.picker.submit(&self.device, &self.queue, request,
            &self.meshes, &[&self.mesh_ink, &self.strokes, &self.paths, &self.markers], &self.faces); }
    }

    pub fn take_pick(&mut self, transform: [f32; 16]) -> Option<Result<Option<[u32; 2]>, &'static str>> {
        let view = match crate::pick_view::View::new(transform, self.size, self.density) { Ok(view) => view, Err(error) => {
            self.picker.cancel(); return Some(Err(error));
        } };
        match self.picker.take(view) {
            Some(Ok(crate::pick_gpu::Reply::Hit(hit))) => Some(Ok(hit)),
            Some(Ok(crate::pick_gpu::Reply::Stale)) => { self.pump_pick(transform); None },
            Some(Err(error)) => Some(Err(error)), None => None,
        }
    }

    pub fn set_pick_mode(&mut self, mode: crate::pick_sources::Mode) {
        if self.pick_mode != mode { self.picker.cancel(); self.pick_mode = mode; }
    }

    pub fn resolve_pick(&self, scene: &Scene, pixel: [u32; 2]) -> Option<crate::pick_sources::Target> {
        self.sources.resolve(scene, pixel, self.pick_mode)
    }

    pub fn part_usage(&self) -> [u64; 4] { let [count, bytes] = self.mesh_ink.usage(); [count, bytes, self.faces.bytes(), self.sources.bytes() as u64] }

    pub fn cancel_pick(&mut self) { self.picker.cancel(); }

    pub fn pick_usage(&self) -> [u64; 5] {
        let [active, pending, bytes] = self.picker.usage();
        [active, pending, bytes + self.strokes.pick_bytes() + self.paths.pick_bytes() + self.markers.pick_bytes() + self.mesh_ink.pick_bytes() + self.mesh_ink.usage()[1] + self.faces.bytes(),
            self.picker.submitted, self.picker.coalesced]
    }

    pub fn geometry_owners(&self) -> impl Iterator<Item = &std::rc::Rc<crate::gpu_geometry::GpuGeometry>> {
        self.meshes.iter().map(|row| &row.geometry)
    }

    pub fn usage(&self) -> [u64; 4] { crate::memory::gpu(&self.meshes) }

    pub fn uploaded_bytes(&self) -> u64 {
        self.geometry.uploaded_bytes + self.settings_allocations as u64 * 160 + self.settings_bytes as u64
            + self.strokes.uploaded_bytes + self.paths.uploaded_bytes + self.markers.uploaded_bytes + self.grid.uploaded_bytes
    }

    pub fn set_grid(&mut self, settings: Option<crate::grid::Settings>) -> Result<(), &'static str> {
        if settings == self.grid_settings { return Ok(()); }
        let strokes = settings.map(crate::grid::Settings::strokes).transpose()?.unwrap_or_default();
        self.grid.set(&self.device, strokes)?; self.grid_settings = settings; Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage(); [count, bytes, self.grid.uploads, self.grid.uploaded_bytes]
    }

    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, target)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
        for mesh in &mut self.meshes { if let Some(row) = scene.objects().iter().find(|r| r.id == mesh.id) {
            let [calls, bytes] = mesh.update(&self.queue, row,style(scene,row.id,ids.contains(&row.id)));
            self.settings_writes += calls; self.settings_bytes += bytes;
        } }
        if !ids.is_empty() { self.highlight.objects(&self.device, scene, ids)?; } Ok(())
    }

    pub fn set_preview(&mut self, draft: Option<&crate::tool::Draft>) -> Result<(), &'static str> { self.preview.set(&self.device, draft) }
    pub fn preview_bytes(&self) -> u64 { self.preview.bytes() }

    pub fn set_gizmo(&mut self, camera:&crate::camera::Camera, origin:Option<[f64;3]>, active:f32) {
        let matrix=camera.view_projection();
        let layout=origin.and_then(|p|crate::widget_layout::world_per_pixel(&matrix,p,self.size[1] as f64/self.density as f64)
            .and_then(|scale|crate::widget_layout::layout(&matrix,p,scale,self.size,active)));
        self.widget.prepare(&self.device,&self.queue,layout);
    }
    pub fn gizmo_usage(&self) -> [u64;4] { self.widget.usage() }
    pub fn gizmo_extent(&self) -> Option<[u32;2]> {self.widget.extent()}

    pub fn set_text(&self,labels:Vec<crate::text::TextLabel>)->anyhow::Result<()> {
        if labels.is_empty() {self.text.borrow_mut().take();return Ok(());}
        let mut lane=self.text.borrow_mut();let lane=lane.get_or_insert_with(||crate::text_atlas::Lane::new(&self.device,&self.queue,self.format));lane.document.set_labels(labels)
    }
    pub fn text_error(&self)->Option<String> {self.text_error.borrow().clone()}
    pub fn text_usage(&self)->[u64;6] {self.text.borrow().as_ref().map_or([0;6],|lane|{let [rasters,bytes]=lane.raster_usage();[lane.document.shape_count,lane.preparations,lane.skips,rasters as u64,bytes as u64,lane.resets]})}
    pub fn text_demo(&self)->anyhow::Result<()> {self.set_text(vec![crate::text_example::label()])}

    pub fn highlight_bytes(&self) -> u64 { self.highlight.bytes() }

    pub fn stats(&self) -> [usize; 4] {
        [self.geometry.uploads, self.settings_allocations, self.settings_writes, self.settings_bytes]
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) { self.draw_mode(view, background, transform, false); }

    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let mut bytes: Vec<u8> = transform.iter().flat_map(|value| value.to_ne_bytes()).collect();
        let display = [if normals { 1.0f32 } else { 0.0 }, 0.0, 0.0, 0.0];
        bytes.extend(display.iter().flat_map(|value| value.to_ne_bytes()));
        self.queue.write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(&self.queue, transform, self.size, self.density);
        self.paths.view(&self.queue, transform, self.size, self.density);
        self.markers.view(&self.queue, transform, self.size, self.density);
        self.grid.view(&self.queue, transform, self.size, self.density);
        self.mesh_ink.view(&self.queue, transform, self.size, self.density);
        self.highlight.view(&self.queue, transform, self.size, self.density);
        self.preview.view(&self.queue, transform, self.size, self.density);
        let [r, g, b] = background.rgb();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r, g, b, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
            self.paths.draw(&mut pass);
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder,view);
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
            match lane.prepare(&self.device,&self.queue,self.size,self.density) {
            Ok(())=>{
                let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("screen glyphs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view,depth_slice:None,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});if let Err(error)=lane.draw(&mut pass) {*self.text_error.borrow_mut()=Some(error.to_string());}
            }
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.queue.submit([encoder.finish()]);
    }
}

fn style(scene:&Scene,id:ObjectId,selected:bool)->crate::gpu_mesh::Style {crate::gpu_mesh::Style {selected,pickable:scene.pick_id(id.value()).is_some(),colour:scene.view.faces.get(&id).copied()}}

fn rgba(rgb:[u8;3])->[f32;4] {[rgb[0] as f32/255.,rgb[1] as f32/255.,rgb[2] as f32/255.,1.]}
```

## <a id="checkpoint-61d-glyphs"></a>Step 5 of 5 · Draw the specimen and prove real coverage reuse (`61d-glyphs`)

<div class="webgpu-schematic" style="width:100%;margin:8px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="76" viewBox="0 0 1120 76" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="72" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5/5: 61d-glyphs</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/commands.rs, src/browse…</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Example Text</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged shaping</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">unchanged original scene</text>
</svg>
</div>

Connect Example Text through the command dock and copy actual texture readback checks. Test DPI, clipping, presentation reuse, accepted font reset, overlay picking and Close cleanup.

> **Check:** *Does Glyphon 0.11 retain each uploaded raster in the CPU image cache?* — No. It rasterises temporary images for atlas upload. Actual CPU image-cache counts remain zero here; the checks prove rendered pixels, atlas reuse and font-revision reset without inventing retained CPU bytes.

### 1. `src/commands.rs`

Connect the presentation specimen, current text diagnostics and explicit Close cleanup.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Report", Host::Report),
    ("Report Previous", Host::PreviousReport),
    ("Selection", Host::Selection),
```

Replace it with:

```rust
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Example Text",Host::TextDemo),
    ("Report", Host::Report),
    ("Report Previous", Host::PreviousReport),
    ("Selection", Host::Selection),
```

**Locate this block:**

```rust
use crate::editor::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host { Help, Open, Replace, CancelOpen, Reload, CancelReload, Save, Report, PreviousReport, Selection, LayersOn, LayersOff, LayersAll }

pub enum Command { Editor(Action), Host(Host) }
```

Replace it with:

```rust
use crate::editor::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host { TextDemo, Help, Open, Replace, CancelOpen, Reload, CancelReload, Save, Report, PreviousReport, Selection, LayersOn, LayersOff, LayersAll }

pub enum Command { Editor(Action), Host(Host) }
```

**Complete updated block reference:**

```rust
use crate::editor::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Host { TextDemo, Help, Open, Replace, CancelOpen, Reload, CancelReload, Save, Report, PreviousReport, Selection, LayersOn, LayersOff, LayersAll }

pub enum Command { Editor(Action), Host(Host) }

pub type Factory = fn() -> Action;
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Snap", || Action::Snap(crate::snap::Setting::Objects(None))),
    ("Snap On", || Action::Snap(crate::snap::Setting::Objects(Some(true)))),
    ("Snap Off", || Action::Snap(crate::snap::Setting::Objects(Some(false)))),
    ("Grid Snap", || Action::Snap(crate::snap::Setting::Grid(None))),
    ("Grid Snap On", || Action::Snap(crate::snap::Setting::Grid(Some(true)))),
    ("Grid Snap Off", || Action::Snap(crate::snap::Setting::Grid(Some(false)))),
    ("Snap End", || Action::Snap(crate::snap::Setting::Mode(crate::snap::END))),
    ("Snap Near", || Action::Snap(crate::snap::Setting::Mode(crate::snap::NEAR))),
    ("Snap Mid", || Action::Snap(crate::snap::Setting::Mode(crate::snap::MID))),
    ("Snap Center", || Action::Snap(crate::snap::Setting::Mode(crate::snap::CENTER))),
    ("Snap Perp", || Action::Snap(crate::snap::Setting::Mode(crate::snap::PERP))),
    ("Move", || Action::BeginTool(crate::tool::Shape::Move)),
    ("Point", || Action::BeginTool(crate::tool::Shape::Point)),
    ("Line", || Action::BeginTool(crate::tool::Shape::Line)),
    ("Hide",||Action::Hide),
    ("Show",||Action::Show),
    ("Example Box", || Action::AddBox),
    ("Example CAD",||Action::AddCad),
    ("Example Surface",||Action::AddSurface),
    ("Example Nested",||Action::AddNested),
    ("Example Line", || Action::AddLine),
    ("Example Point", || Action::AddPoint),
    ("Example Curve", || Action::AddCurve),
    ("Example Object Color", || Action::AddColours(crate::colour::Mode::Object)),
    ("Example Point Colors", || Action::AddColours(crate::colour::Mode::Points)),
    ("Example Face Colors", || Action::AddColours(crate::colour::Mode::Faces)),
    ("View Normals On", || Action::NormalView(true)),
    ("View Normals Off", || Action::NormalView(false)),
    ("View Grid On", || Action::Grid(Some(crate::grid::Settings::default()))),
    ("View Grid Off", || Action::Grid(None)),
    ("Example Sharp", || Action::AddNormals(crate::normal_example::Mode::Sharp)),
    ("Example Smooth", || Action::AddNormals(crate::normal_example::Mode::Smooth)),
    ("Example Winding", || Action::AddNormals(crate::normal_example::Mode::Winding)),
    ("Controls On", || Action::Controls(true)),
    ("Controls Off", || Action::Controls(false)),
    ("Example Polyline", || Action::AddPolyline),
    ("Example Arrow", || Action::AddArrow),
    ("Example Triangle", || Action::ToggleExtra),
    ("Select Next", || Action::SelectNext),
    ("Select All", || Action::SelectAll),
    ("Select None", || Action::Picked(None)),
    ("Select Object", || Action::PickMode(crate::pick_sources::Mode::Object)),
    ("Select Face", || Action::PickMode(crate::pick_sources::Mode::Face)),
    ("Select Edge", || Action::PickMode(crate::pick_sources::Mode::Edge)),
    ("Select Control", || Action::PickMode(crate::pick_sources::Mode::Control)),
    ("Delete", || Action::Delete),
    ("Undo", || Action::Undo),
    ("Redo", || Action::Redo),
    ("Background", || Action::Background),
    ("Zoom In", || Action::Zoom(2.0)),
    ("Zoom Out", || Action::Zoom(0.5)),
    ("Pan Left", || Action::Pan(-0.25, 0.0)),
    ("Pan Right", || Action::Pan(0.25, 0.0)),
    ("Orbit Right", || Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0)),
    ("Orbit Up", || Action::Orbit(0.0, std::f64::consts::FRAC_PI_6)),
    ("View Isometric", || Action::Isometric),
    ("Fit", || Action::Fit),
    ("Fit Selected", || Action::FitSelected),
    ("View Reset", || Action::ResetView),
    ("View Perspective", || Action::Projection(crate::camera::Projection::Perspective)),
    ("View Orthographic", || Action::Projection(crate::camera::Projection::Orthographic)),
    ("Close", || Action::Close),
    ("Unload Sources", || Action::UnloadSources),
];

pub const HOSTS: &[(&str, Host)] = &[
    ("Layers On",Host::LayersOn),("Layers Off",Host::LayersOff),("Layers All",Host::LayersAll),
    ("Help", Host::Help),
    ("Open", Host::Open),
    ("Open Replace", Host::Replace),
    ("Cancel Open", Host::CancelOpen),
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Example Text",Host::TextDemo),
    ("Report", Host::Report),
    ("Report Previous", Host::PreviousReport),
    ("Selection", Host::Selection),
];

pub fn names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).chain(ACTIONS.iter().map(|(name, _)| *name)).collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names().into_iter().filter_map(|name| {
        let compact = name.split_whitespace().collect::<String>().to_ascii_lowercase();
        let mut rest = compact.as_str();
        for (index, word) in words.iter().enumerate() {
            let word = word.to_ascii_lowercase();
            rest = rest.strip_prefix(&word)?;
            if rest.is_empty() { return Some((name, index + 1, compact.len())); }
        }
        None
    }).max_by_key(|(_, _, letters)| *letters).map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(|| line.trim().to_owned(), |(name, count)| {
        std::iter::once(name).chain(words[count..].iter().copied()).collect::<Vec<_>>().join(" ")
    })
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name == "Move" && words.len()>count {
        return crate::offset::parse(&format!("Move {}", words[count..].join(" ")))
            .map(|offset| Command::Editor(Action::Translate(offset))).map_err(str::to_owned);
    }
    if name == "Grid Snap" && words.len() > count {
        if words.len() != count+1 { return Err("Grid Snap (On Off) or a spacing".into()); }
        let step = words[count].parse::<f64>().map_err(|_| "Grid Snap (On Off) or a spacing")?;
        if !step.is_finite() || step<=0. { return Err("The grid spacing must be above zero · Example: Grid Snap 50".into()); }
        return Ok(Command::Editor(Action::Snap(crate::snap::Setting::Spacing(step))));
    }
    if words.len() != count { return Err(format!("{name} takes no arguments")); }
    if let Some((_, make)) = ACTIONS.iter().find(|(key, _)| *key == name) { return Ok(Command::Editor(make())); }
    let (_, host) = HOSTS.iter().find(|(key, _)| *key == name).ok_or("Command registry is incomplete")?;
    Ok(Command::Host(*host))
}
```

### 2. `src/browser.rs`

Connect the presentation specimen, current text diagnostics and explicit Close cleanup.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-cads",&serde_json::to_string(&cads).unwrap())?;
```

Replace it with:

```rust
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-cads",&serde_json::to_string(&cads).unwrap())?;
```

**Locate this block:**

```rust
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { request.borrow_mut().cancel(); }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
            Ok(crate::commands::Command::Host(host)) => {
            use crate::commands::Host;
            match host {
            Host::Help => { panel.result(&crate::commands::names().join(" · ")); None }
            Host::Selection => {
                let rows = editor.selected_objects();
```

Replace it with:

```rust
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { request.borrow_mut().cancel();let _=renderer.set_text(Vec::new()); }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
            Ok(crate::commands::Command::Host(host)) => {
            use crate::commands::Host;
            match host {
            Host::TextDemo=>{if let Err(error)=renderer.text_demo() {panel.result(&error.to_string());}None},
            Host::Help => { panel.result(&crate::commands::names().join(" · ")); None }
            Host::Selection => {
                let rows = editor.selected_objects();
```

**Complete updated block reference:**

```rust
use crate::background::Background;
use crate::editor::{Action, Change, Editor};
use crate::gesture::Gesture;
use crate::renderer::Renderer;
use crate::viewport::Viewport;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot draw:") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

fn failed(fault: crate::gpu_fault::Fault, message: String) {
    if !fault.remember(message.clone()) { return; }
    wasm_bindgen_futures::spawn_local(async move {
        if crate::browser_runtime::stop_if(&fault) {
            crate::browser_report::fatal(&message);
            if crate::browser_recovery::request(&message) { return; }
            report(&format!("Cannot draw: {message}"));
        }
    });
}

fn save_result(bytes: &[u8]) -> Result<String, String> {
    crate::file_output::download(bytes)
        .map(|()| format!("Saved editable document ({} bytes)", bytes.len()))
        .map_err(|error| format!("Save failed: {error:?}"))
}

pub async fn run() -> Result<(), JsValue> { run_with(crate::drawing_document::new()).await }

pub async fn run_with(shared: crate::drawing_document::Shared) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let recovery = crate::browser_recovery::adopt()?;
    crate::browser_report::start()?;
    let startup = crate::browser_startup::Guard::new()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter_started = crate::browser_phase::now();
    let probe = crate::browser_adapter::Probe::new().ok();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await;
    if !startup.permits() { return Ok(()); }
    crate::browser_phase::finish("adapter request", adapter_started, 0, "GPU");
    let adapter = adapter.map_err(|error| JsValue::from_str(&error.to_string()))?;
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await;
    if !startup.permits() {
        if let Ok((device, _)) = device { device.destroy(); }
        return Ok(());
    }
    crate::browser_phase::finish("device request", device_started, 0, "GPU");
    let (device, queue) = device.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let fault = crate::gpu_fault::Fault::default();
    let errors = fault.clone();
    device.on_uncaptured_error(std::sync::Arc::new(move |error| failed(errors.clone(), format!("WebGPU error: {error}"))));
    let lost = fault.clone();
    device.set_device_lost_callback(move |reason, message| failed(lost.clone(), format!("WebGPU device lost ({reason:?}): {message}")));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let mut editor = shared.borrow_mut();
    let pipeline_started = crate::browser_phase::now();
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
        &editor.scene,
    );
    crate::browser_phase::finish("renderer setup", pipeline_started, 0, "GPU");
    renderer.resize(Viewport { width, height });
    inspect(&editor, &renderer)?;
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        crate::commands::names(),
    );
    if let Some(message) = crate::browser_report::previous_notice() { panel.result(message); }
    if let Some(message) = &recovery { panel.result(message); }
    panel.update(None, &canvas)?;
    if resize(
        &window,
        &canvas,
        &surface,
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None, &canvas)?;
        let frame_started = crate::browser_phase::now();
        present(
            &surface,
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            editor.normal_view,
            &mut panel,
        )?;
        let completing = fault.clone();
        // Queue completion runs after submitted GPU work; checking the current device prevents a late result from reviving an exited runtime.
        renderer.queue.on_submitted_work_done(move || {
            if crate::browser_runtime::owns(&completing) && completing.message().is_none() {
                crate::browser_phase::finish("first frame complete", frame_started, 0, "GPU");
                let _ = crate::browser_report::observe("milestone", "geometry on screen");
            }
        });
    }
    let input_canvas = canvas.clone();
    let browser_window = window.clone();
    let pointer_canvas = canvas.clone();
    let mut gesture = Gesture::default();
    let mut touch=crate::touch::Touch::default();
    let pick_remove=std::cell::Cell::new(false);
    let pick_additive = std::rc::Rc::new(std::cell::Cell::new(false));
    let request = std::rc::Rc::new(std::cell::RefCell::new(crate::read_gate::ReadGate::default()));
    let read_mode = std::cell::Cell::new(crate::file_input::Mode::Append);
    let delivery: crate::file_input::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let reload: crate::browser_reload::Shared = std::rc::Rc::new(std::cell::RefCell::new(Default::default()));
    let reload_delivery: crate::browser_reload::Delivery = std::rc::Rc::new(std::cell::RefCell::new(None));
    let owned_request = std::rc::Rc::clone(&request);
    let owned_reload = std::rc::Rc::clone(&reload);
    let active_fault = fault.clone();
    drop(editor);
    let owned_editor = std::rc::Rc::clone(&shared);
    let update = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if active_fault.message().is_some() { return; }
        let mut editor = shared.borrow_mut();
        if matches!(event.type_().as_str(), "blur" | "pagehide" | "viewer-reload") { renderer.cancel_pick(); }
        if event.type_() == "pagehide" {
            gesture.cancel();touch.cancel();let _=editor.apply(Action::CanvasCancel); let _=editor.apply(Action::GizmoCancel);
            crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());renderer.set_gizmo(&editor.camera,None,-1.);
            if event.dyn_ref::<web_sys::PageTransitionEvent>().is_some_and(|event| event.persisted()) { return; }
            wasm_bindgen_futures::spawn_local(async { crate::browser_runtime::stop(); });
            return;
        }
        if matches!(event.type_().as_str(),"blur"|"resize"|"pointercancel")||event.dyn_ref::<web_sys::KeyboardEvent>().is_some_and(|key|key.key()=="Escape"&&!key.is_composing()) {
            gesture.cancel();touch.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="lostpointercapture"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|editor.canvas_drag.as_ref().is_some_and(|d|d.pointer==p.pointer_id()&&d.released.is_none())) {
            gesture.cancel();renderer.cancel_pick();if editor.apply(Action::CanvasCancel)==Ok(Change::Preview) {crate::browser_upload::prepare(&mut renderer,&editor.scene,editor.whole_selection(),active_fault.clone());}
        }
        if event.type_()=="pointerdown"&&event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.is_primary())&&editor.canvas_drag.as_ref().is_none_or(|d|d.released.is_some()) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}
        let shortcut = event.dyn_ref::<web_sys::KeyboardEvent>().and_then(|key| panel.shortcut(key));
        let line = if let Some(shortcut) = shortcut {
            event.prevent_default();
            panel.answer(shortcut.command(), "");
            Some(shortcut.command().to_ascii_lowercase())
        } else { match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        } };

        let navigation=if line.is_none()&&!panel.consumed {navigation_action(&event,&pointer_canvas,&mut gesture,&mut touch)} else {None};
        let line=if gesture.take_repeat() {event.prevent_default();if editor.draft.is_some()||editor.gizmo_edit.is_some() {Some(String::new())} else {panel.repeat_line()}} else {line};
        let line = if (editor.draft.is_some() || editor.gizmo_edit.is_some()) && event.dyn_ref::<web_sys::KeyboardEvent>()
            .is_some_and(|key| event.type_() == "keydown" && key.key() == "Escape" && !key.is_composing()) {
            event.prevent_default(); gesture.cancel(); Some("Escape".into())
        } else { line };

        let mut highlight_changed = false;
        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
            request.borrow_mut().cancel();
            report("Open cancelled.");
            panel.answer("Open", "Open cancelled.");
            None
        } else if event.type_() == "change" {
            crate::file_input::choose(&event, std::rc::Rc::clone(&request), read_mode.get(), std::rc::Rc::clone(&delivery));
            None
        } else if event.type_() == "viewer-file-error" {
            if let Some(message) = event.dyn_ref::<web_sys::CustomEvent>().and_then(|event| event.detail().as_string()) {
                report(&message);
                panel.answer("Open", &message);
            }
            None
        } else if event.type_() == "viewer-reload" {
            if let Some(reply) = reload_delivery.borrow_mut().take() {
                let command = reply.intent.map_or("Reload Sources", |intent| intent.command());
                match reply.complete(&mut editor) {
                    Ok(Some(crate::edit_replay::Reply::Changed(_))) => {
                        highlight_changed = true;
                        crate::browser_upload::prepare(&mut renderer, editor.display_scene(), editor.whole_selection(), active_fault.clone());
                        crate::browser_phase::revision(&editor.scene);
                        let message = if command == "Reload Sources" {
                            "Editable sources restored; display retained.".to_owned()
                        } else { format!("{command} completed after source reload.") };
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(Some(crate::edit_replay::Reply::Saved(bytes))) => {
                        let message = save_result(&bytes).unwrap_or_else(|error| error);
                        report(&message); panel.answer(command, &message);
                    }
                    Ok(None) => {}
                    Err(error) => { report(&error); panel.answer(command, &error); }
                }
            }
            None
        } else if event.type_() == "viewer-file" {
            match crate::file_input::action(&event, &delivery) {
                Ok(action) => action,
                Err(error) => { let message = format!("Cannot retain reload file: {error:?}");
                    report(&message); panel.answer("Open", &message); None }
            }
        } else if let Some(line) = line {
            let command = editor.tool_line(&line).map(|result| result.map(crate::commands::Command::Editor).map_err(str::to_owned))
                .unwrap_or_else(|| crate::commands::parse(&line));
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { request.borrow_mut().cancel();let _=renderer.set_text(Vec::new()); }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
            Ok(crate::commands::Command::Host(host)) => {
            use crate::commands::Host;
            match host {
            Host::TextDemo=>{if let Err(error)=renderer.text_demo() {panel.result(&error.to_string());}None},
            Host::Help => { panel.result(&crate::commands::names().join(" · ")); None }
            Host::Selection => {
                let rows = editor.selected_objects();
                panel.result(&if rows.len() > 1 { format!("Objects {:?}", rows.iter().map(|id| id.value()).collect::<Vec<_>>()) }
                    else { editor.selection().map_or_else(|| "Nothing selected".to_owned(), |target| target.describe()) });
                None
            }
            Host::Reload => {
                let result = crate::browser_reload::start(std::rc::Rc::clone(&reload), editor.reload_keys(), std::rc::Rc::clone(&reload_delivery));
                match result {
                    Ok(()) => panel.result("Reloading editable sources…"),
                    Err(error) => { report(&error); panel.result(&error); }
                }
                None
            }
            Host::CancelReload => {
                reload.borrow_mut().cancel();
                report("Reload cancelled."); panel.result("Reload cancelled.");
                None
            }
            Host::CancelOpen => {
                request.borrow_mut().cancel();
                report("Open cancelled.");
                panel.result("Open cancelled.");
                None
            }
            Host::Open | Host::Replace => {
                let ticket = request.borrow_mut().begin();
                if let Err(error) = ticket { report(error); panel.result(error); return; }
                if host == Host::Replace { reload.borrow_mut().cancel(); }
                read_mode.set(if host == Host::Replace { crate::file_input::Mode::Replace }
                    else { crate::file_input::Mode::Append });
                if let Some(input) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("open"))
                    .and_then(|element| element.dyn_into::<web_sys::HtmlInputElement>().ok())
                {
                    input.click();
                }
                None
            }
            Host::Report | Host::PreviousReport => {
                let output = if host == Host::PreviousReport { crate::browser_report::download_previous() }
                    else { crate::browser_report::download() };
                let result = output.map(|()| "Diagnostic report downloaded.".to_owned())
                    .unwrap_or_else(|error| format!("Diagnostic report failed: {error:?}"));
                panel.result(&result);
                None
            }
            Host::LayersOn=>{panel.tree.open=true;None},
            Host::LayersOff=>{panel.tree.open=false;None},
            Host::LayersAll=>{panel.tree.all();None},
            Host::Save => {
                let result = crate::browser_reload::restore_for(crate::edit_intent::Intent::Save,
                    &editor, std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery))
                    .and_then(|waiting| {
                        if waiting { Ok("Reloading editable sources…".to_owned()) }
                        else { crate::document::snapshot(&editor.scene).map_err(str::to_owned)
                            .and_then(|bytes| save_result(&bytes)) }
                    });
                panel.result(&result.unwrap_or_else(|error| error));
                None
            }
            }
            }
            }
        } else if !panel.consumed {
            navigation
        } else {
            None
        };
        let handle_action=gizmo_action(&event,&pointer_canvas,&editor,panel.consumed);
        let action = if let Some((key,edit))=panel.tree.edit.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerEdit {key,edit})
        } else if let Some((key,property))=panel.tree.property.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::LayerProperty {key,property})
        } else if let Some((ids,additive))=panel.tree.action.take() {
            renderer.cancel_pick();gesture.cancel();Some(Action::TreePick {ids,additive})
        } else if let Some(handle_action)=handle_action {
            renderer.cancel_pick();gesture.cancel();Some(handle_action)
        } else if event.type_() == "viewer-pick" {
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => {
                    let target=hit.and_then(|hit|renderer.resolve_pick(&editor.scene,hit));
                    if editor.canvas_drag.is_some() {Some(Action::CanvasHit(target.map(|t|t.owner)))}
                    else if pick_remove.get() {Some(Action::SelectRows {ids:target.filter(|t|t.part==crate::pick_sources::Part::Object).map(|t|t.owner).into_iter().collect(),add:false,remove:true})}
                    else {Some(Action::PickedMany {target,additive:pick_additive.get()})}
                },
                Some(Err(error)) => { report(error); None },
                None => None,
            }
        } else { match action {
            Some(Action::Pick(screen)) if editor.draft.is_some() => {
                Some(Action::ToolScreen { screen,size:[pointer_canvas.get_bounding_client_rect().width(),pointer_canvas.get_bounding_client_rect().height()],commit:true })
            }
            None if editor.draft.is_some() && matches!(event.type_().as_str(), "blur" | "resize" | "pointercancel" | "lostpointercapture") => Some(Action::ToolHover(None)),
            None if editor.draft.is_some() && !panel.consumed && event.type_() == "pointermove" => {
                event.dyn_ref::<web_sys::PointerEvent>().filter(|p| p.buttons() == 0).map(|p| {
                    let r = pointer_canvas.get_bounding_client_rect();
                    let screen = [(2. * (p.client_x() as f64-r.left())/r.width()-1.) as f32, (1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32];
                    Action::ToolScreen { screen,size:[r.width(),r.height()],commit:false }
                })
            }
            Some(Action::Touch(actions))=> {
                let mut applied=Vec::new();
                for action in actions {match action {
                    Action::Pick(screen)=>{if editor.draft.is_some() {let r=pointer_canvas.get_bounding_client_rect();applied.push(Action::ToolScreen {screen,size:[r.width(),r.height()],commit:true});} else {pick_additive.set(false);pick_remove.set(false);if let Err(error)=renderer.request_pick(screen,editor.camera.uniform()) {report(error);}}},
                    action=>{renderer.cancel_pick();applied.push(action);}
                }}Some(Action::Touch(applied))
            },
            Some(Action::CanvasStart {from,to,size,modified,pointer}) if editor.draft.is_none()=> {
                renderer.cancel_pick();
                let modified=modified||editor.pick_mode!=crate::pick_sources::Mode::Object;
                let start=Action::CanvasStart {from,to,size,modified,pointer};
                let release=event.dyn_ref::<web_sys::PointerEvent>().filter(|_|event.type_()=="pointerup").map(|pointer|Action::CanvasRelease {screen:to,size,add:pointer.shift_key(),remove:pointer.ctrl_key()||pointer.meta_key()});
                if !modified {if let Err(error)=renderer.request_pick(from,editor.camera.uniform()) {report(error);return;}}
                Some(Action::Touch(std::iter::once(start).chain(release).collect()))
            },
            Some(Action::CanvasStart {..}|Action::CanvasMove {..}|Action::CanvasRelease {..}) if editor.draft.is_some()=>None,
            Some(action @ (Action::CanvasMove {..}|Action::CanvasRelease {..}))=>Some(action),
            Some(Action::Pick(screen)) => {
                pick_remove.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p|p.ctrl_key()||p.meta_key()));
                pick_additive.set(event.dyn_ref::<web_sys::PointerEvent>().is_some_and(|p| p.shift_key() && !p.ctrl_key() && !p.meta_key()));
                if let Err(error) = renderer.request_pick(screen, editor.camera.uniform()) { report(error); }
                None
            }
            Some(action) => { renderer.cancel_pick(); Some(action) },
            None => None,
        } };
        if let Some(action) = action {
            highlight_changed |= matches!(&action, Action::SelectAll);
            let rename_action=matches!(&action,Action::LayerEdit {edit:crate::layer_edit::Edit::Rename(_),..});
            let tool_action = matches!(&action, Action::BeginTool(_) | Action::ToolPoint(_) | Action::CancelTool | Action::Snap(_) | Action::GizmoRelease {..} | Action::GizmoCancel | Action::GizmoNumber(_) | Action::ToolScreen { commit:true,.. });
            if matches!(&action, Action::CancelTool) { panel.cancel_input(); }
            if matches!(&action, Action::Close | Action::UnloadSources | Action::Undo | Action::Redo
                | Action::Replace(_) | Action::ReplaceAt(_, _)) { reload.borrow_mut().cancel(); }
            let replacement = matches!(&action, Action::Replace(_) | Action::ReplaceAt(_, _));
            let closing = matches!(&action, Action::Close);
            let unloading = matches!(&action, Action::UnloadSources);
            let waiting = if editor.selected_objects().len() > 1 { Ok(None) }
                else { crate::edit_intent::Intent::capture(&action, editor.whole_selection()) }
                .map_err(str::to_owned).and_then(|intent| match intent {
                    Some(intent) => crate::browser_reload::restore_for(intent, &editor,
                        std::rc::Rc::clone(&reload), std::rc::Rc::clone(&reload_delivery)),
                    None => Ok(false),
                });
            let change = waiting.and_then(|waiting| {
                if waiting { panel.result("Reloading editable sources…"); Ok(None) }
                else { editor.apply(action).map(Some).map_err(str::to_owned) }
            });
            if rename_action {panel.tree.rename_result(change.as_ref().err().map(String::as_str));}
            if tool_action && change.is_ok() { panel.result(&editor.tool_status); }
            match change {
                Ok(Some(Change::Scene)) => {
                    highlight_changed = true;
                    crate::browser_upload::prepare(&mut renderer, &editor.scene, editor.whole_selection(), active_fault.clone());
                    if unloading {
                        report("Editable sources unloaded; display retained.");
                        panel.result("Editable sources unloaded; display retained.");
                    } else if closing {
                        report("Document closed.");
                        panel.result("Document closed.");
                    } else if event.type_() == "viewer-file" {
                        crate::browser_phase::revision(&editor.scene);
                        let message = if replacement { "Document replaced. Undo restores the previous document." }
                            else { "File imported. Undo removes the entire import." };
                        report(message);
                        panel.answer(if replacement { "Open Replace" } else { "Open" }, message);
                    }
                }
                Ok(Some(Change::Preview)) => {
                    highlight_changed=true;
                    crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());
                }
                Ok(Some(Change::View)) | Ok(None) => {}
                Err(error) => {
                    report(&error);
                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }
                    else { panel.result(&error); }
                }
            }
        }
        if let Some(draft) = editor.draft.as_ref() { panel.tool(draft.prompt(), draft.tool.buttons()); }
        else { panel.tool(String::new(), Vec::new()); }
        let rect=pointer_canvas.get_bounding_client_rect();
        let number=editor.number_prompt([rect.width(),rect.height()]);
        if editor.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some()) && number.is_none() {
            if editor.apply(Action::GizmoCancel).is_ok() {highlight_changed=true;crate::browser_upload::prepare(&mut renderer,editor.display_scene(),editor.whole_selection(),active_fault.clone());}
        }
        panel.rectangle=editor.canvas_drag.as_ref().filter(|d|d.missed).and_then(|d|d.rectangle());
        panel.number(number);
        panel.tree.refresh(&editor.scene,editor.selected_objects());
        if let Err(error) = renderer.set_preview(editor.draft.as_ref()) { panel.result(error); }
        renderer.set_pick_mode(editor.pick_mode);
        if highlight_changed {
            if let Err(error) = renderer.set_highlight(editor.display_scene(), editor.selection())
                .and_then(|()| renderer.select_objects(editor.display_scene(), &editor.selected_objects())) { report(error); return; }
        }
        if let Err(error) = renderer.set_grid(editor.camera.grid) {
            report(error); return;
        }
        if resize(
            &browser_window,
            &pointer_canvas,
            &surface,
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            if let Some(frame)=panel.free_frame([rect.width(),rect.height()]) {if editor.camera.frame(frame) {renderer.cancel_pick();let _=editor.apply(Action::CanvasCancel);}}
            let hover=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|event.type_()=="pointermove" && p.buttons()==0 && !panel.consumed).map(|p| {
                let r=pointer_canvas.get_bounding_client_rect();
                ([(2.*(p.client_x() as f64-r.left())/r.width()-1.) as f32,(1.-2.*(p.client_y() as f64-r.top())/r.height()) as f32],[r.width(),r.height()])
            });
            editor.hover_gizmo(hover);
            renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
            if let Err(error) = inspect(&editor, &renderer) {
            report(&format!("Cannot report projection: {error:?}"));
            return;
        }
            if let Err(error) = present(
                &surface,
                &renderer,
                &editor.background,
                &editor.camera.uniform(),
                editor.normal_view,
                &mut panel,
            ) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    let mut listeners = crate::listeners::Listeners::new(update);
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        listeners.listen(canvas.as_ref(), name)?;
    }
    listeners.listen(canvas.as_ref(), "wheel")?;
    listeners.listen(window.as_ref(), "resize")?;
    listeners.listen(window.as_ref(), "blur")?;
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-file")?;
    listeners.listen(window.as_ref(), "viewer-file-error")?;
    listeners.listen(window.as_ref(), "viewer-reload")?;
    listeners.listen(window.as_ref(), "pagehide")?;
    crate::browser_runtime::install(listeners, owned_request, owned_reload, fault, owned_editor);
    report(&recovery.unwrap_or_else(|| "Wheel zooms; type commands anywhere in the drawing. Escape cancels a drag.".into()));
    Ok(())
}

fn inspect(editor: &Editor, renderer: &Renderer) -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or("No document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    let view_ids=editor.scene.objects().iter().map(|row|row.id).chain(editor.scene.lines().iter().map(|row|row.id)).chain(editor.scene.paths().iter().map(|row|row.id)).chain(editor.scene.points().iter().map(|row|row.id));
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
    canvas.set_attribute("data-view-flags",&serde_json::json!({"current":crate::layer_edit::current_key(&editor.scene),"rows":view_rows}).to_string())?;
    canvas.set_attribute("data-tree",&serde_json::json!(crate::hierarchy::rows(&editor.scene).iter().map(|r|serde_json::json!({"key":r.key,"label":r.label,"depth":r.depth,"end":r.end,"readonly":r.readonly,"ids":r.ids.iter().map(|id|id.value()).collect::<Vec<_>>()})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-cads",&serde_json::to_string(&cads).unwrap())?;
    canvas.set_attribute("data-surface-cpu",&serde_json::to_string(&crate::memory::surfaces(editor.presentation_scenes())).unwrap())?;
    let surfaces=editor.scene.objects().iter().filter_map(|row|row.surface.as_ref().map(|s|serde_json::json!({"id":row.id.value(),"source":s.source.guid(),"controls":s.source.m_cv_count,"steps":s.grid.steps,"samples":s.grid.samples.len(),"faces":row.mesh.parts.faces,"edges":s.grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>()}))).collect::<Vec<_>>();
    canvas.set_attribute("data-surfaces",&serde_json::to_string(&surfaces).unwrap())?;
    canvas.set_attribute("data-curve-cpu", &serde_json::to_string(&crate::memory::curves(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-control-count", &editor.scene.control_markers().len().to_string())?;
    canvas.set_attribute("data-point-cpu", &serde_json::to_string(&crate::memory::points(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-path-gpu", &serde_json::to_string(&renderer.path_usage()).unwrap())?;
    canvas.set_attribute("data-path-cpu", &serde_json::to_string(&crate::memory::paths(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-stroke-usage", &serde_json::to_string(&renderer.stroke_usage()).unwrap())?;
    canvas.set_attribute("data-line-usage", &serde_json::to_string(&crate::memory::lines(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-edit-preview", &editor.edit_preview.as_ref().map_or_else(|| "null".into(),|p| serde_json::json!({"ids":p.ids().iter().map(|id| id.value()).collect::<Vec<_>>(),"matrix":p.matrix()}).to_string()))?;
    canvas.set_attribute("data-tool", &editor.draft.as_ref().map_or_else(|| "null".into(), |d| serde_json::json!({"name":d.tool.name(),"points":d.points,"cursor":d.cursor,"snap":d.snapped.map(|kind| format!("{kind:?}"))}).to_string()))?;
    canvas.set_attribute("data-snap-settings", &serde_json::json!({"objects":editor.snapping.objects,"modes":editor.snapping.modes,"grid":editor.snapping.grid,"step":editor.snapping.step}).to_string())?;
    canvas.set_attribute("data-snap-usage", &serde_json::to_string(&editor.snap_usage()).unwrap())?;
    canvas.set_attribute("data-preview-bytes", &renderer.preview_bytes().to_string())?;
    canvas.set_attribute("data-highlight-bytes", &renderer.highlight_bytes().to_string())?;
    canvas.set_attribute("data-selection-part", &editor.selection().map_or_else(String::new, |target| target.describe()))?;
    canvas.set_attribute("data-pick-mode", &format!("{:?}", editor.pick_mode))?;
    canvas.set_attribute("data-part-usage", &serde_json::to_string(&renderer.part_usage()).unwrap())?;
    canvas.set_attribute("data-selected-ids", &serde_json::to_string(&editor.selected_objects().iter().map(|id| id.value()).collect::<Vec<_>>()).unwrap())?;
    canvas.set_attribute("data-selected-id", &editor.selected.map(|id| format!("{id:?}")).unwrap_or_default())?;
    canvas.set_attribute("data-grid-enabled", if editor.camera.grid.is_some() { "true" } else { "false" })?;
    canvas.set_attribute("data-grid-gpu", &serde_json::to_string(&renderer.grid_usage()).unwrap())?;
    canvas.set_attribute("data-normal-view", if editor.normal_view { "true" } else { "false" })?;
    canvas.set_attribute("data-camera-matrix", &serde_json::to_string(&editor.camera.view_projection().m).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-object-count", &editor.scene.objects().len().to_string())?;
    let metadata: Vec<_> = editor.scene.objects().iter().map(|row| {
        let m = &row.metadata;
        (&row.guid, &m.source_guid, &m.name, m.is_visible, m.is_locked)
    }).collect();
    let origins: Vec<_> = editor.scene.objects().iter().filter_map(|row| row.origin().map(|origin|
        (&row.guid, origin.id.to_string(), origin.version.hex(),
            origin.location.as_ref().map(|location| location.value())))).collect();
    let residency: Vec<_> = editor.scene.objects().iter().map(|row|
        (&row.guid, row.geometry().is_some(), row.release_epoch())).collect();
    canvas.set_attribute("data-source-residency", &serde_json::to_string(&residency)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-source-origins", &serde_json::to_string(&origins)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-row-metadata", &serde_json::to_string(&metadata)
        .map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-gpu-stats", &format!("{:?}", renderer.stats()))?;
    let cpu = crate::memory::cpu(editor.presentation_scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    canvas.set_attribute("data-cpu-usage", &format!("{cpu:?}"))?;
    canvas.set_attribute("data-gpu-usage", &format!("{:?}", renderer.usage()))?;
    canvas.set_attribute("data-projection", &format!("{:?}", editor.camera.projection))
}

fn navigation_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
    touch:&mut crate::touch::Touch,
) -> Option<Action> {
    let action = match event.type_().as_str() {
        "wheel" => {
            let wheel = event.dyn_ref::<web_sys::WheelEvent>()?;
            let mode = wheel.delta_mode();
            let rect=canvas.get_bounding_client_rect();
            let Action::Zoom(factor)=crate::navigation::wheel(wheel.delta_y(),mode,rect.height())? else {return None;};
            let screen=[(2.*(wheel.client_x() as f64-rect.left())/rect.width()-1.) as f32,(1.-2.*(wheel.client_y() as f64-rect.top())/rect.height()) as f32];
            Some(Action::ZoomAt {factor,screen})
        }
        "keydown" => {
            let key = event.dyn_ref::<web_sys::KeyboardEvent>()?;
            if key.is_composing() || key.alt_key() {
                return None;
            }
            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return None;
            }
            return None;
        }
        _=>{
            if let Some(pointer)=event.dyn_ref::<web_sys::PointerEvent>().filter(|p|p.pointer_type()=="touch") {
                event.prevent_default();let r=canvas.get_bounding_client_rect();let rect=[r.left(),r.top(),r.width(),r.height()];let at=[pointer.client_x() as f64,pointer.client_y() as f64];let id=pointer.pointer_id();
                let actions=match event.type_().as_str() {
                    "pointerdown"=>{gesture.cancel();let multiple=touch.count()>0;if touch.press(id,at,event.time_stamp()) {if canvas.set_pointer_capture(id).is_err() {touch.cancel();}}if multiple {vec![Action::GizmoCancel,Action::CanvasCancel]} else {Vec::new()}},
                    "pointermove"=>touch.move_to(id,at,rect),
                    "pointerup"=>{let actions=touch.release(id,at,event.time_stamp(),rect);let _=canvas.release_pointer_capture(id);actions},
                    "pointercancel"=>{touch.cancel();Vec::new()},"lostpointercapture"=>{if touch.contains(id) {touch.cancel();}Vec::new()},_=>Vec::new(),
                };return (!actions.is_empty()).then_some(Action::Touch(actions));
            }
            return pointer_action(event,canvas,gesture);
        },
    }?;
    event.prevent_default();
    Some(action)
}

fn pointer_action(
    event: &web_sys::Event,
    canvas: &web_sys::HtmlCanvasElement,
    gesture: &mut Gesture,
) -> Option<Action> {
    match event.type_().as_str() {
        "contextmenu" => {
            event.prevent_default();
            return None;
        }
        "blur" | "resize" => {
            gesture.cancel();
            return None;
        }
        _ => {}
    }
    let pointer = event.dyn_ref::<web_sys::PointerEvent>()?;
    let id = pointer.pointer_id();
    let position = [pointer.client_x() as f64, pointer.client_y() as f64];
    let motion = match event.type_().as_str() {
        "pointerdown" => {
            if pointer.is_primary() && gesture.press_with(id, pointer.button(), position,pointer.ctrl_key()||pointer.meta_key()) {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
            motion
        }
        "pointercancel" | "lostpointercapture" => {
            gesture.cancel_pointer(id);
            None
        }
        _ => None,
    }?;
    let rect = canvas.get_bounding_client_rect();
    let mut action=motion.action([rect.left(),rect.top(),rect.width(),rect.height()])?;
    match &mut action {
        Action::CanvasStart {modified,pointer:id,..}=>{*id=pointer.pointer_id();*modified=pointer.shift_key()||pointer.ctrl_key()||pointer.meta_key();},
        Action::CanvasRelease {add,remove,..}=>{*add=pointer.shift_key();*remove=pointer.ctrl_key()||pointer.meta_key();},_=>{}
    }Some(action)
}

fn resize(
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    surface: &wgpu::Surface<'_>,
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
    let density = crate::browser_recovery::density(window.device_pixel_ratio());
    renderer.set_density(density as f32);
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        density,
        limit,
    ) else {
        return false;
    };
    if size.width != config.width || size.height != config.height {
        canvas.set_width(size.width);
        canvas.set_height(size.height);
        config.width = size.width;
        config.height = size.height;
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    normals: bool,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw_mode(&view, background, transform, normals);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}

fn gizmo_action(event:&web_sys::Event,canvas:&web_sys::HtmlCanvasElement,editor:&Editor,consumed:bool)->Option<Action> {
    if matches!(event.type_().as_str(),"blur"|"resize") && editor.gizmo_edit.is_some() {return Some(Action::GizmoCancel);}
    let pointer=event.dyn_ref::<web_sys::PointerEvent>()?;
    if matches!(event.type_().as_str(),"pointercancel"|"lostpointercapture") {
        return editor.gizmo_edit.as_ref().filter(|e|e.pointer==Some(pointer.pointer_id())).map(|_|Action::GizmoCancel);
    }
    let rect=canvas.get_bounding_client_rect();let size=[rect.width(),rect.height()];
    if size.iter().any(|v| !v.is_finite() || *v<=0.) {return None;}
    let screen=[(2.*(pointer.client_x() as f64-rect.left())/size[0]-1.) as f32,(1.-2.*(pointer.client_y() as f64-rect.top())/size[1]) as f32];
    let id=pointer.pointer_id();
    if editor.gizmo_edit.as_ref().is_some_and(|e|e.pointer==Some(id)) {
        event.prevent_default();
        return match event.type_().as_str() {
            "pointermove"=>Some(Action::GizmoMove {pointer:id,screen,size,shift:pointer.shift_key()}),
            "pointerup"=>{let _=canvas.release_pointer_capture(id);Some(Action::GizmoRelease {pointer:id,screen,size,shift:pointer.shift_key()})},
            _=>None,
        };
    }
    if consumed || event.type_()!="pointerdown" || !pointer.is_primary() || pointer.button()!=0 {return None;}
    if pointer.shift_key() || pointer.ctrl_key() || pointer.meta_key() {return None;}
    let finger=pointer.pointer_type()=="touch";
    if editor.gizmo_hit_radius(screen,size,if finger {18.} else {8.}).is_some() {
        event.prevent_default();
        if canvas.set_pointer_capture(id).is_err() {return Some(Action::GizmoCancel);}
        Some(if finger {Action::GizmoTouch {pointer:id,screen,size}} else {Action::GizmoPress {pointer:id,screen,size,modified:false}})
    } else if editor.gizmo_edit.is_some() {Some(Action::GizmoCancel)} else {None}
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/text_atlas_tests.rs`

Copy actual anti-aliased coverage, CSS density, clipping, placement reuse and accepted/failed font replacement GPU checks.

Copy this supplied file:

```text
--8<-- "journey/code/61d-glyphs-06.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Register the source component.

After that block, append this supplied code:

```text
--8<-- "journey/code/61d-glyphs-07.rs" skip=23
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/pick_gpu_tests.rs`

Copy the actual scene-text overlay readback, source/GPU reuse and absence from scene picking.

After that block, append this supplied code:

```text
--8<-- "journey/code/61d-glyphs-08.rs" skip=153
```

**Result after `61d-glyphs`:** Type Close, Example CAD, Fit and View Isometric, then Example Text. The real Noto specimen appears at the top left. Orbit and zoom: its CSS size stays fixed while the CAD scene moves. Type Close to release the specimen. This presentation text remains outside scene picking and document history.

## Build, Verify & Visual Result

![Recorded visual output for Lesson 10.3 (61d-glyphs)](../../screenshots/journey/61d-glyphs.png)

```sh
npm --prefix ../session_tests run course -- dependencies 10.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 10.3
npm --prefix ../session_tests run course -- save 10.3
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../recovery.md).

Continue with [10.4 · Original world labels, plates and integer picks](62-source.md).
