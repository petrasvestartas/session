# 19.2.9 · Prove imported feature ownership and round trips

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 1–1 minutes.** [Estimate](typing-load.md).

Exercise the connected import through original source comparisons, hierarchy ownership, independent visibility, nested transforms, Delete/Undo, and repeated archives.

## Type

Continue from [Build an original interaction specimen](96p-build-an-imported-interaction-specimen.md). [Save or recover your work](recovery.md).

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/interaction_bridge_tests.rs`

Run supplied regression fixtures against original source payloads and the connected import path.

Copy this supplied file:

```text
--8<-- "journey/code/96q-prove-imported-feature-ownership-and-roundtrips-01.rs"
```

</details>

### 2. `src/lib.rs`

Include the ownership and archive proofs in the native suite.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod inventory_tests;
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod interaction_bridge_tests;

#[cfg(test)]
mod inventory_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96q-prove-imported-feature-ownership-and-roundtrips-02.rs"
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

Inspect the four imported-feature regression tests, then repeat the Open sample.pb exercise from the preceding checkpoint. Keep the contact and child Element visible while switching the independent display controls.

### Recorded visual result

**Previous native render — current verification pending.**

The existing original comparison scene is unchanged; this step prepares imported Element ownership.

![Native renderer output for prove imported feature ownership and round trips.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Decode Mesh messages before comparing their fields because protobuf map serialization order is not stable. Compare the remaining Element payload exactly, including empty feature GUIDs and original outlines. Generated paths must remain six after every Save/Open and must never appear as saved original polylines.

original source → bounded feature plan → derived display → source-preserving archive.



What makes a successful round-trip stronger than equal object counts?

The saved document must retain the original Element fields and opaque interactions, omit generated source geometry, restore editable child ownership, and regenerate exactly one set of feature pieces.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

In Chrome, toggle both Element controls, delete/undo the child, and Save/Close/Open twice. Confirm the same hierarchy without accumulated pieces.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.9
npm --prefix ../session_tests run course -- save 19.2.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This bridge extends the handwritten viewer. Compare the full interaction type and source-name grouping with maintained `app/interactions.rs` and `app/hierarchy.rs`. Native, browser and hardware evidence remain separate; see [local checks](local-checks.md).

The fixtures verify full interaction labels, original feature ownership, read-only generated paths, editable nested child Elements, exact-once placement, Delete/Undo identity, three archive round trips without extra source polylines, and atomic failure. Browser execution is recorded separately.

The imported interaction specimen in Chrome: original Elements, derived feature groups and independent child editing. The browser contract checks visibility controls, Delete/Undo and repeated Save/Open.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 19.2.9
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
