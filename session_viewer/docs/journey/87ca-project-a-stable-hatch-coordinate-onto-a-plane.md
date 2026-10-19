# 17.2.8 · Project a stable hatch coordinate onto a plane

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 11–21 minutes.** [Estimate](typing-load.md).

Solve the screen plane for depth, then transform three screen columns back into anchor-relative world space.

## Type

Continue from [Condition the camera inverse for clipping](87c-copy-conditioned-screen-and-hatch-projections.md). [Save or recover your work](recovery.md).

### 1. `src/clipping_uniform.rs`

Build homogeneous hatch numerator and denominator from the projected plane.

<details>
<summary>Locate the existing block</summary>

```rust
fn inverse(m: &Xform) -> Option<Xform> {
    let scales: [f64; 4] = std::array::from_fn(|row| {
        (0..4)
            .map(|col| m.m[col * 4 + row].abs())
            .fold(0.0, f64::max)
    });

    if scales.iter().any(|scale| !scale.is_normal()) {
        return None;
    }

    let mut inverse =
        Xform::from_matrix(std::array::from_fn(|i| m.m[i] / scales[i % 4])).inverse()?;

    for (i, value) in inverse.m.iter_mut().enumerate() {
        *value /= scales[i / 4];
    }

    Some(inverse)
}
```

</details>

After that block, type:

```rust

fn hatch_projection(u: &mut ClipUniform, i: usize, plane: &ClipPlane, s: [f64; 4], inverse: &Xform, v: &ClipView, period: f64) {
        // a plane point over canvas (x, y) is inverse · (x, y, depth(x, y), 1), homogeneous
        let columns = [
            [1.0, 0.0, -s[0] / s[2], 0.0],
            [0.0, 1.0, -s[1] / s[2], 0.0],
            [0.0, 0.0, -s[3] / s[2], 1.0],
        ];
        let from = [
            v.anchor[0] - plane.origin[0],
            v.anchor[1] - plane.origin[1],
            v.anchor[2] - plane.origin[2],
        ];
        let phase = dot(plane.hatch, from);
        let phase = if period.is_normal() {
            phase.rem_euclid(period)
        } else {
            phase
        };

        for (c, column) in columns.iter().enumerate() {
            let h: [f64; 4] =
                std::array::from_fn(|r| (0..4).map(|q| inverse.m[q * 4 + r] * column[q]).sum());
            u.hatch[i][c] = (dot(plane.hatch, [h[0], h[1], h[2]]) + phase * h[3]) as f32;
            u.hatch_w[i][c] = h[3] as f32;
        }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/87ca-project-a-stable-hatch-coordinate-onto-a-plane-typed-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.8
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

Actual finite visibility keeps the front red originals and blue solid visible while suppressing the hidden rear stroke.

![Native renderer output for project a stable hatch coordinate onto a plane.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Dot their spatial components with the hatch direction. Keep the homogeneous w separately: perspective interpolation needs numerator divided by denominator. Wrap the distant anchor phase by a power-of-two period to preserve the local hatch position when converting to f32.

original clipping plane → anchor-relative projection → shared pixel and sample coverage.



Why keep hatch_w instead of storing one affine hatch coordinate?

Perspective division is not affine in screen space. The numerator and denominator must be evaluated at the fragment before division.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace the three screen columns for an orthographic plane, then predict how a perspective denominator varies across the canvas.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.8
npm --prefix ../session_tests run course -- save 17.2.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Preparation. The later clipping and section browser contracts exercise the completed GPU passes.

[Full validation scope](release.md).

</details>
