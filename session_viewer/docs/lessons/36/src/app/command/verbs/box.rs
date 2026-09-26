// --8<-- [start:box-spec]
// `self` in the list also imports the module itself, so `shape::start` and `shape::solid` read as calls into it.
use crate::app::command::tool::shape::{
    self, Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use crate::app::command::{Action, Spec};
use session_rust::{BRep, Geometry, Mesh, Point, Xform};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Box"],
    aliases: &[],
    hint: "Box (Brep Mesh): base center, corner or length and width, height · Example: Box 0,0,0 100 50 30",
    options: &["Box Brep", "Box Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

// `ask,` is short for `ask: ask`: each field names the function of the same name below.
pub static SHAPE: Shape = Shape {
    name: "Box",
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
// --8<-- [end:box-spec]

// --8<-- [start:box-questions]
/// Base center, a corner or the length, the width after a typed length, then the height.
pub fn ask(answers: &[Answer], part: &Part, _option: &str) -> Option<Ask> {
    match (answers.len(), part.sizes.as_slice()) {
        // The count of sizes read so far says which question comes next: one size means the length was typed.
        (0, _) => Some(Ask::point("Base center")),
        (1, _) => Some(Ask::size("Corner or length")),
        (_, &[length]) => Some(Ask::size("Width").or(length)),
        (_, &[_, width]) => Some(Ask::height("Height").or(width)),
        _ => None,
    }
}

/// Length, width and signed height; a corner click gives length and width at once.
pub fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    // The first answer is the base center, which the frame already holds.
    let mut rest = answers.iter().skip(1);

    match rest.next() {
        Some(Answer::Point(corner)) => {
            // The base center sits in the middle, so each size is twice the corner's offset from it.
            let [u, v, _] = frame.local(corner);
            let message = "The corner must lie off both axes of the base center";
            part.sizes.push(positive(2.0 * u.abs(), message)?);
            part.sizes.push(positive(2.0 * v.abs(), message)?);
        }
        Some(Answer::Number(length)) => {
            part.sizes.push(positive(*length, "The length")?);

            if let Some(width) = rest.next() {
                part.sizes
                    .push(positive(width_of(frame, width), "The width")?);
            }
        }
        None => return Ok(part),
    }

    if let Some(height) = rest.next() {
        part.sizes
            .push(nonzero(frame.height(height), "The height")?);
    }

    Ok(part)
}

/// A number, or twice a click's distance across the length.
pub fn width_of(frame: &Frame, answer: &Answer) -> f64 {
    match answer {
        Answer::Number(width) => *width,
        Answer::Point(p) => 2.0 * frame.local(p)[1].abs(),
    }
}
// --8<-- [end:box-questions]

// --8<-- [start:box-build]
/// The length as a line, the base, then the whole box.
pub fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [length] => vec![vec![
            frame.at(-length * 0.5, 0.0, 0.0),
            frame.at(length * 0.5, 0.0, 0.0),
        ]],
        [length, width] => vec![frame.rectangle(length, width, 0.0)],
        [length, width, height, ..] => frame.cuboid(length, width, height),
        _ => Vec::new(),
    }
}

/// A kernel box centered on the axis, standing on the plane.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [length, width, height] = part.sizes[..] else {
        return Err("Box needs a length, a width and a height".into());
    };
    // The kernel box is centred on the origin: lift it half its height, then the frame puts it on the plane.
    // A negative height lifts by a negative half, so the box hangs below the plane.
    let place = &part.frame.to_xform() * &Xform::translation(0.0, 0.0, height * 0.5);

    if option == "Mesh" {
        let mut mesh = Mesh::create_box(length, width, height.abs());
        mesh.transform(&place);
        mesh.name = "box".into();
        return Ok(Geometry::Mesh(Rc::new(mesh)));
    }

    Ok(shape::solid(
        BRep::create_box(length, width, height.abs()),
        option,
        &place,
        "box",
    ))
}
// --8<-- [end:box-build]

// --8<-- [start:box-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, XZ, bounds, build, corners, near, p, plane};

    /// Typed sizes and a corner click give the same box; a negative height goes below the plane.
    #[test]
    fn typed_sizes_and_a_corner_give_the_box() {
        let top = plane(XY.0, XY.1);
        let typed = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(50.0),
            Answer::Number(30.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &top, &typed, "Brep").unwrap() else {
            panic!()
        };
        assert_eq!(brep.face_count(), 6);
        assert!(brep.is_solid());
        assert!(near(
            bounds(&brep.vertex_points()),
            [[-50.0, -25.0, 0.0], [50.0, 25.0, 30.0]]
        ));
        let corner = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(50.0, 25.0, 0.0)),
            Answer::Number(30.0),
        ];
        let clicked = build(&SHAPE, &top, &corner, "Brep").unwrap();
        assert!(near(
            bounds(&corners(&clicked)),
            [[-50.0, -25.0, 0.0], [50.0, 25.0, 30.0]]
        ));
        let below = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(50.0),
            Answer::Number(-30.0),
        ];
        let low = build(&SHAPE, &top, &below, "Brep").unwrap();
        assert!(near(
            bounds(&corners(&low)),
            [[-50.0, -25.0, -30.0], [50.0, 25.0, 0.0]]
        ));
        let Geometry::Mesh(mesh) = build(&SHAPE, &top, &typed, "Mesh").unwrap() else {
            panic!()
        };
        assert_eq!(mesh.number_of_faces(), 6);
        assert!(mesh.is_closed());
    }

    /// On the Front plane the height runs along −Y.
    #[test]
    fn the_front_plane_stands_the_box_along_minus_y() {
        let front = plane(XZ.0, XZ.1);
        let typed = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(50.0),
            Answer::Number(30.0),
        ];
        let made = build(&SHAPE, &front, &typed, "Brep").unwrap();
        assert!(near(
            bounds(&corners(&made)),
            [[-50.0, -30.0, -25.0], [50.0, 0.0, 25.0]]
        ));
    }

    /// Zero sizes and a corner on an axis are refused.
    #[test]
    fn zero_sizes_are_refused() {
        let frame = Frame::new(&plane(XY.0, XY.1), p(0.0, 0.0, 0.0));
        let center = Answer::Point(p(0.0, 0.0, 0.0));
        assert!(read(&frame, &[center.clone(), Answer::Number(0.0)], "Brep").is_err());
        assert!(
            read(
                &frame,
                &[center.clone(), Answer::Point(p(10.0, 0.0, 0.0))],
                "Brep"
            )
            .is_err()
        );
        assert!(
            read(
                &frame,
                &[
                    center,
                    Answer::Number(10.0),
                    Answer::Number(5.0),
                    Answer::Number(0.0)
                ],
                "Brep"
            )
            .is_err()
        );
    }
}
// --8<-- [end:box-tests]
