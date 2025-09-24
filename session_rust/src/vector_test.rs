#[cfg(test)]
mod tests {
    use crate::Vector;

    #[test]
    fn test_vector_constructor() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        assert!(!vector.guid.to_string().is_empty());
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
    }

    #[test]
    fn test_vector_equality() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let mut v2 = Vector::new(1.0, 2.0, 3.0);

        // Set same GUID to make them equal
        v2.guid = v1.guid.clone();
        v2.name = v1.name.clone();
        assert_eq!(v1, v2);
        assert!(!(v1 != v2));

        let v3 = Vector::new(1.0, 2.0, 3.0);
        let mut v4 = Vector::new(1.1, 2.0, 3.0);
        v4.guid = v3.guid.clone();
        v4.name = v3.name.clone();
        assert_ne!(v3, v4);
        assert!(v3 != v4);
    }

    #[test]
    fn test_vector_to_json_data() {
        let mut vector = Vector::new(10.5, 20.7, 30.9);
        vector.name = "force_vector_X".to_string();

        let json_result = vector.to_json_data();
        assert!(json_result.is_ok());

        let json_data = json_result.unwrap();
        assert!(json_data.contains("\"type\": \"Vector\""));
        assert!(json_data.contains("\"name\": \"force_vector_X\""));
        assert!(json_data.contains("\"x\": 10.5"));
        assert!(json_data.contains("\"y\": 20.7"));
        assert!(json_data.contains("\"z\": 30.9"));
        assert!(json_data.contains("\"guid\""));
    }

    #[test]
    fn test_vector_from_json_data() {
        let original_vector = Vector::new(45.1, 67.8, 89.2);
        let json_data = original_vector.to_json_data().unwrap();
        let restored_vector = Vector::from_json_data(&json_data).unwrap();

        assert_eq!(restored_vector.x, 45.1);
        assert_eq!(restored_vector.y, 67.8);
        assert_eq!(restored_vector.z, 89.2);
        assert_eq!(restored_vector.guid, original_vector.guid);
    }

    #[test]
    fn test_vector_to_json_from_json() {
        let original = Vector::new(100.25, 200.50, 300.75);
        let filename = "test_vector.json";

        // Save to file
        let save_result = original.to_json(filename);
        assert!(save_result.is_ok());

        // Load from file
        let loaded_result = Vector::from_json(filename);
        assert!(loaded_result.is_ok());

        let loaded = loaded_result.unwrap();
        assert_eq!(loaded.x, original.x);
        assert_eq!(loaded.y, original.y);
        assert_eq!(loaded.z, original.z);
        assert_eq!(loaded.name, original.name);
        assert_eq!(loaded.guid, original.guid);
    }

    #[test]
    fn test_vector_default() {
        let vector = Vector::default();
        assert_eq!(vector.x, 0.0);
        assert_eq!(vector.y, 0.0);
        assert_eq!(vector.z, 0.0);
        assert!(!vector.guid.to_string().is_empty());
        assert_eq!(vector.name, "my_vector");
    }

    #[test]
    fn test_vector_display() {
        let vector = Vector::new(1.5, 2.5, 3.5);
        let display_string = format!("{vector}");
        assert!(display_string.contains("Vector("));
        assert!(display_string.contains("1.5"));
        assert!(display_string.contains("2.5"));
        assert!(display_string.contains("3.5"));
        assert!(display_string.contains(&vector.guid.to_string()));
        assert!(display_string.contains(&vector.name));
    }
}
