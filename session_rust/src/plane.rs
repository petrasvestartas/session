use crate::{Point, Vector};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt;
use uuid::Uuid;
use crate::{HasJsonData, FromJsonData};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Plane {
    /// The origin point.
    pub origin: Point,
    /// The x-axis.
    pub xaxis: Vector,
    /// The x-axis.
    pub yaxis: Vector,
    /// The x-axis.
    pub zaxis: Vector,
    /// The normal x coordinate.
    pub a : f32,
    /// The normal y coordinate.
    pub b : f32,
    /// The normal z coordinate.
    pub c : f32,
    /// The plane offset from origin.
    pub d : f32,
    /// Unique identifier
    pub guid: Uuid,
    /// Object name
    pub name: String,
    /// Line color as RGBA components [r, g, b, a]
    pub linecolor: [f32; 4],
    /// Width value
    pub width: f32,
}

// Custom serialization to skip xform field
impl Serialize for Plane {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Plane", 12)?;
        state.serialize_field("origin", &self.origin)?;
        state.serialize_field("xaxis", &self.xaxis)?;
        state.serialize_field("yaxis", &self.yaxis)?;
        state.serialize_field("zaxis", &self.zaxis)?;
        state.serialize_field("a", &self.a)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("c", &self.c)?;
        state.serialize_field("d", &self.d)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("linecolor", &self.linecolor)?;
        state.serialize_field("width", &self.width)?;
        state.end()
    }
}

impl Plane{
    /// Creates a new `Line` with default `Data`.
    ///
    /// # Arguments
    ///
    /// * `x0` - The x components of the start point.
    /// * `y0` - The y components of the start point.
    /// * `z0` - The z components of the start point.
    /// * `x1` - The x components of the end point.
    /// * `y1` - The y components of the end point.
    /// * `z1` - The z components of the end point.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(line.x0, 0.0);
    /// assert_eq!(line.y0, 0.0);
    /// assert_eq!(line.z0, 0.0);
    /// assert_eq!(line.x1, 0.0);
    /// assert_eq!(line.y1, 0.0);
    /// assert_eq!(line.z1, 1.0);
    /// 
    /// ```
    pub fn new(origin: Point, xaxis: Vector, yaxis: Vector) -> Self {
        let zaxis = Vector::cross(&xaxis, &yaxis);
        let a = zaxis.x;
        let b = zaxis.y;
        let c = zaxis.z;
        let d = -a * origin.x - b * origin.y - c * origin.z;
        Plane {
            origin,
            xaxis,
            yaxis,
            zaxis,
            a,
            b,
            c,
            d,
            guid: Uuid::new_v4(),
            name: "Plane".to_string(),
            linecolor: [0.0, 0.0, 0.0, 1.0],
            width: 1.0,
        }
    }

    /// Creates a new `Plane` with a specified name for `Data`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the `Data`.
    /// * `origin` - The origin point.
    /// * `xaxis` - The x-axis.
    /// * `yaxis` - The y-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Plane, Point, Vector};
    /// let plane = Plane::with_name("MyPlane".to_string(), Point::new(0.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    /// assert_eq!(plane.origin.x, 0.0);
    /// assert_eq!(plane.origin.y, 0.0);
    /// assert_eq!(plane.origin.z, 0.0);
    /// assert_eq!(plane.xaxis.x, 1.0);
    /// assert_eq!(plane.xaxis.y, 0.0);
    /// assert_eq!(plane.xaxis.z, 0.0);
    /// assert_eq!(plane.yaxis.x, 0.0);
    /// assert_eq!(plane.yaxis.y, 1.0);
    /// assert_eq!(plane.yaxis.z, 0.0);
    /// assert_eq!(plane.zaxis.x, 0.0);
    /// assert_eq!(plane.zaxis.y, 0.0);
    /// assert_eq!(plane.zaxis.z, 1.0);
    /// assert_eq!(plane.a, 0.0);
    /// assert_eq!(plane.b, 0.0);
    /// assert_eq!(plane.c, 1.0);
    /// assert_eq!(plane.d, 0.0);
    /// ```
    pub fn with_name(name: String, origin: Point, xaxis: Vector, yaxis: Vector) -> Self {
        let zaxis = Vector::cross(&xaxis, &yaxis);
        let a = zaxis.x;
        let b = zaxis.y;
        let c = zaxis.z;
        let d = -a * origin.x - b * origin.y - c * origin.z;
        Plane {
            origin,
            xaxis,
            yaxis,
            zaxis,
            a,
            b,
            c,
            d,
            guid: Uuid::new_v4(),
            name: name.to_string(),
            linecolor: [0.0, 0.0, 0.0, 1.0],
            width: 1.0,
        }
    }

