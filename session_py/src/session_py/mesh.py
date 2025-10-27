import uuid
import math
from enum import Enum
from typing import Optional, List, Dict, Tuple
from .point import Point
from .vector import Vector
from .tolerance import Tolerance
from .color import Color
from .xform import Xform


class NormalWeighting(Enum):
    AREA = "area"
    ANGLE = "angle"
    UNIFORM = "uniform"


class VertexData:
    """Vertex data containing position and attributes.

    Parameters
    ----------
    point : Point, optional
        Initial position. Defaults to origin.

    Attributes
    ----------
    x : float
        X coordinate.
    y : float
        Y coordinate.
    z : float
        Z coordinate.
    attributes : dict
        Custom vertex attributes.
    """

    def __init__(self, point: Point = None):
        if point is None:
            point = Point(0.0, 0.0, 0.0)
        self.x = point.x
        self.y = point.y
        self.z = point.z
        self.attributes = {}

    def position(self) -> Point:
        """Get the vertex position as a Point."""
        return Point(self.x, self.y, self.z)

    def set_position(self, point: Point):
        """Set the vertex position from a Point."""
        self.x = point.x
        self.y = point.y
        self.z = point.z

    def color(self) -> List[float]:
        """Get the vertex color as [r, g, b]."""
        return [
            self.attributes.get("r", 0.5),
            self.attributes.get("g", 0.5),
            self.attributes.get("b", 0.5),
        ]

    def set_color(self, r: float, g: float, b: float):
        """Set the vertex color."""
        self.attributes["r"] = r
        self.attributes["g"] = g
        self.attributes["b"] = b

    def normal(self) -> Optional[List[float]]:
        """Get the vertex normal as [nx, ny, nz]."""
        if (
            "nx" in self.attributes
            and "ny" in self.attributes
            and "nz" in self.attributes
        ):
            return [self.attributes["nx"], self.attributes["ny"], self.attributes["nz"]]
        return None

    def set_normal(self, nx: float, ny: float, nz: float):
        """Set the vertex normal."""
        self.attributes["nx"] = nx
        self.attributes["ny"] = ny
        self.attributes["nz"] = nz


