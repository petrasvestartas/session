# 16.4.33 · Connect window and bounds selection tests

**Chapter 16 · Geometry editing and measurement · Section 16.4 · Solid volume, extension and Explode** · [Contents](contents.md)

**Typing: 26–52 minutes.** [Estimate](typing-load.md).

Add projection, conservative screen bounds and complete-enclosure dispatch. Supply the original validation fixtures separately; the existing selection command connects this owner in the next lesson.

## Type

Continue from [Test crossing selection on original strokes](84zne-test-crossing-selection-on-original-strokes.md). [Save or recover your work](recovery.md).

### 1. `src/selection_region.rs`

Extend the existing original selection owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
```

</details>

Replace it with:

```rust
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
}

/// Whether every sample of `geometry`, in its own frame, passes `inside`; None when only its box can tell.
pub(crate) fn enclosed(geometry: &Geometry, inside: &dyn Fn([f64; 3]) -> bool) -> Option<bool> {
    match geometry {
        Geometry::Point(point) => every(std::iter::once(xyz(point)), inside),
        Geometry::Line(line) => every([xyz(&line.start()), xyz(&line.end())].into_iter(), inside),
        Geometry::Polyline(polyline) => every(
            polyline.coords.chunks_exact(3).map(|c| [c[0], c[1], c[2]]),
            inside,
        ),
        Geometry::NurbsCurve(curve) => every(curve_samples(curve, CURVE_STEPS), inside),
        Geometry::NurbsSurface(surface) => every(surface_samples(surface, SURFACE_STEPS), inside),
        Geometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
        Geometry::BRep(brep) => every(brep_samples(brep), inside),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
            ElementGeometry::BRep(brep) => every(brep_samples(brep), inside),
            ElementGeometry::None => None,
        },
        Geometry::PointCloud(cloud) => {
            let coords = cloud.coords();
            let step = (coords.len() / 3).div_ceil(CLOUD_SAMPLES).max(1);
            every(
                coords
                    .chunks_exact(3)
                    .step_by(step)
                    .map(|c| [c[0], c[1], c[2]]),
                inside,
            )
        }
        Geometry::Plane(_) | Geometry::OBB(_) => None,
    }
```

<details>
<summary>Locate this block</summary>

```rust
/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
```

</details>

Replace it with:

```rust
/// Device pixels of `p` under the clip matrix `m`; None behind the eye.
fn screen(m: &[f64; 16], p: [f64; 3], size: (f64, f64)) -> Option<(f64, f64)> {
    let clip = |r: usize| m[r] * p[0] + m[r + 4] * p[1] + m[r + 8] * p[2] + m[r + 12];
    let w = clip(3);
    (w > 0.0).then(|| {
        (
            (clip(0) / w * 0.5 + 0.5) * size.0,
            (0.5 - clip(1) / w * 0.5) * size.1,
        )
    })
}

/// Whether the screen box around projected corners overlaps the region's rectangle.
fn box_touches(corners: &[Option<(f64, f64)>], region: &Region) -> bool {
    let (Some([left, top, right, bottom]), Some(first)) =
        (region.box_, corners.iter().flatten().next())
    else {
        return false;
    };
    let mut lo = *first;
    let mut hi = *first;

    for at in corners.iter().flatten() {
        lo = (lo.0.min(at.0), lo.1.min(at.1));
        hi = (hi.0.max(at.0), hi.1.max(at.1));
    }

    lo.0 <= right && hi.0 >= left && lo.1 <= bottom && hi.1 >= top
}

/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
```

<details>
<summary>Complete updated block reference</summary>

```rust
--8<-- "journey/code/84znf-connect-window-and-bounds-selection-tests-1.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/selection_region.rs`

Supply the original region-selection validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
/// Vertices, edges and curved faces; a flat face is bounded by its edges.
fn brep_samples(brep: &BRep) -> impl Iterator<Item = [f64; 3]> + '_ {
    let vertices = brep.m_vertices.iter().map(|vertex| xyz(&vertex.point));
    let edges = brep
        .m_curves_3d
        .iter()
        .flat_map(|curve| curve_samples(curve, EDGE_STEPS));
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/84znf-connect-window-and-bounds-selection-tests-2.rs" skip=559
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.4.33
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

![Native renderer output for connect window and bounds selection tests.](../screenshots/journey/84zza-describe-removable-cad-parts.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [16.4.15 · Describe removable CAD parts](84zza-describe-removable-cad-parts.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original CAD samples → screen region → window or crossing result.



Why keep the bounds fallback separate from sampled geometry?

Planes and empty original families cannot establish membership from samples, but their projected bounds can still guide selection.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Trace a complete window test, a crossing test and an original with no samples through their separate results.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.4.33
npm --prefix ../session_tests run course -- save 16.4.33
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original selection helpers connect in 85. The existing browser scene is unchanged while they are prepared.



[Full validation scope](release.md).

</details>
