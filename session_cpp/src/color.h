#pragma once
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include <array>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

namespace session_cpp {

/**
 * @class Color
 * @brief A color is defined by RGBA coordinates from 0 to 255.
 */
class Color {
public:
  std::string name = "my_color"; ///< Name of the color
  std::string guid = ::guid();   ///< Unique identifier
  unsigned int r;                ///< Red component (0-255)
  unsigned int g;                ///< Green component (0-255)
  unsigned int b;                ///< Blue component (0-255)
  unsigned int a;                ///< Alpha component (0-255)

  /// Constructor with RGBA values.
  Color(unsigned int r = 255, unsigned int g = 255, unsigned int b = 255,
        unsigned int a = 255, std::string name = "my_color")
      : name(name), r(r), g(g), b(b), a(a) {}

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert point to string representation
  std::string to_string() const;

  /// Equality operator
  bool operator==(const Color &other) const;

  /// Inequality operator
  bool operator!=(const Color &other) const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert to JSON-serializable object.
  nlohmann::ordered_json jsondump() const;

  /// Create color from JSON data.
  static Color jsonload(const nlohmann::json &data);

  /// Serialize to JSON file

  /// Deserialize from JSON file

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Create a white color.
  static Color white();

  /// Create a black color.
  static Color black();

  /// Create a grey color.
  static Color grey();

  /// Create a red color.
  static Color red();

  /// Create an orange color.
  static Color orange();

  /// Create a yellow color.
  static Color yellow();

  /// Create a lime color.
  static Color lime();

  /// Create a green color.
  static Color green();

  /// Create a mint color.
  static Color mint();

  /// Create a cyan color.
  static Color cyan();

  /// Create an azure color.
  static Color azure();

  /// Create a blue color.
  static Color blue();

  /// Create a violet color.
  static Color violet();

  /// Create a magenta color.
  static Color magenta();

  /// Create a pink color.
  static Color pink();

  /// Create a maroon color.
  static Color maroon();

  /// Create a brown color.
  static Color brown();

  /// Create an olive color.
  static Color olive();

  /// Create a teal color.
  static Color teal();

  /// Create a navy color.
  static Color navy();

  /// Create a purple color.
  static Color purple();

  /// Create a silver color.
  static Color silver();

  /// Convert to normalized float array [0-1].
  std::array<double, 4> to_float_array() const;

  /// Create color from normalized float values [0-1].
  static Color from_float(double r, double g, double b, double a);
};

/**
 * @brief  To use this operator, you can do:
 *         Point point(1.5, 2.5, 3.5);
 *         std::cout << "Created point: " << point << std::endl;
 * @param os The output stream.
 * @param point The Point to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const Color &color);

} // namespace session_cpp

// fmt formatter specialization for Color - enables direct fmt::print(color)
template <> struct fmt::formatter<session_cpp::Color> {
  constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }

  auto format(const session_cpp::Color &o, fmt::format_context &ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};
