use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Hide"],
        "",
        parse,
    )
};

/// Hide the selection from view.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Hide))
}

#[derive(Debug)]
struct Hide;

impl Action for Hide {
    /// Mark the selected rows hidden.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.hide_selected();
        Ok("hidden".into())
    }
}
