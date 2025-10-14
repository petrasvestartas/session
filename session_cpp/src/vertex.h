#pragma once
#include "fmt/core.h" // for fmt::formatter specializations in this header
#include "guid.h"     // for ::guid() used in in-class member initializers
#include "json.h"     // for nlohmann::json types used in function signatures
#include <ostream>
#include <string>

namespace session_cpp {

/**
 * @class Vertex
 * @brief A graph vertex with a unique identifier and attribute string.
 */
class Vertex {
public:
  std::string name = "my_vertex"; ///< The name of the vertex
  std::string guid = ::guid();    ///< The unique identifier of the vertex
  std::string attribute = "";     ///< Vertex attribute data as string
  int index = -1;                 ///< Integer index for the vertex

  /**
   * @brief Initialize a new Vertex.
   * @param name The name of the vertex.
   * @param attribute The attribute of the vertex.
   */
  Vertex(std::string name = "my_vertex", std::string attribute = "")
      : name(name), attribute(attribute) {}

  /// Convert vertex to string representation
  std::string to_string() const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert the Vertex to a JSON-serializable dictionary.
  nlohmann::ordered_json jsondump() const;

  /// Create Vertex from JSON data dictionary.
  static Vertex jsonload(const nlohmann::json &data);
};

} // namespace session_cpp
