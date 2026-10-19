# 19.2.8 · Build an original interaction specimen

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 24–48 minutes.** [Estimate](typing-load.md).

Build a small original Session that exercises the complete bridge: a named contact, an unnamed feature-producing relation, an unmatched cut, and a nested editable child Element.

## Type

Continue from [Use the same projection path for created Elements](96o-reuse-projections-for-created-elements.md). [Save or recover your work](recovery.md).

### 1. `src/interaction_example.rs`

Author the original geometry, graph and feature associations used by the exercise.

Create the file and type:

```rust
--8<-- "journey/code/96p-build-an-imported-interaction-specimen-01.rs"
```

### 2. `src/lib.rs`

Expose the specimen to native examples and tests.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod inventory_tests;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod interaction_example;

#[cfg(test)]
mod inventory_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96p-build-an-imported-interaction-specimen-02.rs"
```

</details>

<details>
<summary>Optional authoring helper</summary>

Copy this supplied authoring helper once. Use the linked data or recorded comparison for the browser exercise; you do not need to compile or run this native helper.

### 3. `examples/sample.rs`

Supply this optional course-author fixture generator; browser exercises use the linked data files or recorded comparison.

<details>
<summary>Locate the existing block</summary>

```text
fn main() -> std::io::Result<()> {
    std::fs::write("sample.pb", viewer_journey::specimen::precise_bytes())
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96p-build-an-imported-interaction-specimen-03.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Download the [supplied interaction sample.pb](fixtures/interaction-96p.pb). Type Open in the browser and choose it. Expand the host: inspect the InteractionContactFace seam, InteractionFeatureSolid connector and unclaimed cut. Keep the child selectable and compare Element Attributes On with Element Interactions Off.

### Recorded visual result

**Previous native render — current verification pending.**

The existing original comparison scene is unchanged; this step prepares imported Element ownership.

![Native renderer output for build an original interaction specimen.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The sample keeps unknown interaction payloads and original feature GUIDs. Its Open example uses these exact serialized originals; generated display paths are created only by the viewer.

original source → bounded feature plan → derived display → source-preserving archive.



Why include an unmatched cut and a child inside attributes?

They expose two common shortcuts: silently dropping features that lack graph matches, and marking every descendant of attributes read-only.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Open sample.pb, expand Layers All, and follow host → interactions → full relation type/name → feature → piece. Locate the independently editable child under attributes.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.8
npm --prefix ../session_tests run course -- save 19.2.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This bridge extends the handwritten viewer. Compare the full interaction type and source-name grouping with maintained `app/interactions.rs` and `app/hierarchy.rs`. Native, browser and hardware evidence remain separate; see [local checks](local-checks.md).

Run this checkpoint’s native tests, then continue to the connected imported-interaction exercise. Preparation checkpoints keep the existing scene until the import adapter is connected.

[Full validation scope](release.md).

</details>
