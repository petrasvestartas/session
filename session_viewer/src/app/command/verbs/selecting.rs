use crate::State;
use crate::app::command::tool::cut::planar;
use crate::engine::gpu::Instance;
use session_rust::element::ElementGeometry;
use session_rust::{BRep, Geometry, Mesh, NurbsCurve, NurbsSurface, Point, Xform};

const CURVE_STEPS: usize = 64; // steps along a free curve
pub(crate) const EDGE_STEPS: usize = 16; // steps along a BRep edge
const FACE_STEPS: usize = 8; // steps across a curved BRep face, each way
const SURFACE_STEPS: usize = 16; // steps across a surface, each way
const CLOUD_SAMPLES: usize = 65_536; // most points tested per cloud

/// Rows a selection command may take: shown, not hidden by H or a layer, not locked; ascending.
pub fn candidates(state: &State) -> Vec<u32> {
    let scene = &state.scene;
    (0..scene.row_count() as u32)
        .filter(|&row| {
            scene.selectable(row)
                && scene
                    .identity_of(row)
                    .is_some_and(|id| !scene.hidden.contains(&id))
                && state
                    .gpu
                    .objects
                    .row(row)
                    .is_some_and(|object| object.flags & Instance::FLAG_HIDDEN == 0)
        })
        .collect()
}

/// Select `found` (ascending) instead of the selection, added to it, or taken out of it; the count selected after.
pub fn apply(state: &mut State, found: Vec<u32>, add: bool, remove: bool) -> usize {
    if remove {
        let mut keep = state.selected_rows();
        keep.retain(|row| found.binary_search(row).is_err());
        state.select_rows(keep, false);
    } else {
        state.select_rows(found, add);
    }

    state.selected_rows().len()
}

/// A closed loop indexed by pixel row: where each row's centre line crosses it, for the even-odd test.
pub(crate) struct Region {
    top: f64,               // the first row's top edge, device pixels
    rows: Vec<Vec<f64>>,    // crossings per row, ascending
    box_: Option<[f64; 4]>, // left, top, right, bottom when the loop is a rectangle
}

/// What a region takes: what lies wholly inside it, or whatever it touches.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Reach {
    Window,   // every sample inside
    Crossing, // a sample inside, or a stroke across the rectangle
}

impl Region {
    /// Index the loop through `points`; the last point joins the first.
    pub(crate) fn new(points: &[(f64, f64)]) -> Self {
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

        Self {
            top,
            rows,
            box_: None,
        }
    }

    /// The rectangle with corners `a` and `b`, device pixels.
    pub(crate) fn rectangle(a: (f64, f64), b: (f64, f64)) -> Self {
        let (left, right) = (a.0.min(b.0), a.0.max(b.0));
        let (top, bottom) = (a.1.min(b.1), a.1.max(b.1));
        Self {
            box_: Some([left, top, right, bottom]),
            ..Self::new(&[(left, top), (right, top), (right, bottom), (left, bottom)])
        }
    }

    /// True when the segment from `a` to `b` meets the rectangle; false for a loop that is no rectangle.
    fn crosses(&self, a: (f64, f64), b: (f64, f64)) -> bool {
        let Some([left, top, right, bottom]) = self.box_ else {
            return false;
        };
        // clip the segment's parameter range to each side in turn (Liang-Barsky)
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let (mut t0, mut t1) = (0.0_f64, 1.0_f64);

        for (p, q) in [
            (-dx, a.0 - left),
            (dx, right - a.0),
            (-dy, a.1 - top),
            (dy, bottom - a.1),
        ] {
            if p == 0.0 {
                if q < 0.0 {
                    return false;
                }
                continue;
            }

            let t = q / p;

            if p < 0.0 {
                t0 = t0.max(t);
            } else {
                t1 = t1.min(t);
            }
        }

        t0 <= t1
    }

