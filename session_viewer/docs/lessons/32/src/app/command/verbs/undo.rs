use crate::State;
use crate::app::command::{Action, Spec};

// A verb that needs no points is a plain Spec: names, a parser, and an Action type.
pub const SPEC: Spec = Spec {
    names: &["Undo"],
    aliases: &[],
    hint: "",
    options: &[],
    arity: Some(0), // `Undo 3` is refused before parse runs
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Step one edit back.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Undo)) // put it on the heap as a `Box<dyn Action>`
}

#[derive(Debug)]
struct Undo; // no fields: it exists only to carry the Action impl

impl Action for Undo {
    /// Restore the document as it was before the last edit.
    fn run(&self, state: &mut State) -> Result<String, String> {
        if !state.scene.undo() {
            return Err("nothing to undo".into());
        }

        state.after_history(); // rebuild what the GPU shows from the document as it now is
        Ok("undone".into())
    }
}
