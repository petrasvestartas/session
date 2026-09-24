use crate::State;
use crate::app::command::{Action, Spec, number};

pub const SPEC: Spec = Spec {
    names: &["Scale"],
    aliases: &["s"],
    hint: "Select an object, then Scale factor · Example: Scale 2",
    options: &[],
    arity: Some(1),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// Grow or shrink the selection about the gizmo.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let k = number(rest.first().copied(), "Scale 2")?;

    if k <= 0.0 {
        return Err("Scale wants a factor above zero".into());
    }

    Ok(Box::new(Scale(k)))
}

#[derive(Debug)]
struct Scale(f64);

impl Action for Scale {
    /// Scale about the gizmo centre, or the world origin without one.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let about = state.gizmo.as_ref().map(|g| g.origin.clone());
        state.apply(crate::state::edit::scaling_about(self.0, about.as_ref()), "scale")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
