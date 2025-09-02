use crate::{Point, Mesh, Vector, Xform};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::fmt;
use uuid::Uuid;
use crate::{HasJsonData, FromJsonData};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Line {
    /// The x coordinate of the start point.
    pub x0: f32,
    /// The y coordinate of the start point.
    pub y0: f32,
    /// The z coordinate of the start point.
    pub z0: f32,
    /// The x coordinate of the end point.
    pub x1: f32,
    /// The y coordinate of the end point.
    pub y1: f32,
    /// The z coordinate of the end point.
    pub z1: f32,
    /// Unique identifier
    pub guid: Uuid,
    /// Object name
    pub name: String,
    /// Line color as RGBA components [r, g, b, a]
    pub linecolor: [f32; 4],
    /// Width value
    pub width: f32,
    /// Mesh for visualization (pipe)
    #[serde(skip)]
    pub mesh: Option<Mesh>,
}

// Custom serialization to skip xform field
impl Serialize for Line {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Line", 10)?;
        state.serialize_field("x0", &self.x0)?;
        state.serialize_field("y0", &self.y0)?;
        state.serialize_field("z0", &self.z0)?;
        state.serialize_field("x1", &self.x1)?;
        state.serialize_field("y1", &self.y1)?;
        state.serialize_field("z1", &self.z1)?;
        state.serialize_field("guid", &self.guid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("linecolor", &self.linecolor)?;
        state.serialize_field("width", &self.width)?;
        state.end()
    }
}

impl Line{
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
    pub fn new(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
        Self {
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
            guid: Uuid::new_v4(),
            name: "Line".to_string(),
            linecolor: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
            mesh: None,
        }
    }

