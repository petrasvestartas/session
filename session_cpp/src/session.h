#pragma once
#include "color.h"
#include "fmt/core.h"
#include "graph.h"
#include "guid.h"
#include "json.h"
#include "objects.h"
#include "point.h"
#include "tree.h"
#include "vector.h"
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <string>
#include <unordered_map>
#include <variant>

namespace session_cpp {

// All geometry types as a variant - easily extensible for Curve, Mesh, etc.
using Geometry = std::variant<std::shared_ptr<Point>, std::shared_ptr<Vector>>;

/**
 * @class Session
 * @brief A session containing geometry objects.
 */
class Session {
public:
  std::string name = "my_session"; ///< The name of the session
  std::string guid = ::guid();     ///< The unique identifier of the session
  Objects objects;                 ///< Collection of geometry objects
  std::unordered_map<std::string, Geometry>
      lookup;  ///< Fast GUID-based lookup for heterogeneous geometry (shared
               ///< ownership)
  Tree tree;   ///< Hierarchical tree structure
  Graph graph; ///< Graph structure for relationships

  /**
   * @brief Constructor.
   * @param name The name of the session.
   */
  Session(std::string name = "my_session")
      : name(std::move(name)), objects(),
        tree(this->name + "_tree"), graph(this->name + "_graph") {
    // Create empty root node with session name
    auto root_node = std::make_shared<TreeNode>(this->name);
    tree.add(root_node);
  }

  /// Convert session to string representation
  std::string to_string() const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Geometry Management
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Get a geometry object by GUID with type safety.
   * @tparam T The geometry type to retrieve (Point, Vector, etc.)
   * @param guid The GUID of the geometry object
   * @return Shared pointer to the object if found and of correct type, nullptr
   * otherwise
   */
  template <typename T> std::shared_ptr<T> get_object(const std::string &guid) {
    auto it = lookup.find(guid);
    if (it == lookup.end())
      return nullptr;
    auto ptr = std::get_if<std::shared_ptr<T>>(&it->second);
    return ptr ? *ptr : nullptr;
  }

  /**
   * @brief Get a geometry object by GUID (const version).
   * @tparam T The geometry type to retrieve
   * @param guid The GUID of the geometry object
   * @return Shared pointer to the object if found and of correct type, nullptr
   * otherwise
   */
  template <typename T>
  std::shared_ptr<const T> get_object(const std::string &guid) const {
    auto it = lookup.find(guid);
    if (it == lookup.end())
      return nullptr;
    auto ptr = std::get_if<std::shared_ptr<T>>(&it->second);
    return ptr ? *ptr : nullptr;
  }

  /**
   * @brief Add a point to the session.
   * @param point Shared pointer to the point to add
   */
  void add_point(std::shared_ptr<Point> point);

  /**
   * @brief Add a vector to the session.
   * @param vector Shared pointer to the vector to add
   */
  void add_vector(std::shared_ptr<Vector> vector);

  /**
   * @brief Add an edge between two geometry objects in the graph.
   * @param guid1 GUID of the first geometry object
   * @param guid2 GUID of the second geometry object
   * @param attribute Edge attribute description
   */
  void add_edge(const std::string &guid1, const std::string &guid2,
                const std::string &attribute = "");

  /**
   * @brief Remove a geometry object by GUID.
   * @param guid The GUID of the object to remove
   * @return True if removed, false if not found
   */
  bool remove_object(const std::string &guid);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Tree Operations
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Add a parent-child relationship in the tree.
   * @param parent_guid GUID of the parent object
   * @param child_guid GUID of the child object
   * @return True if relationship was added successfully
   */
  bool add_hierarchy(const std::string &parent_guid,
                     const std::string &child_guid);

  /**
   * @brief Get children GUIDs of an object in the tree.
   * @param guid The GUID to search for
   * @return List of children GUIDs
   */
  std::vector<std::string> get_children(const std::string &guid) const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Graph Operations
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Add a relationship edge in the graph.
   * @param from_guid Source object GUID
   * @param to_guid Target object GUID
   * @param relationship_type Type of relationship
   */
  void add_relationship(const std::string &from_guid,
                        const std::string &to_guid,
                        const std::string &relationship_type = "default");

  /**
   * @brief Get all GUIDs connected to the given GUID in the graph.
   * @param guid The GUID to find connections for
   * @return List of connected GUIDs
   */
  std::vector<std::string> get_neighbours(const std::string &guid);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON Serialization
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Serializes the Session instance to JSON.
   * @return JSON representation of the Session instance.
   */
  nlohmann::ordered_json to_json_data();

  /**
   * @brief Creates a Session instance from JSON data.
   * @param data JSON data containing session information.
   * @return Session instance created from the data.
   */
  static Session from_json_data(const nlohmann::json &data);

  /**
   * @brief Saves the Session instance to a JSON file.
   * @param filepath Path where to save the JSON file.
   */
  void to_json(const std::string &filepath);

  /**
   * @brief Loads a Session instance from a JSON file.
   * @param filepath Path to the JSON file to load.
   * @return Session instance loaded from the file.
   */
  static Session from_json(const std::string &filepath);
};
/**
 * @brief  To use this operator, you can do:
 *         Point point(1.5, 2.5, 3.5);
 *         std::cout << "Created point: " << point << std::endl;
 * @param os The output stream.
 * @param point The Point to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const Session &session);
} // namespace session_cpp
