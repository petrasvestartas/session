use crate::app::command::tool::shape::{self, Answer, Ask, Frame, Part, RING, Shape, positive};
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, NurbsCurve, Point};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Nurbs Curve Arc"],
    aliases: &[],
    hint: "Nurbs Curve Arc (Center 3 Points): center, start or radius, end or angle · Example: Nurbs Curve Arc 0,0,0 10 90",
    options: &["Nurbs Curve Arc Center", "Nurbs Curve Arc 3 Points"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Arc",
    options: &[
        ("Center", "Center"),
        ("3 Points", "3Points"),
        ("Cancel", "Escape"),
    ],
    upfront: true,
    ask,
    read,
    outline,
    build,
};

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// Center, start, end; or start, end, a point on the arc.
fn ask(answers: &[Answer], _part: &Part, option: &str) -> Option<Ask> {
    match (option, answers.len()) {
        ("Center", 0) => Some(Ask::point("Center")),
        ("Center", 1) => Some(Ask::size("Start or radius")),
        ("Center", 2) => Some(Ask::size("End or angle in degrees")),
        (_, 0) => Some(Ask::point("Start")),
        (_, 1) => Some(Ask::size("End or chord length")),
        (_, 2) => Some(Ask::size("Point on the arc or bulge")),
        _ => None,
    }
}

/// The circle's frame, x toward the start, and the radius and angle in degrees counterclockwise.
fn read(frame: &Frame, answers: &[Answer], option: &str) -> Result<Part, String> {
    match option {
        "Center" => read_center(frame, answers),
        _ => read_points(frame, answers),
    }
}

/// A click's angle about the center, or typed degrees; negative runs clockwise.
fn read_center(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(start) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(start) = start {
        let [u, v, _] = frame.local(start);
        part.frame = frame.turned(u, v);
    }

    part.sizes.push(positive(frame.size(start), "The radius")?);
    let Some(end) = answers.get(2) else {
        return Ok(part);
    };
    let degrees = match end {
        Answer::Number(degrees) => *degrees,
        Answer::Point(end) => {
            let [u, v, _] = part.frame.local(end);
            let degrees = v.atan2(u).to_degrees();
            if degrees < 0.0 {
                degrees + 360.0
            } else {
                degrees
            }
        }
    };

    if !(degrees.abs() > 1e-9 && degrees.abs() < 360.0) {
        return Err(format!(
            "The angle must be above 0 and below 360 degrees, not {degrees}"
        ));
    }

    if degrees < 0.0 {
        part.frame = part.frame.flipped();
    }

    part.sizes.push(degrees.abs());
    Ok(part)
}

/// The circle through start, a point on it and end, running from start past that point.
fn read_points(frame: &Frame, answers: &[Answer]) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(end) = answers.get(1) else {
        return Ok(part);
    };
    let start = frame.origin.clone();
    let end = match end {
        Answer::Number(length) => frame.at(*length, 0.0, 0.0),
        Answer::Point(end) => end.clone(),
    };
    let chord = positive(start.distance(&end, None), "The chord")?;
    let [u, v, _] = frame.local(&end);
    part.frame = frame.turned(u, v);
    part.sizes.push(chord);
    let Some(middle) = answers.get(2) else {
        return Ok(part);
    };
    let middle = match middle {
        Answer::Number(bulge) => part.frame.at(chord * 0.5, *bulge, 0.0), // to the left of start → end
        Answer::Point(p) => p.clone(),
    };
    let (ab, ac) = (&end - &start, &middle - &start);
    let normal = ac.cross(&ab); // start, middle, end run counterclockwise about it

    if normal.magnitude() <= 1e-9 * ab.magnitude() * ac.magnitude() {
        return Err("The three points are in a line".into());
    }

    let offset = &(&normal.cross(&ab) * ac.dot(&ac)) + &(&ac.cross(&normal) * ab.dot(&ab));
    let center = &start - (&offset * (0.5 / normal.dot(&normal)));
    let radius = center.distance(&start, None);
    let x = (&start - &center).normalized();
    let z = normal.normalized();
    let circle = Frame {
        origin: center,
        y: z.cross(&x),
        x,
        z,
    };
    let [u, v, _] = circle.local(&end);
    let degrees = v.atan2(u).to_degrees();
    part.frame = circle;
    part.sizes = vec![
        radius,
        if degrees <= 0.0 {
            degrees + 360.0
        } else {
            degrees
        },
    ];
    Ok(part)
}

/// The radius or chord as a line, then the arc.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [length] => vec![vec![frame.at(0.0, 0.0, 0.0), frame.at(length, 0.0, 0.0)]],
        [radius, degrees] => {
            let count = ((RING as f64 * degrees / 360.0).ceil() as usize).max(2);
            let arc = (0..=count)
                .map(|i| {
                    let angle = (degrees * i as f64 / count as f64).to_radians();
                    frame.at(radius * angle.cos(), radius * angle.sin(), 0.0)
                })
                .collect();
            vec![arc]
        }
        _ => Vec::new(),
    }
}

