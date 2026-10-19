use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{self, checked, count, curves};

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
    axis: crate::gather_input::no_axis,
    build,
};

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
