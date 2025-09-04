import uuid
import json
from .color import Color


class Point:
    """A 3D point with visual properties and cross-language JSON serialization support.
    
    This class represents a point in 3D space along with visual properties
    such as color, width, and name. It provides JSON serialization/deserialization
    for interoperability between Rust, Python, and C++ implementations.
    
    Attributes:
        x (float): The X coordinate of the point.
        y (float): The Y coordinate of the point.
        z (float): The Z coordinate of the point.
        guid (UUID): The unique identifier of the point.
        name (str): The name of the point.
        pointcolor (Color): The color of the point.
        width (float): The width of the point for display.
    
    Example:
        >>> point = Point(1.0, 2.0, 3.0)
        >>> point.name = "my_point"
        >>> print(point)
        Point(1.0, 2.0, 3.0, ...)
    """
    
    def __init__(self, x=0.0, y=0.0, z=0.0):
        """Initialize a new Point with specified coordinates.
        
        Args:
            x (float, optional): X coordinate. Defaults to 0.0.
            y (float, optional): Y coordinate. Defaults to 0.0.
            z (float, optional): Z coordinate. Defaults to 0.0.
        """
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

    def to_json_data(self) -> dict:
        """Convert the Point to a JSON-serializable dictionary.
        
        Returns:
            Dictionary containing the Point data in JSON format.
            
        Example:
            >>> point = Point(1.0, 2.0, 3.0)
            >>> data = point.to_json_data()
            >>> print(data['type'])
            Point
        """
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
    def from_json_data(cls, data: dict) -> 'Point':
        """Create a Point from JSON data dictionary.
        
        Args:
            data: Dictionary containing point data from JSON.
            
        Returns:
            Point instance created from the JSON data.
            
        Example:
            >>> json_data = {'x': 1.0, 'y': 2.0, 'z': 3.0, 'name': 'test'}
            >>> point = Point.from_json_data(json_data)
            >>> print(point.x)
            1.0
        """
        point = cls(data["x"], data["y"], data["z"])
        point.guid = uuid.UUID(data["guid"])
        point.name = data["name"]
        point.width = data["width"]
        point.pointcolor = Color.from_json_data(data["pointcolor"])
        return point

    def to_json(self, filepath: str) -> None:
        """Serialize the Point to a JSON file.

        Args:
            filepath: Path to the output JSON file

        Example:
            >>> point.to_json("my_point.json")
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> 'Point':
        """Deserialize a Point from a JSON file.

        Args:
            filepath: Path to the JSON file to load

        Returns:
            Point instance loaded from the file.

        Example:
            >>> point = Point.from_json("my_point.json")
        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)
