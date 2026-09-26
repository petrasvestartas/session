use crate::app::command::tool::shape::{
    self, Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use crate::app::command::{Action, Spec};
use session_rust::{BRep, Geometry, Point};
use std::f64::consts::FRAC_1_SQRT_2;

pub const SPEC: Spec = Spec {
    names: &["Pyramid"],
    aliases: &[],
    hint: "Pyramid (Brep Mesh): base center, corner or edge length, height · Example: Pyramid 0,0,0 10 15",
    options: &["Pyramid Brep", "Pyramid Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Pyramid",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, build, p, plane};

    /// Five faces; a clicked corner lands a base corner on the click; the apex tops it.
    #[test]
    fn a_clicked_corner_lands_on_the_click() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(10.0, 0.0, 0.0)),
            Answer::Point(p(13.0, 4.0, 0.0)),
            Answer::Number(15.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &top, &answers, "Brep").unwrap() else {
            panic!()
        };
        assert_eq!(brep.face_count(), 5);
        assert!(brep.is_solid());
        let points = brep.vertex_points();
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(13.0, 4.0, 0.0), None) < 1e-9)
        );
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(10.0, 0.0, 15.0), None) < 1e-9)
        );
        let typed = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(10.0),
            Answer::Number(15.0),
        ];
        let Geometry::BRep(square) = build(&SHAPE, &top, &typed, "Brep").unwrap() else {
            panic!()
        };
        assert!(
            square
                .vertex_points()
                .iter()
                .any(|q| q.distance(&p(5.0, 5.0, 0.0), None) < 1e-9)
        );
    }
}
