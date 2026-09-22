use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Curve"],
    aliases: &[],
    hint: "Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// A NURBS curve through its control points.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
