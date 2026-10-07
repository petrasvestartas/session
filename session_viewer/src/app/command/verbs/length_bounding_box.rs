use super::measure::{to_text, unit_suffix};
use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Length BoundingBox"],
        "Length BoundingBox · world X, Y and Z extents of the selection",
        parse,
    )
};

fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(LengthBoundingBox))
}

#[derive(Debug)]
struct LengthBoundingBox;

impl Action for LengthBoundingBox {
    fn run(&self, state: &mut State) -> Result<String, String> {
        super::measure::loading(state)?;
        let b = super::bounding_box::selected_bounds(state)?;
        let value = format!(
            "X {} · Y {} · Z {} {}",
            to_text(2.0 * b.hx),
            to_text(2.0 * b.hy),
            to_text(2.0 * b.hz),
            unit_suffix(state, 1)
        );
        state.mark_selection(value.clone());
        Ok(format!("Length BoundingBox · {value}"))
    }

    fn needs_selection(&self) -> bool {
        true
    }
}
