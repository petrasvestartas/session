# 15.1.27 · Validate selected originals before beginning or committing a transform

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Freeze the selected targets and prepare their original preview before replacing any active input owner.

## Type

Continue from [Carry transformation state through the existing drawing owner](77g-carry-transformation-state-through-the-existing-drawing-owner.md). [Save or recover your work](recovery.md).

### 1. `src/editor.rs`

Connect checked original transformations to the current Editor transaction boundary.

<details>
<summary>Locate the existing block</summary>

```rust
        if let Some(geometry)=geometry {return self.commit_tool(crate::tool::Built::Geometry(geometry));}
        self.tool_status=draft.prompt();Ok(Change::View)
    }
    fn tool_text(&mut self,text:&str) -> Result<Change,&'static str> {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77h-validate-selected-originals-before-beginning-or-committing-a-transform-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.27
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

![Native renderer output for validate selected originals before beginning or committing a transform.](../screenshots/journey/76d-exercise-exact-curves-and-rational-control-edits.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.6.5 · Exercise exact curves and rational control edits](76d-exercise-exact-curves-and-rational-control-edits.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



selected original targets → checked preview → current tool or one commit.

![Validate selected originals before beginning or committing a transform.](../illustrations/77h-validate-selected-originals-before-beginning-or-committing-a-transform.svg)

Can Copy on a control point replace a valid unfinished tool?

No. Copy requires whole objects and validates them before adopting its owner.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try Copy on a curve control, an unavailable source and a locked row. Compare direct source-part Move with whole-object placement.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.27
npm --prefix ../session_tests run course -- save 15.1.27
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding curve contract remains authored and unrun. The final transformation contract adds direct Copy and Rotate, repeat Copy and Undo, typed interactive Scale modes, invalid input/cancellation and a three-box exercise. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 15.1.27
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
