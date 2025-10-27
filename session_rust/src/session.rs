use crate::{
    Arrow, BoundingBox, Cylinder, Graph, Line, Mesh, Objects, Plane, Point, PointCloud, Polyline,
    Tolerance, Tree, TreeNode, BVH,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use uuid::Uuid;

/// Enum representing all possible geometry types in a Session.
/// This is equivalent to C++'s std::variant<...> for heterogeneous geometry storage.
#[derive(Debug, Clone)]
pub enum Geometry {
    Arrow(Arrow),
    BoundingBox(BoundingBox),
    Cylinder(Cylinder),
    Line(Line),
    Mesh(Mesh),
    Plane(Plane),
    Point(Point),
    PointCloud(PointCloud),
    Polyline(Polyline),
}

impl Geometry {
    /// Get the GUID of the geometry object
    pub fn guid(&self) -> &str {
        match self {
            Geometry::Arrow(g) => &g.guid,
            Geometry::BoundingBox(g) => &g.guid,
            Geometry::Cylinder(g) => &g.guid,
            Geometry::Line(g) => &g.guid,
            Geometry::Mesh(g) => &g.guid,
            Geometry::Plane(g) => &g.guid,
            Geometry::Point(g) => &g.guid,
            Geometry::PointCloud(g) => &g.guid,
            Geometry::Polyline(g) => &g.guid,
        }
    }
}

/// A Session containing geometry objects with hierarchical and graph structures.
///
/// The Session serves as a container for managing geometry objects (currently Points)
/// along with their relationships through tree and graph data structures. It provides
/// JSON serialization capabilities for cross-language interoperability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Session")]
pub struct Session {
    /// Unique identifier for the session
    pub guid: String,
    /// Human-readable name for the session
    pub name: String,
    /// Collection of geometry objects (Points)
    #[serde(rename = "objects")]
    pub objects: Objects,
    /// Lookup table mapping object GUIDs to geometry objects (fast heterogeneous lookup)
    #[serde(skip)]
    pub lookup: HashMap<String, Geometry>,
    /// Hierarchical tree structure for organizing objects
    #[serde(rename = "tree")]
    pub tree: Tree,
    /// Graph structure for representing object relationships
    #[serde(rename = "graph")]
    pub graph: Graph,
    /// Boundary Volume Hierarchy for spatial collision detection
    #[serde(skip)]
    pub bvh: BVH,
    /// Cached BVH for ray casting (indices map to cached_guids)
    #[serde(skip)]
    pub cached_ray_bvh: Option<BVH>,
    /// Cached GUIDs corresponding to cached_boxes order
    #[serde(skip)]
    pub cached_guids: Vec<String>,
    /// Cached AABBs for ray-casting BVH
    #[serde(skip)]
    pub cached_boxes: Vec<BoundingBox>,
    /// Dirty flag for cached ray BVH
    #[serde(skip)]
    pub bvh_cache_dirty: bool,
}

#[derive(Debug, Clone)]
pub struct RayHit {
    pub guid: String,
    pub point: Point,
    pub distance: f64,
}

impl Default for Session {
    /// Creates a default Session with the name "my_session".
    fn default() -> Self {
        Self::new("my_session")
    }
}

impl Session {
    /// Creates a new Session with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name for the session
    ///
    /// # Returns
    /// A new Session instance with a unique GUID, empty objects collection,
    /// and initialized tree and graph structures.
    pub fn new(name: &str) -> Self {
        let guid = Uuid::new_v4().to_string();
        let objects = Objects::new();
        let lookup = HashMap::new();
        let mut tree = Tree::new(&format!("{name}_tree"));
        let graph = Graph::new(&format!("{name}_graph"));

        // Create empty root node with session name
        let root_node = TreeNode::new(name);
        tree.add(&root_node, None);

        // Create boundary-volume-hierarchy, each time we add object we store inside bvh
        let bvh = BVH::new();

        Self {
            guid,
            name: name.to_string(),
            objects,
            lookup,
            tree,
            graph,
            bvh,
            cached_ray_bvh: None,
            cached_guids: Vec::new(),
            cached_boxes: Vec::new(),
            bvh_cache_dirty: true,
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Session to a JSON string.
    ///
    /// # Returns
    /// A Result containing the JSON string representation of the Session,
    /// or an error if serialization fails.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Use custom serialization to ensure consistent structure with C++/Python
        // Convert graph to use array structure instead of nested objects
        let graph_json: serde_json::Value = serde_json::from_str(&self.graph.jsondump()?)?;

        let json_obj = serde_json::json!({
            "type": "Session",
            "guid": self.guid,
            "name": self.name,
            "objects": self.objects,
            "tree": self.tree,
            "graph": graph_json
        });

        Ok(serde_json::to_string_pretty(&json_obj)?)
    }

    /// Deserializes Session from a JSON string.
    ///
    /// # Arguments
    /// * `json_data` - The JSON string to deserialize
    ///
    /// # Returns
    /// A Result containing the deserialized Session, or an error if parsing fails.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_obj: serde_json::Value = serde_json::from_str(json_data)?;

        // Deserialize components using their custom methods
        let objects: Objects = serde_json::from_value(json_obj["objects"].clone())?;
        let tree: Tree = serde_json::from_value(json_obj["tree"].clone())?;
        // Convert graph JSON value to properly formatted string
        let graph_json_str = serde_json::to_string(&json_obj["graph"])?;
        let graph: Graph = Graph::jsonload(&graph_json_str)?;

        // Rebuild lookup table from all objects
        let mut lookup = HashMap::new();
        for arrow in &objects.arrows {
            lookup.insert(arrow.guid.clone(), Geometry::Arrow(arrow.clone()));
        }
        for bbox in &objects.bboxes {
            lookup.insert(bbox.guid.clone(), Geometry::BoundingBox(bbox.clone()));
        }
        for cylinder in &objects.cylinders {
            lookup.insert(cylinder.guid.clone(), Geometry::Cylinder(cylinder.clone()));
        }
        for line in &objects.lines {
            lookup.insert(line.guid.clone(), Geometry::Line(line.clone()));
        }
        for mesh in &objects.meshes {
            lookup.insert(mesh.guid.clone(), Geometry::Mesh(mesh.clone()));
        }
        for plane in &objects.planes {
            lookup.insert(plane.guid.clone(), Geometry::Plane(plane.clone()));
        }
        for point in &objects.points {
            lookup.insert(point.guid.clone(), Geometry::Point(point.clone()));
        }
        for pointcloud in &objects.pointclouds {
            lookup.insert(
                pointcloud.guid.clone(),
                Geometry::PointCloud(pointcloud.clone()),
            );
        }
        for polyline in &objects.polylines {
            lookup.insert(polyline.guid.clone(), Geometry::Polyline(polyline.clone()));
        }

        let session = Session {
            guid: json_obj["guid"].as_str().unwrap_or("").to_string(),
            name: json_obj["name"]
                .as_str()
                .unwrap_or("my_session")
                .to_string(),
            objects,
            lookup,
            tree,
            graph,
            bvh: BVH::new(),
            cached_ray_bvh: None,
            cached_guids: Vec::new(),
            cached_boxes: Vec::new(),
            bvh_cache_dirty: true,
        };

        Ok(session)
    }

    /// Serializes the Session to a JSON file.
    ///
    /// # Arguments
    /// * `filepath` - The path where the JSON file will be written
    ///
    /// # Returns
    /// A Result indicating success or failure of the file write operation.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes Session from a JSON file.
    ///
    /// # Arguments
    /// * `filepath` - The path to the JSON file to read
    ///
    /// # Returns
    /// A Result containing the deserialized Session, or an error if file reading or parsing fails.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(filepath)?;
        Self::jsonload(&json)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // BVH Collision Detection
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Compute bounding box for a geometry object, inflated by tolerance
    fn compute_bounding_box(geometry: &Geometry) -> BoundingBox {
        let inflate = Tolerance::APPROXIMATION;
        match geometry {
            Geometry::Point(p) => BoundingBox::from_point(p.clone(), inflate),
            Geometry::Line(l) => {
                let points = vec![l.start(), l.end()];
                BoundingBox::from_points(&points, inflate)
            }
            Geometry::Polyline(pl) => BoundingBox::from_points(&pl.points, inflate),
            Geometry::PointCloud(pc) => BoundingBox::from_points(&pc.points, inflate),
            Geometry::Mesh(m) => {
                // Extract vertices from mesh vertex data
                let points: Vec<Point> = m
                    .vertex
                    .values()
                    .map(|v| Point::new(v.x, v.y, v.z))
                    .collect();
                if points.is_empty() {
                    BoundingBox::from_point(Point::new(0.0, 0.0, 0.0), inflate)
                } else {
                    BoundingBox::from_points(&points, inflate)
                }
            }
            Geometry::BoundingBox(bb) => {
                // Inflate existing bounding box
                let mut inflated = bb.clone();
                inflated.half_size = crate::Vector::new(
                    inflated.half_size.x() + inflate,
                    inflated.half_size.y() + inflate,
                    inflated.half_size.z() + inflate,
                );
                inflated
            }
            Geometry::Plane(p) => {
                // Create a bounded box around plane origin (finite, test-safe)
                // Keeping the same semantics as Python/C++ default for now.
                BoundingBox::from_point(p.origin(), inflate * 10.0)
            }
            Geometry::Cylinder(c) => {
                // Compute bounding box from cylinder line endpoints and radius
                let points = vec![c.line.start(), c.line.end()];
                let mut bbox = BoundingBox::from_points(&points, inflate);
                // Inflate by cylinder radius
                let radius = c.radius;
                bbox.half_size = crate::Vector::new(
                    bbox.half_size.x() + radius,
                    bbox.half_size.y() + radius,
                    bbox.half_size.z() + radius,
                );
                bbox
            }
            Geometry::Arrow(a) => {
                // Compute bounding box from arrow line endpoints
                let points = vec![a.line.start(), a.line.end()];
                let mut bbox = BoundingBox::from_points(&points, inflate);
                // Inflate by arrow radius
                let radius = a.radius;
                bbox.half_size = crate::Vector::new(
                    bbox.half_size.x() + radius,
                    bbox.half_size.y() + radius,
                    bbox.half_size.z() + radius,
                );
                bbox
            }
        }
    }

    /// Get all collision pairs using BVH and add them as graph edges.
    ///
    /// Automatically:
    /// - Computes bounding boxes for all objects with tolerance inflation
    /// - Builds/rebuilds the BVH with auto-computed world size
    /// - Detects all collision pairs
    /// - Adds collision edges to the graph
    ///
    /// # Returns
    /// A vector of tuples (guid1, guid2) representing colliding geometry pairs
    pub fn get_collisions(&mut self) -> Vec<(String, String)> {
        // Collect all objects with their bounding boxes and GUIDs
        let mut boxes_with_guids: Vec<(BoundingBox, String)> = Vec::new();

        for (guid, geometry) in &self.lookup {
            let bbox = Self::compute_bounding_box(geometry);
            boxes_with_guids.push((bbox, guid.clone()));
        }

        if boxes_with_guids.is_empty() {
            return Vec::new();
        }

        // Build BVH with GUIDs (auto-computes world size)
        self.bvh.build_with_guids(&boxes_with_guids);

        // Extract just the boxes for collision checking
        let boxes: Vec<BoundingBox> = boxes_with_guids
            .iter()
            .map(|(bbox, _)| bbox.clone())
            .collect();

        // Get collision pairs as GUIDs directly
        let collision_pairs = self.bvh.check_all_collisions_guids(&boxes);

        // Add collision edges to graph
        for (guid1, guid2) in &collision_pairs {
            self.graph.add_edge(guid1, guid2, "bvh_collision");
        }

        collision_pairs
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Ray BVH Cache
    ///////////////////////////////////////////////////////////////////////////////////////////

    fn cache_geometry_aabb(&mut self, guid: &str, geometry: &Geometry) {
        let bbox = Self::compute_bounding_box(geometry);
        self.cached_boxes.push(bbox);
        self.cached_guids.push(guid.to_string());
        self.bvh_cache_dirty = true;
    }

    fn rebuild_ray_bvh_cache(&mut self) {
        if self.cached_boxes.len() != self.lookup.len() {
            self.cached_boxes.clear();
            self.cached_guids.clear();
            self.cached_boxes.reserve(self.lookup.len());
            self.cached_guids.reserve(self.lookup.len());
            for (guid, geometry) in &self.lookup {
                let bbox = Self::compute_bounding_box(geometry);
                self.cached_boxes.push(bbox);
                self.cached_guids.push(guid.clone());
            }
        }
        if !self.cached_boxes.is_empty() {
            let world_size = BVH::compute_world_size(&self.cached_boxes);
            self.cached_ray_bvh = Some(BVH::from_boxes(&self.cached_boxes, world_size));
        } else {
            self.cached_ray_bvh = None;
        }
    }

    fn invalidate_bvh_cache(&mut self) {
        self.bvh_cache_dirty = true;
    }

    pub fn ray_cast(
        &mut self,
        origin: &Point,
        direction: &crate::Vector,
        tolerance: f64,
    ) -> Vec<RayHit> {
        let dir_len = direction.compute_length();
        if dir_len <= 0.0 {
            return Vec::new();
        }
        let dir_unit = crate::Vector::new(
            direction.x() / dir_len,
            direction.y() / dir_len,
            direction.z() / dir_len,
        );

        let far = 1e6f64;
        let ray_end = Point::new(
            origin.x() + dir_unit.x() * far,
            origin.y() + dir_unit.y() * far,
            origin.z() + dir_unit.z() * far,
        );
        let ray_line = Line::from_points(origin, &ray_end);

        // Use cached BVH for ray casting
        if self.bvh_cache_dirty || self.cached_ray_bvh.is_none() {
            self.rebuild_ray_bvh_cache();
            self.bvh_cache_dirty = false;
        }
        let bvh = match &self.cached_ray_bvh {
            Some(b) => b,
            None => return Vec::new(),
        };

        let mut candidates: Vec<usize> = Vec::new();
        bvh.ray_cast(origin, &dir_unit, &mut candidates, true);

        let mut hits_all: Vec<RayHit> = Vec::new();

        for idx in candidates {
            if idx >= self.cached_guids.len() {
                continue;
            }
            let guid = self.cached_guids[idx].clone();
            let geom = match self.lookup.get_mut(&guid) {
                Some(g) => g,
                None => continue,
            };

            let mut hit_point: Option<Point> = None;

            match geom {
                Geometry::BoundingBox(bb) => {
                    if let Some(pts) = crate::intersection::ray_box(&ray_line, bb, 0.0, far) {
                        if !pts.is_empty() {
                            hit_point = Some(pts[0].clone());
                        }
                    }
                }
                Geometry::Plane(pl) => {
                    if let Some(p) = crate::intersection::line_plane(&ray_line, pl, true) {
                        hit_point = Some(p);
                    }
                }
                Geometry::Line(l) => {
                    if let Some(p) =
                        crate::intersection::line_line(&ray_line, l, Tolerance::APPROXIMATION)
                    {
                        hit_point = Some(p);
                    }
                }
                Geometry::Polyline(pl) => {
                    let mut best_t = f64::INFINITY;
                    let mut best_p: Option<Point> = None;
                    if pl.points.len() >= 2 {
                        for i in 0..(pl.points.len() - 1) {
                            let seg = Line::from_points(&pl.points[i], &pl.points[i + 1]);
                            if let Some(p) = crate::intersection::line_line(
                                &ray_line,
                                &seg,
                                Tolerance::APPROXIMATION,
                            ) {
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
                    }
                    if let Some(p) = best_p {
                        hit_point = Some(p);
                    }
                }
                Geometry::Mesh(m) => {
                    if let Some(p) = m.ray_cast_bvh(&ray_line, 1e-6) {
                        hit_point = Some(p);
                    }
                }
                Geometry::Cylinder(cy) => {
                    if let Some(p) = crate::intersection::line_line(
                        &ray_line,
                        &cy.line,
                        Tolerance::APPROXIMATION,
                    ) {
                        hit_point = Some(p);
                    }
                }
                Geometry::Arrow(ar) => {
                    if let Some(p) = crate::intersection::line_line(
                        &ray_line,
                        &ar.line,
                        Tolerance::APPROXIMATION,
                    ) {
                        hit_point = Some(p);
                    }
                }
                Geometry::Point(p) => {
                    let vx = p.x() - origin.x();
                    let vy = p.y() - origin.y();
                    let vz = p.z() - origin.z();
                    let cross_x = vy * dir_unit.z() - vz * dir_unit.y();
                    let cross_y = vz * dir_unit.x() - vx * dir_unit.z();
                    let cross_z = vx * dir_unit.y() - vy * dir_unit.x();
                    let dist = (cross_x * cross_x + cross_y * cross_y + cross_z * cross_z).sqrt();
                    if dist <= tolerance {
                        let t = vx * dir_unit.x() + vy * dir_unit.y() + vz * dir_unit.z();
                        if t >= 0.0 {
                            let hp = Point::new(
                                origin.x() + dir_unit.x() * t,
                                origin.y() + dir_unit.y() * t,
                                origin.z() + dir_unit.z() * t,
                            );
                            hit_point = Some(hp);
                        }
                    }
                }
                Geometry::PointCloud(_) => {}
            }

            if let Some(hp) = hit_point {
                let dx = hp.x() - origin.x();
                let dy = hp.y() - origin.y();
                let dz = hp.z() - origin.z();
                let forward = dx * dir_unit.x() + dy * dir_unit.y() + dz * dir_unit.z();
                if forward >= 0.0 {
                    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
                    hits_all.push(RayHit {
                        guid: guid.clone(),
                        point: hp,
                        distance: dist,
                    });
                }
            }
        }

        if hits_all.is_empty() {
            return Vec::new();
        }

        let mut min_d = f64::INFINITY;
        for h in &hits_all {
            if h.distance < min_d {
                min_d = h.distance;
            }
        }
        let eps = tolerance;
        let mut hits: Vec<RayHit> = hits_all
            .into_iter()
            .filter(|h| (h.distance - min_d).abs() <= eps)
            .collect();
        hits.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Adds a point to the Session.
    ///
    /// The point is added to the objects collection, lookup table, graph as a node,
    /// and tree as a child of the root node.
    ///
    /// # Arguments
    /// * `point` - The Point object to add to the session
    ///
    /// # Returns
    /// The TreeNode created for this point
    pub fn add_point(&mut self, point: Point) -> TreeNode {
        let point_guid = point.guid.clone();
        let point_name = point.name.clone();
        let geometry = Geometry::Point(point.clone());

        self.objects.points.push(point);
        self.lookup.insert(point_guid.clone(), geometry);
        if let Some(Geometry::Point(p)) = self.lookup.get(&point_guid) {
            self.cache_geometry_aabb(&point_guid, &Geometry::Point(p.clone()));
        }
        self.graph
            .add_node(&point_guid, &format!("point_{point_name}"));

        TreeNode::new(&point_guid)
    }

    pub fn add_line(&mut self, line: Line) -> TreeNode {
        let guid = line.guid.clone();
        let name = line.name.clone();
        let geometry = Geometry::Line(line.clone());

        self.objects.lines.push(line);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::Line(l)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::Line(l.clone()));
        }
        self.graph.add_node(&guid, &format!("line_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_plane(&mut self, plane: Plane) -> TreeNode {
        let guid = plane.guid.clone();
        let name = plane.name.clone();
        let geometry = Geometry::Plane(plane.clone());

        self.objects.planes.push(plane);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::Plane(p)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::Plane(p.clone()));
        }
        self.graph.add_node(&guid, &format!("plane_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_bbox(&mut self, bbox: BoundingBox) -> TreeNode {
        let guid = bbox.guid.clone();
        let name = bbox.name.clone();
        let geometry = Geometry::BoundingBox(bbox.clone());

        self.objects.bboxes.push(bbox);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::BoundingBox(b)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::BoundingBox(b.clone()));
        }
        self.graph.add_node(&guid, &format!("bbox_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_polyline(&mut self, polyline: Polyline) -> TreeNode {
        let guid = polyline.guid.clone();
        let name = polyline.name.clone();
        let geometry = Geometry::Polyline(polyline.clone());

        self.objects.polylines.push(polyline);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::Polyline(p)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::Polyline(p.clone()));
        }
        self.graph.add_node(&guid, &format!("polyline_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_pointcloud(&mut self, pointcloud: PointCloud) -> TreeNode {
        let guid = pointcloud.guid.clone();
        let name = pointcloud.name.clone();
        let geometry = Geometry::PointCloud(pointcloud.clone());

        self.objects.pointclouds.push(pointcloud);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::PointCloud(p)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::PointCloud(p.clone()));
        }
        self.graph.add_node(&guid, &format!("pointcloud_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> TreeNode {
        let guid = mesh.guid.clone();
        let name = mesh.name.clone();
        let geometry = Geometry::Mesh(mesh.clone());

        self.objects.meshes.push(mesh);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::Mesh(m)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::Mesh(m.clone()));
        }
        self.graph.add_node(&guid, &format!("mesh_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_cylinder(&mut self, cylinder: Cylinder) -> TreeNode {
        let guid = cylinder.guid.clone();
        let name = cylinder.name.clone();
        let geometry = Geometry::Cylinder(cylinder.clone());

        self.objects.cylinders.push(cylinder);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::Cylinder(c)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::Cylinder(c.clone()));
        }
        self.graph.add_node(&guid, &format!("cylinder_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_arrow(&mut self, arrow: Arrow) -> TreeNode {
        let guid = arrow.guid.clone();
        let name = arrow.name.clone();
        let geometry = Geometry::Arrow(arrow.clone());

        self.objects.arrows.push(arrow);
        self.lookup.insert(guid.clone(), geometry);
        if let Some(Geometry::Arrow(a)) = self.lookup.get(&guid) {
            self.cache_geometry_aabb(&guid, &Geometry::Arrow(a.clone()));
        }
        self.graph.add_node(&guid, &format!("arrow_{name}"));

        TreeNode::new(&guid)
    }

    /// Adds a TreeNode to the tree hierarchy.
    ///
    /// # Arguments
    /// * `node` - The TreeNode to add
    /// * `parent` - Optional parent TreeNode (defaults to root if None)
    pub fn add<'a>(&mut self, node: &TreeNode, parent: impl Into<Option<&'a TreeNode>>)
    where
        TreeNode: 'a,
    {
        let parent_opt = parent.into();
        if parent_opt.is_none() {
            if let Some(root) = self.tree.root() {
                self.tree.add(node, Some(&root));
            }
        } else {
            self.tree.add(node, parent_opt);
        }
    }

    /// Adds an edge between two geometry objects in the graph.
    ///
    /// # Arguments
    /// * `from_guid` - The GUID of the source object
    /// * `to_guid` - The GUID of the target object
    /// * `attribute` - The attribute or label for the edge
    pub fn add_edge(&mut self, from_guid: &str, to_guid: &str, attribute: &str) {
        self.graph.add_edge(from_guid, to_guid, attribute);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details - Lookup
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Gets a geometry object by its GUID.
    ///
    /// # Arguments
    /// * `guid` - The GUID of the object to retrieve
    ///
    /// # Returns
    /// An Option containing a reference to the Geometry enum if found, or None if not found.
    pub fn get_object(&self, guid: &str) -> Option<&Geometry> {
        self.lookup.get(guid)
    }

    /// Remove a geometry object by its GUID.
    ///
    /// # Arguments
    /// * `guid` - The UUID of the geometry object to remove.
    ///
    /// # Returns
    /// `true` if the object was removed, `false` if not found.
    pub fn remove_object(&mut self, guid: &str) -> bool {
        // Check if object exists in lookup table
        if !self.lookup.contains_key(guid) {
            return false;
        }

        // Remove from all object collections
        self.objects.points.retain(|p| p.guid != guid);
        self.objects.lines.retain(|l| l.guid != guid);
        self.objects.polylines.retain(|p| p.guid != guid);
        self.objects.planes.retain(|p| p.guid != guid);
        self.objects.bboxes.retain(|b| b.guid != guid);
        self.objects.meshes.retain(|m| m.guid != guid);
        self.objects.cylinders.retain(|c| c.guid != guid);
        self.objects.arrows.retain(|a| a.guid != guid);
        self.objects.pointclouds.retain(|p| p.guid != guid);

        // Remove from lookup table
        self.lookup.remove(guid);
        self.invalidate_bvh_cache();

        // Remove from tree - find node by GUID and remove it
        if let Some(node) = self.tree.find_node_by_guid(&guid.to_string()) {
            self.tree.remove(&node);
        }

        // Remove from graph using string GUID
        if self.graph.has_node(guid) {
            self.graph.remove_node(guid);
        }

        true
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details - Tree
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Add a parent-child relationship in the tree structure.
    ///
    /// # Arguments
    /// * `parent_guid` - The GUID of the parent geometry object.
    /// * `child_guid` - The GUID of the child geometry object.
    ///
    /// # Returns
    /// `true` if the relationship was added successfully.
    pub fn add_hierarchy(&mut self, parent_guid: &str, child_guid: &str) -> bool {
        self.tree
            .add_child_by_guid(&parent_guid.to_string(), &child_guid.to_string())
    }

    /// Get all children GUIDs of a geometry object in the tree.
    ///
    /// # Arguments
    /// * `guid` - The GUID of the geometry object.
    ///
    /// # Returns
    /// A vector containing the GUIDs of all children of the specified geometry object.
    pub fn get_children(&self, guid: &str) -> Vec<String> {
        self.tree.get_children(guid)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details - Graph
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Add a relationship edge in the graph structure.
    ///
    /// # Arguments
    /// * `from_guid` - The GUID of the source geometry object.
    /// * `to_guid` - The GUID of the target geometry object.
    /// * `relationship_type` - The type of relationship.
    pub fn add_relationship(&mut self, from_guid: &str, to_guid: &str, relationship_type: &str) {
        self.graph.add_edge(from_guid, to_guid, relationship_type);
    }

    /// Get all GUIDs connected to the given GUID in the graph.
    ///
    /// # Arguments
    /// * `guid` - The GUID of the geometry object.
    ///
    /// # Returns
    /// A vector containing the GUIDs of all connected geometry objects.
    pub fn get_neighbours(&self, guid: &str) -> Vec<String> {
        self.graph.get_neighbors(guid)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details - Transformed Geometry
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Get all geometry with transformations applied from tree hierarchy.
    ///
    /// Recursively traverses the tree and applies parent transformations to children.
    /// Each child's transformation is the composition of all ancestor transformations
    /// multiplied by its own transformation.
    ///
    /// # Returns
    /// Objects collection with transformed geometry
    pub fn get_geometry(&self) -> Objects {
        use crate::Xform;

        // Deep copy all objects
        let mut transformed_objects = self.objects.clone();

        // Rebuild lookup from copied objects
        let mut transformed_lookup: HashMap<String, Geometry> = HashMap::new();

        for point in &transformed_objects.points {
            transformed_lookup.insert(point.guid.clone(), Geometry::Point(point.clone()));
        }
        for line in &transformed_objects.lines {
            transformed_lookup.insert(line.guid.clone(), Geometry::Line(line.clone()));
        }
        for plane in &transformed_objects.planes {
            transformed_lookup.insert(plane.guid.clone(), Geometry::Plane(plane.clone()));
        }
        for bbox in &transformed_objects.bboxes {
            transformed_lookup.insert(bbox.guid.clone(), Geometry::BoundingBox(bbox.clone()));
        }
        for polyline in &transformed_objects.polylines {
            transformed_lookup.insert(polyline.guid.clone(), Geometry::Polyline(polyline.clone()));
        }
        for pointcloud in &transformed_objects.pointclouds {
            transformed_lookup.insert(
                pointcloud.guid.clone(),
                Geometry::PointCloud(pointcloud.clone()),
            );
        }
        for mesh in &transformed_objects.meshes {
            transformed_lookup.insert(mesh.guid.clone(), Geometry::Mesh(mesh.clone()));
        }
        for cylinder in &transformed_objects.cylinders {
            transformed_lookup.insert(cylinder.guid.clone(), Geometry::Cylinder(cylinder.clone()));
        }
        for arrow in &transformed_objects.arrows {
            transformed_lookup.insert(arrow.guid.clone(), Geometry::Arrow(arrow.clone()));
        }

        fn transform_node(
            node: &TreeNode,
            parent_xform: &Xform,
            transformed_lookup: &HashMap<String, Geometry>,
            transformed_objects: &mut Objects,
        ) {
            // Get geometry from the lookup
            let node_name = node.name();
            let geometry = transformed_lookup.get(&node_name);

            let current_xform = if let Some(geom) = geometry {
                // Get mutable reference and transform in-place
                let combined_xform = parent_xform
                    * match geom {
                        Geometry::Point(g) => &g.xform,
                        Geometry::Line(g) => &g.xform,
                        Geometry::Plane(g) => &g.xform,
                        Geometry::BoundingBox(g) => &g.xform,
                        Geometry::Polyline(g) => &g.xform,
                        Geometry::PointCloud(g) => &g.xform,
                        Geometry::Mesh(g) => &g.xform,
                        Geometry::Cylinder(g) => &g.xform,
                        Geometry::Arrow(g) => &g.xform,
                    };

                // Find and update the geometry in the collections
                match geom {
                    Geometry::Point(_) => {
                        if let Some(g) = transformed_objects
                            .points
                            .iter_mut()
                            .find(|p| p.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::Line(_) => {
                        if let Some(g) = transformed_objects
                            .lines
                            .iter_mut()
                            .find(|l| l.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::Plane(_) => {
                        if let Some(g) = transformed_objects
                            .planes
                            .iter_mut()
                            .find(|p| p.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::BoundingBox(_) => {
                        if let Some(g) = transformed_objects
                            .bboxes
                            .iter_mut()
                            .find(|b| b.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::Polyline(_) => {
                        if let Some(g) = transformed_objects
                            .polylines
                            .iter_mut()
                            .find(|p| p.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::PointCloud(_) => {
                        if let Some(g) = transformed_objects
                            .pointclouds
                            .iter_mut()
                            .find(|p| p.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::Mesh(_) => {
                        if let Some(g) = transformed_objects
                            .meshes
                            .iter_mut()
                            .find(|m| m.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::Cylinder(_) => {
                        if let Some(g) = transformed_objects
                            .cylinders
                            .iter_mut()
                            .find(|c| c.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                    Geometry::Arrow(_) => {
                        if let Some(g) = transformed_objects
                            .arrows
                            .iter_mut()
                            .find(|a| a.guid == node_name)
                        {
                            g.xform = combined_xform.clone();
                        }
                    }
                }

                combined_xform
            } else {
                parent_xform.clone()
            };

            for child in node.children() {
                transform_node(
                    &child,
                    &current_xform,
                    transformed_lookup,
                    transformed_objects,
                );
            }
        }

        if let Some(root) = self.tree.root() {
            transform_node(
                &root,
                &Xform::identity(),
                &transformed_lookup,
                &mut transformed_objects,
            );
        }

        // Apply accumulated transformations to actual geometry coordinates
        for point in &mut transformed_objects.points {
            point.transform();
        }
        for line in &mut transformed_objects.lines {
            line.transform();
        }
        for plane in &mut transformed_objects.planes {
            plane.transform();
        }
        for bbox in &mut transformed_objects.bboxes {
            bbox.transform();
        }
        for polyline in &mut transformed_objects.polylines {
            polyline.transform();
        }
        for pointcloud in &mut transformed_objects.pointclouds {
            pointcloud.transform();
        }
        for mesh in &mut transformed_objects.meshes {
            mesh.transform();
        }
        for cylinder in &mut transformed_objects.cylinders {
            cylinder.transform();
        }
        for arrow in &mut transformed_objects.arrows {
            arrow.transform();
        }

        transformed_objects
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Session({}, {}, points={}, vertices={}, edges={})",
            self.name,
            self.guid,
            self.objects.points.len(),
            self.graph.vertex_count,
            self.graph.edge_count
        )
    }
}

#[cfg(test)]
#[path = "session_test.rs"]
mod session_test;
