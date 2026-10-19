# 15.1.44 · Find extrusion axes and cap eligibility

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

Reuse the original loop normal and planarity helpers to choose an extrusion axis. Orient that axis with the construction plane. Classify open, bent and flat outlines before building a cap, keeping the reason separate from geometry construction.

## Type

Continue from [Align original sections and loft through them](77zzi-align-original-sections-and-loft-through-them.md). [Save or recover your work](recovery.md).

### 1. `src/extrude_geometry.rs`

Type the shared extrusion axis, orientations and cap eligibility.

Create the file and type:

```rust
--8<-- "journey/code/77zzj-find-extrusion-axes-and-cap-eligibility-1.rs"
```

### 2. `src/lib.rs`

Compile the original extrusion helpers before their command registration.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}

mod shader_check;
#[cfg(test)]
mod cloud_shader_contract;

pub mod surfacing;
```

</details>

After that block, type:

```rust

pub mod extrude_geometry;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzj-find-extrusion-axes-and-cap-eligibility-2.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.44
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

![Native renderer output for find extrusion axes and cap eligibility.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curves → offset and cap rules → shared BRep topology.



Why retain the reason an outline cannot be capped?

An open extrusion may still be valid. The command can report the missing cap without discarding its side geometry.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace an open curve, a bent loop and an offset parallel to the outline plane.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.44
npm --prefix ../session_tests run course -- save 15.1.44
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original extrusion helpers are preparatory. The existing command registry connects the completed recipe in 78; earlier helper checkpoints keep the browser drawing unchanged.



[Full validation scope](release.md).

</details>
