use crate::app::command::tool::shape::{self, Answer, Ask, CURVE, Frame, Part, Shape, positive};
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Point, Primitives};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Nurbs Curve Parabola"],
    aliases: &[],
    hint: "Nurbs Curve Parabola: start, end or length, apex or height · Example: Nurbs Curve Parabola 0,0,0 100 40",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Nurbs Curve Parabola",
    options: CURVE,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

const SAMPLES: usize = 32; // preview segments

/// Start the questions, answered by any typed words.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    shape::start(&SHAPE, rest)
}

/// Start, end, then the apex.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Start")),
        1 => Some(Ask::size("End or length")),
        2 => Some(Ask::size("Apex or height")),
        _ => None,
    }
}

/// The frame runs from start to end; the apex is where the curve passes half way, off the chord.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());
    let Some(end) = answers.get(1) else {
        return Ok(part);
    };

    if let Answer::Point(end) = end {
        let [u, v, _] = frame.local(end);
        part.frame = frame.turned(u, v);
    }

    let length = positive(frame.size(end), "The length")?; // a click lands on the plane
    part.sizes.push(length);

    if let Some(apex) = answers.get(2) {
        let [u, v, w] = match apex {
            Answer::Number(height) => [length * 0.5, *height, 0.0], // to the left of start → end
            Answer::Point(p) => part.frame.local(p),
        };

        if v.hypot(w) <= 1e-9 * length {
            return Err("The apex must lie off the line from start to end".into());
        }

        part.sizes.extend([u, w, v]); // the height last: the cursor readout shows it
    }

    Ok(part)
}

/// Start, apex and end in the world.
fn points(part: &Part) -> Option<[Point; 3]> {
    let [length, u, w, v] = part.sizes[..] else {
        return None;
    };
    let frame = &part.frame;
    Some([
        frame.at(0.0, 0.0, 0.0),
        frame.at(u, v, w),
        frame.at(length, 0.0, 0.0),
    ])
}

/// The chord, then the parabola.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let Some([start, apex, end]) = points(part) else {
        return part.sizes.first().map_or_else(Vec::new, |length| {
            vec![vec![
                part.frame.at(0.0, 0.0, 0.0),
                part.frame.at(*length, 0.0, 0.0),
            ]]
        });
    };
    // the Bezier control that makes the curve pass the apex half way
    let control = [0, 1, 2].map(|i| 2.0 * apex[i] - 0.5 * (start[i] + end[i]));
    let curve = (0..=SAMPLES)
        .map(|i| {
            let t = i as f64 / SAMPLES as f64;
            let (a, b, c) = ((1.0 - t) * (1.0 - t), 2.0 * t * (1.0 - t), t * t);
            Point::new(
                a * start[0] + b * control[0] + c * end[0],
                a * start[1] + b * control[1] + c * end[1],
                a * start[2] + b * control[2] + c * end[2],
            )
        })
        .collect();
    vec![curve]
}

/// The kernel parabola through start, apex and end.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [start, apex, end] =
        points(part).ok_or("Nurbs Curve Parabola needs a start, an end and an apex")?;
    let mut curve = Primitives::parabola(&start, &apex, &end);
    curve.name = "parabola".into();
    Ok(Geometry::NurbsCurve(Rc::new(curve)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, build, p, plane};

    /// It passes start, apex half way and end; an apex on the chord is refused.
    #[test]
    fn the_parabola_passes_its_three_points() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(1000.0),
            Answer::Number(400.0),
        ];
        let Geometry::NurbsCurve(curve) = build(&SHAPE, &top, &answers, "").unwrap() else {
            panic!()
        };
        let (a, b) = curve.domain();
        assert_eq!(curve.cv_count(), 3);
        assert!(curve.point_at(a).distance(&p(0.0, 0.0, 0.0), None) < 1e-9);
        assert!(
            curve
                .point_at(0.5 * (a + b))
                .distance(&p(500.0, 400.0, 0.0), None)
                < 1e-9
        );
        assert!(curve.point_at(b).distance(&p(1000.0, 0.0, 0.0), None) < 1e-9);
        let flat = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Point(p(10.0, 0.0, 0.0)),
            Answer::Point(p(5.0, 0.0, 0.0)),
        ];
        assert!(build(&SHAPE, &top, &flat, "").is_err());
    }
}
