use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec::new(
    &["Controls"],
    "",
    parse,
);

/// Pick control points instead of objects.
fn parse(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if !rest.is_empty() {
        return Err(format!("no command `{verb}`"));
    }

    Ok(Box::new(Controls))
}

#[derive(Debug)]
struct Controls;

impl Action for Controls {
    /// Switch picking to control points.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.enable_controls();
        Ok("Control points".into())
    }
}
