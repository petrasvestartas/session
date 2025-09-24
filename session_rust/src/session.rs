use crate::{Graph, Objects, Point, Tree, TreeNode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use uuid::Uuid;

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
    /// Lookup table mapping object GUIDs to their types
    #[serde(rename = "lookup")]
    pub lookup: HashMap<String, String>,
    /// Hierarchical tree structure for organizing objects
    #[serde(rename = "tree")]
    pub tree: Tree,
    /// Graph structure for representing object relationships
    #[serde(rename = "graph")]
    pub graph: Graph,
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

        Self {
            guid,
            name: name.to_string(),
            objects,
            lookup,
            tree,
            graph,
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
    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Use custom serialization to ensure consistent structure with C++/Python
        // Convert graph to use array structure instead of nested objects
        let graph_json: serde_json::Value = serde_json::from_str(&self.graph.to_json_data()?)?;

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
    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_obj: serde_json::Value = serde_json::from_str(json_data)?;

        // Deserialize components using their custom methods
        let objects: Objects = serde_json::from_value(json_obj["objects"].clone())?;
        let tree: Tree = serde_json::from_value(json_obj["tree"].clone())?;
        let graph: Graph = Graph::from_json_data(&json_obj["graph"].to_string())?;

        // Rebuild lookup table from objects
        let mut lookup = HashMap::new();
        for point in &objects.vec {
            lookup.insert(point.guid.clone(), "point".to_string());
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
        let json = self.to_json_data()?;
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
        Self::from_json_data(&json)
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
    pub fn add_point(&mut self, point: Point) {
        let point_guid = point.guid.clone();
        let point_name = point.name.clone();

        self.objects.vec.push(point);
        self.lookup.insert(point_guid.clone(), "point".to_string());

        // Automatically add to graph using point's GUID as node key
        self.graph
            .add_node(&point_guid, &format!("point_{}", point_name));

        // Automatically add to tree as child of root using point's GUID as node name
        let tree_node = TreeNode::new(&point_guid);
        if let Some(root) = self.tree.root() {
            self.tree.add(&tree_node, Some(&root));
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
    /// An Option containing a reference to the Point if found, or None if not found.
    pub fn get_object(&self, guid: &str) -> Option<&Point> {
        self.lookup
            .get(guid)
            .and_then(|_t| self.objects.vec.iter().find(|p| p.guid == guid))
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

        // Remove from points collection
        // Note: In Rust, the field is `vec` but serialized as "points" in JSON
        self.objects.vec.retain(|point| point.guid != guid);

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
            self.objects.vec.len(),
            self.graph.vertex_count,
            self.graph.edge_count
        )
    }
}