    /// Creates a new `Line` with a specified name for `Data`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for the `Data`.
    /// * `x0` - The x component of the start point.
    /// * `y0` - The y component of the start point.
    /// * `z0` - The z component of the start point.
    /// * `x1` - The x component of the end point.
    /// * `y1` - The y component of the end point.
    /// * `z1` - The z component of the end point.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let line = Line::with_name("MyLine".to_string(), 0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(line.x0, 0.0);
    /// assert_eq!(line.y0, 0.0);
    /// assert_eq!(line.z0, 0.0);
    /// assert_eq!(line.x1, 0.0);
    /// assert_eq!(line.y1, 0.0);
    /// assert_eq!(line.z1, 1.0);
    /// ```
    pub fn with_name(name: String, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> Self {
        Line {
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
            guid: Uuid::new_v4(),
            name: name.to_string(),
            linecolor: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
            mesh: None,
        }
    }

    /// Creates a new `Line` from start ´Point´ and end `Point`.
    ///
    /// # Arguments
    ///
    /// * `p0` - The start point.
    /// * `p1` - The end point.
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Point;
    /// use session_rust::primitives::Line;
    /// let p0 = Point::new(0.0, 0.0, 0.0);
    /// let p1 = Point::new(0.0, 0.0, 1.0);
    /// let line = Line::from_points(&p0, &p1);
    /// assert_eq!(line.x0, 0.0);
    /// assert_eq!(line.y0, 0.0);
    /// assert_eq!(line.z0, 0.0);
    /// assert_eq!(line.x1, 0.0);
    /// assert_eq!(line.y1, 0.0);
    /// assert_eq!(line.z1, 1.0);
    /// ```
    pub fn from_points(p0: &Point, p1: &Point) -> Self{
        Line {
            x0:p0.x,
            y0:p0.y,
            z0:p0.z,
            x1:p1.x,
            y1:p1.y,
            z1:p1.z,
            guid: Uuid::new_v4(),
            name: "Line".to_string(),
            linecolor: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
            mesh: None,
        }
    }

    /// Computes the length of the line.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let length = line.length();
    /// assert_eq!(length, 1.0);
    /// ```
    pub fn length(&self) -> f32 {
        ((self.x0 - self.x1).powi(2) + (self.y0 - self.y1).powi(2) + (self.z0 - self.z1).powi(2))
            .sqrt()
    }

    /// Updates the mesh representation using thickness from data.
    /// 
    /// # Returns
    /// A reference to self for method chaining.
    pub fn update_mesh(&mut self) -> &mut Self {
        // Use width as thickness
        let thickness = self.width;
        
        // Create start and end points for the pipe
        let start = Point::new(self.x0, self.y0, self.z0);
        let end = Point::new(self.x1, self.y1, self.z1);
        
        // Use fixed 8 sides for the pipe cross-section
        // Generate the mesh
        self.mesh = Some(Mesh::create_pipe(start, end, thickness));
        
        // Apply colors to mesh if available
        if let Some(ref mut mesh) = self.mesh {
            mesh.facecolors = [1.0, 1.0, 1.0, 1.0];
            mesh.linecolors = self.linecolor;
            mesh.pointcolors = [0.0, 0.0, 0.0, 1.0];
        }
        
        self
    }

    /// Gets the mesh representation of this line as a pipe.
    /// If the mesh doesn't exist, creates it first.
    /// 
    /// # Returns
    /// An Option containing a reference to the Mesh if it exists.
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::primitives::Line;
    /// let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let mesh = line.get_mesh();
    /// assert!(mesh.is_some());
    /// ```
    pub fn get_mesh(&mut self) -> Option<&Mesh> {
        // Create the mesh if it doesn't exist yet
        if self.mesh.is_none() {
            self.update_mesh();
        }
        
        self.mesh.as_ref()
    }

    /// Returns a transform that maps the canonical unit pipe (aligned to +Z, length=1, radius=0.5,
    /// centered at the origin with z in [-0.5, +0.5]) onto this line segment.
    /// Calculates transformation from coordinates since primitives don't store xform matrices.
    pub fn to_pipe_transform(&self) -> Option<Xform> {
        // Calculate transformation from coordinates
        let p0 = Point::new(self.x0, self.y0, self.z0);
        let p1 = Point::new(self.x1, self.y1, self.z1);

        // Direction and length
        let dir = Vector::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
        let len = dir.length();
        let eps = 1e-9;
        if len < eps { return None; }

        let axis = dir.normalize();
        let z_axis = Vector::new(0.0, 0.0, 1.0);

        // Rotation aligning +Z to the line direction
        let mut dot = axis.dot(&z_axis);
        if dot > 1.0 { dot = 1.0; } else if dot < -1.0 { dot = -1.0; }
        let rotation = if (dot - 1.0).abs() < eps {
            Xform::identity()
        } else if (dot + 1.0).abs() < eps {
            // +Z to -Z: 180° around any axis perpendicular to Z (choose X)
            Xform::rotation_x(std::f32::consts::PI)
        } else {
            let rot_axis = z_axis.cross(&axis).normalize();
            let angle = dot.acos();
            Xform::rotation(&rot_axis, angle)
        };

        // Midpoint translation
        let midpoint = Point::new(
            (p0.x + p1.x) * 0.5,
            (p0.y + p1.y) * 0.5,
            (p0.z + p1.z) * 0.5,
        );
        let translation = Xform::translation(midpoint.x, midpoint.y, midpoint.z);

        // Non-uniform scale: XY = 1.0 (unit pipe geometry radius is unused for thickness), Z = length
        let scale = Xform::scaling(1.0, 1.0, len);

        // Compose T * R * S (scale → rotate → translate)
        Some(translation * rotation * scale)
    }

    /// Convert the line to a pipe mesh with specified radius and sides
    pub fn to_pipe_mesh(&self, radius: f32, sides: Option<usize>) -> Option<crate::Mesh> {
        use crate::{Mesh, Point, Vector};
        
        let sides = sides.unwrap_or(8);
        if sides < 3 { return None; }
        
        // Create a simple cylindrical pipe mesh
        let start = Point::new(self.x0, self.y0, self.z0);
        let end = Point::new(self.x1, self.y1, self.z1);
        
        // Calculate direction vector
        let dir = Vector::new(end.x - start.x, end.y - start.y, end.z - start.z);
        let length = dir.length();
        if length < 1e-10 { return None; }
        
        let dir_norm = dir.normalize();
        
        // Find perpendicular vectors for the circular cross-section
        let up = if dir_norm.z.abs() < 0.9 {
            Vector::new(0.0, 0.0, 1.0)
        } else {
            Vector::new(1.0, 0.0, 0.0)
        };
        
        let right = dir_norm.cross(&up).normalize();
        let forward = right.cross(&dir_norm).normalize();
        
        let mut mesh = Mesh::new();
        let mut vertices = Vec::new();
        
        // Create vertices for start and end circles
        for ring in 0..2 {
            let center = if ring == 0 { &start } else { &end };
            for i in 0..sides {
                let angle = 2.0 * std::f32::consts::PI * (i as f32) / (sides as f32);
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                
                let offset = right * (radius * cos_a) + forward * (radius * sin_a);
                let vertex_pos = Point::new(
                    center.x + offset.x,
                    center.y + offset.y,
                    center.z + offset.z,
                );
                
                let vkey = mesh.add_vertex(vertex_pos, None);
                vertices.push(vkey);
            }
        }
        
        // Create side faces (quads, will be triangulated)
        for i in 0..sides {
            let next_i = (i + 1) % sides;
            let v0 = vertices[i];           // start ring, current
            let v1 = vertices[next_i];      // start ring, next
            let v2 = vertices[sides + next_i]; // end ring, next
            let v3 = vertices[sides + i];   // end ring, current
            
            // Create quad as two triangles
            mesh.add_face(vec![v0, v1, v2], None);
            mesh.add_face(vec![v0, v2, v3], None);
        }
        
        // Create end caps
        // Start cap (center vertex + triangles)
        let start_center = mesh.add_vertex(start, None);
        for i in 0..sides {
            let next_i = (i + 1) % sides;
            let v0 = vertices[i];
            let v1 = vertices[next_i];
            mesh.add_face(vec![start_center, v1, v0], None); // Reverse winding for inward-facing cap
        }
        
        // End cap (center vertex + triangles)
        let end_center = mesh.add_vertex(end, None);
        for i in 0..sides {
            let next_i = (i + 1) % sides;
            let v0 = vertices[sides + i];
            let v1 = vertices[sides + next_i];
            mesh.add_face(vec![end_center, v0, v1], None); // Forward winding for outward-facing cap
        }
        
        Some(mesh)
    }
}

impl Default for Line{
    /// Creates a default `Line` as a vertical line.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let l = Line::default();
    /// ```
    fn default() -> Self {
        Line {
            x0: 0.0,
            y0: 0.0,
            z0: 0.0,
            x1: 0.0,
            y1: 0.0,
            z1: 1.0,
            guid: Uuid::new_v4(),
            name: "Line".to_string(),
            linecolor: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
            mesh: None,
        }
    }
}

impl Add<&Vector> for Line {
    type Output = Line;

