use crate::app::command::CommandHost;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    aliases: &["del"],
    arity: Some(0),
    ..Spec::new(&["Delete"], "", parse)
};

/// Remove the selected objects.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Delete))
}

#[derive(Debug)]
struct Delete;

impl Action for Delete {
    /// Drop every selected row from its document, one undo step.
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        crate::op::execute(
            state,
            crate::op::Op::ObjectDelete {
                targets: crate::op::Targets::Selection,
            },
        )
        .map(|r| r.message)
        .map_err(|e| e.message)
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
