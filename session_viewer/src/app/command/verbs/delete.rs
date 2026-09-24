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

/// Remove the selected object.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Delete))
}

#[derive(Debug)]
struct Delete;

impl Action for Delete {
    /// Drop the selected row from its document.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let row = state.scene.selected.ok_or("nothing is selected")?;

        if let Some(reason) = state.locked_reason(&[row]) {
            return Err(reason);
        }

        if !state.scene.delete_row(row) {
            return Err("This object cannot be deleted".into());
        }

        state.after_history();
        Ok("deleted".into())
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
