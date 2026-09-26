use crate::State;
use crate::app::command::{Action, Spec};
use crate::app::selection::SelectionTool;

pub const SPEC: Spec = Spec {
    names: &["Object"],
    aliases: &[],
    hint: "",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Click picks whole objects.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if !rest.is_empty() {
        return Err(format!("no command `{verb}`"));
    }

    Ok(Box::new(Selection(SelectionTool::Object)))
}

#[derive(Debug)]
struct Selection(SelectionTool);

impl Action for Selection {
    /// Clear the selection, then pick this kind from now on.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.escape_selection();
        state.selection_tool = self.0;
        Ok(format!("{:?} selection", self.0))
    }
}
