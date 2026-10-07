use crate::State;
use crate::app::command::tool::elements::{create, lengths, picked_loops};
use crate::app::command::{Action, Spec};
use session_rust::Polyline;
use wood::Plate;

const USAGE: &str =
    "Element Plate thickness · Example: select closed polylines, then Element Plate 40";

pub const SPEC: Spec = Spec::new(
    &["Element Plate"],
    "Element Plate thickness: a wood plate on every selected closed polyline, the top that far along its normal · Example: Element Plate 40",
    parse,
);

/// Read the thickness, 40 when left out.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let [thickness] = lengths(rest, [40.0], USAGE)?;
    Ok(Box::new(ElementPlate(thickness)))
}

/// One plate per outline.
fn build(outlines: &[Polyline], thickness: f64) -> Result<Vec<Plate>, String> {
    match outlines.is_empty() {
        true => Err(format!(
            "Select closed polylines, the plate outlines · {USAGE}"
        )),
        false => Ok(outlines
            .iter()
            .map(|outline| Plate::from_outline(outline, thickness, "plate"))
            .collect()),
    }
}

#[derive(Debug)]
struct ElementPlate(f64);

impl Action for ElementPlate {
    /// One plate per selected closed polyline.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let plates = build(&picked_loops(state), self.0)?;
        create(state, plates, "Plate")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::parse as parse_line;
    use crate::app::command::tool::elements::tests::square;
    use wood::WoodElement;

    #[test]
    fn element_plate_lofts_the_outline_by_its_thickness() {
        assert!(parse_line("Element Plate").is_ok() && parse_line("element plate 25").is_ok());
        assert!(parse_line("Element Plate -4").is_err());
        let plates = build(&[square(600.0, 0.0)], 40.0).unwrap();
        let solid = plates[0].solid();
        assert!((solid.volume().abs() - 600.0 * 600.0 * 40.0).abs() < 1e-6 && solid.is_closed());
        assert!(build(&[], 40.0).is_err());
    }
}
