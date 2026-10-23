# 9.4 · Periodic seams and face-edge direction

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 9.4** (Redraft `6.4`) · **3 checkpoints (`58-uses` → `58za-match-original-face-edge-directions`)** · **Study time: 1.5–3 h**

> **What You Will Learn & Build:** Keep both parameter-space uses of a seam. By the final checkpoint (`58za-match-original-face-edge-directions`), you will match shared BRep edges across independently keyed face meshes.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 9 Visual Explanation: 2D (u,v) Trim Loops → 3D Trimmed B-Rep Surface, Bore Holes & Crease Normals</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: 2D parameter domain (u,v) with outer + hole loops. Center: 3D trimmed face + wires. Right: Smooth G1 vs split crease normals.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel A: 2D (u,v) Parameter Domain Trim Loops -->
    <rect x="18" y="64" width="250" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="143" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. 2D (u, v) Parameter Domain</text>
    <!-- Outer CCW loop square -->
    <rect x="68" y="96" width="150" height="96" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <!-- Inner CW hole loop circle (white cutout!) -->
    <circle cx="143" cy="144" r="26" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
    <text x="143" y="148" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Hole (CW)</text>
    <text x="143" y="109" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Outer Loop (CCW)</text>
    <text x="143" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Trimmed domain in (u, v) ∈ [0,1]²</text>
    <text x="143" y="225" text-anchor="middle" font-size="9" fill="#111111">Validated loop closure &amp; orientation</text>

    <line x1="268" y1="154" x2="290" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel B: 3D Curved B-Rep Surface Patch with Bore Hole -->
    <rect x="292" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="429" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. 3D B-Rep Surface S(u,v) + Wires</text>
    <!-- Curved 3D patch shape -->
    <path d="M 325 170 Q 380 95 450 110 L 535 145 Q 470 195 395 190 Z" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
    <!-- Elliptical 3D bore hole cutout in white -->
    <ellipse cx="430" cy="148" rx="28" ry="15" fill="#ffffff" stroke="#111111" stroke-width="2"/>
    <text x="430" y="151" text-anchor="middle" font-size="8.5" font-weight="700" fill="#111111">Empty Bore</text>
    <text x="429" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Shaded Face + Analytical Boundary Wires</text>
    <text x="429" y="225" text-anchor="middle" font-size="9" fill="#111111">Unified Rg32Uint [brep_owner_id, sub_id]</text>

    <line x1="567" y1="154" x2="589" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel C: Smooth G1 vs Crease-Split Normals -->
    <rect x="591" y="64" width="231" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="706" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Smooth vs. Crease Normals</text>
    <!-- Smooth curve on left -->
    <path d="M 608 175 Q 645 130 682 175" fill="none" stroke="#111111" stroke-width="2"/>
    <circle cx="645" cy="152" r="3.5" fill="#111111"/>
    <line x1="645" y1="152" x2="645" y2="105" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <text x="645" y="100" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">1 Shared N</text>
    <text x="645" y="192" text-anchor="middle" font-size="9" fill="#111111">Smooth G1</text>
    <!-- Sharp crease corner on right -->
    <polyline points="718,175 758,142 798,175" fill="none" stroke="#111111" stroke-width="2"/>
    <circle cx="758" cy="142" r="3.5" fill="#111111"/>
    <line x1="758" y1="142" x2="730" y2="106" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <line x1="758" y1="142" x2="786" y2="106" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <text x="758" y="100" text-anchor="middle" font-size="9" font-weight="700" fill="#111111">Split N1 ≠ N2</text>
    <text x="758" y="192" text-anchor="middle" font-size="9" fill="#111111">Sharp Crease</text>
    <text x="706" y="218" text-anchor="middle" font-size="9" fill="#111111">Crease edges split vertices so shading</text>
    <text x="706" y="231" text-anchor="middle" font-size="9" fill="#111111">stays crisp across mechanical edges</text>
    
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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 9.4 Pipeline Schematic: Periodic seams and face-edge direction</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (3 Checkpoints: 58-uses → 58za-match-original-face-edge-directions)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">original seam edge</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">two face uses</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">distinct pcurves</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">coherent mesh</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">chains</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">shared edge</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 9.4:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/cad_seams.rs  ·  src/lib.rs  ·  src/cad_seam_tests.rs  ·  src/cad_orient.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">3 Checkpoints: 58-uses → 58za-match-original-face-edge-directions — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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
| **1** | [`58-uses`](#checkpoint-58-uses) | Keep both parameter-space uses of a seam. | `src/cad_seams.rs`, `src/lib.rs` | `original seam edge → two face uses → distinct pcurves → coherent mesh chains` |
| **2** | [`58a-periodic`](#checkpoint-58a-periodic) | Prove seam continuity and suppress false open borders. | `src/cad_seam_tests.rs`, `src/lib.rs` | `periodic domain ends → equal points and normals → distinct UV addresses → real boundary ink` |
| **3** | [`58za-match-original-face-edge-directions`](#checkpoint-58za-match-original-face-edge-directions) | Match shared BRep edges across independently keyed face meshes. | `src/cad_orient.rs`, `src/lib.rs` | `shared edge → matched face vertices → opposed walks` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-58-uses"></a>

### Step 1 of 3 · Keep both parameter-space uses of a seam (`58-uses`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/3 Visual Trace (58-uses): Keep both parameter-space uses of a seam</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/cad_seams.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original seam edge</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">two face uses</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">distinct pcurves</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">coherent mesh chains</text>
    
</svg>
</div>

- **Goal:** Keep both parameter-space uses of a seam.
- **Data Flow Trace:** `original seam edge → two face uses → distinct pcurves → coherent mesh chains`
- **Files You Touch:** `src/cad_seams.rs` (*Retain both original face uses and pcurve identities at a closed seam.*) · `src/lib.rs` (*Register the source component.*)

One seam edge can belong to the same face twice. Keep each original face use, orientation and pcurve rather than collapsing them into a single UV address.

> **Self-Check Question:** *Does a seam that returns to the same face have only one parameter-space curve?*  
> **Answer:** No. Its two face uses can follow opposite sides of the parameter domain. They share a physical 3D edge but retain different pcurves and orientations.

- **What You Should See:** The blue parametric roof remains visible. Inspect the supplied seam and periodic-grid fixtures; BRep drawing connects in 59.
- **Hands-On Lab Experiment:** Inspect cylinder edge 2. Compare both face IDs, pcurve IDs and orientations; then compare their mesh positions in world coordinates.

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

### Step 2 of 3 · Prove seam continuity and suppress false open borders (`58a-periodic`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/3 Visual Trace (58a-periodic): Prove seam continuity and suppress false open borders</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/cad_seam_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">periodic domain ends</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">equal points and normals</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">distinct UV addresses</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">real boundary ink</text>
    
</svg>
</div>

- **Goal:** Prove seam continuity and suppress false open borders.
- **Data Flow Trace:** `periodic domain ends → equal points and normals → distinct UV addresses → real boundary ink`
- **Files You Touch:** `src/cad_seam_tests.rs` (*Copy actual same-face seam use and periodic-grid continuity checks.*) · `src/lib.rs` (*Register the source component.*)

Copy seam and periodic-surface checks. Equal physical points may carry distinct UV addresses at the domain ends; closed directions do not gain an artificial open boundary.

> **Self-Check Question:** *Should equal seam positions force equal UV coordinates?*  
> **Answer:** No. The original domain-end addresses must remain distinct even when their positions and normals agree. UV identity and topology describe the surface; display coincidence does not rewrite them.

- **Hands-On Lab Experiment:** Follow one cylinder seam through its two original pcurves. In the periodic example, distinguish coincident domain endpoints from a real open border. Inspect the collector's decision to omit that false border; native execution is optional.

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

### Step 3 of 3 · Match original face edge directions (`58za-match-original-face-edge-directions`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/3 Visual Trace (58za-match-original-face-edge-directions): Match original face edge directions</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/cad_orient.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">shared edge</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">matched face vertices</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">opposed walks</text>
    
</svg>
</div>

- **Goal:** Match shared BRep edges across independently keyed face meshes.
- **Data Flow Trace:** `shared edge → matched face vertices → opposed walks`
- **Files You Touch:** `src/cad_orient.rs` (*Type the complete shared face-edge helpers.*) · `src/lib.rs` (*Compile the one original face-orientation owner.*)

Write the shared halfedge, nearest-vertex and edge-direction helpers once. Match positions and directions across independently keyed face meshes; retain deterministic vertex-key tie breaking.

> **Self-Check Question:** *Why compare positions instead of vertex keys across faces?*  
> **Answer:** Each face mesh owns its own keys. Match the shared edge geometrically before comparing occupied halfedge directions.

- **What You Should See:** The earlier blue parametric roof stays visible while shared orientation helpers compile.
- **Hands-On Lab Experiment:** Trace a seam whose two face meshes use different vertex keys. Identify both nearest-match tie breakers.

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

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 58za-match-original-face-edge-directions
npm --prefix ../session_tests run course -- check 58za-match-original-face-edge-directions
npm --prefix ../session_tests run course -- save 58za-match-original-face-edge-directions
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 9.4 (58za-match-original-face-edge-directions)](../../../screenshots/journey/58za-match-original-face-edge-directions.png)

The earlier blue parametric roof stays visible while shared orientation helpers compile.

- **Final Lesson Experiment:** Trace a seam whose two face meshes use different vertex keys. Identify both nearest-match tie breakers.

Continue with [Oriented CAD solids, original picks and empty bores](../../consolidation/59-orientation.md).
