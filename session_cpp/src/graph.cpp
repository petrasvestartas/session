#include "graph.h"

namespace session_cpp {

std::string Graph::to_string() const {
  return fmt::format("Graph({}, {}, {}, {})", guid, name, vertex_count,
                     edge_count);
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json Graph::jsondump() const {
  // Convert vertices to JSON array
  nlohmann::ordered_json vertices_json = nlohmann::ordered_json::array();
  for (auto &[vertex_name, vertex] : vertices) {
    vertices_json.push_back(vertex.jsondump());
  }

  // Convert edges to JSON array (store each edge only once)
  nlohmann::ordered_json edges_json = nlohmann::ordered_json::array();
  for (auto &[u, neighbors] : edges) {
    for (auto &[v, edge] : neighbors) {
      if (u < v) { // Only store each edge once
        edges_json.push_back(edge.jsondump());
      }
    }
  }

  return nlohmann::ordered_json{{"type", "Graph"},
                                {"name", name},
                                {"guid", guid},
                                {"vertices", vertices_json},
                                {"edges", edges_json},
                                {"vertex_count", vertex_count},
                                {"edge_count", edge_count}};
}

Graph Graph::jsonload(const nlohmann::json &data) {
  Graph graph(data["name"]);
  graph.guid = data["guid"];
  graph.vertex_count = data["vertex_count"];
  graph.edge_count = data["edge_count"];

  // Restore vertices
  for (const auto &vertex_data : data["vertices"]) {
    Vertex vertex = Vertex::jsonload(vertex_data);
    graph.vertices[vertex.name] = vertex;
  }

  // Restore edges
  for (const auto &edge_data : data["edges"]) {
    Edge edge = Edge::jsonload(edge_data);
    std::string u = edge.v0;
    std::string v = edge.v1;
    if (graph.edges.find(u) == graph.edges.end()) {
      graph.edges[u] = std::map<std::string, Edge>();
    }
    if (graph.edges.find(v) == graph.edges.end()) {
      graph.edges[v] = std::map<std::string, Edge>();
    }
    graph.edges[u][v] = edge;
    graph.edges[v][u] = edge;
  }

  return graph;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

bool Graph::has_node(const std::string &key) {
  return vertices.find(key) != vertices.end();
}
bool Graph::has_edge(const std::tuple<std::string, std::string> &key) {
  auto u_it = edges.find(std::get<0>(key));
  if (u_it == edges.end()) {
    return false;
  }
  return u_it->second.find(std::get<1>(key)) != u_it->second.end();
}

std::string Graph::add_node(const std::string &key,
                            const std::string &attribute) {
  if (this->has_node(key)) {
    return vertices[key].name;
  } else {
    // Create new vertex
    Vertex vertex(key, attribute);
    vertex.index = vertex_count; // Set index internally
    vertices[key] = vertex;
    vertex_count += 1;
    return vertex.name;
  }
}

std::tuple<std::string, std::string>
Graph::add_edge(const std::string &u, const std::string &v,
                const std::string &attribute) {
  // Add vertices if they don't exist
  if (!this->has_node(u)) {
    this->add_node(u);
  }
  if (!this->has_node(v)) {
    this->add_node(v);
  }

  // Add edge (store in both directions for undirected graph)
  Edge edge(u, v, attribute);
  edge.index = edge_count; // Set index internally
  if (edges.find(u) == edges.end()) {
    edges[u] = std::map<std::string, Edge>();
  }
  if (edges.find(v) == edges.end()) {
    edges[v] = std::map<std::string, Edge>();
  }
  edges[u][v] = edge;
  edges[v][u] = edge;
  edge_count += 1;

  return std::make_tuple(u, v);
}

void Graph::remove_node(const std::string &key) {
  if (!this->has_node(key)) {
    throw std::runtime_error("Node " + key + " not in graph");
  }

  // Remove all edges connected to this node
  auto edges_it = edges.find(key);
  if (edges_it != edges.end()) {
    // Get all neighbors and remove this node from their adjacency lists
    for (const auto &[neighbor, edge] : edges_it->second) {
      auto neighbor_it = edges.find(neighbor);
      if (neighbor_it != edges.end()) {
        neighbor_it->second.erase(key);
      }
    }
    // Remove this node's adjacency list
    edges.erase(edges_it);
  }

  // Remove the node itself
  vertices.erase(key);

  // Reassign indices to maintain contiguous sequence
  _reassign_indices();
}

void Graph::remove_edge(const std::tuple<std::string, std::string> &edge) {
  std::string u = std::get<0>(edge);
  std::string v = std::get<1>(edge);

  if (this->has_edge(edge)) {
    // Remove edge from u's adjacency list
    auto u_it = edges.find(u);
    if (u_it != edges.end() && u_it->second.find(v) != u_it->second.end()) {
      u_it->second.erase(v);
    }

    // Remove edge from v's adjacency list
    auto v_it = edges.find(v);
    if (v_it != edges.end() && v_it->second.find(u) != v_it->second.end()) {
      v_it->second.erase(u);
    }

    // Reassign edge indices to maintain contiguous sequence
    _reassign_edge_indices();
  }
}

void Graph::_reassign_indices() {
  std::vector<Vertex *> vertex_list;
  // Collect all vertices
  for (auto &[vertex_name, vertex] : this->vertices) {
    vertex_list.push_back(&vertex);
  }

  // Sort by current index to maintain relative order
  std::sort(vertex_list.begin(), vertex_list.end(),
            [](const Vertex *a, const Vertex *b) {
              int a_index = (a->index != -1) ? a->index : INT_MAX;
              int b_index = (b->index != -1) ? b->index : INT_MAX;
              return a_index < b_index;
            });

  // Reassign indices
  for (size_t i = 0; i < vertex_list.size(); ++i) {
    vertex_list[i]->index = static_cast<int>(i);
  }

  vertex_count = static_cast<int>(vertex_list.size());
}

void Graph::_reassign_edge_indices() {
  std::vector<Edge *> edge_list;
  std::set<std::pair<std::string, std::string>> seen;

  // Collect all unique edges
  for (auto &[u, neighbors] : this->edges) {
    for (auto &[v, edge] : neighbors) {
      std::pair<std::string, std::string> edge_tuple =
          (u < v) ? std::make_pair(u, v) : std::make_pair(v, u);
      if (seen.find(edge_tuple) == seen.end()) {
        seen.insert(edge_tuple);
        edge_list.push_back(&edge);
      }
    }
  }

  // Sort by current index to maintain relative order
  std::sort(edge_list.begin(), edge_list.end(),
            [](const Edge *a, const Edge *b) {
              int a_index = (a->index != -1) ? a->index : INT_MAX;
              int b_index = (b->index != -1) ? b->index : INT_MAX;
              return a_index < b_index;
            });

  // Reassign indices
  for (size_t i = 0; i < edge_list.size(); ++i) {
    edge_list[i]->index = static_cast<int>(i);
  }

  // Update the indices in the main edges map
  for (auto &[u, neighbors] : this->edges) {
    for (auto &[v, edge] : neighbors) {
      // Find the updated edge in the sorted list to get its new index
      for (const auto &sorted_edge : edge_list) {
        if (sorted_edge->guid == edge.guid) {
          edge.index = sorted_edge->index;
          break;
        }
      }
    }
  }

  edge_count = static_cast<int>(edge_list.size());
}

std::vector<Vertex> Graph::get_vertices() {
  std::vector<Vertex> result;
  for (auto const &[vertex_name, vertex] : this->vertices) {
    result.push_back(vertex);
  }
  return result;
}

std::vector<std::tuple<std::string, std::string>> Graph::get_edges() {
  std::vector<std::tuple<std::string, std::string>> result;
  std::set<std::pair<std::string, std::string>> seen;

  for (const auto &[u, neighbors] : edges) {
    for (const auto &[v, edge] : neighbors) {
      std::pair<std::string, std::string> edge_tuple =
          (u < v) ? std::make_pair(u, v) : std::make_pair(v, u);
      if (seen.find(edge_tuple) == seen.end()) {
        seen.insert(edge_tuple);
        result.push_back(std::make_tuple(edge_tuple.first, edge_tuple.second));
      }
    }
  }

  return result;
}

std::vector<std::string> Graph::neighbors(const std::string &node) {
  if (!has_node(node)) {
    throw std::runtime_error("Node " + node + " not in graph");
  }
  std::vector<std::string> result;
  auto it = edges.find(node);
  if (it != edges.end()) {
    for (const auto &[neighbor, edge] : it->second) {
      result.push_back(neighbor);
    }
  }
  return result;
}

int Graph::number_of_vertices() const {
  return static_cast<int>(vertices.size());
}

int Graph::number_of_edges() const {
  // Count unique undirected edges similarly to edges()
  std::set<std::pair<std::string, std::string>> seen;
  for (const auto &[u, neighbors] : edges) {
    for (const auto &[v, edge] : neighbors) {
      auto edge_tuple = (u < v) ? std::make_pair(u, v) : std::make_pair(v, u);
      seen.insert(edge_tuple);
    }
  }
  return static_cast<int>(seen.size());
}

void Graph::clear() {
  vertices.clear();
  edges.clear();
  vertex_count = 0;
  edge_count = 0;
}

std::string Graph::node_attribute(const std::string &node,
                                  const std::string &value) {
  if (!has_node(node)) {
    throw std::runtime_error("Node " + node + " not in graph");
  }

  if (value == "") {
    return vertices[node].attribute;
  } else {
    vertices[node].attribute = value;
    return value;
  }
}

std::string Graph::edge_attribute(const std::string &u, const std::string &v,
                                  const std::string &value) {
  // Check if edge exists in either direction
  bool has_u_v =
      edges.find(u) != edges.end() && edges[u].find(v) != edges[u].end();
  bool has_v_u =
      edges.find(v) != edges.end() && edges[v].find(u) != edges[v].end();

  if (!has_u_v && !has_v_u) {
    throw std::runtime_error("Edge (" + u + ", " + v + ") not in graph");
  }

  // Get the edge object (exists in at least one direction)
  Edge &edge = has_u_v ? edges[u][v] : edges[v][u];

  if (value.empty()) {
    return edge.attribute;
  } else {
    // Update attribute in both directions
    edge.attribute = value;
    if (has_u_v)
      edges[u][v].attribute = value;
    if (has_v_u)
      edges[v][u].attribute = value;
    return value;
  }
}

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Vertex &vertex) {
  return os << vertex.to_string();
}

std::ostream &operator<<(std::ostream &os, const Edge &edge) {
  return os << edge.to_string();
}

std::ostream &operator<<(std::ostream &os, const Graph &graph) {
  return os << graph.to_string();
}
} // namespace session_cpp