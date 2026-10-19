# 14.3.15 · Weld solid tessellation once

**Chapter 14 · Coordinates and geometry tools · Section 14.3 · Planar shapes and regular solids** · [Contents](contents.md)

**Typing: 25–49 minutes.** [Estimate](typing-load.md).

Join kernel face meshes by reusing nearby corners, then retain either a placed BRep or its mesh representation. Search neighboring cells to avoid cracks at a cell boundary.

## Type

Continue from [Validate solid sizes and radius](73m-validate-solid-sizes-and-radius.md). [Save or recover your work](recovery.md).

### 1. `src/primitive_shape.rs`

Continue the shared solid foundation with this lesson’s functions.

<details>
<summary>Locate the existing block</summary>

```rust
/// Three great circles.
pub fn sphere_outline(part: &Part) -> Vec<Vec<Point>> {
    let [radius] = part.sizes[..] else {
        return Vec::new();
    };
    let side = part.frame.rolled();
    vec![
        part.frame.ring(radius, 0.0),
        side.ring(radius, 0.0),
        side.rolled().ring(radius, 0.0),
    ]
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/73n-weld-solid-tessellation-once-1.rs" skip=322
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 14.3.15
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

![Native renderer output for weld solid tessellation once.](../screenshots/journey/73h-match-solid-command-options.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [14.3.9 · Match solid command options](73h-match-solid-command-options.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Weld solid tessellation once → original f64 kernel coordinates → later temporary preview and accepted geometry.



Why search neighboring cells as well as the rounded cell?

Two close points can round into adjacent cells. Their actual distance determines whether they share a corner.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Choose two nearby points on opposite sides of a cell boundary. Predict whether a single-cell lookup would leave a crack.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 14.3.15
npm --prefix ../session_tests run course -- save 14.3.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This codes the shared solid-command foundation before connecting the individual builders. The browser scene is unchanged at this checkpoint.



[Full validation scope](release.md).

</details>
