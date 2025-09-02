use crate::{Point, Vector, Xform};
use serde::{Deserialize, Serialize, Serializer};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt;
use uuid::Uuid;
use crate::{HasJsonData, FromJsonData};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct PointCloud {
    /// The collection of point positions (simple points without transformation data).
    pub points: Vec<Point>,

    /// The collection of normals.
    pub normals: Vec<Vector>,

    /// The transformation matrix for the entire point cloud.
    pub xform: Xform,

    /// Unique identifier
    pub guid: Uuid,
    /// Object name
    pub name: String,
    /// Width values for each point
    pub widths: Vec<f32>,
    /// Color values for each point as RGBA components
    pub pointcolors: Vec<[f32; 4]>,
}

impl PointCloud {
    /// Creates a new `PointCloud` with default `Data`.
    ///
    /// # Arguments
    ///
    /// * `points` - The collection of point positions.
    /// * `normals` - The collection of normals.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Point, Vector};
    /// use session_rust::collections::PointCloud;
    /// let points = vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0)];
    /// let normals = vec![Vector::new(0.0, 0.0, 1.0), Vector::new(0.0, 1.0, 0.0), Vector::new(1.0, 0.0, 0.0)];
    /// let cloud = PointCloud::new(points, normals);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the number of points, normals, and colors are not equal.
    pub fn new(points: Vec<Point>, normals: Vec<Vector>) -> Self {
        let widths = vec![1.0; points.len()]; // Default width of 1.0 for all points
        let pointcolors = vec![[1.0, 1.0, 1.0, 1.0]; points.len()]; // Default white color for all points
        Self {
            points,
            normals,
            xform: Xform::default(),
            guid: Uuid::new_v4(),
            name: "PointCloud".to_string(),
            widths,
            pointcolors,
        }
    }
}


impl AddAssign<&Vector> for PointCloud {
    /// Adds the coordinates of another point to this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::PointCloud;
    /// use session_rust::primitives::{Point, Vector};
    /// let mut c = PointCloud::new(vec![Point::new(1.0, 2.0, 3.0)], vec![Vector::new(0.0, 0.0, 1.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// c += &v;
    /// assert_eq!(c.points[0].x, 5.0);
    /// assert_eq!(c.points[0].y, 7.0);
    /// assert_eq!(c.points[0].z, 9.0);
    /// ```
    fn add_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            p.x += other.x;
            p.y += other.y;
            p.z += other.z;
        }
    }
}

impl Add<&Vector> for PointCloud {
    type Output = PointCloud;

    /// Adds the coordinates of a vector to this point and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::PointCloud;
    /// use session_rust::primitives::{Point, Vector};
    /// let c = PointCloud::new(vec![Point::new(1.0, 2.0, 3.0)], vec![Vector::new(0.0, 0.0, 1.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let c2 = c + &v;
    /// assert_eq!(c2.points[0].x, 5.0);
    /// assert_eq!(c2.points[0].y, 7.0);
    /// assert_eq!(c2.points[0].z, 9.0);
    /// ```
    fn add(self, other: &Vector) -> PointCloud {
        let mut c = self.clone();
        for p in &mut c.points {
            p.x += other.x;
            p.y += other.y;
            p.z += other.z;
        }
        return c;
    }
}



impl SubAssign <&Vector> for PointCloud {
    /// Adds the coordinates of another point to this point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::PointCloud;
    /// use session_rust::primitives::{Point, Vector};
    /// let mut c = PointCloud::new(vec![Point::new(1.0, 2.0, 3.0)], vec![Vector::new(0.0, 0.0, 1.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// c -= &v;
    /// assert_eq!(c.points[0].x, -3.0);
    /// assert_eq!(c.points[0].y, -3.0);
    /// assert_eq!(c.points[0].z, -3.0);
    /// ```
    fn sub_assign(&mut self, other: &Vector) {
        for p in &mut self.points {
            p.x -= other.x;
            p.y -= other.y;
            p.z -= other.z;
        }
    }
}

impl Sub<&Vector> for PointCloud {
    type Output = PointCloud;

    /// Adds the coordinates of a vector to this point and returns a new point.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::PointCloud;
    /// use session_rust::primitives::{Point, Vector};
    /// let c = PointCloud::new(vec![Point::new(1.0, 2.0, 3.0)], vec![Vector::new(0.0, 0.0, 1.0)]);
    /// let v = Vector::new(4.0, 5.0, 6.0);
    /// let c2 = c - &v;
    /// assert_eq!(c2.points[0].x, -3.0);
    /// assert_eq!(c2.points[0].y, -3.0);
    /// assert_eq!(c2.points[0].z, -3.0);
    /// ```
    fn sub(self, other: &Vector) -> PointCloud {
        let mut c = self.clone();
        for p in &mut c.points {
            p.x -= other.x;
            p.y -= other.y;
            p.z -= other.z;
        }
        return c;
    }
}

// Custom Serialize implementation for simple format compatible with wink
impl Serialize for PointCloud {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("PointCloud", 5)?;
        state.serialize_field("points", &self.points)?;
        state.serialize_field("normals", &self.normals)?;
        state.serialize_field("widths", &self.widths)?;
        state.serialize_field("xform", &self.xform)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

impl fmt::Display for PointCloud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PointCloud {{ points: {}, normals: {}, widths: {}, name: {} }}",
            self.points.len(),
            self.normals.len(),
            self.widths.len(),
            self.name
        )
    }
}

impl HasJsonData for PointCloud {
    fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "PointCloud",
            "points": self.points.iter().map(|p| p.to_json_data(false)).collect::<Vec<_>>(),
            "normals": self.normals.iter().map(|n| n.to_json_data(false)).collect::<Vec<_>>(),
            "xform": {
                "dtype": "Xform",
                "m": self.xform.m
            },
            "guid": self.guid.to_string(),
            "name": self.name,
            "widths": self.widths,
            "pointcolors": self.pointcolors
        })
    }
}

impl FromJsonData for PointCloud {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let (
            Some(points_array), Some(normals_array),
            Some(xform_obj), Some(guid_str), Some(name),
            Some(widths_array)
        ) = (
            data.get("points").and_then(|v| v.as_array()),
            data.get("normals").and_then(|v| v.as_array()),
            data.get("xform"),
            data.get("guid").and_then(|v| v.as_str()),
            data.get("name").and_then(|v| v.as_str()),
            data.get("widths").and_then(|v| v.as_array()),
        ) {
            let points: Vec<Point> = points_array.iter()
                .filter_map(|v| Point::from_json_data(v))
                .collect();
            let normals: Vec<Vector> = normals_array.iter()
                .filter_map(|v| Vector::from_json_data(v))
                .collect();
            let xform_m = xform_obj.get("m")?.as_array()?;
            let m: Vec<f32> = xform_m.iter().filter_map(|x| x.as_f64().map(|f| f as f32)).collect();
            if m.len() != 16 { return None; }
            let xform = Xform { m: [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15]] };
            let widths: Vec<f32> = widths_array.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect();
            
            // Extract pointcolors from individual points
            let pointcolors: Vec<[f32; 4]> = points.iter().map(|p| p.pointcolor.to_float_array()).collect();
            
            let guid = uuid::Uuid::parse_str(guid_str).ok()?;
            Some(PointCloud {
                points,
                normals,
                xform,
                guid,
                name: name.to_string(),
                widths,
                pointcolors,
            })
        } else {
            None
        }
    }
}
