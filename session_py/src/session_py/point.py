import uuid
import json
from .color import Color


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
    
    Examples
    --------
    >>> point = Point(1.0, 2.0, 3.0)
    >>> assert point.name == "my_point"
    >>> assert point.guid != ""
    >>> assert point.x == 1.0
    >>> assert point.y == 2.0
    >>> assert point.z == 3.0
    >>> assert point.width == 1.0
    >>> assert point.pointcolor == Color.white()
    """
    
    def __init__(self, x=0.0, y=0.0, z=0.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_point"
        self.x = x
        self.y = y
        self.z = z
        self.width = 1.0
        self.pointcolor = Color.white()

    def __str__(self):
        return f"Point({self.x}, {self.y}, {self.z})"

    def __repr__(self):
        return f"Point({self.x}, {self.y}, {self.z}, {self.guid}, {self.name}, {self.pointcolor}, {self.width})"

    def __eq__(self, other):
        return (
            self.name == other.name and
            round(self.x, 6) == round(other.x, 6) and
            round(self.y, 6) == round(other.y, 6) and
            round(self.z, 6) == round(other.z, 6) and
            round(self.width, 6) == round(other.width, 6) and
            self.pointcolor == other.pointcolor
        )

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict:
        """Convert the Point to a JSON-serializable dictionary.
        
        Returns
        -------
        dict
            Dictionary containing the Point data in JSON format.
            
        Examples
        --------
        >>> point = Point(15.5, 25.7, 35.9)
        >>> point.name = "survey_point_A"
        >>> point.width = 2.5
        >>> point.pointcolor = Color(255, 128, 64, 255)
        >>> data = point.to_json_data()
        >>> assert data['type'] == 'Point'
        >>> assert data['name'] == 'survey_point_A'
        >>> assert data['x'] == 15.5
        >>> assert data['y'] == 25.7
        >>> assert data['z'] == 35.9
        >>> assert data['width'] == 2.5
        >>> assert data['pointcolor']['r'] == 255
        >>> assert data['pointcolor']['g'] == 128
        """
        return {
            "type": "Point",
            "guid": self.guid,
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
        
        Parameters
        ----------
        data : dict
            Dictionary containing point data from JSON.
            
        Returns
        -------
        :class:`Point`
            Point instance created from the JSON data.
            
        Examples
        --------
        >>> original_point = Point(42.1, 84.2, 126.3)
        >>> original_point.name = "control_point_B"
        >>> original_point.width = 3.0
        >>> original_point.pointcolor = Color(200, 100, 50, 255)
        >>> data = original_point.to_json_data()
        >>> restored_point = Point.from_json_data(data)
        >>> assert restored_point.x == 42.1
        >>> assert restored_point.y == 84.2
        >>> assert restored_point.z == 126.3
        >>> assert restored_point.name == "control_point_B"
        >>> assert restored_point.width == 3.0
        >>> assert restored_point.pointcolor.r == 200
        """
        point = cls(data["x"], data["y"], data["z"])
        point.guid = data["guid"]
        point.name = data["name"]
        point.width = data["width"]
        point.pointcolor = Color.from_json_data(data["pointcolor"])
        return point

    def to_json(self, filepath: str) -> None:
        """Serialize the Point to a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the output JSON file.

        Examples
        --------
        >>> point = Point(123.45, 678.90, 999.11)
        >>> point.width = 4.5
        >>> point.pointcolor = Color(0, 255, 128, 255)
        >>> point.to_json("my_point.json")
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> 'Point':
        """Deserialize a Point from a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Point`
            Point instance loaded from the file.

        Examples
        --------
        >>> import tempfile, os
        >>> with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        ...     _ = f.write('{"type": "Point", "guid": "123", "name": "test", "x": 1.0, "y": 2.0, "z": 3.0, "width": 1.0, "pointcolor": {"type": "Color", "r": 255, "g": 255, "b": 255, "a": 255}}')
        ...     temp_file = f.name
        >>> point = Point.from_json(temp_file)
        >>> os.unlink(temp_file)  # cleanup
        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)

    ###########################################################################################
    # Details
    ###########################################################################################