class Mesh:
    """A halfedge mesh data structure for representing polygonal surfaces.

    Attributes
    ----------
    halfedge : dict
        Halfedge connectivity structure mapping vertex pairs to faces.
    vertex : dict
        Vertex data dictionary mapping vertex keys to VertexData.
    face : dict
        Face vertex lists mapping face keys to vertex key lists.
    facedata : dict
        Face attributes dictionary.
    edgedata : dict
        Edge attributes dictionary.
    default_vertex_attributes : dict
        Default attributes for new vertices.
    default_face_attributes : dict
        Default attributes for new faces.
    default_edge_attributes : dict
        Default attributes for new edges.
    guid : str
        Unique identifier for the mesh.
    name : str
        Name of the mesh.
    pointcolors : list
        Vertex colors (one per vertex).
    facecolors : list
        Face colors (one per face).
    linecolors : list
        Edge colors (one per edge).
    widths : list
        Edge widths (one per edge).
    """

    def __init__(self):
        self.halfedge = {}
        self.vertex = {}
        self.face = {}
        self.facedata = {}
        self.edgedata = {}
        self.default_vertex_attributes = {"x": 0.0, "y": 0.0, "z": 0.0}
        self.default_face_attributes = {}
        self.default_edge_attributes = {}
        self.triangulation = {}
        self._max_vertex = 0
        self._max_face = 0
        self.guid = str(uuid.uuid4())
        self.name = "my_mesh"
        self.pointcolors = []
        self.facecolors = []
        self.linecolors = []
        self.widths = []
        self.xform = Xform.identity()

    ###########################################################################################
    # Basic Queries
    ###########################################################################################

    def number_of_vertices(self) -> int:
        """Get the number of vertices."""
        return len(self.vertex)

    def number_of_faces(self) -> int:
        """Get the number of faces."""
        return len(self.face)

    def number_of_edges(self) -> int:
        """Get the number of edges."""
        seen = set()
        count = 0
        for u in self.halfedge:
            for v in self.halfedge[u]:
                edge = tuple(sorted([u, v]))
                if edge not in seen:
                    seen.add(edge)
                    count += 1
        return count

    def is_empty(self) -> bool:
        """Check if the mesh is empty."""
        return len(self.vertex) == 0

    def euler(self) -> int:
        """Calculate Euler characteristic (V - E + F)."""
        return (
            self.number_of_vertices() - self.number_of_edges() + self.number_of_faces()
        )

    def clear(self):
        """Clear all mesh data."""
        self.halfedge.clear()
        self.vertex.clear()
        self.face.clear()
        self.facedata.clear()
        self.edgedata.clear()
        self.triangulation.clear()
        self._max_vertex = 0
        self._max_face = 0
        self.pointcolors.clear()
        self.facecolors.clear()
        self.linecolors.clear()
        self.widths.clear()

    ###########################################################################################
    # Vertex and Face Operations
    ###########################################################################################

    def add_vertex(self, position: Point, vkey: Optional[int] = None) -> int:
        """Add a vertex to the mesh.

        Parameters
        ----------
        position : Point
            The position of the vertex.
        vkey : int, optional
            Optional vertex key. If None, auto-generated.

        Returns
        -------
        int
            The vertex key.
        """
        if vkey is None:
            self._max_vertex += 1
            vertex_key = self._max_vertex
        else:
            vertex_key = vkey

        if vertex_key >= self._max_vertex:
            self._max_vertex = vertex_key + 1

        self.vertex[vertex_key] = VertexData(position)
        self.halfedge[vertex_key] = {}
        self.pointcolors.append(Color.white())

        return vertex_key

    def add_face(
        self, vertices: List[int], fkey: Optional[int] = None
    ) -> Optional[int]:
        """Add a face to the mesh.

        Parameters
        ----------
        vertices : list of int
            The vertex keys forming the face.
        fkey : int, optional
            Optional face key. If None, auto-generated.

        Returns
        -------
        int or None
            The face key, or None if the face is invalid.
        """
        if len(vertices) < 3:
            return None

        if not all(v in self.vertex for v in vertices):
            return None

        if len(set(vertices)) != len(vertices):
            return None

        if fkey is None:
            self._max_face += 1
            face_key = self._max_face
        else:
            face_key = fkey

        if face_key >= self._max_face:
            self._max_face = face_key + 1

        self.face[face_key] = vertices.copy()
        self.triangulation.pop(face_key, None)
        self.facecolors.append(Color.white())

        for i in range(len(vertices)):
            u = vertices[i]
            v = vertices[(i + 1) % len(vertices)]

            if u not in self.halfedge:
                self.halfedge[u] = {}
            if v not in self.halfedge:
                self.halfedge[v] = {}

            is_new_edge = u not in self.halfedge[v]

            self.halfedge[u][v] = face_key

            if is_new_edge:
                self.halfedge[v][u] = None
                self.linecolors.append(Color.white())
                self.widths.append(1.0)

        return face_key

    ###########################################################################################
    # Connectivity Queries
    ###########################################################################################

    def vertex_position(self, vertex_key: int) -> Optional[Point]:
        """Get the position of a vertex."""
        if vertex_key not in self.vertex:
            return None
        return self.vertex[vertex_key].position()

    def face_vertices(self, face_key: int) -> Optional[List[int]]:
        """Get the vertices of a face."""
        return self.face.get(face_key)

    def vertex_neighbors(self, vertex_key: int) -> List[int]:
        """Get the neighboring vertices of a vertex."""
        if vertex_key not in self.halfedge:
            return []
        return list(self.halfedge[vertex_key].keys())

    def vertex_faces(self, vertex_key: int) -> List[int]:
        """Get the faces incident to a vertex."""
        faces = []
        for face_key, face_vertices in self.face.items():
            if vertex_key in face_vertices:
                faces.append(face_key)
        return faces

    def is_vertex_on_boundary(self, vertex_key: int) -> bool:
        """Check if a vertex is on the boundary."""
        if vertex_key not in self.halfedge:
            return False

        for v, face_opt in self.halfedge[vertex_key].items():
            if face_opt is None:
                return True

        for u, neighbors in self.halfedge.items():
            if vertex_key in neighbors and neighbors[vertex_key] is None:
                return True

        return False

    ###########################################################################################
    # Geometric Properties
    ###########################################################################################

    def face_normal(self, face_key: int) -> Optional[Vector]:
        """Calculate the normal of a face."""
        vertices = self.face_vertices(face_key)
        if vertices is None or len(vertices) < 3:
            return None

        p0 = self.vertex_position(vertices[0])
        p1 = self.vertex_position(vertices[1])
        p2 = self.vertex_position(vertices[2])

        if p0 is None or p1 is None or p2 is None:
            return None

        u = Vector(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z)
        v = Vector(p2.x - p0.x, p2.y - p0.y, p2.z - p0.z)

        normal = u.cross(v)
        length = normal.magnitude()

        if length > Tolerance.ZERO_TOLERANCE:
            return Vector(normal.x / length, normal.y / length, normal.z / length)

        return None

    def vertex_normal(self, vertex_key: int) -> Optional[Vector]:
        """Calculate the normal of a vertex (area-weighted)."""
        return self.vertex_normal_weighted(vertex_key, NormalWeighting.AREA)

    def vertex_normal_weighted(
        self, vertex_key: int, weighting: NormalWeighting
    ) -> Optional[Vector]:
        """Calculate the normal of a vertex with specified weighting."""
        faces = self.vertex_faces(vertex_key)
        if not faces:
            return None

        normal_acc = Vector(0.0, 0.0, 0.0)

        for face_key in faces:
            face_normal = self.face_normal(face_key)
            if face_normal is None:
                continue

            if weighting == NormalWeighting.AREA:
                weight = self.face_area(face_key) or 1.0
            elif weighting == NormalWeighting.ANGLE:
                weight = self.vertex_angle_in_face(vertex_key, face_key) or 1.0
            else:  # UNIFORM
                weight = 1.0

            normal_acc.x += face_normal.x * weight
            normal_acc.y += face_normal.y * weight
            normal_acc.z += face_normal.z * weight

        length = normal_acc.magnitude()
        if length > Tolerance.ZERO_TOLERANCE:
            return Vector(
                normal_acc.x / length, normal_acc.y / length, normal_acc.z / length
            )

        return None

    def face_area(self, face_key: int) -> Optional[float]:
        """Calculate the area of a face."""
        vertices = self.face_vertices(face_key)
        if vertices is None or len(vertices) < 3:
            return 0.0

        area = 0.0
        p0 = self.vertex_position(vertices[0])
        if p0 is None:
            return None

        for i in range(1, len(vertices) - 1):
            p1 = self.vertex_position(vertices[i])
            p2 = self.vertex_position(vertices[i + 1])
            if p1 is None or p2 is None:
                return None

            u = Vector(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z)
            v = Vector(p2.x - p0.x, p2.y - p0.y, p2.z - p0.z)

            area += u.cross(v).magnitude() * 0.5

        return area

    def vertex_angle_in_face(self, vertex_key: int, face_key: int) -> Optional[float]:
        """Calculate the angle at a vertex in a face."""
        vertices = self.face_vertices(face_key)
        if vertices is None or vertex_key not in vertices:
            return None

        vertex_index = vertices.index(vertex_key)
        n = len(vertices)
        prev_vertex = vertices[(vertex_index - 1) % n]
        next_vertex = vertices[(vertex_index + 1) % n]

        center = self.vertex_position(vertex_key)
        prev_pos = self.vertex_position(prev_vertex)
        next_pos = self.vertex_position(next_vertex)

        if center is None or prev_pos is None or next_pos is None:
            return None

        u = Vector(prev_pos.x - center.x, prev_pos.y - center.y, prev_pos.z - center.z)
        v = Vector(next_pos.x - center.x, next_pos.y - center.y, next_pos.z - center.z)

        u_len = u.magnitude()
        v_len = v.magnitude()

        if u_len < Tolerance.ZERO_TOLERANCE or v_len < Tolerance.ZERO_TOLERANCE:
            return 0.0

        cos_angle = u.dot(v) / (u_len * v_len)
        cos_angle = max(-1.0, min(1.0, cos_angle))
        return math.acos(cos_angle)

    def face_normals(self) -> Dict[int, Vector]:
        """Calculate normals for all faces."""
        normals = {}
        for face_key in self.face:
            normal = self.face_normal(face_key)
            if normal is not None:
                normals[face_key] = normal
        return normals

    def vertex_normals(self) -> Dict[int, Vector]:
        """Calculate normals for all vertices (area-weighted)."""
        return self.vertex_normals_weighted(NormalWeighting.AREA)

    def vertex_normals_weighted(self, weighting: NormalWeighting) -> Dict[int, Vector]:
        """Calculate normals for all vertices with specified weighting."""
        normals = {}
        for vertex_key in self.vertex:
            normal = self.vertex_normal_weighted(vertex_key, weighting)
            if normal is not None:
                normals[vertex_key] = normal
        return normals

    ###########################################################################################
    # Construction
    ###########################################################################################

    @staticmethod
    def from_polygons(
        polygons: List[List[Point]], precision: Optional[float] = None
    ) -> "Mesh":
        """Create a mesh from a list of polygons.

        Parameters
        ----------
        polygons : list of list of Point
            List of polygons, each polygon is a list of points.
        precision : float, optional
            Precision for vertex merging. If None, exact matching is used.

        Returns
        -------
        Mesh
            The constructed mesh with merged vertices.
        """
        mesh = Mesh()
        map_eps = {}
        map_exact = {}

        def get_vkey(p: Point) -> int:
            if precision is not None:
                kx = round(p.x / precision)
                ky = round(p.y / precision)
                kz = round(p.z / precision)
                key = (kx, ky, kz)
                if key in map_eps:
                    return map_eps[key]
                vk = mesh.add_vertex(p)
                map_eps[key] = vk
                return vk
            else:
                key = (p.x, p.y, p.z)
                if key in map_exact:
                    return map_exact[key]
                vk = mesh.add_vertex(p)
                map_exact[key] = vk
                return vk

        for poly in polygons:
            if len(poly) < 3:
                continue
            vkeys = [get_vkey(p) for p in poly]
            mesh.add_face(vkeys)

        return mesh

    ###########################################################################################
    # COMPAS-style Export Methods
    ###########################################################################################

    def vertex_index(self) -> Dict[int, int]:
        """Create a mapping from sparse vertex keys to sequential indices.

        Returns
        -------
        dict[int, int]
            A dictionary mapping vertex_key -> sequential_index (0, 1, 2, ...).
        """
        # Sort keys to ensure consistent ordering
        sorted_keys = sorted(self.vertex.keys())
        return {key: index for index, key in enumerate(sorted_keys)}

    def to_vertices_and_faces(self) -> Tuple[List[Point], List[List[int]]]:
        """Export vertices and faces with sequential 0-based indices.

        Returns
        -------
        tuple
            A tuple of (vertices, faces) where:
            - vertices: List of Point objects in sequential order
            - faces: List of face vertex lists using sequential indices
        """
        vertex_idx = self.vertex_index()
        vertices = [None] * len(self.vertex)

        for key, vdata in self.vertex.items():
            idx = vertex_idx[key]
            vertices[idx] = vdata.position()

        # Sort face keys to ensure consistent ordering
        sorted_face_keys = sorted(self.face.keys())
        faces = []
        for face_key in sorted_face_keys:
            face_vertices = self.face[face_key]
            remapped = [vertex_idx[v] for v in face_vertices]
            faces.append(remapped)

        return vertices, faces

    ###########################################################################################
    # JSON
    ###########################################################################################

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field.

        Returns
        -------
        dict
            Dictionary with 'type', 'guid', 'name', and object fields.

        """
        # Halfedge connectivity
        halfedge_data = {}
        for u, neighbors in self.halfedge.items():
            halfedge_data[str(u)] = {
                str(v): face_key for v, face_key in neighbors.items()
            }

        # Vertex data
        vertex_data = {}
        for key, vdata in self.vertex.items():
            vertex_data[str(key)] = {
                "x": vdata.x,
                "y": vdata.y,
                "z": vdata.z,
                "attributes": vdata.attributes,
            }

        # Face data
        face_data = {}
        for key, vertices in self.face.items():
            face_data[str(key)] = vertices

        # Face attributes
        facedata_json = {}
        for key, attrs in self.facedata.items():
            facedata_json[str(key)] = attrs

        # Edge attributes
        edgedata_json = {}
        for (u, v), attrs in self.edgedata.items():
            edgedata_json[f"{u},{v}"] = attrs

        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "halfedge": halfedge_data,
            "vertex": vertex_data,
            "face": face_data,
            "facedata": facedata_json,
            "edgedata": edgedata_json,
            "default_vertex_attributes": self.default_vertex_attributes,
            "default_face_attributes": self.default_face_attributes,
            "default_edge_attributes": self.default_edge_attributes,
            "max_vertex": self._max_vertex,
            "max_face": self._max_face,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format.

        Parameters
        ----------
        data : dict
            Dictionary containing mesh data.
        guid : str, optional
            GUID for the mesh.
        name : str, optional
            Name for the mesh.

        Returns
        -------
        :class:`Mesh`
            Reconstructed mesh instance.

        """
        mesh = cls()
        mesh.guid = guid if guid is not None else data.get("guid", mesh.guid)
        mesh.name = name if name is not None else data.get("name", mesh.name)

        # Load halfedge connectivity
        if "halfedge" in data:
            for u_str, neighbors in data["halfedge"].items():
                u = int(u_str)
                mesh.halfedge[u] = {}
                for v_str, face_key in neighbors.items():
                    v = int(v_str)
                    mesh.halfedge[u][v] = face_key

        # Load vertex data
        if "vertex" in data:
            for key_str, vdata in data["vertex"].items():
                key = int(key_str)
                vertex_data = VertexData()
                vertex_data.x = vdata["x"]
                vertex_data.y = vdata["y"]
                vertex_data.z = vdata["z"]
                if "attributes" in vdata:
                    vertex_data.attributes = vdata["attributes"]
                mesh.vertex[key] = vertex_data
                if "halfedge" not in data:
                    mesh.halfedge[key] = {}
                if key >= mesh._max_vertex:
                    mesh._max_vertex = key + 1

        # Load face data
        if "face" in data:
            for key_str, vertices in data["face"].items():
                key = int(key_str)
                mesh.face[key] = vertices
                if key >= mesh._max_face:
                    mesh._max_face = key + 1

        # Load face attributes
        if "facedata" in data:
            for key_str, attrs in data["facedata"].items():
                key = int(key_str)
                mesh.facedata[key] = attrs

        # Load edge attributes
        if "edgedata" in data:
            for edge_str, attrs in data["edgedata"].items():
                u, v = map(int, edge_str.split(","))
                mesh.edgedata[(u, v)] = attrs

        if "default_vertex_attributes" in data:
            mesh.default_vertex_attributes = data["default_vertex_attributes"]
        if "default_face_attributes" in data:
            mesh.default_face_attributes = data["default_face_attributes"]
        if "default_edge_attributes" in data:
            mesh.default_edge_attributes = data["default_edge_attributes"]

        if "max_vertex" in data:
            mesh._max_vertex = data["max_vertex"]
        if "max_face" in data:
            mesh._max_face = data["max_face"]

        if "xform" in data:
            mesh.xform = decode_node(data["xform"])

        return mesh

    ###########################################################################################
    # Transformation
    ###########################################################################################

    def transform(self):
        """Apply the stored xform transformation to the mesh.

        Transforms all vertices in-place and resets xform to identity.
        """
        from .xform import Xform

        for vdata in self.vertex.values():
            pos = vdata.position()
            self.xform.transform_point(pos)
            vdata.x = pos.x
            vdata.y = pos.y
            vdata.z = pos.z
        self.xform = Xform.identity()

    def transformed(self):
        """Return a transformed copy of the mesh."""
        import copy

        result = copy.deepcopy(self)
        result.transform()
        return result

    ###########################################################################################
    # Color and Width Management
    ###########################################################################################

    def set_vertex_color(self, index: int, color: Color):
        """Set color for a specific vertex."""
        if 0 <= index < len(self.pointcolors):
            self.pointcolors[index] = color

    def set_face_color(self, index: int, color: Color):
        """Set color for a specific face."""
        if 0 <= index < len(self.facecolors):
            self.facecolors[index] = color

    def set_edge_color(self, index: int, color: Color):
        """Set color for a specific edge."""
        if 0 <= index < len(self.linecolors):
            self.linecolors[index] = color

    def set_edge_width(self, index: int, width: float):
        """Set width for a specific edge."""
        if 0 <= index < len(self.widths):
            self.widths[index] = width
