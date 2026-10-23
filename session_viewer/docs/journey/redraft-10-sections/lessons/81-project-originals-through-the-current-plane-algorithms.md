# 16.1 · Projection, extension and stable edit targets

**Goal:** Accept original cutter curves and a valid selected BRep face. · **Steps:** 31 · **Source:** [app/command/verbs/project_to_plane.rs](../../../../src/app/command/verbs/project_to_plane.rs) · [app/command/verbs/extend.rs](../../../../src/app/command/verbs/extend.rs) · [app/splitting.rs](../../../../src/app/splitting.rs) · [app/command/verbs/measure.rs](../../../../src/app/command/verbs/measure.rs) · [app/command/verbs/trim/parts.rs](../../../../src/app/command/verbs/trim/parts.rs) · [app/command/verbs/explode.rs](../../../../src/app/command/verbs/explode.rs) · [app/walk/frames.rs](../../../../src/app/walk/frames.rs) (`src/app/booleans.rs,src/app/measurements.rs,src/app/recipes.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Intersecting Solids A &amp; B + Cut Curve C = ∂A ∩ ∂B</text>
  <rect x="68" y="64" width="96" height="76" fill="#dbeafe" fill-opacity="0.8" stroke="#2563eb" stroke-width="2"/>
  <rect x="128" y="52" width="96" height="76" fill="#fee2e2" fill-opacity="0.8" stroke="#dc2626" stroke-width="2"/>
  <rect x="128" y="64" width="36" height="64" fill="#fef08a" stroke="#111111" stroke-width="2.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="92" y="106" font-family="monospace" font-size="11" font-weight="700" fill="#1e3a8a">A</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="194" y="94" font-family="monospace" font-size="11" font-weight="700" fill="#991b1b">B</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Boolean (A ∪ B, A \ B, A ∩ B) + Area ∫||Su×Sv||, Vol ⅓∫p·N</text>
  <polygon points="424,64 520,64 520,128 484,128 484,140 424,140" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="462" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">A \ B</text>
  <rect x="548" y="72" width="44" height="56" fill="#fef08a" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="570" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">A ∩ B</text>
  <polygon points="618,128 656,68 694,128" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <circle cx="656" cy="108" r="4" fill="#dc2626"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Trimmed Boolean Solid + Centroid &amp; Area Witness</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,66 956,66 956,118 912,118 912,144 846,144" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <circle cx="892" cy="102" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="998" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">V, A, C_g</text>
</svg>
</div>

## <a id="checkpoint-81-project-originals-through-the-current-plane-algorithms"></a>1. Project originals in their placed frame

**What this code does:** Conjugate the world projection through the original placement transform.

### 1. `src/project_geometry.rs`

Extend this original owner with today’s complete helper. Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
    ]))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ]))
}

/// Why an object is left as it is.
#[derive(Debug, PartialEq)]
pub enum Skip {
    Collapsed,   // every point would land on one
    Unsupported, // not a point, line, polyline, curve or mesh
}

/// The object's geometry projected onto a world plane, in its own frame under `place`.
pub fn projected(
    geometry: &Geometry,
    place: &Xform,
    plane: &Plane,
) -> Result<Geometry, Skip> {
    let back = place.inverse().ok_or(Skip::Unsupported)?;
    let local = &(&back * &Xform::project_to_plane(plane)) * place; // the projection in the object's frame
    let (before, after, geometry) = match geometry {
        Geometry::Point(point) => {
            return Ok(Geometry::Point(Rc::new(
                point.transformed(&local),
            )));
        }
        Geometry::Line(line) => {
            let out = line.transformed(&local);
            (
                vec![line.start(), line.end()],
                vec![out.start(), out.end()],
                Geometry::Line(Rc::new(out)),
            )
        }
        Geometry::Polyline(polyline) => {
            let out = polyline.transformed(&local);
            (
                polyline.get_points(),
                out.get_points(),
                Geometry::Polyline(Rc::new(out)),
            )
        }
        Geometry::NurbsCurve(curve) => {
            let out = curve.transformed(&local);
            let cvs = |curve: &session_rust::NurbsCurve| {
                (0..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect()
            };
            (
                cvs(curve),
                cvs(&out),
                Geometry::NurbsCurve(Rc::new(out)),
            )
        }
        Geometry::Mesh(mesh) => {
            let mut out = mesh.transformed(&local);

            // the old normals no longer fit the flat faces
            for vertex in out.vertex.values_mut() {
                for key in ["nx", "ny", "nz"] {
                    vertex.attributes.remove(key);
                }
            }

            let points = |mesh: &session_rust::Mesh| {
                mesh.vertex
                    .values()
                    .map(|vertex| vertex.position())
                    .collect()
            };
            (
                points(mesh),
                points(&out),
                Geometry::Mesh(Rc::new(out)),
            )
        }
        _ => return Err(Skip::Unsupported),
    };

    if extent(&after) <= 1e-9 * extent(&before) {
        return Err(Skip::Collapsed);
    }

    Ok(geometry)
}

/// The largest distance from the first point to any other.
fn extent(points: &[Point]) -> f64 {
    let Some(first) = points.first() else {
        return 0.0;
    };
    points
        .iter()
        .map(|point| first.distance(point, None))
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Line, Mesh, NurbsCurve, Polyline};

    /// A point from coordinates.
    fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// The typed forms parse; wrong counts and words are refused.
    #[test]
    fn the_planes_and_points_parse() {
        let parsed = |rest: &[&str]| {
            parse("Project To Plane", rest).map(|action| format!("{action:?}"))
        };
        assert!(parsed(&["xz"]).unwrap().contains("\"ZX\""));
        assert!(parsed(&[]).unwrap().contains("\"CPlane\""));
        assert!(
            parsed(&["3points", "0,0,0", "1,0,0", "0,1,0"])
                .unwrap()
                .contains("\"3Point\"")
        );
        assert!(parsed(&["3Point"]).is_ok());

        for rest in [
            &["3Point", "0,0,0", "1,0,0"][..],
            &["3Point", "0,0,0", "1,0,0", "0,1,0", "1,1,1"],
            &["sideways"],
            &["XY", "extra"],
        ] {
            assert!(parsed(rest).is_err(), "{rest:?}");
        }
    }

    /// Point, line, polyline, rational curve and mesh land on XY; x and y stay, normals go.
    #[test]
    fn every_supported_kind_lands_on_the_plane() {
        let plane = Plane::xy_plane();
        let place = Xform::identity();
        let flat = |point: &Point, x: f64, y: f64| {
            point[2].abs() < 1e-12
                && (point[0] - x).abs() < 1e-12
                && (point[1] - y).abs() < 1e-12
        };

        let Ok(Geometry::Point(point)) = projected(
            &Geometry::Point(Rc::new(p(1.0, 2.0, 3.0))),
            &place,
            &plane,
        ) else {
            panic!("a point");
        };
        assert!(flat(&point, 1.0, 2.0));

        let line = Line::from_points(&p(0.0, 0.0, 1.0), &p(5.0, 5.0, 8.0));
        let Ok(Geometry::Line(line)) = projected(
            &Geometry::Line(Rc::new(line)),
            &place,
            &plane,
        ) else {
            panic!("a line");
        };
        assert!(flat(&line.end(), 5.0, 5.0));

        let polyline = Polyline::new(vec![
            p(0.0, 0.0, 0.0),
            p(100.0, 0.0, 50.0),
            p(100.0, 100.0, 100.0),
        ]);
        let Ok(Geometry::Polyline(polyline)) = projected(
            &Geometry::Polyline(Rc::new(polyline)),
            &place,
            &plane,
        ) else {
            panic!("a polyline");
        };
        assert!(flat(&polyline.get_points()[2], 100.0, 100.0));

        let mut curve = NurbsCurve::create(
            false,
            2,
            &[
                p(0.0, 0.0, 0.0),
                p(50.0, 100.0, 40.0),
                p(100.0, 0.0, 80.0),
            ],
        );
        assert!(curve.set_weight(1, 0.5));
        let Ok(Geometry::NurbsCurve(out)) = projected(
            &Geometry::NurbsCurve(Rc::new(curve.clone())),
            &place,
            &plane,
        ) else {
            panic!("a curve");
        };
        assert!(flat(
            &out.get_cv(1).unwrap(),
            curve.get_cv(1).unwrap()[0],
            curve.get_cv(1).unwrap()[1]
        ));
        assert!(
            (out.weight(1) - curve.weight(1)).abs() < 1e-12,
            "weights stay"
        );

        let mut mesh = Mesh::new();
        let key = mesh.add_vertex(p(0.0, 0.0, 5.0), None);
        mesh.add_vertex(p(10.0, 0.0, 6.0), None);
        mesh.add_vertex(p(0.0, 10.0, 7.0), None);
        mesh.vertex
            .get_mut(&key)
            .unwrap()
            .attributes
            .insert("nx".into(), 1.0);
        let Ok(Geometry::Mesh(mesh)) = projected(
            &Geometry::Mesh(Rc::new(mesh)),
            &place,
            &plane,
        ) else {
            panic!("a mesh");
        };
        assert!(
            mesh.vertex
                .values()
                .all(|vertex| vertex.position()[2].abs() < 1e-12)
        );
        assert!(
            mesh.vertex
                .values()
                .all(|vertex| vertex.attributes.get("nx").is_none())
        );
    }

    /// Under a rotated, scaled and moved placement the world result lies on the world plane.
    #[test]
    fn a_placed_object_lands_on_the_world_plane() {
        let place = &(&Xform::translation(100.0, 0.0, 50.0)
            * &Xform::rotation_x(90.0, true))
            * &Xform::scale_xyz(10.0, 10.0, 10.0);
        let polyline = Polyline::new(vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 2.0, 3.0),
            p(-4.0, 5.0, 6.0),
        ]);
        let Ok(Geometry::Polyline(out)) = projected(
            &Geometry::Polyline(Rc::new(polyline.clone())),
            &place,
            &Plane::xy_plane(),
        ) else {
            panic!("a polyline");
        };

        for (before, after) in polyline
            .get_points()
            .iter()
            .zip(out.get_points())
        {
            let (world, moved) = (
                before.transformed(&place),
                after.transformed(&place),
            );
            assert!(moved[2].abs() < 1e-9);
            assert!(
                (moved[0] - world[0]).abs() < 1e-9
                    && (moved[1] - world[1]).abs() < 1e-9
            );
        }
    }

    /// A vertical line collapses on XY; a box is not projectable.
    #[test]
    fn collapsing_and_unsupported_objects_are_skipped() {
        let line = Line::from_points(&p(5.0, 5.0, 0.0), &p(5.0, 5.0, 100.0));
        assert_eq!(
            projected(
                &Geometry::Line(Rc::new(line)),
                &Xform::identity(),
                &Plane::xy_plane()
            )
            .err(),
            Some(Skip::Collapsed)
        );
        let plane = Geometry::Plane(Rc::new(Plane::xy_plane()));
        assert_eq!(
            projected(
                &plane,
                &Xform::identity(),
                &Plane::xy_plane()
            )
            .err(),
            Some(Skip::Unsupported)
        );
    }

    /// Points in a line make no plane; a tilted plane has the expected normal.
    #[test]
    fn three_points_in_a_line_make_no_plane() {
        assert!(
            plane_of(
                &p(0.0, 0.0, 0.0),
                &p(10.0, 0.0, 0.0),
                &p(20.0, 0.0, 0.0)
            )
            .is_err()
        );
        assert!(
            plane_of(
                &p(0.0, 0.0, 0.0),
                &p(0.0, 0.0, 0.0),
                &p(0.0, 1.0, 0.0)
            )
            .is_err()
        );
        let normal = plane_of(
            &p(0.0, 0.0, 0.0),
            &p(100.0, 0.0, 0.0),
            &p(0.0, 100.0, 100.0),
        )
        .unwrap()
        .z_axis();
        let expected = Vector::new(0.0, -1.0, 1.0).normalized();
        assert!(normal.cross(&expected).magnitude() < 1e-9);
    }
}
```

## <a id="checkpoint-81zza-read-original-cutters-in-object-frames"></a>2. Read cutters in object frames

**What this code does:** Write the cutter records and convert supported originals to curves or planar boundaries.

### 1. `src/source_cut.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use session_rust::{
    Geometry, Line, NurbsCurve, NurbsSurface, Plane, Point, Vector, Xform,
};

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
        Geometry::Line(line) => Some(NurbsCurve::create(
            false,
            1,
            &[line.start(), line.end()],
        )),
        Geometry::Polyline(polyline) if polyline.point_count() >= 2 => Some(
            NurbsCurve::create(false, 1, &polyline.get_points()),
        ),
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
            size = size
                .max(p[0].abs())
                .max(p[1].abs())
                .max(p[2].abs());
        }
    }

    let mut plane = Plane::invalid();
    (surface.is_planar(Some(&mut plane), 1e-6 * size) && plane.is_valid())
        .then_some(plane)
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
```

### 2. `src/lib.rs`

Register this original curve-editing owner.

**Locate this existing block:**

```rust
pub mod project_geometry;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod project_geometry;

