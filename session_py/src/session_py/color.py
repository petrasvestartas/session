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

    """

    def __init__(self, r: int, g: int, b: int, a: int, name: str = "my_color"):
        self.guid = str(uuid.uuid4())
        self.name = name
        self.r = int(r)
        self.g = int(g)
        self.b = int(b)
        self.a = int(a)

    ###########################################################################################
    # Operators
    ###########################################################################################

    def __str__(self) -> str:
        """String representation."""
        return f"Color({self.r}, {self.g}, {self.b}, {self.a})"

    def __repr__(self) -> str:
        return (
            f"Color({self.guid}, {self.name}, {self.r}, {self.g}, {self.b}, {self.a})"
        )

    def __eq__(self, other) -> bool:
        if not isinstance(other, Color):
            return False
        return (
            self.name == other.name
            and self.r == other.r
            and self.g == other.g
            and self.b == other.b
            and self.a == other.a
        )

    ###########################################################################################
    # Details
    ###########################################################################################

    def to_float_array(self) -> list[float]:
        """Convert to normalized float array [0-1] (matches Rust implementation)."""
        return [self.r / 255.0, self.g / 255.0, self.b / 255.0, self.a / 255.0]

    @classmethod
    def from_float(cls, r, g, b, a) -> "Color":
        """Create color from normalized float values [0-1]."""
        return cls(r * 255.0, g * 255.0, b * 255.0, a * 255.0)

    ###########################################################################################
    # Presets
    ###########################################################################################

    @classmethod
    def white(cls) -> "Color":
        """Create a white color."""
        color = cls(255, 255, 255, 255)
        color.name = "white"
        return color

    @classmethod
    def black(cls) -> "Color":
        """Create a black color."""
        color = cls(0, 0, 0, 255)
        color.name = "black"
        return color

    @classmethod
    def grey(cls) -> "Color":
        """Create a grey color."""
        color = cls(128, 128, 128, 255)
        color.name = "grey"
        return color

    @classmethod
    def red(cls) -> "Color":
        """Create a red color."""
        color = cls(255, 0, 0, 255)
        color.name = "red"
        return color

    @classmethod
    def orange(cls) -> "Color":
        """Create an orange color."""
        color = cls(255, 128, 0, 255)
        color.name = "orange"
        return color

    @classmethod
    def yellow(cls) -> "Color":
        """Create a yellow color."""
        color = cls(255, 255, 0, 255)
        color.name = "yellow"
        return color

    @classmethod
    def lime(cls) -> "Color":
        """Create a lime color."""
        color = cls(128, 255, 0, 255)
        color.name = "lime"
        return color

    @classmethod
    def green(cls) -> "Color":
        """Create a green color."""
        color = cls(0, 255, 0, 255)
        color.name = "green"
        return color

    @classmethod
    def mint(cls) -> "Color":
        """Create a mint color."""
        color = cls(0, 255, 128, 255)
        color.name = "mint"
        return color

    @classmethod
    def cyan(cls) -> "Color":
        """Create a cyan color."""
        color = cls(0, 255, 255, 255)
        color.name = "cyan"
        return color

    @classmethod
    def azure(cls) -> "Color":
        """Create an azure color."""
        color = cls(0, 128, 255, 255)
        color.name = "azure"
        return color

    @classmethod
    def blue(cls) -> "Color":
        """Create a blue color."""
        color = cls(0, 0, 255, 255)
        color.name = "blue"
        return color

    @classmethod
    def violet(cls) -> "Color":
        """Create a violet color."""
        color = cls(128, 0, 255, 255)
        color.name = "violet"
        return color

    @classmethod
    def magenta(cls) -> "Color":
        """Create a magenta color."""
        color = cls(255, 0, 255, 255)
        color.name = "magenta"
        return color

    @classmethod
    def pink(cls) -> "Color":
        """Create a pink color."""
        color = cls(255, 0, 128, 255)
        color.name = "pink"
        return color

    @classmethod
    def maroon(cls) -> "Color":
        """Create a maroon color."""
        color = cls(128, 0, 0, 255)
        color.name = "maroon"
        return color

    @classmethod
    def brown(cls) -> "Color":
        """Create a brown color."""
        color = cls(128, 64, 0, 255)
        color.name = "brown"
        return color

    @classmethod
    def olive(cls) -> "Color":
        """Create an olive color."""
        color = cls(128, 128, 0, 255)
        color.name = "olive"
        return color

    @classmethod
    def teal(cls) -> "Color":
        """Create a teal color."""
        color = cls(0, 128, 128, 255)
        color.name = "teal"
        return color

    @classmethod
    def navy(cls) -> "Color":
        """Create a navy color."""
        color = cls(0, 0, 128, 255)
        color.name = "navy"
        return color

    @classmethod
    def purple(cls) -> "Color":
        """Create a purple color."""
        color = cls(128, 0, 128, 255)
        color.name = "purple"
        return color

    @classmethod
    def silver(cls) -> "Color":
        """Create a silver color."""
        color = cls(192, 192, 192, 255)
        color.name = "silver"
        return color

    ###########################################################################################
    # Polymorphic JSON Serialization (COMPAS-style)
    ###########################################################################################

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "r": self.r,
            "g": self.g,
            "b": self.b,
            "a": self.a,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format."""
        color = cls(data["r"], data["g"], data["b"], data.get("a", 255))
        color.guid = guid
        color.name = name
        return color
