#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include <array>
#include <cmath>
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
  double x = 0.0;                 ///< X coordinate
  double y = 0.0;                 ///< Y coordinate
  double z = 0.0;                 ///< Z coordinate

public:
  /**
   * @brief Constructor.
   * @param x The X coordinate of the vector.
   * @param y The Y coordinate of the vector.
   * @param z The Z coordinate of the vector.
   */
  Vector(double x, double y, double z) : x(x), y(y), z(z) {}
  Vector() : x(0.0), y(0.0), z(0.0) {}

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

  double &operator[](int index);
  const double &operator[](int index) const;

  Vector &operator*=(double factor);
  Vector &operator/=(double factor);
  Vector &operator+=(const Vector &other);
  Vector &operator-=(const Vector &other);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Copy Operators
  ///////////////////////////////////////////////////////////////////////////////////////////

  Vector operator*(double factor) const;
  Vector operator/(double factor) const;
  Vector operator+(const Vector &other) const;
  Vector operator-(const Vector &other) const;
  friend Vector operator*(double factor, const Vector &v);

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

  static Vector XAxis();
  static Vector YAxis();
  static Vector ZAxis();
  static Vector from_start_and_end(const Vector &start, const Vector &end);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details / Geometry
  ///////////////////////////////////////////////////////////////////////////////////////////

  void reverse();
  double length(double predefined_length = 0.0);
  double compute_length() const;
  bool unitize();
  Vector unitized();

  Vector projection(Vector &projection_vector,
                    double tolerance = geo::GLOBALS::ZERO_TOLERANCE,
                    double *out_projected_vector_length = nullptr,
                    Vector *out_perpendicular_projected_vector = nullptr,
                    double *out_perpendicular_projected_vector_length = nullptr);

  int is_parallel_to(Vector &v);
  double dot(Vector &other);
  Vector cross(Vector &other);
  double angle(Vector &other, bool sign_by_cross_product = true,
               bool degrees = true,
               double tolerance = geo::GLOBALS::ZERO_TOLERANCE);
  Vector get_leveled_vector(double &vertical_height);

  static double cosine_law(double &triangle_edge_length_a,
                           double &triangle_edge_length_b,
                           double &angle_in_between_edges, bool degrees = true);

  static double sine_law_angle(double &triangle_edge_length_a,
                               double &angle_in_front_of_a,
                               double &triangle_edge_length_b,
                               bool degrees = true);

  static double sine_law_length(double &triangle_edge_length_a,
                                double &angle_in_front_of_a,
                                double &angle_in_front_of_b,
                                bool degrees = true);

  static double angle_between_vector_xy_components_degrees(Vector &vector,
                                                           bool degrees = true);

  static Vector sum_of_vectors(std::vector<Vector> &vectors);

  std::array<double, 3> coordinate_direction_3angles(bool degrees = false);
  std::array<double, 2> coordinate_direction_2angles(bool degrees = false);

  bool perpendicular_to(Vector &v);

  void scale(double factor);
  void scale_up();
  void scale_down();
  void rescale(double factor);
  Vector rescaled(double factor);

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
