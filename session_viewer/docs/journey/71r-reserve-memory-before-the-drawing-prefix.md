# 14.1.19 · Reserve memory before the drawing prefix

**Chapter 14 · Coordinates and geometry tools · Section 14.1 · Sheet queries and construction planes** · [Contents](contents.md)

**Typing: 17–34 minutes.** [Estimate](typing-load.md).

Charge temporary page bytes before allocation and adopt the first checked segment prefix.

## Type

Continue from [Keep partial drawing sources explicit](71q-keep-partial-drawing-sources-explicit.md). [Save or recover your work](recovery.md).

### 1. `src/sheet_stream.rs`

Reserve temporary memory and stage one checked original drawing prefix.

<details>
<summary>Locate the existing block</summary>

```rust
    (count > 0).then(|| from..from + count)
}
#[cfg(test)]
mod tests {
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/71r-reserve-memory-before-the-drawing-prefix-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.1.19
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

The blue rotated and selected yellow mirrored references retain their original source definition.

![Native renderer output for reserve memory before the drawing prefix.](../screenshots/journey/70d-exercise-rotated-and-mirrored-original-references.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.2.5 · Exercise rotated and mirrored original references](70d-exercise-rotated-and-mirrored-original-references.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



remaining segment share → reserved temporary bytes → checked prefix.

![Reserve memory before the drawing prefix.](../illustrations/71r-reserve-memory-before-the-drawing-prefix.svg)

Does the segment ceiling bound all CPU and GPU memory?

No. It bounds staged segments; retained original data, display buffers and pending reads need separate accounting.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a25000-segment prefix with its source count and shared page budget.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.1.19
npm --prefix ../session_tests run course -- save 14.1.19
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited nested-reference exercise remains authored and unrun. The final sheet contract covers complete archive reopening, repeated checked-range placements, metadata only after selection, bounded reads and Close during a lookup. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.1.19
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
