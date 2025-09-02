use serde_json::Value;
use std::fmt;

/// Smart JSON data wrapper that allows direct access like Python COMPAS
/// No casting or unwrapping needed!
#[derive(Debug, Clone)]
pub struct JsonData {
    value: Value,
}

impl JsonData {
    pub fn new(value: Value) -> Self {
        JsonData { value }
    }
    
    /// Get string value directly
    pub fn as_string(&self) -> String {
        match &self.value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            _ => "unnamed".to_string(),
        }
    }
    
    /// Get number value directly
    pub fn as_number(&self) -> f64 {
        match &self.value {
            Value::Number(n) => n.as_f64().unwrap_or(0.0),
            Value::String(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
    
    /// Check if this is an array
    pub fn is_array(&self) -> bool {
        self.value.is_array()
    }
    
    /// Get array length
    pub fn len(&self) -> usize {
        match &self.value {
            Value::Array(arr) => arr.len(),
            _ => 0,
        }
    }
    
    /// Iterate over array items
    pub fn iter(&self) -> JsonArrayIter<'_> {
        match &self.value {
            Value::Array(arr) => JsonArrayIter { items: Some(arr.iter()), index: 0 },
            _ => JsonArrayIter { items: None, index: 0 },
        }
    }
}

/// Iterator for JsonData arrays
pub struct JsonArrayIter<'a> {
    items: Option<std::slice::Iter<'a, Value>>,
    index: usize,
}

impl<'a> Iterator for JsonArrayIter<'a> {
    type Item = (usize, JsonData);
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.items {
            if let Some(item) = iter.next() {
                let result = (self.index, JsonData::new(item.clone()));
                self.index += 1;
                return Some(result);
            }
        }
        None
    }
}

// Note: Index trait removed - use get() method instead for safe access

/// Better approach: get() method that returns JsonData
impl JsonData {
    pub fn get(&self, key: &str) -> JsonData {
        match &self.value {
            Value::Object(map) => {
                if let Some(val) = map.get(key) {
                    JsonData::new(val.clone())
                } else {
                    JsonData::new(Value::Null)
                }
            },
            _ => JsonData::new(Value::Null),
        }
    }
    
    pub fn get_index(&self, index: usize) -> JsonData {
        match &self.value {
            Value::Array(arr) => {
                if let Some(val) = arr.get(index) {
                    JsonData::new(val.clone())
                } else {
                    JsonData::new(Value::Null)
                }
            },
            _ => JsonData::new(Value::Null),
        }
    }
}

impl fmt::Display for JsonData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

/// Simplified JSON handler for geometry types with unified Data structure
pub struct JsonHandler;

impl JsonHandler {
    pub fn new() -> Self {
        JsonHandler
    }
    
    /// Serialize a geometry object to JSON similar to COMPAS json_dumps
    pub fn dumps(&self, json_data: Value, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(&json_data)
        } else {
            serde_json::to_string(&json_data)
        }
    }
    
    /// Save to JSON file similar to COMPAS json_dump
    pub fn dump(&self, json_data: Value, path: &str, pretty: bool) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = self.dumps(json_data, pretty)?;
        std::fs::write(path, json_str)?;
        Ok(())
    }
    
    /// Load JSON from string similar to COMPAS json_loads
    pub fn loads(&self, json_str: &str) -> Result<Value, serde_json::Error> {
        serde_json::from_str(json_str)
    }
    
    /// Load JSON from file similar to COMPAS json_load
    pub fn load(&self, path: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let json_str = std::fs::read_to_string(path)?;
        let json_data = self.loads(&json_str)?;
        Ok(json_data)
    }
    
    /// Ultra-simple: Save geometry object to file in one line
    pub fn save_object<T: HasJsonData>(&self, obj: &T, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = obj.to_json_data(false);
        self.dump(json_data, path, true)
    }
    
    /// Ultra-simple: Save collection of geometry objects to file in one line
    pub fn save_collection(&self, objects: Vec<Value>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = serialize_collection(objects, true)?;
        std::fs::write(path, json_str)?;
        Ok(())
    }
    
    /// Ultra-simple: Load geometry data from file in one line
    pub fn load_geometry(&self, path: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.load(path)
    }
}

/// Trait for objects that can provide JSON data
pub trait HasJsonData {
    fn to_json_data(&self, minimal: bool) -> Value;
}

/// Trait for objects that can be created from JSON data
pub trait FromJsonData: Sized {
    fn from_json_data(data: &Value) -> Option<Self>;
}

/// Vec of geometry objects can be deserialized from JSON arrays
impl<T: FromJsonData> FromJsonData for Vec<T> {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let Value::Array(arr) = data {
            let mut result = Vec::new();
            for item in arr {
                if let Some(obj) = T::from_json_data(item) {
                    result.push(obj);
                }
            }
            Some(result)
        } else {
            None
        }
    }
}

/// Collection serialization for arrays/vectors of geometry objects
pub fn serialize_collection(json_data: Vec<Value>, pretty: bool) -> Result<String, serde_json::Error> {
    if pretty {
        serde_json::to_string_pretty(&json_data)
    } else {
        serde_json::to_string(&json_data)
    }
}

// ==============================================
// UNIFIED SMART API (COMPAS-style)
// ==============================================

/// Trait for anything that can be JSON serialized
pub trait JsonSerializable {
    fn to_json_value(&self) -> Value;
}

