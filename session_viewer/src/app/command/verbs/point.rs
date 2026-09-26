use super::geometry::Draw;
use crate::app::command::{Action, Spec};
use session_rust::{Geometry, Point};
use std::rc::Rc;

pub const SPEC: Draw = Draw {
    spec: Spec {
        names: &["Point"],
        aliases: &[],
        hint: "Point · Enter then click or type x,y,z",
        options: &[],
        arity: None,
        wait_for_option: false,
        wait_after_option: false,
        parse,
    },
    points: 1..=1,
    what: "point",
    buttons: &[],
    build,
};

/// One point at a typed coordinate.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Draw::parse(&SPEC, rest)
}

/// The point itself.
fn build(points: &[Point]) -> Result<Geometry, String> {
    Ok(Geometry::Point(Rc::new(points[0].clone())))
}
