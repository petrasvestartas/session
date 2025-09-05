import json
import uuid


class Vertex:
    """A graph vertex with a unique identifier and attribute string."""
    
    def __init__(self, name, attribute="", index=None):
        """Initialize a new Vertex.
        
        Parameters
        ----------
        name : str
            Name identifier for the vertex.
        attribute : str, optional
            Vertex attribute data as string.
        index : int, optional
            Integer index for the vertex.
        """
        self.name = str(name)
        self.attribute = str(attribute)
        self.index = index
    
    def to_json_data(self):
        """Convert the Vertex to a JSON-serializable dictionary."""
        return {
            "name": self.name,
            "attribute": self.attribute,
            "index": self.index
        }
    
    @classmethod
    def from_json_data(cls, data):
        """Create Vertex from JSON data dictionary."""
        return cls(data["name"], data["attribute"], data.get("index"))


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
        self.v0 = str(v0)
        self.v1 = str(v1)
        self.attribute = str(attribute)
    
    @property
    def vertices(self):
        """Get the edge vertices as a tuple."""
        return (self.v0, self.v1)
    
    def to_json_data(self):
        """Convert the Edge to a JSON-serializable dictionary."""
        return {
            "v0": self.v0,
            "v1": self.v1,
            "attribute": self.attribute
        }
    
    @classmethod
    def from_json_data(cls, data):
        """Create Edge from JSON data dictionary."""
        return cls(data["v0"], data["v1"], data["attribute"])
    
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


