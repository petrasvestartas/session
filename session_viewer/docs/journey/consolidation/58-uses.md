# 9.4 · Periodic seams and face-edge direction

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 9.4** (Redraft `6.4`) · **3 checkpoints (`58-uses` → `58za-match-original-face-edge-directions`)**

> **Learning Goal:** Keep both parameter-space uses of a seam.

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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#0f172a">Chapter 9 CAD B-Rep Pipeline: Topological Loops/Trims, Crease Normals & Analytical Boundary Wires</text>
  <text x="20" y="46" font-size="11.5" fill="#475569">How exact Boundary Representations (B-Rep) combine shaded trimmed surface meshes with crisp analytical boundary & isoline curves.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#e2e8f0" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="6" fill="#eff6ff" stroke="#93c5fd" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#1e40af">1. B-Rep Topological Validation</text>
    <text x="28" y="108" font-size="10.5" fill="#1e3a8a">• Solid -&gt; Faces -&gt; Loops (Outer + Holes)</text>
    <text x="28" y="126" font-size="10.5" fill="#1e3a8a">• Trim curves in 2D (u, v) domain</text>
    <text x="28" y="144" font-size="10.5" font-weight="600" fill="#1d4ed8">• Validate orientation, loop closure,</text>
    <text x="28" y="160" font-size="10.5" font-weight="600" fill="#1d4ed8">  and manifold edge adjacency</text>
    <text x="28" y="180" font-size="10.5" fill="#1e3a8a">• Preserve exact NURBS surface source</text>

    <line x1="263" y1="112" x2="295" y2="112" stroke="#2563eb" stroke-width="2" marker-end="url(#arrow-blue)"/>
    <line x1="263" y1="175" x2="295" y2="175" stroke="#059669" stroke-width="2" marker-end="url(#arrow-green)"/>

    <rect x="297" y="68" width="275" height="68" rx="6" fill="#fff7ed" stroke="#fdba74" stroke-width="1.5"/>
    <text x="309" y="86" font-size="11" font-weight="700" fill="#9a3412">2A. Shaded Trimmed Faces (Surface Lane)</text>
    <text x="309" y="104" font-size="10.5" fill="#7c2d12">• Tessellate within (u,v) trim loops</text>
    <text x="309" y="120" font-size="10.5" fill="#7c2d12">• Smooth normals across G1; split at creases!</text>

    <rect x="297" y="145" width="275" height="68" rx="6" fill="#ecfdf5" stroke="#6ee7b7" stroke-width="1.5"/>
    <text x="309" y="163" font-size="11" font-weight="700" fill="#065f46">2B. Boundary Wires &amp; Isolines (Ribbon Lane)</text>
    <text x="309" y="181" font-size="10.5" fill="#064e3b">• Extract 3D topological edges + UV isolines</text>
    <text x="309" y="197" font-size="10.5" fill="#064e3b">• Render via ribbon.wgsl with depth bias</text>

    <line x1="572" y1="138" x2="602" y2="138" stroke="#475569" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="604" y="68" width="216" height="145" rx="6" fill="#fdf2f8" stroke="#f472b6" stroke-width="1.5"/>
    <text x="712" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#9d174d">3. Unified B-Rep Sub-Picking</text>
    <text x="616" y="108" font-size="10.5" fill="#831843">• Every triangle &amp; wire carries</text>
    <text x="616" y="124" font-size="10.5" font-weight="600" fill="#9d174d">  [brep_owner_id, sub_id]</text>
    <text x="616" y="144" font-size="10" fill="#831843">• Click selects whole B-Rep solid</text>
    <text x="616" y="159" font-size="10" fill="#831843">  or drills into Face #i / Edge #j</text>
    <text x="616" y="178" font-size="10.5" fill="#831843">• Outlines &amp; Point Clouds (Sec 6)</text>
    <text x="616" y="193" font-size="10" fill="#831843">  share the same owner ID contract</text>
    
</svg>
</div>

### Lesson Data Flow (`Rust` → `WebGPU` → `WGSL`)

```
original seam edge → two face uses → distinct pcurves → coherent mesh chains.
periodic domain ends → equal points and normals → distinct UV addresses → real boundary ink.
shared edge → matched face vertices → opposed walks.
```

- Keep both parameter-space uses of a seam.
- Prove seam continuity and suppress false open borders.
- Match shared BRep edges across independently keyed face meshes.
- **Why it works (Does a seam that returns to the same face have only one parameter-space curve?):** No. Its two face uses can follow opposite sides of the parameter domain. They share a physical 3D edge but retain different pcurves and orientations.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **Rust (`session_rust`)** | `BRep` Faces, Outer/Inner Trim Loops & Edge Adjacency | Validates topological loop closure and manifold edges so trimmed CAD surfaces and analytical boundary curves stay synchronized. |
| **Rust / WGSL** | Crease-Split Vertex Normals vs. Smooth `G1` Normals | Averages vertex normals across smooth surface patches (`angle < crease_threshold`) and splits vertices along sharp mechanical creases for crisp shading. |
| **WebGPU (`Rg32Uint`)** | Unified `[brep_owner_id, face_or_edge_sub_id]` Picking | Both shaded B-Rep face triangles and boundary ribbon wires write the same B-Rep owner ID plus sub-element index for whole-solid or sub-face/edge selection. |

**Production source (`session_viewer/src`):** [app/walk/brep.rs](../../../src/app/walk/brep.rs), [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs), [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs), [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs), [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-58-uses"></a>

<details>
<summary>Implementation 58-uses: Keep both parameter-space uses of a seam</summary>

Keep both parameter-space uses of a seam.

Data flow: original seam edge → two face uses → distinct pcurves → coherent mesh chains.

### 1. `src/cad_seams.rs`

Retain both original face uses and pcurve identities at a closed seam.

Create the file and type:

```rust
--8<-- "journey/code/58-uses-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod cad_trim_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/58-uses-02.rs" skip=33
```

</details>

<a id="checkpoint-58a-periodic"></a>

<details>
<summary>Implementation 58a-periodic: Prove seam continuity and suppress false open borders</summary>

Prove seam continuity and suppress false open borders.

Data flow: periodic domain ends → equal points and normals → distinct UV addresses → real boundary ink.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/cad_seam_tests.rs`

Copy actual same-face seam use and periodic-grid continuity checks.

Copy this supplied file:

```text
--8<-- "journey/code/58a-periodic-01.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```text

pub mod cad_seams;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/58a-periodic-02.rs" skip=20
```

</details>

</details>

<a id="checkpoint-58za-match-original-face-edge-directions"></a>

<details>
<summary>Implementation 58za-match-original-face-edge-directions: Match original face edge directions</summary>

Match shared BRep edges across independently keyed face meshes.

Data flow: shared edge → matched face vertices → opposed walks.

### 1. `src/cad_orient.rs`

Type the complete shared face-edge helpers.

Create the file and type:

```rust
--8<-- "journey/code/58za-match-original-face-edge-directions-dedup-1.rs"
```

### 2. `src/lib.rs`

Compile the one original face-orientation owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod cad_seam_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/58za-match-original-face-edge-directions-dedup-2.rs" skip=33
```

</details>

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 58za-match-original-face-edge-directions
npm --prefix ../session_tests run course -- check 58za-match-original-face-edge-directions
npm --prefix ../session_tests run course -- save 58za-match-original-face-edge-directions
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

The earlier blue parametric roof stays visible while shared orientation helpers compile.

- **Try this:** Trace a seam whose two face meshes use different vertex keys. Identify both nearest-match tie breakers.

Continue with [Oriented CAD solids, original picks and empty bores](59-orientation.md).
