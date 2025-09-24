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

void Session::add_point(std::shared_ptr<Point> point) {
  // Add to objects collection
  objects.points->push_back(point);

  // Add to lookup table
  lookup[point->guid] = point;

  // Automatically add to graph using point's GUID as node key
  graph.add_node(point->guid, "point_" + point->name);

  // Automatically add to tree as child of root using point's GUID as node name
  auto tree_node = std::make_shared<TreeNode>(point->guid);
  tree.add(tree_node, tree.root());
}

void Session::add_vector(std::shared_ptr<Vector> vector) {
  // Note: Objects class may need vectors collection - for now just add to
  // lookup objects.vectors->push_back(*vector);  // Uncomment when Objects has
  // vectors

  // Add to lookup table
  lookup[vector->guid] = vector;

  // Automatically add to graph using vector's GUID as node key
  graph.add_node(vector->guid, "vector_" + vector->name);

  // Automatically add to tree as child of root using vector's GUID as node name
  auto tree_node = std::make_shared<TreeNode>(vector->guid);
  tree.add(tree_node, tree.root());
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

nlohmann::ordered_json Session::to_json_data() {
  nlohmann::ordered_json data;
  data["type"] = "Session";
  data["name"] = name;
  data["guid"] = guid;
  data["objects"] = objects.to_json_data();
  data["tree"] = tree.to_json_data();
  data["graph"] = graph.to_json_data();
  return data;
}

Session Session::from_json_data(const nlohmann::json &data) {
  Session session(data.value("name", "my_session"));

  // Load objects
  if (data.contains("objects")) {
    session.objects = Objects::from_json_data(data["objects"]);
  }

  // Rebuild lookup from objects
  for (const auto &point_ptr : *session.objects.points) {
    session.lookup[point_ptr->guid] = point_ptr;
  }

  // Load tree structure
  if (data.contains("tree")) {
    session.tree = Tree::from_json_data(data["tree"]);
  }

  // Load graph structure
  if (data.contains("graph")) {
    session.graph = Graph::from_json_data(data["graph"]);
  }

  return session;
}

void Session::to_json(const std::string &filepath) {
  std::ofstream file(filepath);
  if (file.is_open()) {
    file << to_json_data().dump(4);
    file.close();
  }
}

Session Session::from_json(const std::string &filepath) {
  std::ifstream file(filepath);
  if (file.is_open()) {
    nlohmann::json data;
    file >> data;
    file.close();
    return from_json_data(data);
  }
  return Session(); // Return default session if file can't be opened
}

std::ostream &operator<<(std::ostream &os, const Session &session) {
  return os << session.to_string();
}

} // namespace session_cpp