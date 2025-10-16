import uuid
import math
from .color import Color
from .xform import Xform
from .vector import Vector


class Point:
    """A 3D point with visual properties.

    Parameters
    ----------
    x : float, optional
        X coordinate. Defaults to 0.0.
    y : float, optional
        Y coordinate. Defaults to 0.0.
    z : float, optional
        Z coordinate. Defaults to 0.0.

    Attributes
    ----------
    name : str
        The name of the point.
    guid : str
        The unique identifier of the point.
    x : float
        The X coordinate of the point.
    y : float
        The Y coordinate of the point.
    z : float
        The Z coordinate of the point.
    pointcolor : :class:`Color`
        The color of the point.
    width : float
        The width of the point for display.

    """

    def __init__(self, x=0.0, y=0.0, z=0.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_point"
        self._x = x
        self._y = y
        self._z = z
        self.width = 1.0
        self.pointcolor = Color.white()
        self.xform = Xform.identity()

    @property
    def x(self):
        """Get the X coordinate."""
        return self._x

    @x.setter
    def x(self, value):
        """Set the X coordinate."""
        self._x = value

    @property
    def y(self):
        """Get the Y coordinate."""
        return self._y

    @y.setter
    def y(self, value):
        """Set the Y coordinate."""
        self._y = value

    @property
    def z(self):
        """Get the Z coordinate."""
        return self._z

    @z.setter
    def z(self, value):
        """Set the Z coordinate."""
        self._z = value

    def __str__(self):
        from .tolerance import TOL

        return f"Point(x={TOL.format_number(self.x)}, y={TOL.format_number(self.y)}, z={TOL.format_number(self.z)})"

    def __repr__(self):
        return f"Point({self.x}, {self.y}, {self.z}, {self.guid}, {self.name}, {self.pointcolor}, {self.width})"

    def __eq__(self, other):
        return (
            self.name == other.name
            and round(self.x, 6) == round(other.x, 6)
            and round(self.y, 6) == round(other.y, 6)
            and round(self.z, 6) == round(other.z, 6)
            and round(self.width, 6) == round(other.width, 6)
            and self.pointcolor == other.pointcolor
        )

    def __ne__(self, other):
        return not self == other

    ###########################################################################################
    # Transformation
    ###########################################################################################

    def transform(self):
        """Apply the stored xform transformation to the point coordinates.

        Transforms the point in-place and resets xform to identity.
        """
        self.xform.transform_point(self)
        self.xform = Xform.identity()

    def transformed(self):
        """Return a transformed copy of the point.

        Returns a new point with the transformation applied.
        The original point and its xform remain unchanged.

        Returns
        -------
        Point
            A new transformed point.
        """
        import copy

        result = copy.deepcopy(self)
        result.transform()
        return result

    ###########################################################################################
    # No-copy Operators
    ###########################################################################################

    def __getitem__(self, index):
        if index == 0:
            return self.x
        elif index == 1:
            return self.y
        elif index == 2:
            return self.z
        else:
            raise IndexError("Index out of range")

    def __setitem__(self, index, value):
        if index == 0:
            self.x = value
        elif index == 1:
            self.y = value
        elif index == 2:
            self.z = value
        else:
            raise IndexError("Index out of range")

    def __imul__(self, other):
        self.x *= other
        self.y *= other
        self.z *= other
        return self

    def __itruediv__(self, other):
        self.x /= other
        self.y /= other
        self.z /= other
        return self

    def __iadd__(self, other):
        if isinstance(other, Vector):
            self.x += other.x
            self.y += other.y
            self.z += other.z
        else:
            raise TypeError("Point can only be added with Vector")
        return self

    def __isub__(self, other):
        if isinstance(other, Vector):
            self.x -= other.x
            self.y -= other.y
            self.z -= other.z
        else:
            raise TypeError("Point can only be subtracted with Vector")
        return self

    ###########################################################################################
    # Copy Operators
    ###########################################################################################

    def __mul__(self, other):
        return Point(self.x * other, self.y * other, self.z * other)

    def __truediv__(self, other):
        return Point(self.x / other, self.y / other, self.z / other)

    def __add__(self, other):
        if isinstance(other, Vector):
            return Point(self.x + other.x, self.y + other.y, self.z + other.z)
        raise TypeError("Point can only be added with Vector")

    def __sub__(self, other):
        if isinstance(other, Point):
            return Vector(self.x - other.x, self.y - other.y, self.z - other.z)
        elif isinstance(other, Vector):
            return Point(self.x - other.x, self.y - other.y, self.z - other.z)
        raise TypeError(
            "Point can be subtracted with Point (returns Vector) or Vector (returns Point)"
        )

    ###########################################################################################
    # Details
    ###########################################################################################

    @staticmethod
    def ccw(a, b, c):
        """Check if the points are in counter-clockwise order.

        Parameters
        ----------
        a : :class:`Point`
            First point.
        b : :class:`Point`
            Second point.
        c : :class:`Point`
            Third point.

        Returns
        -------
        bool
            True if the points are in counter-clockwise order, False otherwise.

        """

        return (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)

    def mid_point(self, p):
        """Calculate the mid point between this point and another point.

        Parameters
        ----------
        p : :class:`Point`
            The other point.

        Returns
        -------
        :class:`Point`
            The mid point between this point and the other point.

        """

        return Point((self.x + p.x) / 2, (self.y + p.y) / 2, (self.z + p.z) / 2)

    def distance(self, p, double_min=1e-12):
        """Calculate the distance between this point and another point.

        Parameters
        ----------
        p : :class:`Point`
            The other point.
        double_min : float, optional
            The minimum value for the distance. Defaults to 1e-12.

        Returns
        -------
        float
            The distance between this point and the other point.

        """

        x = abs(self[0] - p[0])
        y = abs(self[1] - p[1])
        z = abs(self[2] - p[2])
        length = 0.0

        if y >= x and y >= z:
            length, x, y = x, y, x
        elif z >= x and z >= y:
            length, x, z = x, z, x

        if x > double_min:
            y /= x
            z /= x
            length = x * math.sqrt(1.0 + y * y + z * z)
        elif x > 0.0 and math.isfinite(x):
            length = x
        else:
            length = 0.0

        return length

    @staticmethod
    def area(points):
        """Calculate the area of a polygon.

        Parameters
        ----------
        points : list of :class:`Point`
            The points of the polygon.

        Returns
        -------
        float
            The area of the polygon.

        """

        n = len(points)
        area = 0.0
        for i in range(n):
            j = (i + 1) % n
            area += points[i][0] * points[j][1]
            area -= points[j][0] * points[i][1]

        return abs(area) / 2.0

    @staticmethod
    def centroid_quad(vertices):
        """Calculate the centroid of a quadrilateral.

        Parameters
        ----------
        vertices : list of :class:`Point`
            The vertices of the quadrilateral.

        Returns
        -------
        :class:`Point`
            The centroid of the quadrilateral.

        """

        if len(vertices) != 4:
            raise ValueError("Polygon must have exactly 4 vertices.")

        total_area = 0.0
        centroid_sum = Vector(0, 0, 0)

        for i in range(4):
            p0, p1, p2 = vertices[i], vertices[(i + 1) % 4], vertices[(i + 2) % 4]
            tri_area = (
                abs(
                    p0[0] * (p1[1] - p2[1])
                    + p1[0] * (p2[1] - p0[1])
                    + p2[0] * (p0[1] - p1[1])
                )
                / 2.0
            )
            total_area += tri_area
            tri_centroid = Vector(
                (p0[0] + p1[0] + p2[0]) / 3.0,
                (p0[1] + p1[1] + p2[1]) / 3.0,
                (p0[2] + p1[2] + p2[2]) / 3.0,
            )
            centroid_sum += tri_centroid * tri_area

        result = centroid_sum / total_area
        return Point(result.x, result.y, result.z)

    ###########################################################################################
    # Polymorphic JSON Serialization (COMPAS-style)
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
            "x": self.x,
            "y": self.y,
            "z": self.z,
            "width": self.width,
            "pointcolor": self.pointcolor.__jsondump__(),
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format."""
        from .encoders import decode_node

        pt = cls(data["x"], data["y"], data["z"])
        pt.width = data.get("width", 1.0)

        # Decode nested color (supports polymorphic dicts and plain values)
        pt.pointcolor = decode_node(data.get("pointcolor"))

        # Always assign metadata (per project convention)
        pt.guid = guid
        pt.name = name

        if "xform" in data:
            pt.xform = decode_node(data["xform"])

        return pt
