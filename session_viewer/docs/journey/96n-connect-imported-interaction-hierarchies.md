# 19.2.6 · Connect imported Elements to their feature hierarchy

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 6–12 minutes.** [Estimate](typing-load.md).

Connect projection only after the candidate import has retained its original header, built its hierarchy and resolved original placements.

## Type

Continue from [Project original outlines without changing their coordinates](96m-project-original-feature-outlines.md). [Save or recover your work](recovery.md).

### 1. `src/scene_document.rs`

Connect projections only after original tree placement and source retention are ready.

<details>
<summary>Locate the existing block</summary>

```rust
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
    let elements = owners.iter().filter_map(|(guid, id, _)|
        matches!(source.document.lookup.get(guid), Some(session_rust::Geometry::Element(_))).then_some(*id))
        .collect::<std::collections::HashSet<_>>();
    candidate.forest.classify_elements(key, &elements);
    for (guid, id, _) in &owners {
        if let Some(session_rust::Geometry::Element(element)) = source.document.lookup.get(guid) {
            crate::feature_display::append(&mut candidate, *id, &source.document, element)
                .map_err(str::to_owned)?;
        }
    }
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96n-connect-imported-interaction-hierarchies-01.rs"
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

Accepted imports retain editable children and attach the validated original interaction hierarchy atomically.

### Recorded visual result

**Previous native render — current verification pending.**

The existing original comparison scene is unchanged; this step prepares imported Element ownership.

![Native renderer output for connect imported elements to their feature hierarchy.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Identify original Element owners, classify their editable child boundaries, then append each Element’s feature display using the source Session graph. A failure aborts the candidate; the open scene remains intact.

original source → bounded feature plan → derived display → source-preserving archive.



Why append features to the import candidate?

The original import is already atomic. Invalid outline data or a hierarchy-limit failure can discard the candidate without leaving partially created paths in the user’s drawing.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Corrupt one feature outline with a non-finite coordinate. Verify that import returns an error and preserves the existing scene and its paths.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.6
npm --prefix ../session_tests run course -- save 19.2.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This bridge extends the handwritten viewer. Compare the full interaction type and source-name grouping with maintained `app/interactions.rs` and `app/hierarchy.rs`. Native, browser and hardware evidence remain separate; see [local checks](local-checks.md).



[Full validation scope](release.md).

</details>
