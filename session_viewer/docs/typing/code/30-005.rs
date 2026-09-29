use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Layers On", "Layers Off"],
    wait_for_option: true,
    ..Spec::new(
        &["Layers"],
        "Layers (On Off): show or hide the layer panel",
        parse,
    )
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
        // None: `Layers` typed alone changes nothing, the command line then offers On and Off
        if let Some(open) = self.0 {
            crate::app::feedback::layers_visible(open);
            state.refresh_layers();
        }

        Ok("Layers (On Off)".into())
    }
}
