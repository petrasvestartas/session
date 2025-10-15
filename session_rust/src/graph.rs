use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

/// A graph vertex with a unique identifier and attribute string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Vertex")]
pub struct Vertex {
    /// The unique identifier of the vertex.
    pub guid: String,
    /// The name of the vertex.
    pub name: String,
    /// Vertex attribute data as string.
    pub attribute: String,
    /// Integer index for the vertex. Set internally by Graph.
    pub index: i32,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            name: "my_vertex".to_string(),
            guid: Uuid::new_v4().to_string(),
            attribute: String::new(),
            index: -1,
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Graph({}, {}, vertices={}, edges={})",
            self.name, self.guid, self.vertex_count, self.edge_count
        )
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vertex({}, {}, attr={}, index={})",
            self.name, self.guid, self.attribute, self.index
        )
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Edge({}, {}, {} -> {}, attr={}, index={})",
            self.name, self.guid, self.v0, self.v1, self.attribute, self.index
        )
    }
}

impl Vertex {
    /// Initialize a new Vertex.
    pub fn new(name: Option<String>, attribute: Option<String>) -> Self {
        Self {
            name: name.unwrap_or_default(),
            attribute: attribute.unwrap_or_default(),
            ..Default::default()
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert the Vertex to a JSON-serializable string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Create Vertex from JSON string data.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }
}

/// A graph edge with a unique identifier and attribute string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Edge")]
pub struct Edge {
    /// The unique identifier of the edge.
    pub guid: String,
    /// The name of the edge.
    pub name: String,
    /// The first vertex of the edge.
    pub v0: String,
    /// The second vertex of the edge.
    pub v1: String,
    /// Edge attribute data as string.
    pub attribute: String,
    /// Integer index for the edge.
    pub index: i32,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            name: "my_edge".to_string(),
            guid: Uuid::new_v4().to_string(),
            v0: String::new(),
            v1: String::new(),
            attribute: String::new(),
            index: -1,
        }
    }
}

