use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Trim"],
    aliases: &[],
    hint: "Select a line or curve · Trim 0.2 0.8 keeps that part of its length/domain",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Keep the part of a curve between two parameters.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
