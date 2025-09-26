#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector.h>

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

  ///////////////////////////////////////////////////////////////////////////////////////////
  // No-copy Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  double &operator[](int index);

  const double &operator[](int index) const;

  Point &operator*=(double factor);
  
  Point &operator/=(double factor);

  Point &operator+=(const Point &other);

  Point &operator-=(const Point &other);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Copy Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  Point operator*(double factor) const;

  Point operator/(double factor) const;

  Point operator+(const Vector& other) const;

  Point operator-(const Vector& other) const;

  Point operator+(const Point& other) const;

  Point operator-(const Point& other) const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Check if the points are in counter-clockwise order.
   * 
   * @param a First point.
   * @param b Second point.
   * @param c Third point.
   * @return True if the points are in counter-clockwise order, False otherwise.
   */
  static bool ccw(const Point& a, const Point& b, const Point& c);

  /**
   * @brief Calculate the mid point between this point and another point.
   * 
   * @param p The other point.
   * @return The mid point between this point and the other point.
   */
  Point mid_point(const Point& p) const;

  /**
   * @brief Calculate the distance between this point and another point.
   * 
   * @param p The other point.
   * @param double_min The minimum value for the distance. Defaults to 1e-12.
   * @return The distance between this point and the other point.
   */
  double distance(const Point& p, double double_min = 1e-12) const;

  /**
   * @brief Calculate the area of a polygon.
   * 
   * @param points The points of the polygon.
   * @return The area of the polygon.
   */
  static double area(const std::vector<Point>& points);

  /**
   * @brief Calculate the centroid of a quadrilateral.
   * 
   * @param vertices The vertices of the quadrilateral.
   * @return The centroid of the quadrilateral.
   */
  static Point centroid_quad(const std::vector<Point>& vertices);

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
