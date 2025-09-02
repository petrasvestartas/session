use crate::{Point, Line, Xform};
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::{HasJsonData, FromJsonData};

/// Weighting scheme for vertex normal computation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalWeighting {
    /// Weight face normals by face area (default)
    Area,
    /// Weight face normals by interior angle at the vertex
    Angle,
    /// Uniform weighting (all faces contribute equally)
    Uniform,
}


/// Face data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    attributes: HashMap<String, f32>,
}

impl FaceData {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }
}

/// Edge data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    attributes: HashMap<String, f32>,
}

impl EdgeData {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }
}

/// A halfedge mesh data structure for representing polygonal surfaces.
/// 
/// This implementation follows the COMPAS halfedge mesh design, where mesh
/// connectivity is stored using a halfedge data structure. Each edge is split
/// into two halfedges with opposite orientations, enabling efficient topological
/// queries and mesh operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Halfedge connectivity: halfedge[u][v] represents the halfedge from vertex u to vertex v
    pub halfedge: HashMap<usize, HashMap<usize, Option<usize>>>,
    /// Vertices: maps vertex key to vertex data
    pub vertex: HashMap<usize, VertexData>,
    /// Faces: maps face key to list of vertex keys in order
    pub face: HashMap<usize, Vec<usize>>,
    /// Face attributes: maps face key to face attributes
    pub facedata: HashMap<usize, HashMap<String, f32>>,
    /// Edge attributes: maps edge tuple to edge attributes  
    pub edgedata: HashMap<(usize, usize), HashMap<String, f32>>,
    /// Default vertex attributes
    pub default_vertex_attributes: HashMap<String, f32>,
    /// Default face attributes
    pub default_face_attributes: HashMap<String, f32>,
    /// Default edge attributes
    pub default_edge_attributes: HashMap<String, f32>,
    /// Optional cached triangulations per face for viewer rendering (triangles only).
    /// Keyed by face key; each triangle stores vertex keys into `self.vertex`.
    /// Skipped in serialization to keep storage minimal; can be recomputed.
    #[serde(skip)]
    pub triangulation: HashMap<usize, Vec<[usize; 3]>>, 
    /// Next available vertex key
    max_vertex: usize,
    /// Next available face key
    max_face: usize,
    /// Unique identifier
    pub guid: Uuid,
    /// Object name
    pub name: String,
    /// Transformation matrix (4x4 matrix)
    pub xform: Xform,
    /// Face colors as RGBA components [r, g, b, a]
    pub facecolors: [f32; 4],
    /// Line colors as RGBA components [r, g, b, a]
    pub linecolors: [f32; 4],
    /// Point colors as RGBA components [r, g, b, a]
    pub pointcolors: [f32; 4],
    /// Width value for mesh elements
    pub width: f32,
}

/// Vertex data containing position and attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    /// 3D position of the vertex
    #[serde(flatten)]
    pub position: Point,
    /// Vertex attributes organized by type
    pub attributes: HashMap<String, f32>,
}

impl VertexData {
    /// Create a new vertex from a Point
    pub fn new(point: Point) -> Self {
        Self {
            position: point,
            attributes: HashMap::new(),
        }
    }
    
    /// Get the position as a reference
    pub fn position(&self) -> &Point {
        &self.position
    }
    
    /// Set the position from a Point
    pub fn set_position(&mut self, point: Point) {
        self.position = point;
    }
    
    // Convenience methods for common attributes
    pub fn color(&self) -> [f32; 3] {
        [
            self.attributes.get("r").copied().unwrap_or(0.5),
            self.attributes.get("g").copied().unwrap_or(0.5),
            self.attributes.get("b").copied().unwrap_or(0.5),
        ]
    }
    
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.attributes.insert("r".to_string(), r);
        self.attributes.insert("g".to_string(), g);
        self.attributes.insert("b".to_string(), b);
    }
    
    pub fn normal(&self) -> Option<[f32; 3]> {
        let nx = self.attributes.get("nx")?;
        let ny = self.attributes.get("ny")?;
        let nz = self.attributes.get("nz")?;
        Some([*nx, *ny, *nz])
    }
    
    pub fn set_normal(&mut self, nx: f32, ny: f32, nz: f32) {
        self.attributes.insert("nx".to_string(), nx);
        self.attributes.insert("ny".to_string(), ny);
        self.attributes.insert("nz".to_string(), nz);
    }
    
    pub fn tex_coords(&self) -> Option<[f32; 2]> {
        let u = self.attributes.get("u")?;
        let v = self.attributes.get("v")?;
        Some([*u, *v])
    }
    
    pub fn set_tex_coords(&mut self, u: f32, v: f32) {
        self.attributes.insert("u".to_string(), u);
        self.attributes.insert("v".to_string(), v);
    }
    
    // Generic attribute access
    pub fn get_attribute(&self, name: &str) -> Option<f32> {
        self.attributes.get(name).copied()
    }
    
    pub fn set_attribute(&mut self, name: &str, value: f32) {
        self.attributes.insert(name.to_string(), value);
    }
}



impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    /// Create a new empty halfedge mesh.
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// let mesh = Mesh::new();
    /// assert_eq!(mesh.number_of_vertices(), 0);
    /// assert_eq!(mesh.number_of_faces(), 0);
    /// assert!(mesh.is_empty());
    /// ```
    pub fn new() -> Self {
        let mut default_vertex_attributes = HashMap::new();
        default_vertex_attributes.insert("x".to_string(), 0.0);
        default_vertex_attributes.insert("y".to_string(), 0.0);
        default_vertex_attributes.insert("z".to_string(), 0.0);
        
        
        Mesh {
            halfedge: HashMap::new(),
            vertex: HashMap::new(),
            face: HashMap::new(),
            facedata: HashMap::new(),
            edgedata: HashMap::new(),
            default_vertex_attributes,
            default_face_attributes: HashMap::new(),
            default_edge_attributes: HashMap::new(),
            triangulation: HashMap::new(),
            max_vertex: 0,
            max_face: 0,
            guid: Uuid::new_v4(),
            name: "Mesh".to_string(),
            xform: Xform::identity(),
            facecolors: [1.0, 1.0, 1.0, 1.0],
            linecolors: [0.0, 0.0, 0.0, 1.0],
            pointcolors: [0.0, 0.0, 0.0, 1.0],
            width: 1.0,
        }
    }

    /// Check if the mesh is empty.
    /// 
    /// # Returns
    /// True if the mesh has no vertices and no faces
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// let mesh = Mesh::new();
    /// assert!(mesh.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.vertex.is_empty() && self.face.is_empty()
    }

    /// Clear the mesh, removing all vertices and faces.
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// assert!(!mesh.is_empty());
    /// mesh.clear();
    /// assert!(mesh.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.halfedge.clear();
        self.vertex.clear();
        self.face.clear();
        self.facedata.clear();
        self.edgedata.clear();
        self.triangulation.clear();
        self.max_vertex = 0;
        self.max_face = 0;
    }

    /// Get the number of vertices in the mesh.
    /// 
    /// # Returns
    /// The total number of vertices
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// assert_eq!(mesh.number_of_vertices(), 0);
    /// mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// assert_eq!(mesh.number_of_vertices(), 1);
    /// ```
    pub fn number_of_vertices(&self) -> usize {
        self.vertex.len()
    }

    /// Get the number of faces in the mesh.
    /// 
    /// # Returns
    /// The total number of faces
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// assert_eq!(mesh.number_of_faces(), 0);
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// assert_eq!(mesh.number_of_faces(), 1);
    /// ```
    pub fn number_of_faces(&self) -> usize {
        self.face.len()
    }

    /// Get the number of edges in the mesh.
    /// 
    /// # Returns
    /// The total number of edges (undirected)
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// assert_eq!(mesh.number_of_edges(), 3);
    /// ```
    pub fn number_of_edges(&self) -> usize {
        let mut seen = HashSet::new();
        let mut count = 0;
        
        for u in self.halfedge.keys() {
            if let Some(neighbors) = self.halfedge.get(u) {
                for v in neighbors.keys() {
                    let edge = if u < v { (*u, *v) } else { (*v, *u) };
                    if seen.insert(edge) {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    /// Compute the Euler characteristic (V - E + F) of the mesh.
    /// 
    /// # Returns
    /// The Euler characteristic
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// mesh.add_face(vec![v0, v1, v2], None);
    /// assert_eq!(mesh.euler(), 1); // V=3, E=3, F=1 -> 3-3+1=1
    /// ```
    pub fn euler(&self) -> i32 {
        let v = self.number_of_vertices() as i32;
        let e = self.number_of_edges() as i32;
        let f = self.number_of_faces() as i32;
        v - e + f
    }

    /// Add a vertex to the mesh.
    /// 
    /// # Arguments
    /// * `position` - The 3D position of the vertex
    /// * `key` - Optional specific key for the vertex. If None, auto-generates.
    /// 
    /// # Returns
    /// The key of the added vertex
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// let vertex_key = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
    /// assert_eq!(mesh.number_of_vertices(), 1);
    /// ```
    pub fn add_vertex(&mut self, position: Point, key: Option<usize>) -> usize {
        let vertex_key = key.unwrap_or_else(|| {
            self.max_vertex += 1;
            self.max_vertex
        });
        
        // Update max_vertex if explicit key is larger
        if vertex_key >= self.max_vertex {
            self.max_vertex = vertex_key + 1;
        }
        
        let vertex_data = VertexData::new(position);
        self.vertex.insert(vertex_key, vertex_data);
        
        // Initialize halfedge connectivity for this vertex
        self.halfedge.entry(vertex_key).or_insert_with(HashMap::new);
        
        vertex_key
    }

    /// Add a face to the mesh.
    /// 
    /// # Arguments
    /// * `vertices` - List of vertex keys defining the face in order
    /// * `fkey` - Optional specific key for the face. If None, auto-generates.
    /// 
    /// # Returns
    /// The key of the added face, or None if the face is invalid
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let face_key = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// assert_eq!(mesh.number_of_faces(), 1);
    /// ```
    pub fn add_face(&mut self, vertices: Vec<usize>, fkey: Option<usize>) -> Option<usize> {
        // Validate the face
        if vertices.len() < 3 {
            return None;
        }
        
        // Check that all vertices exist
        if !vertices.iter().all(|v| self.vertex.contains_key(v)) {
            return None;
        }
        
        // Check for duplicate vertices
        let mut unique_vertices = HashSet::new();
        for vertex in &vertices {
            if !unique_vertices.insert(*vertex) {
                return None; // Duplicate vertex found
            }
        }
        
        let face_key = fkey.unwrap_or_else(|| {
            self.max_face += 1;
            self.max_face
        });
        
        // Update max_face if explicit key is larger
        if face_key >= self.max_face {
            self.max_face = face_key + 1;
        }
        
        // Add the face
        self.face.insert(face_key, vertices.clone());
        // Invalidate cached triangulation for this face if any
        self.triangulation.remove(&face_key);
        
        // Update halfedge connectivity
        for i in 0..vertices.len() {
            let u = vertices[i];
            let v = vertices[(i + 1) % vertices.len()];
            
            // Ensure both vertices have halfedge entries
            self.halfedge.entry(u).or_insert_with(HashMap::new);
            self.halfedge.entry(v).or_insert_with(HashMap::new);
            
            // Set the halfedge from u to v to point to this face
            self.halfedge.get_mut(&u).unwrap().insert(v, Some(face_key));
            
            // Set the reverse halfedge from v to u (boundary halfedge if no face exists)
            if !self.halfedge.get(&v).unwrap().contains_key(&u) {
                self.halfedge.get_mut(&v).unwrap().insert(u, None);
            }
        }
        
        Some(face_key)
    }

    /// Invalidate triangulation cache for all faces.
    pub fn invalidate_all_triangulation(&mut self) {
        self.triangulation.clear();
    }

    /// Invalidate triangulation cache for a specific face.
    pub fn invalidate_face_triangulation(&mut self, face_key: usize) {
        self.triangulation.remove(&face_key);
    }

    /// Get cached triangulation for a face if present.
    pub fn face_triangulation_cached(&self, face_key: usize) -> Option<&Vec<[usize; 3]>> {
        self.triangulation.get(&face_key)
    }

    /// Compute and cache triangulation for the given face (simple polygon, no holes).
    /// Returns None if the face key is invalid or has fewer than 3 vertices.
    pub fn triangulate_face(&mut self, face_key: usize) -> Option<&Vec<[usize; 3]>> {
        if let Some(vkeys) = self.face.get(&face_key) {
            if vkeys.len() < 3 { return None; }
            let tris = self.ear_clip_triangulate_vertices(vkeys);
            self.triangulation.insert(face_key, tris);
            return self.triangulation.get(&face_key);
        }
        None
    }

    /// Compute triangulation for a face without mutating cache. Useful from immutable context.
    pub fn triangulate_face_vertices(&self, face_vertices: &Vec<usize>) -> Vec<[usize; 3]> {
        if face_vertices.len() < 3 { return Vec::new(); }
        self.ear_clip_triangulate_vertices(face_vertices)
    }

    /// Ear clipping triangulation of a simple 3D polygon (projected to a dominant plane).
    /// No holes supported. Returns triangles as vertex keys in original order.
    fn ear_clip_triangulate_vertices(&self, vkeys: &Vec<usize>) -> Vec<[usize; 3]> {
        if vkeys.len() < 3 { return Vec::new(); }
        if vkeys.len() == 3 {
            return vec![[vkeys[0], vkeys[1], vkeys[2]]];
        }
        
        // Gather points
        let mut pts: Vec<Point> = Vec::with_capacity(vkeys.len());
        for &vk in vkeys.iter() {
            if let Some(p) = self.vertex_position(vk) { pts.push(p); } else { return Vec::new(); }
        }

        // Compute face normal using Newell's method to determine projection plane
        let (nx, ny, nz) = newell_normal(&pts);
        let ax = nx.abs(); let ay = ny.abs(); let az = nz.abs();
        
        // Project to 2D - choose plane by dropping dominant normal axis
        let mut p2d: Vec<[f32; 2]> = Vec::with_capacity(pts.len());
        for p in &pts {
            let proj = if ax >= ay && ax >= az {
                [p.y, p.z]  // Drop X, use YZ plane
            } else if ay >= ax && ay >= az {
                [p.x, p.z]  // Drop Y, use XZ plane  
            } else {
                [p.x, p.y]  // Drop Z, use XY plane
            };
            p2d.push(proj);
        }
        
        // Determine CCW orientation using standard signed area (shoelace)
        let area = signed_area_2d(&p2d);
        let mut idx: Vec<usize> = (0..vkeys.len()).collect();
        if area < 0.0 { idx.reverse(); }

        // Reindex 2D points to CCW order for convexity test
        let p2_ccw: Vec<[f32; 2]> = idx.iter().map(|&i| p2d[i]).collect();

        // Convexity test in 2D (CCW): all consecutive turns must be left turns
        let is_convex_polygon = |pts2: &Vec<[f32; 2]>| -> bool {
            let m = pts2.len();
            if m < 3 { return false; }
            let eps = 1e-12;
            for i in 0..m {
                let a = pts2[i];
                let b = pts2[(i + 1) % m];
                let c = pts2[(i + 2) % m];
                let abx = b[0] - a[0];
                let aby = b[1] - a[1];
                let bcx = c[0] - b[0];
                let bcy = c[1] - b[1];
                let cross = abx * bcy - aby * bcx;
                if cross < -eps { return false; }
            }
            true
        };

        if is_convex_polygon(&p2_ccw) {
            // Fan triangulation for convex polygons; enforce triangle winding matching 3D face normal
            let mut triangles: Vec<[usize; 3]> = Vec::new();
            for i in 1..(idx.len() - 1) {
                let i0 = idx[0];
                let i1 = idx[i];
                let i2 = idx[i + 1];

                // Enforce triangle winding to match the face normal
                let a3 = &pts[i0];
                let b3 = &pts[i1];
                let c3 = &pts[i2];
                let ux = b3.x - a3.x; let uy = b3.y - a3.y; let uz = b3.z - a3.z;
                let vx = c3.x - a3.x; let vy = c3.y - a3.y; let vz = c3.z - a3.z;
                let cx = uy * vz - uz * vy;
                let cy = uz * vx - ux * vz;
                let cz = ux * vy - uy * vx;
                let dot = cx * nx + cy * ny + cz * nz;
                if dot >= 0.0 {
                    triangles.push([vkeys[i0], vkeys[i1], vkeys[i2]]);
                } else {
                    triangles.push([vkeys[i0], vkeys[i2], vkeys[i1]]);
                }
            }
            return triangles;
        }

        // Fallback to full ear clipping for concave polygons (e.g., star polygon)
        return self.full_ear_clip_triangulate(vkeys);
    }

    // For complex polygons, fall back to full ear clipping
    fn full_ear_clip_triangulate(&self, vkeys: &Vec<usize>) -> Vec<[usize; 3]> {
        // Gather points
        let mut pts: Vec<Point> = Vec::with_capacity(vkeys.len());
        for &vk in vkeys.iter() {
            if let Some(p) = self.vertex_position(vk) { pts.push(p); } else { return Vec::new(); }
        }

        // Compute face normal using Newell's method
        let (nx, ny, nz) = newell_normal(&pts);
        let ax = nx.abs(); let ay = ny.abs(); let az = nz.abs();
        
        // Choose projection plane by dropping the dominant axis
        enum Plane { XY, XZ, YZ }
        let plane = if ax >= ay && ax >= az { Plane::YZ } else if ay >= ax && ay >= az { Plane::XZ } else { Plane::XY };

        // Project to 2D
        let mut p2: Vec<[f32;2]> = Vec::with_capacity(pts.len());
        for p in &pts {
            let q = match plane {
                Plane::XY => [p.x, p.y],
                Plane::XZ => [p.x, p.z],
                Plane::YZ => [p.y, p.z],
            };
            p2.push(q);
        }

        // Determine polygon orientation (CCW positive area)
        let area = signed_area_2d(&p2);
        let ccw = area > 0.0;

        // Indices into vkeys/pts
        let mut idx: Vec<usize> = (0..vkeys.len()).collect();
        let mut tris: Vec<[usize;3]> = Vec::with_capacity(vkeys.len().saturating_sub(2));
        let n = idx.len();
        if n < 3 { return tris; }

        // Helper closures
        let is_convex = |a: usize, b: usize, c: usize| -> bool {
            let abx = p2[b][0] - p2[a][0];
            let aby = p2[b][1] - p2[a][1];
            let bcx = p2[c][0] - p2[b][0];
            let bcy = p2[c][1] - p2[b][1];
            let cross = abx * bcy - aby * bcx;
            if ccw { cross > 1e-12 } else { cross < -1e-12 }
        };
        let point_in_tri = |a: usize, b: usize, c: usize, p: usize| -> bool {
            // Barycentric sign method respecting orientation
            let (ax2, ay2) = (p2[a][0], p2[a][1]);
            let (bx2, by2) = (p2[b][0], p2[b][1]);
            let (cx2, cy2) = (p2[c][0], p2[c][1]);
            let (px2, py2) = (p2[p][0], p2[p][1]);
            let sign = |x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32| -> f32 {
                (x1 - x3) * (y2 - y3) - (x2 - x3) * (y1 - y3)
            };
            let s1 = sign(px2, py2, ax2, ay2, bx2, by2);
            let s2 = sign(px2, py2, bx2, by2, cx2, cy2);
            let s3 = sign(px2, py2, cx2, cy2, ax2, ay2);
            let has_neg = (s1 < -1e-12) || (s2 < -1e-12) || (s3 < -1e-12);
            let has_pos = (s1 > 1e-12) || (s2 > 1e-12) || (s3 > 1e-12);
            !(has_neg && has_pos)
        };

        // If orientation is CW, reverse to make it CCW for ear clipping test simplicity
        if !ccw { idx.reverse(); }

        let mut guard = 0usize; // prevent infinite loops
        while idx.len() > 3 && guard < n * n {
            let m = idx.len();
            let mut ear_found = false;
            for j in 0..m {
                let i0 = idx[(j + m - 1) % m];
                let i1 = idx[j];
                let i2 = idx[(j + 1) % m];

                if !is_convex(i0, i1, i2) { continue; }

                // Check if any other vertex lies in triangle (i0,i1,i2)
                let mut contains = false;
                for &k in idx.iter() {
                    if k == i0 || k == i1 || k == i2 { continue; }
                    if point_in_tri(i0, i1, i2, k) { contains = true; break; }
                }
                if contains { continue; }

                // It's an ear: emit triangle in original vertex-key space
                // Enforce triangle winding to match the face normal (Newell)
                let a3 = &pts[i0];
                let b3 = &pts[i1];
                let c3 = &pts[i2];
                let ux = b3.x - a3.x; let uy = b3.y - a3.y; let uz = b3.z - a3.z;
                let vx = c3.x - a3.x; let vy = c3.y - a3.y; let vz = c3.z - a3.z;
                let cx = uy * vz - uz * vy;
                let cy = uz * vx - ux * vz;
                let cz = ux * vy - uy * vx;
                let dot = cx * nx + cy * ny + cz * nz;
                if dot >= 0.0 {
                    tris.push([vkeys[i0], vkeys[i1], vkeys[i2]]);
                } else {
                    tris.push([vkeys[i0], vkeys[i2], vkeys[i1]]);
                }
                idx.remove(j);
                ear_found = true;
                break;
            }
            if !ear_found { 
                break; 
            }
            guard += 1;
        }
        // Handle remaining vertices
        if idx.len() == 3 {
            let i0 = idx[0]; let i1 = idx[1]; let i2 = idx[2];
            let a3 = &pts[i0];
            let b3 = &pts[i1];
            let c3 = &pts[i2];
            let ux = b3.x - a3.x; let uy = b3.y - a3.y; let uz = b3.z - a3.z;
            let vx = c3.x - a3.x; let vy = c3.y - a3.y; let vz = c3.z - a3.z;
            let cx = uy * vz - uz * vy;
            let cy = uz * vx - ux * vz;
            let cz = ux * vy - uy * vx;
            let dot = cx * nx + cy * ny + cz * nz;
            if dot >= 0.0 {
                tris.push([vkeys[i0], vkeys[i1], vkeys[i2]]);
            } else {
                tris.push([vkeys[i0], vkeys[i2], vkeys[i1]]);
            }
        } else if idx.len() > 3 {
            // Fallback: use fan triangulation from first vertex for remaining vertices
            let center = idx[0];
            for i in 1..(idx.len() - 1) {
                let v1 = idx[i];
                let v2 = idx[i + 1];
                tris.push([vkeys[center], vkeys[v1], vkeys[v2]]);
            }
        }
        tris
    }

    /// Get the position of a vertex.
    /// 
    /// # Arguments
    /// * `vertex_key` - The key of the vertex
    /// 
    /// # Returns
    /// The position of the vertex, or None if vertex doesn't exist
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// let v = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
    /// let pos = mesh.vertex_position(v).unwrap();
    /// assert_eq!(pos.x, 1.0);
    /// assert_eq!(pos.y, 2.0);
    /// assert_eq!(pos.z, 3.0);
    /// ```
    pub fn vertex_position(&self, vertex_key: usize) -> Option<Point> {
        self.vertex.get(&vertex_key).map(|v| v.position().clone())
    }

    /// Get the vertices of a face.
    /// 
    /// # Arguments
    /// * `face_key` - The key of the face
    /// 
    /// # Returns
    /// A list of vertex keys defining the face, or None if face doesn't exist
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
    /// let vertices = mesh.face_vertices(f).unwrap();
    /// assert_eq!(*vertices, vec![v0, v1, v2]);
    /// ```
    pub fn face_vertices(&self, face_key: usize) -> Option<&Vec<usize>> {
        self.face.get(&face_key)
    }
    
    /// Get all face data as an iterator over (face_key, face_vertices) pairs.
    /// 
    /// This method provides access to all faces in the mesh for iteration.
    /// Useful for converting to other mesh representations.
    /// 
    /// # Returns
    /// An iterator over (face_key, face_vertices) pairs
    /// 
    /// # Example
    /// 
    /// ```
    /// use session_rust::collections::Mesh;
    /// use session_rust::primitives::Point;
    /// let mut mesh = Mesh::new();
    /// let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    /// let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    /// let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
    /// ```
    pub fn get_face_data(&self) -> impl Iterator<Item = (usize, &Vec<usize>)> {
        self.face.iter().map(|(k, v)| (*k, v))
    }

    /// Build a mesh from polygons, merging vertices within an optional precision.
    ///
    /// - If `precision` is Some(eps), vertices whose coordinates are within eps are merged
    ///   using integer grid quantization (round(x/eps)).
    /// - If `precision` is None, only exactly equal coordinates are merged (bitwise equality).
    pub fn from_polygons_with_merge(polygons: Vec<Vec<Point>>, precision: Option<f32>) -> Self {
        let mut mesh = Mesh::new();

        // Maps for vertex deduplication
        let mut map_eps: HashMap<(i64, i64, i64), usize> = HashMap::new();
        let mut map_exact: HashMap<(u64, u64, u64), usize> = HashMap::new();
        let eps = precision.unwrap_or(0.0);
        let use_eps = eps > 0.0;

        // Helper to get or create a vertex key for a given point
        let mut get_vkey = |p: &Point, mesh: &mut Mesh| -> usize {
            if use_eps {
                let kx = (p.x / eps).round() as i64;
                let ky = (p.y / eps).round() as i64;
                let kz = (p.z / eps).round() as i64;
                let key = (kx, ky, kz);
                if let Some(&vk) = map_eps.get(&key) { return vk; }
                let vk = mesh.add_vertex(p.clone(), None);
                map_eps.insert(key, vk);
                vk
            } else {
                let key = (p.x.to_bits() as u64, p.y.to_bits() as u64, p.z.to_bits() as u64);
                if let Some(&vk) = map_exact.get(&key) { return vk; }
                let vk = mesh.add_vertex(p.clone(), None);
                map_exact.insert(key, vk);
                vk
            }
        };

        for poly in polygons.into_iter() {
            if poly.len() < 3 { continue; }
            let mut vkeys: Vec<usize> = Vec::with_capacity(poly.len());
            for p in &poly {
                let vk = get_vkey(p, &mut mesh);
                vkeys.push(vk);
            }
            
            // Add face as-is without triangulation (preserves original polygon faces)
            let _ = mesh.add_face(vkeys, None);
        }

        mesh
    }


    /// Convenience wrapper that forwards to `from_polygons_with_merge`.
    pub fn from_polygons(polygons: Vec<Vec<Point>>, precision: Option<f32>) -> Self {
        Self::from_polygons_with_merge(polygons, precision)
    }

    /// Export mesh as separate buffers compatible with `ModelMesh`.
    /// Returns (positions, indices, normals, colors, vertex_count, triangle_count).
    pub fn to_model_mesh_buffers(&mut self) -> (Vec<f32>, Vec<u32>, Vec<f32>, Vec<f32>, usize, usize) {
        // 1) Collect all unique vertex keys used by faces, in insertion order
        let mut vkey_to_index: HashMap<usize, usize> = HashMap::new();
        let mut unique_vkeys: Vec<usize> = Vec::new();
        for (_f, vlist) in self.get_face_data() {
            for &vk in vlist {
                if !vkey_to_index.contains_key(&vk) {
                    let idx = unique_vkeys.len();
                    vkey_to_index.insert(vk, idx);
                    unique_vkeys.push(vk);
                }
            }
        }

        let v_count = unique_vkeys.len();
        let mut positions: Vec<f32> = Vec::with_capacity(v_count * 3);
        let mut normals_acc: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]; v_count];
        let mut colors: Vec<f32> = Vec::with_capacity(v_count * 3);

        // 2) Fill positions and colors
        for &vk in &unique_vkeys {
            if let Some(p) = self.vertex_position(vk) {
                positions.push(p.x as f32);
                positions.push(p.y as f32);
                positions.push(p.z as f32);
            } else {
                positions.extend_from_slice(&[0.0, 0.0, 0.0]);
            }

            // Use stored vertex color if present, else default white
            if let Some(vd) = self.vertex.get(&vk) {
                let c = vd.color();
                colors.push(c[0] as f32);
                colors.push(c[1] as f32);
                colors.push(c[2] as f32);
            } else {
                colors.extend_from_slice(&[1.0, 1.0, 1.0]);
            }
        }

        // 3) Triangulate faces using cached triangulation and build indices
        let mut indices: Vec<u32> = Vec::new();
        let mut tri_count = 0;
        
        // First, collect all triangulations to avoid borrowing conflicts
        let face_keys: Vec<usize> = self.face.keys().copied().collect();
        let mut all_triangles: Vec<[usize; 3]> = Vec::new();
        
        for face_key in face_keys {
            if let Some(triangles) = self.triangulate_face(face_key) {
                all_triangles.extend_from_slice(triangles);
            }
        }
        
        // Now process triangles without borrowing conflicts
        for tri in all_triangles {
            let a = *vkey_to_index.get(&tri[0]).unwrap();
            let b = *vkey_to_index.get(&tri[1]).unwrap();
            let c = *vkey_to_index.get(&tri[2]).unwrap();
            indices.push(a as u32);
            indices.push(b as u32);
            indices.push(c as u32);
            tri_count += 1;

            // Accumulate area-weighted normals
            let p0 = self.vertex_position(tri[0]).unwrap();
            let p1 = self.vertex_position(tri[1]).unwrap();
            let p2 = self.vertex_position(tri[2]).unwrap();
            let ux = p1.x - p0.x; let uy = p1.y - p0.y; let uz = p1.z - p0.z;
            let vx = p2.x - p0.x; let vy = p2.y - p0.y; let vz = p2.z - p0.z;
            let nx = uy * vz - uz * vy;
            let ny = uz * vx - ux * vz;
            let nz = ux * vy - uy * vx;
            normals_acc[a][0] += nx; normals_acc[a][1] += ny; normals_acc[a][2] += nz;
            normals_acc[b][0] += nx; normals_acc[b][1] += ny; normals_acc[b][2] += nz;
            normals_acc[c][0] += nx; normals_acc[c][1] += ny; normals_acc[c][2] += nz;
        }

        // 4) Normalize accumulated normals
        let mut normals: Vec<f32> = Vec::with_capacity(v_count * 3);
        for i in 0..v_count {
            let (nx, ny, nz) = (normals_acc[i][0], normals_acc[i][1], normals_acc[i][2]);
            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            if len > 0.0 {
                normals.push((nx / len) as f32);
                normals.push((ny / len) as f32);
                normals.push((nz / len) as f32);
            } else {
                normals.extend_from_slice(&[0.0, 0.0, 1.0]);
            }
        }

        (positions, indices, normals, colors, v_count, tri_count)
    }

    /// Export mesh as interleaved [x,y,z, nx,ny,nz, r,g,b] with separate indices.
    /// Returns (interleaved, indices, vertex_count, triangle_count).
    pub fn to_model_mesh_interleaved(&mut self) -> (Vec<f32>, Vec<u32>, usize, usize) {
        let (positions, indices, normals, colors, v_count, tri_count) = self.to_model_mesh_buffers();
        let mut interleaved: Vec<f32> = Vec::with_capacity(v_count * 9);
        for i in 0..v_count {
            interleaved.push(positions[i * 3 + 0]);
            interleaved.push(positions[i * 3 + 1]);
            interleaved.push(positions[i * 3 + 2]);
            interleaved.push(normals[i * 3 + 0]);
            interleaved.push(normals[i * 3 + 1]);
            interleaved.push(normals[i * 3 + 2]);
            interleaved.push(colors[i * 3 + 0]);
            interleaved.push(colors[i * 3 + 1]);
            interleaved.push(colors[i * 3 + 2]);
        }
        (interleaved, indices, v_count, tri_count)
    }

    /// Return true if the vertex is on a boundary (i.e., participates in at least one boundary halfedge).
    pub fn is_vertex_on_boundary(&self, vertex_key: usize) -> bool {
        // Check outgoing halfedges from this vertex
        if let Some(neigh) = self.halfedge.get(&vertex_key) {
            for (_v, face_opt) in neigh.iter() {
                if face_opt.is_none() {
                    return true;
                }
            }
        }
        // Check incoming halfedges to this vertex
        for (_u, neigh) in self.halfedge.iter() {
            if let Some(face_opt) = neigh.get(&vertex_key) {
                if face_opt.is_none() {
                    return true;
                }
            }
        }
        false
    }

    /// Extract all unique edges of the mesh as Line objects.
    /// This includes both boundary and interior edges.
    pub fn extract_edges_as_lines(&self) -> Vec<Line> {
        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for (_, face_vertices) in self.face.iter() {
            for i in 0..face_vertices.len() {
                let a = face_vertices[i];
                let b = face_vertices[(i + 1) % face_vertices.len()];
                let key = if a < b { (a, b) } else { (b, a) };
                if !seen.insert(key) { continue; }
                if let (Some(p0), Some(p1)) = (self.vertex_position(a), self.vertex_position(b)) {
                    let mut line = Line::new(p0.x, p0.y, p0.z, p1.x, p1.y, p1.z);
                    line.linecolor = self.linecolors;
                    line.width = self.width;
                    out.push(line);
                }
            }
        }
        out
    }

    /// Extract all edges as pipe meshes with specified radius and sides
    pub fn extract_edges_as_pipes(&self, radius: f32, sides: Option<usize>) -> Vec<Mesh> {
        let mut out = Vec::new();
        let mut seen = std::collections::HashSet::new();
        
        for (_, face_vertices) in self.face.iter() {
            for i in 0..face_vertices.len() {
                let a = face_vertices[i];
                let b = face_vertices[(i + 1) % face_vertices.len()];
                let key = if a < b { (a, b) } else { (b, a) };
                if !seen.insert(key) { continue; }
                if let (Some(p0), Some(p1)) = (self.vertex_position(a), self.vertex_position(b)) {
                    let seg = Line::new(p0.x, p0.y, p0.z, p1.x, p1.y, p1.z);
                    if let Some(pipe_mesh) = seg.to_pipe_mesh(radius, sides) {
                        out.push(pipe_mesh);
                    }
                }
            }
        }
        out
    }

    /// Extract transforms for pipes along unique boundary edges of the mesh.
    /// Uses the canonical unit pipe definition (aligned +Z, length=1, radius=0.5).
    pub fn extract_edge_pipe_transforms(&self) -> Vec<Xform> {
        let mut out: Vec<Xform> = Vec::new();
        let mut seen: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        

        for (u, neigh) in &self.halfedge {
            for (v, face_opt) in neigh {
                // Only consider boundary halfedges (no face on this oriented halfedge)
                if face_opt.is_none() {
                    let a = *u;
                    let b = *v;
                    let key = if a < b { (a, b) } else { (b, a) };
                    if !seen.insert(key) { continue; }
                    if let (Some(p0), Some(p1)) = (self.vertex_position(a), self.vertex_position(b)) {
                        let seg = Line::new(p0.x, p0.y, p0.z, p1.x, p1.y, p1.z);
                        if let Some(tf) = seg.to_pipe_transform() { out.push(tf); }
                    }
                }
            }
        }
        out
    }

    /// Create a low-resolution unit pipe (cylinder) with radius 0.5.
    /// - Radius = 0.5
    /// - Length = 1.0 (z in [-0.5, 0.5])
    /// - Radial segments = 8 (low-res for performance)
    pub fn create_unit_pipe_low_res() -> Self {
        let mut m = Mesh::new();
        let sides: usize = 8; // Reduced from 32 to 8 for performance
        let r: f32 = 0.5;
        let hz: f32 = 0.5;

        let mut ring_bot: Vec<usize> = Vec::with_capacity(sides);
        let mut ring_top: Vec<usize> = Vec::with_capacity(sides);

        for i in 0..sides {
            let theta = 2.0 * PI * (i as f32) / (sides as f32);
            let x = r * theta.cos();
            let y = r * theta.sin();
            let vb = m.add_vertex(Point::new(x, y, -hz), None);
            let vt = m.add_vertex(Point::new(x, y, hz), None);
            ring_bot.push(vb);
            ring_top.push(vt);
        }

        for i in 0..sides {
            let j = (i + 1) % sides;
            // Quad face for side wall (ordered to produce outward normals)
            let _ = m.add_face(vec![ring_bot[i], ring_bot[j], ring_top[j], ring_top[i]], None);
        }

        m
    }

    /// Create a high-resolution unit pipe (cylinder) with radius 0.5.
    /// - Radius = 0.5
    /// - Length = 1.0 (z in [-0.5, 0.5])
    /// - Radial segments = 32
    pub fn create_unit_pipe_high_res() -> Self {
        let mut m = Mesh::new();
        let sides: usize = 32;
        let r: f32 = 0.5;
        let hz: f32 = 0.5;

        let mut ring_bot: Vec<usize> = Vec::with_capacity(sides);
        let mut ring_top: Vec<usize> = Vec::with_capacity(sides);

        for i in 0..sides {
            let theta = 2.0 * PI * (i as f32) / (sides as f32);
            let x = r * theta.cos();
            let y = r * theta.sin();
            let vb = m.add_vertex(Point::new(x, y, -hz), None);
            let vt = m.add_vertex(Point::new(x, y, hz), None);
            ring_bot.push(vb);
            ring_top.push(vt);
        }

        for i in 0..sides {
            let j = (i + 1) % sides;
            // Quad face for side wall (ordered to produce outward normals)
            let _ = m.add_face(vec![ring_bot[i], ring_bot[j], ring_top[j], ring_top[i]], None);
        }

        m
    }

    /// Create a low-resolution unit sphere (icosphere) with radius 0.5.
    /// Subdivisions: 1 for performance (low-res)
    pub fn create_unit_sphere_low_res() -> Self {
        Self::create_unit_sphere_subdivisions(1) // Reduced from 3 to 1 for performance
    }

    /// Create a high-resolution unit sphere (icosphere) with radius 0.5.
    /// Subdivisions: 3 for a smooth sphere.
    pub fn create_unit_sphere_high_res() -> Self {
        Self::create_unit_sphere_subdivisions(3)
    }

    /// Create a unit sphere with specified subdivision levels.
    pub fn create_unit_sphere_subdivisions(subdiv: usize) -> Self {
        // Build an icosphere in local space, then load into Mesh.
        let radius: f32 = 0.5;
        let _subdiv_param = subdiv;

        // Initial icosahedron vertices
        let t = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let mut pts: Vec<Point> = vec![
            Point::new(-1.0,                        t,    0.0),
            Point::new( 1.0,                        t,    0.0),
            Point::new(-1.0,                       -t,    0.0),
            Point::new( 1.0,                       -t,    0.0),
            Point::new( 0.0,                       -1.0,  t  ),
            Point::new( 0.0,                        1.0,  t  ),
            Point::new( 0.0,                       -1.0, -t  ),
            Point::new( 0.0,                        1.0, -t  ),
            Point::new( t,                         0.0, -1.0),
            Point::new( t,                         0.0,  1.0),
            Point::new(-t,                         0.0, -1.0),
            Point::new(-t,                         0.0,  1.0),
            Point::new( 0.0,  1.0, -t  ),
            Point::new(  t,   0.0, -1.0),
            Point::new(  t,   0.0,  1.0),
            Point::new( -t,   0.0, -1.0),
            Point::new( -t,   0.0,  1.0),
        ];

        // Normalize to radius
        let norm_to_r = |p: &Point, r: f32| -> Point {
            let len = (p.x*p.x + p.y*p.y + p.z*p.z).sqrt();
            if len > 0.0 { Point::new(p.x/len*r, p.y/len*r, p.z/len*r) } else { Point::new(0.0, 0.0, r) }
        };
        for p in &mut pts { *p = norm_to_r(p, radius); }

        // Base icosahedron faces
        let mut faces: Vec<[usize; 3]> = vec![
            [0, 11, 5],  [0, 5, 1],   [0, 1, 7],   [0, 7, 10],  [0, 10, 11],
            [1, 5, 9],   [5, 11, 4],  [11, 10, 2], [10, 7, 6],  [7, 1, 8],
            [3, 9, 4],   [3, 4, 2],   [3, 2, 6],   [3, 6, 8],   [3, 8, 9],
            [4, 9, 5],   [2, 4, 11],  [6, 2, 10],  [8, 6, 7],   [9, 8, 1],
        ];

        // Subdivision helper: edge midpoint cache
        use std::collections::HashMap;
        
        for _ in 0.._subdiv_param {
            let mut new_faces: Vec<[usize; 3]> = Vec::with_capacity(faces.len()*4);
            let mut midpoint_cache: HashMap<(usize, usize), usize> = HashMap::new();
            
            let get_midpoint = |a: usize, b: usize, pts: &mut Vec<Point>, cache: &mut HashMap<(usize, usize), usize>| -> usize {
                let key = if a < b { (a, b) } else { (b, a) };
                if let Some(&idx) = cache.get(&key) { return idx; }
                let pa = &pts[a];
                let pb = &pts[b];
                let pm = Point::new((pa.x + pb.x)*0.5, (pa.y + pb.y)*0.5, (pa.z + pb.z)*0.5);
                let pm = norm_to_r(&pm, radius);
                let idx = pts.len();
                pts.push(pm);
                cache.insert(key, idx);
                idx
            };
            
            for [i, j, k] in faces.iter().copied() {
                let a = get_midpoint(i, j, &mut pts, &mut midpoint_cache);
                let b = get_midpoint(j, k, &mut pts, &mut midpoint_cache);
                let c = get_midpoint(k, i, &mut pts, &mut midpoint_cache);
                new_faces.push([i, a, c]);
                new_faces.push([j, b, a]);
                new_faces.push([k, c, b]);
                new_faces.push([a, b, c]);
            }
            faces = new_faces;
        }

        // Build Mesh from points and faces
        let mut m = Mesh::new();
        let mut vmap: Vec<usize> = Vec::with_capacity(pts.len());
        for p in pts.into_iter() {
            let vk = m.add_vertex(p, None);
            vmap.push(vk);
        }
        // Enforce consistent outward winding per triangle
        for [a, b, c] in faces {
            let pa = m.vertex_position(vmap[a]).unwrap();
            let pb = m.vertex_position(vmap[b]).unwrap();
            let pc = m.vertex_position(vmap[c]).unwrap();
            let ux = pb.x - pa.x; let uy = pb.y - pa.y; let uz = pb.z - pa.z;
            let vx = pc.x - pa.x; let vy = pc.y - pa.y; let vz = pc.z - pa.z;
            let nx = uy * vz - uz * vy;
            let ny = uz * vx - ux * vz;
            let nz = ux * vy - uy * vx;
            // Outward if normal roughly agrees with radial direction (use pa)
            let dot = nx * pa.x + ny * pa.y + nz * pa.z;
            if dot >= 0.0 {
                let _ = m.add_face(vec![vmap[a], vmap[b], vmap[c]], None);
            } else {
                let _ = m.add_face(vec![vmap[a], vmap[c], vmap[b]], None);
            }
        }

        m
    }

    /// Create a low-resolution pipe mesh for backward compatibility.
    /// 8-sided cylinder with radius and length based on start/end points.
    pub fn create_pipe(start: Point, end: Point, thickness: f32) -> Self {
        let mut m = Mesh::new();
        let sides: usize = 8;
        let r = thickness * 0.5;
        
        // Calculate direction and length
        let dir_x = end.x - start.x;
        let dir_y = end.y - start.y;
        let dir_z = end.z - start.z;
        let length = (dir_x*dir_x + dir_y*dir_y + dir_z*dir_z).sqrt();
        
        if length < 1e-9 { return m; }
        
        // Normalize direction
        let nx = dir_x / length;
        let ny = dir_y / length;
        let nz = dir_z / length;
        
        // Create orthogonal basis
        let (ux, uy, uz) = if nz.abs() < 0.9 {
            let len = (nx*nx + ny*ny).sqrt();
            (-ny/len, nx/len, 0.0)
        } else {
            let len = (ny*ny + nz*nz).sqrt();
            (0.0, -nz/len, ny/len)
        };
        
        let vx = ny*uz - nz*uy;
        let vy = nz*ux - nx*uz;
        let vz = nx*uy - ny*ux;
        
        // Generate vertices
        let mut ring_start: Vec<usize> = Vec::with_capacity(sides);
        let mut ring_end: Vec<usize> = Vec::with_capacity(sides);
        
        for i in 0..sides {
            let theta = 2.0 * PI * (i as f32) / (sides as f32);
            let cos_t = theta.cos();
            let sin_t = theta.sin();
            
            let offset_x = r * (cos_t * ux + sin_t * vx);
            let offset_y = r * (cos_t * uy + sin_t * vy);
            let offset_z = r * (cos_t * uz + sin_t * vz);
            
            let vs = m.add_vertex(Point::new(
                start.x + offset_x,
                start.y + offset_y,
                start.z + offset_z
            ), None);
            
            let ve = m.add_vertex(Point::new(
                end.x + offset_x,
                end.y + offset_y,
                end.z + offset_z
            ), None);
            
            ring_start.push(vs);
            ring_end.push(ve);
        }
        
        // Create side faces as triangles (not quads)
        for i in 0..sides {
            let j = (i + 1) % sides;
            // Split quad into two triangles
            let _ = m.add_face(vec![ring_start[i], ring_start[j], ring_end[j]], None);
            let _ = m.add_face(vec![ring_start[i], ring_end[j], ring_end[i]], None);
        }
        
        // Create end caps
        // Start cap (center vertex + triangles)
        let start_center = m.add_vertex(start, None);
        for i in 0..sides {
            let j = (i + 1) % sides;
            let _ = m.add_face(vec![start_center, ring_start[j], ring_start[i]], None);
        }
        
        // End cap (center vertex + triangles)
        let end_center = m.add_vertex(end, None);
        for i in 0..sides {
            let j = (i + 1) % sides;
            let _ = m.add_face(vec![end_center, ring_end[i], ring_end[j]], None);
        }
        
        m
    }

    /// Resolve vertex normal with fallback hierarchy:
    /// 1. Stored per-vertex nx,ny,nz attributes
    /// 2. Computed area-weighted vertex normal
    /// 3. Face normal (if face_key_hint provided)
    /// 4. Default +Z normal
    pub fn vertex_normal_resolved(&self, vertex_key: usize, face_key_hint: Option<usize>) -> crate::Vector {
        use crate::Vector;
        
        // 1. Check for stored per-vertex normal attributes
        if let Some(vd) = self.vertex.get(&vertex_key) {
            if let (Some(&nx), Some(&ny), Some(&nz)) = (
                vd.attributes.get("nx"),
                vd.attributes.get("ny"),
                vd.attributes.get("nz")
            ) {
                let len = (nx*nx + ny*ny + nz*nz).sqrt();
                if len > 1e-9 {
                    return Vector::new(nx/len, ny/len, nz/len);
                }
            }
        }
        
        // 2. Compute area-weighted vertex normal
        let mut normal_acc = Vector::new(0.0, 0.0, 0.0);
        let mut face_count = 0;
        
        // Find all faces adjacent to this vertex
        for (_face_key, face_vertices) in &self.face {
            if face_vertices.contains(&vertex_key) {
                if let Some(face_normal) = self.compute_face_normal_from_vertices(face_vertices) {
                    normal_acc.x += face_normal.x;
                    normal_acc.y += face_normal.y;
                    normal_acc.z += face_normal.z;
                    face_count += 1;
                }
            }
        }
        
        if face_count > 0 {
            let len = (normal_acc.x*normal_acc.x + normal_acc.y*normal_acc.y + normal_acc.z*normal_acc.z).sqrt();
            if len > 1e-9 {
                return Vector::new(normal_acc.x/len, normal_acc.y/len, normal_acc.z/len);
            }
        }
        
        // 3. Use face normal if hint provided
        if let Some(face_key) = face_key_hint {
            if let Some(face_vertices) = self.face.get(&face_key) {
                if let Some(face_normal) = self.compute_face_normal_from_vertices(face_vertices) {
                    return face_normal;
                }
            }
        }
        
        // 4. Default +Z normal
        Vector::new(0.0, 0.0, 1.0)
    }

    /// Compute face normal from vertex list using cross product
    fn compute_face_normal_from_vertices(&self, face_vertices: &[usize]) -> Option<crate::Vector> {
        use crate::Vector;
        
        if face_vertices.len() < 3 { return None; }
        
        let v0_key = face_vertices[0];
        let v1_key = face_vertices[1];
        let v2_key = face_vertices[2];
        
        let v0 = self.vertex.get(&v0_key)?.position();
        let v1 = self.vertex.get(&v1_key)?.position();
        let v2 = self.vertex.get(&v2_key)?.position();
        
        let edge1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let edge2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
        
        let normal = edge1.cross(&edge2);
        let length = normal.length();
        
        if length > 1e-10 {
            Some(Vector::new(normal.x / length, normal.y / length, normal.z / length))
        } else {
            None
        }
    }

    
    /// Get normal for a specific face
    pub fn face_normal(&self, face_key: usize) -> Option<crate::Vector> {
        let face_vertices = self.face.get(&face_key)?;
        self.compute_face_normal_from_vertices(face_vertices)
    }

    /// Get normal for a specific vertex (area-weighted average)
    pub fn vertex_normal(&self, vertex_key: usize) -> Option<crate::Vector> {
        Some(self.vertex_normal_resolved(vertex_key, None))
    }

    /// Get area of a specific face
    pub fn face_area(&self, face_key: usize) -> Option<f32> {
        use crate::Vector;
        let face_vertices = self.face.get(&face_key)?;
        if face_vertices.len() < 3 { return Some(0.0); }
        
        let v0 = self.vertex.get(&face_vertices[0])?.position();
        let v1 = self.vertex.get(&face_vertices[1])?.position();
        let v2 = self.vertex.get(&face_vertices[2])?.position();
        
        let edge1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let edge2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
        
        Some(edge1.cross(&edge2).length() * 0.5)
    }

    /// Get normals for all faces
    pub fn face_normals(&self) -> std::collections::HashMap<usize, crate::Vector> {
        let mut normals = std::collections::HashMap::new();
        for (&face_key, _) in &self.face {
            if let Some(normal) = self.face_normal(face_key) {
                normals.insert(face_key, normal);
            }
        }
        normals
    }

    /// Get normals for all vertices
    pub fn vertex_normals(&self) -> std::collections::HashMap<usize, crate::Vector> {
        let mut normals = std::collections::HashMap::new();
        for (&vertex_key, _) in &self.vertex {
            normals.insert(vertex_key, self.vertex_normal_resolved(vertex_key, None));
        }
        normals
    }


    /// Get weighted normal for a vertex
    pub fn vertex_normal_weighted(&self, vertex_key: usize, weighting: NormalWeighting) -> Option<crate::Vector> {
        use crate::Vector;
        
        let mut normal_acc = Vector::new(0.0, 0.0, 0.0);
        let mut weight_sum = 0.0;
        
        for (&face_key, face_vertices) in &self.face {
            if !face_vertices.contains(&vertex_key) { continue; }
            
            let face_normal = self.face_normal(face_key)?;
            let weight = match weighting {
                NormalWeighting::Area => self.face_area(face_key).unwrap_or(0.0),
                NormalWeighting::Angle => self.vertex_angle_in_face(vertex_key, face_key).unwrap_or(0.0),
                NormalWeighting::Uniform => 1.0,
            };
            
            normal_acc.x += face_normal.x * weight;
            normal_acc.y += face_normal.y * weight;
            normal_acc.z += face_normal.z * weight;
            weight_sum += weight;
        }
        
        if weight_sum > 1e-10 {
            let len = (normal_acc.x * normal_acc.x + normal_acc.y * normal_acc.y + normal_acc.z * normal_acc.z).sqrt();
            if len > 1e-10 {
                Some(Vector::new(normal_acc.x / len, normal_acc.y / len, normal_acc.z / len))
            } else {
                Some(Vector::new(0.0, 0.0, 1.0))
            }
        } else {
            Some(Vector::new(0.0, 0.0, 1.0))
        }
    }

    /// Get weighted normals for all vertices
    pub fn vertex_normals_weighted(&self, weighting: NormalWeighting) -> std::collections::HashMap<usize, crate::Vector> {
        let mut normals = std::collections::HashMap::new();
        for (&vertex_key, _) in &self.vertex {
            if let Some(normal) = self.vertex_normal_weighted(vertex_key, weighting) {
                normals.insert(vertex_key, normal);
            }
        }
        normals
    }

    /// Get neighboring vertices for a given vertex
    pub fn vertex_neighbors(&self, vertex_key: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        for (_, face_vertices) in &self.face {
            if let Some(pos) = face_vertices.iter().position(|&v| v == vertex_key) {
                let len = face_vertices.len();
                let prev = if pos == 0 { len - 1 } else { pos - 1 };
                let next = (pos + 1) % len;
                
                if !neighbors.contains(&face_vertices[prev]) {
                    neighbors.push(face_vertices[prev]);
                }
                if !neighbors.contains(&face_vertices[next]) {
                    neighbors.push(face_vertices[next]);
                }
            }
        }
        neighbors
    }

    /// Get faces adjacent to a given vertex
    pub fn vertex_faces(&self, vertex_key: usize) -> Vec<usize> {
        let mut faces = Vec::new();
        for (&face_key, face_vertices) in &self.face {
            if face_vertices.contains(&vertex_key) {
                faces.push(face_key);
            }
        }
        faces
    }

    /// Get angle of vertex in a specific face
    pub fn vertex_angle_in_face(&self, vertex_key: usize, face_key: usize) -> Option<f32> {
        use crate::Vector;
        
        let face_vertices = self.face.get(&face_key)?;
        let vertex_index = face_vertices.iter().position(|&v| v == vertex_key)?;
        
        if face_vertices.len() < 3 { return Some(0.0); }
        
        let prev_index = if vertex_index == 0 { face_vertices.len() - 1 } else { vertex_index - 1 };
        let next_index = (vertex_index + 1) % face_vertices.len();
        
        let v_curr = self.vertex.get(&face_vertices[vertex_index])?.position();
        let v_prev = self.vertex.get(&face_vertices[prev_index])?.position();
        let v_next = self.vertex.get(&face_vertices[next_index])?.position();
        
        let edge1 = Vector::new(v_prev.x - v_curr.x, v_prev.y - v_curr.y, v_prev.z - v_curr.z);
        let edge2 = Vector::new(v_next.x - v_curr.x, v_next.y - v_curr.y, v_next.z - v_curr.z);
        
        let len1 = edge1.length();
        let len2 = edge2.length();
        
        if len1 > 1e-10 && len2 > 1e-10 {
            let dot = (edge1.x * edge2.x + edge1.y * edge2.y + edge1.z * edge2.z) / (len1 * len2);
            Some(dot.clamp(-1.0, 1.0).acos())
        } else {
            Some(0.0)
        }
    }

    /// Get cached triangulation for a face, computing if needed
    pub fn get_face_triangulation(&mut self, face_key: usize) -> Option<&Vec<[usize; 3]>> {
        self.triangulate_face(face_key)
    }
    
    /// Clear triangulation cache (call when mesh topology changes)
    pub fn clear_triangulation_cache(&mut self) {
        self.triangulation.clear();
    }
    
    /// Get all triangulated faces for rendering. Returns iterator of (face_key, triangles).
    pub fn get_all_triangulations(&mut self) -> impl Iterator<Item = (usize, &Vec<[usize; 3]>)> {
        // Ensure all faces are triangulated
        let face_keys: Vec<usize> = self.face.keys().copied().collect();
        for face_key in face_keys {
            self.triangulate_face(face_key);
        }
        
        self.triangulation.iter().map(|(&k, v)| (k, v))
    }

}

    impl Mesh {
        /// Load a mesh from a JSON file using the legacy JSON format:
        /// { "cube": { "polygons": [ { "vertices": [{"x":...,"y":...,"z":...}, ...] }, ... ] } }
        /// Additionally supports a root-level { "polygons": [...] } with the same structure.
        pub fn from_json_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
            let content = std::fs::read_to_string(file_path)?;
            let json_data: serde_json::Value = serde_json::from_str(&content)?;

            // Helper to parse polygons array into Vec<Vec<Point>>
            fn parse_polygons(polygons_data: &serde_json::Value) -> Option<Vec<Vec<Point>>> {
                let mut polygons: Vec<Vec<Point>> = Vec::new();
                if let Some(polygon_array) = polygons_data.as_array() {
                    for polygon_data in polygon_array {
                        if let Some(vertices_data) = polygon_data.get("vertices") {
                            if let Some(vertex_array) = vertices_data.as_array() {
                                let mut polygon = Vec::new();
                                for vertex_data in vertex_array {
                                    if let (Some(x), Some(y), Some(z)) = (
                                        vertex_data.get("x").and_then(|v| v.as_f64()),
                                        vertex_data.get("y").and_then(|v| v.as_f64()),
                                        vertex_data.get("z").and_then(|v| v.as_f64()),
                                    ) {
                                        polygon.push(Point::new(x as f32, y as f32, z as f32));
                                    }
                                }
                                if !polygon.is_empty() {
                                    polygons.push(polygon);
                                }
                            }
                        }
                    }
                    return Some(polygons);
                }
                None
            }

            // Try nested under "cube"
            if let Some(cube) = json_data.get("cube") {
                if let Some(polygons_data) = cube.get("polygons") {
                    if let Some(polygons) = parse_polygons(polygons_data) {
                        return Ok(Mesh::from_polygons_with_merge(polygons, None));
                    }
                }
            }

            // Try root-level "polygons"
            if let Some(polygons_data) = json_data.get("polygons") {
                if let Some(polygons) = parse_polygons(polygons_data) {
                    return Ok(Mesh::from_polygons_with_merge(polygons, None));
                }
            }

            Err("Invalid JSON format or missing polygons".into())
    }
}

/// Project a 3D polygon to 2D for triangulation.
#[allow(dead_code)]
fn project_polygon_to_2d(polygon: &[Point]) -> Vec<[f32; 2]> {
    use crate::Vector;
    
    if polygon.len() < 3 {
        return Vec::new();
    }
    
    // Calculate polygon normal using Newell's method for robustness
    let mut normal = Vector::new(0.0, 0.0, 0.0);
    for i in 0..polygon.len() {
        let current = &polygon[i];
        let next = &polygon[(i + 1) % polygon.len()];
        
        normal.x += (current.y - next.y) * (current.z + next.z);
        normal.y += (current.z - next.z) * (current.x + next.x);
        normal.z += (current.x - next.x) * (current.y + next.y);
    }
    
    // Normalize the normal vector
    let length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    if length > 1e-10 {
        normal.x /= length;
        normal.y /= length;
        normal.z /= length;
    } else {
        // Degenerate polygon, use XY plane
        normal = Vector::new(0.0, 0.0, 1.0);
    }
    
    // Choose the projection plane based on the largest component of the normal
    let abs_x = normal.x.abs();
    let abs_y = normal.y.abs();
    let abs_z = normal.z.abs();
    
    let points_2d: Vec<[f32; 2]> = if abs_z >= abs_x && abs_z >= abs_y {
        // Project to XY plane (drop Z)
        polygon.iter().map(|p| [p.x, p.y]).collect()
    } else if abs_y >= abs_x && abs_y >= abs_z {
        // Project to XZ plane (drop Y)
        polygon.iter().map(|p| [p.x, p.z]).collect()
    } else {
        // Project to YZ plane (drop X)
        polygon.iter().map(|p| [p.y, p.z]).collect()
    };
    
    points_2d
}

/// Triangulate a polygon using the ear clipping algorithm
#[allow(dead_code)]
fn earclip_triangulate(points: &[[f32; 2]]) -> Result<Vec<[usize; 3]>, &'static str> {
    if points.len() < 3 {
        return Err("Polygon must have at least 3 vertices");
    }
    
    if points.len() == 3 {
        return Ok(vec![[0, 1, 2]]);
    }
    
    // Check winding order and reverse if clockwise
    let mut polygon_points = points.to_vec();
    let signed_area = compute_signed_area(&polygon_points);
    let was_reversed = signed_area > 0.0;
    
    if was_reversed {
        polygon_points.reverse();
    }
    
    // Simple ear clipping implementation
    let mut triangles = Vec::new();
    let mut indices: Vec<usize> = (0..polygon_points.len()).collect();
    
    while indices.len() > 3 {
        let mut ear_found = false;
        
        for i in 0..indices.len() {
            let prev_idx = if i == 0 { indices.len() - 1 } else { i - 1 };
            let next_idx = (i + 1) % indices.len();
            
            let prev = indices[prev_idx];
            let curr = indices[i];
            let next = indices[next_idx];
            
            // Check if this forms a valid ear
            if is_ear(&polygon_points, &indices, prev, curr, next) {
                // Add triangle
                triangles.push([prev, curr, next]);
                
                // Remove the ear vertex
                indices.remove(i);
                ear_found = true;
                break;
            }
        }
        
        if !ear_found {
            return Err("Unable to find valid ear for triangulation");
        }
    }
    
    // Add the final triangle
    if indices.len() == 3 {
        triangles.push([indices[0], indices[1], indices[2]]);
    }
    
    // If we reversed the points, adjust triangle indices back
    if was_reversed {
        let n = points.len();
        for triangle in &mut triangles {
            triangle[0] = n - 1 - triangle[0];
            triangle[1] = n - 1 - triangle[1];
            triangle[2] = n - 1 - triangle[2];
        }
    }
    
    Ok(triangles)
}

