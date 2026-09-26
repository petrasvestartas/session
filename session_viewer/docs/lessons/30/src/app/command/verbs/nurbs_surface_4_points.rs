use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::checked;
use crate::app::command::{Action, Spec};
use session_rust::{NurbsSurface, Point};

pub const SPEC: Spec = Spec {
    names: &["Nurbs Surface 4 Points"],
    aliases: &["nurbssurface_4_points"],
    hint: "Nurbs Surface 4 Points: four corners in order · Example: Nurbs Surface 4 Points 0,0,0 10,0,0 10,10,3 0,10,0",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface 4 Points",
    chips: &[("Cancel", "Escape")],
    steps: &[
        Step::Point("corner 1 of 4"),
        Step::Point("corner 2 of 4"),
        Step::Point("corner 3 of 4"),
        Step::Point("corner 4 of 4"),
    ],
    axis: super::loft::no_axis,
    build,
};

/// Corners typed with the command answer at once.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if !rest.is_empty() && rest.len() != 4 {
        return Err("Nurbs Surface 4 Points takes 4 corner points".into());
    }

    gather::start(&RECIPE, rest)
}

/// The bilinear patch a, b, c, d: u runs a to b, v runs a to d.
pub fn patch(corners: &[Point; 4]) -> Result<NurbsSurface, String> {
    let [a, b, c, d] = corners;
    let area = (c - a).cross(&(d - b)).magnitude();
    let size = [b, c, d]
        .iter()
        .map(|p| p.distance(a, None))
        .fold(1.0, f64::max);

    if area <= 1e-12 * size * size {
        return Err("The 4 corners enclose no area".into());
    }

    let points = [a.clone(), d.clone(), b.clone(), c.clone()];
    NurbsSurface::create(false, false, 1, 1, 2, 2, &points)
}

/// The patch through the four answered corners.
fn build(input: &Input) -> Result<Made, String> {
    let corners: Vec<Point> = (0..4).filter_map(|index| input.point(index)).collect();
    let corners: [Point; 4] = corners
        .try_into()
        .map_err(|_| "Nurbs Surface 4 Points takes 4 corner points".to_string())?;
    Ok(Made {
        geometries: vec![checked(patch(&corners)?, "surface")?],
        message: "Created a NURBS surface from 4 corners".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::surfacing::tests::p;

    /// The corners come back and the middle is their mean.
    #[test]
    fn the_corners_are_reproduced() {
        let corners = [
            p(0., 0., 0.),
            p(100., 0., 0.),
            p(100., 100., 50.),
            p(0., 100., 0.),
        ];
        let surface = patch(&corners).unwrap();
        let (u, v) = (surface.domain(0).unwrap(), surface.domain(1).unwrap());
        let at = |s: f64, t: f64| {
            surface
                .point_at(u.0 + s * (u.1 - u.0), v.0 + t * (v.1 - v.0))
                .unwrap()
        };
        assert!(at(0., 0.).distance(&corners[0], None) < 1e-9);
        assert!(at(1., 0.).distance(&corners[1], None) < 1e-9);
        assert!(at(1., 1.).distance(&corners[2], None) < 1e-9);
        assert!(at(0., 1.).distance(&corners[3], None) < 1e-9);
        assert!(at(0.5, 0.5).distance(&p(50., 50., 12.5), None) < 1e-9);
        let flat = [p(0., 0., 0.), p(1., 0., 0.), p(2., 0., 0.), p(3., 0., 0.)];
        assert!(patch(&flat).is_err());
    }
}