    /// Creates a new `Plane` from a point and a normal vector.
    ///
    /// # Arguments
    ///
    /// * `point` - A point on the plane.
    /// * `normal` - The normal vector of the plane.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Plane, Point, Vector};
    /// let point = Point::new(1.0, 2.0, 3.0);
    /// let normal = Vector::new(0.0, 0.0, 1.0);
    /// let plane = Plane::from_point_normal(&point, &normal);
    /// assert_eq!(plane.origin.x, 1.0);
    /// assert_eq!(plane.origin.y, 2.0);
    /// assert_eq!(plane.origin.z, 3.0);
    /// assert_eq!(plane.zaxis.x, 0.0);
    /// assert_eq!(plane.zaxis.y, 0.0);
    /// assert_eq!(plane.zaxis.z, 1.0);
    /// assert_eq!(plane.d, -3.0); // -(0*1 + 0*2 + 1*3)
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if the normal vector has zero length.
    pub fn from_point_normal(point: &Point, normal: &Vector) -> Self {
        // Clone and unitize the normal vector
        let mut zaxis = normal.clone();
        if !zaxis.unitize() {
            panic!("Normal vector cannot be zero length");
        }

        // Create two perpendicular vectors to form a coordinate system
        // Choose an initial vector that's not parallel to the normal
        let initial = if zaxis.x.abs() < 0.9 {
            Vector::new(1.0, 0.0, 0.0)  // Use x-axis if normal is not too close to x-axis
        } else {
            Vector::new(0.0, 1.0, 0.0)  // Use y-axis if normal is close to x-axis
        };

        // Get first perpendicular vector (x-axis of the plane)
        let mut xaxis = initial.cross(&zaxis);
        xaxis.unitize();

        // Get second perpendicular vector (y-axis of the plane)
        let mut yaxis = zaxis.cross(&xaxis);
        yaxis.unitize();

        // Calculate plane equation coefficients (Ax + By + Cz + D = 0)
        let a = zaxis.x;
        let b = zaxis.y;
        let c = zaxis.z;
        let d = -(a * point.x + b * point.y + c * point.z);

        Plane {
            origin: point.clone(),
            xaxis,
            yaxis,
            zaxis,
            a,
            b,
            c,
            d,
            guid: Uuid::new_v4(),
            name: "Plane".to_string(),
            linecolor: [0.0, 0.0, 0.0, 1.0],
            width: 1.0,
        }
    }