/// Check if three consecutive vertices form a valid ear
#[allow(dead_code)]
fn is_ear(points: &[[f32; 2]], indices: &[usize], prev: usize, curr: usize, next: usize) -> bool {
    let a = points[prev];
    let b = points[curr];
    let c = points[next];
    
    // Check if the angle at curr is convex (less than 180 degrees)
    let ab = [b[0] - a[0], b[1] - a[1]];
    let bc = [c[0] - b[0], c[1] - b[1]];
    let cross = ab[0] * bc[1] - ab[1] * bc[0];
    
    if cross <= 0.0 {
        return false; // Not convex
    }
    
    // Check if any other vertex lies inside the triangle
    for &idx in indices {
        if idx != prev && idx != curr && idx != next {
            if point_in_triangle(points[idx], a, b, c) {
                return false;
            }
        }
    }
    
    true
}

/// Check if a point is inside a triangle using barycentric coordinates
#[allow(dead_code)]
fn point_in_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);
    
    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
    
    !(has_neg && has_pos)
}

/// Helper function for point-in-triangle test
#[allow(dead_code)]
fn sign(p1: [f32; 2], p2: [f32; 2], p3: [f32; 2]) -> f32 {
    (p1[0] - p3[0]) * (p2[1] - p3[1]) - (p2[0] - p3[0]) * (p1[1] - p3[1])
}

