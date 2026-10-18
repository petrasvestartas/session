# 16.4.21 · Assemble removable face groups

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 29–58 minutes.** [Estimate](typing-load.md).

Combine curve regions, fallback fence planes and crossed plane sections into one Parts value. Keep the source BRep unchanged by working on its clone.

## Type

Continue from [Cut planar BRep faces](84zzf-cut-planar-brep-faces.md). [Save or recover your work](recovery.md).

### 1. `src/trim_parts.rs`

Add this part-building behavior to the existing owner.

<details>
<summary>Locate the existing block</summary>

```rust
            if let Some(p) = surface.get_cv(i, j) {
                size = size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs());
            }
        }
    }

    size
}
```

</details>

After that block, type:

```rust
/// Split the faces of `brep` by the blades: curves on its faces make regions, planes make sides.
fn faces(brep: &BRep, blades: &[Blade]) -> Result<Parts, String> {
    let mut brep = brep.clone();
    let mut regions: Vec<Vec<usize>> = Vec::new(); // faces a curve split, one region each
    let mut planes: Vec<Plane> = Vec::new();
    let mut section: Vec<Vec<Point>> = Vec::new();
    let mut refused = None;

    for blade in blades {
        let curve = match &blade.cutter {
            Cutter::Plane(plane) => {
                planes.push(plane.clone());
                continue;
            }
            Cutter::Curve(curve) => curve,
        };
        let mut hit = false;

        for face in 0..brep.face_count() {
            let before = brep.face_count();

            match split::split_brep_face_by_curves(
                &brep,
                face,
                std::slice::from_ref(curve),
                TOLERANCE,
            ) {
                Ok(next) if next.face_count() > before => {
                    regions.push(
                        [face]
                            .into_iter()
                            .chain(before..next.face_count())
                            .collect(),
                    );
                    brep = next;
                    hit = true;
                }
                Ok(_) => {}
                Err(error) => refused = Some(error),
            }
        }

        match (&blade.fence, hit) {
            (_, true) => section.push(samples(curve, 48)),
            (Some(fence), false) => planes.push(fence.clone()),
            (None, false) => {}
        }
    }

    // a plane that misses this object is some other target's
    let mut crossing = Vec::new();

    for plane in planes {
        match cut_by_plane(&mut brep, &plane) {
            Ok(split) => {
                section.extend(split);
                crossing.push(plane);
            }
            Err(error) if error == "not crossed" => {}
            Err(error) => return Err(error),
        }
    }

    let planes = crossing;

    if regions.is_empty() && section.is_empty() {
        return Err(refused.unwrap_or_else(|| {
            "Curves cut a BRep only where they lie on its faces; use a line, a Plane or a planar surface to cut through it".into()
        }));
    }

    let meshes = brep.face_meshes_q(Some(crate::cad_boundary::QUALITY));
    let groups = if planes.is_empty() {
        // each region of a split face is a part; unsplit faces are no part
        regions
            .into_iter()
            .flatten()
            .map(|face| vec![face])
            .collect()
    } else {
        sides(&brep, &meshes, &planes, &regions)
    };

    if groups.len() < 2 {
        return Err("not crossed".into());
    }

    Ok(Parts::Faces {
        brep,
        groups,
        meshes,
        section,
    })
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/84zzg-assemble-removable-face-groups-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.21
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

![Native renderer output for assemble removable face groups.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD geometry → removable parts → retained source geometry.



Why ignore a plane that misses one target?

The same cutter list can cross another selected target.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace one curve that splits a face and one fence plane that crosses another target.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.21
npm --prefix ../session_tests run course -- save 16.4.21
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
