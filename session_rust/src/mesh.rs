use crate::{BoundingBox, Color, Line, Point, Tolerance, Vector, Xform, BVH};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Weighting scheme for vertex normal computation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalWeighting {
    Area,
    Angle,
    Uniform,
}

/// A halfedge mesh data structure for representing polygonal surfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Mesh")]
pub struct Mesh {
    pub halfedge: HashMap<usize, HashMap<usize, Option<usize>>>, // Halfedge connectivity
    pub vertex: HashMap<usize, VertexData>,                      // Vertex data
    pub face: HashMap<usize, Vec<usize>>,                        // Face vertex lists
    pub facedata: HashMap<usize, HashMap<String, f64>>,          // Face attributes
    pub edgedata: HashMap<(usize, usize), HashMap<String, f64>>, // Edge attributes
    pub default_vertex_attributes: HashMap<String, f64>,         // Default vertex attrs
    pub default_face_attributes: HashMap<String, f64>,           // Default face attrs
    pub default_edge_attributes: HashMap<String, f64>,           // Default edge attrs
    #[serde(skip)]
    pub triangulation: HashMap<usize, Vec<[usize; 3]>>, // Cached triangulations
    max_vertex: usize,                                           // Next vertex key
    max_face: usize,                                             // Next face key
    pub guid: String,                                            // Unique identifier
    pub name: String,                                            // Mesh name
    #[serde(skip)]
    pub pointcolors: Vec<Color>,               // Vertex colors
    #[serde(skip)]
    pub facecolors: Vec<Color>,                // Face colors
    #[serde(skip)]
    pub linecolors: Vec<Color>,                // Edge colors
    #[serde(skip)]
    pub widths: Vec<f64>,                      // Edge widths
    #[serde(default = "Xform::identity")]
    pub xform: Xform,   // Transformation matrix
    // Cached triangle BVH for ray queries (not serialized)
    #[serde(skip)]
    pub tri_bvh: Option<BVH>,
    #[serde(skip)]
    pub tri_tris: Vec<[usize; 3]>,
    #[serde(skip)]
    pub tri_vertices: Vec<Point>,
}

/// Vertex data containing position and attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    pub x: f64,                           // X coordinate
    pub y: f64,                           // Y coordinate
    pub z: f64,                           // Z coordinate
    pub attributes: HashMap<String, f64>, // Vertex attributes
}

impl VertexData {
    pub fn new(point: Point) -> Self {
        Self {
            x: point.x(),
            y: point.y(),
            z: point.z(),
            attributes: HashMap::new(),
        }
    }

    pub fn position(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }

    pub fn set_position(&mut self, point: Point) {
        self.x = point.x();
        self.y = point.y();
        self.z = point.z();
    }

    pub fn color(&self) -> [f64; 3] {
        [
            self.attributes.get("r").copied().unwrap_or(0.5),
            self.attributes.get("g").copied().unwrap_or(0.5),
            self.attributes.get("b").copied().unwrap_or(0.5),
        ]
    }

    pub fn set_color(&mut self, r: f64, g: f64, b: f64) {
        self.attributes.insert("r".to_string(), r);
        self.attributes.insert("g".to_string(), g);
        self.attributes.insert("b".to_string(), b);
    }

    pub fn normal(&self) -> Option<[f64; 3]> {
        let nx = self.attributes.get("nx")?;
        let ny = self.attributes.get("ny")?;
        let nz = self.attributes.get("nz")?;
        Some([*nx, *ny, *nz])
    }

    pub fn set_normal(&mut self, nx: f64, ny: f64, nz: f64) {
        self.attributes.insert("nx".to_string(), nx);
        self.attributes.insert("ny".to_string(), ny);
        self.attributes.insert("nz".to_string(), nz);
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    /// Creates a new empty halfedge mesh
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
            guid: uuid::Uuid::new_v4().to_string(),
            name: "my_mesh".to_string(),
            pointcolors: Vec::new(),
            facecolors: Vec::new(),
            linecolors: Vec::new(),
            widths: Vec::new(),
            xform: Xform::identity(),
            tri_bvh: None,
            tri_tris: Vec::new(),
            tri_vertices: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertex.is_empty() && self.face.is_empty()
    }

