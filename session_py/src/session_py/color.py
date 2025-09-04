
import json
import uuid


class Color:
    """A color with RGBA values for cross-language compatibility.
    
    This class represents a color with red, green, blue, and alpha components.
    It provides JSON serialization/deserialization for interoperability
    between Rust, Python, and C++ implementations.
    
    Attributes:
        r (int): The red component of the color (0-255).
        g (int): The green component of the color (0-255).
        b (int): The blue component of the color (0-255).
        a (int): The alpha component of the color (0-255).
        name (str): The name of the color.
        guid (str): Unique identifier.
    
    Example:
        >>> red = Color(255, 0, 0, 255, "red")
        >>> white = Color.white()
        >>> print(red.r)
        255
    """
    def __init__(self, r: int, g: int, b: int, a: int, name: str = "Color"):
        self.guid = str(uuid.uuid4())
        self.name = name
        self.r = int(r)
        self.g = int(g)
        self.b = int(b)
        self.a = int(a)

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

    def to_json_data(self) -> dict:
        """Convert to JSON-serializable dictionary."""
        return {
            "type": "Color",
            "guid": self.guid,
            "name": self.name,
            "r": self.r,
            "g": self.g,
            "b": self.b,
            "a": self.a
        }

    @classmethod
    def from_json_data(cls, data):
        """Create color from JSON data."""
        if not all(key in data for key in ["r", "g", "b", "a"]):
            return None
        return cls(data["r"], data["g"], data["b"], data["a"], data.get("name"))

    def to_json(self, minimal=False):
        """Serialize to JSON string."""
        return json.dumps(self.to_json_data(minimal))

    @classmethod
    def from_json(cls, json_str):
        """Deserialize from JSON string."""
        try:
            data = json.loads(json_str)
            return cls.from_json_data(data)
        except (json.JSONDecodeError, TypeError):
            return None

    def __str__(self):
        """String representation."""
        return f"Color(r={self.r}, g={self.g}, b={self.b}, a={self.a})"

    def __repr__(self):
        """Detailed string representation."""
        return self.__str__()
