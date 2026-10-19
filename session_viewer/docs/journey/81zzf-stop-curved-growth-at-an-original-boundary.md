# 16.1.7 · Stop curved growth at an original boundary

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 30–60 minutes.** [Estimate](typing-load.md).

Grow the original NURBS domain and inspect curve or plane crossings beyond its old end. Trim the extension at the first accepted boundary.

## Type

Continue from [Stop straight growth at the first boundary](81zze-stop-straight-growth-at-the-first-boundary.md). [Save or recover your work](recovery.md).

### 1. `src/extend_reach.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        let s2 = (s1 - f1 * (s1 - s0) / (f1 - f0)).max(s1 * 0.1);
        (s0, f0) = (s1, f1);
        s1 = s2;
        f1 = added(s1)? - distance;
    }

    grow(curve, end, s1).ok_or_else(|| "Cannot extend this curve".into())
}
```

</details>

After that block, type:

```rust
/// A curve grown along its own shape to the first boundary curve or plane it meets.
fn to_boundary(curve: &NurbsCurve, end: End, cutters: &[Cutter]) -> Result<NurbsCurve, String> {
    if cutters.is_empty() {
        return Err(
            "Meshes and BReps stop lines and polylines; pick a curve or plane for a curve".into(),
        );
    }

    let (d0, d1) = curve.domain();
    let at = if end == End::Start { d0 } else { d1 };
    let from = curve.point_at(at);
    let speed = curve.evaluate(at, 1)[1].magnitude().max(1e-12);
    // how far the nearest boundary is, as a first guess
    let away = cutters
        .iter()
        .map(|cutter| match cutter {
            Cutter::Plane(plane) => (&from - &plane.origin()).dot(&plane.z_axis()).abs(),
            Cutter::Curve(boundary) => boundary.closest_point(&from).distance(&from, None),
        })
        .fold(f64::INFINITY, f64::min)
        .max(curve.length(None) * 0.05);
    let mut s = away / speed;

    for _ in 0..7 {
        let grown = grow(curve, end, s).ok_or("Cannot extend this curve")?;
        let piece = beyond(curve, &grown, end).ok_or("Cannot extend this curve")?;
        let (p0, p1) = piece.domain();
        let near = (p1 - p0) * 1e-9;
        let mut crossings = Vec::new();

        for cutter in cutters {
            match cutter {
                Cutter::Plane(plane) => {
                    crossings.extend(intersection::curve_plane(&piece, plane, Some(NEAR)))
                }
                Cutter::Curve(boundary) => {
                    let pieces = split::split_curve_by_curves(
                        &piece,
                        std::slice::from_ref(boundary),
                        NEAR,
                    )
                    .unwrap_or_default();

                    for part in pieces.iter().take(pieces.len().saturating_sub(1)) {
                        crossings.push(
                            intersection::curve_closest_point(&piece, &part.point_at_end(), p0, p1)
                                .0,
                        );
                    }
                }
            }
        }

        // the crossing nearest the old end
        let first = match end {
            End::End => crossings
                .into_iter()
                .filter(|t| *t > p0 + near)
                .min_by(f64::total_cmp),
            End::Start => crossings
                .into_iter()
                .filter(|t| *t < p1 - near)
                .max_by(f64::total_cmp),
        };

        if let Some(t) = first {
            let (g0, g1) = grown.domain();
            let mut cut = grown;
            let done = match end {
                End::End => cut.trim(g0, t),
                End::Start => cut.trim(t, g1),
            };
            return done
                .then_some(cut)
                .ok_or_else(|| "Cannot cut the curve at the boundary".into());
        }

        s *= 2.0;
    }

    Err("does not reach the boundary".into())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/81zzf-stop-curved-growth-at-an-original-boundary-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.7
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Curved extension can stop at an original boundary while retaining its source geometry.

**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for stop curved growth at an original boundary.](../screenshots/journey/80za-read-original-projection-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original cutter → object frame → physical reach → edited curve.



Why test the gained domain?

Crossings inside the original curve do not bound the new segment.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace both end directions through increasing domain growth.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.7
npm --prefix ../session_tests run course -- save 16.1.7
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original cutter and reach helpers prepare cancellable editing of the original curve.



[Full validation scope](release.md).

</details>
