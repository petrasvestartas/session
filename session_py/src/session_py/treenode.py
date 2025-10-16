import uuid
from typing import Optional


class TreeNode:
    """A node of a tree data structure.

    TreeNodes can represent either:
    - Geometry nodes: name is set to the geometry's GUID for lookup
    - Organizational nodes: name is a descriptive string (e.g., "folder", "group")

    When adding geometry to a Session, the TreeNode.name is automatically set to
    the geometry.guid, allowing the tree hierarchy to reference geometry objects.

    Parameters
    ----------
    name : str, optional
        The name of the tree node. For geometry nodes, this should be the geometry's GUID.
        For organizational nodes, this can be any descriptive string.

    Attributes
    ----------
    name : str
        The name of the tree node. For geometry nodes, this is the geometry's GUID.
    guid : UUID
        The unique identifier of the tree node itself (distinct from geometry GUID).
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
    # JSON (polymorphic)
    ###########################################################################################

    def __jsondump__(self) -> dict:
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "children": [child.__jsondump__() for child in self.children],
        }

    @classmethod
    def __jsonload__(
        cls, data: dict, guid: Optional[str] = None, name: Optional[str] = None
    ) -> "TreeNode":
        """Deserialize from polymorphic JSON format."""
        node = cls(name=data["name"])
        node.guid = guid if guid is not None else data.get("guid", node.guid)
        for child_data in data.get("children", []):
            # Children are polymorphic nodes themselves
            from .encoders import decode_node

            child_node = decode_node(child_data)
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
