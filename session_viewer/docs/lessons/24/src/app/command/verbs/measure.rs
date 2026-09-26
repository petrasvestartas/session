// --8<-- [start:mark-target]
use crate::State;
use crate::app::command::tool::{Overlay, Stroke};
use crate::app::selection::SelectionMode;
use crate::camera::Unit;
use session_rust::{Geometry, Mesh, Point, Vector, Xform};

/// A measured answer drawn in the scene until the next command, Esc or a change of rows.
// A mark remembers the scene's row revision, which grows on every change; once they differ the objects may have moved and it hides.
pub(crate) struct Mark {
    points: Vec<Point>, // world points, joined by a line
    label: String,      // the value beside them
    revision: u64,      // the scene rows it was measured on
}

/// One selected object to measure.
// `'a` ties each Target to the State it borrows the geometry from, so no kernel object is copied to measure it.
pub(crate) struct Target<'a> {
    pub row: u32,               // scene row
    pub geometry: &'a Geometry, // its kernel object
    pub place: Xform,           // local to world
    pub face: Option<usize>,    // the chosen face, when a face is selected
}

/// The selected objects with geometry, and how many selected rows have none (texts, sheets).
pub(crate) fn selected(state: &State) -> (Vec<Target<'_>>, usize) {
    let rows = state.selected_rows();
    let face = match state.selection {
        SelectionMode::Face { parent, face } => Some((parent, face)),
        _ => None,
    };
    let targets: Vec<Target> = rows
        .iter()
        .filter_map(|&row| {
            Some(Target {
                row,
                geometry: state.scene.geometry(row)?,
                place: state.scene.placement_of(row)?,
                face: face
                    .filter(|(parent, _)| *parent == row)
                    .map(|(_, face)| face),
            })
        })
        .collect();
    let skipped = rows.len() - targets.len();
    (targets, skipped)
}

/// Ask for released documents of the selection back; the error says to try again.
pub(crate) fn loading(state: &mut State) -> Result<(), String> {
    // A released document (lesson 16) keeps no kernel objects in memory, so it is fetched back before anything is measured.
    let docs: Vec<usize> = state
        .selected_rows()
        .iter()
        .filter_map(|&row| state.scene.released_doc(row))
        .collect();

    let Some(&doc) = docs.first() else {
        return Ok(());
    };

    for &doc in &docs {
        state.scene.want(doc);
    }

    state.fetch_wanted();
    Err(format!(
        "Loading '{}' to measure it; try again in a moment",
        state.scene.docs[doc].name
    ))
}
// --8<-- [end:mark-target]

// --8<-- [start:face-area]
/// Visit the triangles the viewer draws for face `face`, in world coordinates: the cached triangulation, else a fan.
pub(crate) fn for_each_triangle(
    mesh: &Mesh,
    face: usize,
    place: &Xform,
    visit: impl FnMut([Point; 3]),
) {
    // A fan = triangles from the first corner to each next pair of corners, the simplest way to cut a polygon.
    let at = |key: usize| mesh.vertex_point(key).map(|p| p.transformed(place));
    let triangle = |[a, b, c]: [usize; 3]| Some([at(a)?, at(b)?, at(c)?]);

    if let Some(cached) = mesh
        .triangulation
        .get(&face)
        .filter(|tris| !tris.is_empty())
    {
        cached
            .iter()
            .filter_map(|&keys| triangle(keys))
            .for_each(visit);
        return;
    }

    let corners = mesh.face.get(&face).map_or(&[][..], Vec::as_slice);
    (1..corners.len().saturating_sub(1))
        .filter_map(|i| triangle([corners[0], corners[i], corners[i + 1]]))
        .for_each(visit);
}

