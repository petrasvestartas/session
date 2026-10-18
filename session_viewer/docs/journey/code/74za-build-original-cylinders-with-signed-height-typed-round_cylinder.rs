use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Cylinder",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

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

