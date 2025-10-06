use crate::{Plane, Point, Vector};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use uuid::Uuid;

/// A polyline defined by a collection of points with an associated plane.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Polyline")]
pub struct Polyline {
    pub guid: String,
    pub name: String,
    pub points: Vec<Point>,
    pub plane: Plane,
}

impl Default for Polyline {
    fn default() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_polyline".to_string(),
            points: Vec::new(),
            plane: Plane::default(),
        }
    }
}

impl Polyline {
    /// Creates a new `Polyline` with default guid and name.
    ///
    /// # Arguments
    ///
    /// * `points` - The collection of points.
    pub fn new(points: Vec<Point>) -> Self {
        // Delegate plane computation to Plane::from_points
        let plane = if points.len() >= 3 {
            Plane::from_points(points.clone())
        } else {
            Plane::default()
        };

        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_polyline".to_string(),
            points,
            plane,
        }
    }

    /// Returns the number of points in the polyline.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns true if the polyline has no points.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns the number of segments in the polyline.
    /// A polyline with n points has n-1 segments.
    pub fn segment_count(&self) -> usize {
        if self.points.len() > 1 {
            self.points.len() - 1
        } else {
            0
        }
    }

    /// Calculates the total length of the polyline.
    pub fn length(&self) -> f32 {
        let mut total_length = 0.0;
        for i in 0..self.segment_count() {
            let mut segment_vector = self.points[i + 1].clone() - self.points[i].clone();
            total_length += segment_vector.magnitude();
        }
        total_length
    }

    /// Returns a reference to the point at the given index.
    pub fn get_point(&self, index: usize) -> Option<&Point> {
        self.points.get(index)
    }

    /// Returns a mutable reference to the point at the given index.
    pub fn get_point_mut(&mut self, index: usize) -> Option<&mut Point> {
        self.points.get_mut(index)
    }

    /// Adds a point to the end of the polyline.
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
        // Recompute plane if we have at least 3 points
        if self.points.len() == 3 {
            self.plane = Plane::from_points(self.points.clone());
        }
    }

    /// Inserts a point at the specified index.
    pub fn insert_point(&mut self, index: usize, point: Point) {
        self.points.insert(index, point);
        // Recompute plane if we have at least 3 points
        if self.points.len() == 3 {
            self.plane = Plane::from_points(self.points.clone());
        }
    }

    /// Removes and returns the point at the specified index.
    pub fn remove_point(&mut self, index: usize) -> Option<Point> {
        if index < self.points.len() {
            let point = self.points.remove(index);
            // Recompute plane if we still have at least 3 points
            if self.points.len() == 3 {
                self.plane = Plane::from_points(self.points.clone());
            }
            Some(point)
        } else {
            None
        }
    }

    /// Reverses the order of points in the polyline.
    pub fn reverse(&mut self) {
        self.points.reverse();
        self.plane.reverse();
    }

    /// Returns a new polyline with reversed point order.
    pub fn reversed(&self) -> Self {
        let mut reversed = self.clone();
        reversed.reverse();
        reversed
    }

    /// Serializes the Polyline to a JSON string.
    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        self.serialize(&mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes a Polyline from a JSON string.
    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Polyline to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes a Polyline from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

impl AddAssign<&Vector> for Polyline {
    /// Translates all points in the polyline by a vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The translation vector.
    fn add_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            *p += other.clone();
        }
        // Update plane origin
        self.plane = Plane::new(
            self.plane.origin() + other.clone(),
            self.plane.x_axis(),
            self.plane.y_axis(),
        );
    }
}

impl Add<&Vector> for Polyline {
    type Output = Polyline;

    /// Translates the polyline by a vector and returns a new polyline.
    ///
    /// # Arguments
    ///
    /// * `other` - The translation vector.
    fn add(self, other: &Vector) -> Polyline {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl SubAssign<&Vector> for Polyline {
    /// Translates all points in the polyline by the negative of a vector.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to subtract.
    fn sub_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            *p -= other.clone();
        }
        // Update plane origin
        self.plane = Plane::new(
            self.plane.origin() - other.clone(),
            self.plane.x_axis(),
            self.plane.y_axis(),
        );
    }
}

impl Sub<&Vector> for Polyline {
    type Output = Polyline;

    /// Translates the polyline by the negative of a vector and returns a new polyline.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to subtract.
    fn sub(self, other: &Vector) -> Polyline {
        let mut result = self.clone();
        result -= other;
        result
    }
}

impl fmt::Display for Polyline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Polyline(guid={}, name={}, points={})",
            self.guid,
            self.name,
            self.points.len()
        )
    }
}
