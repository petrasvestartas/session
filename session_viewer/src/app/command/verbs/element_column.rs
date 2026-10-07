use crate::State;
use crate::app::command::tool::elements::{
    Param, Recipe, create, is_curve, lengths, picked_polylines, start,
};
use crate::app::command::{Action, Spec};
use session_rust::{Line, Polyline};
use wood::Column;
use wood::geometry::profile_rectangle;

const USAGE: &str =
    "Element Column width depth · Example: select lines, then Element Column 200 300";

pub const SPEC: Spec = Spec::new(
    &["Element Column"],
    "Element Column: pick lines, the axes · Width 200, Depth 300 or two numbers change them · Enter creates a column on each · Element Column 200 300 on a selection creates at once",
    parse,
);

/// Read the typed values; left out they take their defaults and the command asks for picks.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let [width, depth] = lengths(rest, [200.0, 300.0], USAGE)?;
    Ok(Box::new(ElementColumn {
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
        typed: !rest.is_empty(),
    }))
}

/// One column per axis, a polyline taking its first and last point.
fn build(axes: &[Polyline], width: f64, depth: f64) -> Result<Vec<Column>, String> {
    let columns: Vec<Column> = axes
        .iter()
        .filter_map(|axis| {
            Some(Line::from_points(
                &axis.get_point(0)?,
                &axis.get_point(axis.point_count() - 1)?,
            ))
        })
        .filter(|axis| axis.length() > 0.0)
        .map(|axis| Column::from_profile(&axis, profile_rectangle(width, depth), 0.0, "column"))
        .collect();

    match columns.is_empty() {
        true => Err(format!("Select lines, the column axes · {USAGE}")),
        false => Ok(columns),
    }
}

/// What the command picks and makes.
static RECIPE: Recipe = Recipe {
    name: "Element Column",
    picks: "lines, the column axes",
    fits: is_curve,
    options: &[
        ("Width", "Width"),
        ("Depth", "Depth"),
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
        "Column",
    )
}

#[derive(Debug)]
struct ElementColumn {
    params: Vec<Param>, // the values, typed or default
    typed: bool,        // typed values with a fitting selection make the elements at once
}

impl Action for ElementColumn {
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
    fn element_column_lofts_the_rectangle_up_the_line() {
        assert!(
            parse_line("Element Column 200 300").is_ok() && parse_line("Element Column x").is_err()
        );
        let axis = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 3000.0),
        ]);
        let columns = build(&[axis], 200.0, 300.0).unwrap();
        let solid = columns[0].solid();
        assert!((solid.volume().abs() - 200.0 * 300.0 * 3000.0).abs() < 1e-6 && solid.is_closed());
        assert!(columns[0].base_plane().is_some());
    }
}
