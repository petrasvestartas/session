 used: &[bool]) -> Vec<T> {
    list.iter()
        .zip(used)
        .filter(|(_, kept)| **kept)
        .map(|(item, _)| item.clone())
        .collect()
}

/// A mesh cut by `plane` into its two sides, sharing the cut vertices; the cut as segments.
pub fn mesh_sides(mesh: &Mesh, plane: &Plane) -> Result<(Vec<Mesh>, Vec<[Point; 2]>), String> {
    if !mesh.get_face_holes().is_empty() {
        return Err("Trim cannot cut a mesh with holed faces".into());
    }

    let (mut points, faces) = mesh.to_vertices_and_faces();
    let normal = plane.z_axis();
    let origin = plane.origin();
    let size = points
        .iter()
        .map(|p| p[0].abs().max(p[1].abs()).max(p[2].abs()))
        .fold(1.0, f64::max);
    let near = 1e-9 * size;
    let side: Vec<f64> = points
        .iter()
        .map(|p| {
            let d = (p - &origin).dot(&normal);
            if d.abs() <= near { 0.0 } else { d }
        })
        .collect();

    if side.iter().all(|d| *d >= 0.0) || side.iter().all(|d| *d <= 0.0) {
        return Err("not crossed".into());
    }

    let mut crossings: HashMap<(usize, usize), usize> = HashMap::new(); // edge to its cut vertex
    let mut halves: [Vec<Vec<usize>>; 2] = [Vec::new(), Vec::new()];
    let mut section = Vec::new();

    for face in &faces {
        if face.iter().all(|v| side[*v] >= 0.0) {
            halves[0].push(face.clone());
            continue;
        }

        if face.iter().all(|v| side[*v] <= 0.0) {
            halves[1].push(face.clone());
            continue;
        }

        let mut above = Vec::new();
        let mut below = Vec::new();
        let mut on = Vec::new(); // this face's points on the plane

        for (i, &a) in face.iter().enumerate() {
            let b = face[(i + 1) % face.len()];

            if side[a] >= 0.0 {
                above.push(a);
            }

            if side[a] <= 0.0 {
                below.push(a);
            }

            if side[a] == 0.0 {
                on.push(a);
            }

            if side[a] * side[b] < 0.0 {
                let key = (a.min(b), a.max(b));
                let at = *crossings.entry(key).or_insert_with(|| {
                    let t = side[a] / (side[a] - side[b]);
                    let (p, q) = (&points[a], &points[b]);
                    points.push(Point::new(
                        p[0] + t * (q[0] - p[0]),
                        p[1] + t * (q[1] - p[1]),
                        p[2] + t * (q[2] - p[2]),
                    ));
                    points.len() - 1
                });
                above.push(at);
                below.push(at);
                on.push(at);
            }
        }

        if let [a, b] = on.as_slice() {
            section.push([points[*a].clone(), points[*b].clone()]);
        }

        for (half, ring) in [(0, above), (1, below)] {
            if ring.len() >= 3 {
                halves[half].push(ring);
            }
        }
    }

    Err("Mesh table reconstruction is not connected yet".into())
}

