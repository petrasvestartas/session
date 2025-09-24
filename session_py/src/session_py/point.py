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
            self.name == other.name
            and round(self.x, 6) == round(other.x, 6)
            and round(self.y, 6) == round(other.y, 6)
            and round(self.z, 6) == round(other.z, 6)
            and round(self.width, 6) == round(other.width, 6)
            and self.pointcolor == other.pointcolor
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
    def from_json_data(cls, data: dict) -> "Point":
        """Create a Point from JSON data dictionary.

        Parameters
        ----------
        data : dict
            Dictionary containing point data from JSON.

        Returns
        -------
        :class:`Point`
            Point instance created from the JSON data.

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

        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> "Point":
        """Deserialize a Point from a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Point`
            Point instance loaded from the file.

        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)

    ###########################################################################################
    # Details
    ###########################################################################################
