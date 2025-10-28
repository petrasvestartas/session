#include "session.h"
#include "graph.h"
#include "tree.h"
#include "intersection.h"
#include <algorithm>
#include <functional>

/**
 * ADDING NEW GEOMETRY TYPES - CHECKLIST
 * =====================================
 * 
 * When adding a new geometry type (e.g., Sphere, Cone, etc.), update:
 * 
 * 1. SESSION.H:
 *    - Add to Geometry variant (e.g., std::shared_ptr<Sphere>)
 *    - Add collection to Objects class (e.g., std::shared_ptr<std::vector<std::shared_ptr<Sphere>>> spheres)
 *    - Add to Objects constructor initialization
 *    - Declare Session::add_XXX() method
 * 
 * 2. SESSION.CPP:
 *    - Implement Session::add_XXX() method
 *    - Add case in Session::compute_bounding_box() visitor
 *    - Add case in Session::ray_intersect_geometry() visitor
 *    - Add collection to Session::get_geometry() transformation loop
 *    - Add collection to Session::jsonload() rebuild lookup
 * 
 * 3. OBJECTS.H/CPP:
 *    - Add collection member and initialization
 *    - Add to Objects::jsondump()
 *    - Add to Objects::jsonload()
 *    - Add to Objects::to_string()
 * 
 * 4. INTERSECTION.H/CPP (if needed):
 *    - Implement ray-geometry intersection method
 *    - Consider adding to existing methods if applicable
 * 
 * Critical visitor patterns to update:
 * - compute_bounding_box(): Define AABB for collision detection
 * - ray_intersect_geometry(): Define precise ray intersection logic
 * - get_geometry(): Apply transformations to new type
 */