    pub fn clear(&mut self) {
        self.halfedge.clear();
        self.vertex.clear();
        self.face.clear();
        self.facedata.clear();
        self.edgedata.clear();
        self.triangulation.clear();
        self.max_vertex = 0;
        self.max_face = 0;
        self.pointcolors.clear();
        self.facecolors.clear();
        self.linecolors.clear();
        self.widths.clear();
        self.invalidate_triangle_bvh();
    }

    pub fn number_of_vertices(&self) -> usize {
        self.vertex.len()
    }

    pub fn number_of_faces(&self) -> usize {
        self.face.len()
    }

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

    pub fn euler(&self) -> i32 {
        let v = self.number_of_vertices() as i32;
        let e = self.number_of_edges() as i32;
        let f = self.number_of_faces() as i32;
        v - e + f
    }

    pub fn add_vertex(&mut self, position: Point, key: Option<usize>) -> usize {
        let vertex_key = key.unwrap_or_else(|| {
            self.max_vertex += 1;
            self.max_vertex
        });

        if vertex_key >= self.max_vertex {
            self.max_vertex = vertex_key + 1;
        }

        let vertex_data = VertexData::new(position);
        self.vertex.insert(vertex_key, vertex_data);
        self.halfedge.entry(vertex_key).or_default();
        self.pointcolors.push(Color::white());
        self.invalidate_triangle_bvh();

        vertex_key
    }

    pub fn add_face(&mut self, vertices: Vec<usize>, fkey: Option<usize>) -> Option<usize> {
        if vertices.len() < 3 {
            return None;
        }

        if !vertices.iter().all(|v| self.vertex.contains_key(v)) {
            return None;
        }

        let mut unique_vertices = HashSet::new();
        for vertex in &vertices {
            if !unique_vertices.insert(*vertex) {
                return None;
            }
        }

        let face_key = fkey.unwrap_or_else(|| {
            self.max_face += 1;
            self.max_face
        });

        if face_key >= self.max_face {
            self.max_face = face_key + 1;
        }

        self.face.insert(face_key, vertices.clone());
        self.triangulation.remove(&face_key);
        self.facecolors.push(Color::white());
        self.invalidate_triangle_bvh();

        for i in 0..vertices.len() {
            let u = vertices[i];
            let v = vertices[(i + 1) % vertices.len()];

            self.halfedge.entry(u).or_default();
            self.halfedge.entry(v).or_default();

            let is_new_edge = !self.halfedge.get(&v).unwrap().contains_key(&u);

            self.halfedge.get_mut(&u).unwrap().insert(v, Some(face_key));

            if is_new_edge {
                self.halfedge.get_mut(&v).unwrap().insert(u, None);
                self.linecolors.push(Color::white());
                self.widths.push(1.0);
            }
        }

        Some(face_key)
    }

    pub fn vertex_position(&self, vertex_key: usize) -> Option<Point> {
        self.vertex.get(&vertex_key).map(|v| v.position())
    }

    pub fn face_vertices(&self, face_key: usize) -> Option<&Vec<usize>> {
        self.face.get(&face_key)
    }

    pub fn vertex_neighbors(&self, vertex_key: usize) -> Vec<usize> {
        self.halfedge
            .get(&vertex_key)
            .map(|neighbors| neighbors.keys().copied().collect())
            .unwrap_or_default()
    }

    pub fn vertex_faces(&self, vertex_key: usize) -> Vec<usize> {
        let mut faces = Vec::new();
        for (face_key, face_vertices) in &self.face {
            if face_vertices.contains(&vertex_key) {
                faces.push(*face_key);
            }
        }
        faces
    }

    pub fn is_vertex_on_boundary(&self, vertex_key: usize) -> bool {
        if let Some(neigh) = self.halfedge.get(&vertex_key) {
            for (_v, face_opt) in neigh.iter() {
                if face_opt.is_none() {
                    return true;
                }
            }
        }

        for (_u, neigh) in self.halfedge.iter() {
            if let Some(face_opt) = neigh.get(&vertex_key) {
                if face_opt.is_none() {
                    return true;
                }
            }
        }
        false
    }

