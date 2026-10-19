# 14.3.11 · Build a right-handed solid frame

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Place kernel coordinates in a construction-plane frame. Rotate, flip and roll its axes without changing original source geometry.

## Type

Continue from [Describe solid command questions](73i-describe-solid-command-questions.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_shape.rs`

Continue the shared solid foundation with this lesson’s functions.

<details>
<summary>Locate the existing block</summary>

```rust
    /// Enter takes `value`.
    pub fn or(self, value: f64) -> Self {
        Self {
            default: Some(value),
            ..self
        }
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73j-build-a-right-handed-solid-frame-1.rs" skip=158
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.11
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

![Native renderer output for build a right-handed solid frame.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Build a right-handed solid frame → original f64 kernel coordinates → later temporary preview and accepted geometry.



Why reverse both y and z when flipping the frame?

Reversing both axes keeps x cross y equal to z, preserving a rotation.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Use the Top and Front plane axes to calculate the normal. Predict a local point after flipping the frame.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.11
npm --prefix ../session_tests run course -- save 14.3.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This codes the shared solid-command foundation before connecting the individual builders. The browser scene is unchanged at this checkpoint.



[Full validation scope](release.md).

</details>
