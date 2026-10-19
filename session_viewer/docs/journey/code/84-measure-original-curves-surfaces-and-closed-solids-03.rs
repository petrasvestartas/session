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

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Line, Mesh, NurbsCurve, Point, Polyline};
    use std::rc::Rc;

    #[test]
    fn curves_measure_in_world_units() {
        let identity = Xform::identity();
        let line = Geometry::Line(Rc::new(Line::new(0.0, 0.0, 0.0, 3.0, 4.0, 0.0)));
        assert_eq!(compute_length(&line, &identity), Some(5.0));
        let polyline = Geometry::Polyline(Rc::new(Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(60.0, 0.0, 0.0),
            Point::new(60.0, 60.0, 0.0),
        ])));
        let place = &Xform::translation(20.0, 0.0, 0.0) * &Xform::scale_xyz(1.5, 1.5, 1.5);
        let placed = compute_length(&polyline, &place).unwrap();
        assert!((placed - 180.0).abs() < 1e-9);
        let points = [
            Point::new(8.0, 3.0, 0.0),
            Point::new(9.0, 4.0, 0.0),
            Point::new(10.0, 3.0, 0.0),
        ];
        let curve = Geometry::NurbsCurve(Rc::new(NurbsCurve::create(false, 2, &points)));
        let exact = (2.0 * 2.0_f64.sqrt() + 2.0 * 1.0_f64.asinh()) / 2.0;
        assert!((compute_length(&curve, &identity).unwrap() - exact).abs() < 1e-9);
        let mesh = Geometry::Mesh(Rc::new(Mesh::create_box(1.0, 1.0, 1.0)));
        assert_eq!(compute_length(&mesh, &identity), None);
    }
}
