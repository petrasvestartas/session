use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Hide"],
    aliases: &[],
    hint: "",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
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
