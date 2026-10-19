# 15.1.21 · Validate point and number transformation answers

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Resolve a valid factor, reference reach or signed plane angle before adopting the candidate points.

## Type

Continue from [Own transformation questions and the selected original targets](77a-own-transformation-questions-and-the-selected-original-targets.md). [Save or recover your work](recovery.md).

### 1. `src/transform_draft.rs`

Validate current transform points and numbers without adopting an invalid reference.

<details>
<summary>Locate the existing block</summary>

```rust
        };format!("{} · {question} · Esc cancels{}",self.kind.name(),if self.kind==Kind::Scale{format!(" · {}D",self.mode)}else{String::new()})
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77b-validate-point-and-number-transformation-answers-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.21
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

![Native renderer output for validate point and number transformation answers.](../screenshots/journey/76d-exercise-exact-curves-and-rational-control-edits.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.6.5 · Exercise exact curves and rational control edits](76d-exercise-exact-curves-and-rational-control-edits.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



point or number answer → checked references → current original transform.

![Validate point and number transformation answers.](../illustrations/77b-validate-point-and-number-transformation-answers.svg)

Can Scale1D take a factor before its direction?

No. Its first reference point determines the direction. Scale 2D and3D can take a factor after the origin.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try a repeated reference point, a nonpositive factor and valid1D reference/new-length input.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.21
npm --prefix ../session_tests run course -- save 15.1.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding curve contract remains authored and unrun. The final transformation contract adds direct Copy and Rotate, repeat Copy and Undo, typed interactive Scale modes, invalid input/cancellation and a three-box exercise. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 15.1.21
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
