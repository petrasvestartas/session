# 18.7 · Measured frame costs and dependency-aware cache reports

**Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion** · **Checkpoints:** `95-record-comparable-large-scene-conditions` → `95f-prove-cache-invalidation-and-honest-performance-reports` (7 steps) · **Production source:** [engine/gpu/ssao.rs](../../../../src/engine/gpu/ssao.rs) · [engine/gpu/ssao/pipelines.rs](../../../../src/engine/gpu/ssao/pipelines.rs) · [shaders/ssao.wgsl](../../../../src/shaders/ssao.wgsl) · [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl) · [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl) · [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs) · [engine/performance.rs](../../../../src/engine/performance.rs) · [engine/gpu/timing.rs](../../../../src/engine/gpu/timing.rs) · [app/command/tool/elements.rs](../../../../src/app/command/tool/elements.rs)

> **Goal:** Prove cache invalidation and honest performance reports.

## 1. Visual WebGPU Schematic

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Indexed Mesh</text>
  <polygon points="65,68 185,68 65,148" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
  <polygon points="185,68 185,148 65,148" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="65" cy="68" r="4" fill="#111111"/><text x="44" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v0</text>
  <circle cx="185" cy="68" r="4" fill="#111111"/><text x="193" y="68" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v1</text>
  <circle cx="65" cy="148" r="4" fill="#111111"/><text x="44" y="152" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v2</text>
  <circle cx="185" cy="148" r="4" fill="#111111"/><text x="193" y="152" font-family="monospace" font-size="10" font-weight="700" fill="#111111">v3</text>
  <text x="135" y="176" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">idx: [0,1,2, 2,1,3]</text>
  <text x="135" y="191" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">4 Verts · 2 Triangles</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. view_proj * pos</text>
  <circle cx="332" cy="112" r="5" fill="#111111"/>
  <text x="332" y="132" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Eye</text>
  <line x1="332" y1="112" x2="495" y2="62" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <line x1="332" y1="112" x2="495" y2="158" stroke="#111111" stroke-width="1.25" stroke-dasharray="3 3"/>
  <rect x="384" y="86" width="24" height="50" rx="2" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="462" y="64" width="38" height="92" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="396" y="80" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Near</text>
  <text x="481" y="58" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Far</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">mat4x4&lt;f32&gt; Uniform</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Z-Depth &amp; Ray Pick</text>
  <rect x="688" y="68" width="84" height="62" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="642" y="84" width="84" height="62" rx="4" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <line x1="604" y1="142" x2="676" y2="108" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="680" cy="106" r="4" fill="#111111"/>
  <text x="644" y="78" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 (Front)</text>
  <text x="730" y="63" font-family="monospace" font-size="9" fill="#111111">z=0.8</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Depth32Float + Ray Hit</text>
</svg>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **WGSL (`@compute`)** | `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>)` | Runs general-purpose parallel GPU threads (outside vertex/fragment rasterization) to project triangles and prefix-sum tile counts. |
| **WGSL (Atomics)** | `atomicAdd(&tile_counts[tile_id], 1u)` | Safely increments shared GPU buffer counters when multiple triangles overlap the same screen tile concurrently. |
| **Math (Winding Cap)** | `signed_crossings != 0` at clip plane depth | Fills only the solid interior of a clipped B-Rep/mesh while leaving interior bores and open shells hollow. |
| **WGSL (`ssao.wgsl`)** | Multi-level depth pyramid horizon integration | Samples 6 mip levels of linear depth and per-object contact radius to shade crevices and ground contact. |

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-95-record-comparable-large-scene-conditions"></a>

### Step 1 of 7 · Record comparable large scene conditions (`95-record-comparable-large-scene-conditions`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/bench_config.rs, src/…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">reuse unchanged scene w…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">measure rotation and zo…</text>
</svg>

Record comparable large scene conditions.

> **Check:** *Can another GPU or smaller viewport validate the Lenovo fix?* — No. Record the machine, browser, adapter, exact original scene hash, viewport/DPR, saved camera, display settings and measurement method. Keep load time separate from rotation and zoom.

### 1. `src/bench_config.rs`

Own explicit conditions instead of unqualified performance claims.

Create the file and type:

```rust
--8<-- "journey/code/95-record-comparable-large-scene-conditions-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

After that block, type:

```rust
--8<-- "journey/code/95-record-comparable-large-scene-conditions-02.rs" skip=35
```

---

<a id="checkpoint-95a-measure-warmed-frame-times-and-the-slow-tail"></a>

### Step 2 of 7 · Measure warmed frame times and the slow tail (`95a-measure-warmed-frame-times-and-the-slow-tail`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/bench_samples.rs, src…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">reuse unchanged scene w…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">measure rotation and zo…</text>
</svg>

Measure warmed frame times and the slow tail.

> **Check:** *Does average FPS describe occasional long stalls?* — Keep raw frame durations, discard a declared warmup and report median, nearest-rank P95 and maximum. Reject missing, zero or nonfinite samples.

### 1. `src/bench_samples.rs`

Summarize real samples with an explicit warmup and slow-frame tail.

Create the file and type:

```rust
--8<-- "journey/code/95a-measure-warmed-frame-times-and-the-slow-tail-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

