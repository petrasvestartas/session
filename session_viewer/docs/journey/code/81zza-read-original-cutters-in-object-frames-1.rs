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

/// `plane` moved by `xform`, rebuilt from three moved points so a stretch keeps it true.
pub fn plane_moved(plane: &Plane, xform: &Xform) -> Option<Plane> {
    let origin = plane.origin();
    let a = origin.transformed(xform);
    let b = (&origin + &plane.x_axis()).transformed(xform);
    let c = (&origin + &plane.y_axis()).transformed(xform);
    let normal = (&b - &a).cross(&(&c - &a));
    (normal.magnitude() > 1e-14).then(|| Plane::from_point_normal(a, normal, None))
}

