use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Delete"],
    aliases: &["del"],
    hint: "",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Remove the selected objects.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Delete))
}

#[derive(Debug)]
struct Delete;

impl Action for Delete {
    /// Drop every selected row from its document, one undo step.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.delete_selection()
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
