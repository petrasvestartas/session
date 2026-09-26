use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::{self, checked, count, curves};
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Nurbs Surface Loft"],
    aliases: &["nurbssurface_loft"],
    hint: "Nurbs Surface Loft (Smooth Straight): click curves in section order, Enter lofts",
    options: &["Nurbs Surface Loft Smooth", "Nurbs Surface Loft Straight"],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Loft",
    chips: &[
        ("Smooth", "Smooth"),
        ("Straight", "Straight"),
        ("Finish", ""),
        ("Cancel", "Escape"),
    ],
    steps: &[Step::Curves {
        prompt: "select curves in order",
        min: 2,
        max: usize::MAX,
    }],
    axis: super::loft::no_axis,
    build,
};

/// Start lofting; the selection in pick order gives the sections.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    gather::start(&RECIPE, rest)
}

/// A loft cubic across the sections, or straight strips between them.
fn build(input: &Input) -> Result<Made, String> {
    let sections = curves(&input.curves[0]);
    let degree = if input.choice == "Straight" { 1 } else { 3 };
    let surface = surfacing::loft(&sections, false, degree)?;
    Ok(Made {
        geometries: vec![checked(surface, "loft")?],
        message: format!(
            "Lofted {} into a NURBS surface",
            count(sections.len(), "curve")
        ),
    })
}
