# 16.4.15 · Describe removable CAD parts

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Represent curve intervals, BRep face groups and mesh sides without replacing the original CAD geometry. Write seam-aware curve intervals and pieces.

## Type

Continue from [Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). [Save or recover your work](recovery.md).

### 1. `src/trim_parts.rs`

Type the original CAD part representation and seam-aware curve helpers.

Create the file and type:

```rust
--8<-- "journey/code/84zza-describe-removable-cad-parts-1.rs"
```

### 2. `src/lib.rs`

Register the shared part owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod measure_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/84zza-describe-removable-cad-parts-2.rs" skip=32
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.15
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for describe removable cad parts.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original CAD geometry → removable parts → retained source geometry.



Why can the last interval end before it starts?

A closed curve wraps through its parameter seam; join its tail and head.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Draw three cuts on a closed curve and trace the last interval through the seam.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.15
npm --prefix ../session_tests run course -- save 16.4.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
