# 10.2 · Validated shaping and revision-based reuse

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Checkpoints:** `60ba-validation` → `60g-text-shaping` (7 steps) · **Production `session_viewer/src` target:** [engine/text.rs](../../../src/engine/text.rs) · [engine/gpu/text.rs](../../../src/engine/gpu/text.rs) · [engine/gpu/text_plane.rs](../../../src/engine/gpu/text_plane.rs) · [engine/gpu/text_plate.rs](../../../src/engine/gpu/text_plate.rs) · [app/scene_text.rs](../../../src/app/scene_text.rs) · [app/fonts.rs](../../../src/app/fonts.rs) (`src/app/fonts.rs,src/shaders/matrix.wgsl,src/engine/gpu/matrices.rs`) · **Goal:** Prove the current fonts shaping and reuse rules.

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
  <text x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 10.2 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/text.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">60ba-validation -&gt; 60g-text-shaping</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/fonts.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/matrix.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/>
  <text x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Validate a complete label…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. R8Unorm Glyph Atlas (B&amp;W)</text>
  <rect x="616" y="54" width="168" height="84" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="628" y="64" width="34" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="645" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">A</text>
  <rect x="670" y="64" width="34" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="687" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">B</text>
  <rect x="712" y="64" width="34" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="729" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">7</text>
  <text x="700" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1px Padded Shelves</text>
  <text x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Validated shaping and revis…</text>
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
  <text x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove the current fonts s…</text>
</svg>
</div>

## <a id="checkpoint-60ba-validation"></a>Step 1 of 7 · Validate a complete label replacement before changing it (`60ba-validation`)

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
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1/7: 60ba-validation</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">incoming labels</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">finite placement and metr…</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">atomic replacement</text>
</svg>
</div>

Check byte budget, unique identities, finite CSS metrics, world positions, orthonormal plane axes and ordered clips before replacing any current shaped runs.

> **Check:** *Should a bad label discard the currently visible text?* — No. Every replacement label is checked before any current run is removed. Invalid metrics, positions or clips leave the existing label set and shaping intact.

### 1. `src/text.rs`

Validate metrics, positions, plane axes, padding and clip bounds.

After that block, type:

```rust
--8<-- "journey/code/60ba-validation-01.rs" skip=62
```

## <a id="checkpoint-60bb-shaping"></a>Step 2 of 7 · Shape words into glyph clusters and baselines (`60bb-shaping`)

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
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2/7: 60bb-shaping</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">UTF-8 words</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">glyph clusters</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">positioned baselines</text>
</svg>
</div>

Use advanced shaping with the explicit Noto family, unconstrained line width and original label identity. Preserve glyph cluster byte ranges and line metrics; they are not one-character indices.

> **Check:** *Must every input character produce exactly one glyph?* — No. Ligatures can combine characters and combining marks can attach to another glyph. Cluster byte ranges retain the connection to original UTF-8 text while the shaped glyph count can differ.

### 1. `src/text.rs`

Create advanced shaped buffers and distinguish layout-changing source fields.

After that block, type:

```rust
--8<-- "journey/code/60bb-shaping-01.rs" skip=58
```

## <a id="checkpoint-60c-document"></a>Step 3 of 7 · Own the shaped runs with their font revisions (`60c-document`)

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
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3/7: 60c-document</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">explicit font set</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">document revision</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">font revision</text>
</svg>
</div>

Keep current label runs, explicit fonts, document/font revisions and shaping counters in one document. The constructor loads fonts once without shaping any empty label list.

> **Check:** *Does creating an empty text document need to shape any label?* — No. Its runs and counters start empty. Fonts are ready, and the first accepted source label creates a shaped run. Later presentation changes can keep it.

### 1. `src/text.rs`

Own explicit fonts, shaped runs and independent source/font revisions.

After that block, type:

```rust
--8<-- "journey/code/60c-document-01.rs" skip=41
```

