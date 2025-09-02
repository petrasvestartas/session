import uuid
import json
from color import Color


class Point:
    """A point is defined by XYZ coordinates.

    Parameters
    ----------
    x : float
        The X coordinate of the point.
    y : float
        The Y coordinate of the point.
    z : float
        The Z coordinate of the point.
    guid : uuid, optional
        The unique identifier of the point.
    name : str, optional
        The name of the point.
    pointcolor : Color, optional
        The color of the point.
    width : float, optional
        The width of the point.
    """
    
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z
        self.guid = uuid.uuid4()
        self.name = "Point"
        self.pointcolor = Color.white()
        self.width = 1.0

    def distance_to(self, other):
        """Calculate distance to another point."""
        dx = self.x - other.x
        dy = self.y - other.y
        dz = self.z - other.z
        return (dx * dx + dy * dy + dz * dz) ** 0.5

    def to_json_data(self):
        """Convert to JSON-serializable dictionary."""
        return {
            "dtype": "Point",
            "x": self.x,
            "y": self.y,
            "z": self.z,
            "guid": str(self.guid),
            "name": self.name,
            "pointcolor": self.pointcolor.to_float_array(),
            "width": self.width
        }

    @classmethod
    def from_json_data(cls, data):
        """Create point from JSON data."""
        point = cls(data["x"], data["y"], data["z"])
        point.name = data["name"]
        point.guid = uuid.UUID(data["guid"])
        point.width = data["width"]
        r, g, b, a = data["pointcolor"]
        point.pointcolor = Color.from_float(r, g, b, a)
        return point

    def to_json(self):
        """Serialize to JSON string."""
        return json.dumps(self.to_json_data())

    @classmethod
    def from_json(cls, json_str):
        """Deserialize from JSON string."""
        return cls.from_json_data(json.loads(json_str))

    def __str__(self):
        return f"Point({self.x}, {self.y}, {self.z}, {self.guid}, {self.name}, {self.pointcolor}, {self.width})"