    /// Adds the coordinates of a vector to this line and returns a new line.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Line, Vector};
    /// let line0 = Line::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
    /// let v = Vector::new(0.0, 0.0, 1.0);
    /// let line1 = line0 + &v;
    /// assert_eq!(line1.x0, 0.0);
    /// assert_eq!(line1.y0, 1.0);
    /// assert_eq!(line1.z0, 3.0);
    /// assert_eq!(line1.x1, 3.0);
    /// assert_eq!(line1.y1, 4.0);
    /// assert_eq!(line1.z1, 6.0);
    /// ```
    fn add(self, other: &Vector) -> Line {
        Line {
            x0: self.x0 + other.x,
            y0: self.y0 + other.y,
            z0: self.z0 + other.z,
            x1: self.x1 + other.x,
            y1: self.y1 + other.y,
            z1: self.z1 + other.z,
            guid: Uuid::new_v4(),
            name: "Line".to_string(),
            linecolor: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
            mesh: None,
        }
    }
}

impl AddAssign<&Vector> for Line {
    /// Adds the coordinates of a vector to this line.
    ///
    /// # Arguments
    ///
    /// * `vector` - traslation vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// use session_rust::primitives::Vector;
    /// let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// line += &v;
    /// assert_eq!(line.x0, 1.0);
    /// assert_eq!(line.y0, 1.0);
    /// assert_eq!(line.z0, 1.0);
    /// assert_eq!(line.x1, 1.0);
    /// assert_eq!(line.y1, 1.0);
    /// assert_eq!(line.z1, 2.0);
    /// ```
    fn add_assign(&mut self, vector: &Vector) {
        self.x0 += vector.x;
        self.y0 += vector.y;
        self.z0 += vector.z;
        self.x1 += vector.x;
        self.y1 += vector.y;
        self.z1 += vector.z;
    }
}

impl Div<f32> for Line {
    type Output = Line;

