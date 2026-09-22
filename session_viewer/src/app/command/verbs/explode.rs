use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Explode"],
    aliases: &[],
    hint: "Select a polyline · Explode creates its individual line segments",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Split a polyline into its segments.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