/// Single geometry objects implement JsonSerializable
impl<T: HasJsonData> JsonSerializable for T {
    fn to_json_value(&self) -> Value {
        self.to_json_data(false)
    }
}

/// Implementation for Vec<T> where T has JSON data
impl<T: HasJsonData> JsonSerializable for Vec<T> {
    fn to_json_value(&self) -> Value {
        let json_objects: Vec<Value> = self.iter().map(|item| item.to_json_data(false)).collect();
        Value::Array(json_objects)
    }
}

/// Implementation for nested Vec<Vec<T>> - enables collections of collections!
impl<T: HasJsonData> JsonSerializable for Vec<Vec<T>> {
    fn to_json_value(&self) -> Value {
        let nested_arrays: Vec<Value> = self.iter()
            .map(|inner_vec| {
                let json_objects: Vec<Value> = inner_vec.iter()
                    .map(|item| item.to_json_data(false))
                    .collect();
                Value::Array(json_objects)
            })
            .collect();
        Value::Array(nested_arrays)
    }
}

/// Vec of pre-serialized Values implements JsonSerializable
impl JsonSerializable for Vec<Value> {
    fn to_json_value(&self) -> Value {
        Value::Array(self.clone())
    }
}

/// PERFECTLY SIMPLE: Save ANY geometry object or collection (COMPAS-style json_dump)
/// 
/// No error handling needed - just works!
/// 
/// Works for:
/// - Single objects: `json_dump(&point, "file.json")`
/// - Collections: `json_dump(&vec_of_points, "file.json")`
/// - Pre-serialized: `json_dump(&vec_of_json_values, "file.json")`
/// 
/// # Example
/// ```
/// use session_rust::primitives::Point;
/// use session_rust::common::json_dump;
/// 
/// // Single object - no .unwrap() needed!
/// let point1 = Point::new(1.0, 2.0, 3.0);
/// json_dump(&point1, "point.json");
/// 
/// // Collection of objects - no .unwrap() needed!
/// let point2 = Point::new(4.0, 5.0, 6.0);
/// let point3 = Point::new(7.0, 8.0, 9.0);
/// let points = vec![point1, point2, point3];
/// json_dump(&points, "collection.json");
/// ```
pub fn json_dump<T: JsonSerializable>(obj: &T, path: &str) {
    let json_value = obj.to_json_value();
    let json_string = match serde_json::to_string_pretty(&json_value) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Warning: JSON serialization failed: {}. Using minimal format.", e);
            serde_json::to_string(&json_value).unwrap_or_else(|_| "{}".to_string())
        }
    };
    
    if let Err(e) = std::fs::write(path, json_string) {
        eprintln!("Warning: Failed to write to {}: {}. Data not saved.", path, e);
    }
}

/// PERFECTLY SIMPLE: Load geometry directly as Rust types (COMPAS-style json_load)
/// 
/// No error handling, no casting, no unwrapping - just works!
/// Returns the actual geometry type directly!
/// 
/// # Example  
/// ```no_run
/// use session_rust::primitives::Point;
/// use session_rust::common::json_load;
/// use std::fs::File;
/// use std::io::Write;
/// use serde_json::json;
/// 
/// // For doctest purposes, create a test file
/// let test_data = json!({
///     "dtype": "Point",
///     "x": 1.0,
///     "y": 2.0,
///     "z": 3.0,
///     "name": "TestPoint",
///     "guid": "00000000-0000-0000-0000-000000000000"
/// });
/// 
/// // Write to a temporary file
/// let temp_path = "temp_test_point.json";
/// let mut file = File::create(temp_path).unwrap();
/// file.write_all(test_data.to_string().as_bytes()).unwrap();
/// 
/// // Now load the point
/// let point: Point = json_load(temp_path);
/// println!("Point at ({}, {}, {})", point.x, point.y, point.z);
/// 
/// // Clean up
/// std::fs::remove_file(temp_path).unwrap();
/// ```
pub fn json_load<T: FromJsonData>(path: &str) -> T {
    let json_str = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Warning: Failed to read {}: {}. Using default.", path, e);
            return T::from_json_data(&Value::Object(serde_json::Map::new()))
                .unwrap_or_else(|| panic!("Failed to create default object"));
        }
    };
    
    let json_data: Value = match serde_json::from_str(&json_str) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Warning: Failed to parse JSON from {}: {}. Using default.", path, e);
            Value::Object(serde_json::Map::new())
        }
    };
    
    T::from_json_data(&json_data)
        .unwrap_or_else(|| panic!("Failed to deserialize {} from JSON", std::any::type_name::<T>()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;
    
    #[test]
    fn test_json_serialization() {
        let handler = JsonHandler::new();
        let point = Point::new(1.0, 2.0, 3.0);
        
        // Test serialization using direct field access (no data wrapper)
        let json_data = point.to_json_data(false);
        let json_str = handler.dumps(json_data, true).unwrap();
        assert!(json_str.contains("dtype"));
        assert!(json_str.contains("guid"));
        assert!(json_str.contains("name"));
        assert!(json_str.contains("x"));
        assert!(json_str.contains("y"));
        assert!(json_str.contains("z"));
        assert!(json_str.contains("Point"));
        
        // Test direct access to point fields
        assert_eq!(point.name, "Point");
        assert!(!point.guid.is_nil());
    }
}
