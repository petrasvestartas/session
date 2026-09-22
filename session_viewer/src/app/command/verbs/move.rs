use crate::State;
use crate::app::command::{Action, Spec, offset};
use session_rust::Xform;

pub const SPEC: Spec = Spec {
    names: &["Move"],
    aliases: &["m"],
    hint: "Select an object, then Move dx,dy,dz · Example: Move 10,0,0",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Shift the selection by a typed offset.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Move(offset(rest)?)))
}

#[derive(Debug)]
struct Move([f64; 3]);

impl Action for Move {
    /// Translate the selection and record it in history.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.apply(Xform::translation(self.0[0], self.0[1], self.0[2]), "move")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
