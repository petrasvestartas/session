import json
import uuid
from collections import defaultdict


class Graph:
    """A graph data structure with string-only nodes and attributes.
    
    This implementation enforces that all node keys and attributes must be strings,
    providing strong typing and cross-language compatibility.
    
    Parameters
    ----------
    name : str, optional
        Name of the graph.
    default_node_attributes : dict, optional
        Default attributes for new nodes.
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
    >>> graph.number_of_nodes()
    2
    """
    
    def __init__(self, name="my_graph"):
        """Initialize a new Graph."""
        self.name = name
        self.guid = str(uuid.uuid4())
        self.node = {}  # node_key -> attribute_string
        self.adjacency = {}  # node -> {neighbor_set}
        self.edge = defaultdict(dict)  # node -> {neighbor -> attribute_string}
        self._max_node = -1

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
            
        Examples
        --------
        >>> graph = Graph()
        >>> data = graph.to_json_data()
        >>> graph2 = Graph.from_json_data(data)
        >>> assert graph2.name == "my_graph"
        """
        graph = cls(name=data["name"])
        graph.guid = str(data["guid"])
        graph._max_node = data.get("_max_node", -1)
        
        # Restore nodes
        for key_str, attr in data["node"].items():
            key = int(key_str) if key_str.isdigit() else key_str
            graph.node[key] = attr
            
        # Restore adjacency and edges
        for u_str, nbrs in data["adjacency"].items():
            u = int(u_str) if u_str.isdigit() else u_str
            graph.adjacency[u] = set()
            for v_str in nbrs:
                v = int(v_str) if v_str.isdigit() else v_str
                graph.adjacency[u].add(v)
                
        for u_str, nbrs in data["edge"].items():
            u = int(u_str) if u_str.isdigit() else u_str
            graph.edge[u] = {}
            for v_str, attr in nbrs.items():
                v = int(v_str) if v_str.isdigit() else v_str
                graph.edge[u][v] = attr
                
        return graph

    def to_json_data(self):
        """Convert the Graph to a JSON-serializable dictionary.
        
        Returns
        -------
        dict
            Dictionary representation of the graph.
            
        Examples
        --------
        >>> from .point import Point
        >>> graph = Graph("object_relationships")
        >>> point1 = Point(1.0, 2.0, 3.0)
        >>> point2 = Point(4.0, 5.0, 6.0)
        >>> point3 = Point(7.0, 8.0, 9.0)
        >>> _ = graph.add_node(point1.guid, "start_point")
        >>> _ = graph.add_node(point2.guid, "middle_point")
        >>> _ = graph.add_node(point3.guid, "end_point")
        >>> _ = graph.add_edge(point1.guid, point2.guid, "connects_to")
        >>> _ = graph.add_edge(point2.guid, point3.guid, "leads_to")
        >>> data = graph.to_json_data()
        >>> assert data["name"] == "object_relationships"
        >>> assert "guid" in data
        >>> assert len(data["node"]) == 3
        >>> assert len(data["adjacency"]) == 3
        >>> assert point1.guid in data["node"]
        >>> assert point2.guid in data["adjacency"][point1.guid]
        >>> assert data["edge"][point1.guid][point2.guid] == "connects_to"
        """
        return {
            "type": "Graph",
            "name": self.name,
            "guid": self.guid,
            "node": dict(self.node),
            "edge": {u: dict(nbrs) for u, nbrs in self.edge.items()},
            "adjacency": {u: list(nbrs) for u, nbrs in self.adjacency.items()},
            "_max_node": self._max_node,
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
        >>> data = {"type": "Graph", "name": "test", "guid": "123", "node": {}, "edge": {}, "adjacency": {}, "default_node_attributes": {}, "default_edge_attributes": {}, "_max_node": -1}
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
        >>> graph = Graph("spatial_relationships")
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
        
        self.node[key] = str(attribute)
        if key not in self.adjacency:
            self.adjacency[key] = set()
        return key

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
        
        # Add nodes if they don't exist
        if not self.has_node(u):
            self.add_node(u)
        if not self.has_node(v):
            self.add_node(v)
            
        # Add edge (undirected, so add both directions)
        edge_attr = str(attribute)
        if u not in self.adjacency:
            self.adjacency[u] = set()
        if v not in self.adjacency:
            self.adjacency[v] = set()
        self.adjacency[u].add(v)
        self.adjacency[v].add(u)
        self.edge[u][v] = edge_attr
        self.edge[v][u] = edge_attr
        
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
            If the node does not exist.
            
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
        for neighbor in list(self.adjacency[key]):
            self.remove_edge((key, neighbor))
            
        # Remove the node itself
        del self.node[key]
        del self.adjacency[key]
        if key in self.edge:
            del self.edge[key]

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
            self.adjacency[u].discard(v)
            self.adjacency[v].discard(u)
            del self.edge[u][v]
            del self.edge[v][u]

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
        return key in self.node

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
        
        return u in self.adjacency and v in self.adjacency[u]

    def nodes(self, data=False):
        """Iterate over all nodes in the graph.
        
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
        >>> nodes = list(graph.nodes())
        >>> assert "node1" in nodes
        >>> assert len(nodes) == 1
        """
        if data:
            for key, attr in self.node.items():
                yield key, attr
        else:
            for key in self.node:
                yield key

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
        for u, nbrs in self.edge.items():
            for v, attr in nbrs.items():
                edge = (u, v) if u < v else (v, u)
                if edge not in seen:
                    seen.add(edge)
                    if data:
                        yield edge, attr
                    else:
                        yield edge

    def neighbors(self, node):
        """Get all neighbors of a node.
        
        Parameters
        ----------
        node : str
            The node to get neighbors for.
            
        Returns
        -------
        iterator
            Iterator over neighbor nodes.
            
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
        return iter(self.adjacency.get(node, set()))

    def number_of_nodes(self):
        """Get the number of nodes in the graph.
        
        Returns
        -------
        int
            Number of nodes.
            
        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1")
        'node1'
        >>> graph.number_of_nodes()
        1
        """
        return len(self.node)

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
        return sum(len(nbrs) for nbrs in self.adjacency.values()) // 2

    def clear(self):
        """Remove all nodes and edges from the graph.
        
        Examples
        --------
        >>> graph = Graph()
        >>> graph.add_node("node1")
        'node1'
        >>> graph.clear()
        >>> graph.number_of_nodes()
        0
        """
        self.node.clear()
        self.adjacency.clear()
        self.edge.clear()

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
        
        if value is not None:
            self.node[node] = str(value)
        else:
            return self.node[node]

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
        
        if value is not None:
            # Update both directions for undirected graph
            str_value = str(value)
            self.edge[u][v] = str_value
            self.edge[v][u] = str_value
        else:
            return self.edge[u][v]

    ###########################################################################################
    # Filtering Methods
    ###########################################################################################

    def nodes_where(self, conditions=None, data=False, **kwargs):
        """Filter nodes by attribute conditions.
        
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
        >>> nodes = list(graph.nodes_where(data=True))
        >>> len(nodes) == 2
        True
        """
        conditions = conditions or {}
        conditions.update(kwargs)

        for key, attr in self.nodes(True):
            is_match = True
            attr = attr or {}

            for name, value in conditions.items():
                if name not in attr:
                    is_match = False
                    break
                if attr[name] != value:
                    is_match = False
                    break

            if is_match:
                if data:
                    yield key, attr
                else:
                    yield key

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

        for key in self.edges():
            is_match = True
            u, v = key
            attr = self.edge[u][v] if u in self.edge and v in self.edge[u] else {}

            for name, value in conditions.items():
                if name not in attr:
                    is_match = False
                    break
                if attr[name] != value:
                    is_match = False
                    break

            if is_match:
                if data:
                    yield key, attr
                else:
                    yield key
