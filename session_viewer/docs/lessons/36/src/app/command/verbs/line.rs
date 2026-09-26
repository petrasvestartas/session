use super::geometry::Draw;
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Line, Point};
use std::rc::Rc;

pub const SPEC: Draw = Draw {
    spec: Spec {
        names: &["Line"],
        aliases: &[],
        hint: "Line · Enter then click or type endpoints · Example: Line 0,0,0 100,0,0",
        options: &[],
        arity: None,
        wait_for_option: false,
        wait_after_option: false,
        parse,
    },
    points: 2..=2,
    what: "line",
    buttons: &[],
    build,
};

/// A line between two typed points.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Draw::parse(&SPEC, rest)
}

/// The line between the two points.
fn build(points: &[Point]) -> Result<Geometry, String> {
    Ok(Geometry::Line(Rc::new(segment(points)?)))
}

/// A line of some length from the first point to the second; `pub` so Arrow builds on it.
pub fn segment(points: &[Point]) -> Result<Line, String> {
    let line = Line::from_points(&points[0], &points[1]);

    if line.length() <= 1e-12 { // two clicks on one spot make no line
        return Err("line endpoints must differ".into());
    }

    Ok(line)
}
