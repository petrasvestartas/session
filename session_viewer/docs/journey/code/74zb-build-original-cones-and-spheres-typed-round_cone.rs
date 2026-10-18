use crate::primitive_shape as shape;
use crate::primitive_shape::{ BREP_MESH, Part, Shape};
use crate::round_cylinder as cylinder;
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Cone",
    options: BREP_MESH,
    upfront: false,
    ask: cylinder::ask,
    read: cylinder::read,
    outline,
    build,
};

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

