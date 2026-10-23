# 18.6 · Matched rendering comparisons from original sources

**Section 9: Compute Tile Visibility, Clipping Half-Spaces, Section Caps, and Ambient Occlusion** · **Lesson 18.6** (Redraft `9.10`) · **4 checkpoints (`94-declare-the-rendering-comparison-contract` → `94c-prove-the-original-comparison-and-inspect-differences`)**

> **Learning Goal:** Declare the rendering comparison contract.

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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Section 9 Compute Tile Visibility, Clipping Half-Spaces, Winding Section Caps & Horizon GTAO (Ch 17–18)</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How @compute shaders bin triangles into screen tiles for hidden-line removal, clip solids with winding caps, and shade crevices.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="250" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="143" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. @compute Tile Visibility (Ch 17)</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• projected_triangle.wgsl projects tris</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#1d4ed8">• atomicAdd tile counts + prefix-sum CSR</text>
    <text x="28" y="144" font-size="10.5" fill="#1e3a8a">• ink_visibility.wgsl tests center-line</text>
    <text x="28" y="160" font-size="10.5" fill="#1e3a8a">  against covering triangles in tile</text>
    <text x="28" y="178" font-size="10.5" fill="#1e3a8a">• Hidden lines dashed/faded cleanly!</text>

    <line x1="268" y1="138" x2="298" y2="138" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>

    <rect x="300" y="68" width="265" height="145" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="432" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9a3412">2. Clipping &amp; Winding Caps (Ch 17)</text>
    <text x="312" y="108" font-size="10.5" fill="#7c2d12">• Half-space: dot(pos_world, plane) &lt; 0</text>
    <text x="312" y="126" font-size="10.5" font-weight="600" fill="#c2410c">• Signed front/back crossing count</text>
    <text x="312" y="142" font-size="10" fill="#7c2d12">  at clip plane depth (cap.wgsl):</text>
    <text x="312" y="158" font-size="10" fill="#7c2d12">  Fills closed solid interiors while</text>
    <text x="312" y="173" font-size="10" fill="#7c2d12">  leaving internal bores hollow!</text>
    <text x="312" y="191" font-size="10.5" fill="#7c2d12">• Cap picks resolve to solid owner_id</text>

    <line x1="565" y1="138" x2="595" y2="138" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="597" y="68" width="223" height="145" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="708" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#065f46">3. Horizon GTAO &amp; X-Ray (Ch 18)</text>
    <text x="609" y="108" font-size="10.5" fill="#064e3b">• ambient_depth.wgsl -&gt; 6-mip pyramid</text>
    <text x="609" y="126" font-size="10.5" font-weight="600" fill="#047857">• ssao.wgsl + bilateral blur +</text>
    <text x="609" y="142" font-size="10.5" font-weight="600" fill="#047857">  ambient_composite.wgsl (Arctic)</text>
    <text x="609" y="160" font-size="10" fill="#064e3b">• Glass / X-Ray translucent shells</text>
    <text x="609" y="175" font-size="10" fill="#064e3b">  allow picking interior Wood joints</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
same original source → declared camera and pipeline differences → actual maintained frame.
```

- Declare the rendering comparison contract.
- Read actual maintained frames without source mutation.
- Prove the original comparison and inspect differences.
- **Why it works (Can two similar screenshots prove equal viewer behavior?):** No. Use the same original source and declare units, camera, viewport, pixel density, settings and source revision. Then distinguish source/picking invariants from deliberate image differences.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **WGSL (`@compute`)** | `@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>)` | Runs general-purpose parallel GPU threads (outside vertex/fragment rasterization) to project triangles and prefix-sum tile counts. |
| **WGSL (Atomics)** | `atomicAdd(&tile_counts[tile_id], 1u)` | Safely increments shared GPU buffer counters when multiple triangles overlap the same screen tile concurrently. |
| **Math (Winding Cap)** | `signed_crossings != 0` at clip plane depth | Fills only the solid interior of a clipped B-Rep/mesh while leaving interior bores and open shells hollow. |
| **WGSL (`ssao.wgsl`)** | Multi-level depth pyramid horizon integration | Samples 6 mip levels of linear depth and per-object contact radius to shade crevices and ground contact. |

**Production source (`session_viewer/src`):** [engine/gpu/ssao.rs](../../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../../src/app/command/tool/elements.rs), [build.rs](../../../../build.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-94-declare-the-rendering-comparison-contract"></a>

<details>
<summary>Implementation 94-declare-the-rendering-comparison-contract: Declare the rendering comparison contract</summary>

Declare the rendering comparison contract.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

### 1. `src/comparison.rs`

Own one original comparison specimen and explicit maintained-renderer differences.

Create the file and type:

```rust
--8<-- "journey/code/94-declare-the-rendering-comparison-contract-01.rs"
```

### 2. `src/lib.rs`

Register the comparison owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod opacity_command_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/94-declare-the-rendering-comparison-contract-02.rs" skip=40
```

