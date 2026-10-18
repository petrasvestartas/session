# 15.2.5 · Read options and typed batches without partial results

**Chapter 15 · Transforms and surface construction · Section 15.2 · Profile collection and surfacing** · [Contents](contents.md)

**Typing: 16–31 minutes.** [Estimate](typing-load.md).

Resolve current options, typed points, finite numbers and distance vectors on a candidate owner before adopting any answer.

## Type

Continue from [Accept curve picks and geometric answers atomically](78c-accept-curve-picks-and-geometric-answers-atomically.md). [Save or recover your work](recovery.md).

### 1. `src/gather_draft.rs`

Extend the pending gather owner with checked current recipe behavior.

<details>
<summary>Locate the existing block</summary>

```rust
        let point=crate::tool_coords::checked(point).map_err(str::to_owned)?;let mut next=self.clone();let answer=match next.current(){Some(Step::Point(_))=>Answer::Point(Point::new(point[0],point[1],point[2])),Some(Step::Distance(_))=>{let(base,_)=next.axis()?;Answer::Vector(Vector::new(point[0]-base[0],point[1]-base[1],point[2]-base[2]))},_=>return Err("The current step needs a curve pick or a typed number".into())};next.progress.answer(next.recipe.steps,&mut next.answers,answer);let made=next.build()?;*self=next;Ok(made)
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/78d-read-options-and-typed-batches-without-partial-results-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.2.5
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

Three original boxes retain independent placements.

![Native renderer output for read options and typed batches without partial results.](../screenshots/journey/78-copy-current-curve-collection-and-surfacing-recipes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.2.1 · Copy current curve collection and surfacing recipes](78-copy-current-curve-collection-and-surfacing-recipes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



typed words or Enter → candidate options and answers → complete checked result.

![Read options and typed batches without partial results.](../illustrations/78d-read-options-and-typed-batches-without-partial-results.svg)

How can Cap Off span two submissions?

Retain the compact Cap prefix, then resolve Off to its current option. Enter accepts a sufficiently populated curve group or a numeric default.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Submit Cap, then Off, then -2. Try a bad axis-end/angle batch and verify the earlier axis start remains accepted.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.2.5
npm --prefix ../session_tests run course -- save 15.2.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding transformation browser contract remains authored and unrun. The final extrusion/revolution contract exercises the registered names, failed input, cap options, one Undo/Redo and the same rendered exercise. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 15.2.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
