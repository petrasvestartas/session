use crate::app::command::tool::shape::{self, MESH_ONLY, Part, Shape};
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Primitives};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Quad Sphere"],
    aliases: &[],
    hint: "Quad Sphere (Mesh): center, radius · six patches of 8 × 8 quads · Example: Quad Sphere 0,0,0 10",
    options: &["Quad Sphere Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Quad Sphere",
    options: MESH_ONLY,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

const QUADS: usize = 8; // per patch side

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// Six kernel patches meshed and welded into one closed mesh.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Quad Sphere needs a radius".into());
    };
    let patches: Vec<_> = Primitives::quad_sphere(0.0, 0.0, 0.0, radius)
        .iter()
        .map(|patch| Primitives::quad_mesh(patch, QUADS, QUADS))
        .collect();
    let mut mesh = shape::to_mesh(&patches);
    mesh.transform(&part.frame.to_xform());
    mesh.name = "quad sphere".into();
    Ok(Geometry::Mesh(Rc::new(mesh)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::Answer;
    use crate::app::command::tool::shape::tests::{XY, build, corners, p, plane};

    /// 384 quads, closed, every corner at the radius within the patches' accuracy; Brep is not an option.
    #[test]
    fn the_quad_sphere_is_closed_and_round() {
        let center = p(5.0, 0.0, 0.0);
        let answers = [Answer::Point(center.clone()), Answer::Number(300.0)];
        let made = build(&SHAPE, &plane(XY.0, XY.1), &answers, "Mesh").unwrap();
        let Geometry::Mesh(mesh) = &made else {
            panic!()
        };
        assert_eq!(mesh.number_of_faces(), 384);
        assert!(mesh.is_closed());

        for q in corners(&made) {
            assert!((q.distance(&center, None) - 300.0).abs() < 2e-4 * 300.0); // the kernel patches are within 1e-4 of the sphere
        }

        assert!(shape::start(&SHAPE, &["Brep"]).is_err());
    }
}
