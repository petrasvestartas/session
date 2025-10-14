from .treenode import TreeNode


def test_treenode_constructor():
    node = TreeNode("root")
    assert node.name == "root"
    assert node.is_root


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


def test_treenode_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    root = TreeNode("project_root")
    folder1 = TreeNode("src")
    folder2 = TreeNode("docs")
    file1 = TreeNode("main.py")
    file2 = TreeNode("README.md")
    root.add(folder1)
    root.add(folder2)
    folder1.add(file1)
    folder2.add(file2)

    path = Path(__file__).resolve().parents[2] / "test_treenode.json"
    json_dump(root, path)
    loaded = json_load(path)

    assert isinstance(loaded, TreeNode)
    assert loaded.name == "project_root"
    assert len(loaded.children) == 2
    assert loaded.children[0].name == "src"
    assert loaded.children[0].children[0].name == "main.py"
