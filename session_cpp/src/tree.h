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
#include "treenode.h"

namespace session_cpp {

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
  nlohmann::ordered_json jsondump() const;

  /// Create point from JSON data
  static Tree jsonload(const nlohmann::json &data);

  /// Serialize to JSON file

  /// Deserialize from JSON file

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
