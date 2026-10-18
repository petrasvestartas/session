# 17.1.1 · Bound the physical tile grid and reference storage

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

Tile coverage uses physical framebuffer pixels, not CSS pixels.

## Type

Continue from [Exercise current original commands together](85ay-exercise-current-original-commands-together.md). [Save or recover your work](recovery.md).

### 1. `src/tile_math.rs`

Bound the physical tile grid and reference storage.

Create the file and type:

```rust
--8<-- "journey/code/86-copy-finite-tile-allocation-rules-typed-1.rs"
```

### 2. `src/lib.rs`

Register the completed source owner.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod lasso_geometry_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/86-copy-finite-tile-allocation-rules-02.rs" skip=39
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The preceding drawing remains visible. This CPU allocation owner compiles; GPU projection and reference-list dispatch connect in the following finite-ink lessons.

### Recorded visual result

**Previous native render — current verification pending.**

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for bound the physical tile grid and reference storage.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

Begin with four-pixel tiles and double their span until the tile count fits the fixed budget. Reserve sixteen bytes per tile record, plus hierarchical scan totals; triangle references then share one word pool rather than a fixed list for every tile. A reference stores two words: a triangle index and depth. Compute the initial pool from the viewport and triangle count, then cap it by the maximum total reference budget. These CPU sizes must agree with the WGSL offsets in the following scan and fill lessons.

physical canvas → bounded tile layout → shared word pool.



Why grow tile span instead of allocating an unrestricted grid?

Doubling the span reduces the number of tiles by roughly four. This bounds both the header and maximum reference pool when the physical canvas grows.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Calculate the grid at 1800 × 1400 and 2800 × 1800 pixels. Explain why the second canvas needs eight-pixel tiles and why doubling device pixel ratio may change the layout.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.1
npm --prefix ../session_tests run course -- save 17.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Retain finite original-triangle projection and tile lists. The tutorial adapter connects identical center-line visibility to color and cropped integer picking; full plane fitting, curved-edge sag and packed production arenas are audited in94–96.

Allocation preparation; tile projection and visibility are not connected at this checkpoint.

[Full validation scope](release.md).

</details>
