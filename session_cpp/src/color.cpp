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

nlohmann::ordered_json Color::jsondump() const {
  return nlohmann::ordered_json{{"type", "Color"},
                                {"guid", guid},
                                {"name", name},
                                {"r", static_cast<int>(r)},
                                {"g", static_cast<int>(g)},
                                {"b", static_cast<int>(b)},
                                {"a", static_cast<int>(a)}};
}

Color Color::jsonload(const nlohmann::json &data) {
  Color color(static_cast<unsigned int>(data["r"]),
                      static_cast<unsigned int>(data["g"]),
                      static_cast<unsigned int>(data["b"]),
                      static_cast<unsigned int>(data["a"]), data["name"]);
  color.guid = data["guid"];
  return color;
}


///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////////////////
// Presets
///////////////////////////////////////////////////////////////////////////////////////////

Color Color::white() { return Color(255, 255, 255, 255, "white"); }

Color Color::black() { return Color(0, 0, 0, 255, "black"); }

Color Color::grey() { return Color(128, 128, 128, 255, "grey"); }

Color Color::red() { return Color(255, 0, 0, 255, "red"); }

Color Color::orange() { return Color(255, 128, 0, 255, "orange"); }

Color Color::yellow() { return Color(255, 255, 0, 255, "yellow"); }

Color Color::lime() { return Color(128, 255, 0, 255, "lime"); }

Color Color::green() { return Color(0, 255, 0, 255, "green"); }

Color Color::mint() { return Color(0, 255, 128, 255, "mint"); }

Color Color::cyan() { return Color(0, 255, 255, 255, "cyan"); }

Color Color::azure() { return Color(0, 128, 255, 255, "azure"); }

Color Color::blue() { return Color(0, 0, 255, 255, "blue"); }

Color Color::violet() { return Color(128, 0, 255, 255, "violet"); }

Color Color::magenta() { return Color(255, 0, 255, 255, "magenta"); }

Color Color::pink() { return Color(255, 0, 128, 255, "pink"); }

Color Color::maroon() { return Color(128, 0, 0, 255, "maroon"); }

Color Color::brown() { return Color(128, 64, 0, 255, "brown"); }

Color Color::olive() { return Color(128, 128, 0, 255, "olive"); }

Color Color::teal() { return Color(0, 128, 128, 255, "teal"); }

Color Color::navy() { return Color(0, 0, 128, 255, "navy"); }

Color Color::purple() { return Color(128, 0, 128, 255, "purple"); }

Color Color::silver() { return Color(192, 192, 192, 255, "silver"); }

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

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
