use crate::State;
use crate::app::command::tool::elements::{create, picked_planes};
use crate::app::command::{Action, Spec};
use session_rust::Plane;
use wood::Support;

const USAGE: &str = "Element Support · Example: select points or planes, then Element Support";

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Element Support"],
        "Element Support: a wood column support with the manufacturer's dimensions on every selected point (standing up z) or plane · Example: Element Support",
        parse,
    )
};

/// No arguments.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ElementSupport))
}

/// One support per plane.
fn build(planes: &[Plane]) -> Result<Vec<Support>, String> {
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

#[derive(Debug)]
struct ElementSupport;

impl Action for ElementSupport {
    /// One support per selected point or plane.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let supports = build(&picked_planes(state))?;
        create(state, supports, "Support")
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::parse as parse_line;
    use wood::WoodElement;

    #[test]
    fn element_support_stands_on_its_plane() {
        assert!(parse_line("Element Support").is_ok() && parse_line("Element Support 1").is_err());
        let supports = build(&[Plane::xy_plane()]).unwrap();
        assert!(supports[0].solid().number_of_faces() > 0);
        assert_eq!(
            supports[0].base_plane().unwrap().origin(),
            Plane::xy_plane().origin()
        );
        assert!(build(&[]).is_err());
    }
}
