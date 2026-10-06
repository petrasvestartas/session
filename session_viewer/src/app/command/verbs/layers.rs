use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Layers On", "Layers Off", "Layers All"],
    wait_for_option: true,
    ..Spec::new(
        &["Layers"],
        "Layers (On Off All): show or hide the layer panel; All shows it with every row expanded",
        parse,
    )
};

/// Show or hide the layer panel, or show it with every row expanded.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if let [word] = rest
        && word.eq_ignore_ascii_case("all")
    {
        return Ok(Box::new(LayersAll));
    }

    Ok(Box::new(Layers(on_off(rest, "Layers (On Off All)")?)))
}

#[derive(Debug)]
struct Layers(Option<bool>);

impl Action for Layers {
    /// Open or close the panel, or flip it without an option, then rebuild its rows.
    fn run(&self, state: &mut State) -> Result<String, String> {
        crate::app::feedback::layers_visible(self.0.unwrap_or(!crate::app::feedback::layers_open()));
        state.refresh_layers();

        Ok("Layers (On Off All)".into())
    }
}

#[derive(Debug)]
struct LayersAll;

impl Action for LayersAll {
    /// Open the panel with every row that has children expanded.
    fn run(&self, state: &mut State) -> Result<String, String> {
        crate::app::feedback::layers_visible(true);
        state.refresh_layers();
        state.expand_layers();
        state.refresh_layers();

        Ok("Layers (On Off All)".into())
    }
}
