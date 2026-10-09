use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Element Attributes On", "Element Attributes Off"],
    wait_for_option: true,
    ..Spec::new(
        &["Element Attributes"],
        "Element Attributes (On Off): show or hide the objects hung under each element, such as its base plane",
        parse,
    )
};

/// Show or hide the element attributes.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ElementAttributes(on_off(
        rest,
        "Element Attributes (On Off)",
    )?)))
}

#[derive(Debug)]
struct ElementAttributes(Option<bool>);

impl Action for ElementAttributes {
    /// Show or hide the rows hung under every element.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let shown = state.show_attributes(self.0);
        Ok(format!(
            "Element Attributes {}",
            if shown { "On" } else { "Off" }
        ))
    }
}
