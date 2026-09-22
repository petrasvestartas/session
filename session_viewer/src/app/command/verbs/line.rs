use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Line"],
    aliases: &[],
    hint: "Line · Enter then click or type endpoints · Example: Line 0,0,0 100,0,0",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// A line between two typed points.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
