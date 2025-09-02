use crate::{Point, Line, Mesh, Xform, Vector};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::fmt;
use uuid::Uuid;
use crate::{HasJsonData, FromJsonData};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineCloud {
    /// The collection of lines.
    pub lines: Vec<Line>,
    /// The transformation matrix.
    pub xform: Xform,
    /// Unique identifier
    pub guid: Uuid,
    /// Object name
    pub name: String,
    /// Width values for each line
    pub widths: Vec<f32>,
    
    /// Collection of meshes for visualization (pipes)
    #[serde(skip)]
    pub meshes: Vec<Mesh>,
    
    /// Flag indicating if meshes need to be rebuilt
    #[serde(skip)]
    dirty: bool,
}

impl LineCloud {
    /// Creates a new `LineCloud` with default `Data`.
    ///
    /// # Arguments
    ///
    /// * `lines` - The collection of lines.
    /// * `colors` - The collection of colors.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::LineCloud;
    /// use session_rust::primitives::Line;
    /// let lines = vec![Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)];
    /// let line_cloud = LineCloud::new(lines);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the number of lines and colors are not equal.
    pub fn new(lines: Vec<Line>) -> Self {
        let widths = vec![1.0; lines.len()]; // Default width of 1.0 for all lines
        Self {
            lines,
            xform: Xform::default(),
            guid: Uuid::new_v4(),
            name: "LineCloud".to_string(),
            widths,
            meshes: Vec::new(),
            dirty: true,
        }
    }
}

impl AddAssign<&Vector> for LineCloud {
    /// Adds a vector to all line points.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to add.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::LineCloud;
    /// use session_rust::primitives::{Line, Vector};
    /// let mut lc = LineCloud::new(
    ///     vec![Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)]
    /// );
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// lc += &v;
    /// assert_eq!(lc.lines[0].x0, 2.0);
    /// assert_eq!(lc.lines[0].x1, 5.0);
    /// ```
    fn add_assign(&mut self, other: &Vector) {
        for line in &mut self.lines {
            line.x0 += other.x;
            line.y0 += other.y;
            line.z0 += other.z;
            line.x1 += other.x;
            line.y1 += other.y;
            line.z1 += other.z;
        }
        self.dirty = true;
    }
}

impl Add<&Vector> for LineCloud {
    type Output = LineCloud;

    /// Adds a vector to all line points and returns a new LineCloud.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to add.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::LineCloud;
    /// use session_rust::primitives::{Line, Vector};
    /// let lc = LineCloud::new(
    ///     vec![Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)]
    /// );
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// let lc2 = lc + &v;
    /// assert_eq!(lc2.lines[0].x0, 2.0);
    /// assert_eq!(lc2.lines[0].x1, 5.0);
    /// ```
    fn add(self, other: &Vector) -> LineCloud {
        let mut lc = self.clone();
        for line in &mut lc.lines {
            line.x0 += other.x;
            line.y0 += other.y;
            line.z0 += other.z;
            line.x1 += other.x;
            line.y1 += other.y;
            line.z1 += other.z;
        }
        lc
    }
}

impl SubAssign<&Vector> for LineCloud {
    /// Subtracts a vector from all line points.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to subtract.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::LineCloud;
    /// use session_rust::primitives::{Line, Vector};
    /// let mut lc = LineCloud::new(
    ///     vec![Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)]
    /// );
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// lc -= &v;
    /// assert_eq!(lc.lines[0].x0, 0.0);
    /// assert_eq!(lc.lines[0].x1, 3.0);
    /// ```
    fn sub_assign(&mut self, other: &Vector) {
        for line in &mut self.lines {
            line.x0 -= other.x;
            line.y0 -= other.y;
            line.z0 -= other.z;
            line.x1 -= other.x;
            line.y1 -= other.y;
            line.z1 -= other.z;
        }
        self.dirty = true;
    }
}

impl Sub<&Vector> for LineCloud {
    type Output = LineCloud;

    /// Subtracts a vector from all line points and returns a new LineCloud.
    ///
    /// # Arguments
    ///
    /// * `other` - The vector to subtract.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::LineCloud;
    /// use session_rust::primitives::{Line, Vector};
    /// let lc = LineCloud::new(
    ///     vec![Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)]
    /// );
    /// let v = Vector::new(1.0, 1.0, 1.0);
    /// let lc2 = lc - &v;
    /// assert_eq!(lc2.lines[0].x0, 0.0);
    /// assert_eq!(lc2.lines[0].x1, 3.0);
    /// ```
    fn sub(self, other: &Vector) -> LineCloud {
        let mut lc = self.clone();
        for line in &mut lc.lines {
            line.x0 -= other.x;
            line.y0 -= other.y;
            line.z0 -= other.z;
            line.x1 -= other.x;
            line.y1 -= other.y;
            line.z1 -= other.z;
        }
        lc
    }
}

