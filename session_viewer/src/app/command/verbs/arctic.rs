use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    names: &["Arctic"],
    aliases: &[],
    hint: "Arctic (On Off): soft contact shading and studio lighting · G toggles in the viewport",
    options: &["Arctic On", "Arctic Off"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// Turn contact shading on or off.
pub fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Arctic(on_off(rest, "Arctic (On Off)")?)))
}

#[derive(Debug)]
struct Arctic(Option<bool>);

impl Action for Arctic {
    /// Flip the view flag the frame reads.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.gpu.view.ssao = self.0.unwrap_or(!state.gpu.view.ssao);
        Ok(format!(
            "Arctic {}",
            if state.gpu.view.ssao { "On" } else { "Off" }
        ))
    }
}
