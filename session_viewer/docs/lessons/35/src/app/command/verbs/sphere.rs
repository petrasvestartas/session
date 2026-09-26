use crate::app::command::tool::shape::{self, BREP_MESH, Part, Shape};
use crate::app::command::{Action, Spec};
use session_rust::{BRep, Geometry};

pub const SPEC: Spec = Spec {
    names: &["Sphere"],
    aliases: &[],
    hint: "Sphere (Brep Mesh): center, radius · Example: Sphere 0,0,0 10",
    options: &["Sphere Brep", "Sphere Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Sphere",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// A kernel sphere at the center, poles along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Sphere needs a radius".into());
    };
    Ok(shape::solid(
        BRep::create_sphere(radius),
        option,
        &part.frame.to_xform(),
        "sphere",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::Answer;
    use crate::app::command::tool::shape::tests::{XZ, build, corners, p, plane};

    /// One face, poles on the normal; the Mesh option lies on the sphere; zero is refused.
    #[test]
    fn the_sphere_has_its_poles_on_the_normal() {
        let front = plane(XZ.0, XZ.1);
        let answers = [Answer::Point(p(1.0, 2.0, 3.0)), Answer::Number(10.0)];
        let Geometry::BRep(brep) = build(&SHAPE, &front, &answers, "Brep").unwrap() else {
            panic!()
        };
        assert_eq!(brep.face_count(), 1);
        assert!(brep.is_solid());
        let points = brep.vertex_points();
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(1.0, -8.0, 3.0), None) < 1e-9)
        );
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(1.0, 12.0, 3.0), None) < 1e-9)
        );
        let mesh = build(&SHAPE, &front, &answers, "Mesh").unwrap();
        assert!(matches!(mesh, Geometry::Mesh(_)));

        for q in corners(&mesh) {
            assert!((q.distance(&p(1.0, 2.0, 3.0), None) - 10.0).abs() < 0.1);
        }

        let zero = [Answer::Point(p(0.0, 0.0, 0.0)), Answer::Number(0.0)];
        assert!(build(&SHAPE, &front, &zero, "Brep").is_err());
    }
}
