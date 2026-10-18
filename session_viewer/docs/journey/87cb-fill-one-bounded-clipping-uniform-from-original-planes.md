# 17.2.9 · Fill one bounded clipping uniform from original planes

**Chapter 17 · GPU visibility, clipping and sections · Section 17.2 · Clipping half-spaces** · [Contents](contents.md)

**Typing: 19–38 minutes.** [Estimate](typing-load.md).

Start with an empty uniform when there is no plane or no valid inverse.

## Type

Continue from [Project a stable hatch coordinate onto a plane](87ca-project-a-stable-hatch-coordinate-onto-a-plane.md). [Save or recover your work](recovery.md).

### 1. `src/clipping_uniform.rs`

Populate world and screen half-spaces and reuse the hatch projection helper.

<details>
<summary>Locate the existing block</summary>

```rust
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

</details>

After that block, type:

```rust
--8<-- "journey/code/87cb-fill-one-bounded-clipping-uniform-from-original-planes-typed-1.rs" skip=500
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 17.2.9
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

![Native renderer output for fill one bounded clipping uniform from original planes.](../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [17.1.52 · Exercise finite color and original picking](86aq-exercise-finite-color-and-original-picking.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Bound the plane count by the existing six-plane layout. Translate original plane offsets relative to the camera anchor, transform the coefficients by the inverse, and record which side contains the eye. Convert CSS spacing and boundary widths into physical pixels once. Edge-on planes keep their half-space but receive no hatch projection.

original clipping plane → anchor-relative projection → shared pixel and sample coverage.



Why is an edge-on plane still retained?

It still clips geometry. Its screen depth cannot be solved when the depth coefficient is zero, so it cannot produce a section cap.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict count, eye side and hatch rows for an empty list, an edge-on plane and a plane facing the camera.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 17.2.9
npm --prefix ../session_tests run course -- save 17.2.9
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Preparation. The later clipping and section browser contracts exercise the completed GPU passes.

[Full validation scope](release.md).

</details>
