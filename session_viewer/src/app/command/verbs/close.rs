use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Close"],
    aliases: &[],
    hint: "Close · joins the polyline or curve being drawn back to its first point",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Close the shape being drawn.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Close))
}

#[derive(Debug)]
struct Close;

impl Action for Close {
    /// Add the first point again and finish.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.close_drawing()
    }

    /// It works on the draft, so the draft stays until it finishes.
    fn keeps_draft(&self) -> bool {
        true
    }
}
