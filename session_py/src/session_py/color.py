
import json
import uuid


class Color:
    """A color with RGBA values for cross-language compatibility.
    
    Parameters
    ----------
    r : int, optional
        Red component (0-255). Defaults to 255.
    g : int, optional
        Green component (0-255). Defaults to 255.
    b : int, optional
        Blue component (0-255). Defaults to 255.
    a : int, optional
        Alpha component (0-255). Defaults to 255.
    name : str, optional
        Name of the color. Defaults to "white".
    
    Attributes
    ----------
    name : str
        The name of the color.
    guid : str
        The unique identifier of the color.
    r : int
        The red component of the color (0-255).
    g : int
        The green component of the color (0-255).
    b : int
        The blue component of the color (0-255).
    a : int
        The alpha component of the color (0-255).
    
    Examples
    --------
    >>> red = Color(255, 0, 0, 255, "red")
    >>> assert red.name == "red"
    >>> assert red.guid != ""
    >>> assert red.r == 255
    >>> assert red.g == 0
    >>> assert red.b == 0
    >>> assert red.a == 255
    """
    def __init__(self, r: int, g: int, b: int, a: int, name: str = "my_color"):
        self.guid = str(uuid.uuid4())
        self.name = name
        self.r = int(r)
        self.g = int(g)
        self.b = int(b)
        self.a = int(a)

    def __str__(self):
        """String representation."""
        return f"Color(r={self.r}, g={self.g}, b={self.b}, a={self.a})"

    def __repr__(self):
        return f"Color(r={self.r}, g={self.g}, b={self.b}, a={self.a})"

    def __eq__(self, other):
        if not isinstance(other, Color):
            return False
        return (
            self.name == other.name and
            self.r == other.r and
            self.g == other.g and
            self.b == other.b and
            self.a == other.a
        )

    ###########################################################################################
    # JSON
    ###########################################################################################

    def to_json_data(self, minimal=False) -> dict:
        """Convert to JSON-serializable dictionary.
        
        Parameters
        ----------
        minimal : bool, optional
            If True, return minimal representation without None values.
            
        Returns
        -------
        dict
            Dictionary representation of the color.
            
        Examples
        --------
        >>> color = Color(128, 64, 192, 255, "purple")
        >>> data = color.to_json_data()
        >>> assert data['type'] == 'Color'
        >>> assert data['name'] == 'purple'
        >>> assert data['r'] == 128
        >>> assert data['g'] == 64
        >>> assert data['b'] == 192
        >>> assert data['a'] == 255
        >>> assert 'guid' in data
        """
        data = {
            "type": "Color",
            "guid": self.guid,
            "name": self.name,
            "r": self.r,
            "g": self.g,
            "b": self.b,
            "a": self.a
        }
        if minimal:
            return {k: v for k, v in data.items() if v is not None}
        return data

    @classmethod
    def from_json_data(cls, data):
        """Create color from JSON data.
        
        Parameters
        ----------
        data : dict
            Dictionary containing color data from JSON.
            
        Returns
        -------
        :class:`Color`
            Color instance created from the JSON data.
            
        Examples
        --------
        >>> original_color = Color(200, 150, 100, 255, "bronze")
        >>> data = original_color.to_json_data()
        >>> restored_color = Color.from_json_data(data)
        >>> assert restored_color.r == 200
        >>> assert restored_color.g == 150
        >>> assert restored_color.b == 100
        >>> assert restored_color.a == 255
        >>> assert restored_color.name == "bronze"
        >>> assert restored_color.guid == original_color.guid
        """
        if not all(key in data for key in ["r", "g", "b", "a"]):
            return None
        color = cls(data["r"], data["g"], data["b"], data["a"], data.get("name"))
        if "guid" in data:
            color.guid = data["guid"]
        return color

    def to_json(self, filepath, minimal=False):
        """Save the Color to a JSON file.
        
        Parameters
        ----------
        filepath : str
            Path where to save the JSON file.
        minimal : bool, optional
            If True, save minimal representation.
            
        Examples
        --------
        >>> color = Color(255, 128, 64, 255, "sunset_orange")
        >>> color.to_json("my_color.json")
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(minimal), f, indent=2)

    @classmethod
    def from_json(cls, filepath):
        """Load Color from a JSON file.
        
        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Color`
            Color instance loaded from the file.

        Examples
        --------
        >>> color = Color(255, 128, 64, 255, "orange")
        >>> data = color.to_json_data()
        >>> color2 = Color.from_json_data(data)
        >>> color2.name
        'orange'
        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)

    ###########################################################################################
    # Details
    ###########################################################################################

    @classmethod
    def white(cls) -> 'Color':
        """Create a white color."""
        color = cls(255, 255, 255, 255)
        color.name = "white"
        return color

    @classmethod
    def black(cls) -> 'Color':
        """Create a black color."""
        color = cls(0, 0, 0, 255)
        color.name = "black"
        return color

    def to_float_array(self) -> list[float]:
        """Convert to normalized float array [0-1] (matches Rust implementation)."""
        return [self.r / 255.0, self.g / 255.0, self.b / 255.0, self.a / 255.0]

    @classmethod
    def from_float(cls, r, g, b, a):
        """Create color from normalized float values [0-1]."""
        return cls(r * 255.0, g * 255.0, b * 255.0, a * 255.0)
