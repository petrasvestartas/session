# 9.1.9 · Read original edge-use samples

**Chapter 9 · CAD surfaces and topology · Section 9.1 · Surface controls and trimming** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Write edge-use and chain records, parameter ordering and exact linear pcurve endpoints. Collect unique face-mesh samples and locate the matching periodic sample.

## Type

Continue from [Draw and pick the original parametric roof](55g-surface.md). [Save or recover your work](recovery.md).

### 1. `src/cad_edges.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
--8<-- "journey/code/55za-read-original-edge-use-samples-1.rs"
```

### 2. `src/lib.rs`

Register the original CAD edge records and lookup helpers.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod surface_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/55za-read-original-edge-use-samples-2.rs" skip=32
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Boundary lookup can read an original edge-use sample without changing the displayed roof.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for read original edge-use samples.](../screenshots/journey/55za-read-original-edge-use-samples.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original edge use → face-mesh samples → ordered CAD boundary chain.



Why retain the face use orientation?

A surface seam may have two parameter curves on the same face. Its oriented use selects the boundary being followed.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the pcurve checks and a periodic endpoint. Compare ordinary parameter ordering with the total order used for constrained labels.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 9.1.9
npm --prefix ../session_tests run course -- save 9.1.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original boundary helpers prepare coherent CAD edge drawing from the existing face mesh.



[Full validation scope](release.md).

</details>