## <a id="checkpoint-60d-reuse"></a>Step 4 of 7 · Reshape only when the source layout changes (`60d-reuse`)

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
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4/7: 60d-reuse</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">valid label replacement</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">same layout reuse</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">changed words reshape</text>
</svg>
</div>

Validate the whole incoming set, reuse runs by stable label identity and preserve a shaped buffer when only position, colour, clipping or selection changes.

> **Check:** *Should camera movement or label recolouring reshape the words?* — No. Words, font size and line height determine shaping here. Placement, colour and selection change presentation while reusing the shaped buffer.

### 1. `src/text.rs`

Reuse stable shaped buffers and measure only genuine shaping work.

After that block, type:

```rust
--8<-- "journey/code/60d-reuse-01.rs" skip=46
```

## <a id="checkpoint-60e-font-reload"></a>Step 5 of 7 · Replace usable fonts without corrupting current text (`60e-font-reload`)

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
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5/7: 60e-font-reload</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">new font bytes</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">atomic font adoption</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">run reshaping</text>
</svg>
</div>

Build a replacement font database before adopting it. Refuse unusable data without touching current fonts; valid replacement reshapes every run and advances both font and document revisions.

> **Check:** *Should a failed font download erase the existing labels?* — No. An unusable font set never replaces the current database. The current runs remain. Only an accepted usable replacement advances font revisions and reshapes the text.

### 1. `src/text.rs`

Adopt fonts only after every supplied source contains a usable face.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/60e-font-reload-01.rs"
```

## <a id="checkpoint-60f-diagnostics"></a>Step 6 of 7 · Inspect glyph origins clusters and multiline metrics (`60f-diagnostics`)

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
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6/7: 60f-diagnostics</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">shaped buffer</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">UTF-8 cluster</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">baseline and advance</text>
</svg>
</div>

Expose derived glyph identity, font identity, UTF-8 cluster range, origin, advance, offset and baseline. Clear releases shaped runs while retaining the explicit font system for future labels.

> **Check:** *Can a glyph cluster range be used as a character count?* — No. It addresses bytes in the original UTF-8 string. A multibyte character or ligature can span several bytes while producing one glyph.

### 1. `src/text.rs`

Expose actual shaped glyph metrics and release current run capacity on Clear.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
}

pub fn covers(text: &str) -> bool {
    text.is_ascii() || text.chars().all(|c| c.is_control() || bundled_glyph(c))
}
```

Replace it with:

```rust
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GlyphDiagnostic {
    pub label: u32,
    pub line: usize,
    pub cluster: [usize; 2],
    pub glyph: u16,
    pub font: String,
    pub origin: [f32; 2],
    pub advance: f32,
    pub offset: [f32; 2],
    pub baseline: f32,
    pub line_width: f32,
}

pub fn covers(text: &str) -> bool {
    text.is_ascii() || text.chars().all(|c| c.is_control() || bundled_glyph(c))
}
```

**Locate this block:**

```rust
        Ok(())
    }

}

impl Default for TextDocument {
```

Replace it with:

```rust
        Ok(())
    }

    pub fn clear(&mut self) {
        self.runs.clear();
        self.runs.shrink_to_fit();
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn diagnostics(&self) -> Vec<GlyphDiagnostic> {
        let mut out = Vec::new();

        for run in &self.runs {
            for line in run.buffer.layout_runs() {
                for glyph in line.glyphs {
                    out.push(GlyphDiagnostic {
                        label: run.label.id,
                        line: line.line_i,
                        cluster: [glyph.start, glyph.end],
                        glyph: glyph.glyph_id,
                        font: format!("{:?}", glyph.font_id),
                        origin: [glyph.x, glyph.y],
                        advance: glyph.w,
                        offset: [
                            glyph.x_offset * glyph.font_size,
                            glyph.y_offset * glyph.font_size,
                        ],
                        baseline: line.line_y,
                        line_width: line.line_w,
                    });
                }
            }
        }

        out
    }
}

impl Default for TextDocument {
```

**Complete updated block reference:**

