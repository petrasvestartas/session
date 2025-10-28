#pragma once
#include "color.h"
#include "fmt/core.h"
#include "graph.h"
#include "guid.h"
#include "json.h"
#include "objects.h"
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
#include "tree.h"
#include "bvh.h"
#include "tolerance.h"
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <string>
#include <unordered_map>
#include <variant>

namespace session_cpp {

/**
 * WHEN ADDING A NEW GEOMETRY TYPE:
 * 1. Add it to this Geometry variant below
 * 2. See complete checklist in session.cpp (top of file)
 * 3. Update Objects class collections (objects.h/cpp)
 * 4. Add Session::add_XXX() method
 * 5. Update visitor patterns: compute_bounding_box(), ray_intersect_geometry()
 */

// All geometry types as a variant
using Geometry = std::variant<
    std::shared_ptr<Arrow>,
    std::shared_ptr<BoundingBox>,
    std::shared_ptr<Cylinder>,
    std::shared_ptr<Line>,
    std::shared_ptr<Mesh>,
    std::shared_ptr<Plane>,
    std::shared_ptr<Point>,
    std::shared_ptr<PointCloud>,
    std::shared_ptr<Polyline>
    // ADD NEW GEOMETRY TYPE HERE (e.g., std::shared_ptr<Sphere>)
>;

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
      lookup; ///< Fast lookup table for objects by GUID
  Tree tree;  ///< Tree structure for hierarchy
  Graph graph; ///< Graph structure for relationships
  BVH bvh;    ///< Bounding volume hierarchy for collision detection
  
  // BVH caching for ray casting performance
  BVH cached_ray_bvh;                           ///< Cached BVH for ray casting
  std::vector<std::string> cached_guids;        ///< GUID mapping for cached BVH
  std::vector<BoundingBox> cached_boxes;        ///< Cached AABBs (avoid recomputing)
  bool bvh_cache_dirty = true;                  ///< Flag to rebuild BVH cache

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
   * @return Shared pointer to the TreeNode created for this point
   */
  std::shared_ptr<TreeNode> add_point(std::shared_ptr<Point> point);

  /**
   * @brief Add a line to the session.
   * @return Shared pointer to the TreeNode created for this line
   */
  std::shared_ptr<TreeNode> add_line(std::shared_ptr<Line> line);

  /**
   * @brief Add a plane to the session.
   * @return Shared pointer to the TreeNode created for this plane
   */
  std::shared_ptr<TreeNode> add_plane(std::shared_ptr<Plane> plane);

  /**
   * @brief Add a bounding box to the session.
   * @return Shared pointer to the TreeNode created for this bounding box
   */
  std::shared_ptr<TreeNode> add_bbox(std::shared_ptr<BoundingBox> bbox);

  /**
   * @brief Add a polyline to the session.
   * @return Shared pointer to the TreeNode created for this polyline
   */
  std::shared_ptr<TreeNode> add_polyline(std::shared_ptr<Polyline> polyline);

  /**
   * @brief Add a point cloud to the session.
   * @return Shared pointer to the TreeNode created for this point cloud
   */
  std::shared_ptr<TreeNode> add_pointcloud(std::shared_ptr<PointCloud> pointcloud);

  /**
   * @brief Add a mesh to the session.
   * @return Shared pointer to the TreeNode created for this mesh
   */
  std::shared_ptr<TreeNode> add_mesh(std::shared_ptr<Mesh> mesh);

  /**
   * @brief Add a cylinder to the session.
   * @return Shared pointer to the TreeNode created for this cylinder
   */
  std::shared_ptr<TreeNode> add_cylinder(std::shared_ptr<Cylinder> cylinder);

  /**
   * @brief Add an arrow to the session.
   * @return Shared pointer to the TreeNode created for this arrow
   */
  std::shared_ptr<TreeNode> add_arrow(std::shared_ptr<Arrow> arrow);

  /**
   * @brief Add a TreeNode to the tree hierarchy.
   * @param node The TreeNode to add
   * @param parent Optional parent TreeNode (defaults to root if not provided)
   */
  void add(std::shared_ptr<TreeNode> node, 
           std::shared_ptr<TreeNode> parent = nullptr);