    pub fn face_normal(&self, face_key: usize) -> Option<Vector> {
        let vertices = self.face.get(&face_key)?;
        if vertices.len() < 3 {
            return None;
        }

        let p0 = self.vertex_position(vertices[0])?;
        let p1 = self.vertex_position(vertices[1])?;
        let p2 = self.vertex_position(vertices[2])?;

        let u = Vector::new(p1.x() - p0.x(), p1.y() - p0.y(), p1.z() - p0.z());
        let v = Vector::new(p2.x() - p0.x(), p2.y() - p0.y(), p2.z() - p0.z());

        let mut normal = u.cross(&v);
        let len = normal.magnitude();
        if len > Tolerance::ZERO_TOLERANCE {
            Some(Vector::new(
                normal.x() / len,
                normal.y() / len,
                normal.z() / len,
            ))
        } else {
            None
        }
    }

    pub fn vertex_normal(&self, vertex_key: usize) -> Option<Vector> {
        self.vertex_normal_weighted(vertex_key, NormalWeighting::Area)
    }

    pub fn vertex_normal_weighted(
        &self,
        vertex_key: usize,
        weighting: NormalWeighting,
    ) -> Option<Vector> {
        let faces = self.vertex_faces(vertex_key);
        if faces.is_empty() {
            return None;
        }

        let mut normal_acc = Vector::new(0.0, 0.0, 0.0);

        for face_key in faces {
            if let Some(face_normal) = self.face_normal(face_key) {
                let weight = match weighting {
                    NormalWeighting::Area => self.face_area(face_key).unwrap_or(1.0),
                    NormalWeighting::Angle => self
                        .vertex_angle_in_face(vertex_key, face_key)
                        .unwrap_or(1.0),
                    NormalWeighting::Uniform => 1.0,
                };

                normal_acc.set_x(normal_acc.x() + face_normal.x() * weight);
                normal_acc.set_y(normal_acc.y() + face_normal.y() * weight);
                normal_acc.set_z(normal_acc.z() + face_normal.z() * weight);
            }
        }

        let len = normal_acc.magnitude();
        if len > Tolerance::ZERO_TOLERANCE {
            Some(Vector::new(
                normal_acc.x() / len,
                normal_acc.y() / len,
                normal_acc.z() / len,
            ))
        } else {
            None
        }
    }

    pub fn face_area(&self, face_key: usize) -> Option<f64> {
        let vertices = self.face.get(&face_key)?;
        if vertices.len() < 3 {
            return Some(0.0);
        }

        let mut area = 0.0;
        let p0 = self.vertex_position(vertices[0])?;

        for i in 1..(vertices.len() - 1) {
            let p1 = self.vertex_position(vertices[i])?;
            let p2 = self.vertex_position(vertices[i + 1])?;

            let u = Vector::new(p1.x() - p0.x(), p1.y() - p0.y(), p1.z() - p0.z());
            let v = Vector::new(p2.x() - p0.x(), p2.y() - p0.y(), p2.z() - p0.z());

            area += u.cross(&v).magnitude() * 0.5;
        }

        Some(area)
    }

    pub fn vertex_angle_in_face(&self, vertex_key: usize, face_key: usize) -> Option<f64> {
        let vertices = self.face.get(&face_key)?;
        let vertex_index = vertices.iter().position(|&v| v == vertex_key)?;

        let n = vertices.len();
        let prev_vertex = vertices[(vertex_index + n - 1) % n];
        let next_vertex = vertices[(vertex_index + 1) % n];

        let center = self.vertex_position(vertex_key)?;
        let prev_pos = self.vertex_position(prev_vertex)?;
        let next_pos = self.vertex_position(next_vertex)?;

        let mut u = Vector::new(
            prev_pos.x() - center.x(),
            prev_pos.y() - center.y(),
            prev_pos.z() - center.z(),
        );
        let mut v = Vector::new(
            next_pos.x() - center.x(),
            next_pos.y() - center.y(),
            next_pos.z() - center.z(),
        );

        let u_len = u.magnitude();
        let v_len = v.magnitude();

        if u_len < Tolerance::ZERO_TOLERANCE || v_len < Tolerance::ZERO_TOLERANCE {
            return Some(0.0);
        }

        let cos_angle = u.dot(&v) / (u_len * v_len);
        let cos_angle = cos_angle.clamp(-1.0, 1.0);
        Some(cos_angle.acos())
    }

