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

/// Cancel the running command, else clear the selection.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Escape))
}

#[derive(Debug)]
struct Escape;

impl Action for Escape {
    /// Cancel a running command and keep the selection; with none running, drop the selection.
    fn run(&self, state: &mut State) -> Result<String, String> {
        if state.draft.is_some() {
            state.cancel_drawing();
            return Ok("cancelled".into());
        }

        state.escape_selection();
        Ok("selection cleared".into())
    }

    /// The draft is its to cancel.
    fn keeps_draft(&self) -> bool {
        true
    }
}
