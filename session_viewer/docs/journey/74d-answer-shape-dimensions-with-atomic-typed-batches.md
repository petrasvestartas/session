# 14.4.5 · Answer shape dimensions with atomic typed batches

**Chapter 14 · Coordinates and geometry tools · Section 14.4 · Polyhedra and round solids** · [Contents](contents.md)

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

Resolve numbers, current-plane points, choices and offered Enter defaults before replacing the current answer set.

## Type

Continue from [Build only after the last shape question](74c-build-only-after-the-last-shape-question.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_draft.rs`

Resolve current shape choices, numbers, coordinates and Enter defaults atomically.

<details>
<summary>Locate the existing block</summary>

```rust
        self.answers=answers;self.part=part;Ok(geometry)
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/74d-answer-shape-dimensions-with-atomic-typed-batches-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.4.5
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

A closed original rectangle and an oriented five-sided original polygon remain visible.

![Native renderer output for answer shape dimensions with atomic typed batches.](../screenshots/journey/73g-exercise-oriented-original-planar-constructions.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.8 · Exercise oriented original planar constructions](73g-exercise-oriented-original-planar-constructions.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



typed batch → current questions → checked original answers → one result.

![Answer shape dimensions with atomic typed batches.](../illustrations/74d-answer-shape-dimensions-with-atomic-typed-batches.svg)

Can input after completion change the created representation?

No. Reject extra words atomically, including a trailing option after the last dimension.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try Box0,0,0 2 3 4 extra, then typed dimensions with an offered width and height default.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.4.5
npm --prefix ../session_tests run course -- save 14.4.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited planar contract remains authored and unrun. The final solid contract adds typed/clicked dimensions, choices/defaults, atomic invalid input, original creation, Undo/Redo, temporary previews and current quick-click drift; actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.4.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