  /**
   * @brief Add an edge between two geometry objects in the graph.
   * @param guid1 GUID of the first geometry object
   * @param guid2 GUID of the second geometry object
   * @param attribute Edge attribute description
   */
  void add_edge(const std::string &guid1, const std::string &guid2,
                const std::string &attribute = "");

  /**
   * @brief Remove an object from the session
   * @param obj_guid The GUID of the object to remove
   * @return True if removed, false if not found
   */
  bool remove_object(const std::string &obj_guid);

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
   * @brief Get the children of a parent GUID
   * @param obj_guid The GUID to search for
   * @return List of children GUIDs
   */
  std::vector<std::string> get_children(const std::string &obj_guid) const;

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
   * @brief Get the neighbours of a GUID
   * @param obj_guid The GUID to find connections for
   * @return List of connected GUIDs
   */
  std::vector<std::string> get_neighbours(const std::string &obj_guid);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // BVH Collision Detection
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Compute bounding box for a geometry object, inflated by tolerance.
   * @param geometry The geometry variant
   * @return Inflated bounding box for collision detection
   */
  static BoundingBox compute_bounding_box(const Geometry& geometry);

  /**
   * @brief Get all collision pairs using BVH and add them as graph edges.
   * 
   * Automatically:
   * - Computes bounding boxes for all objects with tolerance inflation
   * - Builds/rebuilds the BVH with auto-computed world size
   * - Detects all collision pairs
   * - Adds collision edges to the graph
   * 
   * @return Vector of (guid1, guid2) pairs representing colliding geometry
   */
  std::vector<std::pair<std::string, std::string>> get_collisions();

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Ray Intersection
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Result of a ray intersection with geometry.
   */
  struct RayHit {
    std::string guid;      ///< GUID of hit object
    Point hit_point;       ///< Intersection point
    double distance;        ///< Distance from ray origin
  };

  /**
   * @brief Cast a ray through the scene and find the closest intersecting geometry.
   * 
   * Uses BVH for acceleration:
   * - Phase 1: BVH ray traversal to get candidate objects (fast, conservative)
   * - Phase 2: Precise geometry intersection tests (slower, exact)
   * - Optimization: Tracks closest hit distance and only keeps nearest hits
   * 
   * @param origin Ray origin point
   * @param direction Ray direction vector
   * @param tolerance Intersection tolerance for proximity tests
   * @return Vector of closest hit(s) - typically one hit, may be multiple if at same distance
   */
  std::vector<RayHit> ray_cast(const Point& origin, const Vector& direction, double tolerance = 1e-3);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Transformed Geometry
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Get all geometry with transformations applied from tree hierarchy.
   * 
   * Recursively traverses the tree and applies parent transformations to children.
   * Each child's transformation is the composition of all ancestor transformations
   * multiplied by its own transformation.
   * 
   * @return Objects collection with transformed geometry
   */
  Objects get_geometry() const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON Serialization
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Serializes the Session instance to JSON.
   * @return JSON representation of the Session instance.
   */
  nlohmann::ordered_json jsondump() const;

  /**
   * @brief Creates a Session instance from JSON data.
   * @param data JSON data containing session information.
   * @return Session instance created from the data.
   */
  static Session jsonload(const nlohmann::json &data);

  /**
   * @brief Saves the Session instance to a JSON file.
   * @param filepath Path where to save the JSON file.
   */

  /**
   * @brief Loads a Session instance from a JSON file.
   * @param filepath Path to the JSON file to load.
   * @return Session instance loaded from the file.
   */

private:
  /**
   * @brief Test ray intersection with a specific geometry object.
   * @param ray The ray to test
   * @param geometry The geometry variant to test against
   * @param tolerance Intersection tolerance
   * @return Hit point if intersection found, nullopt otherwise
   */
  std::optional<Point> ray_intersect_geometry(const Line& ray, const Geometry& geometry, double tolerance);
  
  /**
   * @brief Rebuild the cached BVH for ray casting.
   * Called automatically when BVH cache is dirty.
   */
  void rebuild_ray_bvh_cache();
  
  /**
   * @brief Invalidate the BVH cache (call when geometry is added/removed).
   */
  void invalidate_bvh_cache() { bvh_cache_dirty = true; }
  
  /**
   * @brief Cache geometry bounding box incrementally
   * @param obj_guid The GUID of the geometry
   * @param geometry The geometry variant
   */
  void cache_geometry_aabb(const std::string& obj_guid, const Geometry& geometry);
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
