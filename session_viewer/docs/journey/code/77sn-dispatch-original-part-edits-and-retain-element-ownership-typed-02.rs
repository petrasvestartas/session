ne(source, target, delta),
        Geometry::NurbsCurve(source) => transform_nurbscurve(source, target, delta),
        Geometry::Line(source) => transform_line(source, target, delta),
        Geometry::Point(source) => transform_point(source, target, delta),
        Geometry::BRep(source) => transform_brep(geometry, source, target, delta),
        Geometry::Element(source) => transform_element(source, target, delta),
        _ => Err("This source control cannot be transformed".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A quad and a triangle sharing an edge.
    fn mesh() -> Geometry {
        let mut mesh = Mesh::new();

        for (key, p) in [
            (10, [0., 0., 0.]),
            (20, [10., 0., 0.]),
            (30, [10., 10., 0.]),
            (40, [0., 10., 0.]),
            (90, [0., 0., 10.]),
        ] {
            mesh.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
        }

        mesh.add_face(vec![10, 20, 30, 40], Some(7));
        mesh.add_face(vec![10, 90, 20], Some(19));
        Geometry::Mesh(Rc::new(mesh))
    }

    /// Moving a face moves its vertices only.
    #[test]
    fn rational_curve_control_moves_without_changing_weights_or_other_controls() {
        let mut curve = session_rust::NurbsCurve::create(true, 2, &[
            Point::new(-1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0), Point::new(1.0, 0.0, 0.0),
        ]);
        assert!(curve.set_cv_4d(1, 0.0, 2.0, 0.0, 2.0));
        let _ = curve.guid();
        let geometry = Geometry::NurbsCurve(Rc::new(curve));
        let delta = Xform::translation(0.123456789012345, 0.25, -0.5);
        let moved = transform(&geometry, Target::Control(ControlId::Curve { curve: 0, point: 1 }), &delta).unwrap();
        let (Geometry::NurbsCurve(original), Geometry::NurbsCurve(edited)) = (&geometry, moved) else { panic!("Curve"); };
        assert_eq!(edited.guid(), original.guid());
        assert_eq!(edited.m_nurbsknot, original.m_nurbsknot);
        for index in 0..original.cv_count() {
            assert_eq!(edited.get_cv_4d(index).unwrap().3, original.get_cv_4d(index).unwrap().3);
            let before = original.get_cv(index).unwrap();
            let expected = if index == 1 { before.transformed(&delta) } else { before };
            let actual = edited.get_cv(index).unwrap();
            for axis in 0..3 { assert!((actual[axis] - expected[axis]).abs() < 1e-14); }
        }
    }

    #[test]
    fn mesh_face_moves_shared_source_vertices_not_unrelated_vertices() {
        let source = mesh();
        let Geometry::Mesh(next) =
            transform(&source, Target::Face(7), &Xform::translation(0., 0., 3.)).unwrap()
        else {
            panic!()
        };

        for key in [10, 20, 30, 40] {
            assert_eq!(next.vertex[&key].z, 3.);
        }

        assert_eq!(next.vertex[&90].z, 10.);
        assert_eq!(next.face[&19], vec![10, 90, 20]);
        let Geometry::Mesh(original) = source else {
            panic!()
        };
        assert_eq!(original.vertex[&10].z, 0.);
    }

    /// Edge numbering matches the display edges.
    #[test]
    fn edge_ids_match_the_display_producer_even_with_sparse_keys() {
        let Geometry::Mesh(mesh) = mesh() else {
            panic!()
        };
        let keys = mesh.vertices();
        let positions: Vec<_> = keys
            .iter()
            .map(|k| {
                let p = mesh.vertex_point(*k).unwrap();
                [p[0], p[1], p[2]]
            })
            .collect();
        let slots = crate::source_mesh_topology::SlotMap::new(&keys);
        let topo =
            crate::source_mesh_topology::mesh_topology(&mesh, &keys, &positions, &slots);

        for (i, &(a, b, _)) in topo.edges.iter().enumerate() {
            assert_eq!(
                mesh_keys(&mesh, Target::Edge(i as u32)).unwrap(),
                vec![a, b]
            );
        }
    }

    /// Moving one surface edge keeps weights and the other edge.
    #[test]
    fn surface_boundary_preserves_weights_and_the_opposite_boundary() {
        let mut surface = NurbsSurface::create(
            false,
            false,
            1,
            1,
            2,
            2,
            &[
                Point::new(0., 0., 0.),
                Point::new(1., 0., 0.),
                Point::new(0., 1., 0.),
                Point::new(1., 1., 0.),
            ],
        )
        .unwrap();
        assert!(surface.to_rational());

        for u in 0..2 {
            for v in 0..2 {
                surface.set_cv_4d(u, v, u as f64 * 2., v as f64 * 2., 0., 2.);
            }
        }

        let Geometry::NurbsSurface(next) = transform(
            &Geometry::NurbsSurface(Rc::new(surface)),
            Target::Edge(0),
            &Xform::translation(0., 0., 5.),
        )
        .unwrap() else {
            panic!()
        };
        assert_eq!(next.get_cv(0, 0).unwrap()[2], 5.);
        assert_eq!(next.get_cv(1, 0).unwrap()[2], 0.);
        assert!(next.m_is_rat);
        assert_eq!(next.weight(0, 0), 2.);
        assert_eq!(next.weight(1, 1), 2.);
    }

    /// Moving a box face keeps the solid valid.
    #[test]
    fn moving_box_face_keeps_edges_on_incident_surfaces() {
        let source = Geometry::BRep(Rc::new(session_rust::BRep::create_box(10., 20., 30.)));
        let Geometry::BRep(next) =
            transform(&source, Target::Face(0), &Xform::translation(0., 0., 2.)).unwrap()
        else {
            panic!()
        };
        assert!(next.is_valid());
        assert!(next.is_closed(0));
        validate_boundaries(
            &next,
            &(0..next.m_curves_3d.len()).collect(),
            &(0..next.m_surfaces.len()).collect(),
        )
        .unwrap();
    }
}
