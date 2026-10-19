# 16.2.4 · Adopt sibling pieces beside the original target

**Chapter 16 · Geometry editing and measurement · Section 16.2 · Curve pieces and joined faces** · [Contents](contents.md)

**Typing: 11–21 minutes.** [Estimate](typing-load.md).

Copy the target’s source row and tree parent before replacing originals. Keep the first GUID and name each piece without moving it to the current layer.

## Type

Continue from [Transform cutters into the target frame before dividing](82b-transform-cutters-into-the-target-frame-before-dividing.md). [Save or recover your work](recovery.md).

### 1. `src/split_draft.rs`

Adopt named original sibling pieces with inherited source parent, placement and colours.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/82c-adopt-sibling-pieces-beside-the-original-target-01.rs" skip=8
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.2.4
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

The extended line and projected original curve remain visible.

![Native renderer output for adopt sibling pieces beside the original target.](../screenshots/journey/82a-own-the-split-target-and-toggle-original-cutters.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.2.2 · Own the split target and toggle original cutters](82a-own-the-split-target-and-toggle-original-cutters.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



prepared pieces → target sibling identities → inherited parent and placement.

![Adopt sibling pieces beside the original target.](../illustrations/82c-adopt-sibling-pieces-beside-the-original-target.svg)

Should a new current layer receive the split pieces?

No. Pieces belong beside the source target. Their creation follows that target’s actual tree parent and original placement.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect sibling GUID allocation, source replacement and tree label updates. A caller wraps all these changes in one transaction.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.2.4
npm --prefix ../session_tests run course -- save 16.2.4
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding original-edit contract remains authored and unrun. The final split contract adds actual camera-projected target/cutter clicks, empty confirmation, completion, Escape and one Undo/Redo. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.2.4
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