    /// True when `at` is inside by the even-odd rule, judged on its row's centre line.
    pub(crate) fn contains(&self, at: (f64, f64)) -> bool {
        let row = (at.1 - self.top).floor();

        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
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

/// Selectable rows the region takes, ascending: wholly inside it for a window, touching it for a crossing.
pub(crate) fn found(state: &State, region: &Region, reach: Reach) -> Vec<u32> {
    let origin = state.camera.origin();
    let view = &state.camera.view_proj_anchored(state.aspect(), &origin)
        * &Xform::translation(-origin[0], -origin[1], -origin[2]); // world to clip
    let size = state.viewport();
    let within =
        |m: &[f64; 16], p: [f64; 3]| screen(m, p, size).is_some_and(|at| region.contains(at));

    candidates(state)
        .into_iter()
        .filter(|&row| {
            let sampled = state
                .scene
                .geometry(row)
                .zip(state.scene.placement_of(row))
                .and_then(|(geometry, place)| {
                    let local = (&view * &place).m; // object to clip
                    match reach {
                        Reach::Window => enclosed(geometry, &|p| within(&local, p)),
                        Reach::Crossing => touched(geometry, &|p| screen(&local, p, size), region),
                    }
                });

            // without samples, the world box's corners decide
            sampled.unwrap_or_else(|| {
                state.gpu.objects.row_bounds(row).is_some_and(|bounds| {
                    let corners: Vec<Option<(f64, f64)>> = bounds
                        .corners()
                        .iter()
                        .map(|corner| screen(&view.m, xyz(corner), size))
                        .collect();
                    match reach {
                        Reach::Window => corners
                            .iter()
                            .all(|at| at.is_some_and(|at| region.contains(at))),
                        Reach::Crossing => box_touches(&corners, region),
                    }
                })
            })
        })
        .collect()
}

/// Whether the screen box around projected corners overlaps the region's rectangle.
fn box_touches(corners: &[Option<(f64, f64)>], region: &Region) -> bool {
    let (Some([left, top, right, bottom]), Some(first)) =
        (region.box_, corners.iter().flatten().next())
    else {
        return false;
    };
    let mut lo = *first;
    let mut hi = *first;

    for at in corners.iter().flatten() {
        lo = (lo.0.min(at.0), lo.1.min(at.1));
        hi = (hi.0.max(at.0), hi.1.max(at.1));
    }

    lo.0 <= right && hi.0 >= left && lo.1 <= bottom && hi.1 >= top
}

/// Whether `geometry`, its points projected by `project`, has a point inside the region or a stroke across its rectangle; None when only its box can tell.
pub(crate) fn touched(
    geometry: &Geometry,
    project: &dyn Fn([f64; 3]) -> Option<(f64, f64)>,
    region: &Region,
) -> Option<bool> {
    let strokes = strokes(geometry)?;
    let mut any = false;

    for stroke in &strokes {
        let points: Vec<Option<(f64, f64)>> = stroke.iter().map(|p| project(*p)).collect();

        if points.iter().flatten().any(|at| region.contains(*at)) {
            return Some(true);
        }

        for pair in points.windows(2) {
            if let [Some(a), Some(b)] = pair
                && region.crosses(*a, *b)
            {
                return Some(true);
            }
        }

        any |= !points.is_empty();
    }

    any.then_some(false)
}

/// The geometry as strokes in its own frame, each a run of points joined in order; None for a plane or a frame.
fn strokes(geometry: &Geometry) -> Option<Vec<Vec<[f64; 3]>>> {
    let strokes = match geometry {
        Geometry::Point(point) => vec![vec![xyz(point)]],
        Geometry::Line(line) => vec![vec![xyz(&line.start()), xyz(&line.end())]],
        Geometry::Polyline(polyline) => vec![
            polyline
                .coords
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect(),
        ],
        Geometry::NurbsCurve(curve) => vec![curve_samples(curve, CURVE_STEPS).collect()],
        Geometry::NurbsSurface(surface) => surface_strokes(surface, SURFACE_STEPS),
        Geometry::Mesh(mesh) => mesh_strokes(mesh),
        Geometry::BRep(brep) => brep_strokes(brep),
        Geometry::Element(element) => match element.geometry() {
            ElementGeometry::Mesh(mesh) => mesh_strokes(mesh),
            ElementGeometry::BRep(brep) => brep_strokes(brep),
            ElementGeometry::None => return None,
        },
        Geometry::PointCloud(cloud) => {
            let coords = cloud.coords();
            let step = (coords.len() / 3).div_ceil(CLOUD_SAMPLES).max(1);
            coords
                .chunks_exact(3)
                .step_by(step)
                .map(|c| vec![[c[0], c[1], c[2]]])
                .collect()
        }
        Geometry::Plane(_) | Geometry::OBB(_) => return None,
    };

    Some(strokes)
}

/// The surface's grid lines both ways, `steps` steps each.
fn surface_strokes(surface: &NurbsSurface, steps: usize) -> Vec<Vec<[f64; 3]>> {
    let Some(((u0, u1), (v0, v1))) = surface.domain(0).zip(surface.domain(1)) else {
        return Vec::new();
    };
    let at = |i: usize, j: usize| {
        let u = u0 + (u1 - u0) * i as f64 / steps as f64;
        let v = v0 + (v1 - v0) * j as f64 / steps as f64;
        surface.point_at(u, v).map(|p| xyz(&p))
    };
    let rows = (0..=steps).map(|i| (0..=steps).filter_map(|j| at(i, j)).collect());
    let columns = (0..=steps).map(|j| (0..=steps).filter_map(|i| at(i, j)).collect());
    rows.chain(columns).collect()
}

/// Every face of a mesh as its closed outline.
fn mesh_strokes(mesh: &Mesh) -> Vec<Vec<[f64; 3]>> {
    mesh.face
        .values()
        .map(|face| {
            let mut loop_: Vec<[f64; 3]> = face
                .iter()
                .filter_map(|key| mesh.vertex.get(key).map(|v| [v.x, v.y, v.z]))
                .collect();
            loop_.extend(loop_.first().copied());
            loop_
        })
        .collect()
}

/// Vertices, edges and the grid lines of curved faces.
fn brep_strokes(brep: &BRep) -> Vec<Vec<[f64; 3]>> {
    let vertices = brep
        .m_vertices
        .iter()
        .map(|vertex| vec![xyz(&vertex.point)]);
    let edges = brep
        .m_curves_3d
        .iter()
        .map(|curve| curve_samples(curve, EDGE_STEPS).collect());
    let faces = brep
        .m_surfaces
        .iter()
        .filter(|surface| planar(surface).is_none())
        .flat_map(|surface| surface_strokes(surface, FACE_STEPS));
    vertices.chain(edges).chain(faces).collect()
}

/// Whether every sample of `geometry`, in its own frame, passes `inside`; None when only its box can tell.
pub(crate) fn enclosed(geometry: &Geometry, inside: &dyn Fn([f64; 3]) -> bool) -> Option<bool> {
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

/// A straight curve's corners, else `steps` even steps over its domain.
pub(crate) fn curve_samples(
    curve: &NurbsCurve,
    steps: usize,
) -> impl Iterator<Item = [f64; 3]> + '_ {
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

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Line;

    /// The rectangle 10..50 by 10..50, the plan seen from above one pixel per unit.
    fn rectangle() -> Region {
        Region::rectangle((50.0, 50.0), (10.0, 10.0))
    }

    fn plan(p: [f64; 3]) -> Option<(f64, f64)> {
        Some((p[0], p[1]))
    }

    /// Either corner first gives one rectangle; segments meet it inside, across or not at all.
    #[test]
    fn a_rectangle_holds_and_crosses() {
        let region = rectangle();
        assert!(region.contains((30.0, 30.0)) && !region.contains((60.0, 30.0)));
        assert!(
            region.crosses((0.0, 30.0), (60.0, 30.0)),
            "straight through"
        );
        assert!(region.crosses((20.0, 20.0), (25.0, 25.0)), "inside");
        assert!(!region.crosses((0.0, 0.0), (0.0, 60.0)), "beside");
        assert!(!region.crosses((0.0, 70.0), (70.0, 55.0)), "below");
        assert!(
            !Region::new(&[(0.0, 0.0), (9.0, 0.0), (9.0, 9.0)]).crosses((0.0, 5.0), (9.0, 5.0)),
            "a lasso has no rectangle"
        );
    }

    /// A line half inside: a crossing takes it, a window does not; a line through with both ends out is a crossing too.
    #[test]
    fn a_window_takes_what_is_inside_a_crossing_what_it_touches() {
        let region = rectangle();
        let window = |geometry: &Geometry| {
            enclosed(geometry, &|p| plan(p).is_some_and(|at| region.contains(at)))
        };
        let crossing = |geometry: &Geometry| touched(geometry, &plan, &region);

        let half = Geometry::Line(Line::new(30.0, 30.0, 0.0, 80.0, 30.0, 0.0).into());
        assert_eq!((window(&half), crossing(&half)), (Some(false), Some(true)));

        let through = Geometry::Line(Line::new(0.0, 30.0, 0.0, 80.0, 30.0, 0.0).into());
        assert_eq!(
            (window(&through), crossing(&through)),
            (Some(false), Some(true))
        );

        let inside = Geometry::Line(Line::new(20.0, 20.0, 0.0, 40.0, 40.0, 0.0).into());
        assert_eq!(
            (window(&inside), crossing(&inside)),
            (Some(true), Some(true))
        );

        let away = Geometry::Line(Line::new(60.0, 60.0, 0.0, 90.0, 90.0, 0.0).into());
        assert_eq!((window(&away), crossing(&away)), (Some(false), Some(false)));

        // a box bigger than the rectangle: its edges cross it
        let around = Geometry::Mesh(Mesh::create_box(100.0, 100.0, 1.0).into());
        let shifted = |p: [f64; 3]| plan([p[0] + 30.0, p[1] + 30.0, p[2]]);
        assert_eq!(
            touched(&around, &shifted, &region),
            Some(false),
            "only the outline; the rectangle sits inside the faces"
        );
        let crossing_box = |p: [f64; 3]| plan([p[0] + 80.0, p[1] + 30.0, p[2]]);
        assert_eq!(touched(&around, &crossing_box, &region), Some(true));
    }
}
