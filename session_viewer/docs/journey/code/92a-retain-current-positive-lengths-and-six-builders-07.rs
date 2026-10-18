use session_rust::{Polyline};
use wood::Plate;
const USAGE: &str =
    "Element Plate thickness · Example: select closed polylines, then Element Plate 40";
pub fn build(outlines: &[Polyline], thickness: f64) -> Result<Vec<Plate>, String> {
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

