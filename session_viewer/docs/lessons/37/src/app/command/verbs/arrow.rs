use super::geometry::Draw;
use crate::app::command::{Action, Spec};
use session_rust::{Arrowhead, Geometry, Point};
use std::rc::Rc;

pub const SPEC: Draw = Draw {
    spec: Spec {
        names: &["Arrow"],
        aliases: &[],
        hint: "Arrow · Enter then click or type start and tip · Example: Arrow 0,0,0 100,0,0",
        options: &[],
        arity: None,
        wait_for_option: false,
        wait_after_option: false,
        parse,
    },
    points: 2..=2,
    what: "arrow",
    buttons: &[],
    build,
};

/// A line from the start to the tip, with a head exactly at the tip.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Draw::parse(&SPEC, rest)
}

/// The line with a head at its end.
fn build(points: &[Point]) -> Result<Geometry, String> {
    let mut line = super::line::segment(points)?;
    line.arrowhead = Arrowhead::END;
    Ok(Geometry::Line(Rc::new(line)))
}
