import uuid
import json


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
    
    Examples
    --------
    >>> vector = Vector(1.0, 2.0, 3.0)
    >>> assert vector.name == "my_vector"
    >>> assert vector.guid != ""
    >>> assert vector.x == 1.0
    >>> assert vector.y == 2.0
    >>> assert vector.z == 3.0
    """
    
    def __init__(self, x=0.0, y=0.0, z=0.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_vector"
        self.x = x
        self.y = y
        self.z = z

    def __str__(self):
        return f"Vector({self.x}, {self.y}, {self.z}, {self.guid}, {self.name})"

    def __repr__(self):
        return f"Vector({self.x}, {self.y}, {self.z}, {self.guid}, {self.name})"

    def __eq__(self, other):
        return (
            self.name == other.name and
            round(self.x, 6) == round(other.x, 6) and
            round(self.y, 6) == round(other.y, 6) and
            round(self.z, 6) == round(other.z, 6)
        )

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict:
        """Convert the Vector to a JSON-serializable dictionary.
        
        Returns
        -------
        dict
            Dictionary containing the Vector data in JSON format.
            
        Examples
        --------
        >>> vector = Vector(10.5, 20.7, 30.9)
        >>> vector.name = "force_vector_X"
        >>> data = vector.to_json_data()
        >>> assert data['type'] == 'Vector'
        >>> assert data['name'] == 'force_vector_X'
        >>> assert data['x'] == 10.5
        >>> assert data['y'] == 20.7
        >>> assert data['z'] == 30.9
        >>> assert 'guid' in data
        """
        return {
            "type": "Vector",
            "guid": self.guid,
            "name": self.name,
            "x": self.x,
            "y": self.y,
            "z": self.z,
        }

    @classmethod
    def from_json_data(cls, data: dict) -> 'Vector':
        """Create a Vector from JSON data dictionary.
        
        Parameters
        ----------
        data : dict
            Dictionary containing vector data from JSON.
            
        Returns
        -------
        :class:`Vector`
            Vector instance created from the JSON data.
            
        Examples
        --------
        >>> original_vector = Vector(45.1, 67.8, 89.2)
        >>> data = original_vector.to_json_data()
        >>> restored_vector = Vector.from_json_data(data)
        >>> assert restored_vector.x == 45.1
        >>> assert restored_vector.y == 67.8
        >>> assert restored_vector.z == 89.2
        >>> assert restored_vector.guid == original_vector.guid
        """
        vector = cls(data["x"], data["y"], data["z"])
        vector.guid = data["guid"]
        vector.name = data["name"]
        return vector

    def to_json(self, filepath: str) -> None:
        """Serialize the Vector to a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the output JSON file.

        Examples
        --------
        >>> vector = Vector(100.25, 200.50, 300.75)
        >>> vector.to_json("my_vector.json")
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> 'Vector':
        """Deserialize a Vector from a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Vector`
            Vector instance loaded from the file.

        Examples
        --------
        >>> import tempfile, os
        >>> with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        ...     _ = f.write('{"type": "Vector", "guid": "123", "name": "test", "x": 1.0, "y": 2.0, "z": 3.0}')
        ...     temp_file = f.name
        >>> vector = Vector.from_json(temp_file)
        >>> os.unlink(temp_file)  # cleanup
        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)
