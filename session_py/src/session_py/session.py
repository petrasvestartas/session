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
    
    Examples
    --------
    >>> session = Session()
    >>> point1 = Point(1.0, 2.0, 3.0)
    >>> point2 = Point(4.0, 5.0, 6.0)
    >>> session.add_point(point1)
    >>> session.add_point(point2)
    >>> _ = session.graph.add_node(point1.guid, "first_node")
    >>> _ = session.graph.add_node(point2.guid, "second_node")
    >>> _ = session.graph.add_edge(point1.guid, point2.guid, "connection")
    >>> retrieved_point = session.get_object(point1.guid)
    >>> session.to_json("Session.json")
    """
    
    def __init__(self, name="my_session"):
        self.guid = uuid.uuid4()
        self.name = name
        self.objects = Objects()
        self.lookup: dict[uuid.UUID, Point] = {}
        self.tree = Tree(name=f"{name}_tree")
        self.graph = Graph(name=f"{name}_graph")
        # ToDo:s
        # - BVH Boundary Volume Hierarchy

    def __str__(self) -> str:
        return f"Session(name='{self.name}')"

    def __repr__(self) -> str:
        return f"Session(name='{self.name}')"

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict[str, Any]:
        """Convert the Session to a JSON-serializable dictionary.
        
        Returns
        -------
        dict
            Dictionary representation of the session.
            
        Examples
        --------
        >>> session = Session()
        >>> point1 = Point(1.0, 2.0, 3.0)
        >>> point2 = Point(4.0, 5.0, 6.0)
        >>> session.add_point(point1)
        >>> session.add_point(point2)
        >>> session.add_edge(point1, point2, "connection")
        >>> data = session.to_json_data()
        >>> assert data["name"] == "my_session"
        >>> assert "guid" in data
        >>> assert len(data["objects"]["points"]) == 2
        >>> assert len(data["graph"]["vertices"]) == 2
        >>> assert len(data["graph"]["edges"]) == 1
        """
        return {
            "type": "Session",
            "name": self.name,
            "guid": str(self.guid),
            "objects": self.objects.to_json_data(),
            "tree": self.tree.to_json_data(),
            "graph": self.graph.to_json_data()
        }


    @classmethod
    def from_json_data(cls, data: dict[str, Any]) -> 'Session':
        """Create a Session from JSON data dictionary.
        
        Parameters
        ----------
        data : dict
            Dictionary containing session data.
            
        Returns
        -------
        :class:`Session`
            Session instance created from the data.
            
        Examples
        --------
        >>> session = Session()
        >>> point1 = Point(1.0, 2.0, 3.0)
        >>> point2 = Point(4.0, 5.0, 6.0)
        >>> session.add_point(point1)
        >>> session.add_point(point2)
        >>> session.add_edge(point1, point2, "connection")
        >>> data = session.to_json_data()
        >>> session2 = Session.from_json_data(data)
        >>> assert session2.name == "my_session"
        >>> assert len(session2.lookup) == 2
        >>> assert len(list(session2.graph.vertices())) == 2
        """
        session = cls(name=data.get("name", "my_session"))
        
        # Load objects
        if data.get("objects"):
            session.objects = Objects.from_json_data(data["objects"])
        
        # Rebuild lookup from objects
        for point in session.objects.points:
            session.lookup[uuid.UUID(point.guid)] = point
        
        
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
            
        Examples
        --------
        >>> session = Session()
        >>> point1 = Point(1.0, 2.0, 3.0)
        >>> point2 = Point(4.0, 5.0, 6.0)
        >>> session.add_point(point1)
        >>> session.add_point(point2)
        >>> session.add_edge(point1, point2, "connection")
        >>> session.to_json("my_session.json")
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> 'Session':
        """Deserialize a Session from a JSON file.
        
        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Session`
            Session instance loaded from the file.

        Examples
        --------
        >>> session = Session()
        >>> data = session.to_json_data()
        >>> session2 = Session.from_json_data(data)
        >>> session2.name
        'my_session'
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
        self.lookup[uuid.UUID(point.guid)] = point
        
        # Automatically add to graph using point's GUID as node key
        self.graph.add_node(point.guid, f"point_{point.name}")
        
        # Automatically add to tree using point's GUID as node name
        tree_node = TreeNode(name=point.guid)
        if not self.tree.root:
            self.tree.add(tree_node)
        else:
            # Add as child of root for now (can be reorganized later)
            self.tree.add(tree_node, self.tree.root)
    
    def add_edge(self, point1: Point, point2: Point, attribute: str = "") -> None:
        """Add an edge between two points in the graph.
        
        Parameters
        ----------
        point1 : :class:`Point`
            First point object.
        point2 : :class:`Point`
            Second point object.
        attribute : str, optional
            Edge attribute description.
        """
        self.graph.add_edge(point1.guid, point2.guid, attribute)

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

    def remove_object(self, guid: uuid.UUID) -> bool:
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

    def add_hierarchy(self, parent_guid: uuid.UUID, child_guid: uuid.UUID) -> bool:
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
    
    def get_children(self, guid: str) -> list[uuid.UUID]:
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
        return self.tree.get_children(uuid.UUID(guid))

    ###########################################################################################
    # Details - Graph
    ###########################################################################################

    def add_relationship(self, from_guid: uuid.UUID, to_guid: uuid.UUID, 
                             relationship_type: str = "default") -> None:
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
        self.graph.add_edge(str(from_guid), str(to_guid), relationship_type)
    
    def get_neighbours(self, guid: uuid.UUID) -> list[str]:
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
        return list(self.graph.neighbors(str(guid)))
    
