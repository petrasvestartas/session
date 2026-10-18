# 15.1.47 · Extrude original corners and curves

**Chapter 15 · Transforms and surface construction · Section 15.1 · Transforms, extrusions and revolutions** · [Contents](contents.md)

**Typing: 22–43 minutes.** [Estimate](typing-load.md).

Dispatch original corner chains and curved sections through the shared extrusion helpers. Build polygon sides from their original points, add eligible caps and validate the resulting BRep. Curved inputs use the kernel surface or the closed topology builder.

## Type

Continue from [Build closed curved extrusion topology](77zzl-build-closed-curved-extrusion-topology.md). [Save or recover your work](recovery.md).

### 1. `src/extrude_geometry.rs`

Extend this original owner with the day’s whole extrusion helpers.

<details>
<summary>Locate the existing block</summary>

```rust
    let along = normal.dot(offset);

    if along.abs() <= 1e-9 * offset.magnitude() {
        return Err(Skip::Flat);
    }

    Ok(if along < 0.0 { -normal } else { normal })
}
```

</details>

After that block, type:

```rust
/// One picked curve extruded by `offset`, capped when asked and possible.
fn extrude(picked: &Picked, offset: &Vector, cap: bool) -> Result<(Geometry, Skip), String> {
    let named = |mut brep: BRep| {
        brep.name = "extrusion".into();
        Geometry::BRep(Rc::new(brep))
    };

    match &picked.corners {
        Some(corners) if corners.len() > 2 => {
            let closed = corners.len() >= 4
                && corners[0].distance(&corners[corners.len() - 1], None) <= 1e-12;
            let lift = |p: &Point| p + offset;
            let mut faces: Vec<Polyline> = corners
                .windows(2)
                .map(|pair| {
                    Polyline::new(vec![
                        pair[0].clone(),
                        pair[1].clone(),
                        lift(&pair[1]),
                        lift(&pair[0]),
                        pair[0].clone(),
                    ])
                })
                .collect();
            let skip = match cap_normal(&corners[..corners.len() - 1], closed, offset) {
                Ok(_) if cap => {
                    faces.push(Polyline::new(corners.clone()));
                    faces.push(Polyline::new(corners.iter().map(lift).collect()));
                    Skip::None
                }
                Ok(_) => Skip::None,
                Err(skip) => skip,
            };
            let brep = BRep::from_polylines(&faces, &[]);

            if brep.face_count() == 0 {
                return Err("The polyline could not be extruded".into());
            }

            Ok((named(brep), skip))
        }
        _ => {
            let outline = picked.outline();
            let normal = cap_normal(&outline, picked.curve.is_closed(), offset);

            if let (true, Ok(normal)) = (cap && picked.corners.is_none(), &normal) {
                return Ok((named(capped(&picked.curve, offset, normal)?), Skip::None));
            }

            let surface = Primitives::create_extrusion(&picked.curve, offset);
            let skip = normal.err().unwrap_or(Skip::None);
            Ok((checked(surface, "extrusion")?, skip))
        }
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/77zzm-extrude-original-corners-and-curves-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 15.1.47
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

![Native renderer output for extrude original corners and curves.](../screenshots/journey/77zza-describe-curve-gathering-answers.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.1.35 · Describe curve gathering answers](77zza-describe-curve-gathering-answers.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original curves → offset and cap rules → shared BRep topology.



Why preserve the original curve when building a curved extrusion?

Sampling it into a polygon would change the source geometry and its later editable control points.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a polygon outline with a curved outline and trace Cap On and Cap Off through each branch.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 15.1.47
npm --prefix ../session_tests run course -- save 15.1.47
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original extrusion helpers are preparatory. The existing command registry connects the completed recipe in 78; earlier helper checkpoints keep the browser drawing unchanged.



[Full validation scope](release.md).

</details>
