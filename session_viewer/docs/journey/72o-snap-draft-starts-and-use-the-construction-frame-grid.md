# 14.2.16 · Snap draft starts and use the construction frame grid

**Chapter 14 · Coordinates and geometry tools · Section 14.2 · Coordinates and drawing tools** · [Contents](contents.md)

**Typing: 13–26 minutes.** [Estimate](typing-load.md).

Reuse object targets, prefer the open draft’s start and place free clicks through the current plane.

## Type

Continue from [Close original polylines as one transaction](72n-close-original-polylines-as-one-transaction.md). [Save or recover your work](recovery.md).

### 1. `src/editor.rs`

Snap current original targets and place finite free points on the current construction frame.

<details>
<summary>Locate the existing block</summary>

```rust
            index.query(viewport, &ray, [(screen[0] as f64+1.)*size[0]/2., (1.-screen[1] as f64)*size[1]/2.], self.snapping.modes, from)
        } else { None };
        hit.or_else(|| crate::snap_index::fallback(self.snapping, &self.camera, screen))
    }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/72o-snap-draft-starts-and-use-the-construction-frame-grid-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.2.16
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

Original black physical drawing pens and the blue hairline remain visible.

![Native renderer output for snap draft starts and use the construction frame grid.](../screenshots/journey/71aj-exercise-complete-and-ranged-original-drawings.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.1.37 · Exercise complete and ranged original drawings](71aj-exercise-complete-and-ranged-original-drawings.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



object targets or own start → current plane fallback → plane-axis grid.

![Snap draft starts and use the construction frame grid.](../illustrations/72o-snap-draft-starts-and-use-the-construction-frame-grid.svg)

Does the grid snap use world XY for every plane?

No. Round along the current plane axes and its origin; preserve the fixed plane or previous View-mode depth.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare XZ grid placement with own-start closure and source-target snapping.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.2.16
npm --prefix ../session_tests run course -- save 14.2.16
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The inherited drawing/sheet contracts remain authored and unrun. The completed exercise adds fixed-plane input, original coordinate diagnostics, invalid drafts, Close, Undo/Redo and Save/Open; actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 14.2.16
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
