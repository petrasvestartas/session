use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    names: &["Layers"],
    aliases: &[],
    hint: "Layers (On Off): show or hide the layer panel",
    options: &["Layers On", "Layers Off"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// Show or hide the layer panel.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Layers(on_off(rest, "Layers (On Off)")?)))
}

#[derive(Debug)]
struct Layers(Option<bool>);

impl Action for Layers {
    /// Open or close the panel, then rebuild its rows.
    fn run(&self, state: &mut State) -> Result<String, String> {
        if let Some(open) = self.0 {
            crate::app::feedback::layers_visible(open);
            state.refresh_layers();
        }

        Ok("Layers (On Off)".into())
    }
}
