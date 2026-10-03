# 28 · Prepare a display from an owned source mesh

**Combined study estimate: 1–2 hours.** Includes reading, typing, reasoning and experiments.

**Typing estimate: 22–43 minutes.** 60 added or changed lines; unchanged context is excluded. [How this is estimated](typing-load.md).

**Today:** Give source geometry a shared owner and prepare its display before committing an object.

**Follow:** Kernel mesh → PreparedMesh → shared source geometry plus validated display arrays.

Introduce `PreparedMesh` between source construction and scene insertion. It retains the kernel mesh and a validated derived display. The live scene still uses its earlier insertion path; the next lessons connect this boundary.

`Rc` shares immutable kernel geometry. Optional `Source` records imported-session provenance; generated geometry still has a source mesh without an imported file.

Initialise the kernel mesh's lazy GUID before sharing it, then run the display adapter. `?` rejects invalid display data before consuming an object ID or history entry.

Tests preserve a double coordinate that rounds in the float display, along with name and visibility/locking flags. A finite double too large for a float must fail preparation. Retaining flags does not yet implement their interaction policies.

![An owned kernel mesh is prepared into source ownership and a derived display before insertion.](../illustrations/journey-28.svg)

## Type the change

Continue from [Prove placement and history agree](27d-history.md). Save your own work first: `npm --prefix ../session_tests run course -- save before-28-record` (from `session_viewer`).

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

## Run and look

From `session_viewer`, enter your project folder:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j4
REGEN_PROTO=0 CARGO_BUILD_JOBS=4 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. If Trunk is already running in this project, leave it running; it rebuilds when you save.

Run the source-record checks below. The original double coordinate remains exact while the display uses floats. This preparation type does not change the browser picture yet.

**Verified checkpoint in Chrome.**

![Actual browser result: Prepare a display from an owned source mesh.](../screenshots/journey/28-record-browser.png)

[What this screenshot checks](release.md).

Run the state checks from your project folder:

```sh
REGEN_PROTO=0 cargo test --lib --locked -j4
```

<details>
<summary>Optional experiment</summary>

Change the precision check’s coordinate to 123456789.125 and predict the source and float values. Restore it. Explain why a clone of Rc keeps one source allocation while a float display still needs its own arrays.

</details>

## Explain the change

Why must a save read the source mesh instead of the displayed vertex array?

<details>
<summary>Compare your explanation</summary>

The display array uses floats, triangle indices and draw colours. The kernel mesh keeps double coordinates, original topology, names, visibility and locking. PreparedMesh owns the kernel mesh and derives the display from it, so failure happens before the scene takes ownership.

</details>

## Keep your working result

Return to `session_viewer` in a second terminal. Restore experimental edits before comparing:

```sh
npm --prefix ../session_tests run course -- check 28-record
npm --prefix ../session_tests run course -- save 28-record
```

Check compares your typed source; run the focused check above for behavior. [Recover your work](recovery.md).

<details>
<summary>Where this fits in the finished viewer</summary>

The production viewer retains kernel source objects separately from upload tables. PreparedMesh makes the source-to-display boundary explicit; import and generated-object insertion adopt it next.

</details>

<details>
<summary>Verification notes and browser acceptance</summary>

Prepare a display from an owned source mesh. These commands run in the actual dock; kernel ownership is checked separately in Rust.

[Full validation scope](release.md).

</details>
