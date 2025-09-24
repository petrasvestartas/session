#include "color.h"

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert point to string representation
std::string Color::to_string() const {
  return fmt::format("Color({}, {}, {}, {}, {})", r, g, b, a, name);
}

/// Equality operator
bool Color::operator==(const Color &other) const {
  return r == other.r && g == other.g && b == other.b && a == other.a &&
         name == other.name;
}

/// Inequality operator
bool Color::operator!=(const Color &other) const { return !(*this == other); }

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json Color::to_json_data() const {
  return nlohmann::ordered_json{{"type", "Color"},
                                {"guid", guid},
                                {"name", name},
                                {"r", static_cast<int>(r)},
                                {"g", static_cast<int>(g)},
                                {"b", static_cast<int>(b)},
                                {"a", static_cast<int>(a)}};
}

Color Color::from_json_data(const nlohmann::json &data) {
  Color color = Color(static_cast<unsigned int>(data["r"]),
                      static_cast<unsigned int>(data["g"]),
                      static_cast<unsigned int>(data["b"]),
                      static_cast<unsigned int>(data["a"]), data["name"]);
  color.guid = data["guid"];
  return color;
}

void Color::to_json(const std::string &filepath) const {
  std::ofstream file(filepath);
  file << to_json_data().dump(4);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

Color Color::from_json(const std::string &filepath) {
  std::ifstream file(filepath);
  nlohmann::json data;
  file >> data;
  return from_json_data(data);
}

Color Color::white() { return Color(255, 255, 255, 255, "white"); }

Color Color::black() { return Color(0, 0, 0, 255, "black"); }

std::array<double, 4> Color::to_float_array() const {
  return {r / 255.0, g / 255.0, b / 255.0, a / 255.0};
}

Color Color::from_float(double r, double g, double b, double a) {
  return Color(static_cast<unsigned int>(r * 255.0 + 0.5),
               static_cast<unsigned int>(g * 255.0 + 0.5),
               static_cast<unsigned int>(b * 255.0 + 0.5),
               static_cast<unsigned int>(a * 255.0 + 0.5));
}

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Color &color) {
  return os << color.to_string();
}

} // namespace session_cpp
