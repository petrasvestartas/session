use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    names: &["Attributes"],
    aliases: &[],
    hint: "Attributes (On Off): draw or remove the element features, moving with their element",
    options: &["Attributes On", "Attributes Off"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// Draw or remove the element features.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Attributes(on_off(rest, "Attributes (On Off)")?)))
}

#[derive(Debug)]
struct Attributes(Option<bool>);

impl Action for Attributes {
    /// Add or drop the feature rows inside every element.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let shown = state.show_attributes(self.0);
        Ok(format!("Attributes {}", if shown { "On" } else { "Off" }))
    }
}
