use super::geometry::Draw;
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Point};
use std::rc::Rc;

// The whole verb is this one constant: REGISTRY lists `&point::SPEC`, and nothing else in the viewer names Point.
pub const SPEC: Draw = Draw {
    spec: Spec {
        names: &["Point"],
        aliases: &[],
        hint: "Point · Enter then click or type x,y,z",
        options: &[],
        arity: None,
        wait_for_option: false,
        wait_after_option: false,
        parse, // shorthand for `parse: parse`, the function below
    },
    points: 1..=1, // exactly one point, so the first click finishes it
    what: "point",
    buttons: &[],
    build,
};

/// `Point 1,2,3`: the shared Draw parser checks the words and the count.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Draw::parse(&SPEC, rest)
}

/// The point itself.
fn build(points: &[Point]) -> Result<Geometry, String> {
    Ok(Geometry::Point(Rc::new(points[0].clone())))
}
