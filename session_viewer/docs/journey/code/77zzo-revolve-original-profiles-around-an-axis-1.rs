use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{checked, count};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Revolve",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
        Step::Point("axis start"),
        Step::Point("axis end"),
        Step::Number {
            prompt: "angle",
            default: 360.0,
            low: 0.0,
            high: 360.0,
        },
    ],
    axis: crate::gather_input::no_axis,
    build,
};

/// `profile` turned `degrees` right-handed about the axis from `start` to `end`.
pub fn revolve(
    profile: &NurbsCurve,
    start: &Point,
    end: &Point,
    degrees: f64,
) -> Result<NurbsSurface, String> {
    let axis = end - start;

    if axis.magnitude() <= 1e-12 {
        return Err("The axis start and end must differ".into());
    }

    if !(degrees > 0.0 && degrees <= 360.0) {
        return Err("The angle must be above 0 and at most 360".into());
    }

    Ok(Primitives::create_revolve(
        profile,
        start,
        &axis,
        degrees.to_radians(),
    ))
}

/// One surface per profile, all in one undo step.
fn build(input: &Input) -> Result<Made, String> {
    let (Some(start), Some(end), Some(degrees)) = (input.point(0), input.point(1), input.number(2))
    else {
        return Err("Nurbs Surface Revolve needs an axis and an angle".into());
    };
    let geometries = input.curves[0]
        .iter()
        .map(|picked| checked(revolve(&picked.curve, &start, &end, degrees)?, "revolve"))
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Made {
        message: format!(
            "Revolved {} by {degrees}°",
            count(geometries.len(), "curve")
        ),
        geometries,
    })
}

