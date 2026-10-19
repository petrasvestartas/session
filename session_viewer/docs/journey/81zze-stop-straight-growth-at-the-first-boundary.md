# 16.1.6 · Stop straight growth at the first boundary

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 22–44 minutes.** [Estimate](typing-load.md).

Find the nearest positive crossing of original planes, curves and transformed meshes along a straight end.

## Type

Continue from [Find original end directions and length reach](81zzd-find-original-end-directions-and-length-reach.md). [Save or recover your work](recovery.md).

### 1. `src/extend_reach.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
            return Some((curve.point_at(t), direction));
        }
        _ => return None,
    };
    let mut direction = &tip - &inward;
    direction.normalize_self().then_some(())?;
    Some((tip, direction))
}
```

</details>

After that block, type:

```rust
/// Where a straight end lands: a distance on, or the first boundary ahead.
fn straight(from: &Point, direction: &Vector, reach: &Reach) -> Result<Point, String> {
    let (cutters, meshes) = match reach {
        Reach::Distance(distance) => return Ok(from + &(direction * *distance)),
        Reach::Boundaries { cutters, meshes } => (cutters, meshes),
    };
    let mut best: Option<f64> = None; // distance ahead of the nearest boundary
    let mut take = |t: f64| {
        if t > NEAR && best.is_none_or(|near| t < near) {
            best = Some(t);
        }
    };

    for cutter in cutters {
        match cutter {
            Cutter::Plane(plane) => {
                let ray = Line::from_points(from, &(from + direction));

                if let Some(hit) = intersection::line_plane(&ray, plane, false) {
                    take((&hit - from).dot(direction));
                }
            }
            Cutter::Curve(curve) => {
                // a probe reaching past every control point of the boundary
                let reach = (0..curve.m_cv_count)
                    .filter_map(|i| curve.get_cv(i))
                    .map(|p| p.distance(from, None))
                    .fold(1.0, f64::max)
                    * 2.0;
                let probe =
                    NurbsCurve::create(false, 1, &[from.clone(), from + &(direction * reach)]);

                if let Ok(pieces) =
                    split::split_curve_by_curves(&probe, std::slice::from_ref(curve), NEAR)
                {
                    for piece in pieces.iter().take(pieces.len().saturating_sub(1)) {
                        take((&piece.point_at_end() - from).dot(direction));
                    }
                }
            }
        }
    }

    for (mesh, into) in meshes {
        let Some(back) = into.inverse() else {
            continue;
        };
        let ray = unit_ray(from, direction, into);

        for hit in intersection::ray_mesh(&ray, mesh, 1e-12, true).unwrap_or_default() {
            take((&hit.transformed(&back) - from).dot(direction));
        }
    }

    best.map(|t| from + &(direction * t))
        .ok_or_else(|| "no boundary ahead".into())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/81zze-stop-straight-growth-at-the-first-boundary-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.6
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Straight extension can stop at the first original boundary before editing input is connected.

**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for stop straight growth at the first boundary.](../screenshots/journey/80za-read-original-projection-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original cutter → object frame → physical reach → edited curve.



Why ignore a crossing at the old end?

It does not add a segment. Only a crossing ahead beyond the tolerance can bound growth.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare crossings behind, at and ahead of the old end.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.6
npm --prefix ../session_tests run course -- save 16.1.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original cutter and reach helpers prepare cancellable editing of the original curve.



[Full validation scope](release.md).

</details>
