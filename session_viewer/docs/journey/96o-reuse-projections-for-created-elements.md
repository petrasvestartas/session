# 19.2.7 · Use the same projection path for created Elements

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 11–22 minutes.** [Estimate](typing-load.md).

Use the same feature projection for newly created Elements.

## Type

Continue from [Connect imported Elements to their feature hierarchy](96n-connect-imported-interaction-hierarchies.md). [Save or recover your work](recovery.md).

### 1. `src/element_adopt.rs`

Replace serialized display copies with the shared feature projection and preserve independent child owners.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn adopt(scene: &mut Scene, made: Vec<Made>, consumed: &[ObjectId]) -> Result<Vec<ObjectId>, &'static str> {
    let mut ids = Vec::new();
    for item in made {
        let id = crate::element_original::insert(scene, item.source.clone())?;
        ids.push(id);
        let mut attributes = Vec::new();
        let mut interactions = Vec::new();
        if let Some(plane) = item.plane {
            attributes.push(crate::plane_original::insert(scene, Rc::new(plane))?);
        }
        // Own axes and sections are attributes; contact, joint, cut and drill geometry comes from interactions.
        for feature in &item.source.features {
            let interaction = matches!(feature.feature_type.as_str(), "contact" | "joint" | "cut" | "drill");
            for outline in &feature.outlines {
                let mut source = outline.clone();
                source.set_guid(uuid::Uuid::new_v4().to_string());
                source.name = format!("{} {}", feature.feature_type, feature.name);
                let piece = scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(source))?)?;
                if interaction { interactions.push(piece); } else { attributes.push(piece); }
            }
        }
        if !attributes.is_empty() {
            Rc::make_mut(&mut scene.view).hide(&attributes, true);
            crate::layer_edit::attach_attributes(scene, id, attributes, "attributes")?;
        }
        if !interactions.is_empty() {
            crate::layer_edit::attach_attributes(scene, id, interactions, "interactions")?;
        }
    }
    for id in scene.with_descendants(consumed) { scene.remove(id); }
    Ok(ids)
}

fn group_ids(scene: &Scene, name: &str) -> Vec<ObjectId> {
    scene.forest.documents.iter().flat_map(|doc| {
        doc.nodes.iter().enumerate().filter(move |(_, node)| node.source.name == name && node.readonly)
            .flat_map(|(index, node)| doc.nodes[index + 1..node.end].iter().filter_map(|node| node.object))
    }).collect()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96o-reuse-projections-for-created-elements-01.rs"
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Created Elements use the existing projection owner, with independent child visibility preserved.

### Recorded visual result

**Previous native render — current verification pending.**

The existing original comparison scene is unchanged; this step prepares imported Element ownership.

![Native renderer output for use the same projection path for created elements.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Keep original base-plane attributes as source nodes, then add generated feature pieces through the shared adapter. Visibility collection stops at independent editable child Elements and gathers each read-only piece once, so a parent’s Attributes command cannot hide a child’s geometry.

original source → bounded feature plan → derived display → source-preserving archive.



Why stop visibility traversal at an editable child?

That child owns its geometry and its own attribute/interactions groups. The parent’s attribute switch must not become a second visibility owner for the child object.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Toggle Attributes and Interactions separately on a document with a nested child Element. Confirm that the child remains visible and editable.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.7
npm --prefix ../session_tests run course -- save 19.2.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This bridge extends the handwritten viewer. Compare the full interaction type and source-name grouping with maintained `app/interactions.rs` and `app/hierarchy.rs`. Native, browser and hardware evidence remain separate; see [local checks](local-checks.md).



[Full validation scope](release.md).

</details>
