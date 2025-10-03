#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include <array>
#include <cmath>
#include <tuple>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include "globals.h"

namespace session_cpp {
/**
 * @class Vector
 * @brief A vector defined by XYZ coordinates with display properties.
 */
class Vector {
public:
  std::string guid = ::guid();    ///< Unique identifier for the vector
  std::string name = "my_vector"; ///< Vector identifier/name

private:
  float _x = 0.0f;                   ///< X coordinate (private)
  float _y = 0.0f;                   ///< Y coordinate (private)
  float _z = 0.0f;                   ///< Z coordinate (private)
  mutable float _length = 0.0f;      ///< Cached length value
  mutable bool _has_length = false;  ///< Cache validity flag

  /// Invalidates the cached length when coordinates change
  void invalidate_length_cache() { _has_length = false; }

  /// Gets cached length, computing if necessary (internal only)
  float cached_length() const;

public:
  /// Getters for coordinates
  float x() const { return _x; }
  float y() const { return _y; }
  float z() const { return _z; }

  /// Setters for coordinates (invalidate cached length)
  void set_x(float v) { _x = v; invalidate_length_cache(); }
  void set_y(float v) { _y = v; invalidate_length_cache(); }
  void set_z(float v) { _z = v; invalidate_length_cache(); }

public:
  /**
   * @brief Constructor.
   * @param x The X coordinate of the vector.
   * @param y The Y coordinate of the vector.
   * @param z The Z coordinate of the vector.
   */
  Vector(float x, float y, float z) : _x(x), _y(y), _z(z) {}
  Vector() : _x(0.0f), _y(0.0f), _z(0.0f) {}

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert vector to string representation
  std::string to_string() const;

  /// Equality operator
  bool operator==(const Vector &other) const;

  /// Inequality operator
  bool operator!=(const Vector &other) const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // No-copy Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  float &operator[](int index);
  const float &operator[](int index) const;

  Vector &operator*=(float factor);
  Vector &operator/=(float factor);
  Vector &operator+=(const Vector &other);
  Vector &operator-=(const Vector &other);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Copy Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  Vector operator*(float factor) const;
  Vector operator/(float factor) const;
  Vector operator+(const Vector &other) const;
  Vector operator-(const Vector &other) const;
  friend Vector operator*(float factor, const Vector &v);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert to JSON-serializable object
  nlohmann::ordered_json to_json_data() const;

  /// Create vector from JSON data
  static Vector from_json_data(const nlohmann::json &data);

  /// Serialize to JSON file
  void to_json(const std::string &filepath) const;

  /// Deserialize from JSON file
  static Vector from_json(const std::string &filepath);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Static Methods
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Get unit vector along the x-axis.
  /// 
  /// Returns
  /// -------
  /// Vector
  ///     Unit vector (1, 0, 0).
  static Vector x_axis();
  
  /// Get unit vector along the y-axis.
  /// 
  /// Returns
  /// -------
  /// Vector
  ///     Unit vector (0, 1, 0).
  static Vector y_axis();
  
  /// Get unit vector along the z-axis.
  /// 
  /// Returns
  /// -------
  /// Vector
  ///     Unit vector (0, 0, 1).
  static Vector z_axis();
  static Vector from_start_and_end(const Vector &start, const Vector &end);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details / Geometry
  ///////////////////////////////////////////////////////////////////////////////////////////

  void reverse();
  /// Returns the (cached) magnitude. Does not rescale.
  ///
  /// Returns
  /// -------
  /// float
  ///     The magnitude (length) of the vector.
  float magnitude() const;
  
  /// Computes the length (magnitude) of the vector without caching.
  ///
  /// Returns
  /// -------
  /// float
  ///     The length of the vector.
  float compute_length() const;
  
  /// Get the squared length of the vector (avoids sqrt for performance).
  ///
  /// Returns
  /// -------
  /// float
  ///     The squared length of the vector.
  float length_squared() const;
  
  /// Normalize the vector in place (make it unit length).
  ///
  /// Returns
  /// -------
  /// bool
  ///     True if successful, false if vector has zero length.
  bool normalize_self();
  Vector normalize();

  /// Project this vector onto `projection_vector`.
  /// Returns: (projection_vector, projected_length, perpendicular_vector, perpendicular_length)
  std::tuple<Vector, float, Vector, float>
  projection(Vector &projection_vector,
             float tolerance = geo::GLOBALS::ZERO_TOLERANCE);

  /// Check if this vector is parallel/antiparallel to another.
  ///
  /// Parameters
  /// ----------
  /// other : const Vector&
  ///     Other vector.
  ///
  /// Returns
  /// -------
  /// int
  ///     1 for parallel, -1 for antiparallel, 0 for not parallel.
  int is_parallel_to(const Vector &other);
  
  /// Calculate dot product with another vector.
  ///
  /// Parameters
  /// ----------
  /// other : const Vector&
  ///     Other vector.
  ///
  /// Returns
  /// -------
  /// float
  ///     Dot product value.
  float dot(const Vector &other);
  
  /// Calculate cross product with another vector.
  ///
  /// Parameters
  /// ----------
  /// other : const Vector&
  ///     Other vector.
  ///
  /// Returns
  /// -------
  /// Vector
  ///     Cross product vector (orthogonal to inputs).
  Vector cross(const Vector &other);
  
  /// Angle between this vector and another.
  ///
  /// Parameters
  /// ----------
  /// other : const Vector&
  ///     Other vector.
  /// sign_by_cross_product : bool
  ///     Whether to use cross product for sign determination.
  /// degrees : bool
  ///     Return angle in degrees if true, radians if false.
  /// tolerance : float
  ///     Tolerance for zero-length vectors.
  ///
  /// Returns
  /// -------
  /// float
  ///     Angle between vectors.
  float angle(const Vector &other, bool sign_by_cross_product = true,
               bool degrees = true,
               float tolerance = geo::GLOBALS::ZERO_TOLERANCE);
  Vector get_leveled_vector(float &vertical_height);

  static float cosine_law(float &triangle_edge_length_a,
                           float &triangle_edge_length_b,
                           float &angle_in_between_edges, bool degrees = true);

  static float sine_law_angle(float &triangle_edge_length_a,
                               float &angle_in_front_of_a,
                               float &triangle_edge_length_b,
                               bool degrees = true);

  static float sine_law_length(float &triangle_edge_length_a,
                                float &angle_in_front_of_a,
                                float &angle_in_front_of_b,
                                bool degrees = true);

  /// Angle between XY components in degrees.
  static float angle_between_vector_xy_components(Vector &vector);

  static Vector sum_of_vectors(std::vector<Vector> &vectors);

  std::array<float, 3> coordinate_direction_3angles(bool degrees = false);
  std::array<float, 2> coordinate_direction_2angles(bool degrees = false);

  bool perpendicular_to(Vector &v);

  void scale(float factor);
  void scale_up();
  void scale_down();

}; // End of Vector class

/**
 * @brief  To use this operator, you can do:
 *         Vector vector(1.5, 2.5, 3.5);
 *         std::cout << "Created vector: " << vector << std::endl;
 * @param os The output stream.
 * @param vector The Vector to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const Vector &vector);

} // namespace session_cpp

// fmt formatter specialization for Vector - enables direct fmt::print(vector)
template <> struct fmt::formatter<session_cpp::Vector> {
  constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }

  auto format(const session_cpp::Vector &o, fmt::format_context &ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};
