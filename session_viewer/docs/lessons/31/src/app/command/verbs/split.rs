// --8<-- [start:split-verb]
use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Split"],
        "Select a curve or face · Split · choose cutter curves · Enter confirms · Esc cancels",
        parse,
    )
};

/// Cut a curve or face with other curves.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Split))
}

#[derive(Debug)]
struct Split;

impl Action for Split {
    /// Start or confirm the split.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.split_command()
    }

    /// The split waits here for its cutters.
    fn keeps_split(&self) -> bool {
        true
    }
}
// --8<-- [end:split-verb]
