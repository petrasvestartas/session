# 14.3.14 · Validate solid sizes and radius

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 18–36 minutes.** [Estimate](typing-load.md).

Reject nonfinite and degenerate dimensions. Read a center and radius once, then reuse the same frame for three great-circle previews.

## Type

Continue from [Describe solid builders and dimensions](73l-describe-solid-builders-and-dimensions.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_shape.rs`

Continue the shared solid foundation with this lesson’s functions.

<details>
<summary>Locate the existing block</summary>

```rust
    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    pub(crate) fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        options::find(self.options, words)
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73m-validate-solid-sizes-and-radius-1.rs" skip=224
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.14
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

![Native renderer output for validate solid sizes and radius.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Validate solid sizes and radius → original f64 kernel coordinates → later temporary preview and accepted geometry.



Why can a height be negative while a radius must be positive?

Height specifies a direction along the frame normal. Radius measures a nonnegative distance.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the error for NaN, zero and an oversized radius. Trace the three rolled frames used by the sphere preview.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.14
npm --prefix ../session_tests run course -- save 14.3.14
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This codes the shared solid-command foundation before connecting the individual builders. The browser scene is unchanged at this checkpoint.



[Full validation scope](release.md).

</details>
