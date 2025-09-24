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

    """

    def __init__(self, x=0.0, y=0.0, z=0.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_vector"
        self.x = x
        self.y = y
        self.z = z

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

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self) -> dict:
        """Convert the Vector to a JSON-serializable dictionary.

        Returns
        -------
        dict
            Dictionary containing the Vector data in JSON format.

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
    def from_json_data(cls, data: dict) -> "Vector":
        """Create a Vector from JSON data dictionary.

        Parameters
        ----------
        data : dict
            Dictionary containing vector data from JSON.

        Returns
        -------
        :class:`Vector`
            Vector instance created from the JSON data.

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

        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=4)

    @classmethod
    def from_json(cls, filepath: str) -> "Vector":
        """Deserialize a Vector from a JSON file.

        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Vector`
            Vector instance loaded from the file.

        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)

    ###########################################################################################
    # Details
    ###########################################################################################
