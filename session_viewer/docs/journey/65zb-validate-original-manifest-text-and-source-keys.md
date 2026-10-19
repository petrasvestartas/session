# 11.3.6 · Validate original manifest text and source keys

**Chapter 11 · Outlines and point clouds · Section 11.3 · Point identity and manifest records** · [Contents](contents.md)

**Typing: 28–56 minutes.** [Estimate](typing-load.md).

Validate world text, heights and plane axes. Reuse the original format readers, write immutable-source keys and add placement/name accessors using the earlier records.

## Type

Continue from [Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). [Save or recover your work](recovery.md).

### 1. `src/manifest.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
fn text_right() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// World +Y.
fn text_up() -> [f64; 3] {
    [0.0, 1.0, 0.0]
}

/// A translation matrix.
fn translation(at: [f64; 3]) -> Xform {
    Xform::translation(at[0], at[1], at[2])
}

/// True when any value is not finite.
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/65zb-validate-original-manifest-text-and-source-keys-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 11.3.6
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Manifest validation can reject malformed text and source keys before browser loading is connected.

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for validate original manifest text and source keys.](../screenshots/journey/65za-read-original-manifest-records-and-placements.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [11.3.5 · Read original manifest records and placements](65za-read-original-manifest-records-and-placements.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original family → pure calculation → result or refusal.



Why validate text axes before making labels?

World text needs finite unit directions with a usable right-angle frame. Invalid axes would produce an unusable placement even when the text itself parses.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace empty text, an invalid height and non-unit axes through the original checks. Compare a content-addressed source key with a mutable filename.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 11.3.6
npm --prefix ../session_tests run course -- save 11.3.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original command helpers prepare the later editor bridge. The existing drawing stays unchanged.



[Full validation scope](release.md).

</details>
