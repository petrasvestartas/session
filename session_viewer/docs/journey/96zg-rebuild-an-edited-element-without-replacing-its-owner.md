# 19.4.7 · Rebuild an edited Element without replacing its owner

**Chapter 19 · Integration and original ownership · Section 19.4 · Editing original controls** · [Contents](contents.md)

**Typing: 10–19 minutes.** [Estimate](typing-load.md).

Source deformation already clones the Element and edits its mesh or BRep geometry.

## Type

Continue from [Edit original controls through the source preview](96zf-edit-original-controls-through-the-source-preview.md). [Save or recover your work](recovery.md).

### 1. `src/scene.rs`

Prepare the edited Element first, then retain the original owner identity and presentation.

<details>
<summary>Locate the existing block</summary>

```rust
            Geometry::Plane(source)=>{let prepared=crate::plane_original::prepared(&source)?;let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected a plane source")?;row.mesh=Rc::new(prepared.display);row.original=Some(Geometry::Plane(source));row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};},
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
            Geometry::Element(source)=>{
                let mut staging=Self::empty();
                let prepared=crate::element_original::insert(&mut staging,source)?;
                let mut replacement=staging.objects().iter().find(|row|row.id==prepared).ok_or("Prepared Element is missing")?.clone();
                for vertex in replacement.mesh.vertices(){
                    let point=model.transform_point(&session_rust::Point::new(vertex[0]as f64,vertex[1]as f64,vertex[2]as f64));
                    if (0..3).any(|i|!(point[i]as f32).is_finite()){return Err("Edited Element exceeds display range");}
                }
                let row=self.objects.iter_mut().find(|row|row.id==id).ok_or("Element owner is missing")?;
                replacement.id=row.id;replacement.guid=row.guid.clone();
                replacement.metadata=row.metadata.clone();replacement.model=model;
                *row=replacement;
            },
            Geometry::Plane(source)=>{let prepared=crate::plane_original::prepared(&source)?;let row=self.objects.iter_mut().find(|r|r.id==id).ok_or("Expected a plane source")?;row.mesh=Rc::new(prepared.display);row.original=Some(Geometry::Plane(source));row.editable=crate::edit_source::EditSource::Loaded{geometry:prepared.geometry,source:None};},
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zg-rebuild-an-edited-element-without-replacing-its-owner-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/original_control_tests.rs`

Supply actual original Element, feature source, derived row, tree identity, placement, Undo, Redo and stale-preview checks.

<details>
<summary>Locate the existing block</summary>

```text
 scene,&mut history).unwrap());assert!(crate::source_edit::same(&shared,&crate::source_edit::original(&scene,other).unwrap()));assert!(history.undo(&mut scene));assert!(crate::source_edit::same(&source,&crate::source_edit::original(&scene,first).unwrap()));assert!(history.redo(&mut scene));assert!(!crate::source_edit::same(&source,&crate::source_edit::original(&scene,first).unwrap()));assert!(preview.commit(&mut scene,&mut history).is_err(),"A stale source preview cannot overwrite the edit");
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zg-rebuild-an-edited-element-without-replacing-its-owner-2.rs" skip=500
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/pick_gpu_tests.rs`

Supply actual moved original-control integer pixels, removed old pixels, commit and Undo at both densities.

<details>
<summary>Locate the existing block</summary>

```text
3,0.1],identity()).unwrap();assert_eq!(finish(&mut renderer),Some([owner.value(),0]));
        for mode in [crate::pick_sources::Mode::Edge,crate::pick_sources::Mode::Control]{renderer.set_pick_mode(mode);renderer.request_pick([-0.3,0.1],identity()).unwrap();assert!(finish(&mut renderer).is_none());}
    }
    assert!(crate::source_edit::same(&source,&crate::source_edit::original(&editor.scene,owner).unwrap()));editor.scene.show_controls(false);assert!(editor.scene.control_nets().is_empty());
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zg-rebuild-an-edited-element-without-replacing-its-owner-3.rs" skip=500
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 19.4.7
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

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for rebuild an edited element without replacing its owner.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Its geometry setter invalidates kernel caches while retaining feature source data. Reuse the Element preparation path in an isolated empty scene, validate the prepared vertices through the existing owner placement, then replace only that owner row. Keep its ID, portable row GUID, metadata and placement. The hierarchy, original feature data and existing feature presentation stay attached to that original parent. This control transition changes source geometry; it does not invent a feature-generation algorithm. The original Scene and source remain shared until commit, and one Undo restores them.

source feature visibility → coincident-point classification → existing marker/path display → original owner.



Why prepare the replacement before changing the live row?

Invalid geometry or display coordinates must fail before publishing a new source or consuming history. Reusing the preparation path also keeps Element mesh and BRep display construction consistent.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Create an Element, select a source control and move it. Its original parent and feature rows must retain their identities while the body display changes. Undo and Redo restore the geometry together with its source.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.4.7
npm --prefix ../session_tests run course -- save 19.4.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Connect Element source-control replacement through its existing preparation owner. Current feature editing, contact fills, presentation persistence and GPU/runtime differences remain separate required bridges.

Original-control browser acceptance is pending.

[Full validation scope](release.md).

</details>
