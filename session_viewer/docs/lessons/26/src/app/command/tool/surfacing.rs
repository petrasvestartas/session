// --8<-- [start:surfacing-helpers]
use session_rust::{Geometry, NurbsCurve, NurbsSurface, Point, Primitives, Vector};
use std::rc::Rc;

// `pub use` re-exports: the surfacing verbs import these from here, though they live in gather.
pub use super::gather::SAMPLES;

pub use super::gather::{Picked, count, picked};

/// The unit normal of a loop by Newell's method; None when it encloses no area.
pub fn loop_normal(points: &[Point]) -> Option<Vector> {
    // Newell's method, met in lesson 06, here on the samples of a closed curve.
    let mut sum = [0.0; 3];

    for (index, a) in points.iter().enumerate() {
        let b = &points[(index + 1) % points.len()];
        sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let length = (sum[0] * sum[0] + sum[1] * sum[1] + sum[2] * sum[2]).sqrt();
    let size = diagonal(points);

    // The sum is twice the enclosed area, so its threshold grows with the size squared.
    if length <= 1e-12 * size * size {
        return None;
    }

    Some(Vector::new(
        sum[0] / length,
        sum[1] / length,
        sum[2] / length,
    ))
}

/// The diagonal of the points' box, at least 1.
pub fn diagonal(points: &[Point]) -> f64 {
    let mut low = [f64::MAX; 3];
    let mut high = [f64::MIN; 3];

    for p in points {
        for i in 0..3 {
            low[i] = low[i].min(p[i]);
            high[i] = high[i].max(p[i]);
        }
    }

    let size = (0..3)
        .map(|i| (high[i] - low[i]).powi(2))
        .sum::<f64>()
        .sqrt();
    size.max(1.0)
}

/// True when every point lies on the plane through the first with `normal`, within a millionth of the size.
pub fn planar(points: &[Point], normal: &Vector) -> bool {
    let Some(first) = points.first() else {
        return false;
    };
    let tolerance = 1e-6 * diagonal(points);
    points
        .iter()
        .all(|p| (p - first).dot(normal).abs() <= tolerance)
}
// --8<-- [end:surfacing-helpers]

// --8<-- [start:align-sections]
/// Sections run the same way with their seams lined up; all open or all closed.
pub fn align_sections(sections: &mut [NurbsCurve]) -> Result<(), String> {
    let closed = sections.first().is_some_and(|curve| curve.is_closed());

    if sections.iter().any(|curve| curve.is_closed() != closed) {
        return Err("Curves must be all open or all closed".into());
    }

    for index in 1..sections.len() {
        // Rust allows no second borrow of a slice while one part is being changed; `split_at_mut` gives two separate halves.
        let (before, after) = sections.split_at_mut(index);
        let previous = &before[index - 1];
        let section = &mut after[0];

        if closed {
            let normal = loop_normal(&previous.divide_by_count(SAMPLES, false).0);
            let own = loop_normal(&section.divide_by_count(SAMPLES, false).0);

            if let (Some(normal), Some(own)) = (normal, own)
                && normal.dot(&own) < 0.0
            {
                section.reverse();
            }

            // The seam is where a closed curve starts; seams far apart would twist the surface between them.
            let t = section.closest_parameter(&previous.point_at_start());
            section.change_closed_curve_seam(t);
        } else {
            let (start, end) = (previous.point_at_start(), previous.point_at_end());
            let (a, b) = (section.point_at_start(), section.point_at_end());
            let same = a.distance(&start, None) + b.distance(&end, None);
            let flipped = a.distance(&end, None) + b.distance(&start, None);

            if flipped < same {
                section.reverse();
            }
        }

        // One parameter range on every section, so the loft matches them point for point.
        section.set_domain(0.0, 1.0);
    }

    Ok(())
}
// --8<-- [end:align-sections]

// --8<-- [start:loft-checked]
/// A loft through `sections` in order, cubic or `degree` in v; `closed` repeats the first.
pub fn loft(sections: &[NurbsCurve], closed: bool, degree: usize) -> Result<NurbsSurface, String> {
    if sections.len() < 2 {
        return Err("Select at least 2 curves".into());
    }

    if closed && sections.len() < 3 {
        return Err("A closed loft needs at least 3 curves".into());
    }

    let mut sections = sections.to_vec();
    align_sections(&mut sections)?;

    if closed {
        sections.push(sections[0].clone());
    }

    let surface = Primitives::create_loft(&sections, degree);

    if !surface.is_valid() {
        return Err("The curves could not be lofted".into());
    }

    Ok(surface)
}

/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    // An infinite or NaN control point is refused here, so a broken surface never reaches the document.
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!("The {name} surface could not be built"));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked.iter().map(|picked| picked.curve.clone()).collect()
}
// --8<-- [end:loft-checked]

// --8<-- [start:surfacing-tests]
#[cfg(test)]
pub mod tests {
    use super::*;
    use session_rust::{Line, Polyline, Xform};

