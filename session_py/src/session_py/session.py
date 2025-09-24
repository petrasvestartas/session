import json
import uuid
from typing import Any, Optional
from .objects import Objects
from .point import Point
from .tree import Tree, TreeNode
from .graph import Graph


class Session:
    """A Session containing geometry objects with hierarchical and graph structures.

    The Session class manages collections of geometry objects and provides:
    - Fast GUID-based lookup
    - Hierarchical tree structure for organization
    - Graph structure for object relationships
    - JSON serialization/deserialization

    Parameters
    ----------
    name : str, optional
        Name of the Session. Defaults to "Session".

    Attributes
    ----------
    objects : :class:`Objects`
        Collection of geometry objects in the Session.
    lookup : dict[UUID, :class:`Point`]
        Fast lookup dictionary mapping GUIDs to geometry objects.
    tree : :class:`Tree`
        Hierarchical tree structure for organizing geometry objects.
    graph : :class:`Graph`
        Graph structure for storing relationships between geometry objects.
    name : str
        Name of the Session.

    """

    def __init__(self, name="my_session"):
        self.guid = str(uuid.uuid4())
        self.name = name
        self.objects = Objects()
        self.lookup: dict[str, Any] = {}
        self.tree = Tree(name=f"{name}_tree")
        self.graph = Graph(name=f"{name}_graph")

        # Create empty root node with session name
        root_node = TreeNode(name=self.name)
        self.tree.add(root_node)

        # ToDo:s
        # - BVH Boundary Volume Hierarchy

    def __str__(self) -> str:
        return f"Session(objects={self.objects.to_str()}, tree={self.tree.to_str()}, graph={self.graph.to_str()})"

    def __repr__(self) -> str:
        return f"Session({self.guid}, {self.name}, {self.objects.to_str()}, {self.tree.to_str()}, {self.graph.to_str()})"

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict[str, Any]:
        """Convert the Session to a JSON-serializable dictionary.

        Returns
        -------
        dict
            Dictionary representation of the session.

        """
        return {
            "type": "Session",
            "name": self.name,
            "guid": self.guid,
            "objects": self.objects.to_json_data(),
            "tree": self.tree.to_json_data(),
            "graph": self.graph.to_json_data(),
        }

    @classmethod
    def from_json_data(cls, data: dict[str, Any]) -> "Session":
        """Create a Session from JSON data dictionary.

        Parameters
        ----------
        data : dict
            Dictionary containing session data.

        Returns
        -------
        :class:`Session`
            Session instance created from the data.

        """
        session = cls(name=data.get("name", "my_session"))

        # Load objects
        if data.get("objects"):
            session.objects = Objects.from_json_data(data["objects"])

        # Rebuild lookup from objects
        for point in session.objects.points:
            session.lookup[point.guid] = point

        # Load tree structure (this will override the default tree created by add_point/add_vector)
        if data.get("tree"):
            session.tree = Tree.from_json_data(data["tree"])

        # Load graph structure (this will override the default graph created by add_point/add_vector)
        if data.get("graph"):
            session.graph = Graph.from_json_data(data["graph"])

        return session

    def to_json(self, filepath: str) -> None:
        """Serialize the Session to a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the output JSON file.

        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> "Session":
        """Deserialize a Session from a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Session`
            Session instance loaded from the file.

        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)

    ###########################################################################################
    # Details - Add objects
    ###########################################################################################

    def add_point(self, point: Point) -> None:
        """Add a point to the Session.

        Automatically creates corresponding nodes in both graph and tree structures.

        Parameters
        ----------
        point : :class:`Point`
            The point to add to the session.
        """
        self.objects.points.append(point)
        self.lookup[point.guid] = point

        # Automatically add to graph using point's GUID as node key
        self.graph.add_node(point.guid, f"point_{point.name}")

        # Automatically add to tree as child of root using point's GUID as node name
        tree_node = TreeNode(name=point.guid)
        self.tree.add(tree_node, self.tree.root)

    def add_edge(self, guid1: str, guid2: str, attribute: str = "") -> None:
        """Add an edge between two geometry objects in the graph.

        Parameters
        ----------
        guid1 : str
            GUID of the first geometry object.
        guid2 : str
            GUID of the second geometry object.
        attribute : str, optional
            Edge attribute description.
        """
        self.graph.add_edge(guid1, guid2, attribute)

    ###########################################################################################
    # Details - Lookup
    ###########################################################################################

    def get_object(self, guid: str) -> Optional[Point]:
        """Get a geometry object by its GUID.

        Parameters
        ----------
        guid : str
            The string GUID of the geometry object to retrieve.

        Returns
        -------
        :class:`Point` | None
            The geometry object if found, None otherwise.
        """
        return self.lookup.get(guid)

    def remove_object(self, guid: str) -> bool:
        """Remove a geometry object by its GUID.

        Args:
            guid: The UUID of the geometry object to remove.

        Returns:
            True if the object was removed, False if not found.
        """
        geometry = self.lookup.get(guid)
        if not geometry:
            return False

        # Remove from points collection
        if isinstance(geometry, Point):
            self.objects.points.remove(geometry)

        # Remove from lookup table
        del self.lookup[guid]

        # Remove from tree - tree should handle GUID lookup
        self.tree.remove_node_by_guid(guid)

        # Remove from graph using string GUID
        if self.graph.has_node(str(guid)):
            self.graph.remove_node(str(guid))

        return True

    ###########################################################################################
    # Details - Tree
    ###########################################################################################

    def add_hierarchy(self, parent_guid: str, child_guid: str) -> bool:
        """Add a parent-child relationship in the tree structure.

        Parameters
        ----------
        parent_guid : UUID
            The GUID of the parent geometry object.
        child_guid : UUID
            The GUID of the child geometry object.

        Returns
        -------
        bool
            True if the relationship was added successfully.
        """
        return self.tree.add_child_by_guid(parent_guid, child_guid)

    def get_children(self, guid: str) -> list[str]:
        """Get all children GUIDs of a geometry object in the tree.

        Parameters
        ----------
        guid : str
            The string GUID to search for.

        Returns
        -------
        list[UUID]
            List of children GUIDs.
        """
        return self.tree.get_children(guid)

    ###########################################################################################
    # Details - Graph
    ###########################################################################################

    def add_relationship(
        self, from_guid: str, to_guid: str, relationship_type: str = "default"
    ) -> None:
        """Add a relationship edge in the graph structure.

        Parameters
        ----------
        from_guid : UUID
            The GUID of the source geometry object.
        to_guid : UUID
            The GUID of the target geometry object.
        relationship_type : str, optional
            The type of relationship. Defaults to "default".
        """
        self.graph.add_edge(from_guid, to_guid, relationship_type)

    def get_neighbours(self, guid: str) -> list[str]:
        """Get all GUIDs connected to the given GUID in the graph.

        Parameters
        ----------
        guid : UUID
            The GUID of the geometry object to find connections for.

        Returns
        -------
        list[str]
            List of connected geometry GUIDs as strings.
        """
        return self.graph.get_neighbors(guid)
