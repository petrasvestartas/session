use session_rust::{Polyline};
use wood::BeamVariable;
const USAGE: &str = "Element Beam Variable · Example: pick closed polylines of one point count in station order, then Element Beam Variable";
pub fn build(sections: Vec<Polyline>) -> Result<BeamVariable, String> {
    if sections.len() < 2 {
        return Err(format!("Pick at least two closed sections · {USAGE}"));
    }

    if sections
        .iter()
        .any(|section| section.point_count() != sections[0].point_count())
    {
        return Err(format!("Every section needs one point count · {USAGE}"));
    }

    Ok(BeamVariable::through(sections, "beam_variable"))
}

