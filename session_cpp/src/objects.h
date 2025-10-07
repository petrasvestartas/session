#pragma once
#include "guid.h"  // For ::guid()
#include "json.h"  // For nlohmann::ordered_json
#include "point.h" // For Point
#include <fstream> // For std::ifstream and std::ofstream
#include <memory>  // For std::shared_ptr
#include <string>
#include <vector>

namespace session_cpp {
/**
 * @class Objects
 * @brief A collection of geometry objects.
 */
class Objects {
public:
  std::string name = "my_objects"; ///< The name of the objects
  std::string guid = ::guid();     ///< The unique identifier of the objects
  std::shared_ptr<std::vector<std::shared_ptr<Point>>>
      points; ///< Shared pointer to the list of point shared_ptrs

  /**
   * @brief Constructor.
   * @param name The name of the collection.
   * @param points Shared pointer to the list of points in the collection.
   */
  Objects(std::string name = "my_objects",
          std::shared_ptr<std::vector<std::shared_ptr<Point>>> points = nullptr)
      : name(std::move(name)) {
    this->points =
        points ? std::move(points)
               : std::make_shared<std::vector<std::shared_ptr<Point>>>();
  }

  /// Convert point to string representation
  std::string to_string() const;

  /**
   * @brief Serializes the Objects instance to JSON.
   * @return JSON representation of the Objects instance.
   */
  nlohmann::ordered_json to_json_data() const;

  /**
   * @brief Creates an Objects instance from JSON data.
   * @param data JSON data containing objects information.
   * @return Objects instance created from the data.
   */
  static Objects from_json_data(const nlohmann::json &data);

  /**
   * @brief Saves the Objects instance to a JSON file.
   * @param filepath Path where to save the JSON file.
   */
  void to_json(const std::string &filepath) const;

  /**
   * @brief Loads an Objects instance from a JSON file.
   * @param filepath Path to the JSON file to load.
   * @return Objects instance loaded from the file.
   */
  static Objects from_json(const std::string &filepath);
};
/**
 * @brief  To use this operator, you can do:
 *         Point point(1.5, 2.5, 3.5);
 *         std::cout << "Created point: " << point << std::endl;
 * @param os The output stream.
 * @param point The Point to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const Objects &objects);
} // namespace session_cpp

template <> struct fmt::formatter<session_cpp::Objects> {
  constexpr auto parse(fmt::format_parse_context &ctx) { return ctx.begin(); }

  auto format(const session_cpp::Objects &o, fmt::format_context &ctx) const {
    return fmt::format_to(ctx.out(), "{}", o.to_string());
  }
};
