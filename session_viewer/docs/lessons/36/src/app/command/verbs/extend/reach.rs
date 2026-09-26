use crate::app::command::tool::cut::{Cutter, unit_ray};
use session_rust::{
    Geometry, Line, Mesh, NurbsCurve, Point, Polyline, Vector, Xform, intersection, simple_split,
};
use std::rc::Rc;

const NEAR: f64 = 1e-6; // a crossing this close to the end is the end itself

/// Which end of a curve grows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum End {
    Start,
    End,
}

/// How far an end grows, in the curve's own frame.
pub enum Reach {
    Distance(f64),
    Boundaries {
        cutters: Vec<Cutter>,           // curves and planes
        meshes: Vec<(Rc<Mesh>, Xform)>, // meshes, each with the map from the curve's frame into its own
    },
}

/// A grown curve and the piece it gained.
pub struct Grown {
    pub geometry: Geometry, // the whole curve
    pub tail: Vec<Point>,   // the piece added, from the old end
}

/// The end point of a curve and the direction it leaves in.
pub fn tip(geometry: &Geometry, end: End) -> Option<(Point, Vector)> {
    let (tip, inward) = match geometry {
        Geometry::Line(line) => match end {
            End::Start => (line.start(), line.end()),
            End::End => (line.end(), line.start()),
        },
        Geometry::Polyline(polyline) => {
            let count = polyline.point_count();

            if count < 2 {
                return None;
            }

            match end {
                End::Start => (polyline.get_point(0)?, polyline.get_point(1)?),
                End::End => (
                    polyline.get_point(count - 1)?,
                    polyline.get_point(count - 2)?,
                ),
            }
        }
        Geometry::NurbsCurve(curve) => {
            let (d0, d1) = curve.domain();
            let (t, sign) = match end {
                End::Start => (d0, -1.0),
                End::End => (d1, 1.0),
            };
            let mut direction = &curve.tangent_at(t) * sign;
            direction.normalize_self().then_some(())?;
            return Some((curve.point_at(t), direction));
        }
        _ => return None,
    };
    let mut direction = &tip - &inward;
    direction.normalize_self().then_some(())?;
    Some((tip, direction))
}

/// `geometry` grown at `end` by a distance or to the first boundary ahead.
pub fn extended(geometry: &Geometry, end: End, reach: &Reach) -> Result<Grown, String> {
    match geometry {
        Geometry::Line(line) => {
            let (from, direction) = tip(geometry, end).ok_or("This line has no length")?;
            let to = straight(&from, &direction, reach)?;
            let grow = from.distance(&to, None);
            let mut next = (**line).clone();

            match end {
                End::Start => next.extend(grow, 0.0),
                End::End => next.extend(0.0, grow),
            }

            Ok(Grown {
                geometry: Geometry::Line(Rc::new(next)),
                tail: vec![from, to],
            })
        }
        Geometry::Polyline(polyline) => {
            if polyline.is_closed() {
                return Err("no free end".into());
            }

            let (from, direction) = tip(geometry, end).ok_or("This polyline has no length")?;
            let to = straight(&from, &direction, reach)?;
            let last = polyline.point_count() - 1;
            let mut next: Polyline = (**polyline).clone();
            next.set_point(if end == End::Start { 0 } else { last }, &to);
            Ok(Grown {
                geometry: Geometry::Polyline(Rc::new(next)),
                tail: vec![from, to],
            })
        }
        Geometry::NurbsCurve(curve) => {
            if curve.is_closed() {
                return Err("no free end".into());
            }

            let grown = match reach {
                Reach::Distance(distance) => by_length(curve, end, *distance)?,
                Reach::Boundaries { cutters, .. } => to_boundary(curve, end, cutters)?,
            };
            let tail = tail(curve, &grown, end);
            let mut grown = grown;
            fit_colors(&mut grown);
            Ok(Grown {
                geometry: Geometry::NurbsCurve(Rc::new(grown)),
                tail,
            })
        }
        _ => Err("Extend grows lines, polylines and NURBS curves".into()),
    }
}

