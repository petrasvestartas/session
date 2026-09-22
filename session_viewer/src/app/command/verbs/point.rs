use crate::app::command::{Action, Spec, model};

pub const SPEC: Spec = Spec {
    names: &["Point"],
    aliases: &[],
    hint: "Point · Enter then click or type x,y,z",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// One point at a typed coordinate.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(super::geometry::Model(model(verb, rest)?)))
}