pub mod source_cut;
pub mod source_mesh_topology;
```

## <a id="checkpoint-81zzb-find-original-cutters-near-the-pointer"></a>3. Find cutters near the pointer

**What this code does:** Build fence planes and unit rays in the original object frame.

### 1. `src/source_cut.rs`

Extend this original owner with today’s complete helper. Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// The plane through `line` that holds `normal`: a vertical cut when `normal` is the view's up.
pub fn fence_plane(line: &Line, normal: &Vector) -> Result<Plane, String> {
    let across = line.to_vector().cross(normal);

    if across.magnitude() <= 1e-12 * line.length().max(1.0) {
        return Err("The cutting line runs along the view direction".into());
    }

    Ok(Plane::from_point_normal(
        line.start(),
        across,
        None,
    ))
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
pub fn nearest_px(
    polylines: &[Vec<(f64, f64)>],
    at: (f64, f64),
    aperture: f64,
) -> Option<usize> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::BRep;
    use std::rc::Rc;

    /// A square patch in the plane z = `height`, bent up when `bend`.
    pub(crate) fn patch(height: f64, bend: f64) -> NurbsSurface {
        let mut surface = NurbsSurface::new(3, false, 2, 2, 3, 3);

        for i in 0..3 {
            for j in 0..3 {
                let lift = if i == 1 && j == 1 {
                    bend
                } else {
                    0.0
                };
                surface.set_cv(
                    i,
                    j,
                    &Point::new(i as f64 * 5.0, j as f64 * 5.0, height + lift),
                );
            }
        }

        surface
    }

    /// Planes come from Plane objects, flat surfaces and one-face flat BReps only.
    #[test]
    fn planes_come_from_flat_objects() {
        let plane = Plane::from_point_normal(
            Point::new(0., 0., 2.),
            Vector::new(0., 0., 1.),
            None,
        );
        assert!(as_plane(&Geometry::Plane(Rc::new(plane))).is_some());
        let flat = as_plane(&Geometry::NurbsSurface(Rc::new(patch(
            3.0, 0.0,
        ))))
        .unwrap();
        assert!((flat.origin()[2] - 3.0).abs() < 1e-9);
        assert!(flat.z_axis()[2].abs() > 0.999);
        assert!(
            as_plane(&Geometry::NurbsSurface(Rc::new(patch(
                3.0, 2.0
            ))))
            .is_none()
        );
        assert!(
            as_plane(&Geometry::BRep(Rc::new(BRep::create_box(
                1., 1., 1.
            ))))
            .is_none()
        );
        let mut square = NurbsSurface::new(3, false, 2, 2, 2, 2);

        for (i, j) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            square.set_cv(
                i,
                j,
                &Point::new(i as f64 * 10.0, j as f64 * 10.0, 0.0),
            );
        }

        let cutter = NurbsCurve::create(
            false,
            1,
            &[
                Point::new(5., -1., 0.),
                Point::new(5., 11., 0.),
            ],
        );
        let face =
            session_rust::split::split_surface_by_curves(&square, &[cutter], 1e-6)
                .unwrap();
        assert_eq!(face.face_count(), 2);
        let mut one = face.clone();
        one.m_faces.truncate(1);
        assert!(as_plane(&Geometry::BRep(Rc::new(one))).is_some());
    }

    /// A line along y seen from the top cuts along x = its position; along the view it cannot.
    #[test]
    fn a_fence_stands_on_its_line() {
        let line = Line::from_points(
            &Point::new(3., -5., 0.),
            &Point::new(3., 5., 0.),
        );
        let plane = fence_plane(&line, &Vector::new(0., 0., 1.)).unwrap();
        assert!(plane.z_axis()[0].abs() > 0.999);
        assert!((plane.origin()[0] - 3.0).abs() < 1e-12);
        let up = Line::from_points(
            &Point::new(0., 0., 0.),
            &Point::new(0., 0., 5.),
        );
        assert!(fence_plane(&up, &Vector::new(0., 0., 1.)).is_err());
    }

    /// The nearest polyline within the aperture wins.
    #[test]
    fn the_nearest_polyline_on_screen() {
        let lines = vec![
            vec![(0.0, 0.0), (100.0, 0.0)],
            vec![(0.0, 10.0), (100.0, 10.0)],
        ];
        assert_eq!(
            nearest_px(&lines, (50.0, 3.0), 12.0),
            Some(0)
        );
        assert_eq!(
            nearest_px(&lines, (50.0, 8.0), 12.0),
            Some(1)
        );
        assert_eq!(nearest_px(&lines, (50.0, 40.0), 12.0), None);
    }

    /// A stretched plane keeps its points on it.
    #[test]
    fn a_stretched_plane_keeps_its_points() {
        let plane = Plane::from_point_normal(
            Point::new(1., 2., 3.),
            Vector::new(1., 1., 1.),
            None,
        );
        let stretch = Xform::scale_xyz(2.0, 1.0, 5.0);
        let moved = plane_moved(&plane, &stretch).unwrap();
        let normal = moved.z_axis();

        for p in [
            plane.origin(),
            &plane.origin() + &plane.x_axis(),
            &plane.origin() + &plane.y_axis(),
        ] {
            let q = p.transformed(&stretch);
            assert!((&q - &moved.origin()).dot(&normal).abs() < 1e-9);
        }
    }

    /// A small mesh placed at a large scale is still hit by a ray moved into its frame.
    #[test]
    fn a_scaled_mesh_is_hit() {
        let mesh = session_rust::Mesh::create_box(0.002, 0.002, 0.002);
        let back = Xform::scale_xyz(1e-4, 1e-4, 1e-4);
        let ray = unit_ray(
            &Point::new(0., 0., 100.),
            &Vector::new(0., 0., -1.),
            &back,
        );
        assert!((ray.length() - 1.0).abs() < 1e-12);
        let hits =
            session_rust::intersection::ray_mesh(&ray, &mesh, 1e-12, false).unwrap();
        assert!((hits[0][2] - 0.001).abs() < 1e-9);
    }
}
```

## <a id="checkpoint-81zzc-grow-original-domains-and-extract-the-tail"></a>4. Grow domains and extract the tail

**What this code does:** Define end and reach records.

### 1. `src/extend_reach.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use crate::source_cut::{Cutter, unit_ray};
use session_rust::{
    Geometry, Line, Mesh, NurbsCurve, Point, Polyline, Vector, Xform, intersection,
    split,
};
use std::rc::Rc;

const NEAR: f64 = 1e-6; // a crossing this close to the end is the end itself

/// Which end of a curve grows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum End {
    Start,
    End,
}

/// How far an end grows, in the curve's own frame.
pub enum Reach {
    Distance(f64),
    Boundaries {
        cutters: Vec<Cutter>,           // curves and planes
        meshes: Vec<(Rc<Mesh>, Xform)>, // meshes, each with the map from the curve's frame into its own
    },
}

/// A grown curve and the piece it gained.
pub struct Grown {
    pub geometry: Geometry, // the whole curve
    pub tail: Vec<Point>,   // the piece added, from the old end
}

/// The curve with its domain grown by `s` at `end`.
fn grow(curve: &NurbsCurve, end: End, s: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let mut grown = curve.clone();
    let done = match end {
        End::Start => grown.extend(d0 - s, d1),
        End::End => grown.extend(d0, d1 + s),
    };
    done.then_some(grown)
}

/// The part of `grown` beyond the old domain.
fn beyond(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let (g0, g1) = grown.domain();
    let mut piece = grown.clone();
    let done = match end {
        End::Start => piece.trim(g0, d0),
        End::End => piece.trim(d1, g1),
    };
    done.then_some(piece)
}

/// The added piece as points, from the old end.
fn tail(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Vec<Point> {
    let Some(piece) = beyond(curve, grown, end) else {
        return Vec::new();
    };
    let mut points = crate::source_cut::samples(&piece, 24);

    if end == End::Start {
        points.reverse();
    }

    points
}

/// Per-control colours only fit the control count they were made for.
fn fit_colors(curve: &mut NurbsCurve) {
    if curve.pointcolors.len() != curve.m_cv_count {
        curve.pointcolors.clear();
    }

    if curve.linecolors.len() + 1 != curve.m_cv_count {
        curve.linecolors.clear();
    }
}
```

### 2. `src/lib.rs`

Register this original curve-editing owner.

**Locate this existing block:**

```rust
pub mod source_cut;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_cut;

pub mod extend_reach;
pub mod source_mesh_topology;
```

## <a id="checkpoint-81zzd-find-original-end-directions-and-length-reach"></a>5. Find end directions and length reach

**What this code does:** Find outward directions for supported original curve families.

### 1. `src/extend_reach.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
}

/// The curve with its domain grown by `s` at `end`.
fn grow(curve: &NurbsCurve, end: End, s: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let mut grown = curve.clone();
    let done = match end {
        End::Start => grown.extend(d0 - s, d1),
        End::End => grown.extend(d0, d1 + s),
    };
    done.then_some(grown)
}

/// The part of `grown` beyond the old domain.
fn beyond(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let (g0, g1) = grown.domain();
    let mut piece = grown.clone();
    let done = match end {
        End::Start => piece.trim(g0, d0),
        End::End => piece.trim(d1, g1),
    };
    done.then_some(piece)
}

