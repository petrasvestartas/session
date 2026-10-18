use session_rust::{Geometry,Xform};
/// World length of a line, polyline or NURBS curve placed by `place`; None for other kinds.
pub fn compute_length(geometry: &Geometry, place: &Xform) -> Option<f64> {
    match geometry {
        Geometry::Line(line) => Some(line.transformed(place).length()),
        Geometry::Polyline(polyline) => Some(polyline.transformed(place).length()),
        Geometry::NurbsCurve(curve) => Some(curve.transformed(place).length(None)),
        _ => None,
    }
}

