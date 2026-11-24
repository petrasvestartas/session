use crate::app::command::CommandHost;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(&["Redo"], "", parse)
};

/// Step one edit forward again.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Redo))
}

#[derive(Debug)]
struct Redo;

impl Action for Redo {
    /// Re-apply the edit that was undone.
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        crate::op::execute(state, crate::op::Op::HistoryRedo)
            .map(|r| r.message)
            .map_err(|e| e.message)
    }
}
