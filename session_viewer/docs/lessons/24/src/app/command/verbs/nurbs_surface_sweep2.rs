use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::{checked, count, curves};
use crate::app::command::{Action, Spec};
use session_rust::{NurbsCurve, NurbsSurface, Primitives};

pub const SPEC: Spec = Spec {
    names: &["Nurbs Surface Sweep2"],
    aliases: &["nurbssurface_sweep2"],
    hint: "Nurbs Surface Sweep2: first rail, second rail, shape curves, Enter sweeps",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Sweep2",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select the first rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select the second rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select shape curves",
            min: 1,
            max: usize::MAX,
        },
    ],
    axis: super::loft::no_axis,
    build,
};

/// Start sweeping; selected curves are rail 1, rail 2, then shapes.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    gather::start(&RECIPE, rest)
}

/// Rail 2 runs with rail 1 and every shape starts on rail 1.
pub fn sweep2(
    first: &NurbsCurve,
    second: &NurbsCurve,
    shapes: &[NurbsCurve],
) -> Result<NurbsSurface, String> {
    let mut second = second.clone();
    let start = first.point_at_start();

    if second
        .point_at_start()
        .distance(&first.point_at_end(), None)
        < second.point_at_start().distance(&start, None)
    {
        second.reverse();
    }

    let shapes: Vec<NurbsCurve> = shapes
        .iter()
        .map(|shape| {
            let mut shape = shape.clone();
            let near = |curve: &NurbsCurve, rail: &NurbsCurve| {
                curve.point_at_start().distance(
                    &rail.point_at(rail.closest_parameter(&curve.point_at_start())),
                    None,
                )
            };

            if near(&shape, &second) < near(&shape, first) {
                shape.reverse();
            }

            shape
        })
        .collect();
    let surface = Primitives::create_sweep2(first, &second, &shapes);

    if !surface.is_valid() {
        return Err("The shapes could not be swept along the rails".into());
    }

    Ok(surface)
}

/// The sweep of the shapes between the two rails.
fn build(input: &Input) -> Result<Made, String> {
    let shapes = curves(&input.curves[2]);
    let surface = sweep2(
        &input.curves[0][0].curve,
        &input.curves[1][0].curve,
        &shapes,
    )?;
    Ok(Made {
        geometries: vec![checked(surface, "sweep")?],
        message: format!("Swept {} along 2 rails", count(shapes.len(), "shape")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::surfacing::tests::{curve, p};

    /// The shape's ends ride the rails; a reversed rail 2 changes nothing.
    #[test]
    fn shape_ends_lie_on_the_rails() {
        let first = curve(&[p(0., 0., 0.), p(100., 0., 0.)]);
        let second = curve(&[p(0., 50., 0.), p(100., 50., 20.)]);
        let shape = curve(&[p(0., 0., 0.), p(0., 25., 15.), p(0., 50., 0.)]);
        let surface = sweep2(&first, &second, std::slice::from_ref(&shape)).unwrap();

        for t in [0.0, 0.5, 1.0] {
            for rail in [&first, &second] {
                let q = rail.point_at(t);
                assert!(surface.closest_point(&q).distance(&q, None) < 1e-3, "{t}");
            }
        }

        let mut turned = second.clone();
        turned.reverse();
        let again = sweep2(&first, &turned, &[shape]).unwrap();
        let (u, v) = (surface.domain(0).unwrap(), surface.domain(1).unwrap());
        let (a, b) = (again.domain(0).unwrap(), again.domain(1).unwrap());
        let one = surface
            .point_at((u.0 + u.1) / 2.0, (v.0 + v.1) / 2.0)
            .unwrap();
        let two = again
            .point_at((a.0 + a.1) / 2.0, (b.0 + b.1) / 2.0)
            .unwrap();
        assert!(one.distance(&two, None) < 1e-6);
    }
}
