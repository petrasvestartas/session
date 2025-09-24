#[cfg(test)]
mod tests {
    use crate::{Point, Session};

    #[test]
    fn test_session_constructor() {
        let session = Session::new("my_session");
        assert_eq!(session.name, "my_session");
        assert!(!session.guid.is_empty());
        assert_eq!(session.objects.vec.len(), 0);
        assert!(session.tree.root().is_some());
        assert_eq!(session.graph.vertex_count, 0);
    }

    #[test]
    fn test_session_default() {
        let session = Session::default();
        assert_eq!(session.name, "my_session");
        assert!(!session.guid.is_empty());
        assert_eq!(session.objects.vec.len(), 0);
        assert!(session.tree.root().is_some());
        assert_eq!(session.graph.vertex_count, 0);
    }

    #[test]
    fn test_session_to_json_data() {
        let mut session = Session::new("my_session");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point1_guid = point1.guid.clone();
        let point2_guid = point2.guid.clone();

        session.add_point(point1);
        session.add_point(point2);
        session.add_edge(&point1_guid, &point2_guid, "connection");

        let json_result = session.to_json_data();
        assert!(json_result.is_ok());
        let json_data = json_result.unwrap();

        // Check that JSON contains expected structure
        assert!(json_data.contains("\"name\": \"my_session\""));
        assert!(json_data.contains("\"type\": \"Session\""));
        assert!(json_data.contains("\"objects\""));
        assert!(json_data.contains("\"graph\""));
        assert!(json_data.contains("\"tree\""));
    }

    #[test]
    fn test_session_from_json_data() {
        let mut session = Session::new("my_session");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point1_guid = point1.guid.clone();
        let point2_guid = point2.guid.clone();

        session.add_point(point1);
        session.add_point(point2);
        session.add_edge(&point1_guid, &point2_guid, "connection");

        let json_data = session.to_json_data().unwrap();
        let session2 = Session::from_json_data(&json_data).unwrap();
        assert_eq!(session2.name, "my_session");
        assert_eq!(session2.lookup.len(), 2);
        assert_eq!(session2.graph.get_vertices().len(), 2);
    }

    #[test]
    fn test_session_to_json_from_json() {
        let mut session = Session::new("my_session");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point1_guid = point1.guid.clone();
        let point2_guid = point2.guid.clone();

        session.add_point(point1);
        session.add_point(point2);
        session.add_edge(&point1_guid, &point2_guid, "connection");

        // Use in-memory JSON (do not create any files in this test)
        let json_data = session.to_json_data().unwrap();
        let loaded_session = Session::from_json_data(&json_data).unwrap();

        assert_eq!(loaded_session.name, session.name);
        assert_eq!(loaded_session.lookup.len(), session.lookup.len());
        assert_eq!(
            loaded_session.graph.get_vertices().len(),
            session.graph.get_vertices().len()
        );

        // No file was created in this test
    }

    #[test]
    fn test_session_add_point() {
        let mut session = Session::new("my_session");
        let point = Point::new(1.0, 2.0, 3.0);
        let point_guid = point.guid.clone();

        session.add_point(point);

        assert_eq!(session.objects.vec.len(), 1);
        assert!(session.lookup.contains_key(&point_guid));
        assert!(session.graph.has_node(&point_guid));
    }

    #[test]
    fn test_session_add_edge() {
        let mut session = Session::new("my_session");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point1_guid = point1.guid.clone();
        let point2_guid = point2.guid.clone();

        session.add_point(point1);
        session.add_point(point2);
        session.add_edge(&point1_guid, &point2_guid, "connection");

        assert!(session.graph.has_edge((&point1_guid, &point2_guid)));
    }

    #[test]
    fn test_session_get_object() {
        let mut session = Session::new("my_session");
        let point = Point::new(1.0, 2.0, 3.0);
        let point_guid = point.guid.clone();
        let expected_x = point.x;
        let expected_y = point.y;
        let expected_z = point.z;

        session.add_point(point);

        let retrieved = session.get_object(&point_guid).unwrap();
        assert_eq!(retrieved.guid, point_guid);
        assert_eq!(retrieved.x, expected_x);
        assert_eq!(retrieved.y, expected_y);
        assert_eq!(retrieved.z, expected_z);
    }

    #[test]
    fn test_session_remove_object() {
        let mut session = Session::new("my_session");
        let point = Point::new(1.0, 2.0, 3.0);
        let point_guid = point.guid.clone();

        session.add_point(point);
        assert_eq!(session.objects.vec.len(), 1);
        assert!(session.lookup.contains_key(&point_guid));

        let removed = session.remove_object(&point_guid);
        assert!(removed);
        assert_eq!(session.objects.vec.len(), 0);
        assert!(!session.lookup.contains_key(&point_guid));
        assert!(!session.graph.has_node(&point_guid));
    }

    #[test]
    fn test_session_add_relationship() {
        let mut session = Session::new("my_session");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point1_guid = point1.guid.clone();
        let point2_guid = point2.guid.clone();

        session.add_point(point1);
        session.add_point(point2);

        session.add_relationship(&point1_guid, &point2_guid, "related");

        assert!(session.graph.has_edge((&point1_guid, &point2_guid)));
    }

    #[test]
    fn test_session_get_neighbours() {
        let mut session = Session::new("my_session");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point3 = Point::new(7.0, 8.0, 9.0);
        let point1_guid = point1.guid.clone();
        let point2_guid = point2.guid.clone();
        let point3_guid = point3.guid.clone();

        session.add_point(point1);
        session.add_point(point2);
        session.add_point(point3);

        session.add_relationship(&point1_guid, &point2_guid, "connected");
        session.add_relationship(&point1_guid, &point3_guid, "linked");

        let mut neighbours = session.get_neighbours(&point1_guid);
        neighbours.sort();
        let mut expected = vec![point2_guid, point3_guid];
        expected.sort();
        assert_eq!(neighbours, expected);
    }

    #[test]
    fn test_session_to_json_file() {
        let mut session = Session::new("test_session");
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point1_guid = point1.guid.clone();
        let point2_guid = point2.guid.clone();

        session.add_point(point1);
        session.add_point(point2);
        session.add_edge(&point1_guid, &point2_guid, "test_connection");

        let filename = "test_session.json";
        // Ensure a clean state for the file (ignore error if it doesn't exist)
        let _ = std::fs::remove_file(filename);

        // Write to file
        session.to_json(filename).unwrap();

        // Read from file
        let loaded_session = Session::from_json(filename).unwrap();

        assert_eq!(loaded_session.name, session.name);
        assert_eq!(loaded_session.objects.vec.len(), session.objects.vec.len());
        assert_eq!(
            loaded_session.graph.number_of_vertices(),
            session.graph.number_of_vertices()
        );
        assert_eq!(
            loaded_session.graph.number_of_edges(),
            session.graph.number_of_edges()
        );

        // Keep the file as requested - don't delete it
    }
}