</details>

<a id="checkpoint-94a-link-the-maintained-native-rendering-reference"></a>

<details>
<summary>Implementation 94a-link-the-maintained-native-rendering-reference: Link the maintained native rendering reference</summary>

Link the maintained native rendering reference.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

### 1. `Cargo.toml`

Link the exact maintained viewer from the reconstructed checkpoint.

<details>
<summary>Locate the existing block</summary>

```toml
pollster = "=0.4.0"
log = "=0.4.34"
```

</details>

After that block, type:

```toml
--8<-- "journey/code/94a-link-the-maintained-native-rendering-reference-01.rs" skip=36
```

</details>

<a id="checkpoint-94b-read-actual-maintained-frames-without-source-mutation"></a>

<details>
<summary>Implementation 94b-read-actual-maintained-frames-without-source-mutation: Read actual maintained frames without source mutation</summary>

Read actual maintained frames without source mutation.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 1. `src/reference_example.rs`

Copy the supplied maintained-frame command line example.

Copy this supplied file:

```text
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-01.rs"
```

</details>

### 2. `Cargo.toml`

Expose the maintained reference as a native example.

<details>
<summary>Locate the existing block</summary>

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
current_viewer = { package = "session_viewer", path = "../.." }
```

</details>

After that block, type:

```toml
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-02.rs" skip=121
```

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 3. `src/reference_frame.rs`

Supply the optional native comparison wrapper; the browser application does not call it.

Copy this supplied file:

```text
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-03.rs"
```

</details>

### 4. `src/lib.rs`

Keep the maintained reference native-only.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod comparison;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/94b-read-actual-maintained-frames-without-source-mutation-04.rs" skip=21
```

</details>

<a id="checkpoint-94c-prove-the-original-comparison-and-inspect-differences"></a>

<details>
<summary>Implementation 94c-prove-the-original-comparison-and-inspect-differences: Prove the original comparison and inspect differences</summary>

Prove the original comparison and inspect differences.

Data flow: same original source → declared camera and pipeline differences → actual maintained frame.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/comparison_tests.rs`

Copy supplied actual source and maintained-frame fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register supplied comparison fixtures.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(not(target_arch="wasm32"))]
pub mod reference_frame;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-02.rs" skip=59
```

</details>

### 3. `index.html`

Package the actual original comparison and later metadata protobuf fixtures for browser acceptance.

<details>
<summary>Locate the existing block</summary>

```html
  <link data-trunk rel="copy-dir" href="assets/text" data-target-path="text">
```

</details>

After that block, type:

```html
--8<-- "journey/code/94c-prove-the-original-comparison-and-inspect-differences-package-original-fixtures.html" skip=78
```

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 94c-prove-the-original-comparison-and-inspect-differences
npm --prefix ../session_tests run course -- check 94c-prove-the-original-comparison-and-inspect-differences
npm --prefix ../session_tests run course -- save 94c-prove-the-original-comparison-and-inspect-differences
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

The teaching renderer draws the same four original solids used by the maintained native reference.

- **Try this:** Inspect the supplied original-source and actual maintained GPU fixtures. Inspect the teaching frame, then compare the maintained reference under the same declared settings.

Continue with [Measured frame costs and dependency-aware cache reports](95-record-comparable-large-scene-conditions.md).
