use session_rust::{Polyline,Line};
use wood::Column;
use wood::geometry::profile_rectangle;
const USAGE: &str =
    "Element Column width depth · Example: select lines, then Element Column 200 300";
pub fn build(axes: &[Polyline], width: f64, depth: f64) -> Result<Vec<Column>, String> {
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