/// Area of one mesh face in world units; a fan that folds back (a concave polygon) sums with signs, so it stays exact.
pub(crate) fn compute_face_area(mesh: &Mesh, face: usize, place: &Xform) -> f64 {
    let cached = mesh
        .triangulation
        .get(&face)
        .is_some_and(|tris| !tris.is_empty());
    // A triangle's cross product is as long as twice its area; summed as vectors, the parts where a concave fan overlaps cancel.
    let mut summed = Vector::new(0.0, 0.0, 0.0); // signed fan normals
    let mut unsigned = 0.0; // triangle areas, doubled

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        let cross = (&b - &a).cross(&(&c - &a));
        unsigned += cross.magnitude();
        summed += cross;
    });

    if cached || unsigned - summed.magnitude() <= 1e-12 * unsigned {
        return unsigned * 0.5;
    }

    // a warped face whose fan never folds back is the drawn area
    let mut folded = false;

    for_each_triangle(mesh, face, place, |[a, b, c]| {
        folded |= (&b - &a).cross(&(&c - &a)).dot(&summed) < 0.0;
    });

    if !folded {
        return unsigned * 0.5;
    }

    summed.magnitude() * 0.5
}

/// Area of a mesh in world units: every face, or only `face`.
pub(crate) fn compute_mesh_area(mesh: &Mesh, place: &Xform, face: Option<usize>) -> f64 {
    match face {
        Some(face) => compute_face_area(mesh, face, place),
        None => mesh
            .faces()
            .into_iter()
            .map(|face| compute_face_area(mesh, face, place))
            .sum(),
    }
}

/// Six times the signed volume the mesh's triangles sweep from `origin`, in world units.
pub(crate) fn compute_swept(mesh: &Mesh, place: &Xform, origin: &Point) -> f64 {
    // Each triangle and the origin make a tetrahedron whose triple product is six times its signed volume; over a closed mesh the outside parts cancel.
    let mut total = 0.0;

    for face in mesh.faces() {
        for_each_triangle(mesh, face, place, |[a, b, c]| {
            total += (&a - origin).dot(&(&b - origin).cross(&(&c - origin)));
        });
    }

    total
}
// --8<-- [end:face-area]

// --8<-- [start:value-text]
/// A value with at most three decimals, no trailing zeros, and no minus on zero.
pub fn to_text(value: f64) -> String {
    // too small for three decimals but not zero
    if value != 0.0 && value.abs() < 0.0005 {
        return format!("{value:.3e}");
    }

    let text = format!("{value:.3}");
    let text = text.trim_end_matches('0').trim_end_matches('.');

    match text {
        "-0" => "0".into(),
        text => text.into(),
    }
}

/// The scene unit raised to `power`, e.g. mm².
pub fn unit_suffix(state: &State, power: usize) -> &'static str {
    let units = match state.camera.unit {
        Unit::Millimeters => ["mm", "mm²", "mm³"],
        Unit::Meters => ["m", "m²", "m³"],
    };
    units[power.clamp(1, 3) - 1]
}

/// `count` followed by `one` or its plural.
pub fn plural(count: usize, one: &str) -> String {
    match count {
        1 => format!("1 {one}"),
        _ => format!("{count} {one}s"),
    }
}

/// ` · N other objects skipped`, or nothing.
pub fn skipped_text(count: usize) -> String {
    match count {
        0 => String::new(),
        _ => format!(" · {} skipped", plural(count, "other object")),
    }
}
// --8<-- [end:value-text]

// --8<-- [start:mark-state]
// The measure verbs keep their State methods here, in one more `impl State` block.
impl State {
    /// Draw `label` along world `points` until the next command, Esc or a change of rows.
    pub(crate) fn set_mark(&mut self, points: Vec<Point>, label: String) {
        self.features.mark = Some(Mark {
            points,
            label,
            revision: self.scene.row_revision,
        });
        self.touch();
    }

    /// Show `label` in a chip below the centre of the selected objects until the next command or Esc.
    pub(crate) fn mark_selection(&mut self, label: String) {
        let mut bounds = session_rust::AABB::empty();

        for row in self.selected_rows() {
            if let Some(object) = self.gpu.objects.row_bounds(row) {
                bounds.union_with(&object);
            }
        }

        if bounds.is_valid() {
            self.set_mark(vec![Point::new(bounds.cx, bounds.cy, bounds.cz)], label);
        }
    }

