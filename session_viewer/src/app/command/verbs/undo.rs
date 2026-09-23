use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Undo"],
    aliases: &[],
    hint: "",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Step one edit back.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Undo))
}

#[derive(Debug)]
struct Undo;

impl Action for Undo {
    /// Restore the document as it was before the last edit.
    fn run(&self, state: &mut State) -> Result<String, String> {
        if !state.scene.undo() {
            return Err("nothing to undo".into());
        }

        state.after_history();
        Ok("undone".into())
    }
}
