#pragma once
#include "fmt/core.h" // for fmt::formatter specializations in this header
#include "guid.h"     // for ::guid() used in in-class member initializers
#include "json.h"     // for nlohmann::json types used in function signatures
#include <ostream>
#include <string>

namespace session_cpp {

/**
 * @class Edge
 * @brief A graph edge connecting two vertices with an attribute string.
 */
class Edge {
public:
  std::string guid = ::guid();  ///< The unique identifier of the edge.
  std::string name = "my_edge"; ///< The name of the edge.
  std::string v0 = "";          ///< The first vertex of the edge.
  std::string v1 = "";          ///< The second vertex of the edge.
  std::string attribute = "";   ///< Edge attribute data as string.
  int index = -1;               ///< Integer index for the edge.

  /**
   * @brief Initialize a new Edge.
   * @param v0 The first vertex of the edge.
   * @param v1 The second vertex of the edge.
   * @param attribute The attribute of the edge.
   */
  Edge(std::string v0 = "", std::string v1 = "", std::string attribute = "")
      : v0(v0), v1(v1), attribute(attribute) {}

  /// Convert edge to string representation
  std::string to_string() const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert the Edge to a JSON-serializable dictionary.
  nlohmann::ordered_json jsondump() const;

  /// Create Edge from JSON data dictionary.
  static Edge jsonload(const nlohmann::json &data);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Get the edge vertices as a tuple.
  std::tuple<std::string, std::string> vertices() const;

  /// @brief Check if this edge connects to a given vertex.
  /// @param vertex_id
  /// @return true if this edge connects to the given vertex, false otherwise
  bool connects(const std::string &vertex_id);

  /// @brief Get the other vertex ID connected by this edge.
  /// @param vertex_id
  /// @return the other vertex ID connected by this edge
  std::string other_vertex(const std::string &vertex_id);
};

} // namespace session_cpp
