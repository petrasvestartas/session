// --8<-- [start:circle-spec]
use crate::app::command::tool::shape::{self, Answer, Ask, CURVE, Frame, Part, Shape, positive};
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Nurbs Curve Circle"],
    aliases: &[],
    hint: "Nurbs Curve Circle: center, radius · Example: Nurbs Curve Circle 0,0,0 10",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Circle",
    // A curve has no Brep or Mesh choice: its only button is Cancel.
    options: CURVE,
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
// --8<-- [end:circle-spec]

// --8<-- [start:circle-questions]
/// Center, then radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Radius")),
        _ => None,
    }
}

/// The radius in the plane.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        // `size` measures a click in the plane only, so a snapped point off the plane still gives the radius seen along the normal.
        part.sizes.push(positive(frame.size(radius), "The radius")?);
    }

    Ok(part)
}

/// The circle.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    part.sizes
        .iter()
        .map(|radius| part.frame.ring(*radius, 0.0))
        .collect()
}

/// The kernel circle, placed in the plane.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Nurbs Curve Circle needs a radius".into());
    };
    // The kernel circle lies flat on world XY around the origin; the frame turns it onto the drawing plane.
    let mut curve = Primitives::circle(0.0, 0.0, 0.0, radius);
    curve.transform(&part.frame.to_xform());
    curve.name = "circle".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}
// --8<-- [end:circle-questions]

// --8<-- [start:circle-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XZ, build, corners, p, plane};

    /// Nine controls, closed, every sample at the radius in the plane.
    #[test]
    fn the_circle_lies_in_the_plane() {
        let center = p(1.0, 2.0, 3.0);
        let answers = [
            Answer::Point(center.clone()),
            Answer::Point(p(11.0, 2.0, 3.0)),
        ];
        let made = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "").unwrap();
        let Geometry::NurbsCurve(curve) = &made else {
            panic!()
        };
        assert_eq!(curve.cv_count(), 9);
        assert!(curve.is_closed());

        for q in corners(&made) {
            assert!((q.distance(&center, None) - 10.0).abs() < 1e-9);
            assert!((q[1] - 2.0).abs() < 1e-9);
        }
    }
}
// --8<-- [end:circle-tests]
