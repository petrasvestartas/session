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
        let inflate = Tolerance::APPROXIMATION as f32;
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
                // Create a bounded box around plane origin
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
        self.graph.add_node(&guid, &format!("line_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_plane(&mut self, plane: Plane) -> TreeNode {
        let guid = plane.guid.clone();
        let name = plane.name.clone();
        let geometry = Geometry::Plane(plane.clone());

        self.objects.planes.push(plane);
        self.lookup.insert(guid.clone(), geometry);
        self.graph.add_node(&guid, &format!("plane_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_bbox(&mut self, bbox: BoundingBox) -> TreeNode {
        let guid = bbox.guid.clone();
        let name = bbox.name.clone();
        let geometry = Geometry::BoundingBox(bbox.clone());

        self.objects.bboxes.push(bbox);
        self.lookup.insert(guid.clone(), geometry);
        self.graph.add_node(&guid, &format!("bbox_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_polyline(&mut self, polyline: Polyline) -> TreeNode {
        let guid = polyline.guid.clone();
        let name = polyline.name.clone();
        let geometry = Geometry::Polyline(polyline.clone());

        self.objects.polylines.push(polyline);
        self.lookup.insert(guid.clone(), geometry);
        self.graph.add_node(&guid, &format!("polyline_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_pointcloud(&mut self, pointcloud: PointCloud) -> TreeNode {
        let guid = pointcloud.guid.clone();
        let name = pointcloud.name.clone();
        let geometry = Geometry::PointCloud(pointcloud.clone());

        self.objects.pointclouds.push(pointcloud);
        self.lookup.insert(guid.clone(), geometry);
        self.graph.add_node(&guid, &format!("pointcloud_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> TreeNode {
        let guid = mesh.guid.clone();
        let name = mesh.name.clone();
        let geometry = Geometry::Mesh(mesh.clone());

        self.objects.meshes.push(mesh);
        self.lookup.insert(guid.clone(), geometry);
        self.graph.add_node(&guid, &format!("mesh_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_cylinder(&mut self, cylinder: Cylinder) -> TreeNode {
        let guid = cylinder.guid.clone();
        let name = cylinder.name.clone();
        let geometry = Geometry::Cylinder(cylinder.clone());

        self.objects.cylinders.push(cylinder);
        self.lookup.insert(guid.clone(), geometry);
        self.graph.add_node(&guid, &format!("cylinder_{name}"));

        TreeNode::new(&guid)
    }

    pub fn add_arrow(&mut self, arrow: Arrow) -> TreeNode {
        let guid = arrow.guid.clone();
        let name = arrow.name.clone();
        let geometry = Geometry::Arrow(arrow.clone());

        self.objects.arrows.push(arrow);
        self.lookup.insert(guid.clone(), geometry);
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
