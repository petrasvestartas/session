import uuid
from typing import Any, Dict, Optional
from .objects import Objects
from .point import Point
from .tree import Tree
from .treenode import TreeNode
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
        self.lookup: Dict[str, Any] = {}
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
    # JSON (polymorphic)
    ###########################################################################################

    def __jsondump__(self) -> dict:
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "objects": self.objects.__jsondump__(),
            "tree": self.tree.__jsondump__(),
            "graph": self.graph.__jsondump__(),
        }

    @classmethod
    def __jsonload__(
        cls, data: dict, guid: Optional[str] = None, name: Optional[str] = None
    ) -> "Session":
        """Deserialize from polymorphic JSON format."""
        from .encoders import decode_node

        session = cls(name=data.get("name", "my_session"))
        session.guid = guid if guid is not None else data.get("guid", session.guid)

        # Load nested structures via decode_node
        if data.get("objects"):
            session.objects = decode_node(data["objects"])  # Objects
        if data.get("tree"):
            session.tree = decode_node(data["tree"])  # Tree
        if data.get("graph"):
            session.graph = decode_node(data["graph"])  # Graph

        # Rebuild lookup from all objects
        for point in session.objects.points:
            session.lookup[point.guid] = point
        for line in session.objects.lines:
            session.lookup[line.guid] = line
        for plane in session.objects.planes:
            session.lookup[plane.guid] = plane
        for bbox in session.objects.bboxes:
            session.lookup[bbox.guid] = bbox
        for polyline in session.objects.polylines:
            session.lookup[polyline.guid] = polyline
        for pointcloud in session.objects.pointclouds:
            session.lookup[pointcloud.guid] = pointcloud
        for mesh in session.objects.meshes:
            session.lookup[mesh.guid] = mesh
        for cylinder in session.objects.cylinders:
            session.lookup[cylinder.guid] = cylinder
        for arrow in session.objects.arrows:
            session.lookup[arrow.guid] = arrow

        return session

    ###########################################################################################
    # Details - Add objects
    ###########################################################################################

    def add_point(self, point: Point) -> TreeNode:
        """Add a point to the Session.

        Automatically creates corresponding nodes in both graph and tree structures.

        Parameters
        ----------
        point : :class:`Point`
            The point to add to the session.

        Returns
        -------
        TreeNode
            The TreeNode created for this point.
        """
        self.objects.points.append(point)
        self.lookup[point.guid] = point
        self.graph.add_node(point.guid, f"point_{point.name}")
        tree_node = TreeNode(name=point.guid)
        return tree_node

    def add_line(self, line) -> TreeNode:
        """Add a line to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this line.
        """
        self.objects.lines.append(line)
        self.lookup[line.guid] = line
        self.graph.add_node(line.guid, f"line_{line.name}")
        tree_node = TreeNode(name=line.guid)
        return tree_node

    def add_plane(self, plane) -> TreeNode:
        """Add a plane to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this plane.
        """
        self.objects.planes.append(plane)
        self.lookup[plane.guid] = plane
        self.graph.add_node(plane.guid, f"plane_{plane.name}")
        tree_node = TreeNode(name=plane.guid)
        return tree_node

    def add_bbox(self, bbox) -> TreeNode:
        """Add a bounding box to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this bounding box.
        """
        self.objects.bboxes.append(bbox)
        self.lookup[bbox.guid] = bbox
        self.graph.add_node(bbox.guid, f"bbox_{bbox.name}")
        tree_node = TreeNode(name=bbox.guid)
        return tree_node

    def add_polyline(self, polyline) -> TreeNode:
        """Add a polyline to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this polyline.
        """
        self.objects.polylines.append(polyline)
        self.lookup[polyline.guid] = polyline
        self.graph.add_node(polyline.guid, f"polyline_{polyline.name}")
        tree_node = TreeNode(name=polyline.guid)
        return tree_node

    def add_pointcloud(self, pointcloud) -> TreeNode:
        """Add a point cloud to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this point cloud.
        """
        self.objects.pointclouds.append(pointcloud)
        self.lookup[pointcloud.guid] = pointcloud
        self.graph.add_node(pointcloud.guid, f"pointcloud_{pointcloud.name}")
        tree_node = TreeNode(name=pointcloud.guid)
        return tree_node

    def add_mesh(self, mesh) -> TreeNode:
        """Add a mesh to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this mesh.
        """
        self.objects.meshes.append(mesh)
        self.lookup[mesh.guid] = mesh
        self.graph.add_node(mesh.guid, f"mesh_{mesh.name}")
        tree_node = TreeNode(name=mesh.guid)
        return tree_node

    def add_cylinder(self, cylinder) -> TreeNode:
        """Add a cylinder to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this cylinder.
        """
        self.objects.cylinders.append(cylinder)
        self.lookup[cylinder.guid] = cylinder
        self.graph.add_node(cylinder.guid, f"cylinder_{cylinder.name}")
        tree_node = TreeNode(name=cylinder.guid)
        return tree_node

    def add_arrow(self, arrow) -> TreeNode:
        """Add an arrow to the Session.

        Returns
        -------
        TreeNode
            The TreeNode created for this arrow.
        """
        self.objects.arrows.append(arrow)
        self.lookup[arrow.guid] = arrow
        self.graph.add_node(arrow.guid, f"arrow_{arrow.name}")
        tree_node = TreeNode(name=arrow.guid)
        return tree_node

    def add(self, node: TreeNode, parent: TreeNode = None) -> None:
        """Add a TreeNode to the tree hierarchy.

        Parameters
        ----------
        node : TreeNode
            The TreeNode to add.
        parent : TreeNode, optional
            Parent TreeNode (defaults to root if not provided).
        """
        if parent is None:
            self.tree.add(node, self.tree.root)
        else:
            self.tree.add(node, parent)

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
