import json
import uuid
from typing import Any


class TreeNode:
    """A node of a tree data structure.

    Parameters
    ----------
    name : str, optional
        The name of the tree node.

    Attributes
    ----------
    name : str
        The name of the tree node.
    guid : UUID
        The unique identifier of the tree node.
    parent : :class:`TreeNode`
        The parent node of the tree node.
    children : list[:class:`TreeNode`]
        The children of the tree node.

    """

    def __init__(self, name="my_node"):
        self.name = name
        self.guid = str(uuid.uuid4())
        self._parent = None
        self._children = []
        self._tree = None

    def __str__(self):
        """String representation."""
        return f"TreeNode({self.name}"

    def __repr__(self):
        return f"TreeNode({self.name}, {self.guid}, {len(self.children)} children)"

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict[str, Any]:
        """Convert the TreeNode to a JSON-serializable dictionary."""
        return {
            "type": "TreeNode",
            "name": self.name,
            "guid": self.guid,
            "children": [child.to_json_data() for child in self.children],
        }

    @classmethod
    def from_json_data(cls, data: dict[str, Any]) -> "TreeNode":
        """Create a TreeNode from JSON data dictionary."""
        node = cls(name=data["name"])
        for child_data in data.get("children", []):
            child_node = cls.from_json_data(child_data)
            node.add(child_node)
        return node

    ###########################################################################################
    # Details
    ###########################################################################################

    @property
    def is_root(self):
        return self._parent is None

    @property
    def is_leaf(self):
        return not self._children

    @property
    def is_branch(self):
        return not self.is_root and not self.is_leaf

    @property
    def parent(self):
        return self._parent

    @property
    def children(self):
        return self._children

    @property
    def tree(self):
        if self.is_root:
            return self._tree
        else:
            return self.parent.tree  # type: ignore

    def add(self, node):
        """Add a child node to this node.

        Parameters
        ----------
        node : :class:`TreeNode`
            The node to add.

        """
        if not isinstance(node, TreeNode):
            raise TypeError("The node is not a TreeNode object.")
        if node not in self._children:
            self._children.append(node)
        node._parent = self

    def remove(self, node):
        """Remove a child node from this node.

        Parameters
        ----------
        node : :class:`TreeNode`
            The node to remove.

        """
        self._children.remove(node)
        node._parent = None

    @property
    def ancestors(self):
        this = self
        while this.parent:
            yield this.parent
            this = this.parent

    @property
    def descendants(self):
        for child in self.children:
            yield child
            for descendant in child.descendants:
                yield descendant

    def traverse(self, strategy="depthfirst", order="preorder"):
        """Traverse the tree from this node.

        Parameters
        ----------
        strategy : {"depthfirst", "breadthfirst"}, optional
            The traversal strategy.
        order : {"preorder", "postorder"}, optional
            The traversal order.

        """
        if strategy == "depthfirst":
            if order == "preorder":
                yield self
                for child in self.children:
                    for node in child.traverse(strategy, order):
                        yield node
            elif order == "postorder":
                for child in self.children:
                    for node in child.traverse(strategy, order):
                        yield node
                yield self
            else:
                raise ValueError("Unknown traversal order: {}".format(order))
        elif strategy == "breadthfirst":
            queue = [self]
            while queue:
                node = queue.pop(0)
                yield node
                queue.extend(node.children)
        else:
            raise ValueError("Unknown traversal strategy: {}".format(strategy))


