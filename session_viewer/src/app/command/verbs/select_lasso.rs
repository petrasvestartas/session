use super::selecting::{self, Reach, Region};
use crate::State;
use crate::app::command::tool::{Next, Overlay, Stroke, Tool};
use crate::app::command::{Action, Spec};
use session_rust::{Plane, Point};

pub const SPEC: Spec = Spec {
    arity: Some(0),
    ..Spec::new(
        &["Select Lasso"],
        "Select Lasso · drag a loop around objects · Shift adds · Ctrl removes · Esc cancels",
        parse,
    )
};

const SPACING: f64 = 2.0; // device pixels between recorded points
const MAX_POINTS: usize = 4096; // most points kept; a longer loop keeps every other one
const BLUE: [u8; 3] = [30, 110, 170];

/// Arm the lasso.
fn parse(_verb: &str, _rest: &[&str]) -> Result<Box<dyn Action>, String> {
    Ok(Box::new(SelectLasso))
}

#[derive(Debug)]
struct SelectLasso;

impl Action for SelectLasso {
    /// The next left drag draws the loop.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.open_tool(Box::new(Lasso::default()))
    }
}

/// The loop being drawn, device pixels.
#[derive(Debug, Default)]
struct Lasso {
    points: Vec<(f64, f64)>, // device pixels
    spacing: f64,            // device pixels between points, doubled each time the loop fills up
}

impl Lasso {
    /// Record `at` when it is far enough from the last point; true when the loop grew.
    fn follow(&mut self, at: (f64, f64)) -> bool {
        let Some(last) = self.points.last() else {
            return false;
        };

        if (at.0 - last.0).hypot(at.1 - last.1) < self.spacing.max(SPACING) {
            return false;
        }

        // full: thin the loop evenly instead of cutting its end off
        if self.points.len() >= MAX_POINTS {
            let mut index = 0;
            self.points.retain(|_| {
                index += 1;
                index % 2 == 1
            });
            self.spacing = 2.0 * self.spacing.max(SPACING);
        }

        self.points.push(at);
        true
    }
}

impl Tool for Lasso {
    fn name(&self) -> &'static str {
        "Select Lasso"
    }

    fn prompt(&self, _points: &[Point]) -> String {
        "drag a loop around objects · Shift adds · Ctrl removes".into()
    }

    fn asks_points(&self) -> bool {
        false
    }

    fn placed(
        &mut self,
        _state: &mut State,
        _points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        Ok(Next::More)
    }

    /// The cursor alone snaps nothing.
    fn hovered(&mut self, _state: &mut State, _at: (f64, f64)) -> Option<bool> {
        Some(false)
    }

    fn guide(&self, _points: &[Point], _cursor: Option<&Point>) -> Vec<Point> {
        Vec::new()
    }

    fn pressed(&mut self, _state: &mut State, at: (f64, f64)) -> bool {
        self.points.clear();
        self.points.push(at);
        self.spacing = SPACING;
        true
    }

    fn dragged(&mut self, _state: &mut State, at: (f64, f64)) -> bool {
        self.follow(at)
    }

    /// Close the loop and select what lies wholly inside; a click keeps the lasso armed.
    fn released(&mut self, state: &mut State, add: bool, remove: bool) -> Result<Next, String> {
        let points = std::mem::take(&mut self.points);

        if points.len() < 3 {
            return Ok(Next::More);
        }

        let found = selecting::found(state, &Region::new(&points), Reach::Window);

        if found.is_empty() {
            return Err(
                "nothing visible lies entirely inside the lasso; the selection is unchanged".into(),
            );
        }

        let count = found.len();
        let total = selecting::apply(state, found, add && !remove, remove);

        if add || remove {
            return Ok(Next::Done(format!(
                "{count} inside the lasso · {total} selected"
            )));
        }

        Ok(Next::Done(format!("{total} selected inside the lasso")))
    }

    fn abandoned(&mut self) {
        self.points.clear();
    }

    /// The loop so far, closed back to its start.
    fn marks(&self, _state: &State) -> Option<Overlay> {
        let first = *self.points.first()?;

        if self.points.len() < 2 {
            return None;
        }

        let mut points = self.points.clone();
        points.push(first);
        Some(Overlay {
            strokes: vec![Stroke {
                points,
                color: BLUE,
                width: 1.5,
                dashed: false,
            }],
            ..Default::default()
        })
    }

    fn status(&self) -> serde_json::Value {
        serde_json::json!({ "command": self.name(), "outline": self.points.len() })
    }
}

