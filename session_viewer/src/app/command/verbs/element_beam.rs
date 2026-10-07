use crate::State;
use crate::app::command::tool::elements::{
    Param, Recipe, create, is_curve, lengths, picked_polylines, start,
};
use crate::app::command::{Action, Spec};
use session_rust::Polyline;
use wood::Beam;
use wood::geometry::profile_rectangle;

const USAGE: &str =
    "Element Beam width height · Example: select lines or polylines, then Element Beam 120 200";

pub const SPEC: Spec = Spec::new(
    &["Element Beam"],
    "Element Beam: pick lines or polylines, the axes · Width 150, Height 250 or two numbers change them · Enter creates a beam on each, mitred at its corners · Element Beam 120 200 on a selection creates at once",
    parse,
);

/// Read the typed values; left out they take their defaults and the command asks for picks.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let [width, height] = lengths(rest, [120.0, 200.0], USAGE)?;
    Ok(Box::new(ElementBeam {
        params: vec![
            Param {
                name: "Width",
                value: width,
            },
            Param {
                name: "Height",
                value: height,
            },
        ],
        typed: !rest.is_empty(),
    }))
}

/// One beam per axis of at least one segment.
fn build(axes: &[Polyline], width: f64, height: f64) -> Result<Vec<Beam>, String> {
    let beams: Vec<Beam> = axes
        .iter()
        .filter(|axis| axis.segment_count() >= 1)
        .map(|axis| Beam::from_profile(axis, profile_rectangle(width, height), "beam"))
        .collect();

    match beams.is_empty() {
        true => Err(format!(
            "Select lines or polylines, the beam axes · {USAGE}"
        )),
        false => Ok(beams),
    }
}

/// What the command picks and makes.
static RECIPE: Recipe = Recipe {
    name: "Element Beam",
    picks: "lines or polylines, the beam axes",
    fits: is_curve,
    options: &[
        ("Width", "Width"),
        ("Height", "Height"),
        ("Create", ""),
        ("Cancel", "Escape"),
    ],
    make,
};

/// The elements from the picks, in pick order.
fn make(state: &mut State, values: &[f64]) -> Result<String, String> {
    create(
        state,
        build(&picked_polylines(state), values[0], values[1])?,
        "Beam",
    )
}

#[derive(Debug)]
struct ElementBeam {
    params: Vec<Param>, // the values, typed or default
    typed: bool,        // typed values with a fitting selection make the elements at once
}

impl Action for ElementBeam {
    /// Make the elements, or ask for picks and values.
    fn run(&self, state: &mut State) -> Result<String, String> {
        start(state, &RECIPE, self.params.clone(), self.typed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::parse as parse_line;
    use session_rust::Point;
    use wood::WoodElement;

    #[test]
    fn element_beam_sweeps_the_rectangle_along_the_axis() {
        assert!(parse_line("Element Beam 100 200").is_ok() && parse_line("Element Beam").is_ok());
        assert!(parse_line("Element Beam 1 2 3").is_err());
        let axis = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1000.0, 0.0, 0.0),
        ]);
        let solid = build(&[axis], 100.0, 200.0).unwrap()[0].solid();
        assert!((solid.volume().abs() - 1000.0 * 100.0 * 200.0).abs() < 1e-6 && solid.is_closed());
        assert!(build(&[Polyline::new(vec![Point::new(0.0, 0.0, 0.0)])], 1.0, 1.0).is_err());
    }
}
