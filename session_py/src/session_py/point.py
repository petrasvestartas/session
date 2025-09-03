import uuid
import json
from .color import Color


class Point:
    """A point defined by XYZ coordinates with display properties.

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
    
    def __init__(self, x=0.0, y=0.0, z=0.0):
        self.guid = uuid.uuid4()
        self.name = "my_point"
        self.x = x
        self.y = y
        self.z = z
        self.width = 1.0
        self.pointcolor = Color.white()

    def __str__(self):
        return f"Point({self.x}, {self.y}, {self.z}, {self.guid}, {self.name}, {self.pointcolor}, {self.width})"

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self):
        """Convert to JSON-serializable dictionary."""
        return {
            "type": "Point",
            "guid": str(self.guid),
            "name": self.name,
            "x": self.x,
            "y": self.y,
            "z": self.z,
            "width": self.width,
            "pointcolor": self.pointcolor.to_json_data(),
        }

    @classmethod
    def from_json_data(cls, data):
        """Create point from JSON data."""
        point = cls(data["x"], data["y"], data["z"])
        point.guid = uuid.UUID(data["guid"])
        point.name = data["name"]
        point.width = data["width"]
        point.pointcolor = Color.from_json_data(data["pointcolor"])
        return point

    def to_json(self, filepath):
        """Serialize to JSON string."""
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath):
        """Deserialize from JSON string."""
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)