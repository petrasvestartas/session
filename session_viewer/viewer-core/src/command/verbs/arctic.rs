use crate::app::command::{Action, CommandHost, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Arctic On", "Arctic Off"],
    wait_for_option: true,
    ..Spec::new(
        &["Arctic"],
        "Arctic (On Off): soft contact shading, studio lighting and outlines",
        parse,
    )
};

/// Turn contact shading on or off.
pub fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Arctic(on_off(rest, "Arctic (On Off)")?)))
}

#[derive(Debug)]
struct Arctic(Option<bool>);

impl Action for Arctic {
    /// Flip the view flag the frame reads.
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        let on = self.0.unwrap_or(!state.arctic());
        state.set_arctic(on);
        Ok(format!(
            "Arctic {}",
            if state.arctic() { "On" } else { "Off" }
        ))
    }
}
