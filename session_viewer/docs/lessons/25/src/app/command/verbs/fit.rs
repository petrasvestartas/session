use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Fit"],
        "Fit zooms to the selection, or the whole scene when nothing is selected",
        parse,
    )
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
