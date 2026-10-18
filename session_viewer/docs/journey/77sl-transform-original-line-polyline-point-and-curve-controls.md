# 15.1.13 · Transform original line polyline point and curve controls

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 21–41 minutes.** [Estimate](typing-load.md).

Use each original control address to move one point in a cloned source.

## Type

Continue from [Transform original mesh and surface controls](77sk-transform-original-mesh-and-surface-controls.md). [Save or recover your work](recovery.md).

### 1. `src/source_deform.rs`

Transform original line polyline point and curve controls.

<details>
<summary>Locate the existing block</summary>

```rust
fn transform_nurbssurface(source: &Rc<session_rust::NurbsSurface>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut surface = (**source).clone();

    for (u, v) in surface_keys(&surface, target)? {
        let p = surface
            .get_cv(u, v)
            .ok_or("Missing surface control")?
            .transformed(delta);

        if !surface.set_cv(u, v, &p) {
            return Err("Cannot set surface control".into());
        }
    }

    surface.m_mesh = None; // drop the cached mesh
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}
```

</details>

After that block, type:

```rust

fn transform_polyline(source: &Rc<session_rust::Polyline>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(i)) = target else {
        return Err("Select a polyline vertex".into());
    };
    let p = source
        .get_point(i)
        .ok_or("Unknown polyline vertex")?
        .transformed(delta);
    let mut next = (**source).clone();
    next.set_point(i, &p);
    Ok(Geometry::Polyline(Rc::new(next)))
}

fn transform_nurbscurve(source: &Rc<session_rust::NurbsCurve>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Curve { curve: 0, point }) = target else {
        return Err("Select a curve control point".into());
    };
    let p = source
        .get_cv(point)
        .ok_or("Unknown curve control")?
        .transformed(delta);
    let mut next = (**source).clone();

    if !set_curve_control(&mut next, point, &p) {
        return Err("Cannot set curve control".into());
    }

    Ok(Geometry::NurbsCurve(Rc::new(next)))
}

fn transform_line(source: &Rc<session_rust::Line>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let Target::Control(ControlId::Vertex(i @ 0..=1)) = target else {
        return Err("Select a line endpoint".into());
    };
    let mut ends = [source.start(), source.end()];
    ends[i] = ends[i].transformed(delta);
    let mut next = session_rust::Line::from_points(&ends[0], &ends[1]);
    next.set_guid(source.guid().to_string());
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sl-transform-original-line-polyline-point-and-curve-controls-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.13
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

![Native renderer output for transform original line polyline point and curve controls.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Reuse the rational curve setter from the preceding lesson. Rebuilding a Line from its two endpoints must restore its original identity and display metadata; changing its shape does not create a new object. Point and polyline edits retain their corresponding source owners.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Which Line metadata must survive an endpoint edit?

Its original GUID, name, width, dash, colour and arrowhead remain unchanged.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the changed endpoint and unchanged metadata when only Line vertex1 moves. Compare a rational curve edit with a polyline vertex edit.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.13
npm --prefix ../session_tests run course -- save 15.1.13
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
