# 16.3 · Joined-face topology and exact surface measures

**Section 8: Precision Coordinate Frames, Solid Primitives, Surfacing, and CAD Editing** · **Lesson 16.3** (Redraft `8.13`) · **4 checkpoints (`83-split-original-faces-without-separating-the-solid` → `83ze-integrate-original-surface-area`)** · **Study time: 2–4 h**

> **What You Will Learn & Build:** Divide a selected original surface face while preserving joined topology and one-step history. By the final checkpoint (`83ze-integrate-original-surface-area`), you will reuse mesh area for meshes, meshed BReps and Elements.

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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Section 8 Visual Explanation: Orthonormal Plane Frames, Parametric Solids, Loft/Sweep & Rational Control Edits</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Orthonormal Plane (origin, X, Y, Z). Center: Loft/Extrude surfacing across section curves. Right: Rational NURBS control drag.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="64" width="245" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="140" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Orthonormal Plane Frame (f64)</text>
    <circle cx="115" cy="162" r="4" fill="#111111"/>
    <line x1="115" y1="162" x2="115" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="115" y1="162" x2="195" y2="178" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <line x1="115" y1="162" x2="175" y2="122" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <text x="115" y="94" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Z = X × Y</text>
    <text x="206" y="182" font-size="9.5" font-weight="700" fill="#111111">X</text>
    <text x="186" y="122" font-size="9.5" font-weight="700" fill="#111111">Y</text>
    <text x="140" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Rejects collinear / non-finite axes</text>

    <line x1="263" y1="154" x2="285" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="287" y="64" width="275" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="424" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Loft / Sweep / Extrude Surfacing</text>
    <path d="M 320 175 Q 370 120 420 165 L 520 135 Q 470 90 420 135 Z" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <text x="424" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Parametric Solids &amp; Surfacing Recipes</text>

    <line x1="562" y1="154" x2="584" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="586" y="64" width="236" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. Rational Control-Point Deform</text>
    <polyline points="615,175 665,110 745,115 790,175" fill="none" stroke="#111111" stroke-width="1" stroke-dasharray="3,3"/>
    <path d="M 615 175 C 655 125, 750 125, 790 175" fill="none" stroke="#111111" stroke-width="2"/>
    <circle cx="665" cy="110" r="4.5" fill="#111111"/>
    <circle cx="745" cy="115" r="4.5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="704" y="212" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Preserves rational weight w on drag</text>
    
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
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 16.3 Pipeline Schematic: Joined-face topology and exact surface measures</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (4 Checkpoints: 83-split-original-faces-without-separating-the-solid → 83ze-integrate-original-surface-area)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="151" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">original selected</text>
    <text x="89" y="124" text-anchor="middle" font-size="11" fill="#111111">face</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="90" x2="316" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">on-surface original</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">cutters</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="90" x2="481" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">joined trimmed</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">regions</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="90" x2="646" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">one transaction</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="66" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="90" x2="811" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">original geometry</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 16.3:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/split_draft.rs  ·  src/split_face_tests.rs  ·  src/lib.rs  ·  src/measure_math.rs  ·  src/measure_length.rs  ·  src/cad_orient.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">4 Checkpoints: 83-split-original-faces-without-separating-the-solid → 83ze-integrate-original-surface-area — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **CAD Coords** | `w10,20,0` vs `@5,0,0` vs `10<30` | `w` forces absolute World coordinates; `@` measures relative to the last accepted point; `<` uses polar distance and angle in the active Construction Plane. |
| **Rational NURBS** | Homogeneous control point `(w*x, w*y, w*z, w)` | Represents circles, ellipses, and conic arcs with zero approximation error using non-uniform weights `w`. |
| **B-Rep Split/Trim** | Plane/Cutter intersection -> Face loop classification | Splits or trims B-Rep and mesh faces along cutting planes/curves while keeping retained topology tables valid. |

