# 15.1.35 · Describe curve gathering answers

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 18–35 minutes.** [Estimate](typing-load.md).

Describe ordered curve questions and typed answers. Keep original curves and their placed corner data separate from command progress.

## Type

Continue from [Exercise original copy transforms history and exact persistence](77o-exercise-original-copy-transforms-history-and-exact-persistence.md). [Save or recover your work](recovery.md).

### 1. `src/gather_input.rs`

Type the shared curve-gathering declarations.

Create the file and type:

```rust
--8<-- "journey/code/77zza-describe-curve-gathering-answers-1.rs"
```

### 2. `src/lib.rs`

Register these helpers for the later kernel surfacing recipes.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod transform_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77zza-describe-curve-gathering-answers-2.rs" skip=34
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.35
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

![Native renderer output for describe curve gathering answers.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>



original curve picks → ordered answers → Session kernel recipe.



Why preserve both a NURBS curve and optional corners?

Kernel builders use the curve; line and polyline previews can retain their exact corners.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a curve-pick step followed by two point answers. Identify which values belong to original geometry.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.35
npm --prefix ../session_tests run course -- save 15.1.35
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
