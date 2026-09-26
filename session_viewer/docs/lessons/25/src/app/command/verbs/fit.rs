use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Fit"],
    aliases: &[],
    hint: "Fit zooms to the selection, or the whole scene when nothing is selected",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Zoom to the selection, or to everything.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Fit))
}

#[derive(Debug)]
struct Fit;

impl Action for Fit {
    /// Frame the selection, or the whole scene.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.fit_selected_or_all();
        Ok("fitted".into())
    }
}
