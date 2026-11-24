use crate::app::command::{Action, CommandHost, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(&["Open"], "Open restores a saved .session file", parse)
};

/// Load a saved .session file.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Open))
}

#[derive(Debug)]
struct Open;

impl Action for Open {
    /// Ask the host for a file to read.
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        state.open_session_prompt()
    }
}
