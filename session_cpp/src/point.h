#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

namespace session_cpp {
/**
 * @class Point
 * @brief A point defined by XYZ coordinates with display properties.
 */
class Point {
public:
  std::string guid = ::guid();       ///< Unique identifier for the point
  std::string name = "my_point";     ///< Point identifier/name
  double x = 0.0;                    ///< X coordinate
  double y = 0.0;                    ///< Y coordinate
  double z = 0.0;                    ///< Z coordinate
  double width = 1.0;                ///< Point diameter in pixels
  Color pointcolor = Color::white(); ///< Color of the point

public:
  /**
   * @brief Constructor.
   * @param x The X coordinate of the point.
   * @param y The Y coordinate of the point.
   * @param z The Z coordinate of the point.
   */
  Point(double x, double y, double z) : x(x), y(y), z(z) {}

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Operators - const because they oinly read values, dont modify them
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert point to string representation
  std::string to_string() const;

  /// Equality operator
  bool operator==(const Point &other) const;

  /// Inequality operator
  bool operator!=(const Point &other) const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert to JSON-serializable object
  nlohmann::ordered_json to_json_data() const;

  /// Create point from JSON data
  static Point from_json_data(const nlohmann::json &data);

  /// Serialize to JSON file
  void to_json(const std::string &filepath) const;

  /// Deserialize from JSON file
  static Point from_json(const std::string &filepath);

}; // End of Point class

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

/**
 * @brief  To use this operator, you can do:
 *         Point point(1.5, 2.5, 3.5);
 *         std::cout << "Created point: " << point << std::endl;
 * @param os The output stream.
 * @param point The Point to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const Point &point);

} // namespace session_cpp

// fmt formatter specialization for Point - enables direct fmt::print(point)
template <> struct fmt::formatter<session_cpp::Point> {
  constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }

  auto format(const session_cpp::Point &o, fmt::format_context &ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};