    /// A point from coordinates.
    pub fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// A curve through control points, domain 0..1.
    pub fn curve(points: &[Point]) -> NurbsCurve {
        let mut curve = NurbsCurve::create(false, (points.len() - 1).min(3), points);
        curve.set_domain(0.0, 1.0);
        curve
    }

    /// A closed square of side `size` at height `z`, degree 1.
    pub fn square(size: f64, z: f64) -> NurbsCurve {
        curve(&[
            p(0.0, 0.0, z),
            p(size, 0.0, z),
            p(size, size, z),
            p(0.0, size, z),
            p(0.0, 0.0, z),
        ])
    }

    /// Lines, polylines and curves land where their placement puts them.
    #[test]
    fn world_curve_applies_the_row_placement() {
        let place = Xform::translation(10.0, 0.0, 5.0);
        let line = Geometry::Line(Rc::new(Line::from_points(&p(0., 0., 0.), &p(1., 0., 0.))));
        let polyline = Geometry::Polyline(Rc::new(Polyline::new(vec![
            p(0., 0., 0.),
            p(0., 0., 0.),
            p(1., 0., 0.),
            p(1., 1., 0.),
        ])));
        let nurbs = Geometry::NurbsCurve(Rc::new(curve(&[
            p(0., 0., 0.),
            p(1., 2., 0.),
            p(2., 0., 0.),
        ])));

        for geometry in [line, polyline, nurbs] {
            let picked = picked(&geometry, &place).unwrap();
            assert!(
                picked
                    .curve
                    .point_at_start()
                    .distance(&p(10., 0., 5.), None)
                    < 1e-12
            );
            assert_eq!(picked.curve.domain(), (0.0, 1.0));
        }

        let Some(Picked {
            corners: Some(corners),
            ..
        }) = picked(
            &Geometry::Polyline(Rc::new(Polyline::new(vec![
                p(0., 0., 0.),
                p(0., 0., 0.),
                p(1., 0., 0.),
            ]))),
            &place,
        )
        else {
            panic!()
        };
        assert_eq!(corners.len(), 2, "duplicates go");
        assert!(picked(&Geometry::Point(Rc::new(p(0., 0., 0.))), &place).is_none());
    }

    /// A periodic curve becomes clamped and stays closed.
    #[test]
    fn a_periodic_curve_is_clamped() {
        let ring = NurbsCurve::create(
            true,
            3,
            &[
                p(0., 0., 0.),
                p(10., 0., 0.),
                p(10., 10., 0.),
                p(0., 10., 0.),
            ],
        );
        assert!(ring.is_periodic());
        let picked = picked(&Geometry::NurbsCurve(Rc::new(ring)), &Xform::identity()).unwrap();
        assert!(!picked.curve.is_periodic());
        assert!(picked.curve.is_closed());
    }

    /// An open section pointing the other way is turned round.
    #[test]
    fn align_sections_reverses_an_opposite_open_section() {
        let mut sections = vec![
            curve(&[p(0., 0., 0.), p(10., 0., 0.)]),
            curve(&[p(10., 5., 0.), p(0., 5., 0.)]),
        ];
        align_sections(&mut sections).unwrap();
        assert!(sections[1].point_at_start().distance(&p(0., 5., 0.), None) < 1e-9);
        let mut mixed = vec![square(5.0, 0.0), curve(&[p(0., 0., 0.), p(1., 0., 0.)])];
        assert!(align_sections(&mut mixed).is_err());
    }

    /// Closed sections share their winding and start near each other.
    #[test]
    fn closed_sections_share_winding_and_seam() {
        let mut turned = square(10.0, 5.0);
        turned.reverse();
        let t = turned.closest_parameter(&p(10.0, 10.0, 5.0));
        turned.change_closed_curve_seam(t);
        let mut sections = vec![square(10.0, 0.0), turned];
        align_sections(&mut sections).unwrap();
        let a = loop_normal(&sections[0].divide_by_count(SAMPLES, false).0).unwrap();
        let b = loop_normal(&sections[1].divide_by_count(SAMPLES, false).0).unwrap();
        assert!(a.dot(&b) > 0.99);
        assert!(sections[1].point_at_start().distance(&p(0., 0., 5.), None) < 1e-6);
    }

    /// A square is planar with normal z; a bent loop is not planar.
    #[test]
    fn loop_normal_and_planar_on_a_square_and_a_bent_loop() {
        let square = [p(0., 0., 0.), p(4., 0., 0.), p(4., 3., 0.), p(0., 3., 0.)];
        let normal = loop_normal(&square).unwrap();
        assert!((normal[2] - 1.0).abs() < 1e-12);
        assert!(planar(&square, &normal));
        let bent = [
            p(0., 0., 0.),
            p(40., 0., 0.),
            p(40., 30., 10.),
            p(0., 30., 0.),
        ];
        assert!(!planar(&bent, &loop_normal(&bent).unwrap()));
        assert!(loop_normal(&[p(0., 0., 0.), p(1., 0., 0.), p(2., 0., 0.)]).is_none());
    }
}
// --8<-- [end:surfacing-tests]
