# 17.1.2 · Grow projection and reference pools from demand

**Chapter 17 · GPU visibility, clipping and sections · Section 17.1 · Finite ink and tiled visibility** · [Contents](contents.md)

**Typing: 12–24 minutes.** [Estimate](typing-load.md).

Map triangle projection onto two-dimensional dispatch rows of 64 invocations per workgroup, keeping each dimension under the device dispatch limit.

## Type

Continue from [Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). [Save or recover your work](recovery.md).

### 1. `src/tile_math.rs`

Grow projection and reference pools from demand.

<details>
<summary>Locate the existing block</summary>

```rust
        self.header_records() * 16 + pool_words * 4
    }

    /// Largest pool, in words.
    pub fn max_pool_words(self) -> u64 {
        self.count() as u64 * REFERENCES_PER_TILE * REFERENCE_WORDS
    }

    /// First pool size for `triangles`, in words.
    pub fn initial_pool_words(self, triangles: u32) -> u64 {
        let references = self.count() as u64 * 2 + u64::from(triangles) * 8;
        (references * REFERENCE_WORDS)
            .max(MIN_POOL_WORDS)
            .min(self.max_pool_words())
    }
}
```

</details>

After that block, type:

```rust
/// Projection workgroups across and down for `triangles`, 64 each, rows of PROJECT_ROW_GROUPS.
pub fn project_groups(triangles: u32) -> (u32, u32) {
    let groups = triangles.div_ceil(64);
    (
        groups.min(PROJECT_ROW_GROUPS),
        groups.div_ceil(PROJECT_ROW_GROUPS),
    )
}

/// Next pool size: at least `floor`; doubled when the report overflowed.
/// Records for a table of `need` triangles, None while `capacity` fits: exact after a load jump, an eighth spare after an edit.
pub fn projected_records(need: u64, capacity: u64, most: u64) -> Option<u64> {
    if need <= capacity && need * 4 >= capacity {
        return None;
    }

    let spare = if need > capacity * 3 / 2 { 0 } else { need / 8 };
    Some((need + spare).min(most).max(need))
}

pub fn next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option<u64>,
    ceiling: u64,
) -> u64 {
    let mut want = current.max(floor);

    // a report at or past capacity means the lists did not fit
    if let Some(needed) = report
        && needed >= capacity
    {
        want = want.max(current.saturating_mul(2));
    }

    want.min(ceiling)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/86ma-grow-projection-and-reference-pools-from-demand-typed-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/tile_math.rs`

Grow projection and reference pools from demand.

<details>
<summary>Locate the existing block</summary>

```text
et spare = if need > capacity * 3 / 2 { 0 } else { need / 8 };
    Some((need + spare).min(most).max(need))
}

pub fn next_pool_words(
    current: u64,
    floor: u64,
    capacity: u64,
    report: Option<u64>,
    ceiling: u64,
) -> u64 {
    let mut want = current.max(floor);

    // a report at or past capacity means the lists did not fit
    if let Some(needed) = report
        && needed >= capacity
    {
        want = want.max(current.saturating_mul(2));
    }

    want.min(ceiling)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/86ma-grow-projection-and-reference-pools-from-demand-typed-2.rs" skip=500
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.1.2
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

The rendered headed line and world text remain black; a red dashed extension tail ends at its black +0.800 chip.

![Native renderer output for grow projection and reference pools from demand.](../screenshots/journey/86-copy-finite-tile-allocation-rules.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.1 · Bound the physical tile grid and reference storage](86-copy-finite-tile-allocation-rules.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Reuse the projected table while it is between one quarter full and capacity; allocate an exact table for a large load jump, or an eighth of spare capacity for an edit. Grow the shared reference pool only from its initial floor or an asynchronous report that reached capacity. Saturating doubling and the explicit ceiling keep that growth bounded. The later GPU owner will enforce device buffer limits and consume this calculation; no synchronous GPU readback is introduced here.

physical canvas → bounded tile layout → shared word pool.



Why treat a report exactly equal to capacity as saturated?

A clamped report cannot distinguish exactly full from overflow. Conservatively growing prevents a full pool from remaining permanently too small.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Starting at 1000 words, trace four saturated reports with a 10000-word ceiling. Then compare no report with a report below capacity.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.1.2
npm --prefix ../session_tests run course -- save 17.1.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Retain finite original-triangle projection and tile lists. The tutorial adapter connects identical center-line visibility to color and cropped integer picking; full plane fitting, curved-edge sag and packed production arenas are audited in94–96.

Compile this cumulative browser application. The completed allocation owner has eight independent numerical fixtures covering dispatch limits, pool growth and physical viewport budgets.

Allocation preparation; tile projection and visibility are not connected at this checkpoint.

[Full validation scope](release.md).

</details>
