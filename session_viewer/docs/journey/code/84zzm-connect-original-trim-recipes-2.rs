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

    let sides = halves
        .into_iter()
        .map(|faces| {
            // only the vertices this side uses
            let mut index = vec![usize::MAX; points.len()];
            let mut used = Vec::new();

            for face in &faces {
                for &v in face {
                    if index[v] == usize::MAX {
                        index[v] = used.len();
                        used.push(points[v].clone());
                    }
                }
            }

            let faces = faces
                .iter()
                .map(|face| face.iter().map(|v| index[*v]).collect())
                .collect();
            let mut side = Mesh::from_vertices_and_faces(used, faces);
            side.name = mesh.name.clone();
            side.set_objectcolor(mesh.get_objectcolor().clone());
            side
        })
        .collect();
    Ok((sides, section))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A curve cutter.
    fn blade(a: [f64; 3], b: [f64; 3]) -> Blade {
        let curve = NurbsCurve::create(
            false,
            1,
            &[Point::new(a[0], a[1], a[2]), Point::new(b[0], b[1], b[2])],
        );
        Blade {
            cutter: Cutter::Curve(curve),
            fence: None,
        }
    }

    /// A plane cutter.
    fn plane(origin: [f64; 3], normal: [f64; 3]) -> Blade {
        let plane = Plane::from_point_normal(
            Point::new(origin[0], origin[1], origin[2]),
            Vector::new(normal[0], normal[1], normal[2]),
            None,
        );
        Blade {
            cutter: Cutter::Plane(plane),
            fence: None,
        }
    }

    /// A line from its ends.
    fn line(a: [f64; 3], b: [f64; 3]) -> Geometry {
        Geometry::Line(Rc::new(Line::from_points(
            &Point::new(a[0], a[1], a[2]),
            &Point::new(b[0], b[1], b[2]),
        )))
    }

    /// The ends of a line geometry.
    fn ends(geometry: &Geometry) -> [[f64; 3]; 2] {
        let Geometry::Line(line) = geometry else {
            panic!("not a line")
        };
        let (a, b) = (line.start(), line.end());
        [[a[0], a[1], a[2]], [b[0], b[1], b[2]]]
    }

    /// Lines keep the part they were not clicked on; three parts keep the two outer ones.
    #[test]
    fn a_line_keeps_what_is_not_removed() {
        let mut source = Line::from_points(&Point::new(0., 0., 0.), &Point::new(100., 0., 0.));
        source.width = 3.0;
        source.name = "rail".into();
        let source = Geometry::Line(Rc::new(source));
        let parts = cut(&source, &[blade([50., -5., 0.], [50., 5., 0.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let kept_left = kept(&source, &parts, &[1]).unwrap();
        assert_eq!(ends(&kept_left[0]), [[0., 0., 0.], [50., 0., 0.]]);
        let Geometry::Line(styled) = &kept_left[0] else {
            panic!()
        };
        assert_eq!((styled.width, styled.name.as_str()), (3.0, "rail"));
        let parts = cut(
            &source,
            &[
                blade([30., -5., 0.], [30., 5., 0.]),
                blade([70., -5., 0.], [70., 5., 0.]),
            ],
        )
        .unwrap();
        assert_eq!(parts.count(), 3);
        let outer = kept(&source, &parts, &[1]).unwrap();
        assert_eq!(outer.len(), 2);
        assert!((ends(&outer[0])[1][0] - 30.0).abs() < 1e-9);
        assert!((ends(&outer[1])[0][0] - 70.0).abs() < 1e-9);
        let right = kept(&source, &parts, &[0]).unwrap();
        assert_eq!(right.len(), 1, "the two kept parts stay one line");
        assert!(
            (ends(&right[0])[0][0] - 30.0).abs() < 1e-9
                && (ends(&right[0])[1][0] - 100.0).abs() < 1e-9
        );
        assert!(kept(&source, &parts, &[0, 1, 2]).is_err());
        assert!(cut(&source, &[blade([50., 5., 0.], [50., 15., 0.])]).is_err());
    }

    /// A polyline keeps its corner; a NURBS curve ends at the cut; a closed square makes two parts.
    #[test]
    fn polylines_curves_and_closed_curves() {
        let corner = Polyline::new(vec![
            Point::new(0., 60., 0.),
            Point::new(40., 60., 0.),
            Point::new(40., 100., 0.),
        ]);
        let source = Geometry::Polyline(Rc::new(corner));
        let parts = cut(&source, &[blade([20., 50., 0.], [20., 70., 0.])]).unwrap();
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::Polyline(rest) = &rest[0] else {
            panic!()
        };
        let points = rest.get_points();
        assert_eq!(points.len(), 3);
        assert!(points[0].distance(&Point::new(20., 60., 0.), None) < 1e-9);
        assert!(points[1].distance(&Point::new(40., 60., 0.), None) < 1e-9);
        let curve = NurbsCurve::create(
            false,
            2,
            &[
                Point::new(0., -60., 0.),
                Point::new(50., 0., 0.),
                Point::new(100., -60., 0.),
            ],
        );
        let source = Geometry::NurbsCurve(Rc::new(curve));
        let parts = cut(&source, &[blade([50., -80., 0.], [50., 10., 0.])]).unwrap();
        let left = kept(&source, &parts, &[1]).unwrap();
        let Geometry::NurbsCurve(left) = &left[0] else {
            panic!()
        };
        assert!((left.point_at_end()[0] - 50.0).abs() < 1e-6);
        let square = Polyline::new(vec![
            Point::new(0., 0., 0.),
            Point::new(10., 0., 0.),
            Point::new(10., 10., 0.),
            Point::new(0., 10., 0.),
            Point::new(0., 0., 0.),
        ]);
        let source = Geometry::Polyline(Rc::new(square));
        let parts = cut(&source, &[blade([5., -5., 0.], [5., 15., 0.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let half = kept(&source, &parts, &[0]).unwrap();
        assert_eq!(half.len(), 1);
    }

    /// A plane cuts a vertical line; mixed cutters cut at both.
    #[test]
    fn planes_and_curves_cut_together() {
        let source = line([0., 0., -10.], [0., 0., 10.]);
        let parts = cut(&source, &[plane([0., 0., 0.], [0., 0., 1.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let across = line([0., 0., 0.], [100., 0., 0.]);
        let parts = cut(
            &across,
            &[
                plane([20., 0., 0.], [1., 0., 0.]),
                blade([60., -5., 0.], [60., 5., 0.]),
            ],
        )
        .unwrap();
        assert_eq!(parts.count(), 3);
        let points = parts.cut_points();
        assert!((points[0][0] - 20.0).abs() < 1e-6 && (points[1][0] - 60.0).abs() < 1e-6);
    }

    /// A box cut by a plane: removing the top leaves an open, valid five-face BRep.
    #[test]
    fn a_box_cut_by_a_plane_loses_its_top() {
        let source = BRep::create_box(10., 10., 10.);
        let low = source
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let parts = cut(
            &Geometry::BRep(Rc::new(source)),
            &[plane([0., 0., low + 2.], [0., 0., 1.])],
        )
        .unwrap();
        assert_eq!(parts.count(), 2);
        let Parts::Faces {
            brep,
            groups,
            section,
            ..
        } = &parts
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 10);
        assert_eq!(section.len(), 4, "four cut edges");
        let upper = groups
            .iter()
            .position(|faces| {
                faces
                    .iter()
                    .all(|f| brep.point_at(*f, 0.5, 0.5)[2] > low + 2.0 - 1e-9)
            })
            .unwrap();
        let rest = kept(&Geometry::BRep(Rc::new(BRep::new())), &parts, &[upper]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 5);
        assert!(rest.is_valid());
        assert!(!rest.is_solid());

        for edge in 0..rest.m_edges.len() {
            assert!(!rest.edge_faces(edge).is_empty(), "edge {edge} has a face");
        }

        let used: std::collections::HashSet<i32> = rest
            .m_edges
            .iter()
            .flat_map(|e| [e.start_vertex, e.end_vertex])
            .collect();
        assert_eq!(used.len(), rest.m_vertices.len(), "no unused vertex");
        assert!(
            cut(
                &Geometry::BRep(Rc::new(BRep::create_box(10., 10., 10.))),
                &[plane([0., 0., 100.], [0., 0., 1.])]
            )
            .is_err()
        );
        assert_eq!(without_faces(rest, &[]).unwrap().face_count(), 5);
    }

    /// A plane through a curved face is refused rather than cut half way.
    #[test]
    fn a_plane_refuses_curved_faces() {
        let source = Geometry::BRep(Rc::new(BRep::create_cylinder(5., 10.)));
        let Geometry::BRep(cylinder) = &source else {
            panic!()
        };
        let low = cylinder
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let answer = cut(&source, &[plane([0., 0., low + 5.], [0., 0., 1.])]);
        assert!(answer.is_err_and(|error| error.contains("flat faces")));
    }

    /// A line off the box cuts it as a vertical fence; a line on the top face splits that face only.
    #[test]
    fn lines_fence_or_split_a_box() {
        let source = Geometry::BRep(Rc::new(BRep::create_box(10., 10., 10.)));
        let Geometry::BRep(boxed) = &source else {
            panic!()
        };
        let low = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let high = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::NEG_INFINITY, f64::max);
        let x = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[0])
            .fold(f64::INFINITY, f64::min)
            + 3.0;
        let fence = crate::source_cut::fence_plane(
            &Line::from_points(
                &Point::new(x, -50., low - 5.),
                &Point::new(x, 50., low - 5.),
            ),
            &Vector::new(0., 0., 1.),
        )
        .unwrap();
        let mut off = blade([x, -50., low - 5.], [x, 50., low - 5.]);
        off.fence = Some(fence);
        let parts = cut(&source, &[off]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 5);
        assert!(rest.is_valid());
        let on = blade([x, -50., high], [x, 50., high]);
        let parts = cut(&source, &[on]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[1]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 6);
        assert!(rest.is_valid());
    }

    /// A planar surface split by a line keeps the chosen region as a BRep.
    #[test]
    fn a_surface_becomes_the_kept_region() {
        let mut surface = NurbsSurface::new(3, false, 2, 2, 2, 2);
        surface.set_cv(0, 0, &Point::new(0., 0., 0.));
        surface.set_cv(1, 0, &Point::new(10., 0., 0.));
        surface.set_cv(0, 1, &Point::new(0., 10., 0.));
        surface.set_cv(1, 1, &Point::new(10., 10., 0.));
        let source = Geometry::NurbsSurface(Rc::new(surface));
        let parts = cut(&source, &[blade([4., -5., 0.], [4., 15., 0.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 1);
    }

    /// A mesh box cut at z = 2: the areas add up, both sides are open and share the cut.
    #[test]
    fn a_mesh_splits_into_two_open_sides() {
        let mut source = Mesh::create_box(10., 10., 10.);
        source.name = "block".into();
        let area = source.area();
        let (points, _) = source.to_vertices_and_faces();
        let low = points.iter().map(|p| p[2]).fold(f64::INFINITY, f64::min);
        let cutter =
            Plane::from_point_normal(Point::new(0., 0., low + 2.), Vector::new(0., 0., 1.), None);
        let (sides, section) = mesh_sides(&source, &cutter).unwrap();
        assert!((sides[0].area() + sides[1].area() - area).abs() < 1e-9);
        assert!(!sides[0].is_closed() && !sides[1].is_closed());
        assert_eq!(sides[1].number_of_vertices(), 8);
        assert_eq!(sides[0].name, "block");
        assert!(!section.is_empty());
        let away =
            Plane::from_point_normal(Point::new(0., 0., 100.), Vector::new(0., 0., 1.), None);
        assert!(mesh_sides(&source, &away).is_err());
    }

}
