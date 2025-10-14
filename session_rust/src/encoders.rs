use serde::{Deserialize, Serialize};
use std::fs;

/// Serialize data to JSON string with pretty formatting.
pub fn json_dumps<T: Serialize>(
    data: &T,
    pretty: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    if pretty {
        Ok(serde_json::to_string_pretty(data)?)
    } else {
        Ok(serde_json::to_string(data)?)
    }
}

/// Deserialize data from JSON string.
pub fn json_loads<T: for<'de> Deserialize<'de>>(
    json_str: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(json_str)?)
}

/// Write data to JSON file with pretty formatting.
pub fn json_dump<T: Serialize>(
    data: &T,
    filepath: &str,
    pretty: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_str = json_dumps(data, pretty)?;
    fs::write(filepath, json_str)?;
    Ok(())
}

/// Read data from JSON file.
pub fn json_load<T: for<'de> Deserialize<'de>>(
    filepath: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(filepath)?;
    json_loads(&json_str)
}

/// Encode a value to JSON (wrapper for Serialize types).
/// This function automatically calls the serde serialization.
pub fn encode_value<T: Serialize>(
    value: &T,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::to_value(value)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::Line;
    use crate::point::Point;
    use crate::vector::Vector;

    #[test]
    fn test_json_dump_and_load() {
        let mut original = Point::new(1.5, 2.5, 3.5);
        original.name = "test_point".to_string();

        let filepath = "test_encoders_point.json";
        json_dump(&original, filepath, true).unwrap();

        let loaded: Point = json_load(filepath).unwrap();

        assert_eq!(loaded.x(), original.x());
        assert_eq!(loaded.y(), original.y());
        assert_eq!(loaded.z(), original.z());
        assert_eq!(loaded.name, original.name);

        std::fs::remove_file(filepath).ok();
    }

    #[test]
    fn test_json_dumps_and_loads() {
        let mut original = Vector::new(42.1, 84.2, 126.3);
        original.name = "test_vector".to_string();

        let json_str = json_dumps(&original, true).unwrap();
        assert!(!json_str.is_empty());
        assert!(json_str.contains("Vector"));

        let loaded: Vector = json_loads(&json_str).unwrap();

        assert_eq!(loaded.x(), original.x());
        assert_eq!(loaded.y(), original.y());
        assert_eq!(loaded.z(), original.z());
        assert_eq!(loaded.name, original.name);
    }

    #[test]
    fn test_encode_collection() {
        let points = vec![
            Point::new(1.0, 2.0, 3.0),
            Point::new(4.0, 5.0, 6.0),
            Point::new(7.0, 8.0, 9.0),
        ];

        let json_str = json_dumps(&points, true).unwrap();
        assert!(!json_str.is_empty());

        let loaded: Vec<Point> = json_loads(&json_str).unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded[0].x(), 1.0);
        assert_eq!(loaded[1].y(), 5.0);
        assert_eq!(loaded[2].z(), 9.0);
    }

    #[test]
    fn test_nested_collections() {
        let lines = vec![
            Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            Line::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
        ];

        let json_str = json_dumps(&lines, true).unwrap();
        let loaded: Vec<Line> = json_loads(&json_str).unwrap();

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].end().x(), 1.0);
        assert_eq!(loaded[1].end().y(), 1.0);
    }

    #[test]
    fn test_roundtrip_with_file() {
        let vectors = vec![
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
        ];

        let filepath = "test_encoders_collection.json";
        json_dump(&vectors, filepath, true).unwrap();

        let loaded: Vec<Vector> = json_load(filepath).unwrap();

        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded[0].x(), 1.0);
        assert_eq!(loaded[1].y(), 1.0);
        assert_eq!(loaded[2].z(), 1.0);

        std::fs::remove_file(filepath).ok();
    }

    #[test]
    fn test_pretty_vs_compact() {
        let point = Point::new(1.0, 2.0, 3.0);

        let pretty = json_dumps(&point, true).unwrap();
        let compact = json_dumps(&point, false).unwrap();

        assert!(pretty.len() > compact.len());
        assert!(pretty.contains("\n"));
        assert!(!compact.contains("\n"));

        let loaded_pretty: Point = json_loads(&pretty).unwrap();
        let loaded_compact: Point = json_loads(&compact).unwrap();

        assert_eq!(loaded_pretty.x(), 1.0);
        assert_eq!(loaded_compact.x(), 1.0);
    }

    #[test]
    fn test_decode_primitives() {
        let num: i32 = 42;
        let json_str = json_dumps(&num, false).unwrap();
        let loaded: i32 = json_loads(&json_str).unwrap();
        assert_eq!(loaded, 42);

        let float: f64 = 2.5;
        let json_str = json_dumps(&float, false).unwrap();
        let loaded: f64 = json_loads(&json_str).unwrap();
        assert_eq!(loaded, 2.5);

        let text = "hello";
        let json_str = json_dumps(&text, false).unwrap();
        let loaded: String = json_loads(&json_str).unwrap();
        assert_eq!(loaded, "hello");

        let flag = true;
        let json_str = json_dumps(&flag, false).unwrap();
        let loaded: bool = json_loads(&json_str).unwrap();
        assert!(loaded);
    }

    #[test]
    fn test_decode_list() {
        let data = vec![1, 2, 3];
        let json_str = json_dumps(&data, false).unwrap();
        let loaded: Vec<i32> = json_loads(&json_str).unwrap();
        assert_eq!(loaded, vec![1, 2, 3]);

        let points = vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)];
        let json_str = json_dumps(&points, false).unwrap();
        let loaded: Vec<Point> = json_loads(&json_str).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].x(), 1.0);
        assert_eq!(loaded[1].x(), 4.0);
    }

    #[test]
    fn test_decode_dict() {
        use std::collections::HashMap;

        let mut data = HashMap::new();
        data.insert("a".to_string(), 1);
        data.insert("b".to_string(), 2);
        let json_str = json_dumps(&data, false).unwrap();
        let loaded: HashMap<String, i32> = json_loads(&json_str).unwrap();
        assert_eq!(loaded.get("a"), Some(&1));
        assert_eq!(loaded.get("b"), Some(&2));

        let vec = Vector::new(1.0, 2.0, 3.0);
        let json_str = json_dumps(&vec, false).unwrap();
        let loaded: Vector = json_loads(&json_str).unwrap();
        assert_eq!(loaded.x(), 1.0);
    }

    #[test]
    fn test_list_in_list_in_list() {
        let data = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
        let json_str = json_dumps(&data, false).unwrap();
        let loaded: Vec<Vec<Vec<i32>>> = json_loads(&json_str).unwrap();

        assert_eq!(loaded[0][0][0], 1);
        assert_eq!(loaded[1][1][1], 8);
        assert_eq!(loaded.len(), 2);
    }

    #[test]
    fn test_dict_of_lists() {
        use serde_json::json;

        let points = vec![Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)];

        let data = json!({
            "numbers": [1, 2, 3],
            "letters": ["a", "b", "c"],
            "points": points
        });

        let json_str = data.to_string();
        let loaded: serde_json::Value = json_loads(&json_str).unwrap();

        assert_eq!(loaded["numbers"].as_array().unwrap().len(), 3);
        assert_eq!(loaded["letters"][0], "a");

        let loaded_points: Vec<Point> = serde_json::from_value(loaded["points"].clone()).unwrap();
        assert_eq!(loaded_points.len(), 2);
        assert_eq!(loaded_points[0].x(), 1.0);
    }

    #[test]
    fn test_list_of_dict() {
        use serde_json::json;

        let point = Point::new(1.0, 2.0, 3.0);

        let data = json!([
            {"name": "point1", "value": 10},
            {"name": "point2", "value": 20},
            {"geometry": point}
        ]);

        let json_str = data.to_string();
        let loaded: serde_json::Value = json_loads(&json_str).unwrap();

        assert_eq!(loaded.as_array().unwrap().len(), 3);
        assert_eq!(loaded[0]["name"], "point1");
        assert_eq!(loaded[1]["value"], 20);

        let loaded_point: Point = serde_json::from_value(loaded[2]["geometry"].clone()).unwrap();
        assert_eq!(loaded_point.z(), 3.0);
    }

    #[test]
    fn test_dict_of_dicts() {
        use serde_json::json;

        let point = Point::new(1.0, 2.0, 3.0);
        let vec = Vector::new(0.0, 0.0, 1.0);

        let data = json!({
            "config": {
                "tolerance": 0.001,
                "scale": 1000
            },
            "geometry": {
                "point": point,
                "vector": vec
            }
        });

        let json_str = data.to_string();
        let loaded: serde_json::Value = json_loads(&json_str).unwrap();

        assert_eq!(loaded["config"]["tolerance"], 0.001);
        assert_eq!(loaded["config"]["scale"], 1000);

        let loaded_point: Point =
            serde_json::from_value(loaded["geometry"]["point"].clone()).unwrap();
        let loaded_vec: Vector =
            serde_json::from_value(loaded["geometry"]["vector"].clone()).unwrap();
        assert_eq!(loaded_point.x(), 1.0);
        assert_eq!(loaded_vec.z(), 1.0);
    }
}
