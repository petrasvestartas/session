#include "vector.h"

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert vector to string representation
std::string Vector::to_string() const {
  return fmt::format("Vector({}, {}, {}, {}, {})", x, y, z, guid, name);
}

/// Equality operator
bool Vector::operator==(const Vector &other) const {
  return x == other.x && y == other.y && z == other.z;
}

/// Inequality operator
bool Vector::operator!=(const Vector &other) const { return !(*this == other); }

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert to JSON-serializable object
nlohmann::ordered_json Vector::to_json_data() const {
  return nlohmann::ordered_json{{"type", "Vector"}, {"guid", guid},
                                {"name", name},     {"x", x},
                                {"y", y},           {"z", z}};
}

/// Create vector from JSON data
Vector Vector::from_json_data(const nlohmann::json &data) {
  Vector vector(data["x"], data["y"], data["z"]);
  vector.guid = data["guid"];
  vector.name = data["name"];
  return vector;
}

/// Serialize to JSON file
void Vector::to_json(const std::string &filepath) const {
  std::ofstream file(filepath);
  file << to_json_data().dump(4);
}

/// Deserialize from JSON file
Vector Vector::from_json(const std::string &filepath) {
  std::ifstream file(filepath);
  nlohmann::json data;
  file >> data;
  return from_json_data(data);
}
///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Vector &point) {
  return os << point.to_string();
}

} // namespace session_cpp