**Authoritative production source (`session_viewer/src`):** [app/command/verbs/project_to_plane.rs](../../../../src/app/command/verbs/project_to_plane.rs), [app/command/verbs/extend.rs](../../../../src/app/command/verbs/extend.rs), [app/splitting.rs](../../../../src/app/splitting.rs), [app/command/verbs/measure.rs](../../../../src/app/command/verbs/measure.rs), [app/command/verbs/trim/parts.rs](../../../../src/app/command/verbs/trim/parts.rs), [app/command/verbs/explode.rs](../../../../src/app/command/verbs/explode.rs), [app/walk/frames.rs](../../../../src/app/walk/frames.rs), [build.rs](../../../../build.rs).

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`83-split-original-faces-without-separating-the-solid`](#checkpoint-83-split-original-faces-without-separating-the-solid) | Divide a selected original surface face while preserving joined topology and one-step history. | `src/split_draft.rs`, `src/split_face_tests.rs`, `src/lib.rs` | `original selected face → on-surface original cutters → joined trimmed regions → one transaction` |
| **2** | [`83za-visit-original-mesh-triangles-and-format-measures`](#checkpoint-83za-visit-original-mesh-triangles-and-format-measures) | Write one visitor for the original mesh triangles in world coordinates. | `src/measure_math.rs`, `src/lib.rs` | `original triangles → world transform → area or signed swept volume` |
| **3** | [`83zb-compute-original-mesh-measures-and-curve-length`](#checkpoint-83zb-compute-original-mesh-measures-and-curve-length) | Reuse the triangle visitor for face area, mesh area and signed swept volume. | `src/measure_math.rs`, `src/measure_length.rs`, `src/lib.rs`, `src/cad_orient.rs` | `original geometry → placed source → measure or refusal` |
| **4** | [`83ze-integrate-original-surface-area`](#checkpoint-83ze-integrate-original-surface-area) | Reuse mesh area for meshes, meshed BReps and Elements. | `src/measure_area.rs`, `src/lib.rs` | `original geometry → placed source → measure or refusal` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-83-split-original-faces-without-separating-the-solid"></a>

### Step 1 of 4 · Split original faces without separating the solid (`83-split-original-faces-without-separating-the-solid`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/4 Visual Trace (83-split-original-faces-without-separating-the-solid): Split original faces without separating the solid</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/split_draft.rs, src/split_face_tests.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original selected face</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">on-surface original</text>
    <text x="316" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">cutters</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">joined trimmed regions</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">one transaction</text>
    
</svg>
</div>

- **Goal:** Divide a selected original surface face while preserving joined topology and one-step history.
- **Data Flow Trace:** `original selected face → on-surface original cutters → joined trimmed regions → one transaction`
- **Files You Touch:** `src/split_draft.rs` (*Allow original surfaces and validated BRep faces through the shared pending split owner.*) · `src/split_face_tests.rs` (*Copy integrated maintained joined-face, original placement, topology, area, archive and history fixtures.*) · `src/lib.rs` (*Register the copied original-face split fixtures.*)

Allow original surfaces and explicitly selected BRep faces through the shared split owner. Retain one original BRep with trimmed regions.

> **Self-Check Question:** *Does splitting a face create separate mesh objects?*  
> **Answer:** No. A NURBS surface becomes one trimmed BRep. Splitting one face of a solid keeps that solid joined, replaces that face with regions and preserves its other faces. A multi-face BRep requires an explicit face selection.

- **What You Should See:** One original surface becomes a two-region trimmed BRep beside its retained original cutter. Splitting a solid face keeps the solid joined; one Undo restores its original topology.
- **Hands-On Lab Experiment:** Type Close, Nurbs Surface 4 Points w-2,-2,0 w2,-2,0 w2,2,0 w-2,2,0 and Line w0,-3,0 w0,3,0. Type View Isometric, Select None and Fit. Click the surface away from the cutter, type Split, click the cutter beyond the surface and press Enter. Inspect two trimmed faces on one original BRep; Undo restores the original surface and Redo restores both regions. For a solid box, Ctrl+Shift-select a face first and use a cutter lying on that face. A whole multi-face solid, out-of-range face or off-surface cutter is refused. Save/Open must retain the trimmed original.

<details>
<summary>Implementation 83-split-original-faces-without-separating-the-solid: Split original faces without separating the solid</summary>

Divide a selected original surface face while preserving joined topology and one-step history.

Data flow: original selected face → on-surface original cutters → joined trimmed regions → one transaction.

### 1. `src/split_draft.rs`

Allow original surfaces and validated BRep faces through the shared pending split owner.

<details>
<summary>Locate the existing block</summary>

```rust
    pub fn new(scene: &Scene, id: ObjectId, face: Option<usize>) -> Result<Self, String> {
        let target = Owner::capture(scene, id).map_err(str::to_owned)?;
        if !crate::split_geometry::is_cutter(&target.source) {
            return Err("Select a curve; surface-face splitting connects next".into());
        }
        let face = crate::split_geometry::face_index(&target.source, face)?;
        Ok(Self {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/83-split-original-faces-without-separating-the-solid-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/split_face_tests.rs`

Copy integrated maintained joined-face, original placement, topology, area, archive and history fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/83-split-original-faces-without-separating-the-solid-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/lib.rs`

Register the copied original-face split fixtures.

<details>
<summary>Locate the existing block</summary>

```text
#[cfg(test)]
mod split_tests;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83-split-original-faces-without-separating-the-solid-03.rs" skip=30
```

</details>

</details>

<a id="checkpoint-83za-visit-original-mesh-triangles-and-format-measures"></a>

### Step 2 of 4 · Visit original mesh triangles and format measures (`83za-visit-original-mesh-triangles-and-format-measures`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 2/4 Visual Trace (83za-visit-original-mesh-triangles-and-format-measures): Visit original mesh triangles and format measures</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/measure_math.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original triangles</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">world transform</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">area or signed swept volume</text>
    
</svg>
</div>

- **Goal:** Write one visitor for the original mesh triangles in world coordinates.
- **Data Flow Trace:** `original triangles → world transform → area or signed swept volume`
- **Files You Touch:** `src/measure_math.rs` (*Type the whole original mesh measurement helpers using the shared visitor.*) · `src/lib.rs` (*Compile this shared original measurement module.*)

Write one visitor for the original mesh triangles in world coordinates. Reuse the cached triangulation when present and the existing face fan otherwise. Keep short numeric and count formatting under the same measurement helper owner.

> **Self-Check Question:** *Why measure the cached triangles when they exist?*  
> **Answer:** They are the triangles the viewer draws. Reuse their geometry instead of inventing another triangulation for measurements.

- **What You Should See:** The earlier drawing remains visible while mesh measurement helpers are prepared.
- **Hands-On Lab Experiment:** Trace a cached face and a fan face through one transform. Compare a small nonzero value with rounded negative zero.

<details>
<summary>Implementation 83za-visit-original-mesh-triangles-and-format-measures: Visit original mesh triangles and format measures</summary>

Write one visitor for the original mesh triangles in world coordinates.

Data flow: original triangles → world transform → area or signed swept volume.

### 1. `src/measure_math.rs`

Type the whole original mesh measurement helpers using the shared visitor.

Create the file and type:

```rust
--8<-- "journey/code/83za-visit-original-mesh-triangles-and-format-measures-1.rs"
```

### 2. `src/lib.rs`

Compile this shared original measurement module.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod split_face_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/83za-visit-original-mesh-triangles-and-format-measures-2.rs" skip=35
```

</details>

<a id="checkpoint-83zb-compute-original-mesh-measures-and-curve-length"></a>

### Step 3 of 4 · Compute original mesh measures and curve length (`83zb-compute-original-mesh-measures-and-curve-length`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 3/4 Visual Trace (83zb-compute-original-mesh-measures-and-curve-length): Compute original mesh measures and curve length</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/measure_math.rs, src/measure_length.rs, src/lib.rs, src/cad_orient.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original geometry</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">placed source</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">measure or refusal</text>
    
</svg>
</div>

- **Goal:** Reuse the triangle visitor for face area, mesh area and signed swept volume.
- **Data Flow Trace:** `original geometry → placed source → measure or refusal`
- **Files You Touch:** `src/measure_math.rs` (*Reuse the original triangle visitor for mesh area and signed swept volume.*) · `src/measure_length.rs` (*Type the complete original helpers introduced today.*) · `src/lib.rs` (*Register this original measurement owner.*) · `src/cad_orient.rs` (*Supply only original orientation validation fixtures against the already taught shared owner.*)

Reuse the triangle visitor for face area, mesh area and signed swept volume. Measure placed lines, polylines and NURBS curves through the Session kernel. Reuse the earlier face-orientation owner for solid measurements; supply only its original measurement validation fixtures.

> **Self-Check Question:** *Why transform a curve before measuring it?*  
> **Answer:** Scaling changes its length. Measure the placed source geometry in world units instead of measuring the unplaced source or its screen preview.

- **What You Should See:** Original mesh measures and curve length helpers compile without changing the current drawing.
- **Hands-On Lab Experiment:** Compare a translated curve with a scaled curve. Trace a concave face through the shared visitor.

<details>
<summary>Implementation 83zb-compute-original-mesh-measures-and-curve-length: Compute original mesh measures and curve length</summary>

Reuse the triangle visitor for face area, mesh area and signed swept volume.

Data flow: original geometry → placed source → measure or refusal.

### 1. `src/measure_math.rs`

Reuse the original triangle visitor for mesh area and signed swept volume.

<details>
<summary>Locate the existing block</summary>

```rust
    let corners = mesh.face.get(&face).map_or(&[][..], Vec::as_slice);
    (1..corners.len().saturating_sub(1))
        .filter_map(|i| triangle([corners[0], corners[i], corners[i + 1]]))
        .for_each(visit);
}
```

</details>

After that block, type:

```rust
/// Area of one mesh face in world units; a fan that folds back (a concave polygon) sums with signs, so it stays exact.
pub fn compute_face_area(mesh: &Mesh, face: usize, place: &Xform) -> f64 {
    let cached = mesh
        .triangulation
        .get(&face)
        .is_some_and(|tris| !tris.is_empty());
    let mut summed = Vector::new(0.0, 0.0, 0.0); // signed fan normals
    let mut unsigned = 0.0; // triangle areas, doubled

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        let cross = (&b - &a).cross(&(&c - &a));
        unsigned += cross.magnitude();
        summed += cross;
    });

    if cached || unsigned - summed.magnitude() <= 1e-12 * unsigned {
        return unsigned * 0.5;
    }

    // a warped face whose fan never folds back is the drawn area
    let mut folded = false;

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        folded |= (&b - &a).cross(&(&c - &a)).dot(&summed) < 0.0;
    });

    if !folded {
        return unsigned * 0.5;
    }

    summed.magnitude() * 0.5
}

/// Area of a mesh in world units: every face, or only `face`.
pub fn compute_mesh_area(mesh: &Mesh, place: &Xform, face: Option<usize>) -> f64 {
    match face {
        Some(face) => compute_face_area(mesh, face, place),
        None => mesh
            .faces()
            .into_iter()
            .map(|face| compute_face_area(mesh, face, place))
            .sum(),
    }
}

/// Six times the signed volume the mesh's triangles sweep from `origin`, in world units.
pub fn compute_swept(mesh: &Mesh, place: &Xform, origin: &Point) -> f64 {
    let mut total = 0.0;

    for face in mesh.faces() {
        for_each_triangle(mesh, face, place, |[a, b, c]| {
            total += (&a - origin).dot(&(&b - origin).cross(&(&c - origin)));
        });
    }

    total
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/measure_math.rs`

Supply only the unchanged original mesh measurement fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use session_rust::{Mesh,Point,Vector,Xform};
/// Visit the triangles the viewer draws for face `face`, in world coordinates: the cached triangulation, else a fan.
pub fn for_each_triangle(
    mesh: &Mesh,
    face: usize,
    place: &Xform,
    visit: impl FnMut([Point; 3]),
) {
    let at = |key: usize| mesh.vertex_point(key).map(|p| p.transformed(place));
    let triangle = |[a, b, c]: [usize; 3]| Some([at(a)?, at(b)?, at(c)?]);

    if let Some(cached) = mesh
        .triangulation
        .get(&face)
        .filter(|tris| !tris.is_empty())
    {
        cached
            .iter()
            .filter_map(|&keys| triangle(keys))
            .for_each(visit);
        return;
    }

    let corners = mesh.face.get(&face).map_or(&[][..], Vec::as_slice);
    (1..corners.len().saturating_sub(1))
        .filter_map(|i| triangle([corners[0], corners[i], corners[i + 1]]))
        .for_each(visit);
}

/// Area of one mesh face in world units; a fan that folds back (a concave polygon) sums with signs, so it stays exact.
pub fn compute_face_area(mesh: &Mesh, face: usize, place: &Xform) -> f64 {
    let cached = mesh
        .triangulation
        .get(&face)
        .is_some_and(|tris| !tris.is_empty());
    let mut summed = Vector::new(0.0, 0.0, 0.0); // signed fan normals
    let mut unsigned = 0.0; // triangle areas, doubled

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        let cross = (&b - &a).cross(&(&c - &a));
        unsigned += cross.magnitude();
        summed += cross;
    });

    if cached || unsigned - summed.magnitude() <= 1e-12 * unsigned {
        return unsigned * 0.5;
    }

    // a warped face whose fan never folds back is the drawn area
    let mut folded = false;

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        folded |= (&b - &a).cross(&(&c - &a)).dot(&summed) < 0.0;
    });

    if !folded {
        return unsigned * 0.5;
    }

    summed.magnitude() * 0.5
}

