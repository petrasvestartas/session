use session_rust::{Plane};
use wood::Support;
const USAGE: &str = "Element Support · Example: select points or planes, then Element Support";
pub fn build(planes: &[Plane]) -> Result<Vec<Support>, String> {
    match planes.is_empty() {
        true => Err(format!(
            "Select points or planes, where the supports stand · {USAGE}"
        )),
        false => Ok(planes
            .iter()
            .map(|plane| Support::new(plane, "support"))
            .collect()),
    }
}

