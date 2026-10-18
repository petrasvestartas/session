# 16.4.19 · Group faces by cutting planes

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 20–39 minutes.** [Estimate](typing-load.md).

Extend a plane-plane intersection past surface control points. Group faces by plane-side signs while retaining independently cut face regions.

## Type

Continue from [Preserve trimmed source types](84zzd-preserve-trimmed-source-types.md). [Save or recover your work](recovery.md).

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
/// Faces grouped by the side of every plane they lie on; a curve-cut region stays its own part.
fn sides(
    brep: &BRep,
    meshes: &[Mesh],
    planes: &[Plane],
    regions: &[Vec<usize>],
) -> Vec<Vec<usize>> {
    let mut groups: Vec<(Vec<bool>, Option<usize>, Vec<usize>)> = Vec::new();

    for face in 0..brep.face_count() {
        let center = meshes
            .get(face)
            .map_or_else(|| brep.point_at(face, 0.5, 0.5), |mesh| mesh.centroid());
        let key: Vec<bool> = planes
            .iter()
            .map(|plane| (&center - &plane.origin()).dot(&plane.z_axis()) >= 0.0)
            .collect();
        let region = regions
            .iter()
            .any(|faces| faces.contains(&face))
            .then_some(face);

        match groups
            .iter_mut()
            .find(|(signs, own, _)| *signs == key && *own == region && region.is_none())
        {
            Some((_, _, faces)) => faces.push(face),
            None => groups.push((key, region, vec![face])),
        }
    }

    groups.into_iter().map(|(_, _, faces)| faces).collect()
}

/// A plane-plane line as a segment reaching past every control point of `surface`.
fn across(line: &Line, surface: &NurbsSurface) -> NurbsCurve {
    let start = line.start();
    let mut direction = line.to_vector();
    direction.normalize_self();
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                let t = (&p - &start).dot(&direction);
                low = low.min(t);
                high = high.max(t);
            }
        }
    }

    let margin = (high - low).max(1.0) * 0.1;
    NurbsCurve::create(
        false,
        1,
        &[
            &start + &(&direction * (low - margin)),
            &start + &(&direction * (high + margin)),
        ],
    )
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/84zze-group-faces-by-cutting-planes-1.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.19
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

![Native renderer output for group faces by cutting planes.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD geometry → removable parts → retained source geometry.



Why keep curve-cut regions separate?

Faces split by a curve are individually removable even when they share the same plane-side signs.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare two unsplit faces on one side with two faces split by a curve.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.19
npm --prefix ../session_tests run course -- save 16.4.19
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>
