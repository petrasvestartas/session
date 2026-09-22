use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Escape"],
    aliases: &["esc"],
    hint: "",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Clear the selection.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Escape))
}

#[derive(Debug)]
struct Escape;

impl Action for Escape {
    /// Drop the selection and any gesture.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.escape_selection();
        Ok("selection cleared".into())
    }
}