```rust
use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap, fontdb};
use serde::Serialize;

pub const FONT_FAMILY: &str = "Noto Sans";

pub static FONT_BYTES: &[u8] = include_bytes!("../assets/text/NotoSans-Regular.subset.ttf");

pub static SYMBOL_BYTES: &[u8] =
    include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf");

pub static FALLBACK_BYTES: &[u8] =
    include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf");

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub const FULL_FONTS: [&str; 3] = [
    "NotoSans-Regular.ttf",
    "NotoSansSymbols2-Regular.ttf",
    "NotoSansSymbols-Regular.ttf",
];

const MAX_TEXT_BYTES: usize = 256 * 1024;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TextPlacement {
    Screen {
        left: f32,
        top: f32,
    },
    Anchor {
        world: [f64; 3],
        offset: [f32; 2],
    },
    Nameplate {
        world: [f64; 3],
        padding: [f32; 2],
        rounded: bool,
    },
    WorldPlane {
        world: [f64; 3],
        right: [f64; 3],
        up: [f64; 3],
        world_height: f64,
    },
    WorldBillboard {
        world: [f64; 3],
        world_height: f64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextObject {
    pub row: u32,
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextLabel {
    pub id: u32,
    pub object: Option<TextObject>,
    pub text: String,
    pub font_size: f32,
    pub line_height: f32,
    pub color: [u8; 4],
    pub placement: TextPlacement,
    pub clip: Option<[f32; 4]>,
}

impl TextLabel {
    pub fn ink_color(&self) -> [u8; 4] {
        if self.object.is_some_and(|object| object.selected) {
            [0, 0, 0, 255]
        } else {
            self.color
        }
    }
}

pub struct TextRun {
    pub label: TextLabel,
    pub buffer: Buffer,
}

pub struct TextDocument {
    pub fonts: FontSystem,
    pub runs: Vec<TextRun>,
    pub revision: u64,
    pub font_revision: u64,
    pub shape_count: u64,
    pub shaping_ms: f64,
}

impl TextDocument {
    pub fn new() -> Self {
        Self {
            fonts: bundled_fonts(),
            runs: Vec::new(),
            revision: 0,
            font_revision: 1,
            shape_count: 0,
            shaping_ms: 0.0,
        }
    }

    pub fn set_labels(&mut self, labels: Vec<TextLabel>) -> anyhow::Result<()> {
        let mut bytes = 0usize;
        let mut ids = std::collections::HashSet::new();

        for label in &labels {
            bytes = bytes.saturating_add(label.text.len());
            anyhow::ensure!(bytes <= MAX_TEXT_BYTES, "text document exceeds 256 KiB");
            anyhow::ensure!(ids.insert(label.id), "duplicate text label ID {}", label.id);
            validate_label(label)?;
        }

        if self.runs.len() == labels.len() {
            let mut unchanged = true;

            for (run, label) in self.runs.iter().zip(&labels) {
                unchanged &= run.label == *label;
            }

            if unchanged {
                return Ok(());
            }
        }

        let previous = std::mem::take(&mut self.runs);
        let mut previous_by_id = std::collections::HashMap::new();

        for run in previous {
            previous_by_id.insert(run.label.id, run);
        }

        for label in labels {
            let old = previous_by_id.remove(&label.id);
            let buffer = match old {
                Some(run) if same_layout(&run.label, &label) => run.buffer,
                _ => {
                    self.shape_count += 1;
                    let started = crate::load_measure::now();
                    let buffer = shape(&mut self.fonts, &label);
                    self.shaping_ms += crate::load_measure::now() - started;
                    buffer
                }
            };
            self.runs.push(TextRun { label, buffer });
        }

        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    pub fn replace_fonts(&mut self, sources: Vec<Vec<u8>>) -> anyhow::Result<()> {
        let sources = sources
            .into_iter()
            .map(|bytes| fontdb::Source::Binary(std::sync::Arc::new(bytes)))
            .collect();
        self.replace_sources(sources)
    }

    pub fn replace_sources(&mut self, sources: Vec<fontdb::Source>) -> anyhow::Result<()> {
        let mut db = fontdb::Database::new();

        for source in sources {
            let before = db.faces().count();
            db.load_font_source(source);
            anyhow::ensure!(
                db.faces().count() > before,
                "font data contains no usable face"
            );
        }

        anyhow::ensure!(db.faces().count() > 0, "font set is empty");
        db.set_sans_serif_family(FONT_FAMILY);
        self.fonts = FontSystem::new_with_locale_and_db("en-US".into(), db);

        for run in &mut self.runs {
            let started = crate::load_measure::now();
            run.buffer = shape(&mut self.fonts, &run.label);
            self.shaping_ms += crate::load_measure::now() - started;
            self.shape_count += 1;
        }

        self.font_revision = self.font_revision.wrapping_add(1);
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.runs.clear();
        self.runs.shrink_to_fit();
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn diagnostics(&self) -> Vec<GlyphDiagnostic> {
        let mut out = Vec::new();

        for run in &self.runs {
            for line in run.buffer.layout_runs() {
                for glyph in line.glyphs {
                    out.push(GlyphDiagnostic {
                        label: run.label.id,
                        line: line.line_i,
                        cluster: [glyph.start, glyph.end],
                        glyph: glyph.glyph_id,
                        font: format!("{:?}", glyph.font_id),
                        origin: [glyph.x, glyph.y],
                        advance: glyph.w,
                        offset: [
                            glyph.x_offset * glyph.font_size,
                            glyph.y_offset * glyph.font_size,
                        ],
                        baseline: line.line_y,
                        line_width: line.line_w,
                    });
                }
            }
        }

        out
    }
}

impl Default for TextDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GlyphDiagnostic {
    pub label: u32,
    pub line: usize,
    pub cluster: [usize; 2],
    pub glyph: u16,
    pub font: String,
    pub origin: [f32; 2],
    pub advance: f32,
    pub offset: [f32; 2],
    pub baseline: f32,
    pub line_width: f32,
}

pub fn covers(text: &str) -> bool {
    text.is_ascii() || text.chars().all(|c| c.is_control() || bundled_glyph(c))
}

fn bundled_glyph(c: char) -> bool {
    thread_local! {
        static MAPS: Vec<ttf_parser::cmap::Table<'static>> = [FONT_BYTES, FALLBACK_BYTES, SYMBOL_BYTES]
            .into_iter()
            .filter_map(|bytes| {
                let face = ttf_parser::RawFace::parse(bytes, 0).ok()?;
                ttf_parser::cmap::Table::parse(face.table(ttf_parser::Tag::from_bytes(b"cmap"))?)
            })
            .collect();
    }

    MAPS.with(|maps| {
        maps.iter().any(|map| {
            map.subtables
                .into_iter()
                .any(|table| table.is_unicode() && table.glyph_index(u32::from(c)).is_some())
        })
    })
}

fn bundled_fonts() -> FontSystem {
    let mut db = fontdb::Database::new();

    for bytes in [FONT_BYTES, FALLBACK_BYTES, SYMBOL_BYTES] {
        db.load_font_source(fontdb::Source::Binary(std::sync::Arc::new(bytes)));
    }

    db.set_sans_serif_family(FONT_FAMILY);
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}

fn validate_label(label: &TextLabel) -> anyhow::Result<()> {
    anyhow::ensure!(
        label.font_size.is_finite() && (1.0..=256.0).contains(&label.font_size),
        "font size must be 1..256 CSS px"
    );
    anyhow::ensure!(
        label.line_height.is_finite()
            && label.line_height >= label.font_size
            && label.line_height <= 1024.0,
        "invalid text line height"
    );
    let valid_placement = match label.placement {
        TextPlacement::Screen { left, top } => left.is_finite() && top.is_finite(),
        TextPlacement::Anchor { world, offset } => {
            world.iter().all(finite_f64) && offset.iter().all(finite_f32)
        }
        TextPlacement::Nameplate { world, padding, .. } => {
            world.iter().all(finite_f64) && padding.iter().all(valid_padding)
        }
        TextPlacement::WorldPlane {
            world,
            right,
            up,
            world_height,
        } => {
            world.iter().all(finite_f64)
                && valid_plane_axes(right, up)
                && world_height.is_finite()
                && world_height > 0.0
        }
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } => world.iter().all(finite_f64) && world_height.is_finite() && world_height > 0.0,
    };
    anyhow::ensure!(valid_placement, "invalid text placement");

    if let Some(c) = label.clip {
        anyhow::ensure!(
            c.iter().all(finite_f32) && c[2] >= c[0] && c[3] >= c[1],
            "invalid text clip rectangle"
        );
    }

    Ok(())
}

fn valid_plane_axes(right: [f64; 3], up: [f64; 3]) -> bool {
    let mut right_length = 0.0;
    let mut up_length = 0.0;
    let mut dot = 0.0;

    for axis in 0..3 {
        right_length += right[axis] * right[axis];
        up_length += up[axis] * up[axis];
        dot += right[axis] * up[axis];
    }

    (right_length.sqrt() - 1.0).abs() <= 1e-6
        && (up_length.sqrt() - 1.0).abs() <= 1e-6
        && dot.abs() <= 1e-6
}

fn finite_f32(value: &f32) -> bool {
    value.is_finite()
}

fn finite_f64(value: &f64) -> bool {
    value.is_finite()
}

fn valid_padding(value: &f32) -> bool {
    value.is_finite() && (0.0..=256.0).contains(value)
}

fn same_layout(a: &TextLabel, b: &TextLabel) -> bool {
    a.text == b.text && a.font_size == b.font_size && a.line_height == b.line_height
}

fn shape(fonts: &mut FontSystem, label: &TextLabel) -> Buffer {
    let mut buffer = Buffer::new(fonts, Metrics::new(label.font_size, label.line_height));
    buffer.set_size(fonts, None, None);
    buffer.set_wrap(fonts, Wrap::None);
    buffer.set_text(
        fonts,
        &label.text,
        &Attrs::new()
            .family(Family::Name(FONT_FAMILY))
            .metadata(label.id as usize),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(fonts, false);
    buffer
}
```

