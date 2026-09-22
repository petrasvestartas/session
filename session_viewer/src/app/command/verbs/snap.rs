use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    names: &["Snap"],
    aliases: &[],
    hint: "Snap (On Off): endpoints, vertices and midpoints within 12 pixels",
    options: &["Snap On", "Snap Off"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// Turn snapping on or off while drawing.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Snap(on_off(rest, "Snap (On Off)")?)))
}

#[derive(Debug)]
struct Snap(Option<bool>);

impl Action for Snap {
    /// Flip the flag the drawing code reads.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.snap_enabled = self.0.unwrap_or(!state.snap_enabled);
        Ok(format!(
            "Snap {}",
            if state.snap_enabled { "On" } else { "Off" }
        ))
    }

    /// Snapping is changed mid-draw, so the draft stays.
    fn keeps_draft(&self) -> bool {
        true
    }
}
