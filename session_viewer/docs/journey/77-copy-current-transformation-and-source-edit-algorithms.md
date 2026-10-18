# 15.1.1 · Build directional scale transforms

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Measure scale reach in space, in the construction plane or along one chosen axis.

## Type

Continue from [Exercise exact curves and rational control edits](76d-exercise-exact-curves-and-rational-control-edits.md). [Save or recover your work](recovery.md).

### 1. `src/transform_scale.rs`

Build directional scale transforms.

Create the file and type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-typed-01.rs"
```

### 2. `src/lib.rs`

Register the source transformation component.

<details>
<summary>Locate the existing block</summary>

```rust
#[cfg(test)]
mod freeform_tests;
```

</details>

After that block, type:

```rust
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-02.rs" skip=33
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/transform_scale.rs`

Supply independent original-source and fixed-origin regressions.

<details>
<summary>Locate the existing block</summary>

```text
ion.
fn stretch(origin: &Point, axes: &[Vector], k: f64) -> Xform {
    let mut xform = Xform::identity();

    for a in axes {
        for col in 0..3 {
            for row in 0..3 {
                xform.m[col * 4 + row] += (k - 1.0) * a[row] * a[col];
            }
        }
    }

    // the origin stays where it is
    for row in 0..3 {
        let moved: f64 = (0..3).map(|col| xform.m[col * 4 + row] * origin[col]).sum();
        xform.m[12 + row] = origin[row] - moved;
    }

    xform
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77-copy-current-transformation-and-source-edit-algorithms-typed-03.rs" skip=500
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

This preparation compiles in the cumulative browser application and preserves the current drawing. Its transforms and control views connect to the later editing tools.

### Recorded visual result

**Previous native render — current verification pending.**

Original Circle, Ellipse, Arc, Parabola and freeform Curve remain visible.

![Native renderer output for build directional scale transforms.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

<details>
<summary>Code explanation and diagram</summary>

factor rejects a degenerate reference or nonpositive result. stretch writes a column-major transform as I+(k−1)Σaaᵀ, then chooses translation so the fixed origin maps to itself. Reuse the Session Xform owner; later Scale tools connect these pure calculations to previews and history.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



How does directional scale preserve the other directions?

Add (k−1) times each chosen axis outer product to the identity matrix.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

For k=2 along the x axis, predict the images of [1,0,0] and [0,1,0]. Repeat about origin [5,0,0]; explain the translation term.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.1
npm --prefix ../session_tests run course -- save 15.1.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
