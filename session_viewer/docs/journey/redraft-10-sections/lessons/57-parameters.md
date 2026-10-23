# 9.3 · Trimmed curved faces with real open holes

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 9.3** (Redraft `6.3`) · **2 checkpoints (`57-parameters` → `57a-trims`)** · **Study time: 1–2 h**

> **What You Will Learn & Build:** Keep the trimmed face connected to its surface. By the final checkpoint (`57a-trims`), you will prove trimmed curved faces and open holes.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 9.3 Execution Flow: Trimmed curved faces with real open holes</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (2 Checkpoints: 57-parameters → 57a-trims)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">trimmed mesh vertex</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">original face</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">source surface and</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">UV</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">exact point and</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">normal</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">outer wire and hole</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 9.3:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/cad_samples.rs  ·  src/lib.rs  ·  src/cad_trim_tests.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">2 Checkpoints: 57-parameters → 57a-trims — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 9 CAD B-Rep Pipeline: Topological Loops/Trims, Crease Normals & Analytical Boundary Wires</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How exact Boundary Representations (B-Rep) combine shaded trimmed surface meshes with crisp analytical boundary & isoline curves.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="245" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. B-Rep Topological Validation</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Solid -&gt; Faces -&gt; Loops (Outer + Holes)</text>
    <text x="28" y="126" font-size="10.5" fill="#111111">• Trim curves in 2D (u, v) domain</text>
    <text x="28" y="144" font-size="10.5" font-weight="600" fill="#111111">• Validate orientation, loop closure,</text>
    <text x="28" y="160" font-size="10.5" font-weight="600" fill="#111111">  and manifold edge adjacency</text>
    <text x="28" y="180" font-size="10.5" fill="#111111">• Preserve exact NURBS surface source</text>

    <line x1="263" y1="112" x2="295" y2="112" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="263" y1="175" x2="295" y2="175" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="297" y="68" width="275" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="309" y="86" font-size="11" font-weight="700" fill="#111111">2A. Shaded Trimmed Faces (Surface Lane)</text>
    <text x="309" y="104" font-size="10.5" fill="#111111">• Tessellate within (u,v) trim loops</text>
    <text x="309" y="120" font-size="10.5" fill="#111111">• Smooth normals across G1; split at creases!</text>

    <rect x="297" y="145" width="275" height="68" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="309" y="163" font-size="11" font-weight="700" fill="#111111">2B. Boundary Wires &amp; Isolines (Ribbon Lane)</text>
    <text x="309" y="181" font-size="10.5" fill="#111111">• Extract 3D topological edges + UV isolines</text>
    <text x="309" y="197" font-size="10.5" fill="#111111">• Render via ribbon.wgsl with depth bias</text>

    <line x1="572" y1="138" x2="602" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="604" y="68" width="216" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="712" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Unified B-Rep Sub-Picking</text>
    <text x="616" y="108" font-size="10.5" fill="#111111">• Every triangle &amp; wire carries</text>
    <text x="616" y="124" font-size="10.5" font-weight="600" fill="#111111">  [brep_owner_id, sub_id]</text>
    <text x="616" y="144" font-size="10" fill="#111111">• Click selects whole B-Rep solid</text>
    <text x="616" y="159" font-size="10" fill="#111111">  or drills into Face #i / Edge #j</text>
    <text x="616" y="178" font-size="10.5" fill="#111111">• Outlines &amp; Point Clouds (Sec 6)</text>
    <text x="616" y="193" font-size="10" fill="#111111">  share the same owner ID contract</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **Rust (`session_rust`)** | `BRep` Faces, Outer/Inner Trim Loops & Edge Adjacency | Validates topological loop closure and manifold edges so trimmed CAD surfaces and analytical boundary curves stay synchronized. |
| **Rust / WGSL** | Crease-Split Vertex Normals vs. Smooth `G1` Normals | Averages vertex normals across smooth surface patches (`angle < crease_threshold`) and splits vertices along sharp mechanical creases for crisp shading. |
| **WebGPU (`Rg32Uint`)** | Unified `[brep_owner_id, face_or_edge_sub_id]` Picking | Both shaded B-Rep face triangles and boundary ribbon wires write the same B-Rep owner ID plus sub-element index for whole-solid or sub-face/edge selection. |

