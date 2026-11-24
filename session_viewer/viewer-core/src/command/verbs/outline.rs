use crate::app::command::{Action, CommandHost, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Outline On", "Outline Off"],
    wait_for_option: true,
    ..Spec::new(
        &["Outline"],
        "Outline (On Off): black silhouette around solids",
        parse,
    )
};

/// Turn the black silhouette around solids on or off.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Outline(on_off(rest, "Outline (On Off)")?)))
}

#[derive(Debug)]
struct Outline(Option<bool>);

impl Action for Outline {
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        let on = self.0.unwrap_or(!state.outlines());
        state.set_outlines(on);
        Ok(format!(
            "Outline {}",
            if state.outlines() { "On" } else { "Off" }
        ))
    }
}
