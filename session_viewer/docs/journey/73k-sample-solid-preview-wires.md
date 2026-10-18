# 14.3.12 · Sample solid preview wires

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 13–26 minutes.** [Estimate](typing-load.md).

Sample circles and boxes in the existing frame. These points describe temporary preview wires; the kernel still creates the accepted solid.

## Type

Continue from [Build a right-handed solid frame](73j-build-a-right-handed-solid-frame.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_shape.rs`

Continue the shared solid foundation with this lesson’s functions.

<details>
<summary>Locate the existing block</summary>

```rust
    /// Local to world.
    pub fn to_xform(&self) -> Xform {
        Xform::frame_to_world(&self.origin, &self.x, &self.y, &self.z)
    }

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/73k-sample-solid-preview-wires-1.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.12
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

![Native renderer output for sample solid preview wires.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Sample solid preview wires → original f64 kernel coordinates → later temporary preview and accepted geometry.



Why include the first point again at the end of an oval?

The repeated endpoint closes the preview polyline. It is distinct from a duplicate solid vertex.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the number of circle points and box wire chains. Change the ring sample count temporarily and restore it.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.12
npm --prefix ../session_tests run course -- save 14.3.12
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This codes the shared solid-command foundation before connecting the individual builders. The browser scene is unchanged at this checkpoint.



[Full validation scope](release.md).

</details>
