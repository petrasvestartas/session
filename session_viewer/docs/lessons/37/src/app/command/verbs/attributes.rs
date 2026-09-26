// --8<-- [start:attributes-verb]
use crate::State;
use crate::app::command::{Action, Spec, on_off};

// The command is typed "Element Features"; the code keeps the older name, attributes.
pub const SPEC: Spec = Spec {
    names: &["Element Features"],
    aliases: &[],
    hint: "Element Features (On Off): draw or remove the element features, moving with their element",
    options: &["Element Features On", "Element Features Off"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// Draw or remove the element features.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ElementFeatures(on_off(
        rest,
        "Element Features (On Off)",
    )?)))
}

#[derive(Debug)]
struct ElementFeatures(Option<bool>); // None: the verb typed alone flips the current state

impl Action for ElementFeatures {
    /// Add or drop the feature rows inside every element.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let shown = state.show_attributes(self.0); // the state answers with the value it settled on, so a toggle can report it
        Ok(format!(
            "Element Features {}",
            if shown { "On" } else { "Off" }
        ))
    }
}
// --8<-- [end:attributes-verb]
