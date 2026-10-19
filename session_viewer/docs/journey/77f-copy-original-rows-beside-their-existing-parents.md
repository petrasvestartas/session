# 15.1.25 · Copy original rows beside their existing parents

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Stage checked copies with new row and tree identities, retain names/colours and copy attached attributes under the same parent.

## Type

Continue from [Rebuild an edited original CAD source before adoption](77e-rebuild-an-edited-original-cad-source-before-adoption.md). [Save or recover your work](recovery.md).

### 1. `src/layer_edit.rs`

Copy checked original rows under their existing parent with current attributes, new identities and placed source sharing.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77f-copy-original-rows-beside-their-existing-parents-01.rs" skip=8
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.25
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

Original Circle, Ellipse, Arc, Parabola and freeform Curve remain visible.

![Native renderer output for copy original rows beside their existing parents.](../screenshots/journey/76d-exercise-exact-curves-and-rational-control-edits.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.6.5 · Exercise exact curves and rational control edits](76d-exercise-exact-curves-and-rational-control-edits.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



selected original rows → checked copy plan → new identities and world placement.

![Copy original rows beside their existing parents.](../illustrations/77f-copy-original-rows-beside-their-existing-parents.svg)

Does copying a selected parent also copy every ordinary child?

Only the explicit selected objects are copied; attached attributes follow their owner. Ordinary child objects keep their existing original parent.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare Copy under duplicate named groups, copied row GUIDs, retained source allocation and inverse-derived saved local placement.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.25
npm --prefix ../session_tests run course -- save 15.1.25
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding curve contract remains authored and unrun. The final transformation contract adds direct Copy and Rotate, repeat Copy and Undo, typed interactive Scale modes, invalid input/cancellation and a three-box exercise. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 15.1.25
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
