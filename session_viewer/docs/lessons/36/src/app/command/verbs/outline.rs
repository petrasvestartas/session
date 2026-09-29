// --8<-- [start:outline-verb]
use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Outline On", "Outline Off"],
    wait_for_option: true,
    ..Spec::new(
        &["Outline"],
        "Outline (On Off): black surface outlines · O toggles in the viewport",
        parse,
    )
};

fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Outline(on_off(rest, "Outline (On Off)")?)))
}

#[derive(Debug)]
struct Outline(Option<bool>);

impl Action for Outline {
    fn run(&self, state: &mut State) -> Result<String, String> {
        // independent of Arctic: Outline Off hides the outlines and keeps the shading
        state.gpu.view.show_outlines = self.0.unwrap_or(!state.gpu.view.show_outlines);
        Ok(format!(
            "Outline {}",
            if state.gpu.view.show_outlines {
                "On"
            } else {
                "Off"
            }
        ))
    }
}
// --8<-- [end:outline-verb]
