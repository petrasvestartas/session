# 9.4 · Periodic seams and face-edge direction

Keep both parameter-space uses of a seam without drawing a false open border.

## Ownership and data flow

One geometric seam edge can have distinct pcurve uses on a face. Retain both UV addresses even when their world points/normals coincide across a periodic domain. Boundary classification must distinguish the seam from an exposed border. Matching shared face-edge directions then provides the opposed walks needed to orient connected faces coherently, instead of deduplicating away essential parameter ownership.

## The maintained implementation

app/walk/brep_edges.rs retains and filters maintained edge chains; brep_orient.rs matches incident face directions and signs. Session kernel face/edge-use data is the source of seam identity. The course tests stage this topology contract; current actual seam and isoline images still require feature-specific acceptance.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [app/walk/brep.rs](../../../src/app/walk/brep.rs), [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs), [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs), [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs), [engine/gpu/faces.rs](../../../src/engine/gpu/faces.rs).

## Build the capability

Apply these implementation checkpoints in order in the same `workspace/journey` project. Expand a block while typing; each retains its original complete Type section, including its snippet includes and locate/replace context. Intermediate IDs remain recoverable checkpoints within this lesson.

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

## Run, check and save

From `session_viewer`, after restoring any experimental edits:

```sh
npm --prefix ../session_tests run course -- dependencies 58za-match-original-face-edge-directions
npm --prefix ../session_tests run course -- check 58za-match-original-face-edge-directions
npm --prefix ../session_tests run course -- save 58za-match-original-face-edge-directions
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running so saving rebuilds it. `check` compares your typed source with the selected endpoint without replacing your project; `save` records that source for recovery. The commands are learner instructions, not a claim that this consolidation batch has rerun GPU or browser acceptance. [Checkpoint recovery](../recovery.md) explains restoring a saved state.

## Result and experiment

The earlier blue parametric roof stays visible while shared orientation helpers compile.

Trace a seam whose two face meshes use different vertex keys. Identify both nearest-match tie breakers.

Recorded screenshot paths are identified in the chapter coverage map as historical, pending fresh acceptance. No uninspected screenshot is embedded here. Source comparison, a successful build and visual behavior are separate checks.

Continue with [Oriented CAD solids, original picks and empty bores](59-orientation.md).