/// The added piece as points, from the old end.
fn tail(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Vec<Point> {
    let Some(piece) = beyond(curve, grown, end) else {
        return Vec::new();
    };
    let mut points = crate::source_cut::samples(&piece, 24);

    if end == End::Start {
        points.reverse();
    }

    points
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// The end point of a curve and the direction it leaves in.
pub fn tip(geometry: &Geometry, end: End) -> Option<(Point, Vector)> {
    let (tip, inward) = match geometry {
        Geometry::Line(line) => match end {
            End::Start => (line.start(), line.end()),
            End::End => (line.end(), line.start()),
        },
        Geometry::Polyline(polyline) => {
            let count = polyline.point_count();

            if count < 2 {
                return None;
            }

            match end {
                End::Start => (
                    polyline.get_point(0)?,
                    polyline.get_point(1)?,
                ),
                End::End => (
                    polyline.get_point(count - 1)?,
                    polyline.get_point(count - 2)?,
                ),
            }
        }
        Geometry::NurbsCurve(curve) => {
            let (d0, d1) = curve.domain();
            let (t, sign) = match end {
                End::Start => (d0, -1.0),
                End::End => (d1, 1.0),
            };
            let mut direction = &curve.tangent_at(t) * sign;
            direction.normalize_self().then_some(())?;
            return Some((curve.point_at(t), direction));
        }
        _ => return None,
    };
    let mut direction = &tip - &inward;
    direction.normalize_self().then_some(())?;
    Some((tip, direction))
}

/// The curve with its domain grown by `s` at `end`.
fn grow(curve: &NurbsCurve, end: End, s: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let mut grown = curve.clone();
    let done = match end {
        End::Start => grown.extend(d0 - s, d1),
        End::End => grown.extend(d0, d1 + s),
    };
    done.then_some(grown)
}

/// The part of `grown` beyond the old domain.
fn beyond(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let (g0, g1) = grown.domain();
    let mut piece = grown.clone();
    let done = match end {
        End::Start => piece.trim(g0, d0),
        End::End => piece.trim(d1, g1),
    };
    done.then_some(piece)
}

/// The added piece as points, from the old end.
fn tail(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Vec<Point> {
    let Some(piece) = beyond(curve, grown, end) else {
        return Vec::new();
    };
    let mut points = crate::source_cut::samples(&piece, 24);

    if end == End::Start {
        points.reverse();
    }

    points
}

/// A curve grown along its own shape until the added length is `distance`.
fn by_length(
    curve: &NurbsCurve,
    end: End,
    distance: f64,
) -> Result<NurbsCurve, String> {
    let (d0, d1) = curve.domain();
    let at = if end == End::Start {
        d0
    } else {
        d1
    };
    let speed = curve.evaluate(at, 1)[1].magnitude();

    if speed <= 1e-12 {
        return Err("This curve has no direction at its end".into());
    }

    // the added length for a growth of `s`
    let added = |s: f64| -> Result<f64, String> {
        let grown = grow(curve, end, s).ok_or("Cannot extend this curve")?;
        Ok(beyond(curve, &grown, end)
            .ok_or("Cannot extend this curve")?
            .length(None))
    };
    let (mut s0, mut f0) = (0.0, -distance);
    let mut s1 = distance / speed;
    let mut f1 = added(s1)? - distance;

    // secant steps on the added length
    for _ in 0..8 {
        if f1.abs() <= 1e-9 * distance || (f1 - f0).abs() <= f64::MIN_POSITIVE {
            break;
        }

        let s2 = (s1 - f1 * (s1 - s0) / (f1 - f0)).max(s1 * 0.1);
        (s0, f0) = (s1, f1);
        s1 = s2;
        f1 = added(s1)? - distance;
    }

    grow(curve, end, s1).ok_or_else(|| "Cannot extend this curve".into())
}
```

## <a id="checkpoint-81zze-stop-straight-growth-at-the-first-boundary"></a>6. Stop straight growth at the first boundary

**What this code does:** Find the nearest positive crossing of original planes, curves and transformed meshes along a straight end..

### 1. `src/extend_reach.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
}

/// The curve with its domain grown by `s` at `end`.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Where a straight end lands: a distance on, or the first boundary ahead.
fn straight(from: &Point, direction: &Vector, reach: &Reach) -> Result<Point, String> {
    let (cutters, meshes) = match reach {
        Reach::Distance(distance) => return Ok(from + &(direction * *distance)),
        Reach::Boundaries { cutters, meshes } => (cutters, meshes),
    };
    let mut best: Option<f64> = None; // distance ahead of the nearest boundary
    let mut take = |t: f64| {
        if t > NEAR && best.is_none_or(|near| t < near) {
            best = Some(t);
        }
    };

    for cutter in cutters {
        match cutter {
            Cutter::Plane(plane) => {
                let ray = Line::from_points(from, &(from + direction));

                if let Some(hit) = intersection::line_plane(&ray, plane, false) {
                    take((&hit - from).dot(direction));
                }
            }
            Cutter::Curve(curve) => {
                // a probe reaching past every control point of the boundary
                let reach = (0..curve.m_cv_count)
                    .filter_map(|i| curve.get_cv(i))
                    .map(|p| p.distance(from, None))
                    .fold(1.0, f64::max)
                    * 2.0;
                let probe = NurbsCurve::create(
                    false,
                    1,
                    &[from.clone(), from + &(direction * reach)],
                );

                if let Ok(pieces) = split::split_curve_by_curves(
                    &probe,
                    std::slice::from_ref(curve),
                    NEAR,
                ) {
                    for piece in pieces
                        .iter()
                        .take(pieces.len().saturating_sub(1))
                    {
                        take((&piece.point_at_end() - from).dot(direction));
                    }
                }
            }
        }
    }

    for (mesh, into) in meshes {
        let Some(back) = into.inverse() else {
            continue;
        };
        let ray = unit_ray(from, direction, into);

        for hit in intersection::ray_mesh(&ray, mesh, 1e-12, true).unwrap_or_default() {
            take((&hit.transformed(&back) - from).dot(direction));
        }
    }

    best.map(|t| from + &(direction * t))
        .ok_or_else(|| "no boundary ahead".into())
}

/// The curve with its domain grown by `s` at `end`.
```

## <a id="checkpoint-81zzf-stop-curved-growth-at-an-original-boundary"></a>7. Stop curved growth at an boundary

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Drag Left-&gt;Right (Window) vs Right-&gt;Left (Crossing)</text>
  <rect x="54" y="54" width="108" height="88" fill="#dbeafe" fill-opacity="0.35" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="108" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">dx &gt; 0: Inside Only</text>
  <rect x="180" y="54" width="108" height="88" fill="#dcfce7" fill-opacity="0.35" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="234" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">dx &lt; 0: Any Overlap</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Screen AABB Containment vs Segment-Rect Intersection</text>
  <rect x="434" y="56" width="112" height="84" fill="none" stroke="#2563eb" stroke-width="2"/>
  <polygon points="458,122 490,74 522,122" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="574" y="56" width="112" height="84" fill="none" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <polygon points="638,126 676,68 712,126" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Multi-Object Selection Set Updated</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,136 876,74 914,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <polygon points="944,136 982,74 1020,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
</svg>
</div>

**What this code does:** Grow the original NURBS domain and inspect curve or plane crossings beyond its old end.

### 1. `src/extend_reach.rs`

Extend this original owner with today’s complete helper.

**Locate this existing block:**

```rust
}

/// Per-control colours only fit the control count they were made for.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// A curve grown along its own shape to the first boundary curve or plane it meets.
fn to_boundary(
    curve: &NurbsCurve,
    end: End,
    cutters: &[Cutter],
) -> Result<NurbsCurve, String> {
    if cutters.is_empty() {
        return Err(
            "Meshes and BReps stop lines and polylines; pick a curve or plane for a curve".into(),
        );
    }

    let (d0, d1) = curve.domain();
    let at = if end == End::Start {
        d0
    } else {
        d1
    };
    let from = curve.point_at(at);
    let speed = curve.evaluate(at, 1)[1]
        .magnitude()
        .max(1e-12);
    // how far the nearest boundary is, as a first guess
    let away = cutters
        .iter()
        .map(|cutter| match cutter {
            Cutter::Plane(plane) => (&from - &plane.origin())
                .dot(&plane.z_axis())
                .abs(),
            Cutter::Curve(boundary) => boundary
                .closest_point(&from)
                .distance(&from, None),
        })
        .fold(f64::INFINITY, f64::min)
        .max(curve.length(None) * 0.05);
    let mut s = away / speed;

    for _ in 0..7 {
        let grown = grow(curve, end, s).ok_or("Cannot extend this curve")?;
        let piece = beyond(curve, &grown, end).ok_or("Cannot extend this curve")?;
        let (p0, p1) = piece.domain();
        let near = (p1 - p0) * 1e-9;
        let mut crossings = Vec::new();

        for cutter in cutters {
            match cutter {
                Cutter::Plane(plane) => crossings.extend(intersection::curve_plane(
                    &piece,
                    plane,
                    Some(NEAR),
                )),
                Cutter::Curve(boundary) => {
                    let pieces = split::split_curve_by_curves(
                        &piece,
                        std::slice::from_ref(boundary),
                        NEAR,
                    )
                    .unwrap_or_default();

                    for part in pieces
                        .iter()
                        .take(pieces.len().saturating_sub(1))
                    {
                        crossings.push(
                            intersection::curve_closest_point(
                                &piece,
                                &part.point_at_end(),
                                p0,
                                p1,
                            )
                            .0,
                        );
                    }
                }
            }
        }

        // the crossing nearest the old end
        let first = match end {
            End::End => crossings
                .into_iter()
                .filter(|t| *t > p0 + near)
                .min_by(f64::total_cmp),
            End::Start => crossings
                .into_iter()
                .filter(|t| *t < p1 - near)
                .max_by(f64::total_cmp),
        };

        if let Some(t) = first {
            let (g0, g1) = grown.domain();
            let mut cut = grown;
            let done = match end {
                End::End => cut.trim(g0, t),
                End::Start => cut.trim(t, g1),
            };
            return done
                .then_some(cut)
                .ok_or_else(|| "Cannot cut the curve at the boundary".into());
        }

        s *= 2.0;
    }

    Err("does not reach the boundary".into())
}

/// Per-control colours only fit the control count they were made for.
```

## <a id="checkpoint-81zzg-dispatch-original-extension-families"></a>8. Dispatch extension families

**What this code does:** Connect the existing reach helpers for lines, polylines and NURBS curves.

### 1. `src/extend_reach.rs`

Extend this original owner with today’s complete helper. Supply only the unchanged original validation fixtures.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    if curve.linecolors.len() + 1 != curve.m_cv_count {
        curve.linecolors.clear();
    }
```

Replace it with:

```rust
    if curve.linecolors.len() + 1 != curve.m_cv_count {
        curve.linecolors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Plane;

    /// A boundary line x = `x`.
    fn wall(x: f64) -> Cutter {
        Cutter::Curve(NurbsCurve::create(
            false,
            1,
            &[
                Point::new(x, -1000., 0.),
                Point::new(x, 1000., 0.),
            ],
        ))
    }

    /// Boundaries only.
    fn to(cutters: Vec<Cutter>) -> Reach {
        Reach::Boundaries {
            cutters,
            meshes: Vec::new(),
        }
    }

    /// The ends of a line geometry.
    fn ends(grown: &Grown) -> (Point, Point) {
        let Geometry::Line(line) = &grown.geometry else {
            panic!("not a line")
        };
        (line.start(), line.end())
    }

    /// A line grows to the boundary ahead of the chosen end, or by a distance.
    #[test]
    fn a_line_reaches_the_boundary_ahead() {
        let source = Geometry::Line(Rc::new(Line::from_points(
            &Point::new(0., 0., 0.),
            &Point::new(50., 0., 0.),
        )));
        let grown = extended(&source, End::End, &to(vec![wall(100.)])).unwrap();
        assert!(
            ends(&grown)
                .1
                .distance(&Point::new(100., 0., 0.), None)
                < 1e-9
        );
        let grown = extended(
            &source,
            End::Start,
            &to(vec![wall(-20.), wall(100.)]),
        )
        .unwrap();
        assert!(
            ends(&grown)
                .0
                .distance(&Point::new(-20., 0., 0.), None)
                < 1e-9
        );
        let grown = extended(&source, End::Start, &Reach::Distance(25.)).unwrap();
        assert!(
            ends(&grown)
                .0
                .distance(&Point::new(-25., 0., 0.), None)
                < 1e-9
        );
        assert_eq!(
            extended(&source, End::End, &to(vec![wall(-20.)]))
                .err()
                .as_deref(),
            Some("no boundary ahead")
        );
        let plane = Cutter::Plane(Plane::from_point_normal(
            Point::new(80., 0., 0.),
            Vector::new(1., 0., 0.),
            None,
        ));
        let grown = extended(&source, End::End, &to(vec![plane])).unwrap();
        assert!((ends(&grown).1[0] - 80.0).abs() < 1e-9);
        let block = Rc::new(Mesh::create_box(10., 10., 10.));
        let placed = Xform::translation(-200., 0., 0.); // the box sits at x = -200 in the curve's frame
        let meshes = vec![(block, placed.inverse().unwrap())];
        let grown = extended(
            &source,
            End::Start,
            &Reach::Boundaries {
                cutters: Vec::new(),
                meshes,
            },
        )
        .unwrap();
        assert!(
            (ends(&grown).0[0] + 195.0).abs() < 1e-9,
            "{:?}",
            ends(&grown).0
        );
    }

    /// An open polyline's end moves onto the boundary; a closed one has no free end.
    #[test]
    fn polylines_move_their_end_point() {
        let open = Polyline::new(vec![
            Point::new(0., 10., 0.),
            Point::new(0., 0., 0.),
            Point::new(50., 0., 0.),
        ]);
        let grown = extended(
            &Geometry::Polyline(Rc::new(open)),
            End::End,
            &to(vec![wall(100.)]),
        )
        .unwrap();
        let Geometry::Polyline(grown) = &grown.geometry else {
            panic!()
        };
        assert!(
            grown
                .get_point(2)
                .unwrap()
                .distance(&Point::new(100., 0., 0.), None)
                < 1e-9
        );
        let closed = Polyline::new(vec![
            Point::new(0., 0., 0.),
            Point::new(1., 0., 0.),
            Point::new(1., 1., 0.),
            Point::new(0., 0., 0.),
        ]);
        assert_eq!(
            extended(
                &Geometry::Polyline(Rc::new(closed)),
                End::End,
                &Reach::Distance(1.)
            )
            .err()
            .as_deref(),
            Some("no free end")
        );
    }

    /// A NURBS curve grows by an exact length, smoothly, or to a boundary line.
    #[test]
    fn a_curve_grows_along_its_shape() {
        let curve = NurbsCurve::create(
            false,
            2,
            &[
                Point::new(0., -200., 0.),
                Point::new(50., -150., 0.),
                Point::new(100., -200., 0.),
            ],
        );
        let (_, d1) = curve.domain();
        let before = (curve.point_at(d1), curve.tangent_at(d1));
        let source = Geometry::NurbsCurve(Rc::new(curve.clone()));
        let grown = extended(&source, End::End, &Reach::Distance(20.)).unwrap();
        let Geometry::NurbsCurve(longer) = &grown.geometry else {
            panic!()
        };
        assert!((longer.length(None) - curve.length(None) - 20.0).abs() < 1e-6 * 20.0);
        assert!(longer.point_at(d1).distance(&before.0, None) < 1e-9);
        let (mut after, mut was) = (longer.tangent_at(d1), before.1);
        after.normalize_self();
        was.normalize_self();
        assert!(after.cross(&was).magnitude() < 1e-6);
        let grown = extended(&source, End::End, &to(vec![wall(130.)])).unwrap();
        let Geometry::NurbsCurve(reached) = &grown.geometry else {
            panic!()
        };
        assert!(
            (reached.point_at_end()[0] - 130.0).abs() < 1e-6,
            "{:?}",
            reached.point_at_end()
        );
        assert!(!grown.tail.is_empty());
    }
```

**Locate this block:**

```rust
}

/// Where a straight end lands: a distance on, or the first boundary ahead.
fn straight(from: &Point, direction: &Vector, reach: &Reach) -> Result<Point, String> {
    let (cutters, meshes) = match reach {
```

Replace it with:

```rust
}

/// `geometry` grown at `end` by a distance or to the first boundary ahead.
pub fn extended(geometry: &Geometry, end: End, reach: &Reach) -> Result<Grown, String> {
    match geometry {
        Geometry::Line(line) => {
            let (from, direction) =
                tip(geometry, end).ok_or("This line has no length")?;
            let to = straight(&from, &direction, reach)?;
            let grow = from.distance(&to, None);
            let mut next = (**line).clone();

            match end {
                End::Start => next.extend(grow, 0.0),
                End::End => next.extend(0.0, grow),
            }

            Ok(Grown {
                geometry: Geometry::Line(Rc::new(next)),
                tail: vec![from, to],
            })
        }
        Geometry::Polyline(polyline) => {
            if polyline.is_closed() {
                return Err("no free end".into());
            }

            let (from, direction) =
                tip(geometry, end).ok_or("This polyline has no length")?;
            let to = straight(&from, &direction, reach)?;
            let last = polyline.point_count() - 1;
            let mut next: Polyline = (**polyline).clone();
            next.set_point(
                if end == End::Start {
                    0
                } else {
                    last
                },
                &to,
            );
            Ok(Grown {
                geometry: Geometry::Polyline(Rc::new(next)),
                tail: vec![from, to],
            })
        }
        Geometry::NurbsCurve(curve) => {
            if curve.is_closed() {
                return Err("no free end".into());
            }

            let grown = match reach {
                Reach::Distance(distance) => by_length(curve, end, *distance)?,
                Reach::Boundaries { cutters, .. } => to_boundary(curve, end, cutters)?,
            };
            let tail = tail(curve, &grown, end);
            let mut grown = grown;
            fit_colors(&mut grown);
            Ok(Grown {
                geometry: Geometry::NurbsCurve(Rc::new(grown)),
                tail,
            })
        }
        _ => Err("Extend grows lines, polylines and NURBS curves".into()),
    }
}

/// Where a straight end lands: a distance on, or the first boundary ahead.
fn straight(from: &Point, direction: &Vector, reach: &Reach) -> Result<Point, String> {
    let (cutters, meshes) = match reach {
```

## <a id="checkpoint-81a-find-source-cutters-and-current-extension-reach"></a>9. Edit curve intervals

**What this code does:** Trim or extend supported original curves by a normalized physical-length interval.

### 1. `src/interval_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use session_rust::{Geometry, Line};
use std::rc::Rc;
pub const MAX_POINTS: usize = 4096;
/// A part of the selected curve over 0..1 of its length.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Interval {
    Trim(f64, f64),   // keep this part
    Extend(f64, f64), // grow to this range
}

