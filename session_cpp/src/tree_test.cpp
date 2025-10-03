#include "catch_amalgamated.hpp"
#include "point.h"
#include "tree.h"
#include <filesystem>
#include <fstream>

namespace session_cpp {

TEST_CASE("Tree test treenode constructor.") {
  TreeNode node = TreeNode("my_root");
  REQUIRE(node.name == "my_root");
  REQUIRE(node.is_root());
}

TEST_CASE("Tree test treenode to_json_data.") {
  auto root = std::make_shared<TreeNode>("project_root");
  auto folder1 = std::make_shared<TreeNode>("src");
  auto folder2 = std::make_shared<TreeNode>("docs");
  auto file1 = std::make_shared<TreeNode>("main.py");
  auto file2 = std::make_shared<TreeNode>("README.md");
  folder1->add(file1);
  folder2->add(file2);
  root->add(folder1);
  root->add(folder2);
  auto data = root->to_json_data();
  REQUIRE(data["name"] == "project_root");
  REQUIRE(data["type"] == "TreeNode");
  REQUIRE(data["children"].size() == 2);
  REQUIRE(data["children"][0]["name"] == "src");
  REQUIRE(data["children"][0]["children"].size() == 1);
}

TEST_CASE("Tree test treenode from_json_data.") {
  auto original_root = std::make_shared<TreeNode>("filesystem_root");
  auto folder1 = std::make_shared<TreeNode>("src");
  auto folder2 = std::make_shared<TreeNode>("docs");
  auto file1 = std::make_shared<TreeNode>("main.py");
  auto file2 = std::make_shared<TreeNode>("README.md");
  folder1->add(file1);
  folder2->add(file2);
  original_root->add(folder1);
  original_root->add(folder2);

  auto data = original_root->to_json_data();
  auto restored_root = TreeNode::from_json_data(data);
  REQUIRE(restored_root->name == "filesystem_root");
  auto children = restored_root->children();
  REQUIRE(children.size() == 2);
  REQUIRE(children[1]->name == "docs");
  REQUIRE(children[0]->children().size() == 1);
}

TEST_CASE("Tree test treenode add.") {
  auto parent = std::make_shared<TreeNode>("parent");
  auto child = std::make_shared<TreeNode>("child");
  parent->add(child);
  auto children = parent->children();
  REQUIRE(children.size() == 1);
  REQUIRE(children[0]->name == child->name);
}

TEST_CASE("Tree test treenode remove.") {
  auto parent = std::make_shared<TreeNode>("parent");
  auto child = std::make_shared<TreeNode>("child");
  parent->add(child);
  auto removed = parent->remove(child);
  auto children = parent->children();
  REQUIRE(children.size() == 0);
  REQUIRE(removed == child);
}

TEST_CASE("Tree test treenode traverse.") {
  auto root = std::make_shared<TreeNode>("root");
  auto child = std::make_shared<TreeNode>("child");
  root->add(child);
  auto nodes = root->traverse();
  REQUIRE(nodes.size() == 2);
  REQUIRE(nodes[0] == root.get());
}

TEST_CASE("Tree test tree constructor.") {
  Tree tree("my_tree");
  REQUIRE(tree.name == "my_tree");
  REQUIRE(!tree.guid.empty());
  REQUIRE(tree.root() == nullptr);
}

TEST_CASE("Tree test tree to_json_data.") {
  auto tree = std::make_shared<Tree>("object_hierarchy");
  Point point1(100.0, 200.0, 300.0);
  auto root = std::make_shared<TreeNode>(point1.guid);
  tree->add(root);
  auto data = tree->to_json_data();
  REQUIRE(data["name"] == "object_hierarchy");
  REQUIRE(data["type"] == "Tree");
  REQUIRE(data["root"]["name"] == point1.guid);
}

TEST_CASE("Tree test tree from_json_data.") {
  auto original_tree = std::make_shared<Tree>("spatial_hierarchy");
  Point point1(100.0, 200.0, 300.0);
  Point point2(400.0, 500.0, 600.0);
  Point point3(700.0, 800.0, 900.0);
  auto root = std::make_shared<TreeNode>(point1.guid);
  auto child1 = std::make_shared<TreeNode>(point2.guid);
  auto child2 = std::make_shared<TreeNode>(point3.guid);
  original_tree->add(root);
  original_tree->add(child1, root);
  original_tree->add(child2, root);
  auto data = original_tree->to_json_data();
  Tree restored_tree = Tree::from_json_data(data);
  REQUIRE(restored_tree.name == "spatial_hierarchy");
  REQUIRE(restored_tree.root()->name == point1.guid);
  REQUIRE(restored_tree.nodes().size() == 3);
}

TEST_CASE("Tree test tree to_json_from_json.") {
  auto tree = std::make_shared<Tree>();
  Point point1(0.0, 0.0, 0.0);
  Point point2(1.0, 1.0, 1.0);
  Point point3(2.0, 2.0, 2.0);
  Point point4(3.0, 3.0, 3.0);
  auto root = std::make_shared<TreeNode>(point1.guid);
  auto child1 = std::make_shared<TreeNode>(point2.guid);
  auto child2 = std::make_shared<TreeNode>(point3.guid);
  auto child3 = std::make_shared<TreeNode>(point4.guid);
  tree->add(root);
  tree->add(child1, root);
  tree->add(child2, root);
  tree->add(child3, child1);
  std::string filename = "../test_tree.json";

  tree->to_json(filename);
  Tree loaded_tree = Tree::from_json(filename);

  REQUIRE(loaded_tree.name == tree->name);
  REQUIRE(loaded_tree.root()->name == tree->root()->name);
  REQUIRE(loaded_tree.nodes().size() == tree->nodes().size());
}

TEST_CASE("Tree test tree add.") {
  auto tree = std::make_shared<Tree>();
  auto root = std::make_shared<TreeNode>("root");
  tree->add(root);
  REQUIRE(tree->root() == root);
  REQUIRE(tree->nodes().size() == 1);
}

TEST_CASE("Tree test tree remove.") {
  auto tree = std::make_shared<Tree>();
  auto root = std::make_shared<TreeNode>("root");
  tree->add(root);
  tree->remove(root);
  REQUIRE(tree->root() == nullptr);
}

TEST_CASE("Tree test tree get node by name.") {
  auto tree = std::make_shared<Tree>();
  auto root = std::make_shared<TreeNode>("root");
  tree->add(root);
  auto found = tree->get_node_by_name("root");
  REQUIRE(found == root);
}

} // namespace session_cpp