impl fmt::Display for LineCloud {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LineCloud {{ lines: {}, widths: {}, name: {} }}",
            self.lines.len(),
            self.widths.len(),
            self.name
        )
    }
}


// LineCloud visualization methods
impl LineCloud {
    /// Updates the meshes for all lines using thickness from data.
    /// Called internally when needed or when explicitly requested.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::LineCloud;
    /// use session_rust::primitives::Line;
    /// let mut lc = LineCloud::new(
    ///     vec![Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)]
    /// );
    /// lc.update_meshes();
    /// assert_eq!(lc.meshes.len(), 1);
    /// ```
    pub fn update_meshes(&mut self) -> &mut Self {
        if !self.dirty {
            return self;
        }
        
        // Use default thickness of 1.0 if no specific widths available
        let default_thickness = 1.0;
        // Use fixed 8 sides for all pipes

        
        self.meshes.clear();
        
        // Create a mesh for each line
        for (i, line) in self.lines.iter().enumerate() {
            let start = Point::new(line.x0, line.y0, line.z0);
            let end = Point::new(line.x1, line.y1, line.z1);
            
            // Use line-specific width if available, otherwise use default thickness
            let line_width = if i < self.widths.len() {
                self.widths[i]
            } else {
                default_thickness
            };
            
            // Create pipe mesh
            let mut mesh = Mesh::create_pipe(start, end, line_width);
            
            // Set mesh colors from line color
            mesh.facecolors = [
                line.linecolor[0], line.linecolor[1], line.linecolor[2], line.linecolor[3]
            ];
            
            self.meshes.push(mesh);
        }
        
        self.dirty = false;
        self
    }
    
    /// Gets all the mesh representations of this line cloud as pipes.
    /// If the meshes don't exist, creates them first.
    ///
    /// # Returns
    /// A reference to the vector of meshes.
    ///
    /// # Example
    ///
    /// ```
    /// use session_rust::collections::LineCloud;
    /// use session_rust::primitives::Line;
    /// let mut lc = LineCloud::new(
    ///     vec![Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)]
    /// );
    /// let meshes = lc.get_meshes();
    /// assert_eq!(meshes.len(), 1);
    /// ```
    pub fn get_meshes(&mut self) -> &Vec<Mesh> {
        // Create meshes if they don't exist or if they need updating
        if self.meshes.is_empty() || self.dirty {
            self.update_meshes();
        }
        
        &self.meshes
    }
}

impl HasJsonData for LineCloud {
    fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "LineCloud",
            "lines": self.lines.iter().map(|l| l.to_json_data(false)).collect::<Vec<_>>(),
            "xform": {
                "dtype": "Xform",
                "m": self.xform.m
            },
            "guid": self.guid.to_string(),
            "name": self.name
        })
    }
}

impl FromJsonData for LineCloud {
    fn from_json_data(data: &Value) -> Option<Self> {
        if let (
            Some(lines_array), Some(xform_obj),
            Some(guid_str), Some(name)
        ) = (
            data.get("lines").and_then(|v| v.as_array()),
            data.get("xform"),
            data.get("guid").and_then(|v| v.as_str()),
            data.get("name").and_then(|v| v.as_str()),
        ) {
            let lines: Vec<Line> = lines_array.iter()
                .filter_map(|v| Line::from_json_data(v))
                .collect();
            let xform_m = xform_obj.get("m")?.as_array()?;
            let m: Vec<f32> = xform_m.iter().filter_map(|x| x.as_f64().map(|f| f as f32)).collect();
            if m.len() != 16 { return None; }
            let xform = Xform { m: [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15]] };
            let guid = uuid::Uuid::parse_str(guid_str).ok()?;
            Some(LineCloud {
                lines,
                xform,
                guid,
                name: name.to_string(),
                widths: data.get("widths").and_then(|v| v.as_array()).map(|arr| {
                    arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect()
                }).unwrap_or_default(),
                meshes: Vec::new(),
                dirty: true,
            })
        } else {
            None
        }
    }
}