    /// Creates a new `Plane` from a collection of points.
    ///
    /// For 2 points: Creates a plane where the line between points is the x-axis.
    /// For 3+ points: Computes an average cross product from consecutive point triplets.
    ///
    /// # Arguments
    ///
    /// * `points` - A slice of points (minimum 2 points required).
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Plane, Point};
    /// let points = vec![
    ///     Point::new(0.0, 0.0, 0.0),
    ///     Point::new(1.0, 0.0, 0.0),
    ///     Point::new(0.0, 1.0, 0.0)
    /// ];
    /// let plane = Plane::plane_from_points(&points);
    /// assert_eq!(plane.origin.x, 0.0);
    /// assert_eq!(plane.origin.y, 0.0);
    /// assert_eq!(plane.origin.z, 0.0);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if fewer than 2 points are provided.
    pub fn plane_from_points(points: &[Point]) -> Self {
        
        if points.len() == 0{
            return Plane::default();
        }else if points.len() == 1 {
            return Plane::new(points[0].clone(), Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        }else if points.len() >= 3 {
            
            // For three or more points, compute an average cross product for the plane normal
            let mut normal_sum = Vector::new(0.0, 0.0, 0.0);
            let mut normal_count = 0;
            
            // Calculate cross products from consecutive point triplets
            for i in 0..(points.len() - 2) {
                let v1 = Vector::new(
                    points[i + 1].x - points[i].x,
                    points[i + 1].y - points[i].y,
                    points[i + 1].z - points[i].z,
                );
                let v2 = Vector::new(
                    points[i + 2].x - points[i + 1].x,
                    points[i + 2].y - points[i + 1].y,
                    points[i + 2].z - points[i + 1].z,
                );
                
                let cross = v1.cross(&v2);
                // Only add non-zero cross products (skip colinear segments)
                if cross.length() > 1e-10 {
                    normal_sum.x += cross.x;
                    normal_sum.y += cross.y;
                    normal_sum.z += cross.z;
                    normal_count += 1;
                }
            }
            
            if normal_count > 0 {
                // Average the normals
                normal_sum.x /= normal_count as f32;
                normal_sum.y /= normal_count as f32;
                normal_sum.z /= normal_count as f32;
                
                // Create plane from first point and averaged normal
                Self::from_point_normal(&points[0], &normal_sum)
            } else {
                // All segments are colinear, fall back to 2-point logic
                Self::plane_from_two_points(&points[0], &points[1])
            }
        } else {
            // For two points, guess the y-axis since the first line is x-axis
            Self::plane_from_two_points(&points[0], &points[1])
        }
    }
    
    /// Helper function to create a plane from two points
    fn plane_from_two_points(p1: &Point, p2: &Point) -> Self {
        // The line from p1 to p2 becomes the x-axis
        let x_axis = Vector::new(
            p2.x - p1.x,
            p2.y - p1.y,
            p2.z - p1.z,
        );
        
        // Guess a reasonable y-axis by choosing a vector not parallel to x-axis
        let y_guess = if x_axis.z.abs() < 0.9 {
            Vector::new(0.0, 0.0, 1.0)  // Use world Z if x-axis is not too close to Z
        } else {
            Vector::new(0.0, 1.0, 0.0)  // Use world Y if x-axis is close to Z
        };
        
        // Get perpendicular y-axis via cross product
        let z_axis = x_axis.cross(&y_guess);
        let y_axis = z_axis.cross(&x_axis);
        
        // Compute plane normal (z-axis)
        let normal = x_axis.cross(&y_axis);
        
        // Create plane from first point and computed normal
        Self::from_point_normal(p1, &normal)

    }

    


}

impl Default for Plane {
    /// Creates a zero length `Plane`.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Plane;
    /// let plane = Plane::default();
    /// assert_eq!(plane.origin.x, 0.0);
    /// assert_eq!(plane.origin.y, 0.0);
    /// assert_eq!(plane.origin.z, 0.0);
    /// assert_eq!(plane.xaxis.x, 1.0);
    /// assert_eq!(plane.xaxis.y, 0.0);
    /// assert_eq!(plane.xaxis.z, 0.0);
    /// assert_eq!(plane.yaxis.x, 0.0);
    /// assert_eq!(plane.yaxis.y, 1.0);
    /// assert_eq!(plane.yaxis.z, 0.0);
    /// assert_eq!(plane.zaxis.x, 0.0);
    /// assert_eq!(plane.zaxis.y, 0.0);
    /// assert_eq!(plane.zaxis.z, 1.0);
    /// assert_eq!(plane.a, 0.0);
    /// assert_eq!(plane.b, 0.0);
    /// assert_eq!(plane.c, 1.0);
    /// assert_eq!(plane.d, 0.0);
    /// ```
    fn default() -> Self {
        Plane {
            origin: Point::new(0.0, 0.0, 0.0),
            xaxis: Vector::new(1.0, 0.0, 0.0),
            yaxis: Vector::new(0.0, 1.0, 0.0),
            zaxis: Vector::new(0.0, 0.0, 1.0),
            a: 0.0,
            b: 0.0,
            c: 1.0,
            d: 0.0,
            guid: Uuid::new_v4(),
            name: "Plane".to_string(),
            linecolor: [0.0, 0.0, 0.0, 1.0],
            width: 1.0,
        }
    }
}




