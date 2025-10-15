from .tree import Tree
from .treenode import TreeNode


def test_tree_constructor():
    tree = Tree("my_tree")
    assert tree.name == "my_tree"
    assert tree.guid is not None


def test_treenode_constructor():
    node = TreeNode("my_root")
    assert node.name == "my_root"
    assert node.is_root


def test_treenode_add():
    root = TreeNode("root")
    child = TreeNode("child")
    root.add(child)

    children = list(root.children)
    assert len(children) == 1
    assert children[0].name == "child"


def test_tree_json_roundtrip():
    from pathlib import Path
    from .encoders import json_dump, json_load

    original = Tree("test_tree")
    root = TreeNode("root")
    child = TreeNode("child")
    root.add(child)
    original.add(root)

    path = Path(__file__).resolve().parents[2] / "test_tree.json"
    json_dump(original, path)
    loaded = json_load(path)

    assert loaded.name == original.name
    assert loaded.guid == original.guid


def test_treenode_json_roundtrip():
    # Test TreeNode JSON roundtrip (treenode_test.py already tests file I/O)
    # This test verifies the roundtrip within tree_test module
    original = TreeNode("test_node")
    child = TreeNode("child_node")
    original.add(child)

    # In-memory roundtrip test only (file I/O tested in treenode_test.py)
    from .encoders import json_dumps, json_loads

    json_str = json_dumps(original)
    loaded = json_loads(json_str)

    assert loaded.name == original.name
    assert loaded.guid == original.guid
    assert len(list(loaded.children)) == len(list(original.children))
