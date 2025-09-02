
import json


class Color:
    """A color is defined by RGBA coordinates from 0 to 255 and alpha from 0 to 100.

    Parameters
    ----------
    r : float
        The red component of the color.
    g : float
        The green component of the color.
    b : float
        The blue component of the color.
    a : float
        The alpha component of the color.
    """
    def __init__(self, r, g, b, a):
        self.r = r
        self.g = g
        self.b = b
        self.a = a

    @classmethod
    def white(cls):
        """Create a white color."""
        return cls(255.0, 255.0, 255.0, 100.0)

    @classmethod
    def black(cls):
        """Create a black color."""
        return cls(0.0, 0.0, 0.0, 100.0)

    def to_float_array(self):
        """Convert to normalized float array [0-1] (matches Rust implementation)."""
        return [self.r / 255.0, self.g / 255.0, self.b / 255.0, self.a / 100.0]

    @classmethod
    def from_float(cls, r, g, b, a):
        """Create color from normalized float values [0-1]."""
        return cls(r * 255.0, g * 255.0, b * 255.0, a * 100.0)

    def to_json_data(self, minimal=False):
        """Convert color to JSON-serializable dictionary."""
        return {
            "dtype": "Color",
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
        return cls(data["r"], data["g"], data["b"], data["a"])

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
