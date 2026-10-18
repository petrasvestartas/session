# 16.5.21 · Plan world-preserving original groups

**Chapter 16 · Geometry editing and measurement · Section 16.5 · Text, styles and command integration** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Plan world-preserving original groups.

## Type

Continue from [Remap appended links and remove deleted owners](85s-remap-appended-links-and-remove-deleted-owners.md). [Save or recover your work](recovery.md).

### 1. `src/group_plan.rs`

Create the file and type:

```rust
--8<-- "journey/code/85t-plan-world-preserving-original-groups-01.rs"
```

### 2. `src/lib.rs`

Register this command owner.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod graph_edit;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/85t-plan-world-preserving-original-groups-02.rs" skip=21
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.5.21
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

The preceding solid, wire bounding box and temporary distance annotation remain visible.

![Native renderer output for plan world-preserving original groups.](../screenshots/journey/84m-exercise-original-boxes-and-temporary-distance-annotations.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.14 · Exercise original boxes and temporary distance annotations](84m-exercise-original-boxes-and-temporary-distance-annotations.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



eligible subtrees → deepest shared layer → checked branch.

![Plan world-preserving original groups.](../illustrations/85t-plan-world-preserving-original-groups.svg)

Does grouping move the objects?

No. Recompute their local placements under the shared layer while preserving their world placement.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect the root-first common layer, outer selected subtrees and unique group names.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.5.21
npm --prefix ../session_tests run course -- save 16.5.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Plan world-preserving original groups.

The inherited measurement browser contract remains authored and unrun. The current-command browser contract connects at85ay and still requires real browser execution.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.5.21
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
