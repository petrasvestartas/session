# 8.2.20 · Own rename text and populated deletion confirmation

**Chapter 8 · Hierarchy, layers and direct editing · Section 8.2 · Layers and presentation** · [Contents](contents.md)

**Typing: 16–32 minutes.** [Estimate](typing-load.md).

Focus the layer-name field, consume Enter/Escape before the command dock, preserve invalid drafts and require explicit confirmation before deleting populated layers.

## Type

Continue from [Draw eye lock and independent colour menus](53q-properties-ui.md). [Save or recover your work](recovery.md).

### 1. `src/tree_panel.rs`

Own name-field keys and show explicit populated-layer deletion controls.

<details>
<summary>Locate the existing block</summary>

```rust
    }) {menus.push(menu.response.rect);}
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/53r-name-01.rs" skip=44
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

The nested frame and selected editable parent remain visible with original source edges.

![Native renderer output for own rename text and populated deletion confirmation.](../screenshots/journey/52k-tree.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [8.1.12 · Exercise nested source ownership and exact undo](52k-tree.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



focused name draft → owned Enter or Escape → checked rename → explicit deletion confirmation.

![Own rename text and populated deletion confirmation.](../illustrations/53r-name.svg)

What should H S and Delete do in the rename field?

They edit text, rather than operating on the canvas. Invalid names remain visible with an error, and Escape cancels the draft without a document edit.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Type H/S in a name, submit a used name, correct it, then cancel a populated deletion. No canvas shortcut or accidental deletion should occur.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 8.2.20
npm --prefix ../session_tests run course -- save 8.2.20
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This cumulative subject implements layer editing and independent presentation for the mesh tree and generated geometry lanes already taught. General group placements, display-only streamed documents and later geometry types connect in their own subjects.

The earlier nested-frame acceptance exercise remains; the complete layer control exercise connects at the end of this subject.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 8.2.20
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
