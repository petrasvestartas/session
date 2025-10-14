#include "treenode.h"
#include "tree.h"
#include "fmt/format.h"
#include <algorithm>
#include <iostream>
#include <queue>
#include <sstream>
#include <stdexcept>

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert to JSON-serializable object
nlohmann::ordered_json TreeNode::jsondump() const {

  nlohmann::ordered_json children_array = nlohmann::ordered_json::array();
  for (const auto &child : _children)
    children_array.push_back(child->jsondump());

  return nlohmann::ordered_json{{"type", "TreeNode"},
                                {"guid", guid},
                                {"name", name},
                                {"children", children_array}};
}

/// Create TreeNode from JSON data
std::shared_ptr<TreeNode> TreeNode::jsonload(const nlohmann::json &data) {
  auto node = std::make_shared<TreeNode>(data["name"]);
  node->guid = data["guid"];

  // Recursively create children (matching Python behavior)
  for (const auto &child_data : data["children"]) {
    auto child = TreeNode::jsonload(child_data);
    node->add(child);
  }

  return node;
}

///////////////////////////////////////////////////////////////////////////////////////////
// TreeNode Implementation
///////////////////////////////////////////////////////////////////////////////////////////

std::string TreeNode::to_string() const {
  return fmt::format("TreeNode({}, {}, {} children)", this->name, this->guid,
                     _children.size());
}

bool TreeNode::operator==(const TreeNode &other) const {
  return this->guid == other.guid;
}

bool TreeNode::operator!=(const TreeNode &other) const {
  return !(*this == other);
}

bool TreeNode::is_root() const { return _parent.expired(); }

bool TreeNode::is_leaf() const { return _children.empty(); }

std::shared_ptr<TreeNode> TreeNode::parent() const { return _parent.lock(); }

std::vector<TreeNode *> TreeNode::children() const {
  std::vector<TreeNode *> result;
  for (const auto &child : _children) {
    result.push_back(child.get());
  }
  return result;
}

Tree *TreeNode::tree() const { return _tree.lock().get(); }

void TreeNode::add(std::shared_ptr<TreeNode> child) {
  if (!child)
    return;

  // Always require shared_from_this() to work - this enforces proper shared_ptr
  // usage
  child->_parent = shared_from_this();
  child->_tree = _tree;
  _children.push_back(child);
}

std::shared_ptr<TreeNode> TreeNode::remove(std::shared_ptr<TreeNode> child) {
  auto it = std::find_if(
      _children.begin(), _children.end(),
      [child](const std::shared_ptr<TreeNode> &ptr) { return ptr == child; });

  if (it != _children.end()) {
    auto removed = *it;
    _children.erase(it);
    removed->_parent.reset();
    return removed;
  }
  return nullptr;
}

std::vector<TreeNode *> TreeNode::ancestors() const {
  std::vector<TreeNode *> result;
  auto current = _parent.lock();
  while (current) {
    result.push_back(current.get());
    current = current->_parent.lock();
  }
  return result;
}

std::vector<TreeNode *> TreeNode::descendants() const {
  std::vector<TreeNode *> result;
  for (const auto &child : _children) {
    result.push_back(child.get());
    auto child_descendants = child->descendants();
    result.insert(result.end(), child_descendants.begin(),
                  child_descendants.end());
  }
  return result;
}

std::vector<TreeNode *> TreeNode::traverse(const std::string &strategy,
                                           const std::string &order) const {
  std::vector<TreeNode *> result;

  if (strategy == "depthfirst") {
    if (order == "preorder") {
      result.push_back(const_cast<TreeNode *>(this));
      for (const auto &child : _children) {
        auto child_result = child->traverse(strategy, order);
        result.insert(result.end(), child_result.begin(), child_result.end());
      }
    } else if (order == "postorder") {
      for (const auto &child : _children) {
        auto child_result = child->traverse(strategy, order);
        result.insert(result.end(), child_result.begin(), child_result.end());
      }
      result.push_back(const_cast<TreeNode *>(this));
    }
  } else if (strategy == "breadthfirst") {
    std::queue<TreeNode *> queue;
    queue.push(const_cast<TreeNode *>(this));
    while (!queue.empty()) {
      auto current = queue.front();
      queue.pop();
      result.push_back(current);
      for (const auto &child : current->_children) {
        queue.push(child.get());
      }
    }
  } else {
    throw std::invalid_argument("Unknown traversal strategy: " + strategy);
  }

  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const TreeNode &node) {
  os << node.to_string();
  return os;
}

} // namespace session_cpp
