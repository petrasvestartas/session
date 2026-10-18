use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use session_rust::{BRep, Geometry, Mesh, Point, Xform};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Box",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Base center, a corner or the length, the width after a typed length, then the height.
pub fn ask(answers: &[Answer], part: &Part, _option: &str) -> Option<Ask> {
    match (answers.len(), part.sizes.as_slice()) {
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
    let mut rest = answers.iter().skip(1);

    match rest.next() {
        Some(Answer::Point(corner)) => {
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

