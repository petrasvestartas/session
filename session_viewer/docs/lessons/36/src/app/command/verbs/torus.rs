use crate::app::command::tool::shape::{
    self, Answer, Ask, BREP_MESH, Frame, Part, Shape, positive,
};
use crate::app::command::{Action, Spec};
use session_rust::{BRep, Geometry, Point};

pub const SPEC: Spec = Spec {
    names: &["Torus"],
    aliases: &[],
    hint: "Torus (Brep Mesh): center, major radius, minor radius · Example: Torus 0,0,0 20 5",
    options: &["Torus Brep", "Torus Mesh"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static SHAPE: Shape = Shape {
    name: "Torus",
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

/// Center, major radius, then minor radius.
fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Major radius")),
        2 => Some(Ask::size("Minor radius")),
        _ => None,
    }
}

/// The minor radius is a click's distance from the major circle, below the major radius.
fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(major) = answers.get(1) {
        part.sizes
            .push(positive(frame.size(major), "The major radius")?);
    }

    if let Some(minor) = answers.get(2) {
        let major = part.sizes[0];
        let minor = match minor {
            Answer::Number(value) => *value,
            Answer::Point(_) => (frame.size(minor) - major).abs(),
        };

        if minor >= major {
            return Err("The minor radius must be below the major radius".into());
        }

        part.sizes.push(positive(minor, "The minor radius")?);
    }

    Ok(part)
}

/// The major circle, then the inner, outer, upper and lower rings.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [major] => vec![frame.ring(major, 0.0)],
        [major, minor] => vec![
            frame.ring(major - minor, 0.0),
            frame.ring(major + minor, 0.0),
            frame.ring(major, minor),
            frame.ring(major, -minor),
        ],
        _ => Vec::new(),
    }
}

/// A kernel torus about the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [major, minor] = part.sizes[..] else {
        return Err("Torus needs two radii".into());
    };
    Ok(shape::solid(
        BRep::create_torus(major, minor),
        option,
        &part.frame.to_xform(),
        "torus",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::shape::tests::{XY, bounds, build, corners, p, plane};

    /// One face spanning 2(R + r) across and 2r high; a minor radius at or over the major is refused.
    #[test]
    fn the_torus_spans_its_radii() {
        let top = plane(XY.0, XY.1);
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(500.0),
            Answer::Number(100.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &top, &answers, "Brep").unwrap() else {
            panic!()
        };
        assert_eq!(brep.face_count(), 1);
        let [low, high] = bounds(&corners(&build(&SHAPE, &top, &answers, "Mesh").unwrap()));
        assert!((high[0] - 600.0).abs() < 1.0 && (low[0] + 600.0).abs() < 1.0);
        assert!((high[2] - 100.0).abs() < 1.0 && (low[2] + 100.0).abs() < 1.0);
        let wide = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Number(200.0),
        ];
        assert!(build(&SHAPE, &top, &wide, "Brep").is_err());
        let clicked = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(100.0),
            Answer::Point(p(0.0, 120.0, 0.0)),
        ];
        assert!(build(&SHAPE, &top, &clicked, "Brep").is_ok());
    }
}