## <a id="checkpoint-60g-text-shaping"></a>Step 7 of 7 · Prove the current fonts shaping and reuse rules (`60g-text-shaping`)

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
  <text x="140" y="32" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7/7: 60g-text-shaping</text>
  <text x="140" y="50" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text.rs</text>
  <line x1="266" y1="38" x2="292" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="12" width="252" height="52" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <text x="422" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text x="422" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">real bundled fonts</text>
  <line x1="548" y1="38" x2="574" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="12" width="252" height="52" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text x="704" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">presentation reuse</text>
  <line x1="830" y1="38" x2="856" y2="38" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="12" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="983" y="32" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text x="983" y="50" text-anchor="middle" font-family="sans-serif" font-size="9.5" font-weight="700" fill="#111111">native coverage proofs</text>
</svg>
</div>

Copy current production shaping regressions, adapted to the cumulative command vocabulary. Verify ligatures, kerning, combining accents, fallback fonts, atomic refusal and whole-font coverage.

> **Check:** *What distinguishes this check from a character-to-quad mock?* — It uses real font shaping, ligatures, glyph IDs and UTF-8 clusters. The checks compare actual kerning, composed/decomposed accents, fallback font identities and valid whole-font replacement.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/text.rs`

Copy current production ligature, accent, kerning, fallback, failed replacement, reuse and full-font coverage regressions.

After that block, append this supplied code:

```text
--8<-- "journey/code/60g-text-shaping-01.rs" skip=14
```

## Build, Verify & Visual Result

![Recorded visual output for Lesson 10.2 (60g-text-shaping)](../../screenshots/journey/60g-text-shaping.png)

```sh
npm --prefix ../session_tests run course -- dependencies 10.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 10.2
npm --prefix ../session_tests run course -- save 10.2
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../recovery.md).

Continue with [10.3 · A shared glyph atlas and lazy text drawing](61-atlas.md).