/// Compute the signed area of a 2D polygon
#[allow(dead_code)]
fn compute_signed_area(points: &[[f32; 2]]) -> f32 {
    let mut sum = 0.0;
    let n = points.len();
    
    for i in 0..n {
        let p0 = points[i];
        let p1 = points[(i + 1) % n];
        sum += (p1[0] - p0[0]) * (p1[1] + p0[1]);
    }
    
    sum * 0.5
}

/// Compute Newell's normal for a 3D polygon. Returns a (nx, ny, nz) tuple.
fn newell_normal(points: &[Point]) -> (f32, f32, f32) {
    if points.len() < 3 { return (0.0, 0.0, 1.0); }
    let mut nx = 0.0;
    let mut ny = 0.0;
    let mut nz = 0.0;
    for i in 0..points.len() {
        let p = &points[i];
        let q = &points[(i + 1) % points.len()];
        nx += (p.y - q.y) * (p.z + q.z);
        ny += (p.z - q.z) * (p.x + q.x);
        nz += (p.x - q.x) * (p.y + q.y);
    }
    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len > 0.0 { (nx / len, ny / len, nz / len) } else { (0.0, 0.0, 1.0) }
}

/// Compute standard signed area of a 2D polygon (CCW positive).
fn signed_area_2d(points: &[[f32; 2]]) -> f32 {
    let n = points.len();
    if n < 3 { return 0.0; }
    let mut sum = 0.0;
    for i in 0..n {
        let p0 = points[i];
        let p1 = points[(i + 1) % n];
        sum += p0[0] * p1[1] - p1[0] * p0[1];
    }
    0.5 * sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Line, Xform};

    #[test]
    fn test_halfedge_mesh_new() {
        let mesh = Mesh::new();
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
        assert!(mesh.is_empty());
        assert_eq!(mesh.euler(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut mesh = Mesh::new();
        let vertex_key = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
        assert_eq!(mesh.number_of_vertices(), 1);
        assert!(!mesh.is_empty());
        
        let pos = mesh.vertex_position(vertex_key).unwrap();
        assert_eq!(pos.x, 1.0);
        assert_eq!(pos.y, 2.0);
        assert_eq!(pos.z, 3.0);
    }

    #[test]
    fn test_add_vertex_with_specific_key() {
        let mut mesh = Mesh::new();
        let vertex_key = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Some(42));
        assert_eq!(vertex_key, 42);
        assert_eq!(mesh.number_of_vertices(), 1);
    }

    #[test]
    fn test_add_face() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _face_key = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        assert_eq!(mesh.number_of_faces(), 1);
        assert_eq!(mesh.number_of_edges(), 3);
        assert_eq!(mesh.euler(), 1); // V=3, E=3, F=1 -> 3-3+1=1
    }

    #[test]
    fn test_add_face_invalid() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        
        // Too few vertices
        assert!(mesh.add_face(vec![v0, v1], None).is_none());
        
        // Non-existent vertex
        assert!(mesh.add_face(vec![v0, v1, 999], None).is_none());
        
        // Duplicate vertices
        assert!(mesh.add_face(vec![v0, v1, v0], None).is_none());
    }

    #[test]
    fn test_face_vertices() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let vertices = mesh.face_vertices(f).unwrap();
        assert_eq!(vertices, &vec![v0, v1, v2]);
    }

    #[test]
    fn test_vertex_neighbors() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        mesh.add_face(vec![v0, v1, v2], None);
        
        let neighbors = mesh.vertex_neighbors(v0);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&v1));
        assert!(neighbors.contains(&v2));
    }

    #[test]
    fn test_vertex_faces() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);
        
        let f1 = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let f2 = mesh.add_face(vec![v1, v3, v2], None).unwrap();
        
        let faces = mesh.vertex_faces(v1);
        assert_eq!(faces.len(), 2);
        assert!(faces.contains(&f1));
        assert!(faces.contains(&f2));
    }

    #[test]
    fn test_is_vertex_on_boundary() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        mesh.add_face(vec![v0, v1, v2], None);
        
        // All vertices of a single triangle are on boundary
        assert!(mesh.is_vertex_on_boundary(v0));
        assert!(mesh.is_vertex_on_boundary(v1));
        assert!(mesh.is_vertex_on_boundary(v2));
    }

    #[test]
    fn test_face_normal() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.face_normal(f).unwrap();
        
        // Normal should point in +Z direction for this triangle
        assert!((normal.z - 1.0).abs() < 1e-10);
        assert!(normal.x.abs() < 1e-10);
        assert!(normal.y.abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normal() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.vertex_normal(v0).unwrap();
        
        // Normal should point in +Z direction
        assert!((normal.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_face_area() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let area = mesh.face_area(f).unwrap();
        
        // Area of triangle with base=1, height=1 should be 0.5
        assert!((area - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_face_normals() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.face_normals();
        
        assert_eq!(normals.len(), 1);
        assert!(normals.contains_key(&f));
        let normal = normals.get(&f).unwrap();
        assert!((normal.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normals() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.vertex_normals();
        
        assert_eq!(normals.len(), 3);
        assert!(normals.contains_key(&v0));
        assert!(normals.contains_key(&v1));
        assert!(normals.contains_key(&v2));
    }
    
    #[test]
    fn test_vertex_normal_weighted_area() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.vertex_normal_weighted(v0, NormalWeighting::Area).unwrap();
        
        // Should be the same as the default vertex_normal method
        let normal_default = mesh.vertex_normal(v0).unwrap();
        assert!((normal.x - normal_default.x).abs() < 1e-10);
        assert!((normal.y - normal_default.y).abs() < 1e-10);
        assert!((normal.z - normal_default.z).abs() < 1e-10);
    }
    
    #[test]
    fn test_vertex_normal_weighted_angle() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.vertex_normal_weighted(v0, NormalWeighting::Angle).unwrap();
        
        // For a single triangle, angle weighting should give same direction as area
        // Normal should point in +Z direction
        assert!((normal.z - 1.0).abs() < 1e-10);
        assert!(normal.x.abs() < 1e-10);
        assert!(normal.y.abs() < 1e-10);
    }
    
    #[test]
    fn test_vertex_normal_weighted_uniform() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.vertex_normal_weighted(v0, NormalWeighting::Uniform).unwrap();
        
        // For a single triangle, uniform weighting should give same direction
        // Normal should point in +Z direction
        assert!((normal.z - 1.0).abs() < 1e-10);
        assert!(normal.x.abs() < 1e-10);
        assert!(normal.y.abs() < 1e-10);
    }
    
    #[test]
    fn test_vertex_normals_weighted() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.vertex_normals_weighted(NormalWeighting::Angle);
        
        assert_eq!(normals.len(), 3);
        assert!(normals.contains_key(&v0));
        assert!(normals.contains_key(&v1));
        assert!(normals.contains_key(&v2));
        
        // All vertex normals should point in +Z direction
        let normal_v0 = normals.get(&v0).unwrap();
        assert!((normal_v0.z - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_vertex_angle_in_face() {
        let mut mesh = Mesh::new();
        // Create a right triangle
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        
        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        
        // Angle at v0 should be 90 degrees (π/2 radians)
        let angle = mesh.vertex_angle_in_face(v0, f).unwrap();
        assert!((angle - std::f32::consts::PI / 2.0).abs() < 1e-6);
        
        // Angles at v1 and v2 should be 45 degrees (π/4 radians) each
        let angle1 = mesh.vertex_angle_in_face(v1, f).unwrap();
        let angle2 = mesh.vertex_angle_in_face(v2, f).unwrap();
        assert!((angle1 - std::f32::consts::PI / 4.0).abs() < 1e-6);
        assert!((angle2 - std::f32::consts::PI / 4.0).abs() < 1e-6);
        
        // Sum of angles should be π
        let total_angle = angle + angle1 + angle2;
        assert!((total_angle - std::f32::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn test_from_polygons_simple() {
        let triangle = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![triangle], None);
        assert_eq!(mesh.number_of_vertices(), 3);
        assert_eq!(mesh.number_of_faces(), 1);
        assert_eq!(mesh.number_of_edges(), 3);
    }

    #[test]
    fn test_from_polygons_vertex_merging() {
        let triangle1 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangle2 = vec![
            Point::new(1.0, 0.0, 0.0), // Shared vertex
            Point::new(0.0, 1.0, 0.0), // Shared vertex
            Point::new(1.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![triangle1, triangle2], None);
        assert_eq!(mesh.number_of_vertices(), 4); // Should merge shared vertices
        assert_eq!(mesh.number_of_faces(), 2);
    }

    #[test]
    fn test_from_polygons_precision() {
        let triangle1 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangle2 = vec![
            Point::new(1.0000001, 0.0, 0.0), // Very close to (1,0,0)
            Point::new(0.0, 1.0000001, 0.0), // Very close to (0,1,0)
            Point::new(1.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![triangle1, triangle2], Some(1e-6));
        assert_eq!(mesh.number_of_vertices(), 4); // Should merge vertices within precision
        assert_eq!(mesh.number_of_faces(), 2);
    }

    #[test]
    fn test_from_polygons_invalid_polygons() {
        let invalid_polygon = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0), // Only 2 points
        ];
        let valid_triangle = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        
        let mesh = Mesh::from_polygons(vec![invalid_polygon, valid_triangle], None);
        assert_eq!(mesh.number_of_vertices(), 3); // Only valid triangle should be added
        assert_eq!(mesh.number_of_faces(), 1);
    }

    #[test]
    fn test_from_polygons_cube() {
        // Create a cube using 6 faces
        let faces = vec![
            // Bottom face (z=0)
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
            ],
            // Top face (z=1)
            vec![
                Point::new(0.0, 0.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 0.0, 1.0),
            ],
            // Front face (y=0)
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 1.0),
                Point::new(1.0, 0.0, 1.0),
                Point::new(1.0, 0.0, 0.0),
            ],
            // Back face (y=1)
            vec![
                Point::new(0.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
            ],
            // Left face (x=0)
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(0.0, 0.0, 1.0),
            ],
            // Right face (x=1)
            vec![
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 1.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 0.0),
            ],
        ];
        
        let mesh = Mesh::from_polygons(faces, None);
        assert_eq!(mesh.number_of_vertices(), 8); // A cube has 8 vertices
        assert_eq!(mesh.number_of_faces(), 12);   // 6 quad faces become 12 triangular faces after triangulation
        assert_eq!(mesh.number_of_edges(), 18);   // Updated edge count for triangulated mesh
        assert_eq!(mesh.euler(), 2);             // Euler characteristic: V-E+F = 8-18+12 = 2
    }

    #[test]
    fn test_clear() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![v0, v1, v2], None);
        
        assert!(!mesh.is_empty());
        mesh.clear();
        assert!(mesh.is_empty());
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
    }
}

