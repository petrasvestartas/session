#pragma once
#include "color.h"
#include "xform.h"
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
#include <point.h>

namespace session_cpp {

    /**
     * @class Plane
     * @brief A plane defined by origina and x-axis, z-axis, y-axis.
     */
    class Plane {
        public:
        std::string guid = ::guid();       ///< Unique identifier for the plane
        std::string name = "my_plane";     ///< Plane identifier/name
        Xform xform;   ///< Transformation matrix

        private:
        Point _origin = Point();   ///< Origin (private)
        Vector _x_axis = Vector::x_axis(); ///< X axis (private)
        Vector _y_axis = Vector::y_axis(); ///< Y axis (private)
        Vector _z_axis = Vector::z_axis(); ///< Z axis (private)
        double _a = 0.0; ///< X coordinate (private)
        double _b = 0.0; ///< Y coordinate (private)
        double _c = 1.0; ///< Z coordinate (private)
        double _d = 0.0; ///< W coordinate (private)

        public:

    /// Getters plane attributes
        const Point& origin() const { return _origin; }
        const Vector& x_axis() const { return _x_axis; } 
        const Vector& y_axis() const { return _y_axis; }
        const Vector& z_axis() const { return _z_axis; }
        double a() const { return _a; }
        double b() const { return _b; }
        double c() const { return _c; }
        double d() const { return _d; }

        Plane();
        Plane(Point& point, Vector& x_axis, Vector& y_axis, std::string name = "my_plane");
        static Plane from_point_normal(Point& point, Vector& normal);
        static Plane from_points(std::vector<Point>& points);
        static Plane from_two_points(Point& point1, Point& point2);
        static Plane xy_plane();
        static Plane yz_plane();
        static Plane xz_plane();

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Operators - const because they oinly read values, dont modify them
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert point to string representation
    std::string to_string() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Apply the stored xform transformation to the plane (in-place)
    void transform();

    /// Return a transformed copy of the plane
    Plane transformed() const;

    /// Equality operator
    bool operator==(const Point &other) const;

    /// Inequality operator
    bool operator!=(const Point &other) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // No-copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    Vector &operator[](int index);
    const Vector &operator[](int index) const;

    Plane &operator+=(const Vector &other);
    Plane &operator-=(const Vector &other);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    Plane operator+(const Vector &other) const;
    Plane operator-(const Vector &other) const;


  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert to JSON-serializable object
  nlohmann::ordered_json jsondump() const;

  /// Create vector from JSON data
  static Plane jsonload(const nlohmann::json &data);

  /// Serialize to JSON file

  /// Deserialize from JSON file

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Reverse the plane's normal direction.
   */
  void reverse();

  /**
   * @brief Rotate the plane around its normal.
   * 
   * @param angles_in_radians The rotation angle in radians.
   */
  void rotate(double angles_in_radians);

  /**
   * @brief Check if the plane follows the right-hand rule.
   * 
   * @return True if x_axis × y_axis = z_axis (right-handed).
   */
  bool is_right_hand() const;

  /**
   * @brief Check if two planes have the same or flipped normal.
   * 
   * @param plane0 First plane.
   * @param plane1 Second plane.
   * @param can_be_flipped Bool flag to indicate if the normal can be flipped.
   * @return True if two planes pointing to the same or flipped normal.
   */
  static bool is_same_direction(const Plane &plane0, const Plane &plane1, bool can_be_flipped = true);

  /**
   * @brief Check if two planes are in the same position.
   * 
   * @param plane0 First plane.
   * @param plane1 Second plane.
   * @return True if origins are very close to each other.
   */
  static bool is_same_position(const Plane &plane0, const Plane &plane1);

  /**
   * @brief Check if two planes have the same or flipped normal and share the same origin.
   * 
   * @param plane0 First plane.
   * @param plane1 Second plane.
   * @param can_be_flipped Bool flag to indicate if the normal can be flipped.
   * @return True if two planes are pointing to the same or flipped normal and share the same origin.
   */
  static bool is_coplanar(const Plane &plane0, const Plane plane1, bool can_be_flipped = true);

  /**
   * @brief Translate (move) a plane along its normal direction by a specified distance.
   * 
   * @param distance Distance to move the plane along its normal (positive = normal direction, negative = opposite).
   * @return New plane translated by the specified distance.
   */
  Plane translate_by_normal(double distance) const;

  
}; // End of Plane class

/**
 * @brief  To use this operator, you can do:
 *         Plane plane();
 *         std::cout << "Created plane: " << plane << std::endl;
 * @param os The output stream.
 * @param plane The Plane to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const Plane &plane);

} // namespace session_cpp


// fmt formatter specialization for Vector - enables direct fmt::print(plane)
template <> struct fmt::formatter<session_cpp::Plane> {
    constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }
  
    auto format(const session_cpp::Plane &o, fmt::format_context &ctx) const {
      return fmt::format_to(ctx.out(), "{}", o.to_string());
    }
  };
