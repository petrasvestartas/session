# 16.2.1 · Divide original curves and joined faces

**Chapter 16 · Geometry editing and measurement · Section 16.2 · Curve pieces and joined faces** · [Contents](contents.md)

**Typing: 27–54 minutes.** [Estimate](typing-load.md).

Bound the cutter count and dispatch original lines, polylines, curves, surfaces and selected BRep faces to the Session split algorithms. Return derived geometry and the number of regions without editing the scene.

## Type

Continue from [Validate original Split inputs and name pieces](81za-validate-original-split-inputs-and-name-pieces.md). [Save or recover your work](recovery.md).

### 1. `src/split_geometry.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
        Geometry::Polyline(polyline) => Ok(NurbsCurve::create(false, 1, &polyline.get_points())),
        Geometry::NurbsCurve(curve) => Ok((**curve).clone()),
        _ => Err("Choose a line, polyline or NURBS curve as cutter".into()),
    }
}
```

</details>

After that block, type:

```rust
pub fn divided(
    source: &Geometry,
    face: Option<usize>,
    tools: &[NurbsCurve],
) -> Result<(Vec<Geometry>, usize), String> {
    if tools.is_empty() || tools.len() > 64 {
        return Err("Choose 1–64 cutter curves".into());
    }
    let face = face_index(source, face)?;
    let tolerance = 1e-6;
    let (pieces, regions) = match source {
        Geometry::Line(line) => {
            let pieces: Vec<_> = split::split_line_by_curves(line, &tools, tolerance)?
                .into_iter()
                .map(|p| Geometry::Line(Rc::new(p)))
                .collect();
            let count = pieces.len();
            (pieces, count)
        }
        Geometry::Polyline(line) => {
            let pieces: Vec<_> = split::split_polyline_by_curves(line, &tools, tolerance)?
                .into_iter()
                .map(|p| Geometry::Polyline(Rc::new(p)))
                .collect();
            let count = pieces.len();
            (pieces, count)
        }
        Geometry::NurbsCurve(curve) => {
            let pieces: Vec<_> = split::split_curve_by_curves(curve, &tools, tolerance)?
                .into_iter()
                .map(|p| Geometry::NurbsCurve(Rc::new(p)))
                .collect();
            let count = pieces.len();
            (pieces, count)
        }
        Geometry::NurbsSurface(surface) => {
            let mut brep = split::split_surface_by_curves(surface, &tools, tolerance)?;
            brep.name = surface.name.clone();
            let count = brep.face_count();
            (vec![Geometry::BRep(Rc::new(brep))], count)
        }
        Geometry::BRep(brep) => {
            let next = split::split_brep_face_by_curves(
                brep,
                face.ok_or("Select a face")?,
                &tools,
                tolerance,
            )?;
            let count = next.face_count() - brep.face_count() + 1;
            (vec![Geometry::BRep(Rc::new(next))], count)
        }
        Geometry::Element(element) => {
            let session_rust::element::ElementGeometry::BRep(brep) = element.geometry() else {
                return Err("Element has no BRep".into());
            };
            let next = split::split_brep_face_by_curves(
                brep,
                face.ok_or("Select a face")?,
                &tools,
                tolerance,
            )?;
            let count = next.face_count() - brep.face_count() + 1;
            let mut result = (**element).clone();
            result.set_brep_geometry(next);
            (vec![Geometry::Element(Rc::new(result))], count)
        }
        _ => return Err("Unsupported split target".into()),
    };

    Ok((pieces, regions))
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/82-divide-current-original-curve-families-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.2.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

The extended original line and projected curve remain visible while original splitting connects.

### Recorded visual result

**Previous native render — current verification pending.**

The extended line and projected original curve remain visible.

![Native renderer output for divide original curves and joined faces.](../screenshots/journey/81v-exercise-original-projection-retry-and-repeated-extension.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.1.30 · Exercise original projection retry and repeated extension](81v-exercise-original-projection-retry-and-repeated-extension.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original family → pure calculation → result or refusal.



Do the pieces come from sampled display segments?

No. Split uses original line, polyline or rational curve geometry. Display samples render the results.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare the line, polyline and rational-curve branches. Explain why a split surface returns one BRep with several regions.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.2.1
npm --prefix ../session_tests run course -- save 16.2.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The preceding original-edit contract remains authored and unrun. The final split contract adds actual camera-projected target/cutter clicks, empty confirmation, completion, Escape and one Undo/Redo. Actual browser execution remains required.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 16.2.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
