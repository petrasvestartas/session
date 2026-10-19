use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use session_rust::{BRep, Geometry, Point};
use std::f64::consts::FRAC_1_SQRT_2;

pub static SHAPE: Shape = Shape {
    name: "Pyramid",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Base center, a corner or the edge length, then the height.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Base center")),
        1 => Some(Ask::size("Corner or edge length")),
        2 => Some(Ask::height("Height")),
        _ => None,
    }
}

/// Edge and signed height; a corner click turns the base so a corner lands on it.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    match answers.get(1) {
        Some(Answer::Point(corner)) => {
            let [u, v, _] = frame.local(corner);
            let (cos, sin) = (FRAC_1_SQRT_2, FRAC_1_SQRT_2); // the kernel's corner sits at 45°
            part.frame = frame.turned(u * cos + v * sin, v * cos - u * sin);
            part.sizes
                .push(positive(u.hypot(v) * 2.0_f64.sqrt(), "The edge")?);
        }
        Some(Answer::Number(edge)) => part.sizes.push(positive(*edge, "The edge")?),
        None => return Ok(part),
    }

    if let Some(height) = answers.get(2) {
        part.sizes
            .push(nonzero(frame.height(height), "The height")?);
    }

    Ok(part)
}

/// The base square, then four lines to the apex.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [edge] => vec![frame.rectangle(edge, edge, 0.0)],
        [edge, height] => {
            let apex = frame.at(0.0, 0.0, height);
            let mut wires = vec![frame.rectangle(edge, edge, 0.0)];

            for (u, v) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                wires.push(vec![
                    frame.at(u * edge * 0.5, v * edge * 0.5, 0.0),
                    apex.clone(),
                ]);
            }

            wires
        }
        _ => Vec::new(),
    }
}

/// A kernel pyramid on the base, apex along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [edge, height] = part.sizes[..] else {
        return Err("Pyramid needs an edge and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    Ok(shape::solid(
        BRep::create_pyramid(edge, height),
        option,
        &frame.to_xform(),
        "pyramid",
    ))
}

