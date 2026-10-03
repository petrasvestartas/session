# 28 · Prepare a display from an owned source mesh

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Introduce `PreparedMesh` between source construction and scene insertion. It retains the kernel mesh and a validated derived display. The live scene still uses its earlier insertion path; the next lessons connect this boundary.

`Rc` shares immutable kernel geometry. Optional `Source` records imported-session provenance; generated geometry still has a source mesh without an imported file.

## Type

Continue from [Prove placement and history agree](27d-history.md). [Save or recover your work](recovery.md).

### 1. `src/prepared.rs`

Create the transit owner. Keep original kernel data beside the display prepared from it.

Create the file and type:

```rust
--8<-- "journey/code/28-record-01.rs"
```

### 2. `src/lib.rs`

Expose the preparation boundary without changing the running browser adapter.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod mesh;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28-record-02.rs"
```

### 3. `src/prepared_tests.rs`

Compare original doubles with the display, and retain name, flags and GUID through preparation.

Create the file and type:

```rust
--8<-- "journey/code/28-record-03.rs"
```

### 4. `src/lib.rs`

Compile the source checks in the native test build.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod move_tests;
```

</details>

Replace that block with:

```rust
--8<-- "journey/code/28-record-04.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run the source-record checks below. The original double coordinate remains exact while the display uses floats. This preparation type does not change the browser picture yet.

**Verified checkpoint in Chrome.**

![Actual browser result: Prepare a display from an owned source mesh.](../screenshots/journey/28-record-browser.png)

[Verification scope](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Code explanation and diagram</summary>

Initialise the kernel mesh's lazy GUID before sharing it, then run the display adapter. `?` rejects invalid display data before consuming an object ID or history entry.

Tests preserve a double coordinate that rounds in the float display, along with name and visibility/locking flags. A finite double too large for a float must fail preparation. Retaining flags does not yet implement their interaction policies.

Kernel mesh → PreparedMesh → shared source geometry plus validated display arrays.

![An owned kernel mesh is prepared into source ownership and a derived display before insertion.](../illustrations/journey-28.svg)

Why must a save read the source mesh instead of the displayed vertex array?

The display array uses floats, triangle indices and draw colours. The kernel mesh keeps double coordinates, original topology, names, visibility and locking. PreparedMesh owns the kernel mesh and derives the display from it, so failure happens before the scene takes ownership.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Change the precision check’s coordinate to 123456789.125 and predict the source and float values. Restore it. Explain why a clone of Rc keeps one source allocation while a float display still needs its own arrays.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 28-record
npm --prefix ../session_tests run course -- save 28-record
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The production viewer retains kernel source objects separately from upload tables. PreparedMesh makes the source-to-display boundary explicit; import and generated-object insertion adopt it next.

Prepare a display from an owned source mesh. These commands run in the actual dock; kernel ownership is checked separately in Rust.

[Full validation scope](release.md).

</details>
