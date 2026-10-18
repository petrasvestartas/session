# 15.1.15 · Dispatch original part edits and retain Element ownership

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Validate the requested transform once, then dispatch to the source helpers written in the preceding lessons.

## Type

Continue from [Transform shared BRep boundary positions coherently](77sm-transform-shared-brep-boundary-positions-coherently.md). [Save or recover your work](recovery.md).

### 1. `src/source_deform.rs`

Dispatch original part edits and retain Element ownership.

<details>
<summary>Locate the existing block</summary>

```rust
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

</details>

After that block, type:

```rust

fn transform_element(source: &Rc<session_rust::Element>, target: Target, delta: &Xform) -> Result<Geometry, String> {
    let mut next = (**source).clone();

    match source.geometry() {
        session_rust::element::ElementGeometry::Mesh(mesh) => {
            let Geometry::Mesh(mesh) =
                transform(&Geometry::Mesh(Rc::new(mesh.clone())), target, delta)?
            else {
                unreachable!()
            };
            next.set_geometry((*mesh).clone());
        }
        session_rust::element::ElementGeometry::BRep(brep) => {
            let Geometry::BRep(brep) =
                transform(&Geometry::BRep(Rc::new(brep.clone())), target, delta)?
            else {
                unreachable!()
            };
            next.set_brep_geometry((*brep).clone());
        }
        _ => return Err("Element has no source geometry".into()),
    }

    Ok(Geometry::Element(Rc::new(next)))
}

/// A copy of the geometry with the target moved by `delta`.
pub fn transform(geometry: &Geometry, target: Target, delta: &Xform) -> Result<Geometry, String> {
    if !delta.m.iter().all(|v| v.is_finite()) {
        return Err("Transform must be finite".into());
    }
    match geometry {
        Geometry::Mesh(source) => transform_mesh(source, target, delta),
        Geometry::NurbsSurface(source) => transform_nurbssurface(source, target, delta),
        Geometry::Polyline(source) => transform_polyline(source, target, delta),
        Geometry::NurbsCurve(source) => transform_nurbscurve(source, target, delta),
        Geometry::Line(source) => transform_line(source, target, delta),
        Geometry::Point(source) => transform_point(source, target, delta),
        Geometry::BRep(source) => transform_brep(geometry, source, target, delta),
        Geometry::Element(source) => transform_element(source, target, delta),
        _ => Err("This source control cannot be transformed".into()),
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77sn-dispatch-original-part-edits-and-retain-element-ownership-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.15
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

![Native renderer output for dispatch original part edits and retain element ownership.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Element editing recursively reuses Mesh/BRep transformations and retains the Element wrapper and original feature payload. The independent regressions are registered after the original mesh-topology helpers are complete, because their hole/winding checks depend on that module. These helpers connect to the existing preview and history tools afterward.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Does moving an Element control create a separate editable display mesh?

No. Update its original Mesh/BRep payload inside the cloned Element.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Predict the before/after owner identity when one Element mesh control moves. Trace an invalid transform through the dispatcher without mutating the source.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.15
npm --prefix ../session_tests run course -- save 15.1.15
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
