import uuid
from .vertex import Vertex
from .edge import Edge


class Graph:
    """A graph data structure with string-only vertices and attributes.

    Parameters
    ----------
    name : str, optional
        Name of the graph.
    default_node_attributes : dict, optional
        Default attributes for new vertices.
    default_edge_attributes : dict, optional
        Default attributes for new edges.

    """

    def __init__(self, name="my_graph"):
        """Initialize a new Graph."""
        self.name = name
        self.guid = str(uuid.uuid4())
        self.vertices = {}  # node_name -> Vertex object
        self.edges = {}  # node_name -> {neighbor_name -> Edge object}
        self.vertex_count = 0  # Track next available vertex index
        self.edge_count = 0  # Track next available edge index

    def __str__(self):
        """String representation."""
        return f"Graph({self.name}, {len(self.vertices)} vertices, {len(self.edges)} edges)"

    def __repr__(self):
        return f"Graph({self.name}, {len(self.vertices)} vertices, {len(self.edges)} edges)"

    ###########################################################################################
    # JSON (polymorphic)
    ###########################################################################################

    def __jsondump__(self):
        """Serialize to polymorphic JSON format with type field."""
        # Only store each undirected edge once (u < v)
        seen = set()
        edges_list = []
        for u, neighbors in self.edges.items():
            for v, edge in neighbors.items():
                key = (u, v) if u < v else (v, u)
                if key in seen:
                    continue
                seen.add(key)
                edges_list.append(edge.__jsondump__())

        return {
            "type": f"{self.__class__.__name__}",
            "guid": self.guid,
            "name": self.name,
            "vertices": [vertex.__jsondump__() for vertex in self.vertices.values()],
            "edges": edges_list,
            "vertex_count": self.vertex_count,
            "edge_count": self.edge_count,
        }

    @classmethod
    def __jsonload__(cls, data, guid=None, name=None):
        """Deserialize from polymorphic JSON format."""
        graph = cls(name=data.get("name", "my_graph"))
        graph.guid = guid if guid is not None else data.get("guid", graph.guid)
        graph.vertex_count = data.get("vertex_count", 0)
        graph.edge_count = data.get("edge_count", 0)

        # Restore vertices
        for vertex_data in data.get("vertices", []):
            # When decoding, nested vertices may already be reconstructed objects
            if isinstance(vertex_data, Vertex):
                vtx = vertex_data
            elif isinstance(vertex_data, dict):
                vtx = Vertex.__jsonload__(
                    vertex_data,
                    vertex_data.get("guid"),
                    vertex_data.get("name"),
                )
            else:
                continue
            graph.vertices[str(vtx.name)] = vtx

        # Restore edges
        for edge_data in data.get("edges", []):
            if isinstance(edge_data, Edge):
                e = edge_data
            elif isinstance(edge_data, dict):
                e = Edge.__jsonload__(
                    edge_data,
                    edge_data.get("guid"),
                    edge_data.get("name"),
                )
            else:
                continue
            u, v = str(e.v0), str(e.v1)
            if u not in graph.edges:
                graph.edges[u] = {}
            if v not in graph.edges:
                graph.edges[v] = {}
            graph.edges[u][v] = e
            graph.edges[v][u] = e

        return graph

    ###########################################################################################
    # Details: Essential Graph Methods
    ###########################################################################################

    def has_node(self, key):
        """Check if a node exists in the graph.

        Parameters
        ----------
        key : str
            The node to check for.

        Returns
        -------
        bool
            True if the node exists.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1")
        'node1'
        >>> graph.has_node("node1")
        True
        >>> graph.has_node("node2")
        False
        """
        return key in self.vertices

    def has_edge(self, edge):
        """Check if an edge exists in the graph.

        Parameters
        ----------
        edge : tuple or str
            Either a tuple (u, v) or two separate arguments u, v.

        Returns
        -------
        bool
            True if the edge exists, False otherwise.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("A", "B", "edge_attr")
        ('A', 'B')
        >>> graph.has_edge(("A", "B"))
        True
        >>> graph.has_edge(("C", "D"))
        False
        """
        if isinstance(edge, tuple):
            u, v = edge
        else:
            raise ValueError("Edge must be a tuple (u, v)")

        return u in self.edges and v in self.edges[u]

    def add_node(self, key, attribute=""):
        """Add a node to the graph.

        Parameters
        ----------
        key : str
            The node identifier.
        attribute : str, optional
            Node attribute data.

        Returns
        -------
        str
            The node key that was added.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1", "attribute_data")
        'node1'
        >>> graph.has_node("node1")
        True
        """
        if not isinstance(key, str):
            raise TypeError(f"Node keys must be strings, got {type(key)}")

        if self.has_node(key):
            return self.vertices[key]
        else:
            vertex = Vertex(key, attribute)
            vertex.index = self.vertex_count  # Set index internally
            self.vertices[key] = vertex
            self.vertex_count += 1
            return vertex.name

    def add_edge(self, u, v, attribute=""):
        """Add an edge between u and v.

        Parameters
        ----------
        u : str
            First node (must be string).
        v : str
            Second node (must be string).
        attribute : str, optional
            Single string attribute for the edge.

        Returns
        -------
        tuple
            The edge tuple (u, v).

        Raises
        ------
        TypeError
            If u or v are not strings.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("node1", "node2", "edge_data")
        ('node1', 'node2')
        >>> graph.has_edge(("node1", "node2"))
        True
        """
        if not isinstance(u, str) or not isinstance(v, str):
            raise TypeError(f"Node keys must be strings, got {type(u)} and {type(v)}")

        # Add vertices if they don't exist
        if not self.has_node(u):
            self.add_node(u)
        if not self.has_node(v):
            self.add_node(v)

        # Add edge (store in both directions for undirected graph)
        edge = Edge(u, v, attribute)
        edge.index = self.edge_count  # Set index internally
        if u not in self.edges:
            self.edges[u] = {}
        if v not in self.edges:
            self.edges[v] = {}
        self.edges[u][v] = edge
        self.edges[v][u] = edge
        self.edge_count += 1

        return (u, v)

    def remove_node(self, key):
        """Remove a node and all its edges from the graph.

        Parameters
        ----------
        key : str
            The node to remove.

        Raises
        ------
        KeyError
            If the node is not in the graph.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1")
        'node1'
        >>> graph.remove_node("node1")
        >>> graph.has_node("node1")
        False
        """
        if not self.has_node(key):
            raise KeyError(f"Node {key} not in graph")

        # Remove all edges connected to this node
        if key in self.edges:
            for neighbor in list(self.edges[key].keys()):
                if neighbor in self.edges:
                    self.edges[neighbor].pop(key, None)
            del self.edges[key]

        # Remove the node itself
        del self.vertices[key]

        # Reassign indices to maintain contiguous sequence
        self._reassign_indices()

    def remove_edge(self, edge):
        """Remove an edge from the graph.

        Parameters
        ----------
        edge : tuple
            A tuple (u, v) representing the edge to remove.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("A", "B", "edge_attr")
        ('A', 'B')
        >>> graph.remove_edge(("A", "B"))
        >>> graph.has_edge(("A", "B"))
        False
        """
        u, v = edge
        if self.has_edge((u, v)):
            if u in self.edges and v in self.edges[u]:
                del self.edges[u][v]
            if v in self.edges and u in self.edges[v]:
                del self.edges[v][u]

            # Reassign edge indices to maintain contiguous sequence
            self._reassign_edge_indices()

    def _reassign_indices(self):
        """Reassign vertex indices to maintain contiguous sequence 0, 1, 2, ..."""
        vertices = list(self.vertices.values())
        # Sort by current index to maintain relative order
        vertices.sort(key=lambda v: v.index if v.index is not None else float("inf"))

        for i, vertex in enumerate(vertices):
            vertex.index = i

        self.vertex_count = len(vertices)

    def _reassign_edge_indices(self):
        """Reassign edge indices to maintain contiguous sequence 0, 1, 2, ..."""
        edges = []
        seen = set()

        # Collect all unique edges
        for u, neighbors in self.edges.items():
            for v, edge in neighbors.items():
                edge_tuple = (u, v) if u < v else (v, u)
                if edge_tuple not in seen:
                    seen.add(edge_tuple)
                    edges.append(edge)

        # Sort by current index to maintain relative order
        edges.sort(key=lambda e: e.index if e.index is not None else float("inf"))

        # Reassign indices
        for i, edge in enumerate(edges):
            edge.index = i

        self.edge_count = len(edges)

    def get_vertices(self):
        """Return a list of all vertices in the graph.

        Returns
        -------
        list of :class:`Vertex`
            A list of all vertex objects in the graph.
        """
        return list(self.vertices.values())

    def get_edges(self):
        """Iterate over all edges in the graph.

        Yields
        ------
        tuple
            Edge identifier (u, v)

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("node1", "node2", "edge_data")
        ('node1', 'node2')
        >>> edges = list(graph.edges())
        >>> assert ("node1", "node2") in edges
        >>> assert len(edges) == 1
        """
        seen = set()
        for u, neighbors in self.edges.items():
            for v, edge in neighbors.items():
                edge_tuple = (u, v) if u < v else (v, u)
                if edge_tuple not in seen:
                    seen.add(edge_tuple)
                    yield edge_tuple

    def neighbors(self, node):
        """Get all neighbors of a node.

        Parameters
        ----------
        node : str
            The node to get neighbors for.

        Returns
        -------
        iterator
            Iterator over neighbor vertices.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("A", "B", "edge1")
        ('A', 'B')
        >>> graph.add_edge("A", "C", "edge2")
        ('A', 'C')
        >>> sorted(list(graph.neighbors("A")))
        ['B', 'C']
        """
        return iter(self.edges.get(node, {}).keys())

    def number_of_vertices(self):
        """Get the number of vertices in the graph.

        Returns
        -------
        int
            Number of vertices.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1")
        'node1'
        >>> graph.number_of_vertices()
        1
        """
        return len(self.vertices)

    def number_of_edges(self):
        """Get the number of edges in the graph.

        Returns
        -------
        int
            Number of edges.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("node1", "node2")
        ('node1', 'node2')
        >>> graph.number_of_edges()
        1
        """
        return sum(len(neighbors) for neighbors in self.edges.values()) // 2

    def clear(self):
        """Remove all vertices and edges from the graph.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1")
        'node1'
        >>> graph.clear()
        >>> graph.number_of_vertices()
        0
        """
        self.vertices.clear()
        self.edges.clear()
        self.vertex_count = 0
        self.edge_count = 0

    def node_attribute(self, node, value=None):
        """Get or set node attribute.

        Parameters
        ----------
        node : str
            The node identifier.
        value : str, optional
            If provided, set the attribute to this value.

        Returns
        -------
        str
            The attribute value as string.

        Raises
        ------
        KeyError
            If the node does not exist.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1", "initial_data")
        'node1'
        >>> assert graph.node_attribute("node1") == "initial_data"
        >>> graph.node_attribute("node1", "new_data")
        >>> assert graph.node_attribute("node1") == "new_data"
        """
        if not self.has_node(node):
            raise KeyError(f"Node {node} not in graph")

        node_obj = self.vertices[node]
        if value is not None:
            node_obj.attribute = str(value)
            return value
        else:
            return node_obj.attribute

    def edge_attribute(self, u, v, value=None):
        """Get or set edge attribute.

        Parameters
        ----------
        u : hashable
            First vertex of the edge.
        v : hashable
            Second vertex of the edge.
        value : str, optional
            If provided, set the attribute to this value.

        Returns
        -------
        str
            The attribute value as string.

        Raises
        ------
        KeyError
            If the edge does not exist.

        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("node1", "node2", "edge_data")
        ('node1', 'node2')
        >>> graph.edge_attribute("node1", "node2")
        'edge_data'
        >>> graph.edge_attribute("node1", "node2", "new_data")
        'new_data'
        >>> graph.edge_attribute("node1", "node2")
        'new_data'
        """
        if not self.has_edge((u, v)):
            raise KeyError(f"Edge {(u, v)} not in graph")

        if u in self.edges and v in self.edges[u]:
            edge_obj = self.edges[u][v]
        else:
            raise KeyError(f"Edge {(u, v)} not in graph")

        if value is not None:
            edge_obj.attribute = str(value)
            # Update both directions
            if v in self.edges and u in self.edges[v]:
                self.edges[v][u].attribute = str(value)
            return str(value)
        else:
            return edge_obj.attribute
