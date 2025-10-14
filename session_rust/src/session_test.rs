#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::{
        Arrow, BoundingBox, Cylinder, Line, Mesh, Plane, Point, PointCloud, Polyline, Session,
        TreeNode, Vector,
    };

    #[test]
    fn test_session_serialization_with_all_geometry_types() {
        // Create a session with all geometry types
        let mut my_session = Session::new("test_session");

        // Create all geometry types that Objects class can handle
        let point = Point::new(1., 2., 3.);
        let line = Line::new(0., 0., 0., 1., 1., 1.);
        let plane = Plane::from_point_normal(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let bbox = BoundingBox::from_point(Point::new(0., 0., 0.), 1.0);
        let polyline = Polyline::new(vec![Point::new(0., 0., 0.), Point::new(1., 0., 0.)]);
        let pointcloud = PointCloud::new(vec![Point::new(0., 0., 0.)], vec![], vec![]);
        let mesh = Mesh::new();
        let cylinder = Cylinder::new(Line::new(0., 0., 0., 0., 0., 1.), 0.5);
        let arrow = Arrow::new(Line::new(0., 0., 0., 1., 0., 0.), 0.1);

        // Demonstrate 3-level tree hierarchy
        // Level 1: Root -> "geometry" folder
        let geometry_folder = TreeNode::new("geometry");
        my_session.add(&geometry_folder, None); // defaults to root

        // Level 2: "geometry" -> "primitives" and "complex" folders
        let primitives_folder = TreeNode::new("primitives");
        let complex_folder = TreeNode::new("complex");
        my_session.add(&primitives_folder, &geometry_folder);
        my_session.add(&complex_folder, &geometry_folder);

        // Add all geometry to session - returns TreeNode for easy nesting!
        let arrow_node = my_session.add_arrow(arrow.clone());
        let bbox_node = my_session.add_bbox(bbox.clone());
        let cylinder_node = my_session.add_cylinder(cylinder.clone());
        let line_node = my_session.add_line(line.clone());
        let mesh_node = my_session.add_mesh(mesh.clone());
        let plane_node = my_session.add_plane(plane.clone());
        let point_node = my_session.add_point(point.clone());
        let pointcloud_node = my_session.add_pointcloud(pointcloud.clone());
        let polyline_node = my_session.add_polyline(polyline.clone());

        // Level 3: Organize geometry under folders
        // Primitives: point, line, plane
        my_session.add(&point_node, &primitives_folder);
        my_session.add(&line_node, &primitives_folder);
        my_session.add(&plane_node, &primitives_folder);

        // Complex: mesh, polyline, pointcloud, bbox, cylinder, arrow
        my_session.add(&mesh_node, &complex_folder);
        my_session.add(&polyline_node, &complex_folder);
        my_session.add(&pointcloud_node, &complex_folder);
        my_session.add(&bbox_node, &complex_folder);
        my_session.add(&cylinder_node, &complex_folder);
        my_session.add(&arrow_node, &complex_folder);

        // Add edge relationships between geometry objects
        my_session.add_edge(&point.guid, &line.guid, "point_to_line");
        my_session.add_edge(&line.guid, &plane.guid, "line_to_plane");

        // Verify original session structure before serialization
        assert_eq!(my_session.objects.points.len(), 1);
        assert_eq!(my_session.objects.lines.len(), 1);
        assert_eq!(my_session.objects.planes.len(), 1);
        assert_eq!(my_session.objects.bboxes.len(), 1);
        assert_eq!(my_session.objects.polylines.len(), 1);
        assert_eq!(my_session.objects.pointclouds.len(), 1);
        assert_eq!(my_session.objects.meshes.len(), 1);
        assert_eq!(my_session.objects.cylinders.len(), 1);
        assert_eq!(my_session.objects.arrows.len(), 1);
        assert_eq!(my_session.lookup.len(), 9);

        // Graph structure before serialization
        let original_graph_vertices = my_session.graph.number_of_vertices();
        let original_graph_edges = my_session.graph.number_of_edges();
        assert_eq!(original_graph_vertices, 9);
        assert_eq!(original_graph_edges, 2);

        // Tree should have: root + geometry + primitives + complex + 9 geometry nodes = 13 nodes
        let original_tree_nodes = my_session.tree.nodes();
        assert_eq!(original_tree_nodes.len(), 13);

        // Serialize session using custom jsondump (not serde's Serialize trait)
        let s = my_session.jsondump().unwrap();

        // Deserialize using Session::jsonload to properly rebuild lookup table and graph
        let loaded = Session::jsonload(&s).unwrap();

        // Verify session structure after deserialization
        assert_eq!(loaded.name, my_session.name);

        // Verify all geometry objects are preserved
        assert_eq!(loaded.objects.arrows.len(), my_session.objects.arrows.len());
        assert_eq!(loaded.objects.bboxes.len(), my_session.objects.bboxes.len());
        assert_eq!(
            loaded.objects.cylinders.len(),
            my_session.objects.cylinders.len()
        );
        assert_eq!(loaded.objects.lines.len(), my_session.objects.lines.len());
        assert_eq!(loaded.objects.meshes.len(), my_session.objects.meshes.len());
        assert_eq!(loaded.objects.planes.len(), my_session.objects.planes.len());
        assert_eq!(loaded.objects.points.len(), my_session.objects.points.len());
        assert_eq!(
            loaded.objects.pointclouds.len(),
            my_session.objects.pointclouds.len()
        );
        assert_eq!(
            loaded.objects.polylines.len(),
            my_session.objects.polylines.len()
        );

        // Verify lookup table is preserved (rebuilt from objects during deserialization)
        assert_eq!(loaded.lookup.len(), my_session.lookup.len());

        // Verify graph structure is fully preserved
        assert_eq!(loaded.graph.number_of_vertices(), original_graph_vertices);
        assert_eq!(loaded.graph.number_of_edges(), original_graph_edges);
        assert!(loaded.graph.has_edge((&point.guid, &line.guid)));
        assert!(loaded.graph.has_edge((&line.guid, &plane.guid)));

        // Verify tree structure is preserved
        let loaded_tree_nodes = loaded.tree.nodes();
        assert_eq!(loaded_tree_nodes.len(), original_tree_nodes.len());
        assert!(loaded.tree.root().is_some());

        // File I/O
        json_dump(&my_session, "test_session.json", true).unwrap();
        let from_file: Session = json_load("test_session.json").unwrap();
        assert!(!from_file.objects.points.is_empty());
    }
}
