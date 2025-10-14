#include "tree.h"
#include "fmt/format.h"
#include <algorithm>
#include <iostream>
#include <queue>
#include <sstream>
#include <stdexcept>

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////////
// Tree Implementation
///////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json Tree::jsondump() const {

  return nlohmann::ordered_json{
      {"type", "Tree"},
      {"guid", guid},
      {"name", name},
      {"root",
       _root ? _root->jsondump() : nlohmann::ordered_json(nullptr)}};
}

Tree Tree::jsonload(const nlohmann::json &data) {
  auto tree = std::make_shared<Tree>(data["name"]);
  tree->guid = data["guid"];
  if (!data["root"].is_null()) {
    auto root = TreeNode::jsonload(data["root"]);
    tree->add(root);
  }
  return *tree;
}

/// Serialize to JSON file

/// Deserialize from JSON file

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

std::string Tree::to_string() const { return fmt::format("Tree: {}", name); }

std::shared_ptr<TreeNode> Tree::root() const { return _root; }

void Tree::add(std::shared_ptr<TreeNode> node,
               std::shared_ptr<TreeNode> parent) {
  if (!node)
    throw std::invalid_argument("Cannot add null node");

  if (!parent) {
    // Add as root
    if (_root)
      throw std::runtime_error("Tree already has a root node");

    _root = node;
    // Set tree reference using raw pointer (since Tree is not shared_ptr
    // managed)
    node->_tree = std::weak_ptr<Tree>();
  } else {
    // Add as child to parent
    node->_tree = std::weak_ptr<Tree>();
    parent->add(node);
  }
}

std::vector<std::shared_ptr<TreeNode>> Tree::nodes() const {
  std::vector<std::shared_ptr<TreeNode>> result;
  if (!_root)
    return result;

  // Use a queue for breadth-first traversal to collect all shared_ptrs
  std::queue<std::shared_ptr<TreeNode>> queue;
  queue.push(_root);

  while (!queue.empty()) {
    auto current = queue.front();
    queue.pop();
    result.push_back(current);

    // Add children to queue
    for (const auto &child : current->_children) {
      queue.push(child);
    }
  }

  return result;
}

std::shared_ptr<TreeNode> Tree::remove(std::shared_ptr<TreeNode> node) {
  if (!node) {
    throw std::invalid_argument("Cannot remove null node");
  }

  // If removing the root
  if (node == _root) {
    auto removed = _root;
    _root.reset();
    removed->_tree.reset();
    return removed;
  } else {
    // Get parent and remove the node from its children
    if (auto parent = node->parent()) {
      return parent->remove(node);
    }
    // If node has no parent, it's not in this tree
    throw std::invalid_argument("Node is not in this tree");
  }
}

std::vector<std::shared_ptr<TreeNode>> Tree::leaves() const {
  std::vector<std::shared_ptr<TreeNode>> result;
  if (_root) {
    for (const auto &node : nodes()) {
      if (node->is_leaf()) {
        result.push_back(node);
      }
    }
  }
  return result;
}

std::vector<std::shared_ptr<TreeNode>>
Tree::traverse(const std::string &strategy, const std::string &order) const {
  std::vector<std::shared_ptr<TreeNode>> result;
  if (!_root)
    return result;

  auto raw_nodes = _root->traverse(strategy, order);
  for (auto raw_node : raw_nodes) {
    // Find the corresponding shared_ptr for each raw pointer
    for (const auto &node : nodes()) {
      if (node.get() == raw_node) {
        result.push_back(node);
        break;
      }
    }
  }
  return result;
}

std::shared_ptr<TreeNode>
Tree::get_node_by_name(const std::string &node_name) const {
  for (const auto &node : nodes()) {
    if (node->name == node_name) {
      return node;
    }
  }
  return nullptr;
}

std::vector<std::shared_ptr<TreeNode>>
Tree::get_nodes_by_name(const std::string &node_name) const {
  std::vector<std::shared_ptr<TreeNode>> result;
  for (const auto &node : nodes()) {
    if (node->name == node_name) {
      result.push_back(node);
    }
  }
  return result;
}

std::shared_ptr<TreeNode>
Tree::find_node_by_guid(const std::string &node_guid) const {
  for (const auto &node : nodes()) {
    if (node->guid == node_guid) {
      return node;
    }
  }
  return nullptr;
}

bool Tree::add_child_by_guid(const std::string &parent_guid,
                             const std::string &child_guid) {
  auto parent_node = find_node_by_guid(parent_guid);
  auto child_node = find_node_by_guid(child_guid);

  if (!parent_node || !child_node) {
    return false;
  }

  // Remove child from its current parent if it has one
  if (auto current_parent = child_node->parent()) {
    current_parent->remove(child_node);
  } else {
    // Child is not currently in any parent, we can't move it
    return false;
  }

  // Add to new parent
  parent_node->add(child_node);
  return true;
}

std::vector<std::string>
Tree::get_children_guids(const std::string &node_guid) const {
  auto node = find_node_by_guid(node_guid);
  if (!node) {
    return {};
  }

  std::vector<std::string> result;
  for (const auto &child : node->children()) {
    result.push_back(child->guid);
  }
  return result;
}

void Tree::print_hierarchy() const {
  if (!_root) {
    std::cout << "Empty tree" << std::endl;
    return;
  }

  // Lambda function for recursive printing
  std::function<void(TreeNode *, const std::string &, bool)> print_node =
      [&](TreeNode *node, const std::string &prefix, bool is_last) {
        if (!node)
          return;

        // Print current node
        std::cout << prefix << (is_last ? "└── " : "├── ") << node->to_string()
                  << std::endl;

        // New prefix for children
        std::string new_prefix = prefix + (is_last ? "    " : "│   ");

        // Print children
        const auto &children = node->children();
        for (size_t i = 0; i < children.size(); ++i) {
          bool is_last_child = (i == children.size() - 1);
          print_node(children[i], new_prefix, is_last_child);
        }
      };

  // Start recursive printing from root
  print_node(_root.get(), "", true);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Tree &tree) {
  os << tree.to_string();
  return os;
}

} // namespace session_cpp
