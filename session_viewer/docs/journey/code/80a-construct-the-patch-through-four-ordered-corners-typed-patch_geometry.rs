use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::checked;
use session_rust::{NurbsSurface, Point};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface 4 Points",
    chips: &[("Cancel", "Escape")],
    steps: &[
        Step::Point("corner 1 of 4"),
        Step::Point("corner 2 of 4"),
        Step::Point("corner 3 of 4"),
        Step::Point("corner 4 of 4"),
    ],
    axis: crate::gather_input::no_axis,
    build,
};

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

