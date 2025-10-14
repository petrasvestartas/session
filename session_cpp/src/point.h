#pragma once
#include "color.h"
#include "xform.h"
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
  float width = 1.0f;                ///< Point diameter in pixels
  Color pointcolor = Color::white(); ///< Color of the point
  Xform xform = Xform::identity();   ///< Transformation matrix

private:
  float _x = 0.0f;                   ///< X coordinate (private)
  float _y = 0.0f;                   ///< Y coordinate (private)
  float _z = 0.0f;                   ///< Z coordinate (private)

public:
  /// Getters for coordinates
  float x() const { return _x; }
  float y() const { return _y; }
  float z() const { return _z; }

  /// Setters for coordinates
  void set_x(float v) { _x = v; }
  void set_y(float v) { _y = v; }
  void set_z(float v) { _z = v; }

  /**
   * @brief Constructor.
   * @param x The X coordinate of the point.
   * @param y The Y coordinate of the point.
   * @param z The Z coordinate of the point.
   */
  Point(float x, float y, float z) : _x(x), _y(y), _z(z) {}
  Point() : _x(0.0f), _y(0.0f), _z(0.0f) {}

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
  nlohmann::ordered_json jsondump() const;

  /// Create point from JSON data
  static Point jsonload(const nlohmann::json &data);

  /// Serialize to JSON file

  /// Deserialize from JSON file

  ///////////////////////////////////////////////////////////////////////////////////////////
  // No-copy Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  float &operator[](int index);

  const float &operator[](int index) const;

  Point &operator*=(float factor);
  
  Point &operator/=(float factor);

  Point &operator+=(const Vector &other);

  Point &operator-=(const Vector &other);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Copy Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  Point operator*(float factor) const;

  Point operator/(float factor) const;

  Point operator+(const Vector& other) const;

  Point operator-(const Vector& other) const;

  Vector operator-(const Point& other) const;

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
   * @param float_min The minimum value for the distance. Defaults to 1e-12.
   * @return The distance between this point and the other point.
   */
  float distance(const Point& p, float float_min = 1e-12f) const;

  /**
   * @brief Calculate the area of a polygon.
   * 
   * @param points The points of the polygon.
   * @return The area of the polygon.
   */
  static float area(const std::vector<Point>& points);

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
