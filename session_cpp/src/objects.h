#pragma once
#include "guid.h"  // For ::guid()
#include "json.h"  // For nlohmann::ordered_json
#include "point.h"
#include "vector.h"
#include "line.h"
#include "plane.h"
#include "boundingbox.h"
#include "polyline.h"
#include "pointcloud.h"
#include "mesh.h"
#include "cylinder.h"
#include "arrow.h"
#include <fstream>
#include <memory>
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
  
  // Collections for all geometry types
  std::shared_ptr<std::vector<std::shared_ptr<Point>>> points;
  std::shared_ptr<std::vector<std::shared_ptr<Line>>> lines;
  std::shared_ptr<std::vector<std::shared_ptr<Plane>>> planes;
  std::shared_ptr<std::vector<std::shared_ptr<BoundingBox>>> bboxes;
  std::shared_ptr<std::vector<std::shared_ptr<Polyline>>> polylines;
  std::shared_ptr<std::vector<std::shared_ptr<PointCloud>>> pointclouds;
  std::shared_ptr<std::vector<std::shared_ptr<Mesh>>> meshes;
  std::shared_ptr<std::vector<std::shared_ptr<Cylinder>>> cylinders;
  std::shared_ptr<std::vector<std::shared_ptr<Arrow>>> arrows;

  /**
   * @brief Constructor.
   * @param name The name of the collection.
   */
  Objects(std::string name = "my_objects") : name(std::move(name)) {
    // Initialize all geometry collections
    this->points = std::make_shared<std::vector<std::shared_ptr<Point>>>();
    this->lines = std::make_shared<std::vector<std::shared_ptr<Line>>>();
    this->planes = std::make_shared<std::vector<std::shared_ptr<Plane>>>();
    this->bboxes = std::make_shared<std::vector<std::shared_ptr<BoundingBox>>>();
    this->polylines = std::make_shared<std::vector<std::shared_ptr<Polyline>>>();
    this->pointclouds = std::make_shared<std::vector<std::shared_ptr<PointCloud>>>();
    this->meshes = std::make_shared<std::vector<std::shared_ptr<Mesh>>>();
    this->cylinders = std::make_shared<std::vector<std::shared_ptr<Cylinder>>>();
    this->arrows = std::make_shared<std::vector<std::shared_ptr<Arrow>>>();
  }

  /// Convert point to string representation
  std::string to_string() const;

  /**
   * @brief Serializes the Objects instance to JSON.
   * @return JSON representation of the Objects instance.
   */
  nlohmann::ordered_json jsondump() const;

  /**
   * @brief Creates an Objects instance from JSON data.
   * @param data JSON data containing objects information.
   * @return Objects instance created from the data.
   */
  static Objects jsonload(const nlohmann::json &data);

  /**
   * @brief Saves the Objects instance to a JSON file.
   * @param filepath Path where to save the JSON file.
   */

  /**
   * @brief Loads an Objects instance from a JSON file.
   * @param filepath Path to the JSON file to load.
   * @return Objects instance loaded from the file.
   */
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
