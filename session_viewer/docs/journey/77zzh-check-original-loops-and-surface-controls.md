# 15.1.42 · Check original loops and surface controls

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 25–50 minutes.** [Estimate](typing-load.md).

Find a loop’s unit normal and scale-aware planarity tolerance. Check that a resulting surface is valid and has finite control points before wrapping it as original geometry. Reuse the curve collector instead of another sampling path.

## Type

Continue from [Prepare placed kernel curves](77zzg-prepare-placed-kernel-curves.md). [Save or recover your work](recovery.md).

### 1. `src/surfacing.rs`

Type loop geometry, planarity and finite surface validation.

Create the file and type:

```rust
--8<-- "journey/code/77zzh-check-original-loops-and-surface-controls-1.rs"
```

### 2. `src/lib.rs`

Compile the shared surfacing owner while its recipes are prepared.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;
```

</details>

After that block, type:

```rust

pub mod surfacing;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzh-check-original-loops-and-surface-controls-2.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.42
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

![Native renderer output for check original loops and surface controls.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curves → consistent section frames → validated kernel surface.



Why scale the planarity tolerance by the loop bounds?

A fixed tolerance would behave differently for small and large CAD outlines.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a planar loop, a collapsed loop and a surface with a non-finite control point through their separate checks.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.42
npm --prefix ../session_tests run course -- save 15.1.42
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original surfacing helpers are preparatory. Extrude, revolve and the later surface recipes use this owner; the existing browser drawing is unchanged here.



[Full validation scope](release.md).

</details>
