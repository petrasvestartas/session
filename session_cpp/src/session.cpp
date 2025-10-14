#include "session.h"
#include "graph.h"
#include "tree.h"
#include <algorithm>

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
  auto tree_node = std::make_shared<TreeNode>(point->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_line(std::shared_ptr<Line> line) {
  objects.lines->push_back(line);
  lookup[line->guid] = line;
  graph.add_node(line->guid, "line_" + line->name);
  auto tree_node = std::make_shared<TreeNode>(line->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_plane(std::shared_ptr<Plane> plane) {
  objects.planes->push_back(plane);
  lookup[plane->guid] = plane;
  graph.add_node(plane->guid, "plane_" + plane->name);
  auto tree_node = std::make_shared<TreeNode>(plane->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_bbox(std::shared_ptr<BoundingBox> bbox) {
  objects.bboxes->push_back(bbox);
  lookup[bbox->guid] = bbox;
  graph.add_node(bbox->guid, "bbox_" + bbox->name);
  auto tree_node = std::make_shared<TreeNode>(bbox->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_polyline(std::shared_ptr<Polyline> polyline) {
  objects.polylines->push_back(polyline);
  lookup[polyline->guid] = polyline;
  graph.add_node(polyline->guid, "polyline_" + polyline->name);
  auto tree_node = std::make_shared<TreeNode>(polyline->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_pointcloud(std::shared_ptr<PointCloud> pointcloud) {
  objects.pointclouds->push_back(pointcloud);
  lookup[pointcloud->guid] = pointcloud;
  graph.add_node(pointcloud->guid, "pointcloud_" + pointcloud->name);
  auto tree_node = std::make_shared<TreeNode>(pointcloud->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_mesh(std::shared_ptr<Mesh> mesh) {
  objects.meshes->push_back(mesh);
  lookup[mesh->guid] = mesh;
  graph.add_node(mesh->guid, "mesh_" + mesh->name);
  auto tree_node = std::make_shared<TreeNode>(mesh->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_cylinder(std::shared_ptr<Cylinder> cylinder) {
  objects.cylinders->push_back(cylinder);
  lookup[cylinder->guid] = cylinder;
  graph.add_node(cylinder->guid, "cylinder_" + cylinder->name);
  auto tree_node = std::make_shared<TreeNode>(cylinder->guid);
  return tree_node;
}

std::shared_ptr<TreeNode> Session::add_arrow(std::shared_ptr<Arrow> arrow) {
  objects.arrows->push_back(arrow);
  lookup[arrow->guid] = arrow;
  graph.add_node(arrow->guid, "arrow_" + arrow->name);
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

bool Session::remove_object(const std::string &guid) {
  auto it = lookup.find(guid);
  if (it == lookup.end()) {
    return false;
  }

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
  auto tree_node = tree.find_node_by_guid(guid);
  if (tree_node) {
    tree.remove(tree_node);
  }

  // Remove from graph
  if (graph.has_node(guid)) {
    graph.remove_node(guid);
  }

  return true;
}

// Tree Operations

bool Session::add_hierarchy(const std::string &parent_guid,
                            const std::string &child_guid) {
  return tree.add_child_by_guid(parent_guid, child_guid);
}

std::vector<std::string> Session::get_children(const std::string &guid) const {
  return tree.get_children_guids(guid);
}

// Graph Operations

void Session::add_relationship(const std::string &from_guid,
                               const std::string &to_guid,
                               const std::string &relationship_type) {
  graph.add_edge(from_guid, to_guid, relationship_type);
}

std::vector<std::string> Session::get_neighbours(const std::string &guid) {
  return graph.neighbors(guid);
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

std::ostream &operator<<(std::ostream &os, const Session &session) {
  return os << session.to_string();
}

} // namespace session_cpp