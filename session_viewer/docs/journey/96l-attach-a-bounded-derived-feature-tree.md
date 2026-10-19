# 19.2.4 · Attach a bounded feature tree to its host

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

Convert the feature plan into temporary branches beneath the owning Element.

## Type

Continue from [Keep nested Elements editable and projections unsaved](96k-separate-derived-rows-from-editable-elements.md). [Save or recover your work](recovery.md).

### 1. `src/layer_edit.rs`

Reuse the existing branch transaction and node/depth limits for feature rows.

<details>
<summary>Locate the existing block</summary>

```rust
 let mut count=0;let mut max=0;fn size(b:&Branch,depth:usize,count:&mut usize,max:&mut usize){*count+=1;*max=(*max).max(depth);for c in &b.children{size(c,depth+1,count,max)}}size(&root,0,&mut count,&mut max);if count>crate::hierarchy::MAX_NODES||max>crate::hierarchy::MAX_DEPTH{return Err("Element attributes exceed hierarchy limits")}replace(scene,owner,root);Ok(())
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96l-attach-a-bounded-derived-feature-tree-01.rs" skip=371
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The validated hierarchy adds read-only feature groups beneath their owning original Element.

### Recorded visual result

**Previous native render — current verification pending.**

The existing original comparison scene is unchanged; this step prepares imported Element ownership.

![Native renderer output for attach a bounded feature tree to its host.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Groups contain groups or pieces; every new node carries derived and read-only provenance. Merge into an existing attributes/interactions group when one exists, preserving its original children. Flatten and check the complete node count and depth before replacing the document hierarchy.

original source → bounded feature plan → derived display → source-preserving archive.



Why preserve an existing attributes group?

It may contain original geometry or independent child Elements. Only the new projected descendants are disposable; replacing the whole group would destroy source structure.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the original attributes group before and after projection. Its identity and child Element should remain, with the new feature group appended.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.4
npm --prefix ../session_tests run course -- save 19.2.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This bridge extends the handwritten viewer. Compare the full interaction type and source-name grouping with maintained `app/interactions.rs` and `app/hierarchy.rs`. Native, browser and hardware evidence remain separate; see [local checks](local-checks.md).



[Full validation scope](release.md).

</details>
