#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::{
        Arrow, BoundingBox, Cylinder, Line, Mesh, Plane, Point, PointCloud, Polyline, Session,
        TreeNode, Vector, BVH,
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

    #[test]
    fn test_session_ray_cast_sanity() {
        let mut scene = Session::new("ray_test_rs");

        let pt1 = Point::new(5.0, 0.0, 0.0);
        scene.add_point(pt1.clone());
        let pt2 = Point::new(15.0, 0.0, 0.0);
        scene.add_point(pt2.clone());
        let line1 = Line::from_points(&Point::new(10.0, -2.0, 0.0), &Point::new(10.0, 2.0, 0.0));
        scene.add_line(line1.clone());
        let plane1 = Plane::new(
            Point::new(20.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        scene.add_plane(plane1.clone());
        let poly = Polyline::new(vec![
            Point::new(25.0, -1.0, -1.0),
            Point::new(25.0, 0.0, 0.0),
            Point::new(25.0, 1.0, 1.0),
        ]);
        scene.add_polyline(poly);

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        let hits = scene.ray_cast(&ray_origin, &ray_dir, 0.5);
        println!("Session Ray Casting (rs): {} hit(s)", hits.len());
        assert!(!hits.is_empty());
    }

    #[test]
    fn test_all_geometry_types_ray_cast_subset() {
        let mut scene = Session::new("all_geom_rs");
        scene.add_point(Point::new(0.0, 10.0, 0.0));
        scene.add_line(Line::from_points(
            &Point::new(-1.0, 20.0, 0.0),
            &Point::new(1.0, 20.0, 0.0),
        ));
        scene.add_plane(Plane::new(
            Point::new(0.0, 30.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
        ));
        scene.add_bbox(BoundingBox::new(
            Point::new(0.0, 40.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(2.0, 2.0, 2.0),
        ));
        scene.add_polyline(Polyline::new(vec![
            Point::new(-1.0, 70.0, 0.0),
            Point::new(0.0, 70.0, 0.0),
            Point::new(1.0, 70.0, 0.0),
        ]));

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir = Vector::new(0.0, 1.0, 0.0);

        let hits = scene.ray_cast(&ray_origin, &ray_dir, 1.0);
        println!(
            "All Geometry Types (subset) Ray Casting (rs): {} hit(s)",
            hits.len()
        );
        assert!(!hits.is_empty());
    }

    #[test]
    fn test_performance_points_vs_pure_bvh_rs() {
        use rand::prelude::*;
        let mut rng = StdRng::seed_from_u64(42);

        let object_count = 2000;
        let world_size = 100.0f64;

        let mut scene = Session::new("perf_points_rs");
        let mut pure_boxes: Vec<BoundingBox> = Vec::with_capacity(object_count);

        for _ in 0..object_count {
            let x = (rng.gen::<f64>() - 0.5) * world_size;
            let y = (rng.gen::<f64>() - 0.5) * world_size;
            let z = (rng.gen::<f64>() - 0.5) * world_size;
            let pt = Point::new(x, y, z);
            scene.add_point(pt.clone());
            pure_boxes.push(BoundingBox::new(
                pt.clone(),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(0.5, 0.5, 0.5),
            ));
        }

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        let t0 = std::time::Instant::now();
        let hits = scene.ray_cast(&ray_origin, &ray_dir, 1.0);
        let session_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t2 = std::time::Instant::now();
        let bvh = BVH::from_boxes(&pure_boxes, world_size);
        let mut candidates: Vec<usize> = Vec::new();
        bvh.ray_cast(&ray_origin, &ray_dir, &mut candidates, true);
        let bvh_ms = t2.elapsed().as_secs_f64() * 1000.0;

        println!("Session (rs): {:.3} ms ({} hits)", session_ms, hits.len());
        println!(
            "Pure BVH (rs): {:.3} ms ({} candidates)",
            bvh_ms,
            candidates.len()
        );
        assert!(session_ms >= 0.0 && bvh_ms >= 0.0);
    }

    #[test]
    fn test_ray_cast_mesh_bvh_hit() {
        let mut scene = Session::new("mesh_bvh_hit");
        let tri = vec![
            Point::new(30.0, -1.0, -1.0),
            Point::new(30.0, 1.0, -1.0),
            Point::new(30.0, 0.0, 1.0),
        ];
        let mesh = Mesh::from_polygons(vec![tri], None);
        let mesh_guid = mesh.guid.clone();
        scene.add_mesh(mesh);

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        let hits = scene.ray_cast(&ray_origin, &ray_dir, 1e-3);
        assert!(!hits.is_empty());
        assert!(hits.iter().any(|h| h.guid == mesh_guid));
    }

    #[test]
    fn test_ray_cast_cache_invalidation_remove() {
        let mut scene = Session::new("cache_invalidate_remove");
        let line = Line::from_points(&Point::new(10.0, -2.0, 0.0), &Point::new(10.0, 2.0, 0.0));
        let guid = line.guid.clone();
        scene.add_line(line);

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        let hits_before = scene.ray_cast(&ray_origin, &ray_dir, 1e-3);
        assert!(!hits_before.is_empty());

        scene.remove_object(&guid);

        let hits_after = scene.ray_cast(&ray_origin, &ray_dir, 1e-3);
        assert!(hits_after.is_empty());
    }

    #[test]
    fn test_ray_cast_closest_multi_same_distance() {
        let mut scene = Session::new("closest_multi");

        let line = Line::from_points(&Point::new(10.0, -2.0, 0.0), &Point::new(10.0, 2.0, 0.0));
        let line_guid = line.guid.clone();
        scene.add_line(line);

        let plane = Plane::new(
            Point::new(10.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
        );
        let plane_guid = plane.guid.clone();
        scene.add_plane(plane);

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        let hits = scene.ray_cast(&ray_origin, &ray_dir, 1e-3);
        let guids: Vec<String> = hits.iter().map(|h| h.guid.clone()).collect();
        assert!(guids.contains(&line_guid));
        assert!(guids.contains(&plane_guid));
    }

    #[test]
    fn test_point_tolerance_hit() {
        let mut scene = Session::new("point_tol");
        let p = Point::new(5.0, 5e-4, 0.0);
        scene.add_point(p);

        let ray_origin = Point::new(0.0, 0.0, 0.0);
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        let hits = scene.ray_cast(&ray_origin, &ray_dir, 1e-3);
        assert_eq!(hits.len(), 1);
        assert!(hits[0].distance > 4.9 && hits[0].distance < 5.1);
    }

    #[test]
    fn test_ray_cast_cached_vs_uncached_repeated() {
        use rand::prelude::*;
        let mut rng = StdRng::seed_from_u64(123);

        let object_count = 3000usize;
        let world_size = 200.0f64;
        let repeats = 50usize;

        let mut scene = Session::new("ray_cache_bench");
        // Add random points and some lines to populate BVH
        for _ in 0..object_count {
            let x = (rng.gen::<f64>() - 0.5) * world_size;
            let y = (rng.gen::<f64>() - 0.5) * world_size;
            let z = (rng.gen::<f64>() - 0.5) * world_size;
            scene.add_point(Point::new(x, y, z));
        }
        for i in 0..100 {
            let x = -50.0 + i as f64 * 1.0;
            scene.add_line(Line::from_points(
                &Point::new(x, -10.0, 0.0),
                &Point::new(x, 10.0, 0.0),
            ));
        }

        let ray_origin = Point::new(-100.0, 0.0, 0.0);
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        // First call (uncached or cache rebuild)
        let t0 = std::time::Instant::now();
        let hits0 = scene.ray_cast(&ray_origin, &ray_dir, 1.0);
        let t_first = t0.elapsed().as_secs_f64() * 1000.0;

        // Repeated cached calls
        let t1 = std::time::Instant::now();
        let mut total_hits = 0usize;
        for _ in 0..repeats {
            let hits = scene.ray_cast(&ray_origin, &ray_dir, 1.0);
            total_hits += hits.len();
        }
        let t_cached = t1.elapsed().as_secs_f64() * 1000.0;
        let avg_cached = t_cached / repeats as f64;

        println!(
            "Ray cast cache bench: first={:.3} ms, cached_avg={:.3} ms (hits0={}, total_cached_hits={})",
            t_first, avg_cached, hits0.len(), total_hits
        );

        assert!(t_first >= 0.0 && avg_cached >= 0.0);
    }
}
