# 9.4.3 · Match original face edge directions

**Chapter 9 · CAD surfaces and topology · Section 9.4 · Seams and face directions** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Write the shared halfedge, nearest-vertex and edge-direction helpers once. Match positions and directions across independently keyed face meshes; retain deterministic vertex-key tie breaking.

## Type

Continue from [Prove seam continuity and suppress false open borders](58a-periodic.md). [Save or recover your work](recovery.md).

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

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The earlier blue parametric roof stays visible while shared orientation helpers compile.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for match original face edge directions.](../screenshots/journey/55za-read-original-edge-use-samples.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [9.1.9 · Read original edge-use samples](55za-read-original-edge-use-samples.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



shared edge → matched face vertices → opposed walks.



Why compare positions instead of vertex keys across faces?

Each face mesh owns its own keys. Match the shared edge geometrically before comparing occupied halfedge directions.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a seam whose two face meshes use different vertex keys. Identify both nearest-match tie breakers.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 9.4.3
npm --prefix ../session_tests run course -- save 9.4.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Prepare the one orientation owner reused by display and measurement.



[Full validation scope](release.md).

</details>
