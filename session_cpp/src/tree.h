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
// Forward declarations
class Tree;

class TreeNode; // Forward declaration - Tree has TreeNode and TreeNode has Tree

/**
 * @class TreeNode
 * @brief A node of a tree data structure
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

  std::string guid = ::guid(); ///< Unique identifier for the node
  std::string name;            ///< Node identifier/name

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
  nlohmann::ordered_json to_json_data() const;

  /// Create TreeNode from JSON data
  static std::shared_ptr<TreeNode> from_json_data(const nlohmann::json &data);

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

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

/**
 * @brief  To use this operator, you can do:
 *         TreeNode node("my_node");
 *         std::cout << node << std::endl;
 * @param os The output stream.
 * @param node The TreeNode to insert into the stream.
 * @return A reference to the output stream.
 */
std::ostream &operator<<(std::ostream &os, const TreeNode &node);

/**
 * @class Tree
 * @brief A hierarchical data structure with parent-child relationships.
 */
class Tree {
  friend class TreeNode;

public:
  std::string guid = ::guid();  ///< Unique identifier for the node
  std::string name = "my_tree"; ///< Tree identifier/name

private:
  std::shared_ptr<TreeNode> _root; ///< Root node of the tree root node

public:
  Tree(std::string name = "my_tree") { this->name = name; }

  /// Convert point to string representation
  std::string to_string() const;

  /// Equality operator
  bool operator==(const Tree &other) const;

  /// Inequality operator
  bool operator!=(const Tree &other) const;

  ///////////////////////////////////////////////////////////////////////////////////////////
  // JSON
  ///////////////////////////////////////////////////////////////////////////////////////////

  /// Convert to JSON-serializable object
  nlohmann::ordered_json to_json_data() const;

  /// Create point from JSON data
  static Tree from_json_data(const nlohmann::json &data);

  /// Serialize to JSON file
  void to_json(const std::string &filepath) const;

  /// Deserialize from JSON file
  static Tree from_json(const std::string &filepath);

  ///////////////////////////////////////////////////////////////////////////////////////////
  // Details
  ///////////////////////////////////////////////////////////////////////////////////////////

  /**
   * @brief Get the root node of the tree.
   * @return The root node of the tree.
   */
  std::shared_ptr<TreeNode> root() const;

  /**
   * @brief Add a node to the tree.
   * @param node The node to add.
   * @param parent The parent node of the node to add. If nullptr, the node is
   * added as the root.
   */
  void add(std::shared_ptr<TreeNode> node,
           std::shared_ptr<TreeNode> parent = nullptr);

  /**
   * @brief Get all nodes in the tree.
   * @return A vector of all nodes in the tree.
   */
  std::vector<std::shared_ptr<TreeNode>> nodes() const;

  /**
   * @brief Remove a node from the tree.
   * @param node The node to remove.
   * @throws std::invalid_argument If node is null
   */
  std::shared_ptr<TreeNode> remove(std::shared_ptr<TreeNode> node);

  /**
   * @brief Get all leaf nodes in the tree.
   * @return A vector containing all leaf nodes (nodes without children).
   */
  std::vector<std::shared_ptr<TreeNode>> leaves() const;

  /**
   * @brief Traverse the tree from the root node.
   * @param strategy The traversal strategy ("depthfirst" or "breadthfirst")
   * @param order The traversal order ("preorder", "inorder", "postorder")
   * @return A vector of nodes in the specified traversal order.
   */
  std::vector<std::shared_ptr<TreeNode>>
  traverse(const std::string &strategy = "depthfirst",
           const std::string &order = "preorder") const;

  /**
   * @brief Get the first node with the specified name.
   * @param node_name The name of the node to find.
   * @return The first node with the specified name, or nullptr if not found.
   */
  std::shared_ptr<TreeNode>
  get_node_by_name(const std::string &node_name) const;

  /**
   * @brief Get all nodes with the specified name.
   * @param node_name The name of the nodes to find.
   * @return A vector of all nodes with the specified name.
   */
  std::vector<std::shared_ptr<TreeNode>>
  get_nodes_by_name(const std::string &node_name) const;

  /**
   * @brief Find a node by its GUID.
   * @param node_guid The GUID of the node to find.
   * @return The node with the specified GUID, or nullptr if not found.
   */
  std::shared_ptr<TreeNode>
  find_node_by_guid(const std::string &node_guid) const;

  /**
   * @brief Add a parent-child relationship using GUIDs.
   * @param parent_guid The GUID of the parent node
   * @param child_guid The GUID of the child node
   * @return True if the relationship was added, false if nodes not found
   */
  bool add_child_by_guid(const std::string &parent_guid,
                         const std::string &child_guid);

  /**
   * @brief Get all children GUIDs of a node by its GUID.
   * @param guid The GUID of the parent node
   * @return Vector of children GUIDs (empty if node not found or has no
   * children)
   */
  std::vector<std::string>
  get_children_guids(const std::string &node_guid) const;

  /**
   * @brief Print the spatial hierarchy of the tree.
   */
  void print_hierarchy() const;
};

/**
 * @brief  To use this operator, you can do:
 *         Tree tree("my_tree");
 *         std::cout << tree << std::endl;
 * @param os The output stream.
 * @param node The Tree to insert into the stream.
 * @return A reference to the output stream.
 *
 */
std::ostream &operator<<(std::ostream &os, const Tree &node);

} // namespace session_cpp
