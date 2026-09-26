use super::geometry::Draw;
use crate::app::command::{Action, Spec};
use crate::app::modeling::MAX_POINTS;
use session_rust::{Geometry, Point, Polyline};
use std::rc::Rc;

pub const SPEC: Draw = Draw {
    spec: Spec {
        names: &["Polyline"],
        aliases: &[],
        hint: "Polyline · click points, or choose Rectangle / Polygon · Enter finishes",
        options: &["Polyline Points", "Polyline Rectangle", "Polyline Polygon"],
        arity: None,
        wait_for_option: false,
        wait_after_option: false,
        parse,
    },
    points: 2..=MAX_POINTS,
    what: "polyline",
    buttons: &[
        ("Points", "Polyline Points"),
        ("Rectangle", "Polyline Rectangle"),
        ("Polygon", "Polyline Polygon"),
        ("Close", "Close"),
        ("Finish", ""),
    ],
    build,
};

/// A chain of typed or clicked points.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Draw::parse(&SPEC, rest)
}

/// The points joined in order.
fn build(points: &[Point]) -> Result<Geometry, String> {
    Ok(Geometry::Polyline(Rc::new(Polyline::new(points.to_vec()))))
}
