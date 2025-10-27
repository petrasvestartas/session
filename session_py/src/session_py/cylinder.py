import uuid
from typing import List, Tuple
from .point import Point
from .vector import Vector
from .line import Line
from .xform import Xform
from .mesh import Mesh


class Cylinder:
    """A cylinder geometry defined by a line and radius.

    The cylinder is generated as a 10-sided cylinder mesh that is oriented
    along the line direction and scaled to match the line length and specified radius.

    Attributes
    ----------
    guid : str
        Unique identifier for the cylinder.
    name : str
        Name of the cylinder.
    radius : float
        The radius of the cylinder.
    line : Line
        The centerline of the cylinder.
    mesh : Mesh
        The generated 10-sided cylinder mesh.
    """

    def __init__(self, line: Line, radius: float):
        """Creates a new Cylinder from a line and radius.

        Parameters
        ----------
        line : Line
            The centerline of the cylinder.
        radius : float
            The radius of the cylinder.
        """
        self.guid = str(uuid.uuid4())
        self.name = "my_cylinder"
        self.radius = radius
        self.line = line
        self.mesh = self._create_cylinder_mesh(line, radius)

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
    def _line_to_cylinder_transform(line: Line, radius: float) -> Xform:
        """Create transformation from unit cylinder to oriented cylinder."""
        start = line.start()
        end = line.end()
        line_vec = line.to_vector()
        length = line.length()

        z_axis = line_vec.normalize()
        if abs(z_axis.z) < 0.9:
            x_axis = Vector(0.0, 0.0, 1.0).cross(z_axis).normalize()
        else:
            x_axis = Vector(1.0, 0.0, 0.0).cross(z_axis).normalize()
        y_axis = z_axis.cross(x_axis).normalize()

        scale = Xform.scale_xyz(radius * 2.0, radius * 2.0, length)

        # Create rotation matrix from column vectors
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

        center = Point(
            (start.x + end.x) * 0.5, (start.y + end.y) * 0.5, (start.z + end.z) * 0.5
        )
        translation = Xform.translation(center.x, center.y, center.z)

        return translation * rotation * scale

    @staticmethod
    def _transform_geometry(
        geometry: Tuple[List[Point], List[List[int]]], xform: Xform
    ) -> Mesh:
        """Transform unit geometry and create mesh."""
        vertices, triangles = geometry
        mesh = Mesh()

        vertex_keys = []
        for v in vertices:
            transformed = xform.transformed_point(v)
            vertex_keys.append(mesh.add_vertex(transformed))

        for tri in triangles:
            face_vertices = [
                vertex_keys[tri[0]],
                vertex_keys[tri[1]],
                vertex_keys[tri[2]],
            ]
            mesh.add_face(face_vertices)

        return mesh

    @classmethod
    def _create_cylinder_mesh(cls, line: Line, radius: float) -> Mesh:
        """Create the cylinder mesh."""
        unit_cylinder = cls._unit_cylinder_geometry()
        xform = cls._line_to_cylinder_transform(line, radius)
        return cls._transform_geometry(unit_cylinder, xform)

    ###########################################################################################
    # Transformation
    ###########################################################################################

    def transform(self):
        """Apply the stored xform transformation to the cylinder.

        Transforms the line in-place and resets xform to identity.
        """
        from .xform import Xform

        self.line.transform()  # Transform the line component
        self.xform = Xform.identity()

    def transformed(self):
        """Return a transformed copy of the cylinder."""
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
            Dictionary containing cylinder data.
        guid : str, optional
            GUID for the cylinder.
        name : str, optional
            Name for the cylinder.

        Returns
        -------
        :class:`Cylinder`
            Reconstructed cylinder instance.

        """
        from .encoders import decode_node

        line = decode_node(data["line"])
        radius = data["radius"]
        cylinder = cls(line, radius)
        cylinder.guid = guid
        cylinder.name = name

        if "xform" in data:
            cylinder.xform = decode_node(data["xform"])

        return cylinder
