# 15.1.43 · Align original sections and loft through them

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 23–45 minutes.** [Estimate](typing-load.md).

Align open curve directions and closed curve seams before asking the Session kernel for a loft. Keep section count and degree rules in the same helper. Supply the original validation fixtures separately; the construction recipes connect this owner next.

## Type

Continue from [Check original loops and surface controls](77zzh-check-original-loops-and-surface-controls.md). [Save or recover your work](recovery.md).

### 1. `src/surfacing.rs`

Add section alignment and lofting before the existing result validator.

<details>
<summary>Locate the existing block</summary>

```rust
/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!("The {name} surface could not be built"));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked.iter().map(|picked| picked.curve.clone()).collect()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/77zzi-align-original-sections-and-loft-through-them-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/surfacing.rs`

Supply the five original surfacing validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked.iter().map(|picked| picked.curve.clone()).collect()
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/77zzi-align-original-sections-and-loft-through-them-2.rs" skip=160
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.43
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

![Native renderer output for align original sections and loft through them.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curves → consistent section frames → validated kernel surface.



Why align sections before building the loft?

Opposite directions or mismatched closed seams can twist the surface between otherwise valid original curves.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace two open sections with opposite directions, then a closed section with a shifted seam. The existing browser drawing remains unchanged until the following recipe connection.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.43
npm --prefix ../session_tests run course -- save 15.1.43
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original surfacing helpers are preparatory. Extrude, revolve and the later surface recipes use this owner; the existing browser drawing is unchanged here.



[Full validation scope](release.md).

</details>
