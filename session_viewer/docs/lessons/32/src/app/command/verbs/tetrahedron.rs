use crate::app::command::tool::shape::{self, BREP_MESH, Part, Shape};
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Mesh, Point};
use std::sync::OnceLock;

pub const SPEC: Spec = Spec {
    names: &["Tetrahedron"],
    aliases: &[],
    hint: "Tetrahedron (Brep Mesh): center, radius to the corners · Example: Tetrahedron 0,0,0 10",
    options: &["Tetrahedron Brep", "Tetrahedron Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Tetrahedron",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline,
    build,
};

static UNIT: OnceLock<Vec<Vec<[f64; 3]>>> = OnceLock::new(); // faces at radius 1

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// The kernel mesh of edge length `edge`.
fn make(edge: f64) -> Mesh {
    session_rust::Primitives::tetrahedron(edge)
}

/// The faces at the radius.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    shape::polyhedron_outline(part, shape::unit_faces(&UNIT, make))
}

/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "tetrahedron")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::Answer;
    use crate::app::command::tool::shape::tests::{XZ, build, corners, p, plane};

    /// Every corner at the radius; the Brep option is a closed solid of 4 faces.
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
        assert_eq!(brep.face_count(), 4);
        assert!(brep.is_solid());
    }
}