/// Implementation of DataObject trait for Mesh to support COMPAS-style JSON serialization
impl Mesh {
    /// Get the type identifier for polymorphic deserialization
    pub fn dtype(&self) -> &'static str {
        "Mesh"
    }
    
    /// Get the object's mesh data for serialization
    pub fn geometric_data(&self) -> serde_json::Value {
        // Convert vertex data to serializable format
        let vertices: HashMap<String, serde_json::Value> = self.vertex.iter()
            .map(|(k, v)| (k.to_string(), serde_json::json!({
                "x": v.position().x,
                "y": v.position().y,
                "z": v.position().z,
                "attributes": v.attributes
            })))
            .collect();
        
        // Convert face data to serializable format
        let faces: HashMap<String, Vec<usize>> = self.face.iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        
        // Convert halfedge data to serializable format
        let halfedges: HashMap<String, serde_json::Value> = self.halfedge.iter()
            .map(|(k, v)| (k.to_string(), serde_json::json!(v)))
            .collect();
        
        serde_json::json!({
            "vertex": vertices,
            "face": faces,
            "halfedge": halfedges,
            "facedata": self.facedata,
            "edgedata": self.edgedata,
            "default_vertex_attributes": self.default_vertex_attributes,
            "default_face_attributes": self.default_face_attributes,
            "default_edge_attributes": self.default_edge_attributes,
            "max_vertex": self.max_vertex,
            "max_face": self.max_face
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
    
    /// Set the name of the mesh
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn to_json_data(&self, minimal: bool) -> serde_json::Value {
        let mut json_obj = self.geometric_data();
        json_obj["dtype"] = serde_json::Value::String(self.dtype().to_string());
        json_obj["facecolors"] = serde_json::json!(self.facecolors);
        json_obj["linecolors"] = serde_json::json!(self.linecolors);
        json_obj["pointcolors"] = serde_json::json!(self.pointcolors);
        json_obj["width"] = serde_json::Value::from(self.width);
        
        if !minimal {
            json_obj["guid"] = serde_json::Value::String(self.guid.to_string());
            json_obj["name"] = serde_json::Value::String(self.name.clone());
        }
        
        json_obj
    }
}

impl HasJsonData for Mesh {
    fn to_json_data(&self, minimal: bool) -> serde_json::Value {
        let mut json_obj = self.geometric_data();
        json_obj["dtype"] = serde_json::Value::String(self.dtype().to_string());
        json_obj["facecolors"] = serde_json::json!(self.facecolors);
        json_obj["linecolors"] = serde_json::json!(self.linecolors);
        json_obj["pointcolors"] = serde_json::json!(self.pointcolors);
        json_obj["width"] = serde_json::Value::from(self.width);
        
        if !minimal {
            json_obj["guid"] = serde_json::Value::String(self.guid.to_string());
            json_obj["name"] = serde_json::Value::String(self.name.clone());
        }
        
        json_obj
    }
}

impl FromJsonData for Mesh {
    fn from_json_data(data: &serde_json::Value) -> Option<Self> {
        // Helper to extract f32 from JSON
        fn get_f32(obj: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<f32> {
            obj.get(key)?.as_f64().map(|v| v as f32)
        }
        
        // Parse COMPAS-style mesh JSON structure
        let obj = data.as_object()?;
        
        // Parse vertices from COMPAS format
        let mut vertex = HashMap::new();
        if let Some(vertex_obj) = obj.get("vertex").and_then(|v| v.as_object()) {
            for (key, value) in vertex_obj {
                if let Ok(vertex_key) = key.parse::<usize>() {
                    if let Some(vertex_data) = value.as_object() {
                        let x = get_f32(vertex_data, "x")?;
                        let y = get_f32(vertex_data, "y")?;
                        let z = get_f32(vertex_data, "z")?;
                        let point = Point::new(x, y, z);
                        
                        let mut attributes = HashMap::new();
                        if let Some(attr_obj) = vertex_data.get("attributes").and_then(|v| v.as_object()) {
                            for (attr_key, attr_value) in attr_obj {
                                if let Some(val) = attr_value.as_f64() {
                                    attributes.insert(attr_key.clone(), val as f32);
                                }
                            }
                        }
                        
                        vertex.insert(vertex_key, VertexData { position: point, attributes });
                    }
                }
            }
        }
        
        // Parse faces from COMPAS format
        let mut face = HashMap::new();
        if let Some(face_obj) = obj.get("face").and_then(|v| v.as_object()) {
            for (key, value) in face_obj {
                if let Ok(face_key) = key.parse::<usize>() {
                    if let Some(face_array) = value.as_array() {
                        let face_vertices: Vec<usize> = face_array.iter()
                            .filter_map(|v| v.as_u64().map(|u| u as usize))
                            .collect();
                        println!("🔍 Face {}: {:?}", face_key, face_vertices);
                        face.insert(face_key, face_vertices);
                    }
                }
            }
        }
        println!("🔍 Total faces parsed: {}", face.len());
        
        // Parse other fields with defaults
        let guid = obj.get("guid")
            .and_then(|v| v.as_str())
            .and_then(|s| uuid::Uuid::parse_str(s).ok())
            .unwrap_or_else(uuid::Uuid::new_v4);
        
        let name = obj.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let facecolors = obj.get("facecolors")
            .and_then(|v| v.as_array())
            .and_then(|arr| {
                if arr.len() == 4 {
                    Some([
                        arr[0].as_f64().map(|v| v as f32)?,
                        arr[1].as_f64().map(|v| v as f32)?,
                        arr[2].as_f64().map(|v| v as f32)?,
                        arr[3].as_f64().map(|v| v as f32)?,
                    ])
                } else { None }
            })
            .unwrap_or([1.0, 1.0, 1.0, 1.0]);
        
        let linecolors = obj.get("linecolors")
            .and_then(|v| v.as_array())
            .and_then(|arr| {
                if arr.len() == 4 {
                    Some([
                        arr[0].as_f64().map(|v| v as f32)?,
                        arr[1].as_f64().map(|v| v as f32)?,
                        arr[2].as_f64().map(|v| v as f32)?,
                        arr[3].as_f64().map(|v| v as f32)?,
                    ])
                } else { None }
            })
            .unwrap_or([0.0, 0.0, 0.0, 1.0]);
        
        let pointcolors = obj.get("pointcolors")
            .and_then(|v| v.as_array())
            .and_then(|arr| {
                if arr.len() == 4 {
                    Some([
                        arr[0].as_f64().map(|v| v as f32)?,
                        arr[1].as_f64().map(|v| v as f32)?,
                        arr[2].as_f64().map(|v| v as f32)?,
                        arr[3].as_f64().map(|v| v as f32)?,
                    ])
                } else { None }
            })
            .unwrap_or([1.0, 1.0, 1.0, 1.0]);
        
        let width = obj.get("width")
            .and_then(|v| v.as_f64())
            .map(|w| w as f32)
            .unwrap_or(0.1);
        
        // Calculate max keys
        let max_vertex = vertex.keys().max().copied().unwrap_or(0) + 1;
        let max_face = face.keys().max().copied().unwrap_or(0) + 1;
        
        Some(Mesh {
            vertex,
            face,
            halfedge: HashMap::new(),
            facedata: HashMap::new(),
            edgedata: HashMap::new(),
            default_vertex_attributes: HashMap::new(),
            default_face_attributes: HashMap::new(),
            default_edge_attributes: HashMap::new(),
            triangulation: HashMap::new(),
            max_vertex,
            max_face,
            guid,
            name,
            xform: Xform::identity(),
            facecolors,
            linecolors,
            pointcolors,
            width,
        })
    }
}


// Simple automatic serialization/deserialization with derive macros
