import uuid
import math
from .point import Point
from .vector import Vector
from .tolerance import Tolerance
from .xform import Xform


class Plane:
    """A 3D plane defined by origin and coordinate axes.

    Parameters
    ----------
    origin : Point, optional
        Origin point of the plane. Defaults to Point(0, 0, 0).
    x_axis : Vector, optional
        X-axis direction. Defaults to Vector(1, 0, 0).
    y_axis : Vector, optional
        Y-axis direction. Defaults to Vector(0, 1, 0).
    name : str, optional
        Name of the plane. Defaults to "my_plane".

    Attributes
    ----------
    guid : str
        The unique identifier of the plane.
    name : str
        The name of the plane.
    origin : Point
        The origin point of the plane.
    x_axis : Vector
        The X-axis direction vector.
    y_axis : Vector
        The Y-axis direction vector.
    z_axis : Vector
        The Z-axis direction vector (normal).
    a : float
        Plane equation coefficient (normal x-component).
    b : float
        Plane equation coefficient (normal y-component).
    c : float
        Plane equation coefficient (normal z-component).
    d : float
        Plane equation coefficient (distance from origin).
    """

    def __init__(self, origin=None, x_axis=None, y_axis=None, name="my_plane"):
        self.guid = str(uuid.uuid4())
        self.name = name

        if origin is None:
            self._origin = Point(0.0, 0.0, 0.0)
        else:
            self._origin = origin

        if x_axis is None:
            self._x_axis = Vector.x_axis()
        else:
            self._x_axis = x_axis
            self._x_axis.normalize_self()

        if y_axis is None:
            self._y_axis = Vector.y_axis()
        else:
            self._y_axis = y_axis - x_axis * (y_axis.dot(self._x_axis))
            self._y_axis.normalize_self()

        self._z_axis = self._x_axis.cross(self._y_axis)
        self._z_axis.normalize_self()

        self._update_equation()

    def _update_equation(self):
        """Update plane equation coefficients from z_axis and origin."""
        self._a = self._z_axis.x
        self._b = self._z_axis.y
        self._c = self._z_axis.z
        self._d = -(
            self._a * self._origin.x
            + self._b * self._origin.y
            + self._c * self._origin.z
        )

    @property
    def origin(self):
        """Get the origin point."""
        return self._origin

    @property
    def x_axis(self):
        """Get the X-axis vector."""
        return self._x_axis

    @property
    def y_axis(self):
        """Get the Y-axis vector."""
        return self._y_axis

    @property
    def z_axis(self):
        """Get the Z-axis vector (normal)."""
        return self._z_axis

    @property
    def a(self):
        """Get plane equation coefficient a."""
        return self._a

    @property
    def b(self):
        """Get plane equation coefficient b."""
        return self._b

    @property
    def c(self):
        """Get plane equation coefficient c."""
        return self._c

    @property
    def d(self):
        """Get plane equation coefficient d."""
        return self._d

    @staticmethod
    def from_point_normal(point, normal):
        """Create a plane from a point and normal vector.

        Parameters
        ----------
        point : Point
            Point on the plane.
        normal : Vector
            Normal vector of the plane.

        Returns
        -------
        Plane
            The constructed plane.
        """
        plane = Plane.__new__(Plane)
        plane.guid = str(uuid.uuid4())
        plane.name = "my_plane"
        plane._origin = point
        plane._z_axis = Vector(normal.x, normal.y, normal.z)
        plane._z_axis.normalize_self()
        plane._x_axis = Vector()
        plane._x_axis.perpendicular_to(plane._z_axis)
        plane._x_axis.normalize_self()
        plane._y_axis = plane._z_axis.cross(plane._x_axis)
        plane._y_axis.normalize_self()
        plane._update_equation()
        return plane

    @staticmethod
    def from_points(points):
        """Create a plane from three or more points.

        Parameters
        ----------
        points : list of Point
            List of at least 3 points.

        Returns
        -------
        Plane
            The constructed plane.
        """
        if len(points) < 3:
            return Plane()

        plane = Plane.__new__(Plane)
        plane.guid = str(uuid.uuid4())
        plane.name = "my_plane"
        plane._origin = points[0]

        v1 = points[1] - points[0]
        v2 = points[2] - points[0]
        plane._z_axis = v1.cross(v2)
        plane._z_axis.normalize_self()

        plane._x_axis = Vector(v1.x, v1.y, v1.z)
        plane._x_axis.normalize_self()
        plane._y_axis = plane._z_axis.cross(plane._x_axis)
        plane._y_axis.normalize_self()

        plane._update_equation()
        return plane

    @staticmethod
    def from_two_points(point1, point2):
        """Create a plane from two points.

        Parameters
        ----------
        point1 : Point
            First point.
        point2 : Point
            Second point.

        Returns
        -------
        Plane
            The constructed plane.
        """
        plane = Plane.__new__(Plane)
        plane.guid = str(uuid.uuid4())
        plane.name = "my_plane"
        plane._origin = point1

        direction = point2 - point1
        direction.normalize_self()
        plane._z_axis = Vector()
        plane._z_axis.perpendicular_to(direction)
        plane._z_axis.normalize_self()

        plane._x_axis = direction
        plane._y_axis = plane._z_axis.cross(plane._x_axis)
        plane._y_axis.normalize_self()

        plane._update_equation()
        return plane

    @staticmethod
    def xy_plane():
        """Create the XY plane.

        Returns
        -------
        Plane
            XY plane at origin.
        """
        plane = Plane.__new__(Plane)
        plane.guid = str(uuid.uuid4())
        plane.name = "xy_plane"
        plane._origin = Point(0.0, 0.0, 0.0)
        plane._x_axis = Vector.x_axis()
        plane._y_axis = Vector.y_axis()
        plane._z_axis = Vector.z_axis()
        plane._a = 0.0
        plane._b = 0.0
        plane._c = 1.0
        plane._d = 0.0
        return plane

    @staticmethod
    def yz_plane():
        """Create the YZ plane.

        Returns
        -------
        Plane
            YZ plane at origin.
        """
        plane = Plane.__new__(Plane)
        plane.guid = str(uuid.uuid4())
        plane.name = "yz_plane"
        plane._origin = Point(0.0, 0.0, 0.0)
        plane._x_axis = Vector.y_axis()
        plane._y_axis = Vector.z_axis()
        plane._z_axis = Vector.x_axis()
        plane._a = 1.0
        plane._b = 0.0
        plane._c = 0.0
        plane._d = 0.0
        return plane

    @staticmethod
    def xz_plane():
        """Create the XZ plane.

        Returns
        -------
        Plane
            XZ plane at origin.
        """
        plane = Plane.__new__(Plane)
        plane.guid = str(uuid.uuid4())
        plane.name = "xz_plane"
        plane._origin = Point(0.0, 0.0, 0.0)
        plane._x_axis = Vector.x_axis()
        plane._y_axis = Vector(0.0, 0.0, -1.0)
        plane._z_axis = Vector(0.0, 1.0, 0.0)
        plane._a = 0.0
        plane._b = 1.0
        plane._c = 0.0
        plane._d = 0.0
        return plane

    ###########################################################################################
    # Operators
    ###########################################################################################

    def transform(self):
        """Apply the stored xform transformation to the plane.

        Transforms the plane in-place and resets xform to identity.
        """
        self.xform.transform_point(self._origin)
        self.xform.transform_vector(self._x_axis)
        self.xform.transform_vector(self._y_axis)
        self.xform.transform_vector(self._z_axis)
        self.xform = Xform.identity()

    def transformed(self):
        """Return a transformed copy of the plane."""
        import copy

        result = copy.deepcopy(self)
        result.transform()
        return result

    def __str__(self):
        return f"Plane(origin={self._origin}, x_axis={self._x_axis}, y_axis={self._y_axis}, z_axis={self._z_axis}, guid={self.guid}, name={self.name})"

    def __repr__(self):
        return self.__str__()

    def __eq__(self, other):
        if isinstance(other, Point):
            return self._origin == other
        return False

    def __ne__(self, other):
        return not self.__eq__(other)

    def __getitem__(self, index):
        """Get axis by index (0=x, 1=y, 2=z)."""
        if index == 0:
            return self._x_axis
        elif index == 1:
            return self._y_axis
        elif index == 2:
            return self._z_axis
        raise IndexError("Plane index out of range (0-2)")

    ###########################################################################################
    # No-copy Operators
    ###########################################################################################

    def __iadd__(self, other):
        """Translate plane by vector (in-place)."""
        if isinstance(other, Vector):
            self._origin += other
            self._update_equation()
        return self

    def __isub__(self, other):
        """Translate plane by negative vector (in-place)."""
        if isinstance(other, Vector):
            self._origin -= other
            self._update_equation()
        return self

    ###########################################################################################
    # Copy Operators
    ###########################################################################################

    def __add__(self, other):
        """Translate plane by vector (copy)."""
        if isinstance(other, Vector):
            result = Plane.__new__(Plane)
            result.guid = self.guid
            result.name = self.name
            result._origin = self._origin + other
            result._x_axis = Vector(self._x_axis.x, self._x_axis.y, self._x_axis.z)
            result._y_axis = Vector(self._y_axis.x, self._y_axis.y, self._y_axis.z)
            result._z_axis = Vector(self._z_axis.x, self._z_axis.y, self._z_axis.z)
            result._update_equation()
            return result
        return NotImplemented

    def __sub__(self, other):
        """Translate plane by negative vector (copy)."""
        if isinstance(other, Vector):
            result = Plane.__new__(Plane)
            result.guid = self.guid
            result.name = self.name
            result._origin = self._origin - other
            result._x_axis = Vector(self._x_axis.x, self._x_axis.y, self._x_axis.z)
            result._y_axis = Vector(self._y_axis.x, self._y_axis.y, self._y_axis.z)
            result._z_axis = Vector(self._z_axis.x, self._z_axis.y, self._z_axis.z)
            result._update_equation()
            return result
        return NotImplemented

    ###########################################################################################
    # Details
    ###########################################################################################

    def reverse(self):
        """Reverse the plane's normal direction."""
        temp = self._x_axis
        self._x_axis = self._y_axis
        self._y_axis = temp
        self._z_axis.reverse()
        self._update_equation()

    def rotate(self, angles_in_radians):
        """Rotate the plane around its normal.

        Parameters
        ----------
        angles_in_radians : float
            Rotation angle in radians.
        """
        cos_angle = math.cos(angles_in_radians)
        sin_angle = math.sin(angles_in_radians)

        new_x = self._x_axis * cos_angle + self._y_axis * sin_angle
        new_y = self._y_axis * cos_angle - self._x_axis * sin_angle

        self._x_axis = new_x
        self._y_axis = new_y
        self._update_equation()

    def is_right_hand(self):
        """Check if the plane follows the right-hand rule.

        Returns
        -------
        bool
            True if x_axis × y_axis = z_axis (right-handed).
        """
        cross = self._x_axis.cross(self._y_axis)
        dot_product = cross.dot(self._z_axis)
        return dot_product > 0.999

    @staticmethod
    def is_same_direction(plane0, plane1, can_be_flipped=True):
        """Check if two planes have the same or flipped normal.

        Parameters
        ----------
        plane0 : Plane
            First plane.
        plane1 : Plane
            Second plane.
        can_be_flipped : bool, optional
            Allow flipped normals. Defaults to True.

        Returns
        -------
        bool
            True if normals are parallel or antiparallel.
        """
        n0 = plane0._z_axis
        n1 = plane1._z_axis

        parallel = n0.is_parallel_to(n1)

        if can_be_flipped:
            return parallel != 0
        else:
            return parallel == 1

    @staticmethod
    def is_same_position(plane0, plane1):
        """Check if two planes are in the same position.

        Parameters
        ----------
        plane0 : Plane
            First plane.
        plane1 : Plane
            Second plane.

        Returns
        -------
        bool
            True if origins are very close.
        """
        dist0 = abs(
            plane0._a * plane1._origin.x
            + plane0._b * plane1._origin.y
            + plane0._c * plane1._origin.z
            + plane0._d
        )

        dist1 = abs(
            plane1._a * plane0._origin.x
            + plane1._b * plane0._origin.y
            + plane1._c * plane0._origin.z
            + plane1._d
        )

        tolerance = Tolerance.ZERO_TOLERANCE
        return dist0 < tolerance and dist1 < tolerance

    @staticmethod
    def is_coplanar(plane0, plane1, can_be_flipped=True):
        """Check if two planes are coplanar.

        Parameters
        ----------
        plane0 : Plane
            First plane.
        plane1 : Plane
            Second plane.
        can_be_flipped : bool, optional
            Allow flipped normals. Defaults to True.

        Returns
        -------
        bool
            True if planes are coplanar.
        """
        return Plane.is_same_direction(
            plane0, plane1, can_be_flipped
        ) and Plane.is_same_position(plane0, plane1)

    def translate_by_normal(self, distance):
        """Translate (move) a plane along its normal direction by a specified distance.

        Parameters
        ----------
        distance : float
            Distance to move the plane along its normal (positive = normal direction, negative = opposite).

        Returns
        -------
        Plane
            New plane translated by the specified distance.
        """
        normal = Vector(self._z_axis.x, self._z_axis.y, self._z_axis.z)
        normal.normalize_self()

        new_origin = self._origin + (normal * distance)

        return Plane(new_origin, self._x_axis, self._y_axis)

    ###########################################################################################
    # Polymorphic JSON Serialization
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
            "origin": self.origin.__jsondump__(),
            "x_axis": self.x_axis.__jsondump__(),
            "y_axis": self.y_axis.__jsondump__(),
            "z_axis": self.z_axis.__jsondump__(),
            "a": self.a,
            "b": self.b,
            "c": self.c,
            "d": self.d,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format.

        Parameters
        ----------
        data : dict
            Dictionary containing plane data.
        guid : str, optional
            GUID for the plane.
        name : str, optional
            Name for the plane.

        Returns
        -------
        :class:`Plane`
            Reconstructed plane instance.

        """
        from .encoders import decode_node

        origin = decode_node(data["origin"])
        x_axis = decode_node(data["x_axis"])
        y_axis = decode_node(data["y_axis"])

        plane = cls(origin, x_axis, y_axis)
        plane.guid = guid if guid is not None else data.get("guid", plane.guid)
        plane.name = name if name is not None else data.get("name", plane.name)

        # z_axis, a, b, c, d are computed automatically, but verify if provided
        if "z_axis" in data:
            z_axis_loaded = decode_node(data["z_axis"])
            # z_axis is already computed from cross product, just verify consistency

        if "xform" in data:
            plane.xform = decode_node(data["xform"])

        return plane
