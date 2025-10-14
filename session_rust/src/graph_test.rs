#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::graph::Graph;

    #[test]
    fn test_graph_constructor() {
        let graph = Graph::new("my_graph");
        assert_eq!(graph.name, "my_graph");
        assert!(!graph.guid.is_empty());
    }

    #[test]
    fn test_graph_add_node() {
        let mut graph = Graph::new("my_graph");
        let result = graph.add_node("node1", "attribute_data");
        assert_eq!(result, "node1");
        assert!(graph.has_node("node1"));
    }

    #[test]
    fn test_graph_has_node() {
        let mut graph = Graph::new("my_graph");
        graph.add_node("node1", "");
        assert!(graph.has_node("node1"));
        assert!(!graph.has_node("node2"));
    }

    #[test]
    fn test_graph_add_edge() {
        let mut graph = Graph::new("my_graph");
        let result = graph.add_edge("node1", "node2", "edge_data");
        assert_eq!(result, ("node1".to_string(), "node2".to_string()));
        assert!(graph.has_edge(("node1", "node2")));
        assert!(graph.has_edge(("node2", "node1"))); // Check reverse direction
    }

    #[test]
    fn test_graph_has_edge() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("A", "B", "edge_attr");
        assert!(graph.has_edge(("A", "B")));
        assert!(!graph.has_edge(("C", "D")));
    }

    #[test]
    fn test_graph_number_of_vertices() {
        let mut graph = Graph::new("my_graph");
        graph.add_node("node1", "");
        assert_eq!(graph.number_of_vertices(), 1);
        graph.add_node("node2", "");
        assert_eq!(graph.number_of_vertices(), 2);
    }

    #[test]
    fn test_graph_number_of_edges() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("node1", "node2", "");
        assert_eq!(graph.number_of_edges(), 1);
        graph.add_edge("node2", "node3", "");
        assert_eq!(graph.number_of_edges(), 2);
        graph.add_edge("node1", "node2", ""); // Add existing edge
        assert_eq!(graph.number_of_edges(), 2);
    }

    #[test]
    fn test_graph_get_vertices() {
        let mut graph = Graph::new("my_graph");
        graph.add_node("node1", "node_data");
        let verts = graph.get_vertices();
        assert_eq!(verts.len(), 1);
        assert_eq!(verts[0].name, "node1");
    }

    #[test]
    fn test_graph_get_edges() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("node1", "node2", "edge_data");
        let edges = graph.get_edges();
        assert_eq!(edges.len(), 1);
        // The order is not guaranteed, so we check for both possibilities
        let edge = ("node1".to_string(), "node2".to_string());
        let reversed_edge = ("node2".to_string(), "node1".to_string());
        assert!(edges.contains(&edge) || edges.contains(&reversed_edge));
    }

    #[test]
    fn test_graph_neighbors() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("A", "B", "");
        graph.add_edge("A", "C", "");
        let mut neighbors = graph.neighbors("A");
        neighbors.sort();
        assert_eq!(neighbors, vec!["B".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_graph_remove_node() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("A", "B", "");
        graph.add_edge("A", "C", "");
        graph.remove_node("A");
        assert!(!graph.has_node("A"));
        assert!(!graph.has_edge(("A", "B")));
        assert!(!graph.has_edge(("A", "C")));
        assert_eq!(graph.number_of_vertices(), 2); // B and C should still exist
        assert_eq!(graph.number_of_edges(), 0);
    }

    #[test]
    fn test_graph_remove_edge() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("A", "B", "edge_attr");
        graph.remove_edge(("A", "B"));
        assert!(!graph.has_edge(("A", "B")));
        assert!(!graph.has_edge(("B", "A")));
        assert_eq!(graph.number_of_edges(), 0);
    }

    #[test]
    fn test_graph_clear() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("A", "B", "");
        graph.clear();
        assert_eq!(graph.number_of_vertices(), 0);
        assert_eq!(graph.number_of_edges(), 0);
        assert!(graph.get_vertices().is_empty());
        assert!(graph.get_edges().is_empty());
    }

    #[test]
    fn test_graph_node_attribute() {
        let mut graph = Graph::new("my_graph");
        graph.add_node("node1", "initial_data");
        assert_eq!(graph.node_attribute("node1", None).unwrap(), "initial_data");
        graph.node_attribute("node1", Some("new_data"));
        assert_eq!(graph.node_attribute("node1", None).unwrap(), "new_data");
    }

    #[test]
    fn test_graph_edge_attribute() {
        let mut graph = Graph::new("my_graph");
        graph.add_edge("node1", "node2", "edge_data");
        assert_eq!(
            graph.edge_attribute("node1", "node2", None).unwrap(),
            "edge_data"
        );
        graph.edge_attribute("node1", "node2", Some("new_data"));
        assert_eq!(
            graph.edge_attribute("node1", "node2", None).unwrap(),
            "new_data"
        );
        // Also check the reverse direction
        assert_eq!(
            graph.edge_attribute("node2", "node1", None).unwrap(),
            "new_data"
        );
    }

    #[test]
    fn test_graph_from_json_data() {
        let data = r#"{
            "type": "Graph",
            "name": "test_graph",
            "guid": "test-guid-123",
            "vertex_count": 3,
            "edge_count": 2,
            "vertices": [
                {"name": "node1", "guid": "guid1", "attribute": "type:start", "index": 0},
                {"name": "node2", "guid": "guid2", "attribute": "type:middle", "index": 1},
                {"name": "node3", "guid": "guid3", "attribute": "type:end", "index": 2}
            ],
            "edges": [
                {"v0": "node1", "v1": "node2", "guid": "edge1", "name": "edge1", "attribute": "weight:10", "index": 0},
                {"v0": "node2", "v1": "node3", "guid": "edge2", "name": "edge2", "attribute": "weight:20", "index": 1}
            ]
        }"#;

        let mut graph = Graph::jsonload(data).unwrap();

        assert_eq!(graph.name, "test_graph");
        assert_eq!(graph.number_of_vertices(), 3);
        assert_eq!(graph.number_of_edges(), 2);
        assert!(graph.has_node("node1"));
        assert!(graph.has_node("node2"));
        assert!(graph.has_node("node3"));
        assert!(graph.has_edge(("node1", "node2")));
        assert!(graph.has_edge(("node2", "node3")));
        assert_eq!(graph.node_attribute("node1", None).unwrap(), "type:start");
        assert_eq!(
            graph.edge_attribute("node1", "node2", None).unwrap(),
            "weight:10"
        );
    }

    #[test]
    fn test_graph_to_json_from_json() {
        let mut graph = Graph::new("my_graph");
        graph.add_node("A", "vertex_A");
        graph.add_node("B", "vertex_B");
        graph.add_node("C", "vertex_C");
        graph.add_node("D", "vertex_D");
        graph.add_edge("A", "B", "edge_AB");
        graph.add_edge("B", "C", "edge_BC");
        graph.add_edge("C", "D", "edge_CD");
        let filename = "test_graph.json";

        json_dump(&graph, filename, true).unwrap();
        let loaded_graph = json_load::<Graph>(filename).unwrap();

        assert_eq!(graph.name, loaded_graph.name);
        assert_eq!(
            graph.number_of_vertices(),
            loaded_graph.number_of_vertices()
        );
        assert_eq!(graph.number_of_edges(), loaded_graph.number_of_edges());
    }
}
