# 14.1.6 · Measure original sheet pages and world pens

**Chapter 14 · Coordinates and geometry tools · Section 14.1 · Sheet queries and construction planes** · [Contents](contents.md)

**Typing: 9–18 minutes.** [Estimate](typing-load.md).

Use original doubles for bounds; positive pen widths become radii while invalid pens remain original hairlines.

## Type

Continue from [Decode precise original sheet columns](71d-decode-precise-original-sheet-columns.md). [Save or recover your work](recovery.md).

### 1. `src/sheet_page.rs`

Derive precise bounds, physical pen radii and capacity accounting.

<details>
<summary>Locate the existing block</summary>

```rust
        })
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/71e-measure-original-sheet-pages-and-world-pens-01.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/sheet_page.rs`

Copy original-column and malformed-page fixtures.

<details>
<summary>Locate the existing block</summary>

```text
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/71e-measure-original-sheet-pages-and-world-pens-02.rs" skip=3
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.1.6
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

![Native renderer output for measure original sheet pages and world pens.](../screenshots/journey/70d-exercise-rotated-and-mirrored-original-references.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [13.2.5 · Exercise rotated and mirrored original references](70d-exercise-rotated-and-mirrored-original-references.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original endpoints → f64 bounds → authored radius → payload bytes.

![Measure original sheet pages and world pens.](../illustrations/71e-measure-original-sheet-pages-and-world-pens.svg)

Does a NaN original pen make a NaN GPU radius?

No. Preserve the original word and derive a zero-radius hairline, matching current display defaults.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Copy precise coordinate, repeated high-ID, hairline and malformed-column fixtures.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.1.6
npm --prefix ../session_tests run course -- save 14.1.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited nested-reference exercise remains authored and unrun. The final sheet contract covers complete archive reopening, repeated checked-range placements, metadata only after selection, bounded reads and Close during a lookup. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.1.6
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