    pub fn face_normals(&self) -> HashMap<usize, Vector> {
        let mut normals = HashMap::new();
        for face_key in self.face.keys() {
            if let Some(normal) = self.face_normal(*face_key) {
                normals.insert(*face_key, normal);
            }
        }
        normals
    }

    pub fn vertex_normals(&self) -> HashMap<usize, Vector> {
        self.vertex_normals_weighted(NormalWeighting::Area)
    }

    pub fn vertex_normals_weighted(&self, weighting: NormalWeighting) -> HashMap<usize, Vector> {
        let mut normals = HashMap::new();
        for vertex_key in self.vertex.keys() {
            if let Some(normal) = self.vertex_normal_weighted(*vertex_key, weighting) {
                normals.insert(*vertex_key, normal);
            }
        }
        normals
    }

    pub fn vertex_index(&self) -> HashMap<usize, usize> {
        let mut keys: Vec<usize> = self.vertex.keys().copied().collect();
        keys.sort();
        keys.iter()
            .enumerate()
            .map(|(index, &key)| (key, index))
            .collect()
    }

    pub fn to_vertices_and_faces(&self) -> (Vec<Point>, Vec<Vec<usize>>) {
        let vertex_index = self.vertex_index();
        let mut vertices: Vec<Point> = vec![Point::default(); self.vertex.len()];

        for (&key, data) in &self.vertex {
            let idx = vertex_index[&key];
            vertices[idx] = data.position();
        }

        // Sort face keys to ensure consistent ordering
        let mut face_keys: Vec<usize> = self.face.keys().copied().collect();
        face_keys.sort();

        let mut faces = Vec::new();
        for face_key in face_keys {
            let face_vertices = &self.face[&face_key];
            let remapped: Vec<usize> = face_vertices.iter().map(|v| vertex_index[v]).collect();
            faces.push(remapped);
        }

        (vertices, faces)
    }

