use crate::State;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    names: &["Outline"],
    aliases: &[],
    hint: "Outline (On Off): black surface outlines · O toggles in the viewport",
    options: &["Outline On", "Outline Off"],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(Outline(on_off(rest, "Outline (On Off)")?)))
}

#[derive(Debug)]
struct Outline(Option<bool>);

impl Action for Outline {
    fn run(&self, state: &mut State) -> Result<String, String> {
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
