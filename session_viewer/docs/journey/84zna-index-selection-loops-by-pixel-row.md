# 16.4.28 · Index selection loops by pixel row

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 28–56 minutes.** [Estimate](typing-load.md).

Build a row index for the even-odd point test. Keep rectangles in the same Region owner so later crossing selection can reuse their bounds.

## Type

Continue from [Connect original trim recipes](84zzm-connect-original-trim-recipes.md). [Save or recover your work](recovery.md).

### 1. `src/selection_region.rs`

Type the shared selection region and row index.

Create the file and type:

```rust
--8<-- "journey/code/84zna-index-selection-loops-by-pixel-row-1.rs"
```

### 2. `src/lib.rs`

Register the selection owner.

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

pub mod selection_region;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/84zna-index-selection-loops-by-pixel-row-2.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.28
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

![Native renderer output for index selection loops by pixel row.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD samples → screen region → window or crossing result.



Why exclude the upper endpoint of each crossing edge?

Counting a shared vertex twice would change whether its row is inside.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Sketch a four-corner rectangle and calculate the crossings at the centre of one pixel row.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.28
npm --prefix ../session_tests run course -- save 16.4.28
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