/// Area of a mesh in world units: every face, or only `face`.
pub fn compute_mesh_area(mesh: &Mesh, place: &Xform, face: Option<usize>) -> f64 {
    match face {
        Some(face) => compute_face_area(mesh, face, place),
        None => mesh
            .faces()
            .into_iter()
            .map(|face| compute_face_area(mesh, face, place))
            .sum(),
    }
}

/// Six times the signed volume the mesh's triangles sweep from `origin`, in world units.
pub fn compute_swept(mesh: &Mesh, place: &Xform, origin: &Point) -> f64 {
    let mut total = 0.0;

    for face in mesh.faces() {
        for_each_triangle(mesh, face, place, |[a, b, c]| {
            total += (&a - origin).dot(&(&b - origin).cross(&(&c - origin)));
        });
    }

    total
}

/// A value with at most three decimals, no trailing zeros, and no minus on zero.
pub fn to_text(value: f64) -> String {
    // too small for three decimals but not zero
    if value != 0.0 && value.abs() < 0.0005 {
        return format!("{value:.3e}");
    }

    let text = format!("{value:.3}");
    let text = text.trim_end_matches('0').trim_end_matches('.');

    match text {
        "-0" => "0".into(),
        text => text.into(),
    }
}

/// `count` followed by `one` or its plural.
pub fn plural(count: usize, one: &str) -> String {
    match count {
        1 => format!("1 {one}"),
        _ => format!("{count} {one}s"),
    }
}

