use crate::State;
use crate::app::command::tool::elements::{
    Param, Recipe, create, is_curve, lengths, picked_loops, start,
};
use crate::app::command::{Action, Spec};
use session_rust::Polyline;
use wood::Plate;

const USAGE: &str =
    "Element Plate thickness · Example: select closed polylines, then Element Plate 40";

pub const SPEC: Spec = Spec::new(
    &["Element Plate"],
    "Element Plate: pick closed polylines, the outlines · Thickness 60 or a number changes it · Enter creates a plate on each · Element Plate 40 on a selection creates at once",
    parse,
);

/// Read the typed values; left out they take their defaults and the command asks for picks.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let [thickness] = lengths(rest, [40.0], USAGE)?;
    Ok(Box::new(ElementPlate {
        params: vec![Param {
            name: "Thickness",
            value: thickness,
        }],
        typed: !rest.is_empty(),
    }))
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

/// What the command picks and makes.
static RECIPE: Recipe = Recipe {
    name: "Element Plate",
    picks: "closed polylines, the plate outlines",
    fits: is_curve,
    options: &[
        ("Thickness", "Thickness"),
        ("Create", ""),
        ("Cancel", "Escape"),
    ],
    make,
};

/// The elements from the picks, in pick order.
fn make(state: &mut State, values: &[f64]) -> Result<String, String> {
    create(state, build(&picked_loops(state), values[0])?, "Plate")
}

#[derive(Debug)]
struct ElementPlate {
    params: Vec<Param>, // the values, typed or default
    typed: bool,        // typed values with a fitting selection make the elements at once
}

impl Action for ElementPlate {
    /// Make the elements, or ask for picks and values.
    fn run(&self, state: &mut State) -> Result<String, String> {
        start(state, &RECIPE, self.params.clone(), self.typed)
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
