use crate::State;
use crate::app::command::{Action, Spec, axis_and_number};
use crate::app::gizmo::Axis;

pub const SPEC: Spec = Spec {
    names: &["Rotate"],
    aliases: &["rot"],
    hint: "Select an object, then Rotate axis degrees · Example: Rotate z 45",
    options: &["Rotate x", "Rotate y", "Rotate z"],
    arity: Some(2),
    wait_for_option: true,
    wait_after_option: true,
    parse,
};

/// Turn the selection about a world axis.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let (axis, degrees) = axis_and_number(rest, "Rotate x 90")?;
    Ok(Box::new(Rotate { axis, degrees }))
}

#[derive(Debug)]
struct Rotate {
    axis: Axis,
    degrees: f64,
}

impl Action for Rotate {
    /// Turn about the gizmo centre, or the world origin without one.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let about = state.gizmo.as_ref().map(|g| g.origin.clone()); // turn about the gizmo
        let turn = crate::state::edit::rotation_about(self.axis, self.degrees, about.as_ref());
        state.apply(turn, "rotate")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
