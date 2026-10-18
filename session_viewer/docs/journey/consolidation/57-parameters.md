# 9.3 · Trimmed curved faces with real open holes

Keep displayed triangles tied to source surface parameters while respecting outer and inner wires.

## Ownership and data flow

A triangle vertex maps through its original face to UV and exact surface point/normal. Constrained tessellation must accept the outer region and reject the hole interior; a filled patch is a failure even if source identity and buffers are valid. Retained parameter samples support checking geometry against the original surface without replacing it with the mesh. History retains original ownership and temporary mesher maps are released after display preparation.

## The maintained implementation

app/walk/brep.rs prepares maintained face meshes, while brep_edges.rs consumes actual boundary chains. Original trimming and meshing belong to session_rust BRep/NurbsSurface implementations. Current feature-specific CAD visual acceptance is pending; the existence of these instructional proof checkpoints is not fresh evidence of an empty rendered hole.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/walk/brep.rs](../../../src/app/walk/brep.rs), [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs), [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs), [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs), [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

<a id="checkpoint-57-parameters"></a>

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

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 57a-trims
npm --prefix ../session_tests run course -- check 57a-trims
npm --prefix ../session_tests run course -- save 57a-trims
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The browser drawing remains the preceding specimen while trimmed-wave and hole fixtures exercise the mesher already written. Inspect the retained UV samples and both outer and inner wires. Confirm that the constrained triangles exclude the hole interior; this source-level result is separate from the later Example CAD visible bore witness, whose current acceptance remains pending.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Periodic seams and face-edge direction](58-uses.md).
