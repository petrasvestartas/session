use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Arrow"],
    aliases: &[],
    hint: "Arrow · Enter then click or type start and tip · Example: Arrow 0,0,0 100,0,0",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// A line from the start to the tip, with a head exactly at the tip.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
