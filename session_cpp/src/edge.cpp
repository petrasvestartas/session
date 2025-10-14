#include "edge.h"

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

std::string Edge::to_string() const {
  return fmt::format("Edge({}, {}, {}, {})", guid, name, v0, v1);
}

nlohmann::ordered_json Edge::jsondump() const {
  return nlohmann::ordered_json{
      {"type", "Edge"}, {"guid", guid},           {"name", name},  {"v0", v0},
      {"v1", v1},       {"attribute", attribute}, {"index", index}};
}

/// Create Edge from JSON data dictionary.
Edge Edge::jsonload(const nlohmann::json &json_data) {
  Edge edge(json_data["v0"], json_data["v1"], json_data["attribute"]);
  edge.name = json_data["name"];
  edge.index = json_data["index"];
  edge.guid = json_data["guid"];
  return edge;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

std::tuple<std::string, std::string> Edge::vertices() const {
  return std::make_tuple(v0, v1);
}

bool Edge::connects(const std::string &vertex_id) {
  return v0 == vertex_id || v1 == vertex_id;
}

std::string Edge::other_vertex(const std::string &vertex_id) {
  if (v0 == vertex_id) {
    return v1;
  } else if (v1 == vertex_id) {
    return v0;
  } else {
    throw std::runtime_error("Vertex " + vertex_id +
                             " is not connected by this edge");
  }
}

} // namespace session_cpp
