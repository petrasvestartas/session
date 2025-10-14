#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::vertex::Vertex;

    #[test]
    fn test_vertex_json_roundtrip() {
        let vertex = Vertex::new(Some("v0".to_string()), Some("attribute".to_string()));

        let json_str = serde_json::to_string_pretty(&vertex).unwrap();
        let loaded: Vertex = serde_json::from_str(&json_str).unwrap();

        assert_eq!(loaded.name, "v0");
        assert_eq!(loaded.attribute, "attribute");

        // File I/O test
        json_dump(&vertex, "test_vertex.json", true).unwrap();
        let from_file: Vertex = json_load("test_vertex.json").unwrap();
        assert_eq!(from_file.name, vertex.name);
    }
}
