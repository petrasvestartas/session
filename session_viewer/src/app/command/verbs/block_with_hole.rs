use crate::app::command::tool::shape::{
    self, Answer, Ask, BREP_MESH, Frame, Part, Shape, positive,
};
use crate::app::command::verbs::r#box;
use crate::app::command::{Action, Spec};
use session_rust::{BRep, Geometry, Point, Xform};

pub const SPEC: Spec = Spec {
    names: &["Block With Hole"],
    aliases: &[],
    hint: "Block With Hole (Brep Mesh): base center, corner or length and width, height, hole radius · Example: Block With Hole 0,0,0 40 30 20 5",
    options: &["Block With Hole Brep", "Block With Hole Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Block With Hole",
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

/// The box questions, then the hole radius.
fn ask(answers: &[Answer], part: &Part, option: &str) -> Option<Ask> {
    match part.sizes.len() {
        3 => Some(Ask::size("Hole radius")),
        4 => None,
        _ => r#box::ask(answers, part, option),
    }
}

/// The box sizes, then a hole radius below half the shorter side.
fn read(frame: &Frame, answers: &[Answer], option: &str) -> Result<Part, String> {
    let sides = match answers.get(1) {
        Some(Answer::Point(_)) => 3, // a corner gives length and width at once
        _ => 4,
    };
    let mut part = r#box::read(frame, &answers[..answers.len().min(sides)], option)?;

    if let Some(hole) = answers.get(sides) {
        let hole = positive(frame.size(hole), "The hole radius")?;

        if hole >= 0.5 * part.sizes[0].min(part.sizes[1]) {
            return Err("The hole radius must be below half the shorter side".into());
        }

        part.sizes.push(hole);
    }

    Ok(part)
}

/// The box, then the hole's rings.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let mut wires = r#box::outline(part);

    if let [_, _, height, hole] = part.sizes[..] {
        wires.push(part.frame.ring(hole, 0.0));
        wires.push(part.frame.ring(hole, height));
    }

    wires
}

/// A kernel block centered on the axis, standing on the plane, the hole along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [length, width, height, hole] = part.sizes[..] else {
        return Err("Block With Hole needs its sides and a hole radius".into());
    };
    let place = &part.frame.to_xform() * &Xform::translation(0.0, 0.0, height * 0.5);
    let brep = BRep::create_block_with_hole(length, width, height.abs(), hole);
    Ok(shape::solid(brep, option, &place, "block with hole"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, bounds, build, corners, near, p, plane};

    /// Seven faces standing on the plane; a hole as wide as the block is refused.
    #[test]
    fn the_block_stands_on_the_plane() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(600.0),
            Answer::Number(400.0),
            Answer::Number(200.0),
            Answer::Number(100.0),
        ];
        let made = build(&SHAPE, &top, &answers, "Brep").unwrap();
        let Geometry::BRep(brep) = &made else {
            panic!()
        };
        assert_eq!(brep.face_count(), 7);
        assert!(brep.is_solid());
        assert!(near(
            bounds(&corners(&made)),
            [[-300.0, -200.0, 0.0], [300.0, 200.0, 200.0]]
        ));
        let corner = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(300.0, 200.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(100.0),
        ];
        assert!(build(&SHAPE, &top, &corner, "Mesh").is_ok());
        let mut wide = answers.clone();
        wide[4] = Answer::Number(200.0);
        assert!(build(&SHAPE, &top, &wide, "Brep").is_err());
    }
}
