// --8<-- [start:lasso-spec]
use super::selecting;
use crate::State;
use crate::app::command::tool::cut::planar;
use crate::app::command::tool::{Next, Overlay, Stroke, Tool};
use crate::app::command::{Action, Spec};
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, Mesh, NurbsCurve, NurbsSurface, Plane, Point, Xform};

pub const SPEC: Spec = Spec {
    names: &["Select Lasso"],
    aliases: &[],
    hint: "Select Lasso · drag a loop around objects · Shift adds · Ctrl removes · Esc cancels",
    options: &[],
    arity: Some(0),
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

const SPACING: f64 = 2.0; // device pixels between recorded points
const MAX_POINTS: usize = 4096; // most points kept; a longer loop keeps every other one
const CURVE_STEPS: usize = 64; // steps along a free curve
const EDGE_STEPS: usize = 16; // steps along a BRep edge
const FACE_STEPS: usize = 8; // steps across a curved BRep face, each way
const SURFACE_STEPS: usize = 16; // steps across a surface, each way
const CLOUD_SAMPLES: usize = 65_536; // most points tested per cloud
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
// --8<-- [end:lasso-spec]

// --8<-- [start:lasso-tool]
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

    /// The left button went down: start a new loop; true claims the drag, so the view does not orbit.
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

        let found = inside(state, &Region::new(&points));

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
// --8<-- [end:lasso-tool]

// --8<-- [start:lasso-region]
/// A closed loop indexed by pixel row. Even-odd rule: a point is inside when its row crosses the loop an odd number of times to its left.
struct Region {
    top: f64,            // the first row's top edge, device pixels
    rows: Vec<Vec<f64>>, // crossings per row, ascending
}

impl Region {
    /// Index the loop through `points`; the last point joins the first.
    fn new(points: &[(f64, f64)]) -> Self {
        let top = points
            .iter()
            .map(|p| p.1)
            .fold(f64::INFINITY, f64::min)
            .floor();
        let bottom = points
            .iter()
            .map(|p| p.1)
            .fold(f64::NEG_INFINITY, f64::max)
            .ceil();
        let mut rows = vec![Vec::new(); (bottom - top).max(0.0) as usize + 1];

        for (index, a) in points.iter().enumerate() {
            let b = points[(index + 1) % points.len()];
            let (low, high) = (a.1.min(b.1), a.1.max(b.1));
            let mut row = (low - top - 0.5).ceil().max(0.0) as usize;

            // rows whose centre lies in [low, high); a flat edge crosses none
            while row < rows.len() && top + row as f64 + 0.5 < high {
                let y = top + row as f64 + 0.5;
                rows[row].push(a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1));
                row += 1;
            }
        }

        for row in &mut rows {
            row.sort_by(f64::total_cmp);
        }

        Self { top, rows }
    }

    /// True when `at` is inside by the even-odd rule, judged on its row's centre line.
    fn contains(&self, at: (f64, f64)) -> bool {
        let row = (at.1 - self.top).floor();

        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1 // partition_point counts the crossings left of `at` by binary search on the sorted row
    }
}

/// Device pixels of `p` under the clip matrix `m`; None behind the eye.
fn screen(m: &[f64; 16], p: [f64; 3], size: (f64, f64)) -> Option<(f64, f64)> {
    let clip = |r: usize| m[r] * p[0] + m[r + 4] * p[1] + m[r + 8] * p[2] + m[r + 12];
    let w = clip(3);
    (w > 0.0).then(|| {
        (
            (clip(0) / w * 0.5 + 0.5) * size.0,
            (0.5 - clip(1) / w * 0.5) * size.1,
        )
    })
}
// --8<-- [end:lasso-region]

// --8<-- [start:lasso-inside]
/// Selectable rows whose samples all land inside `region`, ascending.
fn inside(state: &State, region: &Region) -> Vec<u32> {
    let origin = state.camera.origin();
    let view = &state.camera.view_proj_anchored(state.aspect(), &origin)
        * &Xform::translation(-origin[0], -origin[1], -origin[2]); // world to clip
    let size = state.viewport();
    let within =
        |m: &[f64; 16], p: [f64; 3]| screen(m, p, size).is_some_and(|at| region.contains(at));

    selecting::candidates(state)
        .into_iter()
        .filter(|&row| {
            let sampled = state
                .scene
                .geometry(row)
                .zip(state.scene.placement_of(row))
                .and_then(|(geometry, place)| {
                    let local = (&view * &place).m; // object to clip
                    enclosed(geometry, &|p| within(&local, p))
                });

            // without samples, the world box's corners decide
            sampled.unwrap_or_else(|| {
                state.gpu.objects.row_bounds(row).is_some_and(|bounds| {
                    bounds
                        .corners()
                        .iter()
                        .all(|corner| within(&view.m, xyz(corner)))
                })
            })
        })
        .collect()
}