/// Where a straight end lands: a distance on, or the first boundary ahead.
fn straight(from: &Point, direction: &Vector, reach: &Reach) -> Result<Point, String> {
    let (cutters, meshes) = match reach {
        Reach::Distance(distance) => return Ok(from + &(direction * *distance)),
        Reach::Boundaries { cutters, meshes } => (cutters, meshes),
    };
    let mut best: Option<f64> = None; // distance ahead of the nearest boundary
    let mut take = |t: f64| {
        if t > NEAR && best.is_none_or(|near| t < near) {
            best = Some(t);
        }
    };

    for cutter in cutters {
        match cutter {
            Cutter::Plane(plane) => {
                let ray = Line::from_points(from, &(from + direction));

                if let Some(hit) = intersection::line_plane(&ray, plane, false) {
                    take((&hit - from).dot(direction));
                }
            }
            Cutter::Curve(curve) => {
                // a probe reaching past every control point of the boundary
                let reach = (0..curve.m_cv_count)
                    .filter_map(|i| curve.get_cv(i))
                    .map(|p| p.distance(from, None))
                    .fold(1.0, f64::max)
                    * 2.0;
                let probe =
                    NurbsCurve::create(false, 1, &[from.clone(), from + &(direction * reach)]);

                if let Ok(pieces) =
                    simple_split::split_curve_by_curves(&probe, std::slice::from_ref(curve), NEAR)
                {
                    for piece in pieces.iter().take(pieces.len().saturating_sub(1)) {
                        take((&piece.point_at_end() - from).dot(direction));
                    }
                }
            }
        }
    }

    for (mesh, into) in meshes {
        let Some(back) = into.inverse() else {
            continue;
        };
        let ray = unit_ray(from, direction, into);

        for hit in intersection::ray_mesh(&ray, mesh, 1e-12, true).unwrap_or_default() {
            take((&hit.transformed(&back) - from).dot(direction));
        }
    }

    best.map(|t| from + &(direction * t))
        .ok_or_else(|| "no boundary ahead".into())
}

/// The curve with its domain grown by `s` at `end`.
fn grow(curve: &NurbsCurve, end: End, s: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let mut grown = curve.clone();
    let done = match end {
        End::Start => grown.extend(d0 - s, d1),
        End::End => grown.extend(d0, d1 + s),
    };
    done.then_some(grown)
}

/// The part of `grown` beyond the old domain.
fn beyond(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let (g0, g1) = grown.domain();
    let mut piece = grown.clone();
    let done = match end {
        End::Start => piece.trim(g0, d0),
        End::End => piece.trim(d1, g1),
    };
    done.then_some(piece)
}

