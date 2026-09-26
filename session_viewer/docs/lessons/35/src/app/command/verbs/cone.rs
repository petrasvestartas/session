use crate::app::command::tool::shape::{self, BREP_MESH, Part, Shape};
use crate::app::command::verbs::cylinder;
use crate::app::command::{Action, Spec};
use session_rust::{BRep, Geometry, Point};

pub const SPEC: Spec = Spec {
    names: &["Cone"],
    aliases: &[],
    hint: "Cone (Brep Mesh): base center, radius, height to the apex · Example: Cone 0,0,0 5 20",
    options: &["Cone Brep", "Cone Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Cone",
    options: BREP_MESH,
    upfront: false,
    ask: cylinder::ask,
    read: cylinder::read,
    outline,
    build,
};

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// The base ring, then four lines to the apex.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [radius] => vec![frame.ring(radius, 0.0)],
        [radius, height] => {
            let apex = frame.at(0.0, 0.0, height);
            let mut wires = vec![frame.ring(radius, 0.0)];

            for (u, v) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
                wires.push(vec![frame.at(u * radius, v * radius, 0.0), apex.clone()]);
            }

            wires
        }
        _ => Vec::new(),
    }
}

/// A kernel cone on the base, apex along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Cone needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    Ok(shape::solid(
        BRep::create_cone(radius, height),
        option,
        &frame.to_xform(),
        "cone",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::Answer;
    use crate::app::command::tool::shape::tests::{XZ, build, p, plane};

    /// Two faces, the apex on the normal at the height.
    #[test]
    fn the_apex_is_on_the_normal() {
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(500.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 2);
        assert!(brep.is_solid());
        assert!(
            brep.vertex_points()
                .iter()
                .any(|q| q.distance(&p(0.0, -500.0, 0.0), None) < 1e-9)
        );
    }
}
