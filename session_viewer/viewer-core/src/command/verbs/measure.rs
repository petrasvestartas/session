use crate::app::command::CommandHost;
use crate::app::command::tool::{Overlay, Stroke};
use crate::app::selection::SelectionMode;
use crate::camera::Unit;
use session_rust::{Geometry, Mesh, Point, Vector, Xform};

/// A measured answer drawn in the scene until the next command, Esc or a change of rows.
pub struct Mark {
    pub points: Vec<Point>, // world points, joined by a line
    pub label: String,      // the value beside them
    pub revision: u64,      // the scene rows it was measured on
}

/// One selected object to measure.
pub struct Target<'a> {
    pub row: u32,               // scene row
    pub geometry: &'a Geometry, // its kernel object
    pub place: Xform,           // local to world
    pub face: Option<usize>,    // the chosen face, when a face is selected
}

/// The selected objects with geometry, and how many selected rows have none (texts, sheets).
pub fn selected(state: &dyn CommandHost) -> (Vec<Target<'_>>, usize) {
    let rows = state.selected_rows();
    let face = match state.selection() {
        SelectionMode::Face { parent, face } => Some((parent, face)),
        _ => None,
    };
    let targets: Vec<Target> = rows
        .iter()
        .filter_map(|&row| {
            Some(Target {
                row,
                geometry: state.scene().geometry(row)?,
                place: state.scene().placement_of(row)?,
                face: face
                    .filter(|(parent, _)| **parent == row)
                    .map(|(_, &face)| face),
            })
        })
        .collect();
    let skipped = rows.len() - targets.len();
    (targets, skipped)
}

/// Ask for released documents of the selection back; the error says to try again.
pub fn loading(state: &mut dyn CommandHost) -> Result<(), String> {
    let docs: Vec<usize> = state
        .selected_rows()
        .iter()
        .filter_map(|&row| state.scene().released_doc(row))
        .collect();

    let Some(&doc) = docs.first() else {
        return Ok(());
    };

    for &doc in &docs {
        state.scene_mut().want(doc);
    }

    state.fetch_wanted();
    Err(format!(
        "Loading '{}' to measure it; try again in a moment",
        state.scene().docs[doc].name
    ))
}

/// Visit the triangles the viewer draws for face `face`, in world coordinates: the cached triangulation, else a fan.
pub fn for_each_triangle(mesh: &Mesh, face: usize, place: &Xform, visit: impl FnMut([Point; 3])) {
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
pub fn compute_face_area(mesh: &Mesh, face: usize, place: &Xform) -> f64 {
    let cached = mesh
        .triangulation
        .get(&face)
        .is_some_and(|tris| !tris.is_empty());
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
pub fn compute_mesh_area(mesh: &Mesh, place: &Xform, face: Option<usize>) -> f64 {
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
pub fn compute_swept(mesh: &Mesh, place: &Xform, origin: &Point) -> f64 {
    let mut total = 0.0;

    for face in mesh.faces() {
        for_each_triangle(mesh, face, place, |[a, b, c]| {
            total += (&a - origin).dot(&(&b - origin).cross(&(&c - origin)));
        });
    }

    total
}

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
pub fn unit_suffix(state: &dyn CommandHost, power: usize) -> &'static str {
    let units = match state.camera().unit {
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

/// Build the overlay chip and stroke for an active measurement mark.
pub fn mark_overlay(
    mark: Option<&Mark>,
    row_revision: u64,
    screen: &crate::app::snap::Screen,
    pixel_scale: f64,
) -> Option<Overlay> {
    let mark = mark?;

    if mark.revision != row_revision {
        return None;
    }

    let points: Vec<(f64, f64)> = mark.points.iter().filter_map(|p| screen.point(p)).collect();

    // behind the eye: nothing to draw
    if points.len() != mark.points.len() || points.is_empty() {
        return None;
    }

    if let [(x, y)] = points[..] {
        let below = 30.0 * pixel_scale; // clear of the selected-name chip
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
        fill: None,
    })
}

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
