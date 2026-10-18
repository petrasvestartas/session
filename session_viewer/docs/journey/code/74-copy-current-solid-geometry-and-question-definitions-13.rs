use crate::primitive_shape as shape;
use crate::primitive_shape::{ BREP_MESH, Part, Shape};
use session_rust::{Geometry, Mesh, Point};
use std::sync::OnceLock;

pub static SHAPE: Shape = Shape {
    name: "Octahedron",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline,
    build,
};

static UNIT: OnceLock<Vec<Vec<[f64; 3]>>> = OnceLock::new(); // faces at radius 1

/// The kernel mesh of edge length `edge`.
fn make(edge: f64) -> Mesh {
    session_rust::Primitives::octahedron(edge)
}

/// The faces at the radius.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    shape::polyhedron_outline(part, shape::unit_faces(&UNIT, make))
}

/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "octahedron")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::Answer;
    use crate::primitive_shape::tests::{XZ, build, corners, p, plane};

    /// Every corner at the radius; the Brep option is a closed solid of 8 faces.
    #[test]
    fn the_corners_lie_at_the_radius() {
        let center = p(1.0, 2.0, 3.0);
        let answers = [Answer::Point(center.clone()), Answer::Number(300.0)];
        let mesh = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Mesh").unwrap();
        assert!(matches!(mesh, Geometry::Mesh(_)));

        for q in corners(&mesh) {
            assert!((q.distance(&center, None) - 300.0).abs() < 1e-9 * 300.0);
        }

        let Geometry::BRep(brep) = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 8);
        assert!(brep.is_solid());
    }
}
