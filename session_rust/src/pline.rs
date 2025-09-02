use crate::{Point, Vector, Plane, Mesh};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt;
use uuid::Uuid;
use crate::{HasJsonData, FromJsonData};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Pline {
    /// The collection of points.
    pub points: Vec<Point>,
    /// The plane of the polyline.
    pub plane: Plane,
    /// Unique identifier
    pub guid: Uuid,
    /// Object name
    pub name: String,
    /// Point color as RGBA components [r, g, b, a]
    pub pointcolor: [f32; 4],
    /// Line color as RGBA components [r, g, b, a]
    pub linecolor: [f32; 4],
    /// Width value
    pub width: f32,
}

// Custom serialization to skip xform field
impl Serialize for Pline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Pline", 7)?;
        state.serialize_field("points", &self.points)?;
        state.serialize_field("plane", &self.plane)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("pointcolor", &self.pointcolor)?;
        state.serialize_field("linecolor", &self.linecolor)?;
        state.serialize_field("width", &self.width)?;
        state.end()
    }
}

impl Pline {
    /// Creates a new `Pline` with default `Data`.
    ///
    /// # Arguments
    ///
    /// * `points` - The collection of points.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Point;
    /// use session_rust::primitives::Plane;
    /// use session_rust::primitives::Pline;
    /// let points = vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)];
    /// let Pline = Pline::new(points);
    /// ```
    ///
    pub fn new(points: Vec<Point>) -> Self {

        // Delegate plane computation to Plane::plane_from_points
        let plane = Plane::plane_from_points(&points);

        Self {
            points,
            plane,
            guid: Uuid::new_v4(),
            name: "Pline".to_string(),
            pointcolor: [1.0, 1.0, 1.0, 1.0],
            linecolor: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
        }
    }
    
    /// Convert polyline segments to pipe meshes for visualization.
    /// Each segment between consecutive points becomes a cylindrical pipe mesh.
    /// 
    /// # Arguments
    /// 
    /// * `radius` - The radius of the pipe meshes (uses data.thickness if None)
    /// * `sides` - Number of sides for the cylindrical pipes (default: 8)
    /// 
    /// # Returns
    /// 
    /// A vector of `Mesh` objects, one for each segment in the polyline.
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::primitives::{Point, Pline};
    /// let points = vec![
    ///     Point::new(0.0, 0.0, 0.0),
    ///     Point::new(1.0, 0.0, 0.0),
    ///     Point::new(1.0, 1.0, 0.0)
    /// ];
    /// let pline = Pline::new(points);
    /// let pipe_meshes = pline.to_pipe_meshes(Some(0.1), None);
    /// assert_eq!(pipe_meshes.len(), 2); // Two segments
    /// ```
    pub fn to_pipe_meshes(&self, radius: Option<f32>, sides: Option<usize>) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        
        // Need at least 2 points to create segments
        if self.points.len() < 2 {
            return meshes;
        }
        
        // Use provided radius or fall back to data width
        let pipe_radius = radius.unwrap_or_else(|| {
            self.width
        });
        let _pipe_sides = sides.unwrap_or(8);
        
        // Create a pipe mesh for each segment
        for i in 0..self.points.len() - 1 {
            let start_point = &self.points[i];
            let end_point = &self.points[i + 1];
            
            // Create pipe mesh for this segment
            let mut pipe_mesh = Mesh::create_pipe(start_point.clone(), end_point.clone(), pipe_radius);
            pipe_mesh.facecolors = self.linecolor;
            pipe_mesh.linecolors = self.linecolor;
            pipe_mesh.pointcolors = self.pointcolor;
            meshes.push(pipe_mesh);
        }
        
        meshes
    }
}


impl AddAssign<&Vector> for Pline {
    /// Adds the coordinates of another point to this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Pline, Point, Vector};
    /// let mut c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// c += &v;
    /// assert_eq!(c.points[0].x, 5.0);
    /// assert_eq!(c.points[0].y, 7.0);
    /// assert_eq!(c.points[0].z, 9.0);
    /// assert_eq!(c.points[1].x, 8.0);
    /// assert_eq!(c.points[1].y, 10.0);
    /// assert_eq!(c.points[1].z, 12.0);
    /// ```
    fn add_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            p.x += other.x;
            p.y += other.y;
            p.z += other.z;
        }
    }
}

