# 16.4.18 · Preserve trimmed source types

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Rebuild kept curves in their original source family with their presentation. Wrap a surface as a single-face BRep using the existing kernel split API.

## Type

Continue from [Collect curve crossings](84zzc-collect-curve-crossings.md). [Save or recover your work](recovery.md).

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

<details>
<summary>Locate the existing block</summary>

```rust
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
}
```

</details>

After that block, type:

```rust
/// The run `a..b` of `curve` as the source's own type, with its name and pen.
fn rebuilt(source: &Geometry, curve: &NurbsCurve, a: f64, b: f64) -> Option<Geometry> {
    let part = piece(curve, a, b)?;

    match source {
        Geometry::Line(line) => {
            let mut next = Line::from_points(&part.point_at_start(), &part.point_at_end());
            next.name = line.name.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.linecolor = line.linecolor.clone();
            Some(Geometry::Line(Rc::new(next)))
        }
        Geometry::Polyline(polyline) => {
            let mut next = Polyline::new(samples(&part, 1));
            next.name = polyline.name.clone();
            next.width = polyline.width;
            next.dash = polyline.dash.clone();
            next.linecolor = polyline.linecolor.clone();
            Some(Geometry::Polyline(Rc::new(next)))
        }
        Geometry::NurbsCurve(original) => {
            let mut next = part;
            next.refresh_guid();
            next.name = original.name.clone();
            next.width = original.width;

            // per-control colours only fit the control count they were made for
            if next.pointcolors.len() != next.m_cv_count {
                next.pointcolors.clear();
            }

            if next.linecolors.len() + 1 != next.m_cv_count {
                next.linecolors.clear();
            }

            Some(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => None,
    }
}

/// A surface as a one-face BRep, cut by nothing yet.
fn face_of(surface: &NurbsSurface) -> Result<BRep, String> {
    let corner = surface
        .get_cv(0, 0)
        .ok_or("Surface has no control points")?;
    let size = planar_size(surface);
    // a cutter far off the surface cuts nothing, so the kernel only wraps it
    let far = &corner + &Vector::new(size * 10.0, size * 10.0, size * 10.0);
    let away = NurbsCurve::create(
        false,
        1,
        &[far.clone(), &far + &Vector::new(size, 0.0, 0.0)],
    );
    let mut brep = split::split_surface_by_curves(surface, &[away], TOLERANCE)?;
    brep.name = surface.name.clone();
    Ok(brep)
}

/// The largest control point coordinate of a surface, at least 1.
fn planar_size(surface: &NurbsSurface) -> f64 {
    let mut size: f64 = 1.0;

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                size = size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs());
            }
        }
    }

    size
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/84zzd-preserve-trimmed-source-types-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.18
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

![Native renderer output for preserve trimmed source types.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD geometry → removable parts → retained source geometry.



Why discard incompatible per-control colors?

A trimmed curve can have a different control count; stale color arrays no longer describe it.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace which line properties survive trimming and when a NURBS color array is cleared.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.18
npm --prefix ../session_tests run course -- save 16.4.18
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