    /// Divides the coordinates of the line by a scalar and returns a new line.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to divide by.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let line0 = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let line1 = line0 / 2.0;
    /// assert_eq!(line1.x0, 0.0);
    /// assert_eq!(line1.y0, 0.0);
    /// assert_eq!(line1.z0, 0.0);
    /// assert_eq!(line1.x1, 0.0);
    /// assert_eq!(line1.y1, 0.0);
    /// assert_eq!(line1.z1, 0.5);
    /// ```
    fn div(self, factor: f32) -> Line {
        let mut result = self;
        result /= factor;
        result
    }
}

impl DivAssign<f32> for Line {
    /// Divides the coordinates of the Line by a scalar.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to divide by.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// line /= 2.0;
    /// assert_eq!(line.x0, 0.0);
    /// assert_eq!(line.y0, 0.0);
    /// assert_eq!(line.z0, 0.0);
    /// assert_eq!(line.x1, 0.0);
    /// assert_eq!(line.y1, 0.0);
    /// assert_eq!(line.z1, 0.5);
    /// ```
    fn div_assign(&mut self, factor: f32) {
        self.x0 /= factor;
        self.y0 /= factor;
        self.z0 /= factor;
        self.x1 /= factor;
        self.y1 /= factor;
        self.z1 /= factor;
    }
}

impl Index<usize> for Line {
    type Output = f32;

    /// Provides read-only access to the coordinates of the point using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for x0, 1 for y0, 2 for z0, 3 for x1, 4 for y1, 5 for z1).
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let line = Line::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
    /// assert_eq!(line[0], 0.0);
    /// assert_eq!(line[1], 1.0);
    /// assert_eq!(line[2], 2.0);
    /// assert_eq!(line[3], 3.0);
    /// assert_eq!(line[4], 4.0);
    /// assert_eq!(line[5], 5.0);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x0,
            1 => &self.y0,
            2 => &self.z0,
            3 => &self.x1,
            4 => &self.y1,
            5 => &self.z1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Line {
    /// Provides mutable access to the coordinates of the line using the `[]` operator.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the coordinate (0 for x0, 1 for y0, 2 for z0, 3 for x1, 4 for y1, 5 for z1).
    ///
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// line[0] = 1.0;
    /// line[1] = 2.0;
    /// line[2] = 3.0;
    /// line[3] = 4.0;
    /// line[4] = 5.0;
    /// line[5] = 6.0;
    /// assert_eq!(line[0], 1.0);
    /// assert_eq!(line[1], 2.0);
    /// assert_eq!(line[2], 3.0);
    /// assert_eq!(line[3], 4.0);
    /// assert_eq!(line[4], 5.0);
    /// assert_eq!(line[5], 6.0);
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x0,
            1 => &mut self.y0,
            2 => &mut self.z0,
            3 => &mut self.x1,
            4 => &mut self.y1,
            5 => &mut self.z1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl MulAssign<f32> for Line {
    /// Multiplies the coordinates of the line by a scalar.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to multiply by.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let mut line = Line::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
    /// line *= 2.0;
    /// assert_eq!(line.x0, 0.0);
    /// assert_eq!(line.y0, 2.0);
    /// assert_eq!(line.z0, 4.0);
    /// assert_eq!(line.x1, 6.0);
    /// assert_eq!(line.y1, 8.0);
    /// assert_eq!(line.z1, 10.0);
    /// ```
    fn mul_assign(&mut self, factor: f32) {
        self.x0 *= factor;
        self.y0 *= factor;
        self.z0 *= factor;
        self.x1 *= factor;
        self.y1 *= factor;
        self.z1 *= factor;
    }
}

impl Mul<f32> for Line {
    type Output = Line;

    /// Multiplies the coordinates of line point by a scalar and returns a new line.
    ///
    /// # Arguments
    ///
    /// * `factor` - The scalar to multiply by.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let line0 = Line::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
    /// let line1 = line0 * 2.0;
    /// assert_eq!(line1.x0, 0.0);
    /// assert_eq!(line1.y0, 2.0);
    /// assert_eq!(line1.z0, 4.0);
    /// assert_eq!(line1.x1, 6.0);
    /// assert_eq!(line1.y1, 8.0);
    /// assert_eq!(line1.z1, 10.0);
    /// ```
    fn mul(self, factor: f32) -> Line {
        let mut result = self;
        result *= factor;
        result
    }
}

impl Sub<&Vector> for Line {
    type Output = Line;

    /// Subtracts the coordinates of a vector from this Line and returns a new vector.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to subtract coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Line, Vector};
    /// let line0 = Line::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0);
    /// let v = Vector::new(0.0, 0.0, 1.0);
    /// let line1 = line0 - &v;
    /// assert_eq!(line1.x0, 0.0);
    /// assert_eq!(line1.y0, 1.0);
    /// assert_eq!(line1.z0, 1.0);
    /// assert_eq!(line1.x1, 3.0);
    /// assert_eq!(line1.y1, 4.0);
    /// assert_eq!(line1.z1, 4.0);
    /// ```
    fn sub(self, vector: &Vector) -> Line {
        Line {
            x0: self.x0 - vector.x,
            y0: self.y0 - vector.y,
            z0: self.z0 - vector.z,
            x1: self.x1 - vector.x,
            y1: self.y1 - vector.y,
            z1: self.z1 - vector.z,
            guid: Uuid::new_v4(),
            name: "Line".to_string(),
            linecolor: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
            mesh: None,
        }
    }
}

impl SubAssign<&Vector> for Line {
    /// Subtracts the coordinates of a line using a vector.
    ///
    /// # Arguments
    ///
    /// * `vector` - The subtraction vector.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// use session_rust::primitives::Vector;
    /// let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let v = Vector::new(1.0, 2.0, 3.0);
    /// line -= &v;
    /// assert_eq!(line.x0, -1.0);
    /// assert_eq!(line.y0, -2.0);
    /// assert_eq!(line.z0, -3.0);
    /// assert_eq!(line.x1, -1.0);
    /// assert_eq!(line.y1, -2.0);
    /// assert_eq!(line.z1, -2.0);
    /// ```
    fn sub_assign(&mut self, vector: &Vector) {
        self.x0 -= vector.x;
        self.y0 -= vector.y;
        self.z0 -= vector.z;
        self.x1 -= vector.x;
        self.y1 -= vector.y;
        self.z1 -= vector.z;
    }
}

impl From<Line> for Vector {
    /// Converts a `Line` into a `Vector`.
    ///
    /// # Arguments
    ///
    /// * `line` - The `Line` to be converted.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::{Line, Vector};
    /// let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// let v: Vector = line.into();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// assert_eq!(v.z, 1.0);
    /// ```
    fn from(line: Line) -> Self {
        Vector::new(
            line.x1 - line.x0,
            line.y1 - line.y0,
            line.z1 - line.z0
        )
    }
}

impl HasJsonData for Line {
    fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "Line",
            "x0": self.x0,
            "y0": self.y0,
            "z0": self.z0,
            "x1": self.x1,
            "y1": self.y1,
            "z1": self.z1,
            "guid": self.guid.to_string(),
            "name": self.name,
            "linecolor": self.linecolor,
            "width": self.width
        })
    }
}

