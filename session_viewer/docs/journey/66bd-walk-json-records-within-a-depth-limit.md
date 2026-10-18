# 12.1.7 · Walk JSON records within a depth limit

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 13–26 minutes.** [Estimate](typing-load.md).

Inspect every object and array in the parsed JSON value without constructing kernel geometry.

## Type

Continue from [Match JSON control layout to declared counts](66bc-match-json-control-layout-to-declared-counts.md). [Save or recover your work](recovery.md).

### 1. `src/source_validate.rs`

Use an explicit bounded stack to find original control records; reject excess nesting and reuse the already-written record validators.

<details>
<summary>Locate the existing block</summary>

```rust
        let stride = json_count(source, "cv_stride", size as u64, 4)?;

        if count != controls.len() || stride != size {
            return Err("JSON curve controls disagree with their declared count or stride".into());
        }

        for control in controls {
            if control.as_array().map(Vec::len) != Some(size) {
                return Err("incomplete JSON curve control".into());
            }
        }

        json_axis(source, "order", count, "nurbsknots")?;
    }

    Ok(())
}
```

</details>

After that block, type:

```rust

/// Check every NURBS record in session JSON before it is parsed.
pub fn json(text: &str) -> Result<(), String> {
    let source: serde_json::Value = serde_json::from_str(text).map_err(json_error)?;
    let mut pending = vec![(&source, 0usize)];

    while let Some((value, depth)) = pending.pop() {
        if depth > 64 {
            return Err("session JSON exceeds 64 nested levels".into());
        }

        match value {
            serde_json::Value::Object(fields) => {
                if fields.contains_key("control_points")
                    && (fields.contains_key("cv_count") || fields.contains_key("cv_count_u"))
                {
                    json_controls(value)?;
                }

                for child in fields.values() {
                    pending.push((child, depth + 1));
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    pending.push((child, depth + 1));
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/66bd-walk-json-records-within-a-depth-limit-01.rs"
```

</details>

### 2. `src/lib.rs`

Expose the pure original-record validator for a browser-console exercise.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod source_validate;
```

</details>

After that block, type:

```rust

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn validate_source_json(text: &str) -> Result<(), JsValue> {
    source_validate::json(text).map_err(|error| JsValue::from_str(&error))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/66bd-walk-json-records-within-a-depth-limit-02.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

In the browser developer console, call window.wasmBindings.validate_source_json(text). A valid control record returns normally; an invalid record throws its validation message without changing the drawing.

### Recorded visual result

**Previous native render — current verification pending.**

The original cloud and local point marker remain unchanged.

![Native renderer output for walk json records within a depth limit.](../screenshots/journey/65za-read-original-manifest-records-and-placements.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Recognize control records by their control_points and declared curve or surface counts. Delegate each recognized record to the existing control-layout validator; do not duplicate its count and knot rules in the traversal.

original record → bounded scalar/address contract → reusable validator → later archive loader.



Why walk nested JSON with an explicit depth budget?

An otherwise small document can nest records deeply enough to exhaust recursive processing. A bounded stack makes that limit explicit.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Sketch a control record nested inside two arrays. Follow stack entries and depths, then predict what happens when a child lies past depth sixty-four.

In the browser console, run `window.wasmBindings.validate_source_json('{"control_points":[[0,0,0],[1,0,0]],"cv_count":2,"dimension":3,"order":2,"nurbsknots":[0,1]}')`. Change cv_count to 3 and predict the rejection. The validator does not import or modify the active drawing.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.7
npm --prefix ../session_tests run course -- save 12.1.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Actual Chrome accepts a valid original JSON control record, rejects mismatched and oversized counts, and preserves drawing identities, placements, CPU history accounting and GPU upload counters.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 12.1.7
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
