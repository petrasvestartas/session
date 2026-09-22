use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Polyline"],
    aliases: &[],
    hint: "Polyline · click points, or choose Rectangle / Polygon · Enter finishes",
    options: &["Polyline Points", "Polyline Rectangle", "Polyline Polygon"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// A chain of typed or clicked points.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