/// The source trimmed or extended to `a..b` of its length.
pub fn edited(source: &Geometry, interval: Interval) -> Result<Geometry, String> {
    let (a, b, trim) = match interval {
        Interval::Trim(a, b) => (a, b, true),
        Interval::Extend(a, b) => (a, b, false),
    };

    if !a.is_finite() || !b.is_finite() || a >= b || a.abs() > 1e6 || b.abs() > 1e6 {
        return Err("parameters must be finite, increasing, and within ±1e6".into());
    }

    if (trim && (a < 0.0 || b > 1.0)) || (!trim && (a > 0.0 || b < 1.0)) {
        return Err(
            "trim keeps 0 <= a < b <= 1; extend needs a <= 0 and b >= 1".into(),
        );
    }

    match source {
        Geometry::Line(line) => {
            let mut next = Line::from_points(&line.point_at(a), &line.point_at(b));
            next.name = line.name.clone();
            next.linecolor = line.linecolor.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.arrowhead = line.arrowhead;
            Ok(Geometry::Line(Rc::new(next)))
        }
        Geometry::NurbsCurve(curve) => {
            if curve.m_cv_count > MAX_POINTS {
                return Err(format!(
                    "curve edits are limited to {MAX_POINTS} controls"
                ));
            }

            let mut next = (**curve).clone();
            let (lo, hi) = next.domain();
            let a = lo + a * (hi - lo); // 0..1 into the curve domain
            let b = lo + b * (hi - lo);
            let ok = if trim {
                next.trim(a, b)
            } else {
                next.extend(a, b)
            };

            if !ok {
                return Err("kernel refused this curve interval".into());
            }

            Ok(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => Err("trim and extend currently accept lines and NURBS curves".into()),
    }
}
```

### 2. `src/lib.rs`

Register this original curve-editing owner.

**Locate this existing block:**

```rust
pub mod extend_reach;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod extend_reach;

pub mod interval_geometry;
pub mod source_mesh_topology;
```

### 3. `src/interval_geometry.rs`

Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Point;
    #[test]
    fn trim_and_extend_preserve_style_and_intervals() {
        let mut line = Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        line.width = 3.0;
        let source = Geometry::Line(Rc::new(line));
        let Geometry::Line(trim) = edited(&source, Interval::Trim(0.2, 0.8)).unwrap()
        else {
            panic!()
        };
        assert_eq!(trim.start()[0], 2.0);
        assert_eq!(trim.end()[0], 8.0);
        assert_eq!(trim.width, 3.0);
        let Geometry::Line(extend) =
            edited(&source, Interval::Extend(-0.5, 1.5)).unwrap()
        else {
            panic!()
        };
        assert_eq!(extend.start()[0], -5.0);
        assert_eq!(extend.end()[0], 15.0);
        assert!(edited(&source, Interval::Trim(-0.1, 0.8)).is_err());
        assert!(edited(&source, Interval::Extend(0.1, 1.5)).is_err());
    }
    #[test]
    fn curve_trim_uses_normalized_domain() {
        let curve = session_rust::NurbsCurve::create(
            false,
            1,
            &[
                Point::new(0.0, 0.0, 0.0),
                Point::new(10.0, 0.0, 0.0),
            ],
        );
        let source = Geometry::NurbsCurve(Rc::new(curve));
        let Geometry::NurbsCurve(curve) =
            edited(&source, Interval::Trim(0.2, 0.8)).unwrap()
        else {
            panic!()
        };
        assert!((curve.point_at_start()[0] - 2.0).abs() < 1e-9);
        assert!((curve.point_at_end()[0] - 8.0).abs() < 1e-9);
    }
}
```

## <a id="checkpoint-81b-capture-eligible-original-identity-and-placement"></a>10. Capture eligible identity and placement

**What this code does:** Capture eligible original identity and placement.

### 1. `src/source_edit.rs`

Capture and validate original allocation, placement and editing eligibility.

Create the file and type:

```rust
use crate::scene::{ObjectId, Scene};
use session_rust::{Geometry, Xform};
use std::rc::Rc;
pub fn original(scene: &Scene, id: ObjectId) -> Option<Geometry> {
    if let Some(row) = scene.objects().iter().find(|r| r.id == id) {
        if let Some(cad) = &row.cad {
            return Some(Geometry::BRep(cad.source.clone()));
        }
        if let Some(s) = &row.surface {
            return Some(Geometry::NurbsSurface(s.source.clone()));
        }
        return row
            .geometry()
            .map(|g| Geometry::Mesh(g.clone()));
    }
    if let Some(row) = scene.points().iter().find(|r| r.id == id) {
        return Some(Geometry::Point(row.prepared.source.clone()));
    }
    crate::gather_draft::Draft::source(scene, id)
}
pub fn same(a: &Geometry, b: &Geometry) -> bool {
    match (a, b) {
        (Geometry::Line(a), Geometry::Line(b)) => Rc::ptr_eq(a, b),
        (Geometry::Polyline(a), Geometry::Polyline(b)) => Rc::ptr_eq(a, b),
        (Geometry::NurbsCurve(a), Geometry::NurbsCurve(b)) => Rc::ptr_eq(a, b),
        (Geometry::Point(a), Geometry::Point(b)) => Rc::ptr_eq(a, b),
        (Geometry::Mesh(a), Geometry::Mesh(b)) => Rc::ptr_eq(a, b),
        (Geometry::BRep(a), Geometry::BRep(b)) => Rc::ptr_eq(a, b),
        (Geometry::NurbsSurface(a), Geometry::NurbsSurface(b)) => Rc::ptr_eq(a, b),
        _ => false,
    }
}
#[derive(Clone)]
pub struct Owner {
    pub id: ObjectId,
    pub source: Geometry,
    pub model: Xform,
}
impl Owner {
    pub fn capture(scene: &Scene, id: ObjectId) -> Result<Self, &'static str> {
        if scene.pick_id(id.value()) != Some(id) {
            return Err("Pick a visible unlocked original");
        }
        Ok(Self {
            id,
            source: original(scene, id).ok_or("Reload an editable original source")?,
            model: scene
                .placement(id)
                .ok_or("Source placement is missing")?,
        })
    }
    pub fn valid(&self, scene: &Scene) -> Result<(), &'static str> {
        if scene.pick_id(self.id.value()) != Some(self.id)
            || scene
                .placement(self.id)
                .is_none_or(|p| p.m != self.model.m)
            || original(scene, self.id).is_none_or(|g| !same(&self.source, &g))
        {
            return Err("Original source changed; start again");
        }
        Ok(())
    }
}
```

### 2. `src/lib.rs`

Register the editing component.

**Locate this existing block:**

```rust
pub mod interval_geometry;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod interval_geometry;

pub mod source_edit;
pub mod source_mesh_topology;
```

## <a id="checkpoint-81c-replace-an-original-without-replacing-its-row"></a>11. Replace an without replacing its row

**What this code does:** Replace an original without replacing its row.

### 1. `src/scene.rs`

Prepare replacement originals before adopting them on the same source row.

**Locate this existing block:**

```rust
impl Scene {
    pub fn deform_part(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Scene {
    pub fn replace_original(
        &mut self,
        id: ObjectId,
        mut geometry: session_rust::Geometry,
    ) -> Result<(), &'static str> {
        use session_rust::Geometry;
        if self.pick_id(id.value()) != Some(id) {
            return Err("Edit a visible unlocked original");
        }
        let original = crate::source_edit::original(self, id)
            .ok_or("Reload an editable original source")?;
        let model = self
            .placement(id)
            .ok_or("Source placement is missing")?;
        geometry.set_guid(original.guid());
        match geometry {
            Geometry::Point(source) => {
                let prepared = crate::marker::PreparedPoint::new(source)?;
                let p = model.transform_point(&prepared.source);
                if (0..3).any(|i| !(p[i] as f32).is_finite()) {
                    return Err("Edited point exceeds display range");
                }
                self.points
                    .iter_mut()
                    .find(|r| r.id == id)
                    .ok_or("Expected a point source")?
                    .prepared = prepared;
            }
            Geometry::Line(source) if self.lines.iter().any(|r| r.id == id) => {
                let prepared = crate::stroke::PreparedLine::new(source)?;
                for p in [
                    prepared.source.start(),
                    prepared.source.end(),
                ] {
                    let p = model.transform_point(&p);
                    if (0..3).any(|i| !(p[i] as f32).is_finite()) {
                        return Err("Edited line exceeds display range");
                    }
                }
                self.lines
                    .iter_mut()
                    .find(|r| r.id == id)
                    .unwrap()
                    .prepared = prepared;
            }
            Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_) => {
                let prepared = match geometry {
                    Geometry::Line(source) => {
                        crate::chain::PreparedChain::line(source)?
                    }
                    Geometry::Polyline(source) => {
                        crate::chain::PreparedChain::polyline(source)?
                    }
                    Geometry::NurbsCurve(source) => {
                        crate::chain::PreparedChain::curve(source)?
                    }
                    _ => unreachable!(),
                };
                for p in prepared
                    .source
                    .coordinates()
                    .into_iter()
                    .chain(prepared.source.bounds_coordinates())
                {
                    let p = model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
                    if (0..3).any(|i| !(p[i] as f32).is_finite()) {
                        return Err("Edited path exceeds display range");
                    }
                }
                self.paths
                    .iter_mut()
                    .find(|r| r.id == id)
                    .ok_or("Expected a path source")?
                    .prepared = prepared;
            }
            Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::NurbsSurface(_) => {
                let (cad, surface, prepared) = match geometry {
                    Geometry::Mesh(source) => (
                        None,
                        None,
                        crate::prepared::PreparedMesh::new((*source).clone())?,
                    ),
                    Geometry::BRep(source) => {
                        let (cad, mesh) = crate::cad_display::prepare(source)?;
                        (Some(Rc::new(cad)), None, mesh)
                    }
                    Geometry::NurbsSurface(source) => {
                        let s = Rc::new(crate::surface::Prepared::new(source)?);
                        let mesh = s.mesh()?;
                        (None, Some(s), mesh)
                    }
                    _ => unreachable!(),
                };
                for v in prepared.display.vertices() {
                    let p = model.transform_point(&session_rust::Point::new(
                        v[0] as f64,
                        v[1] as f64,
                        v[2] as f64,
                    ));
                    if (0..3).any(|i| !(p[i] as f32).is_finite()) {
                        return Err("Edited object exceeds display range");
                    }
                }
                let row = self
                    .objects
                    .iter_mut()
                    .find(|r| r.id == id)
                    .ok_or("Expected an object source")?;
                row.mesh = Rc::new(prepared.display);
                row.cad = cad;
                row.surface = surface;
                row.editable = crate::edit_source::EditSource::Loaded {
                    geometry: prepared.geometry,
                    source: None,
                };
            }
            _ => return Err("This original source cannot be replaced"),
        }
        Ok(())
    }
    pub fn deform_part(
```

## <a id="checkpoint-81d-own-the-originals-selected-for-projection"></a>12. Own the originals selected for projection

**What this code does:** Own the originals selected for projection.

### 1. `src/project_draft.rs`

Capture selected original projection owners.

Create the file and type:

```rust
use crate::{
    scene::{ObjectId, Scene},
    source_edit::Owner,
};
use session_rust::{Geometry, Plane, Point};
#[derive(Clone)]
pub struct Draft {
    pub targets: Vec<Owner>,
    pub unsupported: usize,
    pub before: Vec<ObjectId>,
    pub points: Vec<[f64; 3]>,
}
impl Draft {
    pub fn new(scene: &Scene, before: Vec<ObjectId>) -> Result<Self, &'static str> {
        if before.is_empty() {
            return Err("Select original objects to project");
        }
        let mut targets = Vec::new();
        let mut unsupported = 0;
        for id in &before {
            if scene.pick_id(id.value()) != Some(*id) {
                return Err("Project visible unlocked originals");
            }
            if crate::source_edit::original(scene, *id).is_some() {
                targets.push(Owner::capture(scene, *id)?)
            } else {
                unsupported += 1
            }
        }
        Ok(Self {
            targets,
            unsupported,
            before,
            points: Vec::new(),
        })
    }
}
```

### 2. `src/lib.rs`

Register the editing component.

**Locate this existing block:**

```rust
pub mod source_edit;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod source_edit;

pub mod project_draft;
pub mod source_mesh_topology;
```

## <a id="checkpoint-81e-validate-candidate-plane-answers-before-accepting-them"></a>13. Validate candidate plane answers before accepting them

**What this code does:** Validate candidate plane answers before accepting them.

### 1. `src/project_draft.rs`

Add complete source-editing methods at their dependency boundary.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn accept(&mut self, added: &[[f64; 3]]) -> Result<Option<Plane>, String> {
        let mut points = self.points.clone();
        for point in added {
            points.push(crate::tool_coords::checked(*point).map_err(str::to_owned)?);
            if points.len() > 3 {
                return Err("Project 3Point takes three plane points".into());
            }
            if points.len() == 2
                && (0..3)
                    .map(|i| (points[1][i] - points[0][i]).powi(2))
                    .sum::<f64>()
                    <= 1e-24
            {
                return Err("Pick a point away from the first one".into());
            }
        }
        let plane = if points.len() == 3 {
            let p = points
                .iter()
                .map(|p| Point::new(p[0], p[1], p[2]))
                .collect::<Vec<_>>();
            Some(crate::project_geometry::plane_of(
                &p[0], &p[1], &p[2],
            )?)
        } else {
            None
        };
        self.points = points;
        Ok(plane)
    }
}
```

## <a id="checkpoint-81f-prepare-projection-results-as-one-transaction"></a>14. Prepare projection results as one transaction

**What this code does:** Prepare projection results as one transaction.

### 1. `src/project_draft.rs`

Add complete source-editing methods at their dependency boundary.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn prepare(
        &self,
        scene: &Scene,
        plane: &Plane,
    ) -> Result<(Vec<(ObjectId, Geometry)>, String), String> {
        let mut edits = Vec::new();
        let mut collapsed = 0;
        let mut unsupported = self.unsupported;
        for owner in &self.targets {
            owner.valid(scene).map_err(str::to_owned)?;
            match crate::project_geometry::projected(&owner.source, &owner.model, plane)
            {
                Ok(geometry) => edits.push((owner.id, geometry)),
                Err(crate::project_geometry::Skip::Collapsed) => collapsed += 1,
                Err(crate::project_geometry::Skip::Unsupported) => unsupported += 1,
            }
        }
        if edits.is_empty() {
            return Err(format!(
                "No objects projected · {collapsed} would collapse · {unsupported} unsupported"
            ));
        }
        let message = format!(
            "Projected {} · {collapsed} would collapse · {unsupported} unsupported",
            edits.len()
        );
        Ok((edits, message))
    }
    pub fn prompt(&self) -> String {
        match self.points.len() {
            0 => "Project To Plane · Plane origin · Esc cancels",
            1 => "Project To Plane · Point on the plane’s x axis · Esc cancels",
            _ => "Project To Plane · Third point on the plane · Esc cancels",
        }
        .into()
    }
}
```

## <a id="checkpoint-81g-own-extension-candidates-and-screen-bounds"></a>15. Own extension candidates and screen bounds

**What this code does:** Own extension candidates and screen bounds.

### 1. `src/extend_draft.rs`

Own temporary extension state and screen-search helpers.

Create the file and type:

```rust
use crate::{
    extend_reach::{self, End, Reach},
    scene::{ObjectId, Scene},
    source_cut::{self, Cutter},
    source_edit::Owner,
};
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;
#[derive(Clone)]
pub struct Hover {
    pub owner: Owner,
    pub end: End,
    pub geometry: Geometry,
    pub tail: Vec<[f64; 3]>,
}
#[derive(Clone)]
pub struct Draft {
    pub before: Vec<ObjectId>,
    pub boundaries: Vec<Owner>,
    pub curves: bool,
    pub distance: Option<f64>,
    pub asking: bool,
    pub candidates: Vec<(Owner, Vec<[f64; 3]>, [[f64; 3]; 2])>,
    pub hover: Option<Hover>,
    pub error: String,
    pub cutters: Vec<(Owner, Cutter)>,
    pub meshes: Vec<(Owner, Rc<session_rust::Mesh>)>,
    pub last: Option<(ObjectId, End)>,
}
impl Draft {}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3)
        .map(|i| (a[i] - b[i]).powi(2))
        .sum::<f64>()
        .sqrt()
}
fn bounds(points: &[[f64; 3]]) -> [[f64; 3]; 2] {
    let mut bounds = crate::bounds::Bounds::point(points[0]);
    for p in points {
        bounds.include(*p)
    }
    [bounds.min, bounds.max]
}
fn boxed(
    screen: &crate::snap_math::Screen,
    bounds: &[[f64; 3]; 2],
    at: [f64; 2],
    aperture: f64,
) -> bool {
    let (mut min, mut max) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
    for corner in 0..8 {
        let p = std::array::from_fn(|i| {
            if corner & (1 << i) == 0 {
                bounds[0][i]
            } else {
                bounds[1][i]
            }
        });
        let Some(p) = screen.point(p) else {
            return true;
        };
        for i in 0..2 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    (0..2).all(|i| at[i] >= min[i] - aperture && at[i] <= max[i] + aperture)
}
```

### 2. `src/lib.rs`

Register the editing component.

**Locate this existing block:**

```rust
pub mod project_draft;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod project_draft;

pub mod extend_draft;
pub mod source_mesh_topology;
```

## <a id="checkpoint-81h-gather-boundaries-and-sample-eligible-curve-owners"></a>16. Gather boundaries and sample eligible curve owners

**What this code does:** Gather boundaries and sample eligible curve owners.

### 1. `src/extend_draft.rs`

Add complete source-editing methods at their dependency boundary.

**Locate this existing block:**

```rust
}
impl Draft {}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
impl Draft {
    pub fn pick(&mut self, scene: &Scene, id: Option<ObjectId>) -> Result<(), String> {
        if self.curves {
            return Err("Click near a curve end".into());
        }
        let id = id.ok_or("Pick a boundary original")?;
        if let Some(at) = self
            .boundaries
            .iter()
            .position(|o| o.id == id)
        {
            self.boundaries.remove(at);
            return Ok(());
        }
        let owner = Owner::capture(scene, id).map_err(str::to_owned)?;
        if source_cut::as_curve(&owner.source).is_none()
            && source_cut::as_plane(&owner.source).is_none()
            && !matches!(
                owner.source,
                Geometry::Mesh(_) | Geometry::BRep(_)
            )
        {
            return Err(
                "Stop at a line, polyline, curve, planar surface, mesh or BRep".into(),
            );
        }
        self.boundaries.push(owner);
        Ok(())
    }
    pub fn candidates(scene: &Scene) -> Vec<(Owner, Vec<[f64; 3]>, [[f64; 3]; 2])> {
        let mut candidates = Vec::new();
        let mut count = 0;
        for id in scene
            .lines()
            .iter()
            .map(|r| r.id)
            .chain(scene.paths().iter().map(|r| r.id))
        {
            let Ok(owner) = Owner::capture(scene, id) else {
                continue;
            };
            let Some(curve) = source_cut::as_curve(&owner.source) else {
                continue;
            };
            let points = source_cut::samples(&curve, 32)
                .iter()
                .map(|p| {
                    let p = p.transformed(&owner.model);
                    [p[0], p[1], p[2]]
                })
                .collect::<Vec<_>>();
            if count + points.len() > 200_000 {
                break;
            }
            count += points.len();
            let bounds = bounds(&points);
            candidates.push((owner, points, bounds));
        }
        candidates
    }
}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
```

## <a id="checkpoint-81i-cache-current-boundary-meshes-and-start-the-end-phase"></a>17. Cache current boundary meshes and start the end phase

**What this code does:** Cache current boundary meshes and start the end phase.

### 1. `src/extend_draft.rs`

Add complete source-editing methods at their dependency boundary.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn stops(
        &self,
    ) -> (
        Vec<(Owner, Cutter)>,
        Vec<(Owner, Rc<session_rust::Mesh>)>,
    ) {
        let mut cutters = Vec::new();
        let mut meshes = Vec::new();
        if self.distance.is_none() {
            for owner in &self.boundaries {
                if let Some(plane) = source_cut::as_plane(&owner.source)
                    .and_then(|p| source_cut::plane_moved(&p, &owner.model))
                {
                    cutters.push((owner.clone(), Cutter::Plane(plane)))
                } else if let Some(curve) = source_cut::as_curve(&owner.source) {
                    cutters.push((
                        owner.clone(),
                        Cutter::Curve(curve.transformed(&owner.model)),
                    ))
                } else {
                    match &owner.source {
                        Geometry::Mesh(mesh) => {
                            meshes.push((owner.clone(), mesh.clone()))
                        }
                        Geometry::BRep(brep) => {
                            for mesh in
                                brep.face_meshes_q(Some(crate::cad_boundary::QUALITY))
                            {
                                meshes.push((owner.clone(), Rc::new(mesh)))
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        (cutters, meshes)
    }
    pub fn start(&mut self, scene: &Scene) -> Result<(), String> {
        if self.distance.is_none() && self.boundaries.is_empty() {
            return Err(
                "Select boundary objects, or type Distance and a length".into(),
            );
        }
        for owner in &self.boundaries {
            owner.valid(scene).map_err(str::to_owned)?
        }
        let candidates = Self::candidates(scene);
        let (cutters, meshes) = self.stops();
        self.cutters = cutters;
        self.meshes = meshes;
        self.last = None;
        self.candidates = candidates;
        self.curves = true;
        self.asking = false;
        self.hover = None;
        Ok(())
    }
    pub fn new(
        scene: &Scene,
        before: Vec<ObjectId>,
        distance: Option<f64>,
        asking: bool,
    ) -> Result<Self, String> {
        let mut result = Self {
            before: before.clone(),
            boundaries: Vec::new(),
            curves: false,
            distance,
            asking,
            candidates: Vec::new(),
            hover: None,
            error: String::new(),
            cutters: Vec::new(),
            meshes: Vec::new(),
            last: None,
        };
        if distance.is_some() {
            result.start(scene)?
        } else {
            for id in before {
                let _ = result.pick(scene, Some(id));
            }
        }
        Ok(result)
    }
}
```

## <a id="checkpoint-81j-resolve-distance-and-boundary-reach-in-source-coordinates"></a>18. Resolve distance and boundary reach in source coordinates

**What this code does:** Resolve distance and boundary reach in source coordinates.

### 1. `src/extend_draft.rs`

Add complete source-editing methods at their dependency boundary.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn text(&mut self, scene: &Scene, text: &str) -> Result<bool, String> {
        let words = text.split_whitespace().collect::<Vec<_>>();
        if words.is_empty() {
            if self.curves {
                return Ok(true);
            }
            self.start(scene)?;
            return Ok(false);
        }
        if words.len() == 1 && words[0].eq_ignore_ascii_case("Distance") {
            self.asking = true;
            return Ok(false);
        }
        let value = match words.as_slice() {
            [value] => *value,
            [name, value] if name.eq_ignore_ascii_case("Distance") => *value,
            _ => return Err("Extend Distance needs one positive length".into()),
        };
        let value = value
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && *v > 0.)
            .ok_or("The distance must be above zero")?;
        let mut next = self.clone();
        next.distance = Some(value);
        next.hover = None;
        next.error.clear();
        next.asking = false;
        next.start(scene)?;
        *self = next;
        Ok(false)
    }
    pub fn reach(
        &self,
        scene: &Scene,
        owner: &Owner,
        end: End,
    ) -> Result<Reach, String> {
        owner.valid(scene).map_err(str::to_owned)?;
        let back = owner
            .model
            .inverse()
            .ok_or("Source placement is singular")?;
        if let Some(distance) = self.distance {
            let (tip, direction) = extend_reach::tip(&owner.source, end)
                .ok_or("This original has no free end")?;
            let from = tip.transformed(&owner.model);
            let mut along = &(&tip + &direction).transformed(&owner.model) - &from;
            if !along.normalize_self() {
                return Err("The end direction is singular".into());
            }
            let to = (&from + &(&along * distance)).transformed(&back);
            return Ok(Reach::Distance(to.distance(&tip, None)));
        }
        let mut cutters = Vec::new();
        let mut meshes = Vec::new();
        for (boundary, cutter) in &self.cutters {
            if boundary.id == owner.id {
                continue;
            }
            boundary
                .valid(scene)
                .map_err(str::to_owned)?;
            if let Some(cutter) = cutter.moved(&back) {
                cutters.push(cutter)
            }
        }
        for (boundary, mesh) in &self.meshes {
            if boundary.id == owner.id {
                continue;
            }
            boundary
                .valid(scene)
                .map_err(str::to_owned)?;
            let to = &boundary
                .model
                .inverse()
                .ok_or("Boundary placement is singular")?
                * &owner.model;
            meshes.push((mesh.clone(), to))
        }
        Ok(Reach::Boundaries { cutters, meshes })
    }
}
```

## <a id="checkpoint-81k-find-the-nearest-visible-curve-end-in-css-pixels"></a>19. Find the nearest visible curve end in CSS pixels

**What this code does:** Find the nearest visible curve end in CSS pixels.

### 1. `src/extend_draft.rs`

Add complete source-editing methods at their dependency boundary.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn nearest(
        &self,
        screen: &crate::snap_math::Screen,
        at: [f64; 2],
    ) -> Option<(f64, usize, End)> {
        let mut best: Option<(f64, usize, End)> = None;
        for (index, (_, points, bounds)) in self.candidates.iter().enumerate() {
            if !boxed(screen, bounds, at, 12.) {
                continue;
            }
            let total = points
                .windows(2)
                .map(|p| apart(p[0], p[1]))
                .sum::<f64>();
            let mut walked = 0.;
            for pair in points.windows(2) {
                let step = apart(pair[0], pair[1]);
                if let (Some(a), Some(b)) =
                    (screen.point(pair[0]), screen.point(pair[1]))
                {
                    let distance = source_cut::segment_px(
                        (a[0], a[1]),
                        (b[0], b[1]),
                        (at[0], at[1]),
                    );
                    if distance <= 12. && best.is_none_or(|(near, ..)| distance < near)
                    {
                        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
                        let t = (((at[0] - a[0]) * dx + (at[1] - a[1]) * dy)
                            / (dx * dx + dy * dy).max(1e-12))
                        .clamp(0., 1.);
                        best = Some((
                            distance,
                            index,
                            if walked + t * step < total * 0.5 {
                                End::Start
                            } else {
                                End::End
                            },
                        ));
                    }
                }
                walked += step;
            }
        }
        best
    }
}
```

## <a id="checkpoint-81l-preview-extension-and-refresh-only-changed-originals"></a>20. Preview extension and refresh only changed originals

**What this code does:** Preview extension and refresh only changed originals.

### 1. `src/extend_draft.rs`

Add complete source-editing methods at their dependency boundary.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn follow(
        &mut self,
        scene: &Scene,
        screen: &crate::snap_math::Screen,
        at: [f64; 2],
    ) -> Result<(), String> {
        if !self.curves {
            return Ok(());
        }
        let best = self.nearest(screen, at);
        let Some((_, index, end)) = best else {
            self.hover = None;
            self.last = None;
            self.error.clear();
            return Ok(());
        };
        let owner = self.candidates[index].0.clone();
        if self.last == Some((owner.id, end))
            && owner.valid(scene).is_ok()
            && self
                .boundaries
                .iter()
                .all(|boundary| boundary.valid(scene).is_ok())
        {
            return Ok(());
        }
        self.last = Some((owner.id, end));
        self.hover = None;
        self.error.clear();
        let result = self
            .reach(scene, &owner, end)
            .and_then(|reach| extend_reach::extended(&owner.source, end, &reach));
        match result {
            Ok(grown) => {
                let tail = grown
                    .tail
                    .iter()
                    .map(|p| {
                        let p = p.transformed(&owner.model);
                        [p[0], p[1], p[2]]
                    })
                    .collect();
                self.hover = Some(Hover {
                    owner,
                    end,
                    geometry: grown.geometry,
                    tail,
                })
            }
            Err(error) => self.error = error,
        }
        Ok(())
    }
    pub fn refreshed(&mut self, scene: &Scene, id: ObjectId) -> Result<(), String> {
        let fresh = Owner::capture(scene, id).map_err(str::to_owned)?;
        let curve = source_cut::as_curve(&fresh.source)
            .ok_or("Edited original is not a curve")?;
        let points = source_cut::samples(&curve, 32)
            .iter()
            .map(|p| {
                let p = p.transformed(&fresh.model);
                [p[0], p[1], p[2]]
            })
            .collect::<Vec<_>>();
        let bounds = bounds(&points);
        self.candidates
            .retain(|(owner, ..)| owner.id != id);
        self.candidates.push((fresh, points, bounds));
        self.hover = None;
        self.last = None;
        Ok(())
    }
    pub fn prompt(&self) -> String {
        if self.asking {
            "Extend · type a distance above zero · Esc cancels".into()
        } else if self.curves {
            if !self.error.is_empty() {
                self.error.clone()
            } else {
                "Extend · click near a curve end · Enter finishes · Esc cancels".into()
            }
        } else {
            "Extend · pick boundaries · Enter accepts · Esc cancels".into()
        }
    }
}
```

## <a id="checkpoint-81m-keep-projection-and-extension-in-the-pending-tool-owner"></a>21. Keep projection and extension in the pending tool owner

**What this code does:** Keep projection and extension in the pending tool owner.

### 1. `src/tool.rs`

Connect projection and extension to pending prompts, buttons and temporary guides.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.tool.build_on_plane(&world, self.frame)
    }
    pub fn outlines(&self) -> Vec<Vec<[f64; 3]>> {
        if let Some(gather) = &self.gather {
```

Replace it with:

```rust
        self.tool.build_on_plane(&world, self.frame)
    }
    pub fn outlines(&self) -> Vec<Vec<[f64; 3]>> {
        if let Some(points) = &self.plane_points {
            let mut points = points.clone();
            if let Some(cursor) = self.cursor {
                points.push(cursor)
            }
            return vec![points];
        }
        if let Some(extend) = &self.extend {
            return extend
                .hover
                .as_ref()
                .map_or_else(Vec::new, |h| vec![h.tail.clone()]);
        }
        if let Some(gather) = &self.gather {
```

**Locate this block:**

```rust
    }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {
            if let Some(gather) = &self.gather {
                return gather.prompt();
            }
```

Replace it with:

```rust
    }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {
            if let Some(p) = &self.project {
                return p.prompt();
            }
            if let Some(e) = &self.extend {
                return e.prompt();
            }
        }
        if self.plane_points.is_none() {
            if let Some(gather) = &self.gather {
                return gather.prompt();
            }
```

**Locate this block:**

```rust
    pub primitive: Option<crate::primitive_draft::Draft>,
    pub transform: Option<crate::transform_draft::Draft>,
    pub gather: Option<crate::gather_draft::Draft>,
}

impl Draft {
    pub fn new(shape: Shape) -> Self {
        Self {
            gather: None,
            tool: Box::new(shape),
            points: Vec::new(),
```

Replace it with:

```rust
    pub primitive: Option<crate::primitive_draft::Draft>,
    pub transform: Option<crate::transform_draft::Draft>,
    pub gather: Option<crate::gather_draft::Draft>,
    pub project: Option<crate::project_draft::Draft>,
    pub extend: Option<crate::extend_draft::Draft>,
}

impl Draft {
    pub fn new(shape: Shape) -> Self {
        Self {
            project: None,
            extend: None,
            gather: None,
            tool: Box::new(shape),
            points: Vec::new(),
```

**Locate this block:**

```rust
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        if let Self::Gather(recipe) = self {
            return recipe
                .chips
```

Replace it with:

```rust
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        if matches!(self, Self::Project) {
            return vec![("Cancel".into(), "Escape".into())];
        }
        if matches!(self, Self::Extend) {
            return vec![
                ("Distance".into(), "Distance".into()),
                ("Next / Done".into(), "".into()),
                ("Cancel".into(), "Escape".into()),
            ];
        }
        if let Self::Gather(recipe) = self {
            return recipe
                .chips
```

**Locate this block:**

```rust
    }
    fn count(&self) -> usize {
        match self {
            Self::Gather(_) => 0,
            Self::Point => 1,
            Self::Line | Self::Move => 2,
```

Replace it with:

```rust
    }
    fn count(&self) -> usize {
        match self {
            Self::Project | Self::Extend => 0,
            Self::Gather(_) => 0,
            Self::Point => 1,
            Self::Line | Self::Move => 2,
```

**Locate this block:**

```rust
impl Tool for Shape {
    fn name(&self) -> &'static str {
        match self {
            Self::Gather(recipe) => recipe.name,
            Self::Point => "Point",
            Self::Line => "Line",
```

Replace it with:

```rust
impl Tool for Shape {
    fn name(&self) -> &'static str {
        match self {
            Self::Project => "Project To Plane",
            Self::Extend => "Extend",
            Self::Gather(recipe) => recipe.name,
            Self::Point => "Point",
            Self::Line => "Line",
```

**Locate this block:**

```rust
pub enum Shape {
    Gather(&'static crate::gather_input::Recipe),
    Point,
    Line,
```

Replace it with:

```rust
pub enum Shape {
    Project,
    Extend,
    Gather(&'static crate::gather_input::Recipe),
    Point,
    Line,
```

## <a id="checkpoint-81n-connect-checked-projection-to-one-editor-transaction"></a>22. Connect checked projection to one editor transaction

**What this code does:** Connect checked projection to one editor transaction.

### 1. `src/editor.rs`

Connect complete source-editing methods to one checked history transaction.

**Locate this existing block:**

```rust
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    fn commit_project(
        &mut self,
        project: crate::project_draft::Draft,
        plane: session_rust::Plane,
    ) -> Result<Change, &'static str> {
        let (edits, message) = project
            .prepare(&self.scene, &plane)
            .map_err(|message| self.primitive_failure(message))?;
        self.history
            .try_edit(&mut self.scene, |scene| {
                for (id, geometry) in edits {
                    scene.replace_original(id, geometry)?
                }
                Ok(())
            })?;
        self.select_objects(project.before, false);
        self.draft = None;
        self.tool_status = message;
        Ok(Change::Scene)
    }
    fn begin_project(&mut self, words: &[String]) -> Result<Change, &'static str> {
        crate::drawing_coords::set_plane(self.construction_plane);
        let plan = crate::project_geometry::parse(
            "Project To Plane",
            &words
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map_err(|message| self.primitive_failure(message))?;
        let mut project =
            crate::project_draft::Draft::new(&self.scene, self.selected_objects())?;
        if plan.plane == "3Point" {
            let plane = project
                .accept(&plan.points)
                .map_err(|message| self.primitive_failure(message))?;
            if let Some(plane) = plane {
                return self.commit_project(project, plane);
            }
            self.apply(Action::BeginTool(
                crate::tool::Shape::Project,
            ))?;
            self.draft.as_mut().unwrap().project = Some(project);
            self.tool_status = self.draft.as_ref().unwrap().prompt();
            return Ok(Change::View);
        }
        let frame = match plan.plane {
            "XY" => crate::cplane::CPlane::Xy,
            "YZ" => crate::cplane::CPlane::Yz,
            "ZX" => crate::cplane::CPlane::Xz,
            _ => crate::cplane::CPlane::view(
                &self
                    .camera
                    .orientation
                    .rotate_vector(session_rust::Vector::y_axis()),
            ),
        };
        let (x, y) = frame.axes();
        self.commit_project(
            project,
            session_rust::Plane::from_point_normal(
                session_rust::Point::new(0., 0., 0.),
                x.cross(&y),
                None,
            ),
        )
    }
    fn project_points(&mut self, added: &[[f64; 3]]) -> Result<Change, &'static str> {
        let mut next = self
            .draft
            .as_ref()
            .and_then(|d| d.project.clone())
            .ok_or("No projection tool is running")?;
        let plane = next
            .accept(added)
            .map_err(|message| self.primitive_failure(message))?;
        if let Some(plane) = plane {
            return self.commit_project(next, plane);
        }
        let draft = self.draft.as_mut().unwrap();
        draft.points = next.points.clone();
        draft.project = Some(next);
        draft.cursor = None;
        draft.snapped = None;
        self.tool_status = draft.prompt();
        Ok(Change::View)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

## <a id="checkpoint-81o-connect-boundary-picks-repeated-end-clicks-and-cancellation"></a>23. Connect boundary picks repeated end clicks and cancellation

**What this code does:** Connect boundary picks repeated end clicks and cancellation.

### 1. `src/editor.rs`

Connect complete source-editing methods to one checked history transaction.

**Locate this existing block:**

```rust
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn editing_boundaries(&self) -> bool {
        self.draft.as_ref().is_some_and(|d| {
            d.plane_points.is_none() && d.extend.as_ref().is_some_and(|e| !e.curves)
        })
    }
    fn begin_extend(
        &mut self,
        distance: Option<f64>,
        asking: bool,
    ) -> Result<Change, &'static str> {
        let next = crate::extend_draft::Draft::new(
            &self.scene,
            self.selected_objects(),
            distance,
            asking,
        )
        .map_err(|message| self.primitive_failure(message))?;
        self.apply(Action::BeginTool(crate::tool::Shape::Extend))?;
        self.select_objects([], false);
        self.draft.as_mut().unwrap().extend = Some(next);
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::View)
    }
    fn extend_pick(&mut self, id: Option<ObjectId>) -> Result<Change, &'static str> {
        let mut next = self
            .draft
            .as_ref()
            .and_then(|d| d.extend.clone())
            .ok_or("No extension tool is running")?;
        next.pick(&self.scene, id)
            .map_err(|message| self.primitive_failure(message))?;
        self.draft.as_mut().unwrap().extend = Some(next);
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::View)
    }
    fn extend_text(&mut self, text: &str) -> Result<Change, &'static str> {
        let mut next = self
            .draft
            .as_ref()
            .and_then(|d| d.extend.clone())
            .ok_or("No extension tool is running")?;
        let finished = next
            .text(&self.scene, text)
            .map_err(|message| self.primitive_failure(message))?;
        if finished {
            self.select_objects(next.before, false);
            self.draft = None;
            self.tool_status = "Extend done".into();
            return Ok(Change::View);
        }
        self.draft.as_mut().unwrap().extend = Some(next);
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::View)
    }
    fn extend_screen(
        &mut self,
        screen: [f32; 2],
        size: [f64; 2],
        commit: bool,
    ) -> Result<Change, &'static str> {
        let mut next = self
            .draft
            .as_ref()
            .and_then(|d| d.extend.clone())
            .ok_or("No extension tool is running")?;
        let view = crate::snap_math::Screen {
            matrix: self.camera.view_projection().m,
            size,
        };
        next.follow(
            &self.scene,
            &view,
            [
                (screen[0] as f64 + 1.) * size[0] / 2.,
                (1. - screen[1] as f64) * size[1] / 2.,
            ],
        )
        .map_err(|message| self.primitive_failure(message))?;
        if commit {
            let hover = next.hover.clone().ok_or_else(|| {
                self.primitive_failure(if next.error.is_empty() {
                    "Click near a curve end".into()
                } else {
                    next.error.clone()
                })
            })?;
            hover.owner.valid(&self.scene)?;
            self.history
                .try_edit(&mut self.scene, |scene| {
                    scene.replace_original(hover.owner.id, hover.geometry)
                })?;
            let added = hover
                .tail
                .windows(2)
                .map(|p| {
                    (0..3)
                        .map(|i| (p[1][i] - p[0][i]).powi(2))
                        .sum::<f64>()
                        .sqrt()
                })
                .sum::<f64>();
            next.refreshed(&self.scene, hover.owner.id)
                .map_err(|message| self.primitive_failure(message))?;
            self.tool_status = format!("Extended by {added:.3} · Undo reverts it");
            self.draft.as_mut().unwrap().extend = Some(next);
            return Ok(Change::Scene);
        }
        self.draft.as_mut().unwrap().extend = Some(next);
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::View)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

## <a id="checkpoint-81p-expose-editing-actions-and-normalized-interval-extension"></a>24. Expose editing actions and normalized interval extension

**What this code does:** Expose editing actions and normalized interval extension.

### 1. `src/editor.rs`

Connect complete source-editing methods to one checked history transaction. Dispatch prepared projection, extension and interval actions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.plane_only = false;
        }
        match action {
            Action::GatherBegin { recipe, text } => {
```

Replace it with:

```rust
            self.plane_only = false;
        }
        match action {
            Action::Project(words) => return self.begin_project(&words),
            Action::ExtendBegin { distance, asking } => {
                return self.begin_extend(distance, asking);
            }
            Action::ExtendPick(id) => return self.extend_pick(id),
            Action::Interval(interval) => return self.edit_interval(interval),
            Action::GatherBegin { recipe, text } => {
```

**Locate this block:**

```rust
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::View)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
        if self
            .draft
```

Replace it with:

```rust
        self.tool_status = self.draft.as_ref().unwrap().prompt();
        Ok(Change::View)
    }
    fn edit_interval(
        &mut self,
        interval: crate::interval_geometry::Interval,
    ) -> Result<Change, &'static str> {
        let ids = self.selected_objects();
        let [id] = ids.as_slice() else {
            return Err("Select one original curve");
        };
        let owner = crate::source_edit::Owner::capture(&self.scene, *id)?;
        let geometry = crate::interval_geometry::edited(&owner.source, interval)
            .map_err(|message| self.primitive_failure(message))?;
        owner.valid(&self.scene)?;
        self.history
            .try_edit(&mut self.scene, |scene| {
                scene.replace_original(*id, geometry)
            })?;
        self.draft = None;
        self.tool_status = "Geometry updated".into();
        Ok(Change::Scene)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
        if self
            .draft
```

**Locate this block:**

```rust
    StreamRefresh,
    GatherBegin {
        recipe: &'static crate::gather_input::Recipe,
        text: String,
```

Replace it with:

```rust
    StreamRefresh,
    Project(Vec<String>),
    ExtendBegin {
        distance: Option<f64>,
        asking: bool,
    },
    ExtendPick(Option<ObjectId>),
    Interval(crate::interval_geometry::Interval),
    GatherBegin {
        recipe: &'static crate::gather_input::Recipe,
        text: String,
```

## <a id="checkpoint-81q-route-accepted-editing-input-and-owner-lifecycle-1"></a>25. Route answers to the active editing owner

**What this code does:** Dispatch typed answers and world points to the current draft.

### 1. `src/editor.rs`

Recognize the pending owner’s typed grammar. Dispatch world points before generic point construction. Show pending boundary or gather highlights. Validate gather input and commit projection as one history edit. Accumulate projection points and start extension from captured originals. Commit interval edits and dispatch tool text. Accept projection coordinates before the generic point draft. Restrict self-endpoint snapping to the ordinary curve draft.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                d.primitive.is_none()
                    && d.transform.is_none()
                    && d.gather.is_none()
                    && d.tool.count() == 0
```

Replace it with:

```rust
                d.primitive.is_none()
                    && d.transform.is_none()
                    && d.gather.is_none()
                    && d.project.is_none()
                    && d.extend.is_none()
                    && d.tool.count() == 0
```

**Locate this block:**

```rust
            }
            return self.place_tool_point(added[0]);
        }
        let draft = self.draft.as_mut().unwrap();
        let Some(built) = draft.submit_many(&added)? else {
            self.tool_status = draft.prompt();
```

Replace it with:

```rust
            }
            return self.place_tool_point(added[0]);
        }
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.project.is_some())
        {
            return self.project_points(&added);
        }
        let draft = self.draft.as_mut().unwrap();
        let Some(built) = draft.submit_many(&added)? else {
            self.tool_status = draft.prompt();
```

**Locate this block:**

```rust
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.gather.is_some() && d.plane_points.is_none())
        {
            return self.gather_text(text);
```

Replace it with:

```rust
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.extend.is_some() && d.plane_points.is_none())
        {
            return self.extend_text(text);
        }
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.gather.is_some() && d.plane_points.is_none())
        {
            return self.gather_text(text);
```

**Locate this block:**

```rust
        self.tool_status = draft.prompt();
        Ok(Change::View)
    }
    pub fn editing_boundaries(&self) -> bool {
        self.draft.as_ref().is_some_and(|d| {
            d.plane_points.is_none() && d.extend.as_ref().is_some_and(|e| !e.curves)
        })
    }
    fn begin_extend(
        &mut self,
        distance: Option<f64>,
```

Replace it with:

```rust
        self.tool_status = draft.prompt();
        Ok(Change::View)
    }
    fn begin_extend(
        &mut self,
        distance: Option<f64>,
```

**Locate this block:**

```rust
            .map_err(|message| self.primitive_failure(message))?;
        self.gather_adopt(next, made)
    }
    fn commit_project(
        &mut self,
        project: crate::project_draft::Draft,
```

Replace it with:

```rust
            .map_err(|message| self.primitive_failure(message))?;
        self.gather_adopt(next, made)
    }
    pub fn editing_boundaries(&self) -> bool {
        self.draft.as_ref().is_some_and(|d| {
            d.plane_points.is_none() && d.extend.as_ref().is_some_and(|e| !e.curves)
        })
    }
    fn commit_project(
        &mut self,
        project: crate::project_draft::Draft,
```

**Locate this block:**

```rust
        self.transform_outcome(next, outcome)
    }
    pub fn highlighted_objects(&self) -> Vec<ObjectId> {
        self.draft
            .as_ref()
            .and_then(|d| d.gather.as_ref())
```

Replace it with:

```rust
        self.transform_outcome(next, outcome)
    }
    pub fn highlighted_objects(&self) -> Vec<ObjectId> {
        if let Some(extend) = self
            .draft
            .as_ref()
            .and_then(|d| d.extend.as_ref())
        {
            return extend
                .boundaries
                .iter()
                .map(|o| o.id)
                .collect();
        }
        self.draft
            .as_ref()
            .and_then(|d| d.gather.as_ref())
```

**Locate this block:**

```rust
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.gather.is_some() && d.plane_points.is_none())
        {
            return self.gather_point(point);
```

Replace it with:

```rust
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.project.is_some() && d.plane_points.is_none())
        {
            return self.project_points(&[point]);
        }
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.extend.is_some() && d.plane_points.is_none())
        {
            return Err("Extend uses curve-end clicks");
        }
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.gather.is_some() && d.plane_points.is_none())
        {
            return self.gather_point(point);
```

**Locate this block:**

```rust
        }
        if draft.plane_points.is_none() {
            if let Some(gather) = &draft.gather {
                let first = text.split_whitespace().next();
```

Replace it with:

```rust
        }
        if draft.extend.is_some() && draft.plane_points.is_none() {
            let word = text.trim();
            if word.is_empty()
                || word
                    .split_whitespace()
                    .next()
                    .is_some_and(|w| {
                        w.eq_ignore_ascii_case("Distance") || w.parse::<f64>().is_ok()
                    })
            {
                return Some(Ok(Action::ToolText(text.into())));
            }
            return None;
        }
        if let Some(choice) = crate::drawing_input::choice(text) {
            return Some(Ok(choice));
        }
        if draft.plane_points.is_none() {
            if let Some(gather) = &draft.gather {
                let first = text.split_whitespace().next();
```

## <a id="checkpoint-81r-route-accepted-editing-input-and-owner-lifecycle-2"></a>26. Finish a pending edit through one commit path

**What this code does:** Keep navigation available while an edit is pending, then finish through its owner.

### 1. `src/editor.rs`

Retain drafts during navigation and cancel on unrelated actions. Reject Close for non-curve owners and incomplete curves. Delegate Finish to extension, projection or gather ownership. Route extension screen clicks and ordinary endpoint closure.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                            d.primitive.is_none()
                                && d.transform.is_none()
                                && d.gather.is_none()
                                && d.tool.count() == 0
```

Replace it with:

```rust
                            d.primitive.is_none()
                                && d.transform.is_none()
                                && d.gather.is_none()
                                && d.project.is_none()
                                && d.extend.is_none()
                                && d.tool.count() == 0
```

**Locate this block:**

```rust
                size,
                commit,
            } => {
                let hit = self.snap_point(screen, size);
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = hit
```

Replace it with:

```rust
                size,
                commit,
            } => {
                if self
                    .draft
                    .as_ref()
                    .is_some_and(|d| d.extend.is_some() && d.plane_points.is_none())
                {
                    return self.extend_screen(screen, size, commit);
                }
                let hit = self.snap_point(screen, size);
                if let Some(draft) = self.draft.as_mut() {
                    draft.snapped = hit
```

**Locate this block:**

```rust
                if self
                    .draft
                    .as_ref()
                    .is_some_and(|d| d.gather.is_some())
                {
                    return self.gather_text("");
```

Replace it with:

```rust
                if self
                    .draft
                    .as_ref()
                    .is_some_and(|d| d.extend.is_some())
                {
                    return self.extend_text("");
                }
                if self
                    .draft
                    .as_ref()
                    .is_some_and(|d| d.project.is_some())
                {
                    return Err("Project 3Point needs three plane points");
                }
                if self
                    .draft
                    .as_ref()
                    .is_some_and(|d| d.gather.is_some())
                {
                    return self.gather_text("");
```

**Locate this block:**

```rust
                    .draft
                    .as_ref()
                    .ok_or("No point tool is running")?;
                if draft.gather.is_some() {
                    return Err("Close works on a polyline or curve");
                }
                if draft.tool.count() != 0 || draft.points.len() < 3 {
```

Replace it with:

```rust
                    .draft
                    .as_ref()
                    .ok_or("No point tool is running")?;
                if draft.gather.is_some()
                    || draft.project.is_some()
                    || draft.extend.is_some()
                {
                    return Err("Close works on a polyline or curve");
                }
                if draft.tool.count() != 0 || draft.points.len() < 3 {
```

**Locate this block:**

```rust
            && self
                .draft
                .as_ref()
                .is_some_and(|d| d.gather.is_some())
        {
            self.apply_action(Action::CancelTool)?;
```

Replace it with:

```rust
            && self
                .draft
                .as_ref()
                .is_some_and(|d| d.project.is_some() || d.extend.is_some())
        {
            self.apply_action(Action::CancelTool)?;
        }
        if !keep_edit
            && self
                .draft
                .as_ref()
                .is_some_and(|d| d.gather.is_some())
        {
            self.apply_action(Action::CancelTool)?;
```

**Locate this block:**

```rust
                | Action::GatherPick(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
```

Replace it with:

```rust
                | Action::GatherPick(_)
                | Action::Project(_)
                | Action::ExtendBegin { .. }
                | Action::ExtendPick(_)
                | Action::Interval(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
```

## <a id="checkpoint-81s-route-accepted-editing-input-and-owner-lifecycle-3"></a>27. Restore source selection when an edit is cancelled

**What this code does:** Discard temporary previews and restore the captured selection on cancellation.

### 1. `src/editor.rs`

Restore captured selection and release preview ownership on Cancel.

**Locate this existing block:**

```rust
                    .as_ref()
                    .and_then(|d| d.gather.as_ref())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    .as_ref()
                    .and_then(|d| d.project.as_ref())
                    .map(|p| p.before.clone())
                    .or_else(|| {
                        self.draft
                            .as_ref()
                            .and_then(|d| d.extend.as_ref())
                            .map(|e| e.before.clone())
                    })
                {
                    self.select_objects(before, false);
                }
                if let Some(before) = self
                    .draft
                    .as_ref()
                    .and_then(|d| d.gather.as_ref())
```

## <a id="checkpoint-81t-register-current-projection-and-extension-commands"></a>28. Register current projection and extension commands

**What this code does:** Register current projection and extension commands.

### 1. `src/editing_commands.rs`

Register current projection and extension names and initial argument guards.

Create the file and type:

```rust
use crate::editor::Action;
pub const NAMES: &[&str] = &[
    "Project To Plane",
    "Project To Plane CPlane",
    "Project To Plane XY",
    "Project To Plane YZ",
    "Project To Plane ZX",
    "Project To Plane 3Point",
    "Extend",
    "Extend Distance",
];
pub fn action(name: &str, words: &[&str]) -> Option<Result<Action, String>> {
    if let Some(option) = name.strip_prefix("Project To Plane") {
        let args = option
            .split_whitespace()
            .chain(words.iter().copied())
            .map(str::to_owned)
            .collect();
        return Some(Ok(Action::Project(args)));
    }
    if let Some(option) = name.strip_prefix("Extend") {
        let words = option
            .split_whitespace()
            .chain(words.iter().copied())
            .collect::<Vec<_>>();
        let positive = |word: &str| {
            word.parse::<f64>()
                .ok()
                .filter(|v| v.is_finite() && *v > 0.)
                .ok_or_else(|| "The distance must be above zero".to_owned())
        };
        return Some(match words.as_slice() {
            [] => Ok(Action::ExtendBegin {
                distance: None,
                asking: false,
            }),
            [name] if name.eq_ignore_ascii_case("Distance") => {
                Ok(Action::ExtendBegin {
                    distance: None,
                    asking: true,
                })
            }
            [name, value] if name.eq_ignore_ascii_case("Distance") => positive(value)
                .map(|value| Action::ExtendBegin {
                    distance: Some(value),
                    asking: false,
                }),
            [value] => positive(value).map(|value| Action::ExtendBegin {
                distance: Some(value),
                asking: false,
            }),
            [a, b] => match (a.parse::<f64>(), b.parse::<f64>()) {
                (Ok(a), Ok(b)) => Ok(Action::Interval(
                    crate::interval_geometry::Interval::Extend(a, b),
                )),
                _ => Err("Extend needs a distance or two domain parameters".into()),
            },
            _ => Err("Extend needs a distance or two domain parameters".into()),
        });
    }
    None
}
```

### 2. `src/lib.rs`

Register the editing component.

**Locate this existing block:**

```rust
pub mod extend_draft;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod extend_draft;

pub mod editing_commands;
pub mod source_mesh_topology;
```

### 3. `src/commands.rs`

Connect the source-editing registry to command vocabulary and parsing.

**Locate this existing block:**

```rust
        .chain(
            crate::gather_registry::ENTRIES
                .iter()
                .map(|(name, _)| *name),
        )
        .collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names()
        .into_iter()
        .filter_map(|name| {
            let compact = name
                .split_whitespace()
                .collect::<String>()
                .to_ascii_lowercase();
            let mut rest = compact.as_str();
            for (index, word) in words.iter().enumerate() {
                let word = word.to_ascii_lowercase();
                rest = rest.strip_prefix(&word)?;
                if rest.is_empty() {
                    return Some((name, index + 1, compact.len()));
                }
            }
            None
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let line = crate::gather_registry::alias(line);
    let alias = crate::transform_commands::alias(&line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line = crate::gather_registry::alias(line);
    let alias = crate::transform_commands::alias(&line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action) = crate::transform_commands::action(name, &words[count..]) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .chain(
            crate::editing_commands::NAMES
                .iter()
                .copied(),
        )
        .chain(
            crate::gather_registry::ENTRIES
                .iter()
                .map(|(name, _)| *name),
        )
        .collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names()
        .into_iter()
        .filter_map(|name| {
            let compact = name
                .split_whitespace()
                .collect::<String>()
                .to_ascii_lowercase();
            let mut rest = compact.as_str();
            for (index, word) in words.iter().enumerate() {
                let word = word.to_ascii_lowercase();
                rest = rest.strip_prefix(&word)?;
                if rest.is_empty() {
                    return Some((name, index + 1, compact.len()));
                }
            }
            None
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let line = crate::gather_registry::alias(line);
    let alias = crate::transform_commands::alias(&line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let line = crate::gather_registry::alias(line);
    let alias = crate::transform_commands::alias(&line);
    let line = alias.as_str();
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action) = crate::editing_commands::action(name, &words[count..]) {
        return action.map(Command::Editor);
    }
    if let Some(action) = crate::transform_commands::action(name, &words[count..]) {
```

## <a id="checkpoint-81u-route-whole-boundary-picks-and-observe-pending-edits"></a>29. Route whole boundary picks and observe pending edits

**What this code does:** Route whole boundary picks and observe pending edits.

### 1. `src/browser.rs`

Route editing pointer picks and expose pending-owner facts for browser acceptance.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &originals.to_string(),
    )?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-gather",&editor.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||"null".into(),|g|serde_json::json!({"name":g.recipe.name,"step":g.progress.step,"rows":g.progress.rows,"answered":g.progress.answered,"choice":g.choice}).to_string()))?;
```

Replace it with:

```rust
        &originals.to_string(),
    )?;
    canvas.set_attribute("data-cplane",&serde_json::json!({"fixed":editor.construction_plane.is_some(),"mode":format!("{:?}",editor.construction_plane),"chooser":editor.plane_menu}).to_string())?;
    canvas.set_attribute("data-project", &editor.draft.as_ref().and_then(|d|d.project.as_ref()).map_or_else(||"null".into(), |p|serde_json::json!({"targets":p.targets.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"points":p.points,"unsupported":p.unsupported}).to_string()))?;
    canvas.set_attribute("data-extend", &editor.draft.as_ref().and_then(|d|d.extend.as_ref()).map_or_else(||"null".into(), |e|serde_json::json!({"boundaries":e.boundaries.iter().map(|o|o.id.value()).collect::<Vec<_>>(),"curves":e.curves,"distance":e.distance,"asking":e.asking,"candidates":e.candidates.len(),"hover":e.hover.as_ref().map(|h|serde_json::json!({"owner":h.owner.id.value(),"end":format!("{:?}",h.end),"tail":h.tail})),"error":e.error}).to_string()))?;
    canvas.set_attribute("data-gather",&editor.draft.as_ref().and_then(|d|d.gather.as_ref()).map_or_else(||"null".into(),|g|serde_json::json!({"name":g.recipe.name,"step":g.progress.step,"rows":g.progress.rows,"answered":g.progress.answered,"choice":g.choice}).to_string()))?;
```

**Locate this block:**

```rust
            if let Err(error) = renderer.set_preview(editor.draft.as_ref()) {
                panel.result(error);
            }
            renderer.set_pick_mode(if editor.gathering_curves() {
                crate::pick_sources::Mode::Object
            } else {
                editor.pick_mode
            });
            if highlight_changed {
                if let Err(error) = renderer
                    .set_highlight(editor.display_scene(), editor.selection())
```

Replace it with:

```rust
            if let Err(error) = renderer.set_preview(editor.draft.as_ref()) {
                panel.result(error);
            }
            renderer.set_pick_mode(
                if editor.editing_boundaries() || editor.gathering_curves() {
                    crate::pick_sources::Mode::Object
                } else {
                    editor.pick_mode
                },
            );
            if highlight_changed {
                if let Err(error) = renderer
                    .set_highlight(editor.display_scene(), editor.selection())
```

**Locate this block:**

```rust
                    Action::BeginTool(_)
                        | Action::GatherBegin { .. }
                        | Action::GatherPick(_)
                        | Action::ToolPoint(_)
                        | Action::CancelTool
                        | Action::Snap(_)
```

Replace it with:

```rust
                    Action::BeginTool(_)
                        | Action::GatherBegin { .. }
                        | Action::GatherPick(_)
                        | Action::Project(_)
                        | Action::ExtendBegin { .. }
                        | Action::ExtendPick(_)
                        | Action::Interval(_)
                        | Action::ToolPoint(_)
                        | Action::CancelTool
                        | Action::Snap(_)
```

**Locate this block:**

```rust
                        for action in actions {
                            match action {
                                Action::Pick(screen) => {
                                    if editor.gathering_curves() {
                                        pick_additive.set(false);
                                        pick_remove.set(false);
                                        if let Err(error) = renderer.request_pick(
```

Replace it with:

```rust
                        for action in actions {
                            match action {
                                Action::Pick(screen) => {
                                    if editor.editing_boundaries()
                                        || editor.gathering_curves()
                                    {
                                        pick_additive.set(false);
                                        pick_remove.set(false);
                                        if let Err(error) = renderer.request_pick(
```

**Locate this block:**

```rust
                }
            } else {
                match action {
                    Some(Action::Pick(screen)) if editor.gathering_curves() => {
                        pick_additive.set(false);
                        pick_remove.set(false);
                        if let Err(error) =
```

Replace it with:

```rust
                }
            } else {
                match action {
                    Some(Action::Pick(screen))
                        if editor.editing_boundaries() || editor.gathering_curves() =>
                    {
                        pick_additive.set(false);
                        pick_remove.set(false);
                        if let Err(error) =
```

**Locate this block:**

```rust
                            .and_then(|hit| renderer.resolve_pick(&editor.scene, hit));
                        if editor.gathering_curves() {
                            Some(Action::GatherPick(target.map(|t| t.owner)))
                        } else if editor.canvas_drag.is_some() {
                            Some(Action::CanvasHit(target.map(|t| t.owner)))
```

Replace it with:

```rust
                            .and_then(|hit| renderer.resolve_pick(&editor.scene, hit));
                        if editor.editing_boundaries() {
                            Some(Action::ExtendPick(target.map(|t| t.owner)))
                        } else if editor.gathering_curves() {
                            Some(Action::GatherPick(target.map(|t| t.owner)))
                        } else if editor.canvas_drag.is_some() {
                            Some(Action::CanvasHit(target.map(|t| t.owner)))
```

## <a id="checkpoint-81v-exercise-original-projection-retry-and-repeated-extension"></a>30. Exercise projection retry and repeated extension

**What this code does:** Exercise original projection retry and repeated extension.

### 1. `src/editing_tests.rs`

Copy integrated original-edit, retry, stale-owner, history and archive fixtures.

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene::Scene,
};
use session_rust::{Geometry, Point, Xform};
use std::rc::Rc;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, line: &str) -> Result<(), String> {
    let action = if let Some(a) = e.tool_line(line) {
        a.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(a) = crate::commands::parse(line)? else {
            return Err("Editor command".into());
        };
        a
    };
    e.apply(action).map_err(|message| {
        if message == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            message.into()
        }
    })?;
    Ok(())
}
fn end_screen(
    e: &Editor,
    id: crate::scene::ObjectId,
    end: crate::extend_reach::End,
) -> [f32; 2] {
    let source = crate::source_edit::original(&e.scene, id).unwrap();
    let (tip, _) = crate::extend_reach::tip(&source, end).unwrap();
    let p = tip.transformed(&e.scene.placement(id).unwrap());
    let q = e
        .camera
        .view_projection()
        .transform_point(&p);
    [q[0] as f32, q[1] as f32]
}
#[test]
fn projection_edits_original_basic_lanes_skips_cad_preserves_identity_and_one_undo() {
    let mut e = editor();
    for line in [
        "Point w0,0,2",
        "Line w0,0,1 w2,1,3",
        "Polyline w0,0,1 w1,0,2 w1,1,3",
        "Nurbs Curve Arc Center w3,0,1 0.5 270",
        "Box w5,0,0 1 1 1",
    ] {
        command(&mut e, line).unwrap()
    }
    let mesh = crate::prepared::PreparedMesh::new(
        session_rust::Mesh::from_vertices_and_faces(
            vec![
                Point::new(0., 0., 1.),
                Point::new(1., 0., 2.),
                Point::new(0., 1., 3.),
            ],
            vec![vec![0, 1, 2]],
        ),
    )
    .unwrap();
    let mesh_id = e.scene.insert(mesh).unwrap();
    let line_id = e.scene.lines()[0].id;
    let source = e.scene.lines()[0].prepared.source.clone();
    let guid = source.guid().to_owned();
    let row_guid = e.scene.lines()[0].guid.clone();
    let cad = e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source
        .clone();
    e.scene
        .place_following(line_id, Xform::translation(10., 0., 2.))
        .unwrap();
    command(&mut e, "Select All").unwrap();
    command(&mut e, "Project To Plane XY").unwrap();
    assert!(e.tool_status.contains("1 unsupported"));
    assert_eq!(e.scene.lines()[0].guid, row_guid);
    assert_eq!(
        e.scene.lines()[0].prepared.source.guid(),
        guid
    );
    assert_eq!(e.scene.lines()[0].model.m[12], 10.);
    assert!(
        e.scene.lines()[0]
            .endpoints()
            .iter()
            .all(|p| p[2].abs() < 1e-12)
    );
    assert_eq!(source.start()[2], 1.);
    assert!(Rc::ptr_eq(
        &cad,
        &e.scene.objects()[0]
            .cad
            .as_ref()
            .unwrap()
            .source
    ));
    let Geometry::Mesh(m) = crate::source_edit::original(&e.scene, mesh_id).unwrap()
    else {
        panic!()
    };
    assert!(m.vertex.values().all(|v| v.z.abs() < 1e-12));
    let bytes = crate::scene_archive::save(&e.scene).unwrap();
    let mut reopened = Scene::empty();
    crate::scene_document::open(&mut reopened, &bytes).unwrap();
    assert!(
        reopened.lines()[0]
            .endpoints()
            .iter()
            .all(|p| p[2].abs() < 1e-12)
    );
    command(&mut e, "Undo").unwrap();
    assert!(Rc::ptr_eq(
        &source,
        &e.scene.lines()[0].prepared.source
    ));
    assert_eq!(e.scene.points()[0].prepared.source[2], 2.);
    command(&mut e, "Redo").unwrap();
    assert!(!Rc::ptr_eq(
        &source,
        &e.scene.lines()[0].prepared.source
    ));
}
#[test]
fn three_point_projection_retries_bad_second_and_third_points_without_adopting_input() {
    let mut e = editor();
    command(&mut e, "Line w0,0,1 w1,1,2").unwrap();
    let id = e.scene.lines()[0].id;
    e.select_objects([id], false);
    command(&mut e, "Project To Plane 3Point").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "w0,0,0").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    command(&mut e, "w1,0,0").unwrap();
    e.apply(Action::ToolHover(Some([0., 1., 0.])))
        .unwrap();
    assert!(command(&mut e, "w2,0,0").is_err());
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 2);
    assert_eq!(
        e.scene.lines()[0].prepared.source.start()[2],
        1.
    );
    command(&mut e, "w0,1,0").unwrap();
    assert!(e.draft.is_none());
    assert!(
        e.scene.lines()[0]
            .endpoints()
            .iter()
            .all(|p| p[2].abs() < 1e-12)
    );
    command(&mut e, "Undo").unwrap();
    assert_eq!(
        e.scene.lines()[0].prepared.source.start()[2],
        1.
    );
}
#[test]
fn collapsed_or_unsupported_projection_preserves_selection_redo_and_previous_owner() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w0,0,1").unwrap();
    command(&mut e, "Point w3,0,0").unwrap();
    command(&mut e, "Undo").unwrap();
    let id = e.scene.lines()[0].id;
    e.select_objects([id], false);
    command(&mut e, "Curve").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "Project To Plane XY").is_err());
    assert_eq!(
        e.draft.as_ref().unwrap().tool.name(),
        "Curve"
    );
    assert_eq!(e.draft.as_ref().unwrap().points.len(), 1);
    assert_eq!(e.selected_objects(), vec![id]);
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
    assert!(
        command(
            &mut e,
            "Project To Plane 3Point w0,0,0 w1,0,0"
        )
        .is_err()
    );
}
#[test]
fn extend_world_distance_handles_scaled_sources_keeps_identity_and_repeats_transactions()
 {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    let id = e.scene.lines()[0].id;
    let source = e.scene.lines()[0].prepared.source.clone();
    let guid = source.guid().to_owned();
    e.scene
        .place_following(id, Xform::scale_xyz(1000., 1000., 1000.))
        .unwrap();
    command(&mut e, "View Reset").unwrap();
    command(&mut e, "Fit").unwrap();
    command(&mut e, "Extend Distance 25").unwrap();
    for _ in 0..2 {
        let screen = end_screen(&e, id, crate::extend_reach::End::End);
        e.apply(Action::ToolScreen {
            screen,
            size: [640., 480.],
            commit: false,
        })
        .unwrap();
        assert!(
            e.draft
                .as_ref()
                .unwrap()
                .extend
                .as_ref()
                .unwrap()
                .hover
                .is_some()
        );
        e.apply(Action::ToolScreen {
            screen,
            size: [640., 480.],
            commit: true,
        })
        .unwrap();
    }
    assert!((e.scene.lines()[0].endpoints()[1][0] - 1050.).abs() < 1e-7);
    assert_eq!(
        e.scene.lines()[0].prepared.source.guid(),
        guid
    );
    assert_eq!(source.end()[0], 1.);
    command(&mut e, "").unwrap();
    assert!(e.draft.is_none());
    command(&mut e, "Undo").unwrap();
    assert!((e.scene.lines()[0].endpoints()[1][0] - 1025.).abs() < 1e-7);
    command(&mut e, "Undo").unwrap();
    assert!(Rc::ptr_eq(
        &source,
        &e.scene.lines()[0].prepared.source
    ));
    command(&mut e, "Redo").unwrap();
    assert!((e.scene.lines()[0].endpoints()[1][0] - 1025.).abs() < 1e-7);
}
#[test]
fn extend_boundary_picks_stay_separate_and_original_end_reaches_nearest_world_boundary()
{
    let mut e = editor();
    for line in [
        "Line w0,0,0 w1,0,0",
        "Line w3,-1,0 w3,1,0",
        "Line w5,-1,0 w5,1,0",
        "View Reset",
        "Fit",
    ] {
        command(&mut e, line).unwrap()
    }
    let (id, near, far) = (
        e.scene.lines()[0].id,
        e.scene.lines()[1].id,
        e.scene.lines()[2].id,
    );
    e.select_objects([near, far], false);
    command(&mut e, "Extend").unwrap();
    assert!(e.selected_objects().is_empty());
    assert!(e.editing_boundaries());
    assert_eq!(e.highlighted_objects(), vec![near, far]);
    command(&mut e, "").unwrap();
    let screen = end_screen(&e, id, crate::extend_reach::End::End);
    e.apply(Action::ToolScreen {
        screen,
        size: [640., 480.],
        commit: true,
    })
    .unwrap();
    assert!((e.scene.lines()[0].endpoints()[1][0] - 3.).abs() < 1e-6);
    command(&mut e, "Escape").unwrap();
    assert_eq!(e.selected_objects(), vec![near, far]);
    command(&mut e, "Undo").unwrap();
    assert_eq!(e.scene.lines()[0].endpoints()[1][0], 1.);
}
#[test]
fn normalized_extension_ranges_preserve_original_style_and_reject_invalid_input_atomically()
 {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w10,0,0").unwrap();
    let id = e.scene.lines()[0].id;
    let original = e.scene.lines()[0].prepared.source.clone();
    e.select_objects([id], false);
    command(&mut e, "Extend -0.5 1.5").unwrap();
    assert_eq!(
        e.scene.lines()[0].prepared.source.start()[0],
        -5.
    );
    assert_eq!(
        e.scene.lines()[0].prepared.source.end()[0],
        15.
    );
    assert_eq!(
        e.scene.lines()[0].prepared.source.linecolor,
        original.linecolor
    );
    command(&mut e, "Undo").unwrap();
    command(&mut e, "Extend Distance").unwrap();
    assert!(command(&mut e, "0").is_err());
    assert!(
        e.draft
            .as_ref()
            .unwrap()
            .extend
            .as_ref()
            .unwrap()
            .asking
    );
    assert!(command(&mut e, "-1").is_err());
    command(&mut e, "2").unwrap();
    assert!(
        e.draft
            .as_ref()
            .unwrap()
            .extend
            .as_ref()
            .unwrap()
            .curves
    );
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(
        e.scene.lines()[0].prepared.source.end()[0],
        15.
    );
    assert!(command(&mut e, "Extend 0.1 1.5").is_err());
    assert_eq!(
        e.scene.lines()[0].prepared.source.end()[0],
        15.
    );
}
#[test]
fn stale_extension_sources_and_missed_end_clicks_never_create_history() {
    let mut e = editor();
    command(&mut e, "Line w0,0,0 w1,0,0").unwrap();
    command(&mut e, "Point w2,0,0").unwrap();
    command(&mut e, "Undo").unwrap();
    let id = e.scene.lines()[0].id;
    command(&mut e, "View Reset").unwrap();
    command(&mut e, "Fit").unwrap();
    command(&mut e, "Extend 2").unwrap();
    assert!(
        e.apply(Action::ToolScreen {
            screen: [0.9, 0.9],
            size: [640., 480.],
            commit: true
        })
        .is_err()
    );
    let screen = end_screen(&e, id, crate::extend_reach::End::End);
    e.scene
        .place_following(id, Xform::translation(1., 0., 0.))
        .unwrap();
    assert!(
        e.apply(Action::ToolScreen {
            screen,
            size: [640., 480.],
            commit: true
        })
        .is_err()
    );
    assert_eq!(
        e.scene.lines()[0].prepared.source.end()[0],
        1.
    );
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.points().len(), 1);
}
```

### 2. `src/lib.rs`

Register the editing component.

**Locate this existing block:**

```rust
pub mod editing_commands;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod editing_commands;

#[cfg(test)]
mod editing_tests;
pub mod source_mesh_topology;
```

## <a id="checkpoint-81za-validate-original-split-inputs-and-name-pieces"></a>31. Validate Split inputs and name pieces

**What this code does:** Accept original cutter curves and a valid selected BRep face.

### 1. `src/split_geometry.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use session_rust::{BRep, Geometry, NurbsCurve, split};
use std::rc::Rc;
/// True for a geometry that can cut: a line, polyline or curve.
pub fn is_cutter(geometry: &Geometry) -> bool {
    matches!(
        geometry,
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_)
    )
}

/// The face to split: None for a curve, the selected face for a BRep.
pub fn face_index(
    geometry: &Geometry,
    selected: Option<usize>,
) -> Result<Option<usize>, String> {
    match geometry {
        Geometry::Line(_)
        | Geometry::Polyline(_)
        | Geometry::NurbsCurve(_)
        | Geometry::NurbsSurface(_) => Ok(None),
        Geometry::BRep(brep) => brep_face(brep, selected).map(Some),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) => {
                brep_face(brep, selected).map(Some)
            }
            _ => Err("Split accepts curves and NURBS/BRep faces".into()),
        },
        _ => {
            Err("Split accepts lines, polylines, NURBS curves and surface faces".into())
        }
    }
}

/// The selected face, or the only one.
fn brep_face(brep: &BRep, selected: Option<usize>) -> Result<usize, String> {
    selected
        .or((brep.face_count() == 1).then_some(0))
        .filter(|face| *face < brep.face_count())
        .ok_or_else(|| {
            "Ctrl+Shift-select one BRep face before Split; the solid stays joined"
                .into()
        })
}

/// A cutter as a NURBS curve.
pub fn curve(geometry: &Geometry) -> Result<NurbsCurve, String> {
    match geometry {
        Geometry::Line(line) => Ok(NurbsCurve::create(
            false,
            1,
            &[line.point_at(0.), line.point_at(1.)],
        )),
        Geometry::Polyline(polyline) => Ok(NurbsCurve::create(
            false,
            1,
            &polyline.get_points(),
        )),
        Geometry::NurbsCurve(curve) => Ok((**curve).clone()),
        _ => Err("Choose a line, polyline or NURBS curve as cutter".into()),
    }
}

pub fn named(source: &Geometry, mut pieces: Vec<Geometry>) -> Vec<Geometry> {
    // name the pieces `x (part 1)`, `x (part 2)`...
    if pieces.len() > 1 {
        let name = source.name();

        for (index, piece) in pieces.iter_mut().enumerate() {
            let name = format!("{name} (part {})", index + 1);

            match piece {
                Geometry::Line(p) => Rc::make_mut(p).name = name,
                Geometry::Polyline(p) => Rc::make_mut(p).name = name,
                Geometry::NurbsCurve(p) => Rc::make_mut(p).name = name,
                _ => unreachable!("only curves create sibling objects"),
            }
        }
    }

    pieces
}
```

### 2. `src/lib.rs`

Register this original source-splitting owner.

**Locate this existing block:**

```rust
mod editing_tests;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod editing_tests;

pub mod source_mesh_topology;
pub mod split_geometry;

#[cfg(target_arch = "wasm32")]
```

## Build & Run

![Recorded visual output for Lesson 16.1 (81za-validate-original-split-inputs-and-name-pieces)](../../../screenshots/journey/81za-validate-original-split-inputs-and-name-pieces.png)

```sh
npm --prefix ../session_tests run course -- dependencies 16.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 16.1
npm --prefix ../session_tests run course -- save 16.1
```

Continue with [16.2 · Split original curves and joined faces](../../consolidation/82-divide-current-original-curve-families.md).
