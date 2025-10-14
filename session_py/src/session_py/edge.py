import uuid


class Edge:
    """A graph edge connecting two vertices with an attribute string."""

    def __init__(self, v0, v1, attribute=""):
        """Initialize a new Edge.

        Parameters
        ----------
        v0 : str
            Name of the first vertex.
        v1 : str
            Name of the second vertex.
        attribute : str, optional
            Edge attribute data as string.
        """
        self.guid = str(uuid.uuid4())
        self.name = "my_edge"
        self.v0 = str(v0)
        self.v1 = str(v1)
        self.attribute = str(attribute)
        self.index = None  # Will be set internally by Graph

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field."""
        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "v0": self.v0,
            "v1": self.v1,
            "attribute": self.attribute,
            "index": self.index,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format."""
        edge = cls(data["v0"], data["v1"], data.get("attribute", ""))
        edge.index = data.get("index")
        edge.guid = guid if guid is not None else data.get("guid", edge.guid)
        edge.name = name if name is not None else data.get("name", edge.name)
        return edge

    @property
    def vertices(self):
        """Get the edge vertices as a tuple."""
        return (self.v0, self.v1)

    def connects(self, vertex_id):
        """Check if this edge connects to a given vertex."""
        return str(vertex_id) in self.vertices

    def other_vertex(self, vertex_id):
        """Get the other vertex ID connected by this edge."""
        vertex_id = str(vertex_id)
        if vertex_id == self.v0:
            return self.v1
        elif vertex_id == self.v1:
            return self.v0
        else:
            raise ValueError(f"Vertex {vertex_id} is not connected by this edge")
