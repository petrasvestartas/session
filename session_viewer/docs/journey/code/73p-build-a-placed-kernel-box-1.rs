use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use session_rust::{BRep, Geometry, Mesh, Point, Xform};
use std::rc::Rc;

/// A number, or twice a click's distance across the length.
pub fn width_of(frame: &Frame, answer: &Answer) -> f64 {
    match answer {
        Answer::Number(width) => *width,
        Answer::Point(p) => 2.0 * frame.local(p)[1].abs(),
    }
}

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

