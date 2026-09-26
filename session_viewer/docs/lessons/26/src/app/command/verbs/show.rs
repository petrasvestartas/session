use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Show"],
    aliases: &[],
    hint: "",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Show everything that was hidden.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Show))
}

#[derive(Debug)]
struct Show;

impl Action for Show {
    /// Clear the hidden flag on every row.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.show_all();
        Ok("everything shown".into())
    }
}
