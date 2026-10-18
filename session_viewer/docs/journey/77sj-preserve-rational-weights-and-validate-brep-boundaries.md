# 15.1.11 · Preserve rational weights and validate BRep boundaries

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 23–46 minutes.** [Estimate](typing-load.md).

For rational curves, store the moved Euclidean point multiplied by its unchanged weight.

## Type

Continue from [Collect affected points across CAD geometry families](77si-collect-affected-points-across-cad-geometry-families.md). [Save or recover your work](recovery.md).

### 1. `src/source_deform.rs`

Preserve rational weights and validate BRep boundaries.

<details>
<summary>Locate the existing block</summary>

```rust
                points(&Geometry::Mesh(Rc::new(mesh.clone())), target)
            }
            session_rust::element::ElementGeometry::BRep(brep) => {
                points(&Geometry::BRep(Rc::new(brep.clone())), target)
            }
            _ => Err("Element has no source geometry".into()),
        },
        _ => match target {
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
            _ => Err("This source has no editable faces or edges".into()),
        },
    }
}
```

</details>

After that block, type:

```rust

fn set_curve_control(curve: &mut session_rust::NurbsCurve, index: usize, point: &Point) -> bool {
    if !curve.m_is_rat {
        return curve.set_cv_point(index, point);
    }
    let Some((_, _, _, weight)) = curve.get_cv_4d(index) else {
        return false;
    };
    curve.set_cv_4d(index, point[0] * weight, point[1] * weight, point[2] * weight, weight)
}

/// Refuse an edit whose edges no longer lie on their surfaces.
fn validate_boundaries(
    brep: &session_rust::BRep,
    changed_curves: &HashSet<usize>,
    changed_surfaces: &HashSet<usize>,
) -> Result<(), String> {
    if !brep.is_valid() {
        return Err("Edit would invalidate BRep topology".into());
    }

    for edge in &brep.m_edges {
        if edge.degenerated
            || (!changed_curves.contains(&(edge.curve_3d_index as usize))
                && !edge
                    .pcurves
                    .iter()
                    .any(|pc| changed_surfaces.contains(&(pc.surface_index as usize))))
        {
            continue;
        }

        let curve = &brep.m_curves_3d[edge.curve_3d_index as usize];
        let (a, b) = curve.domain();

        // sample the 3D edge against each surface's 2D curve
        for pc in &edge.pcurves {
            let surface = &brep.m_surfaces[pc.surface_index as usize];

            for ci in [pc.curve_2d_index, pc.curve_2d_index_2] {
                if ci < 0 {
                    continue;
                }

                let uv = &brep.m_curves_2d[ci as usize];
                let (u0, u1) = uv.domain();

                for sample in 0..=16 {
                    let t = sample as f64 / 16.0;
                    let p = curve.point_at(a + (b - a) * t);
                    let q = uv.point_at(u0 + (u1 - u0) * t);
                    let actual = surface
                        .point_at(q[0], q[1])
                        .ok_or("Cannot evaluate incident surface")?;
                    let tolerance = edge.tolerance.max(1e-6) * 10.0;

                    if (0..3).any(|i| (p[i] - actual[i]).abs() > tolerance) {
                        return Err("This BRep edit requires rebuilding adjacent trims; the original solid was preserved".into());
                    }
                }
            }
        }
    }

    Ok(())
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sj-preserve-rational-weights-and-validate-brep-boundaries-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.11
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

Original Circle, Ellipse, Arc, Parabola and freeform Curve remain visible.

![Native renderer output for preserve rational weights and validate brep boundaries.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Before committing a BRep edit, validate topology and sample changed incident3D edges against their2D trim curves on the adjacent surfaces. Reject an edit that requires trim rebuilding; the caller can then retain the original solid atomically. This bounded check is not a general trim reconstruction algorithm.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Why preserve a rational control weight when moving its point?

The Euclidean point changes, while its original rational weighting remains part of the curve definition.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Explain why translating one boundary curve without its adjacent surface may be rejected. Predict the homogeneous coordinates for point[2,3,4] with weight2.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.11
npm --prefix ../session_tests run course -- save 15.1.11
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
