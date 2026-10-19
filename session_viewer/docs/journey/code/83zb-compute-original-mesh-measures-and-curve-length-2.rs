use session_rust::{Mesh,Point,Vector,Xform};
/// Visit the triangles the viewer draws for face `face`, in world coordinates: the cached triangulation, else a fan.
pub fn for_each_triangle(
    mesh: &Mesh,
    face: usize,
    place: &Xform,
    visit: impl FnMut([Point; 3]),
) {
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
