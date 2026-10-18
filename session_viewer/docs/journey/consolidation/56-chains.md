# 9.2 · Shared CAD boundaries across incident faces

Mesh adjacent original faces with one coherent shared boundary chain.

## Ownership and data flow

Read ordered boundary samples from each original edge use, including isoparametric and constrained-trim labels introduced immediately before this lesson. Tessellation must constrain both incident faces to the shared geometric edge so their boundaries agree instead of approximating independently and cracking. Keep source ownership separate from transient mesher maps; straight and circular witnesses check neighbour-position equality and release.

## The maintained implementation

app/walk/brep_edges.rs edge_chains and display_chain derive maintained edge presentation; app/walk/brep.rs consumes original BRep tessellation and maps it to face/edge lanes. The kernel mesher remains authoritative for shared constraints; this consolidation does not replace tessellation or invent a parallel CAD renderer.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/walk/brep.rs](../../../src/app/walk/brep.rs), [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs), [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs), [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs), [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

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

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 56b-boundaries
npm --prefix ../session_tests run course -- check 56b-boundaries
npm --prefix ../session_tests run course -- save 56b-boundaries
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The existing parametric roof remains visible while shared CAD boundary constraints are introduced. In the supplied straight-box and circular-rim fixtures, trace one original edge chain into both incident face meshes. Matching world samples must agree while retaining distinct face-local ownership; malformed source must be rejected before meshing. This checkpoint does not add a new browser solid command.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Trimmed curved faces with real open holes](57-parameters.md).
