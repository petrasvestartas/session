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
    
    Examples
    --------
    >>> objects = Objects()
    >>> assert objects.name == "my_objects"
    >>> assert objects.guid is not None
    >>> assert len(objects.points) == 0
    """

    def __init__(self, points: list[Point] = None):
        self.name = "my_objects"
        self.guid = str(uuid.uuid4())
        self.points: list[Point] = points or []

    def __str__(self):
        return f"Objects(points={len(self.points)})"

    def __repr__(self):
        return f"Objects({self.guid}, {self.name}, points={len(self.points)})"

    ###########################################################################################
    # JSON Serialization
    ###########################################################################################

    def to_json_data(self):
        """Convert the Objects to a JSON-serializable dictionary.
        
        Returns
        -------
        dict
            Dictionary representation of the objects collection.
            
        Examples
        --------
        >>> from .point import Point
        >>> objects = Objects()
        >>> point1 = Point(1.0, 2.0, 3.0)
        >>> point2 = Point(4.0, 5.0, 6.0)
        >>> point3 = Point(7.0, 8.0, 9.0)
        >>> objects.points = [point1, point2, point3]
        >>> data = objects.to_json_data()
        >>> assert data["name"] == "my_objects"
        >>> assert "guid" in data
        >>> assert len(data["points"]) == 3
        >>> assert data["points"][0]["x"] == 1.0
        >>> assert data["points"][1]["y"] == 5.0
        >>> assert data["points"][2]["z"] == 9.0
        """
        return {
            "type": "Objects",
            "name": self.name,
            "guid": str(self.guid),
            "points": [point.to_json_data() for point in self.points]
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
            
        Examples
        --------
        >>> from .point import Point
        >>> objects = Objects()
        >>> point1 = Point(10.0, 20.0, 30.0)
        >>> point2 = Point(40.0, 50.0, 60.0)
        >>> objects.points = [point1, point2]
        >>> data = objects.to_json_data()
        >>> objects2 = Objects.from_json_data(data)
        >>> assert objects2.name == "my_objects"
        >>> assert len(objects2.points) == 2
        >>> assert objects2.points[0].x == 10.0
        >>> assert objects2.points[1].z == 60.0
        """
        points = [Point.from_json_data(point_data) for point_data in data.get("points", [])]
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
            
        Examples
        --------
        >>> from .point import Point
        >>> objects = Objects()
        >>> point1 = Point(100.0, 200.0, 300.0)
        >>> point2 = Point(400.0, 500.0, 600.0)
        >>> point3 = Point(700.0, 800.0, 900.0)
        >>> objects.points = [point1, point2, point3]
        >>> objects.to_json("my_objects.json")
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

        Examples
        --------
        >>> objects = Objects()
        >>> data = objects.to_json_data()
        >>> objects2 = Objects.from_json_data(data)
        >>> objects2.name
        'my_objects'
        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)
        
    ###########################################################################################
    # Details
    ###########################################################################################