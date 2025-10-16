#pragma once
#include <algorithm>
#include <fstream>
#include <functional>
#include <guid.h>
#include <iostream>
#include <json.h>
#include <memory>
#include <string>
#include <vector>

namespace session_cpp {

// Forward declaration - TreeNode needs to know Tree exists, but not its definition
class Tree;

/**
 * @class TreeNode
 * @brief A node of a tree data structure
 * 
 * TreeNodes can represent either:
 * - Geometry nodes: name is set to the geometry's GUID for lookup
 * - Organizational nodes: name is a descriptive string (e.g., "folder", "group")
 * 
 * When adding geometry to a Session, the TreeNode.name is automatically set to
 * the geometry.guid, allowing the tree hierarchy to reference geometry objects.
 */
class TreeNode : public std::enable_shared_from_this<TreeNode> {
  friend class Tree; // Allow Tree to access private members
private:
  std::weak_ptr<TreeNode> _parent; //< Non-owning pointer to parent
  std::vector<std::shared_ptr<TreeNode>>
      _children;             //< Owning pointers to children
  std::weak_ptr<Tree> _tree; //< Non-owning pointer to tree.

public:
  TreeNode(std::string name = "my_node") { this->name = name; }

  std::string guid = ::guid(); ///< Unique identifier for the tree node itself (distinct from geometry GUID)
  std::string name;            ///< Node identifier/name. For geometry nodes, this is the geometry's GUID

  /// Convert point to string representation
  std::string to_string() const;

  /// Equality operator
  bool operator==(const TreeNode &other) const;

  /// Inequality operator
  bool operator!=(const TreeNode &other) const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert to JSON-serializable object
  nlohmann::ordered_json jsondump() const;

  /// Create TreeNode from JSON data
  static std::shared_ptr<TreeNode> jsonload(const nlohmann::json &data);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details
  ///////////////////////////////////////////////////////////////////////////////////////////

  bool is_root() const;

  bool is_leaf() const;

  /// Get root node
  TreeNode *root() const;

  /// Get all nodes in the tree
  std::vector<TreeNode *> nodes() const;

  Tree *tree() const;

  /// @brief Add a child node to this node.
  /// @param child
  void add(std::shared_ptr<TreeNode> child);

  /// @brief Remove a child node from this node.
  /// @param child
  std::shared_ptr<TreeNode> remove(std::shared_ptr<TreeNode> child);

  /// @brief Get the parent node of this node.
  /// @return The parent node of this node. Returns nullptr if this node is the
  /// root node.
  std::shared_ptr<TreeNode> parent() const;

  /// @brief Get all ancestors of this node.
  /// @return A vector of all ancestors of this node.
  std::vector<TreeNode *> ancestors() const;

  /// @brief Get all the descendants of this node.
  /// @return A vector of all the descendants of this node.
  std::vector<TreeNode *> descendants() const;

  /// @brief Get the children of this node.
  /// @return A vector of raw pointers to the children of this node.
  std::vector<TreeNode *> children() const;

  /// @brief Get the parent of this node.
  /// @return Raw pointer to the parent node, or nullptr if this is the root.

  /// @brief Traverse the tree from this node.
  /// @param strategy The traversal strategy ("depthfirst" or "breadthfirst")
  /// @param order The traversal order ("preorder" or "postorder") - only for
  /// depth-first
  /// @return Vector of nodes in traversal order
  std::vector<TreeNode *> traverse(const std::string &strategy = "depthfirst",
                                   const std::string &order = "preorder") const;
};

/**
 * @brief  To use this operator, you can do:
 *         TreeNode node("my_node");
 *         std::cout << node << std::endl;
 * @param os The output stream.
 * @param node The TreeNode to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const TreeNode &node);

} // namespace session_cpp