After that block, type:

```rust
--8<-- "journey/code/95a-measure-warmed-frame-times-and-the-slow-tail-02.rs" skip=23
```

---

<a id="checkpoint-95b-reject-incomparable-before-and-after-runs"></a>

### Step 3 of 7 · Reject incomparable before and after runs (`95b-reject-incomparable-before-and-after-runs`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/bench_compare.rs, src…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">reuse unchanged scene w…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">measure rotation and zo…</text>
</svg>

Reject incomparable before and after runs.

> **Check:** *Is a faster run meaningful after disabling AO or changing scenes?* — Record that as a different condition. A comparable pair keeps hardware, original scene, viewport, camera, quality and measurement method equal, while naming both renderer revisions.

### 1. `src/bench_compare.rs`

Compare matched measurements and retain original-source memory accounting.

Create the file and type:

```rust
--8<-- "journey/code/95b-reject-incomparable-before-and-after-runs-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

After that block, type:

```rust
--8<-- "journey/code/95b-reject-incomparable-before-and-after-runs-02.rs" skip=24
```

---

<a id="checkpoint-95c-witness-immutable-tree-inputs-across-camera-frames"></a>

### Step 4 of 7 · Witness immutable tree inputs across camera frames (`95c-witness-immutable-tree-inputs-across-camera-frames`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/tree_witness.rs, src/…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">reuse unchanged scene w…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">measure rotation and zo…</text>
</svg>

Witness immutable tree inputs across camera frames.

> **Check:** *Does each camera frame need to rebuild every layer row?* — Reuse rows while immutable document metadata, view flags, graph/groups, selection and edit epoch match. Retain shared metadata witnesses so copy-on-write edits change identity.

### 1. `src/tree_witness.rs`

Witness shared layer inputs without retaining geometry payloads.

Create the file and type:

```rust
--8<-- "journey/code/95c-witness-immutable-tree-inputs-across-camera-frames-01.rs"
```

### 2. `src/lib.rs`

Register the completed performance owner.

After that block, type:

```rust
--8<-- "journey/code/95c-witness-immutable-tree-inputs-across-camera-frames-02.rs" skip=24
```

---

<a id="checkpoint-95d-reuse-layer-rows-while-rotating-and-zooming"></a>

### Step 5 of 7 · Reuse layer rows while rotating and zooming (`95d-reuse-layer-rows-while-rotating-and-zooming`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/tree_panel.rs, src/br…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">reuse unchanged scene w…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">measure rotation and zo…</text>
</svg>

Reuse layer rows while rotating and zooming.

> **Check:** *Should caching hide a newly deleted or renamed object?* — No. Rebuild on changed inputs; camera-only movement reuses the cache. Existing explicit refresh invalidates its witness. Keep tree expansion and editing state independent.

### 1. `src/tree_panel.rs`

Cache unchanged scene/selection/epoch inputs before rebuilding layer state.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/95d-reuse-layer-rows-while-rotating-and-zooming-01.rs"
```

### 2. `src/browser.rs`

Use the cache from the actual teaching frame loop.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/95d-reuse-layer-rows-while-rotating-and-zooming-02.rs"
```

---

<a id="checkpoint-95e-borrow-graph-edges-and-index-their-labels"></a>

### Step 6 of 7 · Borrow graph edges and index their labels (`95e-borrow-graph-edges-and-index-their-labels`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/graph_edit.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">reuse unchanged scene w…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">measure rotation and zo…</text>
</svg>

Borrow graph edges and index their labels.

> **Check:** *Must the graph table clone every vertex attribute?* — Read retained original edges directly and derive only display rows. Build the document label index once, keep the first matching alias and list an undirected edge once.

### 1. `src/graph_edit.rs`

Avoid full graph protobuf conversion and repeated node scans for display rows.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/95e-borrow-graph-edges-and-index-their-labels-01.rs"
```

---

<a id="checkpoint-95f-prove-cache-invalidation-and-honest-performance-reports"></a>

### Step 7 of 7 · Prove cache invalidation and honest performance reports (`95f-prove-cache-invalidation-and-honest-performance-reports`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/bench_tests.rs, src/l…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">reuse unchanged scene w…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">measure rotation and zo…</text>
</svg>

Prove cache invalidation and honest performance reports.

> **Check:** *What completes a performance investigation?* — Correctness checks plus comparable measurements on the affected hardware. A cache test proves avoided work; it does not prove a frame-rate improvement on a different machine.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/bench_tests.rs`

Copy supplied invalid-data, comparability, original-source, cache and history fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/95f-prove-cache-invalidation-and-honest-performance-reports-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register supplied performance correctness fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/95f-prove-cache-invalidation-and-honest-performance-reports-02.rs" skip=23
```

**Result after `95f-prove-cache-invalidation-and-honest-performance-reports`:** The original comparison scene stays visible; cache fixtures prove that camera movement avoids repeated tree reconstruction.

---

## 3. Build, Verify & Save

```sh
npm --prefix ../session_tests run course -- dependencies 18.7
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 18.7
npm --prefix ../session_tests run course -- save 18.7
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [19.1 · Complete source metadata and the actual command contract](../../consolidation/96-retain-non-drawing-originals-and-validate-raw-planes.md).
