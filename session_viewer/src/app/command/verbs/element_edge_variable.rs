use crate::State;
use crate::app::command::tool::elements::{Param, Recipe, create, is_curve, picked_loops, start};
use crate::app::command::{Action, Spec};
use session_rust::Polyline;
use wood::EdgeVariableElement;

const USAGE: &str = "Element Edge Variable · Example: pick closed polylines of one point count in station order, then Element Edge Variable";

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Element Edge Variable"],
        "Element Edge Variable: pick closed sections in station order · Enter creates the edge, its axis from the first section's centre to the last's · on a selection it creates at once",
        parse,
    )
};

/// No arguments.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(ElementEdgeVariable {
        params: Vec::new(),
        typed: true,
    }))
}

/// One variable beam through sections of one point count, in order.
fn build(sections: Vec<Polyline>) -> Result<EdgeVariableElement, String> {
    if sections.len() < 2 {
        return Err(format!("Pick at least two closed sections · {USAGE}"));
    }

    if sections
        .iter()
        .any(|section| section.point_count() != sections[0].point_count())
    {
        return Err(format!("Every section needs one point count · {USAGE}"));
    }

    Ok(EdgeVariableElement::through(sections, "edge_variable"))
}

/// What the command picks and makes.
static RECIPE: Recipe = Recipe {
    name: "Element Edge Variable",
    picks: "closed sections in station order",
    fits: is_curve,
    options: &[("Create", ""), ("Cancel", "Escape")],
    make,
};

/// The elements from the picks, in pick order.
fn make(state: &mut State, _values: &[f64]) -> Result<String, String> {
    create(state, vec![build(picked_loops(state))?], "Edge Variable")
}

#[derive(Debug)]
struct ElementEdgeVariable {
    params: Vec<Param>, // the values, typed or default
    typed: bool,        // typed values with a fitting selection make the elements at once
}

impl Action for ElementEdgeVariable {
    /// Make the elements, or ask for picks and values.
    fn run(&self, state: &mut State) -> Result<String, String> {
        start(state, &RECIPE, self.params.clone(), self.typed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::parse as parse_line;
    use crate::app::command::tool::elements::tests::square;
    use wood::WoodElement;

    #[test]
    fn element_edge_variable_lofts_through_its_stations() {
        assert!(parse_line("Element Edge Variable").is_ok());
        let beam = build(vec![
            square(100.0, 0.0),
            square(100.0, 500.0),
            square(100.0, 1000.0),
        ])
        .unwrap();
        let solid = beam.solid();
        assert!((solid.volume().abs() - 100.0 * 100.0 * 1000.0).abs() < 1e-6 && solid.is_closed());
        assert!((beam.axis.length() - 1000.0).abs() < 1e-9);
        assert!(build(vec![square(100.0, 0.0)]).is_err());
    }
}
