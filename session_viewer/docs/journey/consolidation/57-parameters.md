# 9.3 · Trimmed curved faces with real open holes

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Checkpoints:** `57-parameters` → `57a-trims` (2 steps) · **Production source:** [app/walk/brep.rs](../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs) · [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs)

> **Goal:** Prove trimmed curved faces and open holes.

## 1. Visual WebGPU Schematic

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Trimmed B-Rep (u,v)</text>
  <rect x="55" y="60" width="160" height="96" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <circle cx="135" cy="108" r="24" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text x="135" y="112" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Hole</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Outer + Inner Loops</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. Crease Split</text>
  <polyline points="338,144 419,98 500,144" fill="none" stroke="#111111" stroke-width="2.5"/>
  <circle cx="419" cy="98" r="4.5" fill="#111111"/>
  <line x1="419" y1="98" x2="386" y2="56" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <line x1="419" y1="98" x2="452" y2="56" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <text x="372" y="60" font-family="monospace" font-size="10" font-weight="700" fill="#111111">N1</text>
  <text x="458" y="60" font-family="monospace" font-size="10" font-weight="700" fill="#111111">N2</text>
  <text x="419" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Sharp Edge Normals</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Sub-ID Pick</text>
  <polygon points="650,74 758,74 780,138 628,138" fill="url(#hatch)" stroke="#111111" stroke-width="2.5"/>
  <text x="704" y="112" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">[brep, face_2]</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">Face &amp; Edge IDs</text>
</svg>
</div>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **Rust (`session_rust`)** | `BRep` Outer & Inner Trim Loops | Validates closed `(u,v)` trim loops and holes on curved CAD faces. |
| **Rust / WGSL** | Smooth `G1` vs. Crease Normals | Averages smooth normals and splits vertices across sharp creases. |
| **WebGPU (`Rg32Uint`)** | `[brep_id, face_or_edge_id]` | Unified whole-solid or sub-face/edge selection. |

![Recorded visual output for Lesson 9.3 (57a-trims)](../../screenshots/journey/57a-trims.png)

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-57-parameters"></a>

### Step 1 of 2 · Keep the trimmed face connected to its surface (`57-parameters`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/cad_samples.rs, src/l…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">source surface and UV</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">exact point and normal</text>
</svg>
</div>

A face chooses a kept region on an underlying surface. Record source surface and UV addresses from the actual trimmed mesh vertices; preserve f64 samples and analytic normals.

> **Check:** *Does a trim replace the original surface control points?* — No. The face keeps a region of the underlying surface. Its outer wire and holes choose the visible region while original control points, knots and weights remain unchanged.

### 1. `src/cad_samples.rs`

Retain original surface addresses and exact UV samples on a trimmed face.

Create the file and type:

```rust
--8<-- "journey/code/57-parameters-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

After that block, type:

```rust
--8<-- "journey/code/57-parameters-02.rs" skip=37
```

**Result after `57-parameters`:** The blue parametric roof remains visible. Inspect the supplied trim fixture; BRep drawing connects in 59.

---

<a id="checkpoint-57a-trims"></a>

### Step 2 of 2 · Prove trimmed curved faces and open holes (`57a-trims`)

<div class="webgpu-schematic" style="margin:14px 0;">
<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;" role="img">
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
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/cad_trim_tests.rs, sr…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">retained face samples</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">unchanged source</text>
</svg>
</div>

Copy the curved trim and hole fixtures. Check kept UVs, analytic normals, coherent CAD border chains and a real hole that no display triangle fills.

> **Check:** *Can the viewer draw the entire surface grid and paint over a hole?* — That would still leave shaded triangles and pick targets inside the hole. The kept face must be tessellated from its trim region so the omitted region has neither triangles nor face addresses.

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/cad_trim_tests.rs`

Copy curved-trim source/UV/normal checks and the hole-region triangle regression.

Copy this supplied file:

```text
--8<-- "journey/code/57a-trims-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the source component.

After that block, append this supplied code:

```text
--8<-- "journey/code/57a-trims-02.rs" skip=22
```

---

## 3. Build, Verify & Save

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.3
npm --prefix ../session_tests run course -- save 9.3
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../recovery.md).

Continue with [9.4 · Periodic seams and face-edge direction](58-uses.md).
