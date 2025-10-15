#pragma once
#include "fmt/core.h" // for fmt::formatter specializations in this header
#include "guid.h"     // for ::guid() used in in-class member initializers
#include "json.h"     // for nlohmann::json types used in function signatures
#include <array>
#include <map>
#include <ostream>
#include <string>
#include <tuple>
#include <vector>
// Implementation-only headers required by this translation unit
#include <algorithm> // std::sort
#include <climits>   // INT_MAX
#include <fstream>   // std::ifstream, std::ofstream
#include <set>       // std::set
#include <stdexcept> // std::runtime_error
#include "vertex.h"
#include "edge.h"

namespace session_cpp {

/**
 * @class Graph
 * @brief A graph data structure with string-only vertices and attributes.
 */
class Graph {
private:
  std::map<std::string, Vertex> vertices; /// node_name -> Vertex object

public:
  std::map<std::string, std::map<std::string, Edge>>
      edges; /// node_name -> {neighbor_name -> Edge object}
  std::string name = "my_graph"; ///< The name of the graph
  std::string guid = ::guid();   ///< The unique identifier of the graph
  int vertex_count = 0;          ///< Track next available vertex index
  int edge_count = 0;            ///< Track next available edge index

  /**
   * @brief Initialize a new Graph.
   * @param name The name of the graph.
   */
  Graph(std::string name = "my_graph")
      : name(name), vertex_count(0), edge_count(0) {}

  /// Convert graph to string representation
  std::string to_string() const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// @brief  Convert the Graph to a JSON-serializable dictionary.
  /// @return Dictionary representation of the graph.
  nlohmann::ordered_json jsondump() const;

  /// @brief Create a Graph from JSON data dictionary.
  /// @param data Dictionary containing graph data.
  /// @return Graph instance created from the data.
  static Graph jsonload(const nlohmann::json &data);

  /// @brief Load a Graph from a JSON file.
  /// @param filepath Path to the JSON file to load.
  /// @return Graph instance loaded from the file.

  /// @brief Save the Graph to a JSON file.
  /// @param filepath Path to the JSON file to save.

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// @brief Check if a node exists in the graph.
  /// @param key The node to check for.
  /// @return True if the node exists.
  bool has_node(const std::string &key);

  /// @brief Check if an edge exists in the graph.
  /// @param key Array containing the two vertex names [u, v].
  /// @return True if the edge exists.
  bool has_edge(const std::tuple<std::string, std::string> &key);

  /// @brief Add a node to the graph.
  /// @param key The node identifier.
  /// @param attribute Node attribute data.
  /// @return The name of the vertex (key).
  std::string add_node(const std::string &key,
                       const std::string &attribute = "");

  /// @brief Add an edge between u and v.
  /// @param u First node (must be string).
  /// @param v Second node (must be string).
  /// @param attribute Single string attribute for the edge.
  /// @return The edge tuple as array [u, v].
  std::tuple<std::string, std::string>
  add_edge(const std::string &u, const std::string &v,
           const std::string &attribute = "");

  /// @brief Remove a node and all its edges from the graph.
  /// @param key The node to remove.
  /// @throws std::runtime_error If the node is not in the graph.
  void remove_node(const std::string &key);

  /// @brief Remove an edge from the graph.
  /// @param edge Array containing the two vertex names [u, v].
  void remove_edge(const std::tuple<std::string, std::string> &edge);

private:
  /// @brief Reassign vertex indices to maintain contiguous sequence 0, 1, 2,
  /// ...
  void _reassign_indices();

  /// @brief Reassign edge indices to maintain contiguous sequence 0, 1, 2, ...
  void _reassign_edge_indices();

public:
  /// @brief Get all vertices in the graph.
  /// @return Vector of Vertex objects.
  std::vector<Vertex> get_vertices();

  /// @brief Get all edges in the graph.
  /// @return Vector of edge tuples as arrays [u, v].
  std::vector<std::tuple<std::string, std::string>> get_edges();

  /// @brief Get all neighbors of a node.
  /// @param node The node to get neighbors for.
  /// @return Vector of neighbor vertex names.
  /// @throws std::runtime_error If the node is not in the graph.
  std::vector<std::string> neighbors(const std::string &node);

  /// @brief Get the number of vertices in the graph.
  /// @return Number of vertices.
  int number_of_vertices() const;

  /// @brief Get the number of edges in the graph.
  /// @return Number of edges.
  int number_of_edges() const;

  /// @brief Remove all vertices and edges from the graph.
  void clear();

  /// @brief Get or set node attribute.
  /// @param node node identifier
  /// @param value if provided, set the attribute to this value
  /// @return attribute value as string
  std::string node_attribute(const std::string &node,
                             const std::string &value = "");

  /// @brief Get or set edge attribute.
  /// @param u First node of the edge
  /// @param v Second node of the edge
  /// @param value If provided, set the attribute to this value
  /// @return attribute value as string
  std::string edge_attribute(const std::string &u, const std::string &v,
                             const std::string &value = "");
};

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Vertex &vertex);

std::ostream &operator<<(std::ostream &os, const Edge &edge);

std::ostream &operator<<(std::ostream &os, const Graph &graph);
} // namespace session_cpp

// fmt formatter specialization for Vertex - enables direct fmt::print(vertex)
template <> struct fmt::formatter<session_cpp::Vertex> {
  constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }

  auto format(const session_cpp::Vertex &o, fmt::format_context &ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};

// fmt formatter specialization for Edge - enables direct fmt::print(edge)
template <> struct fmt::formatter<session_cpp::Edge> {
  constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }

  auto format(const session_cpp::Edge &o, fmt::format_context &ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};

// fmt formatter specialization for Graph - enables direct fmt::print(graph)
template <> struct fmt::formatter<session_cpp::Graph> {
  constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }

  auto format(const session_cpp::Graph &o, fmt::format_context &ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};
