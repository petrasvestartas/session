# 9.2 · Shared CAD boundaries across incident faces

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 9.2** (Redraft `6.2`) · **3 checkpoints (`56-chains` → `56b-boundaries`)**

> **Learning Goal:** Reuse constrained-label and isoparametric lookup for each original edge.

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
original edge use → face-mesh samples → ordered CAD boundary chain.
validated BRep → shared boundary tessellation → face meshes → source-owned edge chains.
shared boundary chain → neighbour position equality → unchanged source → released owner.
```

- Reuse constrained-label and isoparametric lookup for each original edge.
- Mesh incident faces with shared boundary constraints.
- Prove straight and circular shared boundaries.
- **Why it works (Why use the actual face mesh for a boundary?):** The boundary must coincide with the triangles already meshed on its surface while retaining original CAD edge identity.

### Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `Browser`)

| Layer | Construct | First-Year Student Explanation |
| :--- | :--- | :--- |
| **Rust (`session_rust`)** | `BRep` Faces, Outer/Inner Trim Loops & Edge Adjacency | Validates topological loop closure and manifold edges so trimmed CAD surfaces and analytical boundary curves stay synchronized. |
| **Rust / WGSL** | Crease-Split Vertex Normals vs. Smooth `G1` Normals | Averages vertex normals across smooth surface patches (`angle < crease_threshold`) and splits vertices along sharp mechanical creases for crisp shading. |
| **WebGPU (`Rg32Uint`)** | Unified `[brep_owner_id, face_or_edge_sub_id]` Picking | Both shaded B-Rep face triangles and boundary ribbon wires write the same B-Rep owner ID plus sub-element index for whole-solid or sub-face/edge selection. |

**Production source (`session_viewer/src`):** [app/walk/brep.rs](../../../../src/app/walk/brep.rs), [app/walk/brep_edges.rs](../../../../src/app/walk/brep_edges.rs), [app/walk/brep_orient.rs](../../../../src/app/walk/brep_orient.rs), [app/walk/brep_shapes.rs](../../../../src/app/walk/brep_shapes.rs), [engine/gpu/faces.rs](../../../../src/engine/gpu/faces.rs).

## Implementation Checkpoints

Apply these checkpoints in order inside `workspace/journey`. Expand each block to view the Rust/WGSL/HTML edits:

<a id="checkpoint-56-chains"></a>

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

## Run, Check, and Save

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- dependencies 56b-boundaries
npm --prefix ../session_tests run course -- check 56b-boundaries
npm --prefix ../session_tests run course -- save 56b-boundaries
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Use [checkpoint recovery](../../recovery.md) to restore any saved checkpoint.

## Expected Visual Result & Experiment

- **Try this:** Read the box and circular-rim fixtures beside the shared boundary collector. Trace one recorded edge into both incident face meshes, then identify the malformed source rejected before meshing. The browser display is unchanged; running the native fixtures is optional.

Continue with [Trimmed curved faces with real open holes](57-parameters.md).