impl Add<&Vector> for Pline {
    type Output = Pline;

    /// Adds the coordinates of a vector to this point and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Pline, Point, Vector};
    /// let c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let c2 = c + &v;
    /// assert_eq!(c2.points[0].x, 5.0);
    /// assert_eq!(c2.points[0].y, 7.0);
    /// assert_eq!(c2.points[0].z, 9.0);
    /// ```
    fn add(self, other: &Vector) -> Pline {
        let mut c = self.clone();
        for p in &mut c.points {
            p.x += other.x;
            p.y += other.y;
            p.z += other.z;
        }
        return c;
    }
}



impl SubAssign <&Vector> for Pline {
    /// Adds the coordinates of another point to this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Pline, Point, Vector};
    /// let mut c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// c -= &v;
    /// assert_eq!(c.points[0].x, -3.0);
    /// assert_eq!(c.points[0].y, -3.0);
    /// assert_eq!(c.points[0].z, -3.0);
    /// assert_eq!(c.points[1].x, 0.0);
    /// assert_eq!(c.points[1].y, 0.0);
    /// assert_eq!(c.points[1].z, 0.0);
    /// ```
    fn sub_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            p.x -= other.x;
            p.y -= other.y;
            p.z -= other.z;
        }
    }
}

impl Sub<&Vector> for Pline {
    type Output = Pline;

    /// Adds the coordinates of a vector to this point and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Pline, Point, Vector};
    /// let c = Pline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let c2 = c - &v;
    /// assert_eq!(c2.points[0].x, -3.0);
    /// assert_eq!(c2.points[0].y, -3.0);
    /// assert_eq!(c2.points[0].z, -3.0);
    /// assert_eq!(c2.points[1].x, 0.0);
    /// assert_eq!(c2.points[1].y, 0.0);
    /// assert_eq!(c2.points[1].z, 0.0);
    /// ```
    fn sub(self, other: &Vector) -> Pline {
        let mut c = self.clone();
        for p in &mut c.points {
            p.x -= other.x;
            p.y -= other.y;
            p.z -= other.z;
        }
        return c;
    }
}

impl fmt::Display for Pline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pline {{ points: {}, data: {} }}",
            self.points.len(),
            serde_json::json!({
                "dtype": "Pline",
                "points": self.points,
                "plane": self.plane,
                "guid": self.guid,
                "name": self.name
            })
        )
    }
}

impl HasJsonData for Pline {
    fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "Pline",
            "points": self.points.iter().map(|p| p.to_json_data(false)).collect::<Vec<_>>(),
            "plane": self.plane.to_json_data(false),
            "guid": self.guid.to_string(),
            "name": self.name
        })
    }
}

impl FromJsonData for Pline {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let (
            Some(points_array), Some(plane_obj),
            Some(guid_str), Some(name)
        ) = (
            data.get("points").and_then(|v| v.as_array()),
            data.get("plane"),
            data.get("guid").and_then(|v| v.as_str()),
            data.get("name").and_then(|v| v.as_str()),
        ) {
            let points: Vec<Point> = points_array.iter()
                .filter_map(|v| Point::from_json_data(v))
                .collect();
            let plane = Plane::from_json_data(plane_obj)?;
            let guid = uuid::Uuid::parse_str(guid_str).ok()?;
            Some(Pline {
                points,
                plane,
                guid,
                name: name.to_string(),
                pointcolor: data.get("pointcolor").and_then(|v| v.as_array()).and_then(|arr| {
                    Some([
                        arr.get(0)?.as_f64()? as f32,
                        arr.get(1)?.as_f64()? as f32,
                        arr.get(2)?.as_f64()? as f32,
                        arr.get(3)?.as_f64()? as f32,
                    ])
                }).unwrap_or([1.0, 1.0, 1.0, 1.0]),
                linecolor: data.get("linecolor").and_then(|v| v.as_array()).and_then(|arr| {
                    Some([
                        arr.get(0)?.as_f64()? as f32,
                        arr.get(1)?.as_f64()? as f32,
                        arr.get(2)?.as_f64()? as f32,
                        arr.get(3)?.as_f64()? as f32,
                    ])
                }).unwrap_or([1.0, 1.0, 1.0, 1.0]),
                width: data.get("width").and_then(|v| v.as_f64()).map(|v| v as f32).unwrap_or(1.0),
            })
        } else {
            None
        }
    }
}