/// Whether every sample, in the object's own frame, passes `inside`, any function or closure taking a point; None when only its box can tell.
fn enclosed(geometry: &Geometry, inside: &dyn Fn([f64; 3]) -> bool) -> Option<bool> {
    match geometry {
        Geometry::Point(point) => every(std::iter::once(xyz(point)), inside),
        Geometry::Line(line) => every([xyz(&line.start()), xyz(&line.end())].into_iter(), inside),
        Geometry::Polyline(polyline) => every(
            polyline.coords.chunks_exact(3).map(|c| [c[0], c[1], c[2]]),
            inside,
        ),
        Geometry::NurbsCurve(curve) => every(curve_samples(curve, CURVE_STEPS), inside),
        Geometry::NurbsSurface(surface) => every(surface_samples(surface, SURFACE_STEPS), inside),
        Geometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
        Geometry::BRep(brep) => every(brep_samples(brep), inside),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => every(mesh_samples(mesh), inside),
            ElementGeometry::BRep(brep) => every(brep_samples(brep), inside),
            ElementGeometry::None => None,
        },
        Geometry::PointCloud(cloud) => {
            let coords = cloud.coords();
            let step = (coords.len() / 3).div_ceil(CLOUD_SAMPLES).max(1);
            every(
                coords
                    .chunks_exact(3)
                    .step_by(step)
                    .map(|c| [c[0], c[1], c[2]]),
                inside,
            )
        }
        Geometry::Plane(_) | Geometry::OBB(_) => None,
    }
}

/// Some(true) when every sample passes, Some(false) at the first that fails, None when there are none.
fn every(
    samples: impl Iterator<Item = [f64; 3]>,
    inside: &dyn Fn([f64; 3]) -> bool,
) -> Option<bool> {
    let mut any = false;

    for sample in samples {
        if !inside(sample) {
            return Some(false);
        }

        any = true;
    }

    any.then_some(true)
}

/// A point as three numbers.
fn xyz(point: &Point) -> [f64; 3] {
    [point[0], point[1], point[2]]
}

/// A straight curve's corners, else `steps` even steps. `impl Iterator + '_`: an iterator borrowing the curve, making samples one at a time, never stored.
fn curve_samples(curve: &NurbsCurve, steps: usize) -> impl Iterator<Item = [f64; 3]> + '_ {
    let straight = curve.degree() == 1;
    let (t0, t1) = curve.domain();
    let corners = (0..if straight { curve.cv_count() } else { 0 }).filter_map(|i| curve.get_cv(i));
    let even = (0..if straight { 0 } else { steps + 1 })
        .map(move |i| curve.point_at(t0 + (t1 - t0) * i as f64 / steps as f64));
    corners.chain(even).map(|p| xyz(&p))
}

/// A grid of `steps` by `steps` even steps over the surface's domain.
fn surface_samples(surface: &NurbsSurface, steps: usize) -> impl Iterator<Item = [f64; 3]> + '_ {
    let domains = surface.domain(0).zip(surface.domain(1));
    let ((u0, u1), (v0, v1)) = domains.unwrap_or_default();
    let side = if domains.is_some() { steps + 1 } else { 0 };
    (0..side * side)
        .filter_map(move |k| {
            let u = u0 + (u1 - u0) * (k / side) as f64 / steps as f64;
            let v = v0 + (v1 - v0) * (k % side) as f64 / steps as f64;
            surface.point_at(u, v)
        })
        .map(|p| xyz(&p))
}

/// Every vertex of a mesh.
fn mesh_samples(mesh: &Mesh) -> impl Iterator<Item = [f64; 3]> + '_ {
    mesh.vertex.values().map(|v| [v.x, v.y, v.z])
}

/// Vertices, edges and curved faces; a flat face is bounded by its edges.
fn brep_samples(brep: &BRep) -> impl Iterator<Item = [f64; 3]> + '_ {
    let vertices = brep.m_vertices.iter().map(|vertex| xyz(&vertex.point));
    let edges = brep
        .m_curves_3d
        .iter()
        .flat_map(|curve| curve_samples(curve, EDGE_STEPS));
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_samples(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces)
}
// --8<-- [end:lasso-inside]

// --8<-- [start:lasso-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{AABB, Line, OBB, Polyline, Vector};
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
// --8<-- [end:lasso-tests]
