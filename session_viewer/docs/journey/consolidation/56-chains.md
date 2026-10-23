# 9.2 · Shared CAD boundaries across incident faces

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 9.2** (Redraft `6.2`) · **3 checkpoints (`56-chains` → `56b-boundaries`)** · **Study time: 1.5–3 h**

> **What You Will Learn & Build:** Reuse constrained-label and isoparametric lookup for each original edge. By the final checkpoint (`56b-boundaries`), you will prove straight and circular shared boundaries.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 9.2 Execution Flow: Shared CAD boundaries across incident faces</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (3 Checkpoints: 56-chains → 56b-boundaries)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">original edge use</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">face-mesh samples</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">ordered CAD</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">boundary chain</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">shared boundary</text>
    <text x="584" y="124" text-anchor="middle" font-size="11" fill="#111111">chain</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">neighbour position</text>
    <text x="749" y="124" text-anchor="middle" font-size="11" fill="#111111">equality</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 9.2:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/cad_edges.rs  ·  src/cad_boundary.rs  ·  src/lib.rs  ·  src/cad_boundary_tests.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">3 Checkpoints: 56-chains → 56b-boundaries — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
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

**Authoritative production source (`session_viewer/src`):** [app/walk/brep.rs](../../../src/app/walk/brep.rs), [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs), [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs), [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs), [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`56-chains`](#checkpoint-56-chains) | Reuse constrained-label and isoparametric lookup for each original edge. | `src/cad_edges.rs` | `original edge use → face-mesh samples → ordered CAD boundary chain` |
| **2** | [`56a-shared`](#checkpoint-56a-shared) | Mesh incident faces with shared boundary constraints. | `src/cad_boundary.rs`, `src/lib.rs` | `validated BRep → shared boundary tessellation → face meshes → source-owned edge chains` |
| **3** | [`56b-boundaries`](#checkpoint-56b-boundaries) | Prove straight and circular shared boundaries. | `src/cad_boundary_tests.rs`, `src/lib.rs` | `shared boundary chain → neighbour position equality → unchanged source → released owner` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-56-chains"></a>

### Step 1 of 3 · Connect original CAD edge chains (`56-chains`)

- **Goal:** Reuse constrained-label and isoparametric lookup for each original edge.
- **Data Flow Trace:** `original edge use → face-mesh samples → ordered CAD boundary chain`
- **Files You Touch:** `src/cad_edges.rs` (*Extend this original owner with today’s complete helper.*)

Reuse constrained-label and isoparametric lookup for each original edge. Skip degenerated edges and retain the owner face, ordered mesh keys and other incident face.

> **Self-Check Question:** *Why use the actual face mesh for a boundary?*  
> **Answer:** The boundary must coincide with the triangles already meshed on its surface while retaining original CAD edge identity.

- **What You Should See:** The boundary lookup helpers are assembled for original BRep chains. The existing drawing remains available.
- **Hands-On Lab Experiment:** Trace one box edge and one cylinder rim through incident face uses. Compare the returned owner and neighbouring face identities.

<details>
<summary>Implementation 56-chains: Connect original CAD edge chains</summary>

Reuse constrained-label and isoparametric lookup for each original edge.

Data flow: original edge use → face-mesh samples → ordered CAD boundary chain.

### 1. `src/cad_edges.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        ordered.dedup_by(same_parameter);
        let mut keys = Vec::with_capacity(ordered.len());

        for (_, key) in ordered {
            keys.push(key);
        }

        return Some(keys);
    }

    None
}
```

</details>

After that block, type:

```rust
pub fn edge_chains(b: &BRep, fms: &[Mesh]) -> Vec<Option<EdgeChain>> {
    let mut out = Vec::with_capacity(b.m_edges.len());

    for (ei, e) in b.m_edges.iter().enumerate() {
        if e.degenerated {
            out.push(None);
            continue;
        }

        let uses = b.edge_faces(ei);
        let mut found: Option<EdgeChain> = None;

        for (k, u) in uses.iter().enumerate() {
            let eu = EdgeUse {
                edge: ei,
                face: u.index as usize,
                orientation: u.orientation,
            };
            let keys = match constrained_chain(&fms[eu.face], ei) {
                Some(keys) => keys,
                None => match iso_chain(b, &fms[eu.face], &eu) {
                    Some(keys) => keys,
                    None => continue,
                },
            };
            let mut other = None;

            for (j, candidate) in uses.iter().enumerate() {
                if j != k && candidate.index as usize != eu.face {
                    other = Some(candidate.index as usize);
                    break;
                }
            }

            found = Some(EdgeChain {
                edge: ei,
                face: eu.face,
                keys,
                other,
            });
            break;
        }

        out.push(found);
    }

    out
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/56-chains-1.rs"
```

</details>

</details>

<a id="checkpoint-56a-shared"></a>

### Step 2 of 3 · Mesh incident faces with shared boundary constraints (`56a-shared`)

- **Goal:** Mesh incident faces with shared boundary constraints.
- **Data Flow Trace:** `validated BRep → shared boundary tessellation → face meshes → source-owned edge chains`
- **Files You Touch:** `src/cad_boundary.rs` (*Validate references and retain one source-owned preparation of all incident face meshes.*) · `src/lib.rs` (*Register the source component.*)

Validate topology and surface control storage, then ask the kernel for all face meshes together at the current five-degree and 0.001 chord quality. Retain original source ownership beside their coherent chains.

> **Self-Check Question:** *Does sharing a boundary require sharing normals across its two faces?*  
> **Answer:** No. Incident faces agree on boundary positions while retaining their own normals. A sharp corner keeps its shading discontinuity, and a curved cap and wall meet without a gap.

- **Hands-On Lab Experiment:** For every box edge, compare all chain positions with vertices on its neighbouring face. Repeat for both circular cylinder rims and inspect their distinct normals.

<details>
<summary>Implementation 56a-shared: Mesh incident faces with shared boundary constraints</summary>

Mesh incident faces with shared boundary constraints.

Data flow: validated BRep → shared boundary tessellation → face meshes → source-owned edge chains.

### 1. `src/cad_boundary.rs`

Validate references and retain one source-owned preparation of all incident face meshes.

Create the file and type:

```rust
--8<-- "journey/code/56a-shared-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod cad_edges;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/56a-shared-02.rs" skip=20
```

</details>

<a id="checkpoint-56b-boundaries"></a>

### Step 3 of 3 · Prove straight and circular shared boundaries (`56b-boundaries`)

- **Goal:** Prove straight and circular shared boundaries.
- **Data Flow Trace:** `shared boundary chain → neighbour position equality → unchanged source → released owner`
- **Files You Touch:** `src/cad_boundary_tests.rs` (*Copy independent incident-face position, source ownership and malformed-input checks.*) · `src/lib.rs` (*Register the source component.*)

Copy checks that compare boundary positions against both incident faces, preserve original BRep serialization and verify that preparation releases its source owner.

> **Self-Check Question:** *What must remain unchanged when display quality changes?*  
> **Answer:** The original BRep, its face and edge topology, control values and source identity remain unchanged. Mesh vertex keys and display samples belong to the derived preparation.

- **Hands-On Lab Experiment:** Read the box and circular-rim fixtures beside the shared boundary collector. Trace one recorded edge into both incident face meshes, then identify the malformed source rejected before meshing. The browser display is unchanged; running the native fixtures is optional.

<details>
<summary>Implementation 56b-boundaries: Prove straight and circular shared boundaries</summary>

Prove straight and circular shared boundaries.

Data flow: shared boundary chain → neighbour position equality → unchanged source → released owner.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/cad_boundary_tests.rs`

Copy independent incident-face position, source ownership and malformed-input checks.

Copy this supplied file:

```text
--8<-- "journey/code/56b-boundaries-01.rs"
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

pub mod cad_boundary;
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/56b-boundaries-02.rs" skip=23
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 56b-boundaries
npm --prefix ../session_tests run course -- check 56b-boundaries
npm --prefix ../session_tests run course -- save 56b-boundaries
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Read the box and circular-rim fixtures beside the shared boundary collector. Trace one recorded edge into both incident face meshes, then identify the malformed source rejected before meshing. The browser display is unchanged; running the native fixtures is optional.

Continue with [Trimmed curved faces with real open holes](57-parameters.md).
