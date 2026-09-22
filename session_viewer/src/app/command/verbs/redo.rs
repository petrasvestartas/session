use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Redo"],
    aliases: &[],
    hint: "",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Step one edit forward again.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Redo))
}

#[derive(Debug)]
struct Redo;

impl Action for Redo {
    /// Re-apply the edit that was undone.
    fn run(&self, state: &mut State) -> Result<String, String> {
        if !state.scene.redo() {
            return Err("nothing to redo".into());
        }

        state.after_history();
        Ok("redone".into())
    }

    fn needs_complete_scene(&self) -> bool {
        true
    }
}
