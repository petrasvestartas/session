use crate::State;
use crate::app::command::tool::elements::{create, lengths, picked_polylines};
use crate::app::command::{Action, Spec};
use session_rust::{Line, Polyline};
use wood::Column;
use wood::geometry::profile_rectangle;

const USAGE: &str =
    "Element Column width depth · Example: select lines, then Element Column 200 300";

pub const SPEC: Spec = Spec::new(
    &["Element Column"],
    "Element Column width depth: a wood column of that rectangle on every selected line, from its start to its end · Example: Element Column 200 300",
    parse,
);

/// Read the width and depth, 200 by 300 when left out.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let [width, depth] = lengths(rest, [200.0, 300.0], USAGE)?;
    Ok(Box::new(ElementColumn(width, depth)))
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

#[derive(Debug)]
struct ElementColumn(f64, f64);

impl Action for ElementColumn {
    /// One column per selected line.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let columns = build(&picked_polylines(state), self.0, self.1)?;
        create(state, columns, "Column")
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