    pub fn from_polygons(polygons: Vec<Vec<Point>>, precision: Option<f64>) -> Self {
        let mut mesh = Mesh::new();
        let mut map_eps: HashMap<(i64, i64, i64), usize> = HashMap::new();
        let mut map_exact: HashMap<(u64, u64, u64), usize> = HashMap::new();
        let eps = precision.unwrap_or(0.0);
        let use_eps = eps > 0.0;

        let mut get_vkey = |p: &Point, mesh: &mut Mesh| -> usize {
            if use_eps {
                let kx = (p.x() / eps).round() as i64;
                let ky = (p.y() / eps).round() as i64;
                let kz = (p.z() / eps).round() as i64;
                let key = (kx, ky, kz);
                if let Some(&vk) = map_eps.get(&key) {
                    return vk;
                }
                let vk = mesh.add_vertex(p.clone(), None);
                map_eps.insert(key, vk);
                vk
            } else {
                let key = (p.x().to_bits(), p.y().to_bits(), p.z().to_bits());
                if let Some(&vk) = map_exact.get(&key) {
                    return vk;
                }
                let vk = mesh.add_vertex(p.clone(), None);
                map_exact.insert(key, vk);
                vk
            }
        };

        for poly in polygons.into_iter() {
            if poly.len() < 3 {
                continue;
            }
            let mut vkeys: Vec<usize> = Vec::with_capacity(poly.len());
            for p in &poly {
                let vk = get_vkey(p, &mut mesh);
                vkeys.push(vk);
            }
            let _ = mesh.add_face(vkeys, None);
        }

        mesh
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Triangle BVH cache and ray casting
    ///////////////////////////////////////////////////////////////////////////////////////////

    fn invalidate_triangle_bvh(&mut self) {
        self.tri_bvh = None;
        self.tri_tris.clear();
        self.tri_vertices.clear();
    }

    fn ensure_triangle_bvh(&mut self) {
        if self.tri_bvh.is_some() && !self.tri_tris.is_empty() && !self.tri_vertices.is_empty() {
            return;
        }

        let (vertices, faces) = self.to_vertices_and_faces();
        let mut tris: Vec<[usize; 3]> = Vec::new();
        let mut tri_boxes: Vec<BoundingBox> = Vec::new();

        for face in faces {
            if face.len() < 3 {
                continue;
            }
            let v0 = face[0];
            for i in 1..(face.len() - 1) {
                let t = [v0, face[i], face[i + 1]];
                tris.push(t);
                let pts = [
                    vertices[t[0]].clone(),
                    vertices[t[1]].clone(),
                    vertices[t[2]].clone(),
                ];
                tri_boxes.push(BoundingBox::from_points(&pts, 0.0));
            }
        }

        if tris.is_empty() {
            self.tri_bvh = None;
            self.tri_tris.clear();
            self.tri_vertices = vertices; // keep for consistency
            return;
        }

        let world_size = BVH::compute_world_size(&tri_boxes);
        let bvh = BVH::from_boxes(&tri_boxes, world_size);
        self.tri_vertices = vertices;
        self.tri_tris = tris;
        self.tri_bvh = Some(bvh);
    }

    pub fn ray_cast_bvh(&mut self, ray: &Line, epsilon: f64) -> Option<Point> {
        self.ensure_triangle_bvh();
        let bvh = match &self.tri_bvh {
            Some(b) => b,
            None => return None,
        };

        let origin = ray.start();
        let dir = ray.to_vector();
        let len = dir.compute_length();
        if len <= Tolerance::ZERO_TOLERANCE {
            return None;
        }
        let dir_unit = Vector::new(dir.x() / len, dir.y() / len, dir.z() / len);

        let mut candidate_ids: Vec<usize> = Vec::new();
        bvh.ray_cast(&origin, &dir_unit, &mut candidate_ids, true);
        if candidate_ids.is_empty() {
            return None;
        }

        let mut best_t = f64::INFINITY;
        let mut best_p: Option<Point> = None;

        for idx in candidate_ids {
            if idx >= self.tri_tris.len() {
                continue;
            }
            let tri = self.tri_tris[idx];
            let v0 = &self.tri_vertices[tri[0]];
            let v1 = &self.tri_vertices[tri[1]];
            let v2 = &self.tri_vertices[tri[2]];
            if let Some(p) = crate::intersection::ray_triangle(ray, v0, v1, v2, epsilon) {
                let dx = p.x() - origin.x();
                let dy = p.y() - origin.y();
                let dz = p.z() - origin.z();
                let t = dx * dir_unit.x() + dy * dir_unit.y() + dz * dir_unit.z();
                if t >= 0.0 && t < best_t {
                    best_t = t;
                    best_p = Some(p);
                }
            }
        }

        best_p
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Color and Width Management
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn set_vertex_color(&mut self, index: usize, color: Color) {
        if index < self.pointcolors.len() {
            self.pointcolors[index] = color;
        }
    }

    pub fn set_face_color(&mut self, index: usize, color: Color) {
        if index < self.facecolors.len() {
            self.facecolors[index] = color;
        }
    }

    pub fn set_edge_color(&mut self, index: usize, color: Color) {
        if index < self.linecolors.len() {
            self.linecolors[index] = color;
        }
    }

    pub fn set_edge_width(&mut self, index: usize, width: f64) {
        if index < self.widths.len() {
            self.widths[index] = width;
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn transform(&mut self) {
        let xform = self.xform.clone();
        for v in self.vertex.values_mut() {
            let mut pt = Point::new(v.x, v.y, v.z);
            xform.transform_point(&mut pt);
            v.x = pt.x();
            v.y = pt.y();
            v.z = pt.z();
        }
        self.xform = Xform::identity();
        self.invalidate_triangle_bvh();
    }

    pub fn transformed(&self) -> Self {
        let mut result = self.clone();
        result.transform();
        result
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Mesh to JSON data
    pub fn jsondump(&self) -> serde_json::Value {
        let pointcolors_flat: Vec<u8> = self
            .pointcolors
            .iter()
            .flat_map(|c| vec![c.r, c.g, c.b])
            .collect();

        let facecolors_flat: Vec<u8> = self
            .facecolors
            .iter()
            .flat_map(|c| vec![c.r, c.g, c.b])
            .collect();

        let linecolors_flat: Vec<u8> = self
            .linecolors
            .iter()
            .flat_map(|c| vec![c.r, c.g, c.b])
            .collect();

        serde_json::json!({
            "type": "Mesh",
            "guid": self.guid,
            "name": self.name,
            "vertex": self.vertex,
            "face": self.face,
            "halfedge": self.halfedge,
            "facedata": self.facedata,
            "edgedata": self.edgedata,
            "default_vertex_attributes": self.default_vertex_attributes,
            "default_face_attributes": self.default_face_attributes,
            "default_edge_attributes": self.default_edge_attributes,
            "max_vertex": self.max_vertex,
            "max_face": self.max_face,
            "pointcolors": pointcolors_flat,
            "facecolors": facecolors_flat,
            "linecolors": linecolors_flat,
            "widths": self.widths
        })
    }

    pub fn jsonload(data: &serde_json::Value) -> Option<Self> {
        let mut mesh = Mesh::new();

        if let Some(guid) = data.get("guid").and_then(|v| v.as_str()) {
            mesh.guid = guid.to_string();
        }
        if let Some(name) = data.get("name").and_then(|v| v.as_str()) {
            mesh.name = name.to_string();
        }

        if let Some(vertex_data) = data.get("vertex") {
            mesh.vertex = serde_json::from_value(vertex_data.clone()).ok()?;
        }
        if let Some(face_data) = data.get("face") {
            mesh.face = serde_json::from_value(face_data.clone()).ok()?;
        }
        if let Some(halfedge_data) = data.get("halfedge") {
            mesh.halfedge = serde_json::from_value(halfedge_data.clone()).ok()?;
        }
        if let Some(facedata) = data.get("facedata") {
            mesh.facedata = serde_json::from_value(facedata.clone()).ok()?;
        }
        if let Some(edgedata) = data.get("edgedata") {
            mesh.edgedata = serde_json::from_value(edgedata.clone()).ok()?;
        }
        if let Some(max_vertex) = data.get("max_vertex").and_then(|v| v.as_u64()) {
            mesh.max_vertex = max_vertex as usize;
        }
        if let Some(max_face) = data.get("max_face").and_then(|v| v.as_u64()) {
            mesh.max_face = max_face as usize;
        }

        // Deserialize flat color arrays
        if let Some(pointcolors_flat) = data.get("pointcolors").and_then(|v| v.as_array()) {
            let rgb_values: Vec<u8> = pointcolors_flat
                .iter()
                .filter_map(|v| v.as_u64().map(|n| n as u8))
                .collect();
            mesh.pointcolors = rgb_values
                .chunks(3)
                .map(|chunk| {
                    if chunk.len() == 3 {
                        Color::new(chunk[0], chunk[1], chunk[2], 255)
                    } else {
                        Color::white()
                    }
                })
                .collect();
        }

        if let Some(facecolors_flat) = data.get("facecolors").and_then(|v| v.as_array()) {
            let rgb_values: Vec<u8> = facecolors_flat
                .iter()
                .filter_map(|v| v.as_u64().map(|n| n as u8))
                .collect();
            mesh.facecolors = rgb_values
                .chunks(3)
                .map(|chunk| {
                    if chunk.len() == 3 {
                        Color::new(chunk[0], chunk[1], chunk[2], 255)
                    } else {
                        Color::white()
                    }
                })
                .collect();
        }

        if let Some(linecolors_flat) = data.get("linecolors").and_then(|v| v.as_array()) {
            let rgb_values: Vec<u8> = linecolors_flat
                .iter()
                .filter_map(|v| v.as_u64().map(|n| n as u8))
                .collect();
            mesh.linecolors = rgb_values
                .chunks(3)
                .map(|chunk| {
                    if chunk.len() == 3 {
                        Color::new(chunk[0], chunk[1], chunk[2], 255)
                    } else {
                        Color::white()
                    }
                })
                .collect();
        }

        if let Some(widths) = data.get("widths").and_then(|v| v.as_array()) {
            mesh.widths = widths.iter().filter_map(|v| v.as_f64()).collect();
        }

        Some(mesh)
    }

    pub fn to_json(&self, filename: &str) -> std::io::Result<()> {
        let data = self.jsondump();
        std::fs::write(filename, serde_json::to_string_pretty(&data)?)
    }

    pub fn from_json(filename: &str) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(filename)?;
        let data: serde_json::Value = serde_json::from_str(&content)?;
        Self::jsonload(&data).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid mesh data")
        })
    }
}

#[cfg(test)]
#[path = "mesh_test.rs"]
mod mesh_test;
