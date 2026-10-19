use session_rust::{Polyline};
use wood::Beam;
use wood::geometry::profile_rectangle;
const USAGE: &str =
    "Element Beam width height · Example: select lines or polylines, then Element Beam 120 200";
pub fn build(axes: &[Polyline], width: f64, height: f64) -> Result<Vec<Beam>, String> {
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

