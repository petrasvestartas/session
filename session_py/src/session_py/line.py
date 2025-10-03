import uuid
import json
from .color import Color
from .point import Point
from .vector import Vector


class Line:
    """A 3D line segment with visual properties.

    Parameters
    ----------
    x0 : float, optional
        X coordinate of start point. Defaults to 0.0.
    y0 : float, optional
        Y coordinate of start point. Defaults to 0.0.
    z0 : float, optional
        Z coordinate of start point. Defaults to 0.0.
    x1 : float, optional
        X coordinate of end point. Defaults to 0.0.
    y1 : float, optional
        Y coordinate of end point. Defaults to 0.0.
    z1 : float, optional
        Z coordinate of end point. Defaults to 1.0.

    Attributes
    ----------
    guid : str
        Unique identifier of the line.
    name : str
        Name of the line.
    linecolor : Color
        Color of the line.
    width : float
        Width of the line for display.
    """

    def __init__(self, x0=0.0, y0=0.0, z0=0.0, x1=0.0, y1=0.0, z1=1.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_line"
        self._x0 = x0
        self._y0 = y0
        self._z0 = z0
        self._x1 = x1
        self._y1 = y1
        self._z1 = z1
        self.width = 1.0
        self.linecolor = Color.white()

    @property
    def x0(self):
        """Get the X coordinate of start point."""
        return self._x0

    @x0.setter
    def x0(self, value):
        """Set the X coordinate of start point."""
        self._x0 = value

    @property
    def y0(self):
        """Get the Y coordinate of start point."""
        return self._y0

    @y0.setter
    def y0(self, value):
        """Set the Y coordinate of start point."""
        self._y0 = value

    @property
    def z0(self):
        """Get the Z coordinate of start point."""
        return self._z0

    @z0.setter
    def z0(self, value):
        """Set the Z coordinate of start point."""
        self._z0 = value

    @property
    def x1(self):
        """Get the X coordinate of end point."""
        return self._x1

    @x1.setter
    def x1(self, value):
        """Set the X coordinate of end point."""
        self._x1 = value

    @property
    def y1(self):
        """Get the Y coordinate of end point."""
        return self._y1

    @y1.setter
    def y1(self, value):
        """Set the Y coordinate of end point."""
        self._y1 = value

    @property
    def z1(self):
        """Get the Z coordinate of end point."""
        return self._z1

    @z1.setter
    def z1(self, value):
        """Set the Z coordinate of end point."""
        self._z1 = value

    @classmethod
    def from_points(cls, p1, p2):
        """Create a line from two points.

        Parameters
        ----------
        p1 : Point
            Start point.
        p2 : Point
            End point.

        Returns
        -------
        Line
            New line from p1 to p2.
        """
        return cls(p1.x, p1.y, p1.z, p2.x, p2.y, p2.z)

    @classmethod
    def with_name(cls, name, x0, y0, z0, x1, y1, z1):
        """Create a line with a specific name.

        Parameters
        ----------
        name : str
            Name for the line.
        x0, y0, z0 : float
            Start point coordinates.
        x1, y1, z1 : float
            End point coordinates.

        Returns
        -------
        Line
            New named line.
        """
        line = cls(x0, y0, z0, x1, y1, z1)
        line.name = name
        return line

    def length(self):
        """Calculate the length of the line.

        Returns
        -------
        float
            Length of the line.
        """
        dx = self._x1 - self._x0
        dy = self._y1 - self._y0
        dz = self._z1 - self._z0
        return (dx * dx + dy * dy + dz * dz) ** 0.5

    def squared_length(self):
        """Calculate the squared length of the line.

        Returns
        -------
        float
            Squared length of the line.
        """
        dx = self._x1 - self._x0
        dy = self._y1 - self._y0
        dz = self._z1 - self._z0
        return dx * dx + dy * dy + dz * dz

    def to_vector(self):
        """Convert line to vector from start to end.

        Returns
        -------
        Vector
            Direction vector of the line.
        """
        return Vector(self._x1 - self._x0, self._y1 - self._y0, self._z1 - self._z0)

    def point_at(self, t):
        """Get point at parameter t along the line.

        Parameters
        ----------
        t : float
            Parameter value (0.0 = start, 1.0 = end).

        Returns
        -------
        Point
            Point at parameter t.
        """
        s = 1.0 - t
        return Point(
            s * self._x0 + t * self._x1,
            s * self._y0 + t * self._y1,
            s * self._z0 + t * self._z1,
        )

    def start(self):
        """Get start point.

        Returns
        -------
        Point
            Start point of the line.
        """
        return Point(self._x0, self._y0, self._z0)

    def end(self):
        """Get end point.

        Returns
        -------
        Point
            End point of the line.
        """
        return Point(self._x1, self._y1, self._z1)

    def __getitem__(self, index):
        """Get coordinate by index (0-5)."""
        coords = [self._x0, self._y0, self._z0, self._x1, self._y1, self._z1]
        return coords[index]

    def __setitem__(self, index, value):
        """Set coordinate by index (0-5)."""
        if index == 0:
            self._x0 = value
        elif index == 1:
            self._y0 = value
        elif index == 2:
            self._z0 = value
        elif index == 3:
            self._x1 = value
        elif index == 4:
            self._y1 = value
        elif index == 5:
            self._z1 = value
        else:
            raise IndexError("Index out of bounds")

    def __iadd__(self, other):
        """Add vector to line in place."""
        if isinstance(other, Vector):
            self._x0 += other.x
            self._y0 += other.y
            self._z0 += other.z
            self._x1 += other.x
            self._y1 += other.y
            self._z1 += other.z
        return self

    def __isub__(self, other):
        """Subtract vector from line in place."""
        if isinstance(other, Vector):
            self._x0 -= other.x
            self._y0 -= other.y
            self._z0 -= other.z
            self._x1 -= other.x
            self._y1 -= other.y
            self._z1 -= other.z
        return self

    def __imul__(self, factor):
        """Multiply line coordinates by scalar in place."""
        self._x0 *= factor
        self._y0 *= factor
        self._z0 *= factor
        self._x1 *= factor
        self._y1 *= factor
        self._z1 *= factor
        return self

    def __itruediv__(self, factor):
        """Divide line coordinates by scalar in place."""
        self._x0 /= factor
        self._y0 /= factor
        self._z0 /= factor
        self._x1 /= factor
        self._y1 /= factor
        self._z1 /= factor
        return self

    def __add__(self, other):
        """Add vector to line."""
        if isinstance(other, Vector):
            return Line(
                self._x0 + other.x,
                self._y0 + other.y,
                self._z0 + other.z,
                self._x1 + other.x,
                self._y1 + other.y,
                self._z1 + other.z,
            )
        return NotImplemented

    def __sub__(self, other):
        """Subtract vector from line."""
        if isinstance(other, Vector):
            return Line(
                self._x0 - other.x,
                self._y0 - other.y,
                self._z0 - other.z,
                self._x1 - other.x,
                self._y1 - other.y,
                self._z1 - other.z,
            )
        return NotImplemented

    def __mul__(self, factor):
        """Multiply line by scalar."""
        return Line(
            self._x0 * factor,
            self._y0 * factor,
            self._z0 * factor,
            self._x1 * factor,
            self._y1 * factor,
            self._z1 * factor,
        )

    def __truediv__(self, factor):
        """Divide line by scalar."""
        return Line(
            self._x0 / factor,
            self._y0 / factor,
            self._z0 / factor,
            self._x1 / factor,
            self._y1 / factor,
            self._z1 / factor,
        )

    def __str__(self):
        """String representation."""
        return f"Line({self._x0}, {self._y0}, {self._z0}, {self._x1}, {self._y1}, {self._z1})"

    def __repr__(self):
        """Detailed representation."""
        return self.__str__()

    def to_json_data(self):
        """Convert to JSON-serializable dictionary.

        Returns
        -------
        dict
            JSON-serializable dictionary.
        """
        return {
            "type": "Line",
            "guid": self.guid,
            "name": self.name,
            "x0": self._x0,
            "y0": self._y0,
            "z0": self._z0,
            "x1": self._x1,
            "y1": self._y1,
            "z1": self._z1,
            "width": self.width,
            "linecolor": self.linecolor.to_json_data(),
        }

    @classmethod
    def from_json_data(cls, data):
        """Create line from JSON data.

        Parameters
        ----------
        data : dict
            JSON data dictionary.

        Returns
        -------
        Line
            Reconstructed line.
        """
        line = cls(
            data["x0"], data["y0"], data["z0"], data["x1"], data["y1"], data["z1"]
        )
        line.guid = data["guid"]
        line.name = data["name"]
        line.linecolor = Color.from_json_data(data["linecolor"])
        line.width = data["width"]
        return line

    def to_json(self, filepath):
        """Save line to JSON file.

        Parameters
        ----------
        filepath : str
            Path to save JSON file.
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath):
        """Load line from JSON file.

        Parameters
        ----------
        filepath : str
            Path to JSON file.

        Returns
        -------
        Line
            Loaded line.
        """
        with open(filepath, "r") as f:
            data = json.load(f)
        return cls.from_json_data(data)