impl Add<&Vector> for Plane {
    type Output = Plane;

    /// Adds the coordinates of a vector to this plane and returns a new plane.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Plane, Vector};
    /// let plane0 = Plane::default();
    /// let v = Vector::new(0.0, 0.0, 1.0);
    /// let plane1 = plane0 + &v;
    /// assert_eq!(plane1.origin.x, 0.0);
    /// assert_eq!(plane1.origin.y, 0.0);
    /// assert_eq!(plane1.origin.z, 1.0);
    /// ```
    fn add(self, other: &Vector) -> Plane {
        Plane {
            origin: self.origin + other,
            xaxis: self.xaxis,
            yaxis: self.yaxis,
            zaxis: self.zaxis,
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            guid: Uuid::new_v4(),
            name: "Plane".to_string(),
            linecolor: [0.0, 0.0, 0.0, 1.0],
            width: 1.0,
        }
    }
}


impl AddAssign<&Vector> for Plane {
    /// Adds the coordinates of a vector to this plane.
    ///
    /// # Arguments
    ///
    /// * `vector` - traslation vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Plane;
    /// use session_rust::primitives::Vector;
    /// let mut plane = Plane::default();
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// plane += &v;
    /// assert_eq!(plane.origin.x, 1.0);
    /// assert_eq!(plane.origin.y, 1.0);
    /// assert_eq!(plane.origin.z, 1.0);
    /// ```
    fn add_assign(&mut self, vector: &Vector) {
        self.origin += vector;
    }
}


impl Sub<&Vector> for Plane {
    type Output = Plane;

    /// Subtracts the coordinates of a vector to this plane and returns a new plane.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Plane, Vector};
    /// let plane0 = Plane::default();
    /// let v = Vector::new(0.0, 0.0, 1.0);
    /// let plane1 = plane0 - &v;
    /// assert_eq!(plane1.origin.x, 0.0);
    /// assert_eq!(plane1.origin.y, 0.0);
    /// assert_eq!(plane1.origin.z, -1.0);
    /// ```
    fn sub(self, vector: &Vector) -> Plane {
        Plane {
            origin: self.origin - vector,
            xaxis: self.xaxis,
            yaxis: self.yaxis,
            zaxis: self.zaxis,
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            guid: Uuid::new_v4(),
            name: "Plane".to_string(),
            linecolor: [0.0, 0.0, 0.0, 1.0],
            width: 1.0,
        }
    }
}


impl SubAssign<&Vector> for Plane {
    /// Subtracts the coordinates of a vector to this plane.
    ///
    /// # Arguments
    ///
    /// * `vector` - traslation vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Plane;
    /// use session_rust::primitives::Vector;
    /// let mut plane = Plane::default();
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// plane -= &v;
    /// assert_eq!(plane.origin.x, -1.0);
    /// assert_eq!(plane.origin.y, -1.0);
    /// assert_eq!(plane.origin.z, -1.0);
    /// ```
    fn sub_assign(&mut self, vector: &Vector) {
        self.origin -= vector;
    }
}


