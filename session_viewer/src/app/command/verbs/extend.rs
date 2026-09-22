use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Extend"],
    aliases: &[],
    hint: "Select a line or curve · Extend -0.2 1.2 extends its domain at both ends",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Stretch a curve past its ends.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
