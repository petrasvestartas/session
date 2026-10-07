use crate::State;
use crate::app::command::tool::elements::{create, lengths, picked_polylines};
use crate::app::command::{Action, Spec};
use session_rust::Polyline;
use wood::Beam;
use wood::geometry::profile_rectangle;

const USAGE: &str =
    "Element Beam width height · Example: select lines or polylines, then Element Beam 120 200";

pub const SPEC: Spec = Spec::new(
    &["Element Beam"],
    "Element Beam width height: a wood beam of that rectangle along every selected line or polyline, mitred at its corners · Example: Element Beam 120 200",
    parse,
);

/// Read the width and height, 120 by 200 when left out.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let [width, height] = lengths(rest, [120.0, 200.0], USAGE)?;
    Ok(Box::new(ElementBeam(width, height)))
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

#[derive(Debug)]
struct ElementBeam(f64, f64);

impl Action for ElementBeam {
    /// One beam per selected axis.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let beams = build(&picked_polylines(state), self.0, self.1)?;
        create(state, beams, "Beam")
    }

    fn needs_selection(&self) -> bool {
        true
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