impl fmt::Display for Plane{
    /// Log color.
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Plane;
    /// let plane = Plane::default();
    /// println!("{}", plane);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Plane {{ origin: {}, xaxis: {}, yaxis: {}, zaxis: {}, name: {} }}", self.origin, self.xaxis, self.yaxis, self.zaxis, self.name)
    }
}
impl Plane {
    /// Get the type identifier for polymorphic deserialization
    pub fn dtype(&self) -> &'static str {
        "Plane"
    }
    
    /// Get the object's geometric data for serialization
    pub fn geometric_data(&self) -> serde_json::Value {
        serde_json::json!({
            "origin": {
                "x": self.origin.x,
                "y": self.origin.y,
                "z": self.origin.z
            },
            "xaxis": {
                "x": self.xaxis.x,
                "y": self.xaxis.y,
                "z": self.xaxis.z
            },
            "yaxis": {
                "x": self.yaxis.x,
                "y": self.yaxis.y,
                "z": self.yaxis.z
            },
            "zaxis": {
                "x": self.zaxis.x,
                "y": self.zaxis.y,
                "z": self.zaxis.z
            },
            "a": self.a,
            "b": self.b,
            "c": self.c,
            "d": self.d
        })
    }
    
    /// Get the object's GUID
    pub fn guid(&self) -> Uuid {
        self.guid
    }
    
    /// Get the object's name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Set the object's name
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn to_json_data(&self, minimal: bool) -> serde_json::Value {
        let mut json_obj = self.geometric_data();
        json_obj["dtype"] = serde_json::Value::String(self.dtype().to_string());
        
        if !minimal {
            json_obj["guid"] = serde_json::Value::String(self.guid.to_string());
            json_obj["name"] = serde_json::Value::String(self.name.clone());
        }
        
        json_obj
    }
}

impl HasJsonData for Plane {
    fn to_json_data(&self, minimal: bool) -> Value {
        let mut json_obj = self.geometric_data();
        json_obj["dtype"] = Value::String(self.dtype().to_string());
        
        if !minimal {
            json_obj["guid"] = Value::String(self.guid.to_string());
            json_obj["name"] = Value::String(self.name.clone());
        }
        
        json_obj
    }
}

impl FromJsonData for Plane {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let (
            Some(origin_obj), Some(xaxis_obj), Some(yaxis_obj), Some(zaxis_obj),
            Some(a), Some(b), Some(c), Some(d),
            Some(guid_str), Some(name)
        ) = (
            data.get("origin").and_then(|v| v.as_object()),
            data.get("xaxis").and_then(|v| v.as_object()),
            data.get("yaxis").and_then(|v| v.as_object()),
            data.get("zaxis").and_then(|v| v.as_object()),
            data.get("a").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("b").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("c").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("d").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("guid").and_then(|v| v.as_str()),
            data.get("name").and_then(|v| v.as_str()),
        ) {
            let origin = Point::new(
                origin_obj.get("x")?.as_f64()? as f32,
                origin_obj.get("y")?.as_f64()? as f32,
                origin_obj.get("z")?.as_f64()? as f32,
            );
            let xaxis = Vector::new(
                xaxis_obj.get("x")?.as_f64()? as f32,
                xaxis_obj.get("y")?.as_f64()? as f32,
                xaxis_obj.get("z")?.as_f64()? as f32,
            );
            let yaxis = Vector::new(
                yaxis_obj.get("x")?.as_f64()? as f32,
                yaxis_obj.get("y")?.as_f64()? as f32,
                yaxis_obj.get("z")?.as_f64()? as f32,
            );
            let zaxis = Vector::new(
                zaxis_obj.get("x")?.as_f64()? as f32,
                zaxis_obj.get("y")?.as_f64()? as f32,
                zaxis_obj.get("z")?.as_f64()? as f32,
            );
            let guid = uuid::Uuid::parse_str(guid_str).ok()?;
            Some(Plane {
                origin, xaxis, yaxis, zaxis,
                a, b, c, d,
                guid,
                name: name.to_string(),
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
