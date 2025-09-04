use std::fmt;
use serde::{Deserialize, Serialize, ser::Serialize as SerTrait};
use uuid::Uuid;
use crate::Color;

/// A 3D point with visual properties and cross-language JSON serialization support.
///
/// This structure represents a point in 3D space along with visual properties
/// such as color, width, and name. It provides JSON serialization/deserialization
/// for interoperability between Rust, Python, and C++ implementations.
///
/// # Fields
///
/// * `x` - X coordinate (f32)
/// * `y` - Y coordinate (f32)  
/// * `z` - Z coordinate (f32)
/// * `guid` - Unique identifier (Uuid)
/// * `name` - Point name (String)
/// * `pointcolor` - Point color (Color)
/// * `width` - Point width for display (f32)
///
/// # Examples
///
/// ```rust
/// use session_rust::{Point, Color};
///
/// let mut point = Point::new(1.0, 2.0, 3.0);
/// point.name = "my_point".to_string();
/// point.pointcolor = Color::new(255, 0, 0, 255);
/// println!("Point: {}", point);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Point")]
pub struct Point {
    pub guid: Uuid,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub pointcolor: Color,
    
}


impl Point {
    /// Creates a new Point with specified coordinates.
    ///
    /// The point is initialized with default properties: a generated UUID,
    /// name "my_point", white color, and width 1.0.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `z` - Z coordinate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use session_rust::Point;
    /// 
    /// let point = Point::new(10.5, 20.0, -5.3);
    /// assert_eq!(point.x, 10.5);
    /// assert_eq!(point.y, 20.0);
    /// assert_eq!(point.z, -5.3);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            guid: Uuid::new_v4(),
            name: "my_point".to_string(),
            pointcolor: Color::white(),
            width: 1.0,
        }
    }

    /// Serializes the Point to a JSON string with pretty formatting.
    ///
    /// This method creates a formatted JSON representation of the point
    /// for cross-language compatibility with Python and C++ implementations.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The JSON string representation
    /// * `Err(Box<dyn std::error::Error>)` - If serialization fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use session_rust::Point;
    /// 
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// let json = point.to_json_data().unwrap();
    /// println!("JSON: {}", json);
    /// ```
    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes a Point from a JSON string.
    ///
    /// Creates a Point instance from JSON data, enabling cross-language
    /// data exchange with Python and C++ implementations.
    ///
    /// # Arguments
    ///
    /// * `json_data` - JSON string representation of the Point
    ///
    /// # Returns
    ///
    /// * `Ok(Point)` - The deserialized point
    /// * `Err(Box<dyn std::error::Error>)` - If deserialization fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use session_rust::Point;
    /// 
    /// let json = r#"{"type":"Point","x":1.0,"y":2.0,"z":3.0}"#;
    /// let point = Point::from_json_data(json).unwrap();
    /// assert_eq!(point.x, 1.0);
    /// ```
    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Point to a JSON file.
    ///
    /// Saves the point as a formatted JSON file for persistence
    /// or cross-language data exchange.
    ///
    /// # Arguments
    ///
    /// * `filepath` - Path to the output JSON file
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the file was written successfully
    /// * `Err(Box<dyn std::error::Error>)` - If writing fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use session_rust::Point;
    /// 
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// point.to_json("point.json").unwrap();
    /// ```
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes a Point from a JSON file.
    ///
    /// Loads a Point instance from a JSON file created by any
    /// implementation (Rust, Python, or C++).
    ///
    /// # Arguments
    ///
    /// * `filepath` - Path to the JSON file to load
    ///
    /// # Returns
    ///
    /// * `Ok(Point)` - The loaded point
    /// * `Err(Box<dyn std::error::Error>)` - If loading fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use session_rust::Point;
    /// 
    /// let point = Point::from_json("point.json").unwrap();
    /// println!("Loaded point: {}", point);
    /// ```
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Point({}, {}, {}, {}, {}, {}, {})", 
            self.x, self.y, self.z, self.guid, self.name, self.pointcolor, self.width
        )
    }
}