impl Edge {
    /// Initialize a new Edge.
    pub fn new(
        name: Option<String>,
        v0: Option<String>,
        v1: Option<String>,
        attribute: Option<String>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            v0: v0.unwrap_or_default(),
            v1: v1.unwrap_or_default(),
            attribute: attribute.unwrap_or_default(),
            ..Default::default()
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert the Edge to a JSON-serializable string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Create Edge from JSON string data.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Get the edge vertices as a tuple.
    pub fn vertices(&self) -> (String, String) {
        (self.v0.clone(), self.v1.clone())
    }

    /// Check if this edge connects to a given vertex.
    pub fn connects(&self, vertex_id: &str) -> bool {
        self.v0 == vertex_id || self.v1 == vertex_id
    }

    /// Get the other vertex ID connected by this edge.
    pub fn other_vertex(&self, vertex_id: &str) -> String {
        if self.v0 == vertex_id {
            self.v1.clone()
        } else {
            self.v0.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Graph")]
pub struct Graph {
    // Public fields, similar to C++
    pub guid: String,
    pub name: String,
    pub vertex_count: i32,
    pub edge_count: i32,

    // Private fields (by Rust's default visibility)
    // std::map translates to HashMap in Rust.
    vertices: HashMap<String, Vertex>,
    pub edges: HashMap<String, HashMap<String, Edge>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_graph".to_string(),
            vertex_count: 0,
            edge_count: 0,
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl Graph {
    /// Creates a new, empty `Graph` with a specific name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Checks if a node exists in the graph.
    pub fn has_node(&self, key: &str) -> bool {
        self.vertices.contains_key(key)
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, key: &str, attribute: &str) -> String {
        if self.has_node(key) {
            return self.vertices.get(key).unwrap().name.clone();
        }

        let mut vertex = Vertex::new(Some(key.to_string()), Some(attribute.to_string()));
        vertex.index = self.vertices.len() as i32;
        self.vertices.insert(key.to_string(), vertex.clone());
        self.vertex_count = self.vertices.len() as i32;
        vertex.name
    }

    /// Adds an edge between u and v.
    pub fn add_edge(&mut self, u: &str, v: &str, attribute: &str) -> (String, String) {
        // Add vertices if they don't exist
        if !self.has_node(u) {
            self.add_node(u, "");
        }
        if !self.has_node(v) {
            self.add_node(v, "");
        }

        if self.has_edge((u, v)) {
            return (u.to_string(), v.to_string());
        }

        let mut edge = Edge::new(
            Some("my_edge".to_string()),
            Some(u.to_string()),
            Some(v.to_string()),
            Some(attribute.to_string()),
        );
        edge.index = self.edge_count;

        self.edges
            .entry(u.to_string())
            .or_default()
            .insert(v.to_string(), edge.clone());
        self.edges
            .entry(v.to_string())
            .or_default()
            .insert(u.to_string(), edge);

        self.edge_count += 1;

        (u.to_string(), v.to_string())
    }

    /// Checks if an edge exists in the graph.
    pub fn has_edge(&self, edge: (&str, &str)) -> bool {
        let (u, v) = edge;
        self.edges
            .get(u)
            .is_some_and(|neighbors| neighbors.contains_key(v))
    }

    /// Gets the number of vertices in the graph.
    pub fn number_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Gets the number of edges in the graph.
    pub fn number_of_edges(&self) -> usize {
        let mut count = 0;
        let mut seen = std::collections::HashSet::new();
        for (u, neighbors) in &self.edges {
            for v in neighbors.keys() {
                let edge = if u < v {
                    (u.clone(), v.clone())
                } else {
                    (v.clone(), u.clone())
                };
                if seen.insert(edge) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Gets all vertices in the graph.
    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.values().cloned().collect()
    }

    /// Gets all edges in the graph as tuples of vertex names.
    pub fn get_edges(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for (u, neighbors) in &self.edges {
            for v in neighbors.keys() {
                let edge = if u < v {
                    (u.clone(), v.clone())
                } else {
                    (v.clone(), u.clone())
                };
                if seen.insert(edge.clone()) {
                    result.push(edge);
                }
            }
        }
        result
    }

    /// Gets all neighbors of a node.
    pub fn neighbors(&self, node: &str) -> Vec<String> {
        self.edges
            .get(node)
            .map_or(Vec::new(), |neighbors| neighbors.keys().cloned().collect())
    }

    /// Gets all neighbors of a node (API compatibility method).
    pub fn get_neighbors(&self, node: &str) -> Vec<String> {
        self.neighbors(node)
    }

    /// Removes a node and all its edges from the graph.
    pub fn remove_node(&mut self, key: &str) {
        if !self.has_node(key) {
            return;
        }

        if let Some(neighbors) = self.edges.remove(key) {
            for neighbor_key in neighbors.keys() {
                if let Some(neighbor_edges) = self.edges.get_mut(neighbor_key) {
                    neighbor_edges.remove(key);
                }
            }
        }

        self.vertices.remove(key);
        self.vertex_count = self.vertices.len() as i32;
        self.edge_count = self.number_of_edges() as i32;
    }

    /// Removes an edge from the graph.
    pub fn remove_edge(&mut self, edge: (&str, &str)) {
        let (u, v) = edge;
        let mut edge_removed = false;

        if let Some(neighbors) = self.edges.get_mut(u) {
            if neighbors.remove(v).is_some() {
                edge_removed = true;
            }
        }
        if let Some(neighbors) = self.edges.get_mut(v) {
            neighbors.remove(u);
        }

        if edge_removed {
            self.edge_count = self.number_of_edges() as i32;
        }
    }

    /// Removes all vertices and edges from the graph.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.vertex_count = 0;
        self.edge_count = 0;
    }

    /// Get or set node attribute.
    pub fn node_attribute(&mut self, node: &str, value: Option<&str>) -> Option<String> {
        if !self.has_node(node) {
            return None;
        }
        let vertex = self.vertices.get_mut(node).unwrap();
        if let Some(val) = value {
            vertex.attribute = val.to_string();
            Some(vertex.attribute.clone())
        } else {
            Some(vertex.attribute.clone())
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Graph to a JSON string.
    pub fn jsondump(&self) -> Result<String, serde_json::Error> {
        // Convert vertices to array, sorted by index to ensure consistent order
        let mut vertices: Vec<&Vertex> = self.vertices.values().collect();
        vertices.sort_by_key(|v| v.index);

        // Convert edges to array (store each edge only once), sorted by index
        let mut edges = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for (u, neighbors) in &self.edges {
            for (v, edge) in neighbors {
                let edge_tuple = if u < v { (u, v) } else { (v, u) };
                if seen.insert(edge_tuple) {
                    edges.push(edge);
                }
            }
        }
        edges.sort_by_key(|e| e.index);

        let json_obj = serde_json::json!({
            "type": "Graph",
            "name": self.name,
            "guid": self.guid,
            "vertices": vertices,
            "edges": edges,
            "vertex_count": self.vertex_count,
            "edge_count": self.edge_count
        });

        serde_json::to_string_pretty(&json_obj)
    }

    /// Deserializes a Graph from a JSON string.
    pub fn jsonload(json_data: &str) -> Result<Self, serde_json::Error> {
        let json_obj: serde_json::Value = serde_json::from_str(json_data)?;

        let mut graph = Graph::new(json_obj["name"].as_str().unwrap_or("my_graph"));
        graph.guid = json_obj["guid"].as_str().unwrap_or("").to_string();
        graph.vertex_count = json_obj["vertex_count"].as_i64().unwrap_or(0) as i32;
        graph.edge_count = json_obj["edge_count"].as_i64().unwrap_or(0) as i32;

        // Restore vertices
        if let Some(vertices_array) = json_obj["vertices"].as_array() {
            for vertex_data in vertices_array {
                let vertex: Vertex = serde_json::from_value(vertex_data.clone())?;
                graph.vertices.insert(vertex.name.clone(), vertex);
            }
        }

        // Restore edges
        if let Some(edges_array) = json_obj["edges"].as_array() {
            for edge_data in edges_array {
                let edge: Edge = serde_json::from_value(edge_data.clone())?;
                let u = &edge.v0;
                let v = &edge.v1;

                graph
                    .edges
                    .entry(u.clone())
                    .or_default()
                    .insert(v.clone(), edge.clone());
                graph
                    .edges
                    .entry(v.clone())
                    .or_default()
                    .insert(u.clone(), edge);
            }
        }

        Ok(graph)
    }

    /// Serializes the Graph to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = self.jsondump()?;
        std::fs::write(filepath, json_data)?;
        Ok(())
    }

    /// Deserializes a Graph from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json_data = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json_data).map_err(|e| e.into())
    }

    /// Get or set edge attribute.
    pub fn edge_attribute(&mut self, u: &str, v: &str, value: Option<&str>) -> Option<String> {
        if !self.has_edge((u, v)) {
            return None;
        }
        if let Some(val) = value {
            let new_attr = val.to_string();
            if let Some(neighbors) = self.edges.get_mut(u) {
                if let Some(edge) = neighbors.get_mut(v) {
                    edge.attribute = new_attr.clone();
                }
            }
            if let Some(neighbors) = self.edges.get_mut(v) {
                if let Some(edge) = neighbors.get_mut(u) {
                    edge.attribute = new_attr.clone();
                }
            }
            Some(new_attr)
        } else {
            self.edges
                .get(u)
                .and_then(|neighbors| neighbors.get(v))
                .map(|edge| edge.attribute.clone())
        }
    }
}

#[cfg(test)]
#[path = "graph_test.rs"]
mod graph_test;
