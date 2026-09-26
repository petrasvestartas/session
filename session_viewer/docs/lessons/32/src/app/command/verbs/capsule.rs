use crate::app::command::tool::shape::{self, MESH_ONLY, Part, Shape};
use crate::app::command::verbs::cylinder;
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Line, Point, Primitives};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Capsule"],
    aliases: &[],
    hint: "Capsule (Mesh): base center, radius, total height · Example: Capsule 0,0,0 5 30",
    options: &["Capsule Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Capsule",
    options: MESH_ONLY,
    upfront: false,
    ask: cylinder::ask,
    read,
    outline,
    build,
};

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// The cylinder's radius and height, taller than the two caps.
fn read(frame: &shape::Frame, answers: &[shape::Answer], option: &str) -> Result<Part, String> {
    let part = cylinder::read(frame, answers, option)?;

    if let [radius, height] = part.sizes[..]
        && height.abs() <= 2.0 * radius
    {
        return Err("The height must exceed the diameter".into());
    }

    Ok(part)
}

/// The base ring, then the rings where the caps begin and four sides.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let [radius, height] = part.sizes[..] else {
        return part
            .sizes
            .first()
            .map_or_else(Vec::new, |radius| vec![part.frame.ring(*radius, 0.0)]);
    };
    let (frame, height) = part.frame.upright(height);
    let (low, high) = (radius, height - radius);
    let mut wires = vec![frame.ring(radius, low), frame.ring(radius, high)];

    for (u, v) in [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)] {
        wires.push(vec![
            frame.at(0.0, 0.0, 0.0),
            frame.at(u * radius, v * radius, low),
            frame.at(u * radius, v * radius, high),
            frame.at(0.0, 0.0, height),
        ]);
    }

    wires
}

/// The kernel capsule on the axis, its caps ending at the base and the height.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Capsule needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    let axis = Line::from_points(
        &Point::new(0.0, 0.0, radius),
        &Point::new(0.0, 0.0, height - radius),
    );
    let mut mesh = Primitives::capsule_mesh(&axis, radius);
    mesh.transform(&frame.to_xform());
    mesh.name = "capsule".into();
    Ok(Geometry::Mesh(Rc::new(mesh)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::Answer;
    use crate::app::command::tool::shape::tests::{XY, bounds, build, corners, p, plane};

    /// The capsule spans exactly the base to the height; a height within the diameter is refused.
    #[test]
    fn the_capsule_spans_the_height() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(600.0),
        ];
        let made = build(&SHAPE, &top, &answers, "Mesh").unwrap();
        let [low, high] = bounds(&corners(&made));
        assert!(low[2].abs() < 1e-9 && (high[2] - 600.0).abs() < 1e-9);
        let Geometry::Mesh(mesh) = made else { panic!() };
        assert!(mesh.is_closed());
        let short = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(200.0),
        ];
        assert!(build(&SHAPE, &top, &short, "Mesh").is_err());
    }
}