/// The exact arc, placed in its circle's frame.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius, degrees] = part.sizes[..] else {
        return Err("Nurbs Curve Arc needs a radius and an angle".into());
    };
    let mut curve = arc(radius, degrees);
    curve.transform(&part.frame.to_xform());
    curve.name = "arc".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}

/// A circular arc about z from angle 0 to `degrees`, one rational quadratic span per 90° or less.
pub fn arc(radius: f64, degrees: f64) -> NurbsCurve {
    let spans = ((degrees / 90.0 - 1e-9).ceil() as usize).max(1);
    let step = degrees.to_radians() / spans as f64;
    let weight = (step * 0.5).cos(); // of each span's middle control
    let mut curve = NurbsCurve::new(3, true, 3, 2 * spans + 1);

    for i in 0..2 * spans + 2 {
        curve.set_nurbsknot(i, (i / 2) as f64);
    }

    for i in 0..=2 * spans {
        let angle = step * 0.5 * i as f64;
        let w = if i % 2 == 1 { weight } else { 1.0 };
        let distance = radius / w; // the middle control lies outside the circle
        curve.set_cv_4d(
            i,
            distance * angle.cos() * w,
            distance * angle.sin() * w,
            0.0,
            w,
        );
    }

    curve
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, build, corners, p, plane};

    /// Every sample of a curve lies at `radius` from `center`.
    fn round(curve: &Geometry, center: &Point, radius: f64) -> bool {
        corners(curve)
            .iter()
            .all(|q| (q.distance(center, None) - radius).abs() <= 1e-12 * radius.max(1.0) * 10.0)
    }

    /// 90° is one span and 270° three; every sample is on the circle, which the kernel arc misses.
    #[test]
    fn the_arc_is_exactly_round() {
        let top = plane(XY.0, XY.1);
        let quarter = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(500.0),
            Answer::Number(90.0),
        ];
        let made = build(&SHAPE, &top, &quarter, "Center").unwrap();
        let Geometry::NurbsCurve(curve) = &made else {
            panic!()
        };
        assert_eq!(curve.cv_count(), 3);
        assert!(round(&made, &p(0.0, 0.0, 0.0), 500.0));
        assert!(
            curve
                .point_at(curve.domain().1)
                .distance(&p(0.0, 500.0, 0.0), None)
                < 1e-9
        );
        let three = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(500.0),
            Answer::Number(270.0),
        ];
        let made = build(&SHAPE, &top, &three, "Center").unwrap();
        let Geometry::NurbsCurve(curve) = &made else {
            panic!()
        };
        assert_eq!(curve.cv_count(), 7);
        assert!(round(&made, &p(0.0, 0.0, 0.0), 500.0));
        let clockwise = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(0.0, 10.0, 0.0)),
            Answer::Number(-90.0),
        ];
        let Geometry::NurbsCurve(curve) = build(&SHAPE, &top, &clockwise, "Center").unwrap() else {
            panic!()
        };
        assert!(
            curve
                .point_at(curve.domain().1)
                .distance(&p(10.0, 0.0, 0.0), None)
                < 1e-9
        );

        for degrees in [0.0, 360.0, -360.0] {
            let bad = [
                Answer::Point(p(0.0, 0.0, 0.0)),
                Answer::Number(5.0),
                Answer::Number(degrees),
            ];
            let error = build(&SHAPE, &top, &bad, "Center").unwrap_err();
            assert!(error.ends_with(&format!("not {degrees}")), "{error}");
        }
    }

    /// Three points: a semicircle through the third point; points in a line are refused.
    #[test]
    fn three_points_pass_the_middle_point() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(10.0, 0.0, 0.0)),
            Answer::Point(p(5.0, 5.0, 0.0)),
        ];
        let made = build(&SHAPE, &top, &answers, "3 Points").unwrap();
        assert!(round(&made, &p(5.0, 0.0, 0.0), 5.0));
        let Geometry::NurbsCurve(curve) = &made else {
            panic!()
        };
        let (a, b) = curve.domain();
        assert!(curve.point_at(a).distance(&p(0.0, 0.0, 0.0), None) < 1e-9);
        assert!(curve.point_at(b).distance(&p(10.0, 0.0, 0.0), None) < 1e-9);
        assert!(
            corners(&made)
                .iter()
                .any(|q| q.distance(&p(5.0, 5.0, 0.0), None) < 0.5)
        );
        assert!(corners(&made).iter().all(|q| q[1] >= -1e-9));
        let bulge = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(1000.0),
            Answer::Number(500.0),
        ];
        assert!(round(
            &build(&SHAPE, &top, &bulge, "3 Points").unwrap(),
            &p(500.0, 0.0, 0.0),
            500.0
        ));
        let below = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(10.0, 0.0, 0.0)),
            Answer::Point(p(5.0, -2.0, 0.0)),
        ];
        assert!(
            corners(&build(&SHAPE, &top, &below, "3 Points").unwrap())
                .iter()
                .all(|q| q[1] <= 1e-9)
        );
        let line = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(10.0, 0.0, 0.0)),
            Answer::Point(p(20.0, 0.0, 0.0)),
        ];
        assert!(build(&SHAPE, &top, &line, "3 Points").is_err());
    }
}
