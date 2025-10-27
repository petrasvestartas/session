import uuid
from typing import Any, Dict, List, Tuple, Optional, NamedTuple
from .objects import Objects
from .point import Point
from .tree import Tree
from .treenode import TreeNode
from .graph import Graph
from .bvh import BVH
from .boundingbox import BoundingBox
from .tolerance import Tolerance


class RayHit(NamedTuple):
    guid: str
    point: Point
    distance: float


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

        # BVH for collision detection (auto-computed world size)
        self.bvh = BVH()

        # Create empty root node with session name
        root_node = TreeNode(name=self.name)
        self.tree.add(root_node)

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
    # BVH Collision Detection
    ###########################################################################################

    @staticmethod
    def _compute_bounding_box(geometry) -> BoundingBox:
        """Compute bounding box for a geometry object, inflated by tolerance.

        Parameters
        ----------
        geometry : object
            Any geometry object (Point, Line, Mesh, etc.)

        Returns
        -------
        BoundingBox
            Inflated bounding box for collision detection.
        """
        inflate = Tolerance.APPROXIMATION

        # Import geometry types
        from .line import Line
        from .polyline import Polyline
        from .pointcloud import PointCloud
        from .mesh import Mesh
        from .plane import Plane
        from .cylinder import Cylinder
        from .arrow import Arrow

        if isinstance(geometry, Point):
            return BoundingBox.from_point(geometry, inflate)
        elif isinstance(geometry, Line):
            points = [geometry.start(), geometry.end()]
            return BoundingBox.from_points(points, inflate)
        elif isinstance(geometry, Polyline):
            return BoundingBox.from_points(geometry.points, inflate)
        elif isinstance(geometry, PointCloud):
            return BoundingBox.from_points(geometry.points, inflate)
        elif isinstance(geometry, Mesh):
            # Extract vertices from mesh
            points = [Point(v.x, v.y, v.z) for v in geometry.vertex.values()]
            if not points:
                return BoundingBox.from_point(Point(0, 0, 0), inflate)
            return BoundingBox.from_points(points, inflate)
        elif isinstance(geometry, BoundingBox):
            # Inflate existing bounding box
            from .vector import Vector

            inflated = BoundingBox(
                center=geometry.center,
                x_axis=geometry.x_axis,
                y_axis=geometry.y_axis,
                z_axis=geometry.z_axis,
                half_size=Vector(
                    geometry.half_size.x + inflate,
                    geometry.half_size.y + inflate,
                    geometry.half_size.z + inflate,
                ),
            )
            return inflated
        elif isinstance(geometry, Plane):
            # Create bounded box around plane origin
            return BoundingBox.from_point(geometry.origin, inflate * 10.0)
        elif isinstance(geometry, Cylinder):
            # Compute from cylinder line endpoints and radius
            points = [geometry.line.start(), geometry.line.end()]
            bbox = BoundingBox.from_points(points, inflate)
            # Inflate by cylinder radius
            from .vector import Vector

            bbox.half_size = Vector(
                bbox.half_size.x + geometry.radius,
                bbox.half_size.y + geometry.radius,
                bbox.half_size.z + geometry.radius,
            )
            return bbox
        elif isinstance(geometry, Arrow):
            # Compute from arrow line endpoints
            points = [geometry.line.start(), geometry.line.end()]
            bbox = BoundingBox.from_points(points, inflate)
            # Inflate by arrow radius
            from .vector import Vector

            bbox.half_size = Vector(
                bbox.half_size.x + geometry.radius,
                bbox.half_size.y + geometry.radius,
                bbox.half_size.z + geometry.radius,
            )
            return bbox
        else:
            # Fallback
            return BoundingBox.from_point(Point(0, 0, 0), inflate)

    def get_collisions(self) -> List[Tuple[str, str]]:
        """Get all collision pairs using BVH and add them as graph edges.

        Automatically:
        - Computes bounding boxes for all objects with tolerance inflation
        - Builds/rebuilds the BVH with auto-computed world size
        - Detects all collision pairs
        - Adds collision edges to the graph

        Returns
        -------
        list of tuple
            List of (guid1, guid2) tuples representing colliding geometry pairs.
        """
        # Collect all objects with their bounding boxes and GUIDs
        boxes_with_guids = []

        for guid, geometry in self.lookup.items():
            bbox = self._compute_bounding_box(geometry)
            boxes_with_guids.append((bbox, guid))

        if not boxes_with_guids:
            return []

        # Build BVH with GUIDs (auto-computes world size)
        self.bvh.build_with_guids(boxes_with_guids)

        # Extract just the boxes for collision checking
        boxes = [bbox for bbox, _ in boxes_with_guids]

        # Get collision pairs as GUIDs directly
        collision_pairs = self.bvh.check_all_collisions_guids(boxes)

        # Add collision edges to graph
        for guid1, guid2 in collision_pairs:
            self.graph.add_edge(guid1, guid2, "bvh_collision")

        return collision_pairs

    def ray_cast(
        self, origin: Point, direction, tolerance: float = 1e-3
    ) -> List[RayHit]:
        from .line import Line
        from .vector import Vector
        from .polyline import Polyline
        from .plane import Plane
        from .boundingbox import BoundingBox
        from .mesh import Mesh
        from .cylinder import Cylinder
        from .arrow import Arrow
        from .intersection import line_line, line_plane, ray_box, ray_mesh_bvh

        dir_vec = Vector(direction.x, direction.y, direction.z)
        if dir_vec.magnitude() <= 0.0:
            return []
        dir_unit = dir_vec.normalize()

        FAR = 1e6
        ray_line = Line(
            origin.x,
            origin.y,
            origin.z,
            origin.x + dir_unit.x * FAR,
            origin.y + dir_unit.y * FAR,
            origin.z + dir_unit.z * FAR,
        )

        boxes_with_guids: List[Tuple[BoundingBox, str]] = []
        for guid, geometry in self.lookup.items():
            bbox = self._compute_bounding_box(geometry)
            boxes_with_guids.append((bbox, guid))
        if not boxes_with_guids:
            return []

        self.bvh.build_with_guids(boxes_with_guids)

        candidates: List[int] = []
        self.bvh.ray_cast(origin, dir_unit, candidates, True)

        hits_all: List[RayHit] = []

        def point_hit(p: Point) -> Tuple[bool, Point, float]:
            vx = p.x - origin.x
            vy = p.y - origin.y
            vz = p.z - origin.z
            cx = vy * dir_unit.z - vz * dir_unit.y
            cy = vz * dir_unit.x - vx * dir_unit.z
            cz = vx * dir_unit.y - vy * dir_unit.x
            dist = (cx * cx + cy * cy + cz * cz) ** 0.5
            if dist > tolerance:
                return False, origin, 0.0
            t = vx * dir_unit.x + vy * dir_unit.y + vz * dir_unit.z
            if t < 0.0:
                return False, origin, 0.0
            hp = Point(
                origin.x + dir_unit.x * t,
                origin.y + dir_unit.y * t,
                origin.z + dir_unit.z * t,
            )
            return True, hp, t

        for idx in candidates:
            if idx < 0 or idx >= len(self.bvh.object_guids):
                continue
            guid = self.bvh.object_guids[idx]
            geom = self.lookup.get(guid)
            if geom is None:
                continue

            hit_point: Optional[Point] = None

            if isinstance(geom, BoundingBox):
                pts = ray_box(ray_line, geom, 0.0, FAR)
                if pts:
                    hit_point = pts[0]
            elif isinstance(geom, Plane):
                hp = line_plane(ray_line, geom, True)
                if hp is not None:
                    hit_point = hp
            elif hasattr(geom, "start") and hasattr(geom, "end"):
                hp = line_line(ray_line, geom, Tolerance.APPROXIMATION)
                if hp is not None:
                    hit_point = hp
            elif isinstance(geom, Polyline):
                best_t = float("inf")
                best_p: Optional[Point] = None
                for i in range(len(geom.points) - 1):
                    seg = Line.from_points(geom.points[i], geom.points[i + 1])
                    hp = line_line(ray_line, seg, Tolerance.APPROXIMATION)
                    if hp is None:
                        continue
                    t = (
                        (hp.x - origin.x) * dir_unit.x
                        + (hp.y - origin.y) * dir_unit.y
                        + (hp.z - origin.z) * dir_unit.z
                    )
                    if t >= 0.0 and t < best_t:
                        best_t = t
                        best_p = hp
                if best_p is not None:
                    hit_point = best_p
            elif isinstance(geom, Mesh):
                pts = ray_mesh_bvh(ray_line, geom, 1e-6, False)
                if pts:
                    hit_point = pts[0]
            elif isinstance(geom, Cylinder):
                hp = line_line(ray_line, geom.line, Tolerance.APPROXIMATION)
                if hp is not None:
                    hit_point = hp
            elif isinstance(geom, Arrow):
                hp = line_line(ray_line, geom.line, Tolerance.APPROXIMATION)
                if hp is not None:
                    hit_point = hp
            elif isinstance(geom, Point):
                ok, hp, t = point_hit(geom)
                if ok:
                    hit_point = hp

            if hit_point is None:
                continue

            d = (
                (hit_point.x - origin.x) * dir_unit.x
                + (hit_point.y - origin.y) * dir_unit.y
                + (hit_point.z - origin.z) * dir_unit.z
            )
            if d >= 0.0:
                hits_all.append(RayHit(guid, hit_point, d))

        if not hits_all:
            return []

        min_d = min(h.distance for h in hits_all)
        eps = max(1e-6, tolerance * 1e-3)
        hits = [h for h in hits_all if abs(h.distance - min_d) <= eps]
        hits.sort(key=lambda h: h.distance)
        return hits

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

    ###########################################################################################
    # Details - Transformed Geometry
    ###########################################################################################

    def get_geometry(self) -> Objects:
        """Get all geometry with transformations applied from tree hierarchy.

        Recursively traverses the tree and applies parent transformations to children.
        Each child's transformation is the composition of all ancestor transformations
        multiplied by its own transformation.

        Returns
        -------
        Objects
            Collection of transformed geometry objects.
        """
        from .xform import Xform
        import copy

        # Deep copy all objects
        transformed_objects = copy.deepcopy(self.objects)

        # Rebuild lookup from copied objects
        transformed_lookup = {}
        for collection in [
            transformed_objects.points,
            transformed_objects.lines,
            transformed_objects.planes,
            transformed_objects.bboxes,
            transformed_objects.polylines,
            transformed_objects.pointclouds,
            transformed_objects.meshes,
            transformed_objects.cylinders,
            transformed_objects.arrows,
        ]:
            for geom in collection:
                transformed_lookup[geom.guid] = geom

        def transform_node(node: TreeNode, parent_xform: Xform) -> None:
            """Recursively transform geometry in tree node and its children."""

            # Get geometry from the lookup,
            geom = transformed_lookup.get(node.name)

            if geom is not None:
                geom.xform = parent_xform * geom.xform
                current_xform = geom.xform
            else:
                current_xform = parent_xform

            for child in node.children:
                transform_node(child, current_xform)

        if self.tree.root:
            transform_node(self.tree.root, Xform.identity())

        # Apply accumulated transformations to actual geometry coordinates
        for collection in [
            transformed_objects.points,
            transformed_objects.lines,
            transformed_objects.planes,
            transformed_objects.bboxes,
            transformed_objects.polylines,
            transformed_objects.pointclouds,
            transformed_objects.meshes,
            transformed_objects.cylinders,
            transformed_objects.arrows,
        ]:
            for geom in collection:
                geom.transform()

        return transformed_objects