impl FromJsonData for Line {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let (
            Some(x0), Some(y0), Some(z0),
            Some(x1), Some(y1), Some(z1),
            Some(guid_str), Some(name),
            Some(linecolor_array), Some(width)
        ) = (
            data.get("x0").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("y0").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("z0").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("x1").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("y1").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("z1").and_then(|v| v.as_f64()).map(|v| v as f32),
            data.get("guid").and_then(|v| v.as_str()),
            data.get("name").and_then(|v| v.as_str()),
            data.get("linecolor").and_then(|v| v.as_array()),
            data.get("width").and_then(|v| v.as_f64()).map(|v| v as f32),
        ) {
            let guid = uuid::Uuid::parse_str(guid_str).ok()?;
            let linecolor = [
                linecolor_array.get(0)?.as_f64()? as f32,
                linecolor_array.get(1)?.as_f64()? as f32,
                linecolor_array.get(2)?.as_f64()? as f32,
                linecolor_array.get(3)?.as_f64()? as f32,
            ];
            Some(Line {
                x0, y0, z0, x1, y1, z1,
                guid,
                name: name.to_string(),
                linecolor,
                width,
                mesh: None,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for Line{
    /// Log line.
    /// # Example
    ///
    /// ```
    /// use session_rust::primitives::Line;
    /// let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    /// println!("{}", line);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "Line({}, {}, {}, {}, {}, {})", self.x0, self.y0, self.z0, self.x1, self.y1, self.z1)
    }
}
