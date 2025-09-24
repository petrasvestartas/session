#include "point.h"

namespace session_cpp {

/// Convert point to string representation
std::string Point::to_string() const {
  return fmt::format("Point({}, {}, {}, {}, {}, {}, {})", x, y, z, guid, name,
                     pointcolor.to_string(), width);
}

/// Equality operator
bool Point::operator==(const Point &other) const {
  return x == other.x && y == other.y && z == other.z;
}

/// Inequality operator
bool Point::operator!=(const Point &other) const { return !(*this == other); }

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert to JSON-serializable object
nlohmann::ordered_json Point::to_json_data() const {
  return nlohmann::ordered_json{
      {"type", "Point"}, {"guid", guid},
      {"name", name},    {"x", x},
      {"y", y},          {"z", z},
      {"width", width},  {"pointcolor", pointcolor.to_json_data()}};
}

/// Create point from JSON data
Point Point::from_json_data(const nlohmann::json &data) {
  Point point(data["x"], data["y"], data["z"]);
  point.guid = data["guid"];
  point.name = data["name"];
  point.pointcolor = Color::from_json_data(data["pointcolor"]);
  point.width = data["width"];
  return point;
}

/// Serialize to JSON file
void Point::to_json(const std::string &filepath) const {
  std::ofstream file(filepath);
  file << to_json_data().dump(4);
}

/// Deserialize from JSON file
Point Point::from_json(const std::string &filepath) {
  std::ifstream file(filepath);
  nlohmann::json data;
  file >> data;
  return from_json_data(data);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Point &point) {
  return os << point.to_string();
}
} // namespace session_cpp