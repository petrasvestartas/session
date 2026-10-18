# 5.4.2 · Sample a validated curve without replacing its source

**Chapter 5 · Lines, points and curves · Section 5.4 · NURBS curves** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Use the control polygon’s turning to choose display chords, matching the current viewer’s three-degree policy and 512 uniform-chord cap.

## Type

Continue from [Validate an original NURBS curve before sampling](38-layout.md). [Save or recover your work](recovery.md).

### 1. `src/curve.rs`

Sample the original evaluator with bounded uniform chords and exact interior knot breaks.

<details>
<summary>Locate the existing block</summary>

```rust
    Ok(domain)
}
```

</details>

After that block, type:

```rust

pub fn sample(source: &NurbsCurve) -> Result<Vec<[f64; 3]>, &'static str> {
    let (start, end) = validate(source)?;
    let mut turning = 0.0; let mut previous: Option<[f64; 3]> = None;
    let control = |index: usize| {
        let offset = index * source.m_cv_stride;
        let weight = if source.m_is_rat { source.m_cv[offset + source.m_dim] } else { 1.0 };
        std::array::from_fn::<_, 3, _>(|axis| if axis < source.m_dim { source.m_cv[offset + axis] / weight } else { 0.0 })
    };
    for index in 1..source.m_cv_count {
        let a = control(index - 1); let b = control(index);
        let direction: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
        let length = direction.iter().map(|v| v * v).sum::<f64>().sqrt();
        if length <= 1e-12 { continue; }
        if !length.is_finite() { return Err("Curve control extent exceeds the display range"); }
        let unit = direction.map(|v| v / length);
        if let Some(before) = previous {
            let dot = (0..3).map(|axis| before[axis] * unit[axis]).sum::<f64>().clamp(-1.0, 1.0);
            turning += dot.acos().to_degrees();
        }
        previous = Some(unit);
    }
    let count = ((turning / 3.0).ceil() as usize).clamp(1, 512);
    let mut parameters: Vec<_> = (0..=count).map(|i| start + (end - start) * i as f64 / count as f64).collect();
    parameters.extend(source.m_nurbsknot.iter().copied().filter(|&t| t > start && t < end));
    parameters.sort_by(f64::total_cmp); parameters.dedup();
    let mut points = Vec::with_capacity(parameters.len());
    for parameter in parameters {
        let point = source.point_at(parameter); let point: [f64; 3] = std::array::from_fn(|axis| point[axis]);
        if point.iter().any(|&v| !(v as f32).is_finite()) { return Err("Sampled curve exceeds the display range"); }
        points.push(point);
    }
    Ok(points)
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/38a-samples-01.rs"
```

</details>

### 2. `src/lib.rs`

Register actual sampled geometry checks.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod curve;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod curve_sample_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/38a-samples-02.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/curve_sample_tests.rs`

Copy original preservation, dense measured error, periodic closure and knot-break checks.

Copy this supplied file:

```text
--8<-- "journey/code/38a-samples-03.rs"
```

</details>

## Run and check

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Run straight, curved, periodic and malformed curve sampling checks. Original control arrays remain unchanged.

### Recorded visual result

**Previous native render — current verification pending.**

This checkpoint was rendered independently and has the same native comparison frame as [Validate an original NURBS curve before sampling](38-layout.md). Use its state checks and browser exercise to assess the changed behavior.

![Native renderer output for sample a validated curve without replacing its source.](../screenshots/journey/37d-render.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [5.3.5 · Draw original point rows and count their resources](37d-render.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Include original interior knot breaks as well. Bound the control count before allocating, keep sample positions in double precision, and refuse nonfinite or unrepresentable display coordinates. A sampled polyline remains an approximation rather than a replacement kernel curve.

validated original curve → control-polygon turning → bounded parameter samples and knot breaks → finite double-precision chords.

![Original kernel evaluations create a bounded display approximation.](../illustrations/journey-38a.svg)

What does the sampled polyline know that the original curve does not lose?

It provides drawing chords at chosen parameters. The original knots, degree, rational weights and exact controls remain the editing source; the chords are explicitly an approximation.

Study estimate, including typing and experiments: 0.75–1.25 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare dense kernel evaluations with the sampled quadratic arch. Check measured chord error, rather than mistaking its control polygon for the curve.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 5.4.2
npm --prefix ../session_tests run course -- save 5.4.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Curve display sampling is complete. Original curve rows, control markers and real input follow in the remaining curve checkpoints.

Native checks prove exact original preservation, straight minimal samples, real quadratic arch shape and measured dense chord error, periodic endpoints, knot-break inclusion and bounded malformed-input refusal.

Chrome retains point and connected rendering while native checks guard original curve evaluation.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 5.4.2
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
