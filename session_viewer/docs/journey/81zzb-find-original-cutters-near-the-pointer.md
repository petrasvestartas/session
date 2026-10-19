# 16.1.3 · Find original cutters near the pointer

**Chapter 16 · Geometry editing and measurement · Section 16.1 · Projection and Split inputs** · [Contents](contents.md)

**Typing: 25–49 minutes.** [Estimate](typing-load.md).

Build fence planes and unit rays in the original object frame. Sample curve spans and find the nearest projected polyline within the pixel aperture.

## Type

Continue from [Read original cutters in object frames](81zza-read-original-cutters-in-object-frames.md). [Save or recover your work](recovery.md).

### 1. `src/source_cut.rs`

Extend this original owner with today’s complete helper.

<details>
<summary>Locate the existing block</summary>

```rust
/// `plane` moved by `xform`, rebuilt from three moved points so a stretch keeps it true.
pub fn plane_moved(plane: &Plane, xform: &Xform) -> Option<Plane> {
    let origin = plane.origin();
    let a = origin.transformed(xform);
    let b = (&origin + &plane.x_axis()).transformed(xform);
    let c = (&origin + &plane.y_axis()).transformed(xform);
    let normal = (&b - &a).cross(&(&c - &a));
    (normal.magnitude() > 1e-14).then(|| Plane::from_point_normal(a, normal, None))
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/81zzb-find-original-cutters-near-the-pointer-1.rs"
```

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/source_cut.rs`

Supply only the unchanged original validation fixtures.

<details>
<summary>Locate the existing block</summary>

```text
use session_rust::{Geometry, Line, NurbsCurve, NurbsSurface, Plane, Point, Vector, Xform};

/// A cutting or bounding object, in the frame of the object it acts on.
#[derive(Clone, Debug)]
pub enum Cutter {
    Curve(NurbsCurve), // cuts where it crosses
    Plane(Plane),      // cuts where the object passes through it
}

impl Cutter {
    /// The cutter moved by `xform`; a plane is rebuilt from three moved points.
    pub fn moved(&self, xform: &Xform) -> Option<Cutter> {
        match self {
            Cutter::Curve(curve) => Some(Cutter::Curve(curve.transformed(xform))),
            Cutter::Plane(plane) => plane_moved(plane, xform).map(Cutter::Plane),
        }
    }
}

/// A line, polyline or NURBS curve as a NURBS curve.
pub fn as_curve(geometry: &Geometry) -> Option<NurbsCurve> {
    match geometry {
        Geometry::Line(line) => Some(NurbsCurve::create(false, 1, &[line.start(), line.end()])),
        Geometry::Polyline(polyline) if polyline.point_count() >= 2 => {
            Some(NurbsCurve::create(false, 1, &polyline.get_points()))
        }
        Geometry::NurbsCurve(curve) => Some((**curve).clone()),
        _ => None,
    }
}

/// A Plane object, a planar surface or a one-face planar BRep as a plane.
pub fn as_plane(geometry: &Geometry) -> Option<Plane> {
    match geometry {
        Geometry::Plane(plane) => Some((**plane).clone()),
        Geometry::NurbsSurface(surface) => planar(surface),
        Geometry::BRep(brep) if brep.face_count() == 1 => planar(
            brep.m_surfaces
                .get(brep.m_faces[0].surface_index as usize)?,
        ),
        _ => None,
    }
}

/// The plane of a flat surface, within a millionth of its size.
pub fn planar(surface: &NurbsSurface) -> Option<Plane> {
    let mut size: f64 = 1.0;

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            let p = surface.get_cv(i, j)?;
            size = size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs());
        }
    }

    let mut plane = Plane::invalid();
    (surface.is_planar(Some(&mut plane), 1e-6 * size) && plane.is_valid()).then_some(plane)
}

/// The plane through `line` that holds `normal`: a vertical cut when `normal` is the view's up.
pub fn fence_plane(line: &Line, normal: &Vector) -> Result<Plane, String> {
    let across = line.to_vector().cross(normal);

    if across.magnitude() <= 1e-12 * line.length().max(1.0) {
        return Err("The cutting line runs along the view direction".into());
    }

    Ok(Plane::from_point_normal(line.start(), across, None))
}

/// `plane` moved by `xform`, rebuilt from three moved points so a stretch keeps it true.
pub fn plane_moved(plane: &Plane, xform: &Xform) -> Option<Plane> {
    let origin = plane.origin();
    let a = origin.transformed(xform);
    let b = (&origin + &plane.x_axis()).transformed(xform);
    let c = (&origin + &plane.y_axis()).transformed(xform);
    let normal = (&b - &a).cross(&(&c - &a));
    (normal.magnitude() > 1e-14).then(|| Plane::from_point_normal(a, normal, None))
}

/// A ray moved by `xform` with a unit direction, so a scaled object's triangles are not taken for parallel.
pub fn unit_ray(origin: &Point, direction: &Vector, xform: &Xform) -> Line {
    let start = origin.transformed(xform);
    let along = (&(origin + direction).transformed(xform) - &start).normalized();
    Line::from_points(&start, &(&start + &along))
}

/// Points along a curve: its corners when straight, else `count` even steps.
pub fn samples(curve: &NurbsCurve, count: usize) -> Vec<Point> {
    if curve.degree() == 1 {
        return curve
            .get_span_vector()
            .iter()
            .map(|t| curve.point_at(*t))
            .collect();
    }

    let (t0, t1) = curve.domain();
    (0..=count)
        .map(|i| curve.point_at(t0 + (t1 - t0) * i as f64 / count as f64))
        .collect()
}

/// Pixel distance from `at` to the segment a-b.
pub fn segment_px(a: (f64, f64), b: (f64, f64), at: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length = dx * dx + dy * dy;
    let t = if length > 0.0 {
        (((at.0 - a.0) * dx + (at.1 - a.1) * dy) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (a.0 + t * dx - at.0).hypot(a.1 + t * dy - at.1)
}

/// The polyline nearest `at` on screen within `aperture` pixels, and its distance.
pub fn nearest_px(polylines: &[Vec<(f64, f64)>], at: (f64, f64), aperture: f64) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;

    for (index, polyline) in polylines.iter().enumerate() {
        let distance = match polyline.as_slice() {
            [single] => segment_px(*single, *single, at),
            _ => polyline
                .windows(2)
                .map(|pair| segment_px(pair[0], pair[1], at))
                .fold(f64::INFINITY, f64::min),
        };

        if distance <= aperture && best.is_none_or(|(_, near)| distance < near) {
            best = Some((index, distance));
        }
    }

    best.map(|(index, _)| index)
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/81zzb-find-original-cutters-near-the-pointer-2.rs" skip=5024
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 16.1.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Pointer queries can identify candidate original cutters without modifying the source.

**Visual review: issue found.** This inherited network reference has faint or interrupted original input curves. Its aggregate pixel checks do not establish curve continuity or contrast. These are input curves, not a full NURBS isocurve display; visual-quality acceptance is incomplete.

[Visual review findings](visual-review.md).

### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for find original cutters near the pointer.](../screenshots/journey/80za-read-original-projection-planes.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [15.4.6 · Read original projection planes](80za-read-original-projection-planes.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original cutter → object frame → physical reach → edited curve.



Why normalize a ray after changing its frame?

Object scaling changes its length. Intersection tolerances require a unit direction in the mesh frame.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Compare a scaled ray and two nearby projected polylines.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 16.1.3
npm --prefix ../session_tests run course -- save 16.1.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

These original cutter and reach helpers prepare cancellable editing of the original curve.



[Full validation scope](release.md).

</details>
