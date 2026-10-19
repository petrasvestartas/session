# 5.3.1 · Prepare an original point for a screen marker

**Chapter 5 · Lines, points and curves · Section 5.3 · Original points and controls** · [Contents](contents.md)

**Typing: 13–25 minutes.** [Estimate](typing-load.md).

Retain the original Point owner and convert only its display position. Pack centre, diameter and colour into one explicit 32-byte record.

## Type

Continue from [Insert connected paths and arrow examples](36e-input.md). [Save or recover your work](recovery.md).

### 1. `src/marker.rs`

Retain exact point ownership and pack finite display data with the viewer’s diameter policy.

Create the file and type:

```rust
--8<-- "journey/code/37-point-01.rs"
```

### 2. `src/lib.rs`

Register point preparation and its source checks.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod chain;
```

</details>

After that block, type:

```rust
pub mod marker;
#[cfg(test)]
mod marker_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/37-point-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/marker_tests.rs`

Copy point precision, packing, style and lifetime checks.

Copy this supplied file:

```text
--8<-- "journey/code/37-point-03.rs"
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

Run point preparation checks. The source keeps its exact coordinates while the marker uses explicit display precision and diameter.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for prepare an original point for a screen marker.](../screenshots/journey/36da-render.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [5.2.9 · Draw connected document rows](36da-render.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Match the current viewer’s point defaults: an unspecified/default pen draws a six-pixel diameter dot, while an explicit width changes its diameter. Invalid display coordinates or overflowing widths fail before GPU allocation.

original Point owner → checked position and style → explicit default diameter → packed marker record.

![The exact point remains the source of its screen marker.](../illustrations/journey-37.svg)

Why retain a source Point beside the display marker?

The marker is a screen representation. Original doubles, identity and metadata remain available for saving, placing and later picking or editing.

Study estimate, including typing and experiments: 0.75–1.25 hours.

</details>

<details>
<summary>Optional experiment</summary>

Use a coordinate that changes during f32 conversion. The source must remain unchanged; cloning prepared data must share its source owner.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 5.3.1
npm --prefix ../session_tests run course -- save 5.3.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Original point payload preparation is complete here. Marker extrusion, document IDs, actual drawing and controls follow in later checkpoints.

Native tests verify exact source precision, identity, shared lifetime, marker packing, default/style policy and finite display refusal. Connected browser checks remain the baseline until markers draw.

Chrome retains the existing connected examples while native checks establish point source and payload policy.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 5.3.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