class Graph:
    """A graph data structure with string-only vertices and attributes.
    
    This implementation enforces that all node keys and attributes must be strings,
    providing strong typing and cross-language compatibility.
    
    Parameters
    ----------
    name : str, optional
        Name of the graph.
    default_node_attributes : dict, optional
        Default attributes for new vertices.
    default_edge_attributes : dict, optional
        Default attributes for new edges.
        
    Examples
    --------
    >>> graph = Graph("my_graph")
    >>> graph.add_node("node1", "attribute_data")
    'node1'
    >>> graph.add_edge("node1", "node2", "edge_data")
    ('node1', 'node2')
    >>> graph.has_node("node1")
    True
    >>> graph.number_of_vertices()
    2
    """
    
    def __init__(self, name="my_graph"):
        """Initialize a new Graph."""
        self.name = name
        self.guid = str(uuid.uuid4())
        self._vertices = {}  # node_name -> Vertex object
        self._edges = {}  # node_name -> {neighbor_name -> Edge object}
        self.count = 0  # Track next available vertex index

    ###########################################################################################
    # JSON Serialization
    ###########################################################################################

    @classmethod
    def from_json_data(cls, data):
        """Create a Graph from JSON data dictionary.
        
        Parameters
        ----------
        data : dict
            Dictionary containing graph data.
            
        Returns
        -------
        :class:`Graph`
            Graph instance created from the data.
        """
        graph = cls(name=data["name"])
        graph.guid = str(data["guid"])
        graph.count = data.get("count", 0)
        
        # Restore vertices
        for vertex_data in data.get("vertices", []):
            vertex = Vertex.from_json_data(vertex_data)
            graph._vertices[vertex.name] = vertex
            
        # Restore edges
        for edge_data in data.get("edges", []):
            edge = Edge.from_json_data(edge_data)
            u, v = edge.v0, edge.v1
            if u not in graph._edges:
                graph._edges[u] = {}
            if v not in graph._edges:
                graph._edges[v] = {}
            graph._edges[u][v] = edge
            graph._edges[v][u] = edge
                
        return graph

    def to_json_data(self):
        """Convert the Graph to a JSON-serializable dictionary.
        
        Returns
        -------
        dict
            Dictionary representation of the graph.
        """
        return {
            "type": "Graph",
            "name": self.name,
            "guid": self.guid,
            "vertices": [vertex.to_json_data() for vertex in self._vertices.values()],
            "edges": [edge.to_json_data() for u, neighbors in self._edges.items() for v, edge in neighbors.items() if u < v],  # Only store each edge once
            "count": self.count
        }

    @classmethod
    def from_json(cls, filepath):
        """Load a Graph from a JSON file.
        
        Parameters
        ----------
        filepath : str
            Path to the JSON file to load.

        Returns
        -------
        :class:`Graph`
            Graph instance loaded from the file.

        Examples
        --------
        >>> # Test the underlying from_json_data method
        >>> data = {"type": "Graph", "name": "test", "guid": "123", "vertices": [], "edges": []}
        >>> graph = Graph.from_json_data(data)
        >>> graph.name
        'test'
        """
        with open(filepath, "r") as f:
            data = json.load(f)
            return cls.from_json_data(data)


    def to_json(self, filepath):
        """Save the Graph to a JSON file.
        
        Parameters
        ----------
        filepath : str
            Path where to save the JSON file.
            
        Examples
        --------
        >>> from .point import Point
        >>> graph = Graph()
        >>> point1 = Point(10.0, 20.0, 30.0)
        >>> point2 = Point(40.0, 50.0, 60.0)
        >>> point3 = Point(70.0, 80.0, 90.0)
        >>> _ = graph.add_node(point1.guid, "corner_point")
        >>> _ = graph.add_node(point2.guid, "center_point")
        >>> _ = graph.add_node(point3.guid, "edge_point")
        >>> _ = graph.add_edge(point1.guid, point2.guid, "distance:50m")
        >>> _ = graph.add_edge(point2.guid, point3.guid, "distance:75m")
        >>> graph.to_json("my_graph.json")
        """
        with open(filepath, "w") as f:
            json.dump(self.to_json_data(), f, indent=2)


    ###########################################################################################
    # Essential Graph Methods
    ###########################################################################################

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
            return self._vertices[key]
        else:
            vertex = Vertex(key, attribute, self.count)
            self._vertices[key] = vertex
            self.count += 1
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
        if u not in self._edges:
            self._edges[u] = {}
        if v not in self._edges:
            self._edges[v] = {}
        self._edges[u][v] = edge
        self._edges[v][u] = edge
        
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
        if key in self._edges:
            for neighbor in list(self._edges[key].keys()):
                if neighbor in self._edges:
                    self._edges[neighbor].pop(key, None)
            del self._edges[key]
            
        # Remove the node itself
        del self._vertices[key]
        
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
            if u in self._edges and v in self._edges[u]:
                del self._edges[u][v]
            if v in self._edges and u in self._edges[v]:
                del self._edges[v][u]

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
        return key in self._vertices

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
        
        return u in self._edges and v in self._edges[u]

    def vertices(self, data=False):
        """Iterate over all vertices in the graph.
        
        Parameters
        ----------
        data : bool, optional
            If True, yield node attributes along with identifiers.
            
        Yields
        ------
        hashable or tuple
            Node identifier or (node, attributes) if data=True.
            
        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1", "node_data")
        'node1'
        >>> vertices = list(graph.vertices())
        >>> assert "node1" in vertices
        >>> assert len(vertices) == 1
        """
        if data:
            for node in self._vertices.values():
                yield node.name, node.attribute
        else:
            for node_id in self._vertices:
                yield node_id

    def edges(self, data=False):
        """Iterate over all edges in the graph.
        
        Parameters
        ----------
        data : bool, optional
            If True, yield edge attributes along with identifiers.
            
        Yields
        ------
        tuple
            Edge identifier (u, v) or ((u, v), attributes) if data=True.
            
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
        for u, neighbors in self._edges.items():
            for v, edge in neighbors.items():
                edge_tuple = (u, v) if u < v else (v, u)
                if edge_tuple not in seen:
                    seen.add(edge_tuple)
                    if data:
                        yield edge_tuple, edge.attribute
                    else:
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
        return iter(self._edges.get(node, {}).keys())

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
        return len(self._vertices)

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
        return sum(len(neighbors) for neighbors in self._edges.values()) // 2

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
        self._vertices.clear()
        self._edges.clear()
        self.count = 0
    
    def _reassign_indices(self):
        """Reassign vertex indices to maintain contiguous sequence 0, 1, 2, ..."""
        vertices = list(self._vertices.values())
        # Sort by current index to maintain relative order
        vertices.sort(key=lambda v: v.index if v.index is not None else float('inf'))
        
        for i, vertex in enumerate(vertices):
            vertex.index = i
        
        self.count = len(vertices)

    ###########################################################################################
    # Attribute Methods
    ###########################################################################################

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
        str or None
            The attribute value as string if getting, None if setting.
            
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
            
        node_obj = self._vertices[node]
        if value is not None:
            node_obj.attribute = str(value)
        else:
            return node_obj.attribute

    def edge_attribute(self, edge, value=None):
        """Get or set edge attribute.
        
        Parameters
        ----------
        edge : tuple
            The edge identifier as (u, v).
        value : str, optional
            If provided, set the attribute to this value.
            
        Returns
        -------
        str or None
            The attribute value as string if getting, None if setting.

        Raises
        ------
        KeyError
            If the edge does not exist.

        Examples
        --------
        >>> graph = Graph("test_graph")
        >>> graph.add_edge("node1", "node2", "edge_data")
        ('node1', 'node2')
        >>> assert graph.edge_attribute(("node1", "node2")) == "edge_data"
        >>> graph.edge_attribute(("node1", "node2"), "new_data")
        >>> assert graph.edge_attribute(("node1", "node2")) == "new_data"
        """
        u, v = edge
        if not self.has_edge(edge):
            raise KeyError(f"Edge {edge} not in graph")
        
        if u in self._edges and v in self._edges[u]:
            edge_obj = self._edges[u][v]
        else:
            raise KeyError(f"Edge {edge} not in graph")
        
        if value is not None:
            edge_obj.attribute = str(value)
            # Update both directions
            if v in self._edges and u in self._edges[v]:
                self._edges[v][u].attribute = str(value)
        else:
            return edge_obj.attribute

    ###########################################################################################
    # Filtering Methods
    ###########################################################################################

    def vertices_where(self, conditions=None, data=False, **kwargs):
        """Filter vertices by attribute conditions.
        
        Parameters
        ----------
        conditions : dict, optional
            Dictionary of attribute conditions to match.
        data : bool, optional
            If True, yield node attributes along with identifiers.
        **kwargs
            Additional conditions as keyword arguments.
            
        Yields
        ------
        hashable or tuple
            Node identifier or (node, attributes) if data=True.
            
        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1", "x:1.0")
        'node1'
        >>> graph.add_node("node2", "x:2.0")
        'node2'
        >>> vertices = list(graph.vertices_where(data=True))
        >>> len(vertices) == 2
        True
        """
        conditions = conditions or {}
        conditions.update(kwargs)

        for node in self._vertices.values():
            is_match = True
            attr = node.attribute or {}
            
            for name, value in conditions.items():
                if name not in attr:
                    is_match = False
                    break
                if attr[name] != value:
                    is_match = False
                    break
                    
            if is_match:
                if data:
                    yield node.name, attr
                else:
                    yield node.name

    def edges_where(self, conditions=None, data=False, **kwargs):
        """Filter edges by attribute conditions.
        
        Parameters
        ----------
        conditions : dict, optional
            Dictionary of attribute conditions to match.
        data : bool, optional
            If True, yield edge attributes along with identifiers.
        **kwargs
            Additional conditions as keyword arguments.
            
        Yields
        ------
        tuple
            Edge identifier (u, v) or ((u, v), attributes) if data=True.
            
        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_edge("node1", "node2", "weight:5.0,color:red")
        ('node1', 'node2')
        >>> graph.add_edge("node1", "node3", "weight:3.0,color:blue")
        ('node1', 'node3')
        >>> edges = list(graph.edges_where(data=True))
        >>> len(edges) == 2
        True
        """
        conditions = conditions or {}
        conditions.update(kwargs)

        seen = set()
        for u, neighbors in self._edges.items():
            for v, edge in neighbors.items():
                edge_tuple = (u, v) if u < v else (v, u)
                if edge_tuple in seen:
                    continue
                seen.add(edge_tuple)
                
                is_match = True
                attr = edge.attribute or {}

                for name, value in conditions.items():
                    if name not in attr:
                        is_match = False
                        break
                    if attr[name] != value:
                        is_match = False
                        break

                if is_match:
                    if data:
                        yield edge_tuple, attr
                    else:
                        yield edge_tuple
