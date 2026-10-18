# 19.2.2 · Group pieces by their original interaction

**Chapter 19 · Integration and original ownership · Section 19.2 · Elements and feature hierarchies** · [Contents](contents.md)

**Typing: 15–30 minutes.** [Estimate](typing-load.md).

The graph determines who contributed a feature.

## Type

Continue from [Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). [Save or recover your work](recovery.md).

### 1. `src/feature_plan.rs`

Claim each feature once in graph order; retain unknown and orphaned interaction features.

<details>
<summary>Locate the existing block</summary>

```rust
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/96j-group-features-by-original-interaction-01.rs" skip=8
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Each claimed feature contributes to one ordered interaction group; unmatched features retain their own group.

### Recorded visual result

**Previous native render — current verification pending.**

The existing original comparison scene is unchanged; this step prepares imported Element ownership.

![Native renderer output for group pieces by their original interaction.](../screenshots/journey/96i-separate-own-and-hosted-element-features.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.2.1 · Separate own attributes from hosted features](96i-separate-own-and-hosted-element-features.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Visit incident edges by their recorded index, then each interaction in stored order. Exact feature/relation GUIDs identify contacts; InteractionFeature relations also claim source-prefixed feature GUIDs. Claim each feature once. Use the full interaction type and either its name or the source Element name, then retain unmatched features as explicit groups. Check has_guid before reading a feature GUID: guid() lazily assigns an ID and must not change an original merely to display it.

original source → bounded feature plan → derived display → source-preserving archive.



What happens to a feature with no matching relation?

It remains visible in an unclaimed group using its own type, name and face label. Display grouping does not invent an interaction or assign a missing source GUID.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Add two relations that could claim the same feature. Confirm that original edge/relation order chooses one group and that the feature is not duplicated.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 19.2.2
npm --prefix ../session_tests run course -- save 19.2.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This bridge extends the handwritten viewer. Compare the full interaction type and source-name grouping with maintained `app/interactions.rs` and `app/hierarchy.rs`. Native, browser and hardware evidence remain separate; see [local checks](local-checks.md).



[Full validation scope](release.md).

</details>