namespace session_cpp {

std::string Session::to_string() const {
  return fmt::format("Session(name={}, objects={}, tree={}, graph={})", name,
                     objects.to_string(), tree.to_string(), graph.to_string());
}

// Geometry Management

std::shared_ptr<TreeNode> Session::add_point(std::shared_ptr<Point> point) {
  objects.points->push_back(point);
  lookup[point->guid] = point;
  graph.add_node(point->guid, "point_" + point->name);
  cache_geometry_aabb(point->guid, point);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(point->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_line(std::shared_ptr<Line> line) {
  objects.lines->push_back(line);
  lookup[line->guid] = line;
  graph.add_node(line->guid, "line_" + line->name);
  cache_geometry_aabb(line->guid, line);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(line->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_plane(std::shared_ptr<Plane> plane) {
  objects.planes->push_back(plane);
  lookup[plane->guid] = plane;
  graph.add_node(plane->guid, "plane_" + plane->name);
  cache_geometry_aabb(plane->guid, plane);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(plane->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_bbox(std::shared_ptr<BoundingBox> bbox) {
  objects.bboxes->push_back(bbox);
  lookup[bbox->guid] = bbox;
  graph.add_node(bbox->guid, "bbox_" + bbox->name);
  cache_geometry_aabb(bbox->guid, bbox);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(bbox->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_polyline(std::shared_ptr<Polyline> polyline) {
  objects.polylines->push_back(polyline);
  lookup[polyline->guid] = polyline;
  graph.add_node(polyline->guid, "polyline_" + polyline->name);
  cache_geometry_aabb(polyline->guid, polyline);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(polyline->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_pointcloud(std::shared_ptr<PointCloud> pointcloud) {
  objects.pointclouds->push_back(pointcloud);
  lookup[pointcloud->guid] = pointcloud;
  graph.add_node(pointcloud->guid, "pointcloud_" + pointcloud->name);
  cache_geometry_aabb(pointcloud->guid, pointcloud);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(pointcloud->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_mesh(std::shared_ptr<Mesh> mesh) {
  objects.meshes->push_back(mesh);
  lookup[mesh->guid] = mesh;
  graph.add_node(mesh->guid, "mesh_" + mesh->name);
  cache_geometry_aabb(mesh->guid, mesh);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(mesh->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_cylinder(std::shared_ptr<Cylinder> cylinder) {
  objects.cylinders->push_back(cylinder);
  lookup[cylinder->guid] = cylinder;
  graph.add_node(cylinder->guid, "cylinder_" + cylinder->name);
  cache_geometry_aabb(cylinder->guid, cylinder);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(cylinder->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_arrow(std::shared_ptr<Arrow> arrow) {
  objects.arrows->push_back(arrow);
  lookup[arrow->guid] = arrow;
  graph.add_node(arrow->guid, "arrow_" + arrow->name);
  cache_geometry_aabb(arrow->guid, arrow);  // Incremental AABB caching
  auto tree_node = std::make_shared<TreeNode>(arrow->guid);
  return tree_node;
}

void Session::add(std::shared_ptr<TreeNode> node, 
                  std::shared_ptr<TreeNode> parent) {
  if (parent == nullptr) {
    tree.add(node, tree.root());
  } else {
    tree.add(node, parent);
  }
}

void Session::add_edge(const std::string &guid1, const std::string &guid2,
                       const std::string &attribute) {
  graph.add_edge(guid1, guid2, attribute);
}

bool Session::remove_object(const std::string &obj_guid) {
  auto it = lookup.find(obj_guid);
  if (it == lookup.end()) {
    return false;
  }

  invalidate_bvh_cache();

  // Determine type and remove from typed collection
  std::visit(
      [this](const auto &ptr) {
        using T = std::decay_t<decltype(*ptr)>;
        if constexpr (std::is_same_v<T, Point>) {
          auto &points = *objects.points;
          points.erase(std::remove_if(
                           points.begin(), points.end(),
                           [&](const auto &p) { return p->guid == ptr->guid; }),
                       points.end());
        }
        // Add other types when Objects class supports them
      },
      it->second);

  // Remove from lookup table
  lookup.erase(it);

  // Remove from tree
  auto tree_node = tree.find_node_by_guid(obj_guid);
  if (tree_node) {
    tree.remove(tree_node);
  }

  // Remove from graph
  if (graph.has_node(obj_guid)) {
    graph.remove_node(obj_guid);
  }

  return true;
}

// Tree Operations

bool Session::add_hierarchy(const std::string &parent_guid,
                            const std::string &child_guid) {
  return tree.add_child_by_guid(parent_guid, child_guid);
}

std::vector<std::string> Session::get_children(const std::string &obj_guid) const {
  return tree.get_children_guids(obj_guid);
}

// Graph Operations

void Session::add_relationship(const std::string &from_guid,
                               const std::string &to_guid,
                               const std::string &relationship_type) {
  graph.add_edge(from_guid, to_guid, relationship_type);
}

std::vector<std::string> Session::get_neighbours(const std::string &obj_guid) {
  return graph.neighbors(obj_guid);
}

// BVH Collision Detection

// ADD NEW GEOMETRY TYPES HERE: Add bounding box computation for collision detection
BoundingBox Session::compute_bounding_box(const Geometry& geometry) {
  double inflate = Tolerance::APPROXIMATION;
  
  return std::visit([inflate](auto&& geom_ptr) -> BoundingBox {
    using T = std::decay_t<decltype(geom_ptr)>;
    
    if constexpr (std::is_same_v<T, std::shared_ptr<Point>>) {
      return BoundingBox::from_point(*geom_ptr, inflate);
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Line>>) {
      std::vector<Point> points = {geom_ptr->start(), geom_ptr->end()};
      return BoundingBox::from_points(points, inflate);
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Polyline>>) {
      return BoundingBox::from_points(geom_ptr->points, inflate);
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<PointCloud>>) {
      return BoundingBox::from_points(geom_ptr->points, inflate);
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Mesh>>) {
      // Extract vertices from mesh
      std::vector<Point> points;
      for (const auto& [key, vertex] : geom_ptr->vertex) {
        points.push_back(Point(vertex.x, vertex.y, vertex.z));
      }
      if (points.empty()) {
        return BoundingBox::from_point(Point(0, 0, 0), inflate);
      }
      return BoundingBox::from_points(points, inflate);
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<BoundingBox>>) {
      // Inflate existing bounding box
      auto inflated = *geom_ptr;
      inflated.half_size = Vector(
        inflated.half_size.x() + inflate,
        inflated.half_size.y() + inflate,
        inflated.half_size.z() + inflate
      );
      return inflated;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Plane>>) {
      // Create bounded box around plane origin
      return BoundingBox::from_point(geom_ptr->origin(), inflate * 10.0);
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Cylinder>>) {
      // Compute from cylinder line endpoints and radius
      std::vector<Point> points = {geom_ptr->line.start(), geom_ptr->line.end()};
      auto bbox = BoundingBox::from_points(points, inflate);
      // Inflate by cylinder radius
      bbox.half_size = Vector(
        bbox.half_size.x() + geom_ptr->radius,
        bbox.half_size.y() + geom_ptr->radius,
        bbox.half_size.z() + geom_ptr->radius
      );
      return bbox;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Arrow>>) {
      // Compute from arrow line endpoints
      std::vector<Point> points = {geom_ptr->line.start(), geom_ptr->line.end()};
      auto bbox = BoundingBox::from_points(points, inflate);
      // Inflate by arrow radius
      bbox.half_size = Vector(
        bbox.half_size.x() + geom_ptr->radius,
        bbox.half_size.y() + geom_ptr->radius,
        bbox.half_size.z() + geom_ptr->radius
      );
      return bbox;
    }
    else {
      // Fallback - ADD YOUR NEW GEOMETRY TYPE ABOVE THIS LINE
      return BoundingBox::from_point(Point(0, 0, 0), inflate);
    }
  }, geometry);
}

std::vector<std::pair<std::string, std::string>> Session::get_collisions() {
  // Collect all objects with their bounding boxes and GUIDs
  std::vector<BoundingBox> boxes;
  std::vector<std::string> guids;
  boxes.reserve(lookup.size());
  guids.reserve(lookup.size());
  
  for (const auto& [g, geometry] : lookup) {
    BoundingBox bbox = compute_bounding_box(geometry);
    boxes.push_back(bbox);
    guids.push_back(g);
  }
  
  if (boxes.empty()) {
    return {};
  }
  
  // Build BVH and check collisions
  double world_size = BVH::compute_world_size(boxes);
  bvh = BVH::from_boxes(boxes, world_size);
  auto [collision_pairs, colliding_indices, checks] = bvh.check_all_collisions(boxes);
  (void)colliding_indices;
  (void)checks;
  
  // Map index pairs to GUID pairs
  std::vector<std::pair<std::string, std::string>> guid_pairs;
  guid_pairs.reserve(collision_pairs.size());
  for (const auto& [i, j] : collision_pairs) {
    if (i >= 0 && i < static_cast<int>(guids.size()) && 
        j >= 0 && j < static_cast<int>(guids.size())) {
      guid_pairs.emplace_back(guids[i], guids[j]);
    }
  }
  
  // Add collision edges to graph
  for (const auto& [guid1, guid2] : guid_pairs) {
    graph.add_edge(guid1, guid2, "bvh_collision");
  }
  
  return guid_pairs;
}

// Transformed Geometry

Objects Session::get_geometry() const {
  // Deep copy all objects
  Objects transformed_objects = objects;
  
  // Rebuild lookup from copied objects
  std::unordered_map<std::string, Geometry> transformed_lookup;
  
  auto add_to_lookup = [&](auto& collection) {
    for (auto& geom : *collection) {
      transformed_lookup[geom->guid] = geom;
    }
  };
  
  add_to_lookup(transformed_objects.points);
  add_to_lookup(transformed_objects.lines);
  add_to_lookup(transformed_objects.planes);
  add_to_lookup(transformed_objects.bboxes);
  add_to_lookup(transformed_objects.polylines);
  add_to_lookup(transformed_objects.pointclouds);
  add_to_lookup(transformed_objects.meshes);
  add_to_lookup(transformed_objects.cylinders);
  add_to_lookup(transformed_objects.arrows);
  
  // Helper lambda to recursively transform nodes
  std::function<void(std::shared_ptr<TreeNode>, const Xform&)> transform_node = 
    [&](std::shared_ptr<TreeNode> node, const Xform& parent_xform) {
      // Get geometry from the lookup
      auto it = transformed_lookup.find(node->name);
      
      Xform current_xform = parent_xform;
      
      if (it != transformed_lookup.end()) {
        // Transform in-place
        std::visit([&](auto&& geom_ptr) {
          geom_ptr->xform = parent_xform * geom_ptr->xform;
          current_xform = geom_ptr->xform;
        }, it->second);
      }
      
      // Recursively process children
      for (auto* child : node->children()) {
        transform_node(child->shared_from_this(), current_xform);
      }
    };
  
  // Start from root with identity transformation
  if (tree.root()) {
    transform_node(tree.root(), Xform::identity());
  }
  
  // Apply accumulated transformations to actual geometry coordinates
  for (auto& point : *transformed_objects.points) {
    point->transform();
  }
  for (auto& line : *transformed_objects.lines) {
    line->transform();
  }
  for (auto& plane : *transformed_objects.planes) {
    plane->transform();
  }
  for (auto& bbox : *transformed_objects.bboxes) {
    bbox->transform();
  }
  for (auto& polyline : *transformed_objects.polylines) {
    polyline->transform();
  }
  for (auto& pointcloud : *transformed_objects.pointclouds) {
    pointcloud->transform();
  }
  for (auto& mesh : *transformed_objects.meshes) {
    mesh->transform();
  }
  for (auto& cylinder : *transformed_objects.cylinders) {
    cylinder->transform();
  }
  for (auto& arrow : *transformed_objects.arrows) {
    arrow->transform();
  }
  
  return transformed_objects;
}

// JSON Serialization

nlohmann::ordered_json Session::jsondump() const {
  nlohmann::ordered_json data;
  data["type"] = "Session";
  data["name"] = name;
  data["guid"] = guid;
  data["objects"] = objects.jsondump();
  data["tree"] = tree.jsondump();
  data["graph"] = graph.jsondump();
  return data;
}

Session Session::jsonload(const nlohmann::json &data) {
  Session session(data.value("name", "my_session"));

  // Load objects
  if (data.contains("objects")) {
    session.objects = Objects::jsonload(data["objects"]);
  }

  // Rebuild lookup from all objects
  for (const auto &arrow_ptr : *session.objects.arrows) {
    session.lookup[arrow_ptr->guid] = arrow_ptr;
  }
  for (const auto &bbox_ptr : *session.objects.bboxes) {
    session.lookup[bbox_ptr->guid] = bbox_ptr;
  }
  for (const auto &cylinder_ptr : *session.objects.cylinders) {
    session.lookup[cylinder_ptr->guid] = cylinder_ptr;
  }
  for (const auto &line_ptr : *session.objects.lines) {
    session.lookup[line_ptr->guid] = line_ptr;
  }
  for (const auto &mesh_ptr : *session.objects.meshes) {
    session.lookup[mesh_ptr->guid] = mesh_ptr;
  }
  for (const auto &plane_ptr : *session.objects.planes) {
    session.lookup[plane_ptr->guid] = plane_ptr;
  }
  for (const auto &point_ptr : *session.objects.points) {
    session.lookup[point_ptr->guid] = point_ptr;
  }
  for (const auto &pointcloud_ptr : *session.objects.pointclouds) {
    session.lookup[pointcloud_ptr->guid] = pointcloud_ptr;
  }
  for (const auto &polyline_ptr : *session.objects.polylines) {
    session.lookup[polyline_ptr->guid] = polyline_ptr;
  }

  // Load tree structure
  if (data.contains("tree")) {
    session.tree = Tree::jsonload(data["tree"]);
  }

  // Load graph structure
  if (data.contains("graph")) {
    session.graph = Graph::jsonload(data["graph"]);
  }

  return session;
}

// Ray Intersection

void Session::cache_geometry_aabb(const std::string& obj_guid, const Geometry& geometry) {
  // Compute and cache bounding box incrementally
  cached_boxes.push_back(compute_bounding_box(geometry));
  cached_guids.push_back(obj_guid);
  bvh_cache_dirty = true;  // Mark BVH as needing rebuild
}

void Session::rebuild_ray_bvh_cache() {
  // Fast rebuild: just reconstruct BVH from already-cached boxes
  // (AABBs were computed incrementally when geometry was added)
  
  if (cached_boxes.size() != lookup.size()) {
    // Cache is invalid (geometry removed or cache empty), full rebuild needed
    cached_boxes.clear();
    cached_guids.clear();
    cached_boxes.reserve(lookup.size());
    cached_guids.reserve(lookup.size());
    
    // Compute and cache all bounding boxes
    for (const auto& [g, geometry] : lookup) {
      cached_boxes.push_back(compute_bounding_box(geometry));
      cached_guids.push_back(g);
    }
  }
  
  // Build BVH from cached boxes (FAST: ~1ms for 10k boxes)
  if (!cached_boxes.empty()) {
    double world_size = BVH::compute_world_size(cached_boxes);
    cached_ray_bvh = BVH::from_boxes(cached_boxes, world_size);
  }
}

std::vector<Session::RayHit> Session::ray_cast(const Point& origin, const Vector& direction, double tolerance) {
  // Rebuild BVH cache if dirty (geometry added/removed)
  if (bvh_cache_dirty) {
    rebuild_ray_bvh_cache();
    bvh_cache_dirty = false;
  }
  
  if (cached_guids.empty()) return {};
  
  // BVH OPTIMIZATION: Get candidate indices from CACHED BVH ray traversal
  // This prunes objects whose AABBs don't intersect the ray, providing
  // acceleration for ALL geometry types (Point, Line, Mesh, Cylinder, etc.)
  std::vector<int> candidate_ids;
  cached_ray_bvh.ray_cast(origin, direction, candidate_ids, true);
  
  // Test candidates with precise geometry intersection and track closest hit
  std::vector<RayHit> hits;
  Line ray = Line::from_points(origin, origin + direction * 10000.0);  // Long ray
  double closest_dist = std::numeric_limits<double>::infinity();
  
  for (int idx : candidate_ids) {
    const std::string& obj_guid = cached_guids[idx];
    const Geometry& geom = lookup[obj_guid];
    
    std::optional<Point> hit = ray_intersect_geometry(ray, geom, tolerance);
    if (hit) {
      double dist = origin.distance(*hit);
      
      // Only keep hits closer than current closest
      if (dist < closest_dist) {
        // Clear previous hits if this is closer
        if (dist < closest_dist - tolerance) {
          hits.clear();
        }
        hits.push_back({guid, *hit, dist});
        closest_dist = dist;
      }
    }
  }
  
  // Already sorted by discovery order (closest first)
  return hits;
}

// ADD NEW GEOMETRY TYPES HERE: Add ray intersection logic for precise ray casting
std::optional<Point> Session::ray_intersect_geometry(const Line& ray, const Geometry& geometry, double tolerance) {
  return std::visit([&](auto&& geom_ptr) -> std::optional<Point> {
    using T = std::decay_t<decltype(geom_ptr)>;
    
    if constexpr (std::is_same_v<T, std::shared_ptr<Point>>) {
      // Ray-point: parametric distance calculation
      Vector ray_dir = ray.end() - ray.start();
      Vector to_point = *geom_ptr - ray.start();
      double t = to_point.dot(ray_dir) / ray_dir.dot(ray_dir);
      if (t < 0) return std::nullopt;  // Point is behind ray
      Point closest = ray.start() + ray_dir * t;
      double dist = geom_ptr->distance(closest);
      return (dist <= tolerance) ? std::optional{closest} : std::nullopt;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Line>>) {
      // Ray-line intersection
      Point hit;
      if (Intersection::line_line(ray, *geom_ptr, hit, tolerance)) {
        return hit;
      }
      return std::nullopt;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Plane>>) {
      // Ray-plane intersection
      Point hit;
      if (Intersection::line_plane(ray, *geom_ptr, hit, tolerance)) {
        return hit;
      }
      return std::nullopt;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Polyline>>) {
      // Ray-polyline: test each segment, return closest
      std::optional<Point> closest_hit;
      double min_dist = std::numeric_limits<double>::infinity();
      
      for (size_t i = 0; i < geom_ptr->segment_count(); ++i) {
        const Point& p0 = geom_ptr->points[i];
        const Point& p1 = geom_ptr->points[i + 1];
        Line seg = Line::from_points(p0, p1);
        Point hit;
        if (Intersection::line_line(ray, seg, hit, tolerance)) {
          double dist = ray.start().distance(hit);
          if (dist < min_dist) {
            min_dist = dist;
            closest_hit = hit;
          }
        }
      }
      return closest_hit;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<PointCloud>>) {
      // Ray-pointcloud: find closest point within tolerance
      std::optional<Point> closest_hit;
      double min_dist = std::numeric_limits<double>::infinity();
      Vector ray_dir = ray.end() - ray.start();
      
      for (const Point& pt : geom_ptr->points) {
        Vector to_point = pt - ray.start();
        double t = to_point.dot(ray_dir) / ray_dir.dot(ray_dir);
        if (t < 0) continue;  // Point is behind ray
        Point closest = ray.start() + ray_dir * t;
        double dist = pt.distance(closest);
        if (dist <= tolerance && dist < min_dist) {
          min_dist = dist;
          closest_hit = closest;
        }
      }
      return closest_hit;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Mesh>>) {
      // Ray-mesh: use triangle BVH
      std::vector<Point> hits = Intersection::ray_mesh_bvh(ray, *geom_ptr, tolerance, true);
      if (!hits.empty()) {
        return hits[0];  // Return closest
      }
      return std::nullopt;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<BoundingBox>>) {
      // Ray-bbox: use slab test to find entry point
      double tmin, tmax;
      if (Intersection::ray_box(ray.start(), ray.end() - ray.start(), *geom_ptr, 0.0, 1.0, tmin, tmax)) {
        // Return entry point (closest intersection)
        Vector ray_dir = ray.end() - ray.start();
        Point hit = ray.start() + ray_dir * tmin;
        return hit;
      }
      return std::nullopt;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Cylinder>>) {
      // Ray-cylinder: check distance to cylinder axis
      const Line& cyl_axis = geom_ptr->line;
      double radius = geom_ptr->radius;
      
      // Find closest points between ray and cylinder axis
      double t_ray, t_cyl;
      if (Intersection::line_line_parameters(ray, cyl_axis, t_ray, t_cyl, tolerance, true, false)) {
        // Check if within cylinder bounds
        if (t_cyl >= 0.0 && t_cyl <= 1.0) {
          Point point_on_ray = ray.point_at(t_ray);
          Point point_on_axis = cyl_axis.point_at(t_cyl);
          double dist = point_on_ray.distance(point_on_axis);
          
          if (dist <= radius + tolerance) {
            return point_on_ray;
          }
        }
      }
      
      return std::nullopt;
    }
    else if constexpr (std::is_same_v<T, std::shared_ptr<Arrow>>) {
      // Ray-arrow: same logic as cylinder
      const Line& arrow_axis = geom_ptr->line;
      double radius = geom_ptr->radius;
      
      // Find closest points between ray and arrow axis
      double t_ray, t_arrow;
      if (Intersection::line_line_parameters(ray, arrow_axis, t_ray, t_arrow, tolerance, true, false)) {
        // Check if within arrow bounds
        if (t_arrow >= 0.0 && t_arrow <= 1.0) {
          Point point_on_ray = ray.point_at(t_ray);
          Point point_on_axis = arrow_axis.point_at(t_arrow);
          double dist = point_on_ray.distance(point_on_axis);
          
          if (dist <= radius + tolerance) {
            return point_on_ray;
          }
        }
      }
      
      return std::nullopt;
    }
    else {
      // ADD YOUR NEW GEOMETRY TYPE ABOVE THIS LINE
      return std::nullopt;
    }
  }, geometry);
}

std::ostream &operator<<(std::ostream &os, const Session &session) {
  return os << session.to_string();
}

} // namespace session_cpp