/// The added piece as points, from the old end.
fn tail(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Vec<Point> {
    let Some(piece) = beyond(curve, grown, end) else {
        return Vec::new();
    };
    let mut points = crate::app::command::tool::cut::samples(&piece, 24);

    if end == End::Start {
        points.reverse();
    }

    points
}

/// A curve grown along its own shape until the added length is `distance`.
fn by_length(curve: &NurbsCurve, end: End, distance: f64) -> Result<NurbsCurve, String> {
    let (d0, d1) = curve.domain();
    let at = if end == End::Start { d0 } else { d1 };
    let speed = curve.evaluate(at, 1)[1].magnitude();

    if speed <= 1e-12 {
        return Err("This curve has no direction at its end".into());
    }

    // the added length for a growth of `s`
    let added = |s: f64| -> Result<f64, String> {
        let grown = grow(curve, end, s).ok_or("Cannot extend this curve")?;
        Ok(beyond(curve, &grown, end)
            .ok_or("Cannot extend this curve")?
            .length(None))
    };
    let (mut s0, mut f0) = (0.0, -distance);
    let mut s1 = distance / speed;
    let mut f1 = added(s1)? - distance;

    // secant steps on the added length
    for _ in 0..8 {
        if f1.abs() <= 1e-9 * distance || (f1 - f0).abs() <= f64::MIN_POSITIVE {
            break;
        }

        let s2 = (s1 - f1 * (s1 - s0) / (f1 - f0)).max(s1 * 0.1);
        (s0, f0) = (s1, f1);
        s1 = s2;
        f1 = added(s1)? - distance;
    }

    grow(curve, end, s1).ok_or_else(|| "Cannot extend this curve".into())
}

/// A curve grown along its own shape to the first boundary curve or plane it meets.
fn to_boundary(curve: &NurbsCurve, end: End, cutters: &[Cutter]) -> Result<NurbsCurve, String> {
    if cutters.is_empty() {
        return Err(
            "Meshes and BReps stop lines and polylines; pick a curve or plane for a curve".into(),
        );
    }

    let (d0, d1) = curve.domain();
    let at = if end == End::Start { d0 } else { d1 };
    let from = curve.point_at(at);
    let speed = curve.evaluate(at, 1)[1].magnitude().max(1e-12);
    // how far the nearest boundary is, as a first guess
    let away = cutters
        .iter()
        .map(|cutter| match cutter {
            Cutter::Plane(plane) => (&from - &plane.origin()).dot(&plane.z_axis()).abs(),
            Cutter::Curve(boundary) => boundary.closest_point(&from).distance(&from, None),
        })
        .fold(f64::INFINITY, f64::min)
        .max(curve.length(None) * 0.05);
    let mut s = away / speed;

    for _ in 0..7 {
        let grown = grow(curve, end, s).ok_or("Cannot extend this curve")?;
        let piece = beyond(curve, &grown, end).ok_or("Cannot extend this curve")?;
        let (p0, p1) = piece.domain();
        let near = (p1 - p0) * 1e-9;
        let mut crossings = Vec::new();

        for cutter in cutters {
            match cutter {
                Cutter::Plane(plane) => {
                    crossings.extend(intersection::curve_plane(&piece, plane, Some(NEAR)))
                }
                Cutter::Curve(boundary) => {
                    let pieces = simple_split::split_curve_by_curves(
                        &piece,
                        std::slice::from_ref(boundary),
                        NEAR,
                    )
                    .unwrap_or_default();

                    for part in pieces.iter().take(pieces.len().saturating_sub(1)) {
                        crossings.push(
                            intersection::curve_closest_point(&piece, &part.point_at_end(), p0, p1)
                                .0,
                        );
                    }
                }
            }
        }

        // the crossing nearest the old end
        let first = match end {
            End::End => crossings
                .into_iter()
                .filter(|t| *t > p0 + near)
                .min_by(f64::total_cmp),
            End::Start => crossings
                .into_iter()
                .filter(|t| *t < p1 - near)
                .max_by(f64::total_cmp),
        };

        if let Some(t) = first {
            let (g0, g1) = grown.domain();
            let mut cut = grown;
            let done = match end {
                End::End => cut.trim(g0, t),
                End::Start => cut.trim(t, g1),
            };
            return done
                .then_some(cut)
                .ok_or_else(|| "Cannot cut the curve at the boundary".into());
        }

        s *= 2.0;
    }

    Err("does not reach the boundary".into())
}

/// Per-control colours only fit the control count they were made for.
fn fit_colors(curve: &mut NurbsCurve) {
    if curve.pointcolors.len() != curve.m_cv_count {
        curve.pointcolors.clear();
    }

    if curve.linecolors.len() + 1 != curve.m_cv_count {
        curve.linecolors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Plane;

    /// A boundary line x = `x`.
    fn wall(x: f64) -> Cutter {
        Cutter::Curve(NurbsCurve::create(
            false,
            1,
            &[Point::new(x, -1000., 0.), Point::new(x, 1000., 0.)],
        ))
    }

    /// Boundaries only.
    fn to(cutters: Vec<Cutter>) -> Reach {
        Reach::Boundaries {
            cutters,
            meshes: Vec::new(),
        }
    }

    /// The ends of a line geometry.
    fn ends(grown: &Grown) -> (Point, Point) {
        let Geometry::Line(line) = &grown.geometry else {
            panic!("not a line")
        };
        (line.start(), line.end())
    }

    /// A line grows to the boundary ahead of the chosen end, or by a distance.
    #[test]
    fn a_line_reaches_the_boundary_ahead() {
        let source = Geometry::Line(Rc::new(Line::from_points(
            &Point::new(0., 0., 0.),
            &Point::new(50., 0., 0.),
        )));
        let grown = extended(&source, End::End, &to(vec![wall(100.)])).unwrap();
        assert!(ends(&grown).1.distance(&Point::new(100., 0., 0.), None) < 1e-9);
        let grown = extended(&source, End::Start, &to(vec![wall(-20.), wall(100.)])).unwrap();
        assert!(ends(&grown).0.distance(&Point::new(-20., 0., 0.), None) < 1e-9);
        let grown = extended(&source, End::Start, &Reach::Distance(25.)).unwrap();
        assert!(ends(&grown).0.distance(&Point::new(-25., 0., 0.), None) < 1e-9);
        assert_eq!(
            extended(&source, End::End, &to(vec![wall(-20.)]))
                .err()
                .as_deref(),
            Some("no boundary ahead")
        );
        let plane = Cutter::Plane(Plane::from_point_normal(
            Point::new(80., 0., 0.),
            Vector::new(1., 0., 0.),
            None,
        ));
        let grown = extended(&source, End::End, &to(vec![plane])).unwrap();
        assert!((ends(&grown).1[0] - 80.0).abs() < 1e-9);
        let block = Rc::new(Mesh::create_box(10., 10., 10.));
        let placed = Xform::translation(-200., 0., 0.); // the box sits at x = -200 in the curve's frame
        let meshes = vec![(block, placed.inverse().unwrap())];
        let grown = extended(
            &source,
            End::Start,
            &Reach::Boundaries {
                cutters: Vec::new(),
                meshes,
            },
        )
        .unwrap();
        assert!(
            (ends(&grown).0[0] + 195.0).abs() < 1e-9,
            "{:?}",
            ends(&grown).0
        );
    }

    /// An open polyline's end moves onto the boundary; a closed one has no free end.
    #[test]
    fn polylines_move_their_end_point() {
        let open = Polyline::new(vec![
            Point::new(0., 10., 0.),
            Point::new(0., 0., 0.),
            Point::new(50., 0., 0.),
        ]);
        let grown = extended(
            &Geometry::Polyline(Rc::new(open)),
            End::End,
            &to(vec![wall(100.)]),
        )
        .unwrap();
        let Geometry::Polyline(grown) = &grown.geometry else {
            panic!()
        };
        assert!(
            grown
                .get_point(2)
                .unwrap()
                .distance(&Point::new(100., 0., 0.), None)
                < 1e-9
        );
        let closed = Polyline::new(vec![
            Point::new(0., 0., 0.),
            Point::new(1., 0., 0.),
            Point::new(1., 1., 0.),
            Point::new(0., 0., 0.),
        ]);
        assert_eq!(
            extended(
                &Geometry::Polyline(Rc::new(closed)),
                End::End,
                &Reach::Distance(1.)
            )
            .err()
            .as_deref(),
            Some("no free end")
        );
    }

    /// A NURBS curve grows by an exact length, smoothly, or to a boundary line.
    #[test]
    fn a_curve_grows_along_its_shape() {
        let curve = NurbsCurve::create(
            false,
            2,
            &[
                Point::new(0., -200., 0.),
                Point::new(50., -150., 0.),
                Point::new(100., -200., 0.),
            ],
        );
        let (_, d1) = curve.domain();
        let before = (curve.point_at(d1), curve.tangent_at(d1));
        let source = Geometry::NurbsCurve(Rc::new(curve.clone()));
        let grown = extended(&source, End::End, &Reach::Distance(20.)).unwrap();
        let Geometry::NurbsCurve(longer) = &grown.geometry else {
            panic!()
        };
        assert!((longer.length(None) - curve.length(None) - 20.0).abs() < 1e-6 * 20.0);
        assert!(longer.point_at(d1).distance(&before.0, None) < 1e-9);
        let (mut after, mut was) = (longer.tangent_at(d1), before.1);
        after.normalize_self();
        was.normalize_self();
        assert!(after.cross(&was).magnitude() < 1e-6);
        let grown = extended(&source, End::End, &to(vec![wall(130.)])).unwrap();
        let Geometry::NurbsCurve(reached) = &grown.geometry else {
            panic!()
        };
        assert!(
            (reached.point_at_end()[0] - 130.0).abs() < 1e-6,
            "{:?}",
            reached.point_at_end()
        );
        assert!(!grown.tail.is_empty());
    }
}
