import uuid
from typing import List, Tuple
from .point import Point
from .vector import Vector
from .line import Line
from .xform import Xform
from .mesh import Mesh


class Arrow:
    """An arrow geometry defined by a line and radius, the head is uniformly scaled.

    The arrow is generated as a 10-sided cylinder body and an 8-sided cone head
    that is oriented along the line direction and scaled to match the line length and specified radius.

    Attributes
    ----------
    guid : str
        Unique identifier for the arrow.
    name : str
        Name of the arrow.
    radius : float
        The radius of the arrow body.
    line : Line
        The centerline of the arrow.
    mesh : Mesh
        The generated arrow mesh (cylinder body + cone head).
    """

    def __init__(self, line: Line, radius: float):
        """Creates a new Arrow from a line and radius.

        Parameters
        ----------
        line : Line
            The centerline of the arrow.
        radius : float
            The radius of the arrow body.
        """
        self.guid = str(uuid.uuid4())
        self.name = "my_arrow"
        self.radius = radius
        self.line = line
        self.mesh = self._create_arrow_mesh(line, radius)

    @staticmethod
    def _unit_cylinder_geometry() -> Tuple[List[Point], List[List[int]]]:
        """Generate the unit cylinder geometry."""
        vertices = [
            Point(0.5, 0.0, -0.5),
            Point(0.404508, 0.293893, -0.5),
            Point(0.154508, 0.475528, -0.5),
            Point(-0.154508, 0.475528, -0.5),
            Point(-0.404508, 0.293893, -0.5),
            Point(-0.5, 0.0, -0.5),
            Point(-0.404508, -0.293893, -0.5),
            Point(-0.154508, -0.475528, -0.5),
            Point(0.154508, -0.475528, -0.5),
            Point(0.404508, -0.293893, -0.5),
            Point(0.5, 0.0, 0.5),
            Point(0.404508, 0.293893, 0.5),
            Point(0.154508, 0.475528, 0.5),
            Point(-0.154508, 0.475528, 0.5),
            Point(-0.404508, 0.293893, 0.5),
            Point(-0.5, 0.0, 0.5),
            Point(-0.404508, -0.293893, 0.5),
            Point(-0.154508, -0.475528, 0.5),
            Point(0.154508, -0.475528, 0.5),
            Point(0.404508, -0.293893, 0.5),
        ]

        triangles = [
            [0, 1, 11],
            [0, 11, 10],
            [1, 2, 12],
            [1, 12, 11],
            [2, 3, 13],
            [2, 13, 12],
            [3, 4, 14],
            [3, 14, 13],
            [4, 5, 15],
            [4, 15, 14],
            [5, 6, 16],
            [5, 16, 15],
            [6, 7, 17],
            [6, 17, 16],
            [7, 8, 18],
            [7, 18, 17],
            [8, 9, 19],
            [8, 19, 18],
            [9, 0, 10],
            [9, 10, 19],
        ]

        return vertices, triangles

    @staticmethod
    def _unit_cone_geometry() -> Tuple[List[Point], List[List[int]]]:
        """Generate the unit cone geometry."""
        vertices = [
            Point(0.0, 0.0, 0.5),
            Point(0.5, 0.0, -0.5),
            Point(0.353553, -0.353553, -0.5),
            Point(0.0, -0.5, -0.5),
            Point(-0.353553, -0.353553, -0.5),
            Point(-0.5, 0.0, -0.5),
            Point(-0.353553, 0.353553, -0.5),
            Point(0.0, 0.5, -0.5),
            Point(0.353553, 0.353553, -0.5),
        ]

        triangles = [
            [0, 2, 1],
            [0, 3, 2],
            [0, 4, 3],
            [0, 5, 4],
            [0, 6, 5],
            [0, 7, 6],
            [0, 8, 7],
            [0, 1, 8],
        ]

        return vertices, triangles

    @classmethod
    def _create_arrow_mesh(cls, line: Line, radius: float) -> Mesh:
        """Create the arrow mesh (cylinder body + cone head)."""
        start = line.start()
        line_vec = line.to_vector()
        length = line.length()

        z_axis = line_vec.normalize()
        if abs(z_axis.z) < 0.9:
            x_axis = Vector(0.0, 0.0, 1.0).cross(z_axis).normalize()
        else:
            x_axis = Vector(1.0, 0.0, 0.0).cross(z_axis).normalize()
        y_axis = z_axis.cross(x_axis).normalize()

        cone_length = length * 0.2
        body_length = length * 0.8

        body_center = Point(
            start.x + line_vec.x * 0.4,
            start.y + line_vec.y * 0.4,
            start.z + line_vec.z * 0.4,
        )

        cone_base_center = Point(
            start.x + line_vec.x * 0.9,
            start.y + line_vec.y * 0.9,
            start.z + line_vec.z * 0.9,
        )

        # Create body transformation
        body_scale = Xform.scale_xyz(radius * 2.0, radius * 2.0, body_length)
        rotation = Xform()
        rotation.m[0] = x_axis.x
        rotation.m[1] = x_axis.y
        rotation.m[2] = x_axis.z
        rotation.m[4] = y_axis.x
        rotation.m[5] = y_axis.y
        rotation.m[6] = y_axis.z
        rotation.m[8] = z_axis.x
        rotation.m[9] = z_axis.y
        rotation.m[10] = z_axis.z
        body_translation = Xform.translation(
            body_center.x, body_center.y, body_center.z
        )
        body_xform = body_translation * rotation * body_scale

        # Create cone transformation
        cone_scale = Xform.scale_xyz(radius * 3.0, radius * 3.0, cone_length)
        cone_translation = Xform.translation(
            cone_base_center.x, cone_base_center.y, cone_base_center.z
        )
        cone_xform = cone_translation * rotation * cone_scale

        # Get geometries
        body_geometry = cls._unit_cylinder_geometry()
        cone_geometry = cls._unit_cone_geometry()

        # Create mesh
        mesh = Mesh()

        # Add body vertices and faces
        body_vertex_map = []
        for v in body_geometry[0]:
            transformed = body_xform.transformed_point(v)
            key = mesh.add_vertex(transformed)
            body_vertex_map.append(key)

        for tri in body_geometry[1]:
            face_vertices = [
                body_vertex_map[tri[0]],
                body_vertex_map[tri[1]],
                body_vertex_map[tri[2]],
            ]
            mesh.add_face(face_vertices)

        # Add cone vertices and faces
        cone_vertex_map = []
        for v in cone_geometry[0]:
            transformed = cone_xform.transformed_point(v)
            key = mesh.add_vertex(transformed)
            cone_vertex_map.append(key)

        for tri in cone_geometry[1]:
            face_vertices = [
                cone_vertex_map[tri[0]],
                cone_vertex_map[tri[1]],
                cone_vertex_map[tri[2]],
            ]
            mesh.add_face(face_vertices)

        return mesh

    ###########################################################################################
    # Transformation
    ###########################################################################################

    def transform(self):
        """Apply the stored xform transformation to the arrow.

        Transforms the line in-place and resets xform to identity.
        """
        from .xform import Xform

        self.line.transform()  # Transform the line component
        self.xform = Xform.identity()

    def transformed(self):
        """Return a transformed copy of the arrow."""
        import copy

        result = copy.deepcopy(self)
        result.transform()
        return result

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
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "radius": self.radius,
            "line": self.line.__jsondump__(),
            "mesh": self.mesh.__jsondump__(),
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format.

        Parameters
        ----------
        data : dict
            Dictionary containing arrow data.
        guid : str, optional
            GUID for the arrow.
        name : str, optional
            Name for the arrow.

        Returns
        -------
        :class:`Arrow`
            Reconstructed arrow instance.

        """
        from .encoders import decode_node

        line = decode_node(data["line"])
        radius = data["radius"]
        arrow = cls(line, radius)
        arrow.guid = guid
        arrow.name = name

        if "xform" in data:
            arrow.xform = decode_node(data["xform"])

        return arrow
