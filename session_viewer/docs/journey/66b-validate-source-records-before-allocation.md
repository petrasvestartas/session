# 12.1.3 · Bound finite coordinates and packed integer reads

**Chapter 12 · Files and streaming · Section 12.1 · Scene manifests and range reads** · [Contents](contents.md)

**Typing: 20–40 minutes.** [Estimate](typing-load.md).

Keep these checks separate from geometry construction.

## Type

Continue from [Compare live byte revisions](66a-compare-live-byte-revisions.md). [Save or recover your work](recovery.md).

### 1. `src/source_validate.rs`

Guard scalar finiteness, coordinate triples and arithmetic before allocating or indexing. Decode a bounded varint without shifts past sixty-three bits.

Create the file and type:

```rust
--8<-- "journey/code/66b-validate-source-records-before-allocation-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

<details>
<summary>Locate the existing block</summary>

```rust

pub mod live_revision;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/66b-validate-source-records-before-allocation-02.rs" skip=24
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 12.1.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

This validator compiles in the browser application; source loading is connected by the following archive lessons.

### Recorded visual result

**Previous native render — current verification pending.**

The original cloud and local point marker remain unchanged.

![Native renderer output for bound finite coordinates and packed integer reads.](../screenshots/journey/65za-read-original-manifest-records-and-placements.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Finite scalars reject NaN and infinity; triples reject incomplete xyz records. The varint reader returns the consumed byte count so a later array reader can advance exactly once. Surface control addressing returns None on arithmetic overflow rather than wrapping into a small index.

original record → bounded scalar/address contract → reusable validator → later archive loader.



Why must integer decoding and control addressing use checked operations?

A continuation byte can require a shift outside u64, and a declared surface stride can overflow usize. Reject either condition before it becomes an address.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace bytes [172,2], an unfinished continuation byte, and an overflowing tenth byte. Then calculate the last address of a two-row, three-column surface with coordinate width three. Predict which result is rejected.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 12.1.3
npm --prefix ../session_tests run course -- save 12.1.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



This preparation is checked by the following original-source import and archive browser exercises.

[Full validation scope](release.md).

</details>
