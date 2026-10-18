# 15.1.10 · Collect affected points across CAD geometry families

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 24–47 minutes.** [Estimate](typing-load.md).

Dispatch the selected target to the existing source key and control helpers.

## Type

Continue from [Resolve surface grids and individual source controls](77sh-resolve-surface-grids-and-individual-source-controls.md). [Save or recover your work](recovery.md).

### 1. `src/source_deform.rs`

Collect affected points across CAD geometry families.

<details>
<summary>Locate the existing block</summary>

```rust
                _ => v + 1 == nv,
            })
            .collect()),
        _ => Err("Unknown surface control, boundary or face".into()),
    }
}

/// One control point of any geometry.
fn control_point(geometry: &Geometry, id: ControlId) -> Result<Point, String> {
    Controls::from_geometry(geometry)
        .points
        .into_iter()
        .find(|p| p.id == id)
        .map(|p| Point::new(p.position[0], p.position[1], p.position[2]))
        .ok_or("Unknown source control".into())
}
```

</details>

After that block, type:

```rust

/// The world points a target covers.
pub fn points(geometry: &Geometry, target: Target) -> Result<Vec<Point>, String> {
    match geometry {
        Geometry::Mesh(mesh) => mesh_keys(mesh, target)?
            .into_iter()
            .map(|k| mesh.vertex_point(k).ok_or("Missing mesh vertex".into()))
            .collect(),
        Geometry::NurbsSurface(surface) => surface_keys(surface, target)?
            .into_iter()
            .map(|(u, v)| surface.get_cv(u, v).ok_or("Missing surface control".into()))
            .collect(),
        Geometry::BRep(brep) => match target {
            Target::Face(face) => {
                let face = brep.m_faces.get(face).ok_or("Unknown BRep face")?;
                let surface = brep
                    .m_surfaces
                    .get(face.surface_index as usize)
                    .ok_or("Missing face surface")?;
                Ok((0..surface.m_cv_count[0])
                    .flat_map(|u| {
                        (0..surface.m_cv_count[1]).filter_map(move |v| surface.get_cv(u, v))
                    })
                    .collect())
            }
            Target::Edge(edge) => {
                let edge = brep.m_edges.get(edge as usize).ok_or("Unknown BRep edge")?;
                let curve = brep
                    .m_curves_3d
                    .get(edge.curve_3d_index as usize)
                    .ok_or("Degenerate edge has no movable curve")?;
                Ok((0..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect())
            }
            Target::Control(id) => control_point(geometry, id).map(|p| vec![p]),
        },
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) => {
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77si-collect-affected-points-across-cad-geometry-families-typed-01.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.10
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

![Native renderer output for collect affected points across cad geometry families.](../screenshots/journey/77-copy-current-transformation-and-source-edit-algorithms.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.1 · Build directional scale transforms](77-copy-current-transformation-and-source-edit-algorithms.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

A BRep face uses its incident surface controls and an edge uses its original3D curve controls. Element payloads reuse the same Mesh/BRep resolvers. This collects edit anchors without changing source geometry or display placement.

original f64 CAD source → pure transform or original control addresses → later edit preview and GPU display.



Does editing an Element require flattening its source payload?

No. Reuse the Mesh or BRep point resolver while retaining the original Element wrapper.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare points returned for a Mesh and an Element containing that Mesh. Predict the rejection for a face request on an original Line.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.10
npm --prefix ../session_tests run course -- save 15.1.10
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



Original edit preparation; the later connected command and control-picking browser exercises establish interaction behavior.

[Full validation scope](release.md).

</details>