class Tree:
    """A hierarchical data structure with parent-child relationships.

    Parameters
    ----------
    name : str, optional
        The name of the tree. Defaults to "Tree".

    Attributes
    ----------
    guid : UUID
        The unique identifier of the tree.
    name : str
        The name of the tree.
    root : :class:`TreeNode`
        The root node of the tree.

    """

    def __init__(self, name="my_tree"):
        self.guid = str(uuid.uuid4())
        self.name = name
        self._root = None

    def __str__(self):
        return "<Tree with {} nodes>".format(len(list(self.nodes)))

    def __repr__(self):
        return "<Tree with {} nodes>".format(len(list(self.nodes)))

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict[str, Any]:
        """Convert the Tree to a JSON-serializable dictionary."""
        return {
            "type": "Tree",
            "guid": self.guid,
            "name": self.name,
            "root": self.root.to_json_data() if self.root else None,
        }

    @classmethod
    def from_json_data(cls, data: dict[str, Any]) -> "Tree":
        """Create a Tree from JSON data dictionary."""
        tree = cls(name=data.get("name", "Tree"))
        if data.get("root"):
            root = TreeNode.from_json_data(data["root"])
            tree.add(root)
        return tree

    def to_json(self, filepath: str) -> None:
        """Serialize the Tree to a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the output JSON file

        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> "Tree":
        """Deserialize a Tree from a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the JSON file to load

        Returns
        -------
        :class:`Tree`
            Tree instance loaded from the file.

        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)

    ###########################################################################################
    # Details
    ###########################################################################################

    @property
    def root(self):
        return self._root

    def add(self, node, parent=None):
        """Add a node to the tree.

        Parameters
        ----------
        node : :class:`TreeNode`
            The node to add.
        parent : :class:`TreeNode`, optional
            The parent node. If None, adds as root.

        """
        if not isinstance(node, TreeNode):
            raise TypeError("The node is not a TreeNode object.")

        if node.parent:
            raise ValueError(
                "The node already has a parent, remove it from that parent first."
            )

        if parent is None:
            # add the node as a root node
            if self.root is not None:
                raise ValueError("The tree already has a root node, remove it first.")

            self._root = node
            node._tree = self  # type: ignore

        else:
            # add the node as a child of the parent node
            if not isinstance(parent, TreeNode):
                raise TypeError("The parent node is not a TreeNode object.")

            if parent.tree is not self:
                raise ValueError("The parent node is not part of this tree.")

            parent.add(node)

    @property
    def nodes(self):
        if self.root:
            for node in self.root.traverse():
                yield node

    def remove(self, node):
        """Remove a node from the tree.

        Parameters
        ----------
        node : :class:`TreeNode`
            The node to remove.

        """
        if node == self.root:
            self._root = None
            node._tree = None
        else:
            node.parent.remove(node)

    @property
    def leaves(self):
        for node in self.nodes:
            if node.is_leaf:
                yield node

    def traverse(self, strategy="depthfirst", order="preorder"):
        """
        Traverse the tree from the root node.

        Parameters
        ----------
        strategy : {"depthfirst", "breadthfirst"}, optional
            The traversal strategy.
        order : {"preorder", "postorder"}, optional
            The traversal order. This parameter is only used for depth-first traversal.

        Yields
        ------
        :class:`TreeNode`
            The next node in the traversal.

        Raises
        ------
        ValueError
            If the strategy is not ``"depthfirst"`` or ``"breadthfirst"``.
            If the order is not ``"preorder"`` or ``"postorder"``.

        """
        if self.root:
            for node in self.root.traverse(strategy=strategy, order=order):
                yield node

    def get_node_by_name(self, name):
        """Get a node by its name.

        Parameters
        ----------
        name : str
            The name of the node.

        """
        for node in self.nodes:
            if node.name == name:
                return node

    def get_nodes_by_name(self, name):
        """
        Get all nodes by their name.

        Parameters
        ----------
        name : str
            The name of the node.

        Returns
        -------
        list[:class:`TreeNode`]
            The nodes.

        """
        nodes = []
        for node in self.nodes:
            if node.name == name:
                nodes.append(node)
        return nodes

    def add_child_by_guid(self, parent_guid: uuid.UUID, child_guid: uuid.UUID) -> bool:
        """
        Add a parent-child relationship using GUIDs.

        Parameters
        ----------
        parent_guid : UUID
            The GUID of the parent node.
        child_guid : UUID
            The GUID of the child node.

        Returns
        -------
        bool
            True if the relationship was added, False if nodes not found.
        """
        parent_node = self.find_node_by_guid(parent_guid)
        child_node = self.find_node_by_guid(child_guid)

        if not parent_node or not child_node:
            return False

        # Remove child from its current parent if it has one
        if child_node.parent:
            child_node.parent.remove(child_node)

        # Add to new parent
        parent_node.add(child_node)
        return True

    def get_children_guids(self, guid: uuid.UUID) -> list[uuid.UUID]:
        """
        Get all children GUIDs of a node by its GUID.

        Parameters
        ----------
        guid : UUID
            The GUID of the parent node.

        Returns
        -------
        list[UUID]
            List of children GUIDs.
        """
        node = self.find_node_by_guid(guid)
        if not node:
            return []

        return [child.guid for child in node.children if hasattr(child, "guid")]

    def print_hierarchy(self):
        """Print the spatial hierarchy of the tree."""

        def _print(node, prefix="", last=True):
            connector = "└── " if last else "├── "
            print("{}{}{}".format(prefix, connector, node))
            prefix += "    " if last else "│   "
            for i, child in enumerate(node.children):
                _print(child, prefix, i == len(node.children) - 1)

        if self.root:
            _print(self.root)
        else:
            print("Empty tree")
