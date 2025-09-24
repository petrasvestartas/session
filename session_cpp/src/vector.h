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
