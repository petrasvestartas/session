use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::{checked, count};
use crate::app::command::{Action, Spec};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

pub const SPEC: Spec = Spec {
    names: &["Nurbs Surface Revolve"],
    aliases: &["nurbssurface_revolve"],
    hint: "Nurbs Surface Revolve: profile curves, Enter, axis start, axis end, angle · Example: Nurbs Surface Revolve 0,0,0 0,0,1 90",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

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
    axis: super::loft::no_axis,
    build,
};

/// Start revolving; typed points and the angle answer at once.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    gather::start(&RECIPE, rest)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::surfacing::tests::{curve, p};

    /// A full turn closes; every sample keeps its radius; a quarter stays open.
    #[test]
    fn a_line_revolves_into_a_cylinder() {
        let line = curve(&[p(20., 0., 0.), p(20., 0., 50.)]);
        let surface = revolve(&line, &p(0., 0., 0.), &p(0., 0., 10.), 360.0).unwrap();
        assert!(surface.is_valid() && surface.is_closed(0));
        let (u, v) = (surface.domain(0).unwrap(), surface.domain(1).unwrap());

        for i in 0..=8 {
            for j in 0..=4 {
                let q = surface
                    .point_at(
                        u.0 + (u.1 - u.0) * i as f64 / 8.0,
                        v.0 + (v.1 - v.0) * j as f64 / 4.0,
                    )
                    .unwrap();
                assert!((q[0].hypot(q[1]) - 20.0).abs() < 1e-9);
            }
        }

        let quarter = revolve(&line, &p(0., 0., 0.), &p(0., 0., 10.), 90.0).unwrap();
        assert!(!quarter.is_closed(0));
        assert!(revolve(&line, &p(0., 0., 0.), &p(0., 0., 0.), 90.0).is_err());
        assert!(revolve(&line, &p(0., 0., 0.), &p(0., 0., 1.), 400.0).is_err());
    }
}
