# 7.4.1 · Separate object and grid snapping

**Chapter 7 · Drawing, snapping and transform tools · Section 7.4 · Object and grid snapping** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Keep object snapping and grid rounding independent. Grid Snap starts enabled at 1000 mm; changing its spacing enables it without changing object snapping or the running draft.

## Type

Continue from [Draw points and lines through the shared command dock](48d-input.md). [Save or recover your work](recovery.md).

### 1. `src/snap.rs`

Define snap kinds, independent switches and validated current1000mm grid settings.

Create the file and type:

```rust
--8<-- "journey/code/49-settings-01.rs"
```

### 2. `src/lib.rs`

Register the snap component and supplied checks.

<details>
<summary>Locate the existing block</summary>

```rust

#[cfg(test)]
mod selection_cycle_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/49-settings-02.rs" skip=41
```

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Continue the earlier Point/Line exercise. This settings model is connected to actual commands at 49f.

### Recorded visual result

**Previous native render — current verification pending.**

The earlier temporary line band and cursor remain, without a saved line.

![Native renderer output for separate object and grid snapping.](../screenshots/journey/48d-input.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [7.3.5 · Draw points and lines through the shared command dock](48d-input.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



independent switches → mode bits → validated spacing → current settings.

![Separate object and grid snapping](../illustrations/journey-49.svg)

Does Snap Off also turn Grid Snap off?

No. Object targets and the construction-plane grid have separate switches. Grid Snap Off disables only rounding to grid cells.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare toggling Grid Snap with setting Grid Snap 50: spacing must be positive and finite, and a valid spacing enables the grid.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 7.4.1
npm --prefix ../session_tests run course -- save 7.4.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This preparation retains Point/Line drawing while adding snap infrastructure. Actual snap commands and canvas queries connect at 49f. Full construction-plane controls follow in 77.

The earlier unfinished line remains. Snapping is connected to canvas input at 49f.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 7.4.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
