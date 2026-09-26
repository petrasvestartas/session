use crate::app::command::tool::shape::{
    self, Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use crate::app::command::{Action, Spec};
use session_rust::{BRep, Geometry, Point};

pub const SPEC: Spec = Spec {
    names: &["Cylinder"],
    aliases: &[],
    hint: "Cylinder (Brep Mesh): base center, radius, height · Example: Cylinder 0,0,0 5 20",
    options: &["Cylinder Brep", "Cylinder Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Cylinder",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// Base center, radius, height: the questions of a cylinder or cone.
pub fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Base center")),
        1 => Some(Ask::size("Radius")),
        2 => Some(Ask::height("Height")),
        _ => None,
    }
}

/// Radius in the plane and signed height.
pub fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        part.sizes.push(positive(frame.size(radius), "The radius")?);
    }

    if let Some(height) = answers.get(2) {
        part.sizes
            .push(nonzero(frame.height(height), "The height")?);
    }

    Ok(part)
}

/// The base ring, then the top ring and four sides.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [radius] => vec![frame.ring(radius, 0.0)],
        [radius, height] => {
            let mut wires = vec![frame.ring(radius, 0.0), frame.ring(radius, height)];

            for (u, v) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
                wires.push(vec![
                    frame.at(u * radius, v * radius, 0.0),
                    frame.at(u * radius, v * radius, height),
                ]);
            }

            wires
        }
        _ => Vec::new(),
    }
}

/// A kernel cylinder on the base, upside down for a negative height.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Cylinder needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    let brep = BRep::create_cylinder(radius, height);
    Ok(shape::solid(brep, option, &frame.to_xform(), "cylinder"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, XZ, bounds, build, corners, near, p, plane};

    /// Three faces, 2r × 2r × h; a negative height goes below; the Front plane stands it along −Y.
    #[test]
    fn the_cylinder_stands_on_its_base() {
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(600.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &plane(XY.0, XY.1), &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 3);
        assert!(brep.is_solid());
        let mesh = build(&SHAPE, &plane(XY.0, XY.1), &answers, "Mesh").unwrap();
        let [low, high] = bounds(&corners(&mesh));
        assert!((high[2] - 600.0).abs() < 1e-9 && low[2].abs() < 1e-9);
        assert!((high[0] - 200.0).abs() < 1e-6 && (low[0] + 200.0).abs() < 1e-6);
        let down = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(-600.0),
        ];
        let low = build(&SHAPE, &plane(XY.0, XY.1), &down, "Mesh").unwrap();
        let [bottom, top] = bounds(&corners(&low));
        assert!((bottom[2] + 600.0).abs() < 1e-9 && top[2].abs() < 1e-9);
        let front = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Mesh").unwrap();
        let [a, b] = bounds(&corners(&front));
        assert!(near(
            [[0.0, a[1], 0.0], [0.0, b[1], 0.0]],
            [[0.0, -600.0, 0.0], [0.0, 0.0, 0.0]]
        ));
        let zero = [Answer::Point(p(0.0, 0.0, 0.0)), Answer::Number(-1.0)];
        assert!(build(&SHAPE, &plane(XY.0, XY.1), &zero, "Brep").is_err());
    }
}
