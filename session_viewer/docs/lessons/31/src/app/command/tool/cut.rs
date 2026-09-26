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
                let lift = if i == 1 && j == 1 { bend } else { 0.0 };
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
        let plane = Plane::from_point_normal(Point::new(0., 0., 2.), Vector::new(0., 0., 1.), None);
        assert!(as_plane(&Geometry::Plane(Rc::new(plane))).is_some());
        let flat = as_plane(&Geometry::NurbsSurface(Rc::new(patch(3.0, 0.0)))).unwrap();
        assert!((flat.origin()[2] - 3.0).abs() < 1e-9);
        assert!(flat.z_axis()[2].abs() > 0.999);
        assert!(as_plane(&Geometry::NurbsSurface(Rc::new(patch(3.0, 2.0)))).is_none());
        assert!(as_plane(&Geometry::BRep(Rc::new(BRep::create_box(1., 1., 1.)))).is_none());
        let mut square = NurbsSurface::new(3, false, 2, 2, 2, 2);

        for (i, j) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            square.set_cv(i, j, &Point::new(i as f64 * 10.0, j as f64 * 10.0, 0.0));
        }

        let cutter = NurbsCurve::create(
            false,
            1,
            &[Point::new(5., -1., 0.), Point::new(5., 11., 0.)],
        );
        let face =
            session_rust::simple_split::split_surface_by_curves(&square, &[cutter], 1e-6).unwrap();
        assert_eq!(face.face_count(), 2);
        let mut one = face.clone();
        one.m_faces.truncate(1);
        assert!(as_plane(&Geometry::BRep(Rc::new(one))).is_some());
    }

    /// A line along y seen from the top cuts along x = its position; along the view it cannot.
    #[test]
    fn a_fence_stands_on_its_line() {
        let line = Line::from_points(&Point::new(3., -5., 0.), &Point::new(3., 5., 0.));
        let plane = fence_plane(&line, &Vector::new(0., 0., 1.)).unwrap();
        assert!(plane.z_axis()[0].abs() > 0.999);
        assert!((plane.origin()[0] - 3.0).abs() < 1e-12);
        let up = Line::from_points(&Point::new(0., 0., 0.), &Point::new(0., 0., 5.));
        assert!(fence_plane(&up, &Vector::new(0., 0., 1.)).is_err());
    }

    /// The nearest polyline within the aperture wins.
    #[test]
    fn the_nearest_polyline_on_screen() {
        let lines = vec![
            vec![(0.0, 0.0), (100.0, 0.0)],
            vec![(0.0, 10.0), (100.0, 10.0)],
        ];
        assert_eq!(nearest_px(&lines, (50.0, 3.0), 12.0), Some(0));
        assert_eq!(nearest_px(&lines, (50.0, 8.0), 12.0), Some(1));
        assert_eq!(nearest_px(&lines, (50.0, 40.0), 12.0), None);
    }

    /// A stretched plane keeps its points on it.
    #[test]
    fn a_stretched_plane_keeps_its_points() {
        let plane = Plane::from_point_normal(Point::new(1., 2., 3.), Vector::new(1., 1., 1.), None);
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
        let ray = unit_ray(&Point::new(0., 0., 100.), &Vector::new(0., 0., -1.), &back);
        assert!((ray.length() - 1.0).abs() < 1e-12);
        let hits = session_rust::intersection::ray_mesh(&ray, &mesh, 1e-12, false).unwrap();
        assert!((hits[0][2] - 0.001).abs() < 1e-9);
    }
}
