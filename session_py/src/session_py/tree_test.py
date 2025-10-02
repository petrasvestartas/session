from .tree import TreeNode, Tree
from .point import Point


def test_treenode_constructor():
    node = TreeNode("root")
    assert node.name == "root"
    assert node.is_root


def test_treenode_to_json_data():
    root = TreeNode("project_root")
    folder1 = TreeNode("src")
    folder2 = TreeNode("docs")
    file1 = TreeNode("main.py")
    file2 = TreeNode("README.md")
    root.add(folder1)
    root.add(folder2)
    folder1.add(file1)
    folder2.add(file2)
    data = root.to_json_data()
    assert data["name"] == "project_root"
    assert data["type"] == "TreeNode"
    assert len(data["children"]) == 2
    assert data["children"][0]["name"] == "src"
    assert len(data["children"][0]["children"]) == 1


def test_treenode_from_json_data():
    original_root = TreeNode("filesystem_root")
    bin_folder = TreeNode("bin")
    lib_folder = TreeNode("lib")
    app_file = TreeNode("app.exe")
    config_file = TreeNode("config.dll")
    original_root.add(bin_folder)
    original_root.add(lib_folder)
    bin_folder.add(app_file)
    lib_folder.add(config_file)
    data = original_root.to_json_data()
    restored_root = TreeNode.from_json_data(data)
    assert restored_root.name == "filesystem_root"
    assert len(restored_root.children) == 2
    assert restored_root.children[0].name == "bin"
    assert len(restored_root.children[0].children) == 1


def test_treenode_add():
    parent = TreeNode("parent")
    child = TreeNode("child")
    parent.add(child)
    assert len(parent.children) == 1
    assert child.parent == parent


def test_treenode_remove():
    parent = TreeNode("parent")
    child = TreeNode("child")
    parent.add(child)
    parent.remove(child)
    assert len(parent.children) == 0
    assert child.parent is None


def test_treenode_traverse():
    root = TreeNode("root")
    child = TreeNode("child")
    root.add(child)
    nodes = list(root.traverse())
    assert len(nodes) == 2
    assert nodes[0] == root


def test_tree_constructor():
    tree = Tree("my_tree")
    assert tree.name == "my_tree"
    assert tree.guid is not None
    assert tree.root is None


def test_tree_to_json_data():
    tree = Tree("object_hierarchy")
    point1 = Point(1.0, 2.0, 3.0)
    point2 = Point(4.0, 5.0, 6.0)
    point3 = Point(7.0, 8.0, 9.0)
    point4 = Point(10.0, 11.0, 12.0)
    root_node = TreeNode(point1.guid)
    child1 = TreeNode(point2.guid)
    child2 = TreeNode(point3.guid)
    grandchild = TreeNode(point4.guid)
    tree.add(root_node)
    tree.add(child1, root_node)
    tree.add(child2, root_node)
    tree.add(grandchild, child1)
    data = tree.to_json_data()
    assert data["name"] == "object_hierarchy"
    assert data["type"] == "Tree"
    assert data["root"]["name"] == point1.guid
    assert len(data["root"]["children"]) == 2


def test_tree_from_json_data():
    original_tree = Tree("spatial_hierarchy")
    point1 = Point(100.0, 200.0, 300.0)
    point2 = Point(400.0, 500.0, 600.0)
    point3 = Point(700.0, 800.0, 900.0)
    root = TreeNode(point1.guid)
    child1 = TreeNode(point2.guid)
    child2 = TreeNode(point3.guid)
    original_tree.add(root)
    original_tree.add(child1, root)
    original_tree.add(child2, root)
    data = original_tree.to_json_data()
    restored_tree = Tree.from_json_data(data)
    assert restored_tree.name == "spatial_hierarchy"
    assert restored_tree.root.name == point1.guid
    assert len(list(restored_tree.nodes)) == 3


def test_tree_to_json_from_json():
    tree = Tree()
    point1 = Point(0.0, 0.0, 0.0)
    point2 = Point(1.0, 1.0, 1.0)
    point3 = Point(2.0, 2.0, 2.0)
    point4 = Point(3.0, 3.0, 3.0)
    root = TreeNode(point1.guid)
    branch1 = TreeNode(point2.guid)
    branch2 = TreeNode(point3.guid)
    leaf = TreeNode(point4.guid)
    tree.add(root)
    tree.add(branch1, root)
    tree.add(branch2, root)
    tree.add(leaf, branch1)
    filename = "test_tree.json"

    tree.to_json(filename)
    loaded_tree = Tree.from_json(filename)

    assert loaded_tree.name == tree.name
    assert loaded_tree.root.name == tree.root.name
    assert len(list(loaded_tree.nodes)) == len(list(tree.nodes))


def test_tree_add():
    tree = Tree()
    root = TreeNode("root")
    tree.add(root)
    assert tree.root == root
    assert len(list(tree.nodes)) == 1


def test_tree_remove():
    tree = Tree()
    root = TreeNode("root")
    tree.add(root)
    tree.remove(root)
    assert tree.root is None


def test_tree_get_node_by_name():
    tree = Tree()
    root = TreeNode("root")
    tree.add(root)
    found = tree.get_node_by_name("root")
    assert found == root
