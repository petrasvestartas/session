use crate::State;
use crate::app::command::tool::elements::{
    Param, Recipe, create, is_station, picked_planes, start,
};
use crate::app::command::{Action, Spec};
use session_rust::Plane;
use wood::Support;

const USAGE: &str = "Element Support · Example: select points or planes, then Element Support";

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Element Support"],
        "Element Support: pick points or planes, where the supports stand · Enter creates a support on each · on a selection it creates at once",
        parse,
    )
};

/// No arguments.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ElementSupport {
        params: Vec::new(),
        typed: true,
    }))
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

/// What the command picks and makes.
static RECIPE: Recipe = Recipe {
    name: "Element Support",
    picks: "points or planes, where the supports stand",
    fits: is_station,
    options: &[("Create", ""), ("Cancel", "Escape")],
    make,
};

/// The elements from the picks, in pick order.
fn make(state: &mut State, _values: &[f64]) -> Result<String, String> {
    create(state, build(&picked_planes(state))?, "Support")
}

#[derive(Debug)]
struct ElementSupport {
    params: Vec<Param>, // the values, typed or default
    typed: bool,        // typed values with a fitting selection make the elements at once
}

impl Action for ElementSupport {
    /// Make the elements, or ask for picks and values.
    fn run(&self, state: &mut State) -> Result<String, String> {
        start(state, &RECIPE, self.params.clone(), self.typed)
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
