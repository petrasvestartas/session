import uuid
import math
from .tolerance import Tolerance, TO_DEGREES, TO_RADIANS


class Vector:
    """A 3D vector with visual properties.

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
    guid : str
        The unique identifier of the vector.
    name : str
        The name of the vector.
    x : float
        The X coordinate of the vector.
    y : float
        The Y coordinate of the vector.
    z : float
        The Z coordinate of the vector.

    """

    def __init__(self, x=0.0, y=0.0, z=0.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_vector"
        self._x = x
        self._y = y
        self._z = z
        self._length = 0.0
        self._has_length = False

    @property
    def x(self):
        """Get the X coordinate."""
        return self._x

    @x.setter
    def x(self, value):
        """Set the X coordinate and invalidate length cache."""
        self._x = value
        self._has_length = False

    @property
    def y(self):
        """Get the Y coordinate."""
        return self._y

    @y.setter
    def y(self, value):
        """Set the Y coordinate and invalidate length cache."""
        self._y = value
        self._has_length = False

    @property
    def z(self):
        """Get the Z coordinate."""
        return self._z

    @z.setter
    def z(self, value):
        """Set the Z coordinate and invalidate length cache."""
        self._z = value
        self._has_length = False

    def __str__(self):
        return f"Vector({self.x}, {self.y}, {self.z})"

    def __repr__(self):
        return f"Vector({self.guid}, {self.name}, {self.x}, {self.y}, {self.z})"

    def __eq__(self, other):
        return (
            self.name == other.name
            and round(self.x, 6) == round(other.x, 6)
            and round(self.y, 6) == round(other.y, 6)
            and round(self.z, 6) == round(other.z, 6)
        )

    def __ne__(self, other):
        return not self == other

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
        self._has_length = False

    def __imul__(self, other):
        self._x *= other
        self._y *= other
        self._z *= other
        self._has_length = False
        return self

    def __itruediv__(self, other):
        self._x /= other
        self._y /= other
        self._z /= other
        self._has_length = False
        return self

    def __iadd__(self, other):
        self._x += other._x
        self._y += other._y
        self._z += other._z
        self._has_length = False
        return self

    def __isub__(self, other):
        self._x -= other._x
        self._y -= other._y
        self._z -= other._z
        self._has_length = False
        return self

    ###########################################################################################
    # Copy Operators
    ###########################################################################################

    def __mul__(self, other):
        return Vector(self._x * other, self._y * other, self._z * other)

    def __truediv__(self, other):
        return Vector(self._x / other, self._y / other, self._z / other)

    def __add__(self, other):
        return Vector(self._x + other._x, self._y + other._y, self._z + other._z)

    def __sub__(self, other):
        return Vector(self._x - other._x, self._y - other._y, self._z - other._z)

    ###########################################################################################
    # Static Methods
    ###########################################################################################

    @staticmethod
    def x_axis():
        """Get unit vector along the x-axis.

        Returns
        -------
        :class:`Vector`
            Unit vector (1, 0, 0).

        """
        return Vector(1.0, 0.0, 0.0)

    @staticmethod
    def y_axis():
        """Get unit vector along the y-axis.

        Returns
        -------
        :class:`Vector`
            Unit vector (0, 1, 0).

        """
        return Vector(0.0, 1.0, 0.0)

    @staticmethod
    def z_axis():
        """Get unit vector along the z-axis.

        Returns
        -------
        :class:`Vector`
            Unit vector (0, 0, 1).

        """
        return Vector(0.0, 0.0, 1.0)

    @staticmethod
    def from_start_and_end(start, end):
        """Vector from start to end (end - start).

        Parameters
        ----------
        start : :class:`Vector`
            Start vector.
        end : :class:`Vector`
            End vector.

        Returns
        -------
        :class:`Vector`
            The vector from start to end.

        """
        return Vector(end.x - start.x, end.y - start.y, end.z - start.z)

    ###########################################################################################
    # Details
    ###########################################################################################

    def reverse(self):
        """Reverse the vector (negate all components).

        Returns
        -------
        :class:`Vector`
            Self.

        """
        self._x = -self._x
        self._y = -self._y
        self._z = -self._z
        self._has_length = False
        return self

    def compute_length(self):
        """Compute the length of the vector using optimized algorithm.

        Returns
        -------
        float
            The length of the vector.
        """
        length = 0.0

        x = abs(self._x)
        y = abs(self._y)
        z = abs(self._z)

        # Handle two zero case:
        x_zero = x < Tolerance.ZERO_TOLERANCE
        y_zero = y < Tolerance.ZERO_TOLERANCE
        z_zero = z < Tolerance.ZERO_TOLERANCE

        if x_zero and y_zero and z_zero:
            length = 0.0
            return length
        elif x_zero and y_zero:
            length = z
            return length
        elif x_zero and z_zero:
            length = y
            return length
        elif y_zero and z_zero:
            length = x
            return length

        # Handle one or none zero case:
        # Sort so that x is the largest component
        if y >= x and y >= z:
            length = x
            x = y
            y = length
        elif z >= x and z >= y:
            length = x
            x = z
            z = length

        # For small denormalized doubles (positive but smaller
        # than DOUBLE_MIN), some compilers/FPUs set 1.0/x to +INF.
        # Without the DOUBLE_MIN test we end up with
        # microscopic vectors that have infinite length!
        if x > 2.22507385850720200e-308:
            y /= x
            z /= x
            length = x * math.sqrt(1.0 + y * y + z * z)
        elif x > 0.0 and math.isfinite(x):
            length = x
        else:
            length = 0.0

        return length

    def magnitude(self):
        """Get the cached magnitude of the vector, computing it if necessary.

        Returns
        -------
        float
            The magnitude (length) of the vector.
        """
        if not self._has_length:
            self._length = self.compute_length()
            self._has_length = True

        return self._length

    def length_squared(self):
        """Get the squared length of the vector (avoids sqrt for performance).

        Returns
        -------
        float
            The squared length of the vector.
        """
        return self._x * self._x + self._y * self._y + self._z * self._z

    def normalize_self(self):
        """Normalize the vector in place (make it unit length).

        Returns
        -------
        bool
            True if successful, False if vector has zero length.
        """
        d = self.magnitude()
        if d > 0.0:
            self._x /= d
            self._y /= d
            self._z /= d
            self._length = 1.0
            self._has_length = True
            return True
        return False

    def normalize(self):
        """Return a normalized copy of the vector.

        Returns
        -------
        Vector
            A new vector that is the unit vector of this vector.
        """
        normalized_vector = Vector(self._x, self._y, self._z)
        normalized_vector.normalize_self()
        return normalized_vector

    def dot(self, other):
        """Calculate dot product with another vector.

        Parameters
        ----------
        other : :class:`Vector`
            Other vector.

        Returns
        -------
        float
            Dot product value.

        """
        return self._x * other._x + self._y * other._y + self._z * other._z

    def cross(self, other):
        """Calculate cross product with another vector.

        Parameters
        ----------
        other : :class:`Vector`
            Other vector.

        Returns
        -------
        :class:`Vector`
            Cross product vector (orthogonal to inputs).

        """
        x = self._y * other._z - self._z * other._y
        y = self._z * other._x - self._x * other._z
        z = self._x * other._y - self._y * other._x
        return Vector(x, y, z)

    def is_parallel_to(self, v):
        """Check if this vector is parallel/antiparallel to another.

        Parameters
        ----------
        v : :class:`Vector`
            Other vector.

        Returns
        -------
        int
            1 if parallel, -1 if antiparallel, 0 otherwise.

        """
        ll = self.magnitude() * v.magnitude()

        if ll > 0.0:
            cos_angle = self.dot(v) / ll
            angle_in_radians = Tolerance.ANGLE_TOLERANCE_DEGREES * TO_RADIANS
            cos_tol = math.cos(angle_in_radians)

            if cos_angle >= cos_tol:
                return 1  # Parallel
            elif cos_angle <= -cos_tol:
                return -1  # Antiparallel
            else:
                return 0  # Not parallel
        else:
            return 0  # Not parallel

    def angle(self, other, sign_by_cross_product=False, degrees=True, tolerance=1e-12):
        """Angle between this vector and another.

        Parameters
        ----------
        other : :class:`Vector`
            The other vector.
        sign_by_cross_product : bool, optional
            If True, sign the angle using the z-component of the cross product.
        degrees : bool, optional
            If True (default), return angle in degrees; otherwise radians.
        tolerance : float, optional
            Denominator tolerance to treat near-zero lengths as zero.

        Returns
        -------
        float
            The angle value (degrees if `degrees` else radians).

        """
        dot_product = self.dot(other)
        len0 = self.magnitude()
        len1 = other.magnitude()

        denominator = len0 * len1
        if denominator < tolerance:
            return 0.0

        cos_angle = dot_product / denominator
        cos_angle = max(-1.0, min(1.0, cos_angle))  # Clamp to [-1, 1]

        angle = math.acos(cos_angle)

        if sign_by_cross_product:
            # Raw cross product z-component for sign check
            cross_z = self._x * other._y - self._y * other._x
            if cross_z < 0:
                angle = -angle

        if degrees:
            return math.degrees(angle)
        return angle

    def projection(self, projection_vector, tolerance=1e-12):
        """Project this vector onto another vector.

        Parameters
        ----------
        projection_vector : :class:`Vector`
            Vector to project onto.
        tolerance : float, optional
            Treat `projection_vector` length below this as zero.

        Returns
        -------
        tuple
            (projection_vector, projected_length, perpendicular_vector, perpendicular_length),
            where projection_vector is :class:`Vector`, projected_length is float,
            perpendicular_vector is :class:`Vector`, and perpendicular_length is float.

        """
        projection_vector_length = projection_vector.magnitude()

        if projection_vector_length < tolerance:
            return Vector(0, 0, 0), 0.0, Vector(0, 0, 0), 0.0

        projection_vector_unit = Vector(
            projection_vector.x / projection_vector_length,
            projection_vector.y / projection_vector_length,
            projection_vector.z / projection_vector_length,
        )

        projected_vector_length = self.dot(projection_vector_unit)
        out_projection_vector = projection_vector_unit * projected_vector_length

        out_perpendicular_vector = self - out_projection_vector
        out_perpendicular_length = out_perpendicular_vector.magnitude()

        return (
            out_projection_vector,
            projected_vector_length,
            out_perpendicular_vector,
            out_perpendicular_length,
        )

    def get_leveled_vector(self, vertical_height):
        """Get a copy scaled by a vertical height along the Z-axis.

        Parameters
        ----------
        vertical_height : float
            Target vertical height.

        Returns
        -------
        :class:`Vector`
            Scaled copy matching the C++ implementation.

        """
        copy = Vector(self._x, self._y, self._z)

        if copy.normalize_self():
            reference_vector = Vector(0, 0, 1)
            angle = copy.angle(
                reference_vector, sign_by_cross_product=True, degrees=True
            )
            inclined_offset = vertical_height / math.cos(angle)
            copy *= inclined_offset

        return copy

    @staticmethod
    def cosine_law(
        triangle_edge_length_a,
        triangle_edge_length_b,
        angle_in_between_edges,
        degrees=True,
    ):
        """Calculate third side of triangle using the cosine law.

        Parameters
        ----------
        triangle_edge_length_a : float
            Length of side a.
        triangle_edge_length_b : float
            Length of side b.
        angle_in_between_edges : float
            Angle between a and b.
        degrees : bool, optional
            If True, the angle is provided in degrees.

        Returns
        -------
        float
            Length of the third side.

        """
        to_radians = TO_RADIANS if degrees else 1.0
        return math.sqrt(
            triangle_edge_length_a**2
            + triangle_edge_length_b**2
            - 2
            * triangle_edge_length_a
            * triangle_edge_length_b
            * math.cos(angle_in_between_edges * to_radians)
        )

    @staticmethod
    def sine_law_angle(
        triangle_edge_length_a,
        angle_in_front_of_a,
        triangle_edge_length_b,
        degrees=True,
    ):
        """Calculate angle using the sine law.

        Parameters
        ----------
        triangle_edge_length_a : float
            Length of side a.
        angle_in_front_of_a : float
            Angle opposite to side a.
        triangle_edge_length_b : float
            Length of side b.
        degrees : bool, optional
            If True, return angle in degrees.

        Returns
        -------
        float
            Angle opposite to side b (degrees if `degrees`).

        """
        to_radians = TO_RADIANS if degrees else 1.0
        to_degrees = TO_DEGREES if degrees else 1.0
        return (
            math.asin(
                (triangle_edge_length_b * math.sin(angle_in_front_of_a * to_radians))
                / triangle_edge_length_a
            )
            * to_degrees
        )

    @staticmethod
    def sine_law_length(
        triangle_edge_length_a, angle_in_front_of_a, angle_in_front_of_b, degrees=True
    ):
        """Calculate side length using the sine law.

        Parameters
        ----------
        triangle_edge_length_a : float
            Length of side a.
        angle_in_front_of_a : float
            Angle opposite to side a.
        angle_in_front_of_b : float
            Angle opposite to side b.
        degrees : bool, optional
            If True, angles are provided in degrees.

        Returns
        -------
        float
            Length of side b.

        """
        to_radians = TO_RADIANS if degrees else 1.0
        return (
            triangle_edge_length_a * math.sin(angle_in_front_of_b * to_radians)
        ) / math.sin(angle_in_front_of_a * to_radians)

    @staticmethod
    def angle_between_vector_xy_components(vector, degrees=True):
        """Angle between the vector's XY components.

        Parameters
        ----------
        vector : :class:`Vector`
            Input vector.
        degrees : bool, optional
            If True, return degrees; otherwise radians.

        Returns
        -------
        float
            Angle in the XY plane.

        """
        to_degrees = TO_DEGREES if degrees else 1.0
        return math.atan(vector.y / vector.x) * to_degrees

    @staticmethod
    def sum_of_vectors(vectors):
        """Sum a list of vectors (component-wise).

        Parameters
        ----------
        vectors : list[:class:`Vector`]
            Vectors to sum.

        Returns
        -------
        :class:`Vector`
            The component-wise sum.

        """
        x = y = z = 0.0
        for vector in vectors:
            x += vector._x
            y += vector._y
            z += vector._z
        return Vector(x, y, z)

    def coordinate_direction_3angles(self, degrees=True):
        """Compute coordinate direction angles (alpha, beta, gamma).

        Parameters
        ----------
        degrees : bool, optional
            Return angles in degrees if True, radians if False.

        Returns
        -------
        tuple
            (alpha, beta, gamma)

        """
        r = math.sqrt(self._x**2 + self._y**2 + self._z**2)

        if r == 0:
            return (0, 0, 0)

        x_proportion = self._x / r
        y_proportion = self._y / r
        z_proportion = self._z / r

        alpha = math.acos(x_proportion)
        beta = math.acos(y_proportion)
        gamma = math.acos(z_proportion)

        if degrees:
            alpha *= TO_DEGREES
            beta *= TO_DEGREES
            gamma *= TO_DEGREES

        return (alpha, beta, gamma)

    def coordinate_direction_2angles(self, degrees=True):
        """Compute coordinate direction angles (phi, theta).

        Parameters
        ----------
        degrees : bool, optional
            Return angles in degrees if True, radians if False.

        Returns
        -------
        tuple
            (phi, theta)

        """
        r = math.sqrt(self._x**2 + self._y**2 + self._z**2)

        if r == 0:
            return (0, 0)

        phi = math.acos(self._z / r)
        theta = math.atan2(self._y, self._x)

        if degrees:
            phi *= TO_DEGREES
            theta *= TO_DEGREES

        return (phi, theta)

    def perpendicular_to(self, v):
        """Set this vector to be perpendicular to `v`.

        Parameters
        ----------
        v : :class:`Vector`
            Reference vector.

        Returns
        -------
        bool
            True on success, False otherwise.

        """
        k = 2

        if abs(v.y) > abs(v.x):
            if abs(v.z) > abs(v.y):
                # |v.z| > |v.y| > |v.x|
                i, j, k = 2, 1, 0
                a, b = v.z, -v.y
            elif abs(v.z) >= abs(v.x):
                # |v.y| >= |v.z| >= |v.x|
                i, j, k = 1, 2, 0
                a, b = v.y, -v.z
            else:
                # |v.y| > |v.x| > |v.z|
                i, j, k = 1, 0, 2
                a, b = v.y, -v.x
        elif abs(v.z) > abs(v.x):
            # |v.z| > |v.x| >= |v.y|
            i, j, k = 2, 0, 1
            a, b = v.z, -v.x
        elif abs(v.z) > abs(v.y):
            # |v.x| >= |v.z| > |v.y|
            i, j, k = 0, 2, 1
            a, b = v.x, -v.z
        else:
            # |v.x| >= |v.y| >= |v.z|
            i, j, k = 0, 1, 2
            a, b = v.x, -v.y

        coords = [0, 0, 0]
        coords[i] = b
        coords[j] = a
        coords[k] = 0.0

        self._x, self._y, self._z = coords
        self._has_length = False

    ###########################################################################################
    # Polymorphic JSON Serialization (COMPAS-style)
    ###########################################################################################

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "x": self.x,
            "y": self.y,
            "z": self.z,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format."""
        vec = cls(data["x"], data["y"], data["z"])
        vec.guid = guid
        vec.name = name
        return vec