**Authoritative production source (`session_viewer/src`):** [app/walk/brep.rs](../../../../src/app/walk/brep.rs), [app/walk/brep_edges.rs](../../../../src/app/walk/brep_edges.rs), [app/walk/brep_orient.rs](../../../../src/app/walk/brep_orient.rs), [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs), [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`57-parameters`](#checkpoint-57-parameters) | Keep the trimmed face connected to its surface. | `src/cad_samples.rs`, `src/lib.rs` | `trimmed mesh vertex → original face → source surface and UV → exact point and normal` |
| **2** | [`57a-trims`](#checkpoint-57a-trims) | Prove trimmed curved faces and open holes. | `src/cad_trim_tests.rs`, `src/lib.rs` | `outer wire and hole → constrained tessellation → retained face samples → unchanged source` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-57-parameters"></a>

### Step 1 of 2 · Keep the trimmed face connected to its surface (`57-parameters`)

- **Goal:** Keep the trimmed face connected to its surface.
- **Data Flow Trace:** `trimmed mesh vertex → original face → source surface and UV → exact point and normal`
- **Files You Touch:** `src/cad_samples.rs` (*Retain original surface addresses and exact UV samples on a trimmed face.*) · `src/lib.rs` (*Register the source component.*)

A face chooses a kept region on an underlying surface. Record source surface and UV addresses from the actual trimmed mesh vertices; preserve f64 samples and analytic normals.

> **Self-Check Question:** *Does a trim replace the original surface control points?*  
> **Answer:** No. The face keeps a region of the underlying surface. Its outer wire and holes choose the visible region while original control points, knots and weights remain unchanged.

- **What You Should See:** The blue parametric roof remains visible. Inspect the supplied trim fixture; BRep drawing connects in 59.
- **Hands-On Lab Experiment:** Inspect the curved trimmed wave fixture. Compare each mesh vertex with the source evaluated at its stored UV and keep every sample within the retained parameter rectangle.

<details>
<summary>Implementation 57-parameters: Keep the trimmed face connected to its surface</summary>

Keep the trimmed face connected to its surface.

Data flow: trimmed mesh vertex → original face → source surface and UV → exact point and normal.

### 1. `src/cad_samples.rs`

Retain original surface addresses and exact UV samples on a trimmed face.

Create the file and type:

```rust
--8<-- "journey/code/57-parameters-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod cad_boundary_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/57-parameters-02.rs" skip=37
```

</details>

<a id="checkpoint-57a-trims"></a>

### Step 2 of 2 · Prove trimmed curved faces and open holes (`57a-trims`)

- **Goal:** Prove trimmed curved faces and open holes.
- **Data Flow Trace:** `outer wire and hole → constrained tessellation → retained face samples → unchanged source`
- **Files You Touch:** `src/cad_trim_tests.rs` (*Copy curved-trim source/UV/normal checks and the hole-region triangle regression.*) · `src/lib.rs` (*Register the source component.*)

Copy the curved trim and hole fixtures. Check kept UVs, analytic normals, coherent CAD border chains and a real hole that no display triangle fills.

> **Self-Check Question:** *Can the viewer draw the entire surface grid and paint over a hole?*  
> **Answer:** That would still leave shaded triangles and pick targets inside the hole. The kept face must be tessellated from its trim region so the omitted region has neither triangles nor face addresses.

- **Hands-On Lab Experiment:** Compare the supplied trimmed-wave and hole data in the browser editor. Locate the retained UV rectangle and the outer and inner wires, then trace why the mesh cannot fill the hole. These are optional validation examples for the mesher already written.

<details>
<summary>Implementation 57a-trims: Prove trimmed curved faces and open holes</summary>

Prove trimmed curved faces and open holes.

Data flow: outer wire and hole → constrained tessellation → retained face samples → unchanged source.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/cad_trim_tests.rs`

Copy curved-trim source/UV/normal checks and the hole-region triangle regression.

Copy this supplied file:

```text
--8<-- "journey/code/57a-trims-01.rs"
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

pub mod cad_samples;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/57a-trims-02.rs" skip=22
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 57a-trims
npm --prefix ../session_tests run course -- check 57a-trims
npm --prefix ../session_tests run course -- save 57a-trims
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Compare the supplied trimmed-wave and hole data in the browser editor. Locate the retained UV rectangle and the outer and inner wires, then trace why the mesh cannot fill the hole. These are optional validation examples for the mesher already written.

Continue with [Periodic seams and face-edge direction](../../consolidation/58-uses.md).
