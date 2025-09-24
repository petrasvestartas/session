from .point import Point
import uuid
import json


class Objects:
    """A collection of objects.

    Parameters
    ----------
    points : list[:class:`Point`], optional
        The list of points in the collection. Defaults to an empty list.

    Attributes
    ----------
    name : str
        The name of the collection.
    guid : UUID
        The unique identifier of the collection.
    points : list[Point]
        The list of points in the collection.

    """

    def __init__(self, points: list[Point] = None):
        self.guid = str(uuid.uuid4())
        self.name = "my_objects"
        self.points: list[Point] = points or []

    def __str__(self):
        return f"Objects(points={len(self.points)})"

    def __repr__(self):
        return f"Objects({self.guid}, {self.name}, points={len(self.points)})"

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self):
        """Convert the Objects to a JSON-serializable dictionary.

        Returns
        -------
        dict
            Dictionary representation of the objects collection.

        """
        return {
            "type": "Objects",
            "name": self.name,
            "guid": str(self.guid),
            "points": [point.to_json_data() for point in self.points],
        }

    @classmethod
    def from_json_data(cls, data):
        """Create an Objects from JSON data dictionary.

        Parameters
        ----------
        data : dict
            Dictionary containing objects data.

        Returns
        -------
        :class:`Objects`
            Objects instance created from the data.

        """
        points = [
            Point.from_json_data(point_data) for point_data in data.get("points", [])
        ]
        objects = cls(points)
        objects.name = data["name"]
        objects.guid = str(data["guid"]) if "guid" in data else str(uuid.uuid4())
        return objects

    def to_json(self, filepath):
        """Save the Objects to a JSON file.

        Parameters
        ----------
        filepath : str
            Path where to save the JSON file.

        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=2)

    @classmethod
    def from_json(cls, filepath):
        """Load Objects from a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Objects`
            Objects instance loaded from the file.

        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)

    ###########################################################################################
    # Details
    ###########################################################################################
