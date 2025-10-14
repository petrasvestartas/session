import uuid
from typing import Any, Optional, TYPE_CHECKING

if TYPE_CHECKING:
    from .treenode import TreeNode


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
    # JSON (polymorphic)
    ###########################################################################################

    def __jsondump__(self) -> dict:
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "root": self.root.__jsondump__() if self.root else None,
        }

    @classmethod
    def __jsonload__(
        cls, data: dict, guid: Optional[str] = None, name: Optional[str] = None
    ) -> "Tree":
        """Deserialize from polymorphic JSON format."""
        tree = cls(name=data.get("name", "Tree"))
        tree.guid = guid if guid is not None else data.get("guid", tree.guid)
        if data.get("root"):
            from .encoders import decode_node

            root = decode_node(data["root"])
            tree.add(root)
        return tree

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
        from .treenode import TreeNode

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
