use crate::app::command::tool::shape::{self, Answer, Ask, CURVE, Frame, Part, Shape, positive};
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Nurbs Curve Ellipse"],
    aliases: &[],
    hint: "Nurbs Curve Ellipse: center, end of the first axis or its radius, second radius · Example: Nurbs Curve Ellipse 0,0,0 20 10",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Ellipse",
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

/// Center, the first axis end, then the second radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("First axis end or radius")),
        2 => Some(Ask::size("Second radius")),
        _ => None,
    }
}

/// A click turns the first axis toward it; the second radius is a click's distance from that axis.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(first) = answers.get(1) {
        if let Answer::Point(end) = first {
            let [u, v, _] = frame.local(end);
            part.frame = frame.turned(u, v);
        }

        part.sizes
            .push(positive(frame.size(first), "The first radius")?);
    }

    if let Some(second) = answers.get(2) {
        let radius = match second {
            Answer::Number(value) => *value,
            Answer::Point(p) => part.frame.local(p)[1].abs(),
        };
        part.sizes.push(positive(radius, "The second radius")?);
    }

    Ok(part)
}

/// The first axis, then the ellipse.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [first] => vec![vec![frame.at(-first, 0.0, 0.0), frame.at(first, 0.0, 0.0)]],
        [first, second] => vec![frame.oval(first, second, 0.0)],
        _ => Vec::new(),
    }
}

/// The kernel ellipse, its first radius along the first axis.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [first, second] = part.sizes[..] else {
        return Err("Nurbs Curve Ellipse needs two radii".into());
    };
    let mut curve = Primitives::ellipse(0.0, 0.0, 0.0, first, second);
    curve.transform(&part.frame.to_xform());
    curve.name = "ellipse".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, build, corners, p, plane};

    /// It starts at the first axis end, and every sample fits (u/a)² + (v/b)² = 1.
    #[test]
    fn the_ellipse_follows_its_axes() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(0.0, 20.0, 0.0)),
            Answer::Number(10.0),
        ];
        let made = build(&SHAPE, &top, &answers, "").unwrap();
        let Geometry::NurbsCurve(curve) = &made else {
            panic!()
        };
        assert_eq!(curve.cv_count(), 9);
        assert!(
            curve
                .point_at(curve.domain().0)
                .distance(&p(0.0, 20.0, 0.0), None)
                < 1e-9
        );

        for q in corners(&made) {
            assert!(((q[1] / 20.0).powi(2) + (q[0] / 10.0).powi(2) - 1.0).abs() < 1e-9);
        }
    }
}