/// ` · N other objects skipped`, or nothing.
pub fn skipped_text(count: usize) -> String {
    match count {
        0 => String::new(),
        _ => format!(" · {} skipped", plural(count, "other object")),
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-2.rs" skip=3574
```

</details>

### 3. `src/measure_length.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-3.rs"
```

### 4. `src/lib.rs`

Register this original measurement owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod measure_math;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-4.rs" skip=23
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/measure_length.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use session_rust::{Geometry,Xform};
/// World length of a line, polyline or NURBS curve placed by `place`; None for other kinds.
pub fn compute_length(geometry: &Geometry, place: &Xform) -> Option<f64> {
    match geometry {
        Geometry::Line(line) => Some(line.transformed(place).length()),
        Geometry::Polyline(polyline) => Some(polyline.transformed(place).length()),
        Geometry::NurbsCurve(curve) => Some(curve.transformed(place).length(None)),
        _ => None,
    }
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-5.rs" skip=493
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/cad_orient.rs`

Supply only original orientation validation fixtures against the already taught shared owner.

<details>
<summary>Locate the existing block</summary>

```text
use crate::cad_edges::EdgeChain;
use session_rust::{BRep, Mesh};

fn walks(fm: &Mesh, s: usize, n: usize) -> Option<bool> {
    let fwd = occupied_halfedge(fm, s, n);
    let back = occupied_halfedge(fm, n, s);

    match (fwd, back) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

fn occupied_halfedge(fm: &Mesh, from: usize, to: usize) -> bool {
    let Some(neighbours) = fm.halfedge.get(&from) else {
        return false;
    };
    matches!(neighbours.get(&to), Some(Some(_)))
}

fn at(fm: &Mesh, k: usize) -> [f64; 3] {
    let v = &fm.vertex[&k];
    [v.x, v.y, v.z]
}

fn neighbour_along(fm: &Mesh, s: usize, dir: [f64; 3]) -> Option<usize> {
    let p = at(fm, s);
    let mut best: Option<(f64, usize)> = None;

    for &w in fm.halfedge.get(&s)?.keys() {
        let q = at(fm, w);
        let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
        let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();

        if l == 0.0 {
            continue;
        }

        let c = (d[0] * dir[0] + d[1] * dir[1] + d[2] * dir[2]) / l;

        if match best {
            Some((bc, bw)) => c > bc || (c == bc && w < bw),
            None => true,
        } {
            best = Some((c, w));
        }
    }

    Some(best?.1)
}

fn vertex_at(fm: &Mesh, p: [f64; 3]) -> Option<usize> {
    let mut best: Option<(f64, usize)> = None;

    for (&k, v) in fm.vertex.iter() {
        let d = (v.x - p[0]).powi(2) + (v.y - p[1]).powi(2) + (v.z - p[2]).powi(2);

        if match best {
            Some((bd, bk)) => d < bd || (d == bd && k < bk),
            None => true,
        } {
            best = Some((d, k));
        }
    }

    Some(best?.1)
}

fn opposed(fms: &[Mesh], c: &EdgeChain) -> Option<bool> {
    let other = c.other?;
    let (fa, fb) = (&fms[c.face], &fms[other]);
    let (s, n) = (c.keys[0], c.keys[1]);
    let away_a = walks(fa, s, n)?;
    let (ps, pn) = (at(fa, s), at(fa, n));
    let dir = [pn[0] - ps[0], pn[1] - ps[1], pn[2] - ps[2]];
    let sb = vertex_at(fb, ps)?;
    let nb = neighbour_along(fb, sb, dir)?;
    let away_b = walks(fb, sb, nb)?;
    Some(away_a != away_b)
}

fn six_volume(fm: &Mesh) -> f64 {
    let mut keys: Vec<usize> = fm.face.keys().copied().collect();
    keys.sort_unstable();
    let mut v = 0.0;

    for k in keys {
        let verts = &fm.face[&k];

        if verts.len() < 3 {
            continue;
        }

        let (a, b, c) = (at(fm, verts[0]), at(fm, verts[1]), at(fm, verts[2]));
        v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]);
    }

    v
}

pub fn face_signs(b: &BRep, fms: &[Mesh], chains: &[Option<EdgeChain>]) -> Vec<f64> {
    let nf = fms.len();

    if !b.is_solid() {
        return vec![1.0; nf];
    }

    let mut adjacent: Vec<Vec<(usize, bool)>> = vec![Vec::new(); nf];

    for c in chains.iter().flatten() {
        if let (Some(other), Some(opp)) = (c.other, opposed(fms, c)) {
            adjacent[c.face].push((other, opp));
            adjacent[other].push((c.face, opp));
        }
    }

    let mut sign = vec![0.0f64; nf];

    for start in 0..nf {
        if sign[start] != 0.0 {
            continue;
        }

        sign[start] = 1.0;
        let mut group = vec![start];
        let mut head = 0;

        while head < group.len() {
            let f = group[head];
            head += 1;

            for &(g, opp) in &adjacent[f] {
                if sign[g] == 0.0 {
                    sign[g] = if opp { sign[f] } else { -sign[f] };
                    group.push(g);
                }
            }
        }

        let mut volume = 0.0;

        for &face in &group {
            volume += sign[face] * six_volume(&fms[face]);
        }

        if volume < 0.0 {
            for &f in &group {
                sign[f] = -sign[f];
            }
        }
    }

    sign
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83zb-compute-original-mesh-measures-and-curve-length-dedup-6.rs" skip=3938
```

</details>

</details>

<a id="checkpoint-83ze-integrate-original-surface-area"></a>

### Step 4 of 4 · Integrate original surface area (`83ze-integrate-original-surface-area`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 4/4 Visual Trace (83ze-integrate-original-surface-area): Integrate original surface area</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/measure_area.rs, src/lib.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="146" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">original geometry</text>
    <line x1="274" y1="68" x2="290" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="292" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="420" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">placed source</text>
    <line x1="548" y1="68" x2="564" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="566" y="42" width="256" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="694" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">measure or refusal</text>
    
</svg>
</div>

- **Goal:** Reuse mesh area for meshes, meshed BReps and Elements.
- **Data Flow Trace:** `original geometry → placed source → measure or refusal`
- **Files You Touch:** `src/measure_area.rs` (*Type the complete original helpers introduced today.*) · `src/lib.rs` (*Register surface area after curve length; reuse the earlier orientation owner.*)

Reuse mesh area for meshes, meshed BReps and Elements. Integrate the NURBS derivative cross product across knot spans with the original Gauss rule.

> **Self-Check Question:** *What does the derivative cross product measure?*  
> **Answer:** Its magnitude converts a small parameter-space rectangle into surface area. The quadrature weights sum those local contributions over each knot span.

- **What You Should See:** Surface-area integration can measure original surface data before the measurement command connection.
- **Hands-On Lab Experiment:** Compare uniform scaling with translation. Check the planar patch and exact cylinder surface against their known areas.

<details>
<summary>Implementation 83ze-integrate-original-surface-area: Integrate original surface area</summary>

Reuse mesh area for meshes, meshed BReps and Elements.

Data flow: original geometry → placed source → measure or refusal.

### 1. `src/measure_area.rs`

Type the complete original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/83ze-integrate-original-surface-area-1.rs"
```

### 2. `src/lib.rs`

Register surface area after curve length; reuse the earlier orientation owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod measure_length;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/83ze-integrate-original-surface-area-dedup-registration.rs" skip=25
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/measure_area.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use crate::measure_math::compute_mesh_area;
use session_rust::{BRep,Geometry,NurbsSurface,Xform};
use session_rust::element::ElementGeometry;
/// World area of a surface kind placed by `place`, all faces or only `face`; None for other kinds.
pub fn compute_area(geometry: &Geometry, place: &Xform, face: Option<usize>) -> Option<f64> {
    match geometry {
        Geometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
        Geometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
        Geometry::NurbsSurface(surface) => Some(compute_surface_area(&surface.transformed(place))),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => Some(compute_mesh_area(mesh, place, face)),
            ElementGeometry::BRep(brep) => Some(compute_brep_area(brep, place, face)),
            ElementGeometry::None => None,
        },
        _ => None,
    }
}

/// World area of a BRep's faces as the viewer meshes them; planar faces are exact.
fn compute_brep_area(brep: &BRep, place: &Xform, face: Option<usize>) -> f64 {
    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    meshes
        .iter()
        .enumerate()
        .filter(|(index, _)| face.is_none_or(|face| face == *index))
        .map(|(_, mesh)| compute_mesh_area(mesh, place, None))
        .sum()
}

const GAUSS: [(f64, f64); 8] = [
    (-0.9602898564975363, 0.1012285362903763),
    (-0.7966664774136267, 0.2223810344533745),
    (-0.5255324099163290, 0.3137066458778873),
    (-0.1834346424956498, 0.3626837833783620),
    (0.1834346424956498, 0.3626837833783620),
    (0.5255324099163290, 0.3137066458778873),
    (0.7966664774136267, 0.2223810344533745),
    (0.9602898564975363, 0.1012285362903763),
]; // 8-point Gauss-Legendre nodes and weights on [-1, 1]

/// Area of a NURBS surface: |Su x Sv| integrated per knot span with 8 x 8 Gauss points.
fn compute_surface_area(surface: &NurbsSurface) -> f64 {
    let us = surface.get_span_vector(0);
    let vs = surface.get_span_vector(1);
    let mut total = 0.0;

    for u in us.windows(2) {
        let (uh, um) = ((u[1] - u[0]) * 0.5, (u[1] + u[0]) * 0.5); // half width, middle

        for v in vs.windows(2) {
            let (vh, vm) = ((v[1] - v[0]) * 0.5, (v[1] + v[0]) * 0.5);

            for (su, wu) in GAUSS {
                for (sv, wv) in GAUSS {
                    let derivatives = surface.evaluate(um + uh * su, vm + vh * sv, 1); // [S, Sv, Su]

                    if let [_, dv, du] = derivatives.as_slice() {
                        total += wu * wv * uh * vh * du.cross(dv).magnitude();
                    }
                }
            }
        }
    }

    total
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/83ze-integrate-original-surface-area-3.rs" skip=2714
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 83ze-integrate-original-surface-area
npm --prefix ../session_tests run course -- check 83ze-integrate-original-surface-area
npm --prefix ../session_tests run course -- save 83ze-integrate-original-surface-area
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

Surface-area integration can measure original surface data before the measurement command connection.

- **Final Lesson Experiment:** Compare uniform scaling with translation. Check the planar patch and exact cylinder surface against their known areas.

Continue with [Measurements, temporary marks and retained CAD pieces](../../consolidation/84-measure-original-curves-surfaces-and-closed-solids.md).