#[cfg(test)]
mod tests {
    use super::super::selecting::{EDGE_STEPS, curve_samples, enclosed};
    use super::*;
    use session_rust::{AABB, BRep, Geometry, Line, Mesh, NurbsCurve, OBB, Polyline, Vector};
    use std::cell::Cell;

    /// Inside the square 0..10 in x and y.
    fn square(p: [f64; 3]) -> bool {
        (0.0..=10.0).contains(&p[0]) && (0.0..=10.0).contains(&p[1])
    }

    /// Even-odd by brute force: crossings left of `at` on its row's centre line.
    fn ray_cast(points: &[(f64, f64)], at: (f64, f64)) -> bool {
        let y = at.1.floor() + 0.5;
        let mut count = 0;

        for (index, a) in points.iter().enumerate() {
            let b = points[(index + 1) % points.len()];

            if (a.1 <= y && y < b.1) || (b.1 <= y && y < a.1) {
                let x = a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1);
                count += usize::from(x < at.0);
            }
        }

        count % 2 == 1
    }

    /// A square holds its centre; a U rejects its notch; a bow-tie follows even-odd; a flat loop holds nothing.
    #[test]
    fn a_loop_holds_what_it_surrounds() {
        let region = Region::new(&[(10.0, 10.0), (50.0, 10.0), (50.0, 50.0), (10.0, 50.0)]);
        assert!(region.contains((30.0, 30.0)));
        assert!(!region.contains((60.0, 30.0)));
        assert!(!region.contains((30.0, 5.0)));
        assert!(!region.contains((30.0, 55.0)));

        let u = Region::new(&[
            (0.0, 0.0),
            (10.0, 0.0),
            (10.0, 30.0),
            (20.0, 30.0),
            (20.0, 0.0),
            (30.0, 0.0),
            (30.0, 40.0),
            (0.0, 40.0),
        ]);
        assert!(!u.contains((15.0, 10.0)), "the notch");
        assert!(u.contains((5.0, 10.0)));
        assert!(u.contains((15.0, 35.0)));

        let tie = Region::new(&[(0.0, 0.0), (10.0, 10.0), (10.0, 0.0), (0.0, 10.0)]);
        assert!(tie.contains((2.0, 5.0)));
        assert!(tie.contains((8.0, 5.0)));
        assert!(!tie.contains((5.0, 2.0)));

        let flat = Region::new(&[(0.0, 0.0), (5.0, 5.0), (10.0, 10.0)]);

        for at in [(5.0, 5.0), (5.5, 5.5), (2.0, 2.0), (7.0, 3.0)] {
            assert!(!flat.contains(at), "{at:?}");
        }
    }

    /// The row index answers like a ray cast over a twelve-point star.
    #[test]
    fn the_scanline_index_agrees_with_ray_casting() {
        let star: Vec<(f64, f64)> = (0..12)
            .map(|i| {
                let angle = i as f64 * std::f64::consts::PI / 6.0;
                let radius = if i % 2 == 0 { 100.0 } else { 40.0 };
                (150.0 + radius * angle.cos(), 150.0 + radius * angle.sin())
            })
            .collect();
        let region = Region::new(&star);
        let mut inside = 0;

        for x in 0..300 {
            for y in 0..300 {
                let at = (x as f64 + 0.25, y as f64 + 0.5);
                assert_eq!(region.contains(at), ray_cast(&star, at), "{at:?}");
                inside += usize::from(region.contains(at));
            }
        }

        assert!(inside > 10_000, "{inside}");
    }

    /// Points, lines, polylines, curves and meshes pass only with every sample inside.
    #[test]
    fn geometry_is_inside_only_when_every_sample_is() {
        let shape = |geometry: Geometry| enclosed(&geometry, &square);
        assert_eq!(
            shape(Geometry::Point(Point::new(5.0, 5.0, 0.0).into())),
            Some(true)
        );
        assert_eq!(
            shape(Geometry::Point(Point::new(15.0, 5.0, 0.0).into())),
            Some(false)
        );
        assert_eq!(
            shape(Geometry::Line(
                Line::new(1.0, 1.0, 0.0, 9.0, 9.0, 0.0).into()
            )),
            Some(true)
        );
        assert_eq!(
            shape(Geometry::Line(
                Line::new(1.0, 1.0, 0.0, 11.0, 9.0, 0.0).into()
            )),
            Some(false)
        );
        let corners = vec![
            Point::new(1.0, 1.0, 0.0),
            Point::new(9.0, 1.0, 0.0),
            Point::new(9.0, 9.0, 0.0),
        ];
        assert_eq!(
            shape(Geometry::Polyline(Polyline::new(corners.clone()).into())),
            Some(true)
        );
        let mut out = corners;
        out.push(Point::new(-1.0, 9.0, 0.0));
        assert_eq!(
            shape(Geometry::Polyline(Polyline::new(out).into())),
            Some(false)
        );
        assert_eq!(
            shape(Geometry::Polyline(Polyline::new(Vec::new()).into())),
            None
        );

        let bulge = [
            Point::new(1.0, 1.0, 0.0),
            Point::new(5.0, 30.0, 0.0),
            Point::new(9.0, 1.0, 0.0),
        ];
        assert_eq!(
            shape(Geometry::NurbsCurve(
                NurbsCurve::create(false, 2, &bulge).into()
            )),
            Some(false)
        );
        let low = [
            Point::new(1.0, 1.0, 0.0),
            Point::new(5.0, 9.0, 0.0),
            Point::new(9.0, 1.0, 0.0),
        ];
        assert_eq!(
            shape(Geometry::NurbsCurve(
                NurbsCurve::create(false, 2, &low).into()
            )),
            Some(true)
        );

        let shifted = |p: [f64; 3]| square([p[0] + 5.0, p[1] + 5.0, p[2]]);
        let small = Geometry::Mesh(Mesh::create_box(4.0, 4.0, 4.0).into());
        let large = Geometry::Mesh(Mesh::create_box(20.0, 20.0, 20.0).into());
        assert_eq!(enclosed(&small, &shifted), Some(true));
        assert_eq!(enclosed(&large, &shifted), Some(false));
    }

    /// A sphere's single face is judged by its surface, not by its seam; a box's flat faces add nothing.
    #[test]
    fn a_curved_face_is_judged_by_its_surface() {
        let sphere = BRep::create_sphere(5.0);
        let half = |p: [f64; 3]| p[0] >= -0.1;
        let seam: Vec<_> = sphere
            .m_curves_3d
            .iter()
            .flat_map(|curve| curve_samples(curve, EDGE_STEPS))
            .collect();
        assert!(
            !seam.is_empty() && seam.iter().all(|p| half(*p)),
            "the seam alone passes"
        );
        assert_eq!(enclosed(&Geometry::BRep(sphere.into()), &half), Some(false));

        let boxed = BRep::create_box(2.0, 2.0, 2.0);
        let calls = Cell::new(0);
        let wide = |p: [f64; 3]| {
            calls.set(calls.get() + 1);
            p.iter().all(|value| value.abs() < 100.0)
        };
        let straight = boxed.m_curves_3d.iter().all(|curve| curve.degree() == 1);
        assert_eq!(enclosed(&Geometry::BRep(boxed.into()), &wide), Some(true));
        assert!(straight, "a box's edges are straight");
        assert_eq!(
            calls.get(),
            8 + 12 * 2,
            "vertices and edge ends, no face samples"
        );
    }

    /// Planes and frames have no samples of their own.
    #[test]
    fn planes_and_frames_fall_back_to_their_box() {
        let plane =
            Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0), None);
        assert_eq!(enclosed(&Geometry::Plane(plane.into()), &square), None);
        let frame = OBB::from_aabb(&AABB::new(5.0, 5.0, 0.0, 1.0, 1.0, 1.0));
        assert_eq!(enclosed(&Geometry::OBB(frame.into()), &square), None);
    }

    /// Points closer than the spacing are skipped; a full loop thins out but keeps its start and end.
    #[test]
    fn a_lasso_records_points_apart() {
        let mut lasso = Lasso::default();
        assert!(!lasso.follow((1.0, 0.0)), "nothing pressed yet");
        lasso.points.push((0.0, 0.0));
        assert!(!lasso.follow((1.0, 0.0)));
        assert!(lasso.follow((5.0, 0.0)));
        assert_eq!(lasso.points.len(), 2);

        for i in 0..20_000 {
            lasso.follow((8.0 + 3.0 * i as f64, 0.0));
        }

        let end = 8.0 + 3.0 * 19_999.0;
        assert!(lasso.points.len() <= MAX_POINTS, "{}", lasso.points.len());
        assert_eq!(lasso.points[0], (0.0, 0.0));
        assert!(
            end - lasso.points.last().unwrap().0 < lasso.spacing,
            "the end is kept"
        );
        lasso.abandoned();
        assert!(lasso.points.is_empty());
    }

    /// The lasso is armed by its bare name.
    #[test]
    fn select_lasso_takes_no_arguments() {
        use crate::app::command::{accept, parse};
        assert!(parse("Select Lasso").is_ok());
        assert!(parse("selectlasso").is_ok());
        assert!(parse("Select Lasso x").is_err());
        assert_eq!(accept("select l"), ("Select Lasso".into(), true));
    }
}
