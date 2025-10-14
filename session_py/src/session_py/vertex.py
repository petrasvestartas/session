import uuid


class Vertex:
    """A graph vertex with a unique identifier and attribute string.

    Parameters
    ----------
    name : str, optional
        Name identifier for the vertex. Defaults to "my_vertex".
    attribute : str, optional
        Vertex attribute data as string. Defaults to "".

    Attributes
    ----------
    guid : str
        The unique identifier of the vertex.
    name : str
        The name of the vertex.
    attribute : str
        Vertex attribute data as string.
    index : int or None
        Integer index for the vertex. Set internally by Graph.

    """

    def __init__(self, name="my_vertex", attribute=""):
        """Initialize a new Vertex.

        Parameters
        ----------
        name : str, optional
            Name identifier for the vertex. Defaults to "my_vertex".
        attribute : str, optional
            Vertex attribute data as string. Defaults to "".

        """
        self.guid = str(uuid.uuid4())
        self.name = str(name)
        self.attribute = str(attribute)
        self.index = None  # Will be set internally by Graph

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "attribute": self.attribute,
            "index": self.index,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format."""
        vertex = cls(data["name"], data.get("attribute", ""))
        vertex.index = data.get("index")
        vertex.guid = guid if guid is not None else data.get("guid", vertex.guid)
        return vertex