    /// The mark as a black line with end squares and a chip between the points, or a chip below one point.
    pub fn mark_overlay(&self) -> Option<Overlay> {
        let mark = self.features.mark.as_ref()?;

        if mark.revision != self.scene.row_revision {
            return None;
        }

        let screen = self.view_screen();
        let points: Vec<(f64, f64)> = mark.points.iter().filter_map(|p| screen.point(p)).collect();

        // behind the eye: nothing to draw
        if points.len() != mark.points.len() || points.is_empty() {
            return None;
        }

        // One point: a chip under the selection. Two: a line with end squares and the value in the middle.
        if let [(x, y)] = points[..] {
            let below = 30.0 * self.pixel_scale(); // clear of the selected-name chip
            return Some(Overlay {
                label: Some(((x, y + below), mark.label.clone())),
                ..Default::default()
            });
        }

        let (sx, sy) = points
            .iter()
            .fold((0.0, 0.0), |(x, y), p| (x + p.0, y + p.1));
        let middle = (sx / points.len() as f64, sy / points.len() as f64);
        Some(Overlay {
            strokes: vec![Stroke {
                points: points.clone(),
                color: [20, 20, 20],
                width: 1.5,
                dashed: false,
            }],
            marks: points,
            label: Some((middle, mark.label.clone())),
        })
    }

    /// The mark as JSON, for the inspection tests.
    pub fn mark_status(&self) -> serde_json::Value {
        match &self.features.mark {
            Some(mark) if mark.revision == self.scene.row_revision => serde_json::json!({
                "points": mark.points.iter().map(|p| [p[0], p[1], p[2]]).collect::<Vec<_>>(),
                "label": mark.label,
            }),
            _ => serde_json::Value::Null,
        }
    }
}
// --8<-- [end:mark-state]

// --8<-- [start:measure-tests]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_print_short() {
        assert_eq!(to_text(141.42135623730951), "141.421");
        assert_eq!(to_text(100.0), "100");
        assert_eq!(to_text(-0.0), "0");
        assert_eq!(to_text(-0.0001 * 0.0), "0");
        assert_eq!(to_text(1.6000000000000003), "1.6");
        assert_eq!(to_text(-40.0), "-40");
        assert_eq!(to_text(0.0004), "4.000e-4");
        assert_eq!(to_text(-0.0002), "-2.000e-4");
        assert_eq!(plural(1, "curve"), "1 curve");
        assert_eq!(plural(3, "curve"), "3 curves");
        assert_eq!(skipped_text(0), "");
        assert_eq!(skipped_text(2), " · 2 other objects skipped");
    }

    /// An L-shaped hexagon: its fan overlaps, the signed sum does not.
    #[test]
    fn a_concave_face_measures_exactly() {
        let ring = [
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
            (0.0, 0.0),
            (2.0, 0.0),
        ];
        let points: Vec<Point> = ring.iter().map(|&(x, y)| Point::new(x, y, 0.0)).collect();
        let fan = Mesh::from_vertices_and_faces(points.clone(), vec![(0..6).collect()]);
        let face = fan.faces()[0];
        assert!((compute_face_area(&fan, face, &Xform::identity()) - 3.0).abs() < 1e-12);
        assert!(fan.area() > 3.5, "the kernel's unsigned fan over-counts");
        let cached = Mesh::from_polylines(vec![points], None);
        assert!((compute_mesh_area(&cached, &Xform::identity(), None) - 3.0).abs() < 1e-12);
        let moved = Xform::translation(-9.0, 3.0, 7.0);
        assert!((compute_mesh_area(&cached, &moved, None) - 3.0).abs() < 1e-12);
        let scaled = Xform::scale_xyz(2.0, 2.0, 2.0);
        assert!((compute_mesh_area(&cached, &scaled, None) - 12.0).abs() < 1e-12);
    }

    /// A warped quad measures its two drawn triangles, not their flat shadow.
    #[test]
    fn a_warped_face_measures_as_drawn() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let quad = Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 2, 3]]);
        let area = compute_mesh_area(&quad, &Xform::identity(), None);
        assert!((area - 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn a_box_sweeps_six_times_its_volume() {
        let mesh = Mesh::create_box(2.0, 2.0, 0.4);
        let origin = Point::new(5.0, -3.0, 1.0);
        let swept = compute_swept(&mesh, &Xform::identity(), &origin).abs() / 6.0;
        assert!((swept - 1.6).abs() < 1e-9);
    }
}
// --8<-- [end:measure-tests]
