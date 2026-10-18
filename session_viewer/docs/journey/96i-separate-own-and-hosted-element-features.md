# 19.2.1 · Separate own attributes from hosted features

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 12–23 minutes.** [Estimate](typing-load.md).

An Element stores both its own attributes and features produced by its relationships.

## Type

Continue from [Prove current source grammar and explicit differences](96h-prove-current-source-grammar-and-explicit-differences.md). [Save or recover your work](recovery.md).

### 1. `src/feature_plan.rs`

Give this operation its own small owner.

Create the file and type:

```rust
--8<-- "journey/code/96i-separate-own-and-hosted-element-features-01.rs"
```

### 2. `src/lib.rs`

Register the new owner without changing the browser entry point.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod inventory_tests;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod feature_plan;

#[cfg(test)]
mod inventory_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96i-separate-own-and-hosted-element-features-02.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Feature classification retains each original type, readable name and source label without mutating its geometry.

### Recorded visual result

**Previous native render — current verification pending.**

The existing original comparison scene is unchanged; this step prepares imported Element ownership.

![Native renderer output for separate own attributes from hosted features.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

Start with a small classifier over the original feature records. Contact, joint, cut and drill features belong to interactions; other types remain own attributes. Names and face indices become labels, while the original outlines remain untouched.

original source → bounded feature plan → derived display → source-preserving archive.



Why return feature indices instead of cloned geometry?

Indices keep the grouping plan tied to the original Element. Display preparation can later borrow exactly those records, without treating a label or a display path as source data.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try an empty feature name, a name equal to its type, and face index −1. Explain which text the label should omit.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.1
npm --prefix ../session_tests run course -- save 19.2.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This bridge extends the handwritten viewer. Compare the full interaction type and source-name grouping with maintained `app/interactions.rs` and `app/hierarchy.rs`. Native, browser and hardware evidence remain separate; see [local checks](local-checks.md).



[Full validation scope](release.md).

</details>
