use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Element Interactions On", "Element Interactions Off"],
    wait_for_option: true,
    ..Spec::new(
        &["Element Interactions"],
        "Element Interactions (On Off): draw or remove what other elements do to each element, its contacts, joints and cuts",
        parse,
    )
};

/// Draw or remove the element interactions.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ElementInteractions(on_off(
        rest,
        "Element Interactions (On Off)",
    )?)))
}

#[derive(Debug)]
struct ElementInteractions(Option<bool>);

impl Action for ElementInteractions {
    /// Walk every element again with or without its interactions.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let shown = state.show_interactions(self.0);
        Ok(format!(
            "Element Interactions {}",
            if shown { "On" } else { "Off" }
        ))
    }
}
