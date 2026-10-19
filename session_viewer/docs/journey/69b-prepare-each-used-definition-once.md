# 13.1.2 · Prepare each used definition once

**Chapter 13 · Shared definitions and drawing sheets · Section 13.1 · Shared CAD definitions** · [Contents](contents.md)

**Typing: 17–33 minutes.** [Estimate](typing-load.md).

Prepare one display template for each used original definition. Repeated placements clone shared owners and derived geometry.

## Type

Continue from [Validate original definition references](69-validate-original-definition-references.md). [Save or recover your work](recovery.md).

### 1. `src/instance_definition.rs`

Prepare each referenced original definition once.

Create the file and type:

```rust
--8<-- "journey/code/69b-prepare-each-used-definition-once-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod instance_plan;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/69b-prepare-each-used-definition-once-02.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 13.1.2
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

The existing CAD, surface and original coarse/detailed point pages remain visible.

![Native renderer output for prepare each used definition once.](../screenshots/journey/69-validate-original-definition-references.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.1.1 · Validate original definition references](69-validate-original-definition-references.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



definition lookup → one prepared template → reused display source.

![Prepare each used definition once.](../illustrations/69b-prepare-each-used-definition-once.svg)

Must every instance run the CAD tessellator?

No. Prepare each used definition once and share its resulting geometry. The placement remains independent.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare original and derived Rc identities for two references to one mesh definition.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 13.1.2
npm --prefix ../session_tests run course -- save 13.1.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited range exercise remains authored and unrun. The final instance exercise checks shared definitions, independent placements, Save/Open, Move, Delete, Undo/Redo and Close; browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 13.1.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
