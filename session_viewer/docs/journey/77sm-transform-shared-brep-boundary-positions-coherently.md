# 15.1.14 · Transform shared BRep boundary positions coherently

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 19–37 minutes.** [Estimate](typing-load.md).

Collect the selected source positions and clone the BRep.

## Type

Continue from [Transform original line polyline point and curve controls](77sl-transform-original-line-polyline-point-and-curve-controls.md). [Save or recover your work](recovery.md).

### 1. `src/source_deform.rs`

Transform shared BRep boundary positions coherently.

<details>
<summary>Locate the existing block</summary>

```rust
    next.name = source.name.clone();
    next.width = source.width;
    next.dash = source.dash.clone();
    next.linecolor = source.linecolor.clone();
    next.arrowhead = source.arrowhead;
    Ok(Geometry::Line(Rc::new(next)))
}

fn transform_point(source: &Rc<session_rust::Point>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(0)) = target else {
        return Err("Unknown point".into());
    };
    Ok(Geometry::Point(Rc::new(source.transformed(delta))))
}
```

</details>

After that block, type:

```rust

fn transform_brep(geometry: &Geometry, source: &Rc<session_rust::BRep>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let selected = points(geometry, target)?;
    let mut next = (**source).clone();
    // every vertex, curve or surface point at a selected position moves
    let matches = |p: &Point| {
        selected
            .iter()
            .any(|s| (0..3).all(|i| (s[i] - p[i]).abs() <= 1e-8))
    };

    for vertex in &mut next.m_vertices {
        if matches(&vertex.point) {
            vertex.point = vertex.point.transformed(delta);
        }
    }

    let mut changed_curves = HashSet::new(); // curves touched
    let mut changed_surfaces = HashSet::new(); // surfaces touched

    for (curve_index, curve) in next.m_curves_3d.iter_mut().enumerate() {
        for i in 0..curve.cv_count() {
            if let Some(p) = curve.get_cv(i)
                && matches(&p)
            {
                if !set_curve_control(curve, i, &p.transformed(delta)) {
                    return Err("Cannot set curve control".into());
                }
                changed_curves.insert(curve_index);
            }
        }
    }

    for (surface_index, surface) in next.m_surfaces.iter_mut().enumerate() {
        for u in 0..surface.m_cv_count[0] {
            for v in 0..surface.m_cv_count[1] {
                if let Some(p) = surface.get_cv(u, v)
                    && matches(&p)
                {
                    surface.set_cv(u, v, &p.transformed(delta));
                    changed_surfaces.insert(surface_index);
                }
            }
        }

        if changed_surfaces.contains(&surface_index) {
            surface.m_mesh = None; // drop the cached mesh
        }
    }

    validate_boundaries(&next, &changed_curves, &changed_surfaces)?;
    Ok(Geometry::BRep(Rc::new(next)))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sm-transform-shared-brep-boundary-positions-coherently-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.14
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

![Native renderer output for transform shared brep boundary positions coherently.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Move coincident vertex, curve and surface controls within the source tolerance; record touched curves and surfaces, and invalidate changed surface meshes. Validate affected boundaries before returning the new solid. A failed validation returns an error and leaves the original geometry available to the transaction.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Why move coincident BRep vertices curve controls and surface controls together?

They describe connected parts of the same original boundary, so moving only one representation can break consistency.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a boundary point present in a vertex, curve and surface. Predict which caches and incident-boundary checks must run when it moves.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.14
npm --prefix ../session_tests run course -- save 15.1.14
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
