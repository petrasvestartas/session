use crate::State;
use crate::app::command::tool::elements::{
    Param, Recipe, create, is_curve, lengths, picked_polylines, start,
};
use crate::app::command::{Action, Spec};
use session_rust::{Line, Polyline};
use wood::EdgeStraightElement;
use wood::geometry::{compute_up, profile_rectangle};

const USAGE: &str =
    "Element Edge Straight width height · Example: select lines or polylines, then Element Edge Straight 120 200";
const COLUMN_USAGE: &str = "Element Edge Straight Column width depth · Example: select lines, then Element Edge Straight Column 200 300";

pub const SPEC: Spec = Spec {
    options: &["Element Edge Straight Column"],
    ..Spec::new(
        &["Element Edge Straight"],
        "Element Edge Straight (Column): pick lines or polylines, the axes · Width 150, Height 250 or two numbers change them · Enter creates a beam on each, mitred at its corners · Column: pick lines, Width 200, Depth 300, a standing column on each · Element Edge Straight 120 200 on a selection creates at once",
        parse,
    )
};

/// Read the option and the typed values; left out they take their defaults and the command asks for picks.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let column = rest.first().is_some_and(|word| word.eq_ignore_ascii_case("Column"));
    let values = if column { &rest[1..] } else { rest };

    if column {
        let [width, depth] = lengths(values, [200.0, 300.0], COLUMN_USAGE)?;
        return Ok(Box::new(ElementEdgeStraight {
            recipe: &COLUMN_RECIPE,
            params: vec![
                Param {
                    name: "Width",
                    value: width,
                },
                Param {
                    name: "Depth",
                    value: depth,
                },
            ],
            typed: !values.is_empty(),
        }));
    }

    let [width, height] = lengths(values, [120.0, 200.0], USAGE)?;
    Ok(Box::new(ElementEdgeStraight {
        recipe: &RECIPE,
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
        typed: !values.is_empty(),
    }))
}

/// One beam per axis of at least one segment.
fn build(axes: &[Polyline], width: f64, height: f64) -> Result<Vec<EdgeStraightElement>, String> {
    let beams: Vec<EdgeStraightElement> = axes
        .iter()
        .filter(|axis| axis.segment_count() >= 1)
        .map(|axis| {
            EdgeStraightElement::from_profile(
                axis,
                profile_rectangle(width, height),
                Vec::new(),
                "beam",
            )
        })
        .collect();

    match beams.is_empty() {
        true => Err(format!(
            "Select lines or polylines, the beam axes · {USAGE}"
        )),
        false => Ok(beams),
    }
}

/// One column per axis, a polyline taking its first and last point, its width across world x.
fn build_columns(
    axes: &[Polyline],
    width: f64,
    depth: f64,
) -> Result<Vec<EdgeStraightElement>, String> {
    let columns: Vec<EdgeStraightElement> = axes
        .iter()
        .filter_map(|axis| {
            Some(Line::from_points(
                &axis.get_point(0)?,
                &axis.get_point(axis.point_count() - 1)?,
            ))
        })
        .filter(|axis| axis.length() > 0.0)
        .map(|axis| {
            EdgeStraightElement::from_profile(
                &Polyline::new(vec![axis.start(), axis.end()]),
                profile_rectangle(width, depth),
                vec![compute_up(&axis, 0.0)],
                "column",
            )
        })
        .collect();

    match columns.is_empty() {
        true => Err(format!("Select lines, the column axes · {COLUMN_USAGE}")),
        false => Ok(columns),
    }
}

/// What the command picks and makes.
static RECIPE: Recipe = Recipe {
    name: "Element Edge Straight",
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

/// What the column option picks and makes.
static COLUMN_RECIPE: Recipe = Recipe {
    name: "Element Edge Straight Column",
    picks: "lines, the column axes",
    fits: is_curve,
    options: &[
        ("Width", "Width"),
        ("Depth", "Depth"),
        ("Create", ""),
        ("Cancel", "Escape"),
    ],
    make: make_columns,
};

/// The beams from the picks, in pick order.
fn make(state: &mut State, values: &[f64]) -> Result<String, String> {
    create(
        state,
        build(&picked_polylines(state), values[0], values[1])?,
        "Edge Straight",
    )
}

/// The columns from the picks, in pick order.
fn make_columns(state: &mut State, values: &[f64]) -> Result<String, String> {
    create(
        state,
        build_columns(&picked_polylines(state), values[0], values[1])?,
        "Edge Straight Column",
    )
}

#[derive(Debug)]
struct ElementEdgeStraight {
    recipe: &'static Recipe, // the beam or the column
    params: Vec<Param>,      // the values, typed or default
    typed: bool,             // typed values with a fitting selection make the elements at once
}

impl Action for ElementEdgeStraight {
    /// Make the elements, or ask for picks and values.
    fn run(&self, state: &mut State) -> Result<String, String> {
        start(state, self.recipe, self.params.clone(), self.typed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::parse as parse_line;
    use session_rust::Point;
    use wood::WoodElement;

    #[test]
    fn element_edge_straight_sweeps_the_rectangle_along_the_axis() {
        assert!(
            parse_line("Element Edge Straight 100 200").is_ok()
                && parse_line("Element Edge Straight").is_ok()
        );
        assert!(parse_line("Element Edge Straight 1 2 3").is_err());
        let axis = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1000.0, 0.0, 0.0),
        ]);
        let solid = build(&[axis], 100.0, 200.0).unwrap()[0].solid();
        assert!((solid.volume().abs() - 1000.0 * 100.0 * 200.0).abs() < 1e-6 && solid.is_closed());
        assert!(build(&[Polyline::new(vec![Point::new(0.0, 0.0, 0.0)])], 1.0, 1.0).is_err());
    }

    #[test]
    fn element_edge_straight_column_lofts_the_rectangle_up_the_line() {
        assert!(
            parse_line("Element Edge Straight Column 200 300").is_ok()
                && parse_line("element edge straight column").is_ok()
                && parse_line("Element Edge Straight Column x").is_err()
        );
        let axis = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 3000.0),
        ]);
        let columns = build_columns(&[axis], 200.0, 300.0).unwrap();
        let solid = columns[0].solid();
        assert!((solid.volume().abs() - 200.0 * 300.0 * 3000.0).abs() < 1e-6 && solid.is_closed());
        assert!(columns[0].base_plane().is_some());

        // the width lies along world x on a vertical line: volume alone passes a quarter turn off
        let ring = columns[0].sections()[0].get_points();
        let across_x = ring.iter().map(|point| point[0].abs()).fold(0.0, f64::max);
        let across_y = ring.iter().map(|point| point[1].abs()).fold(0.0, f64::max);
        assert!((across_x - 100.0).abs() < 1e-9 && (across_y - 150.0).abs() < 1e-9);
    }
}
