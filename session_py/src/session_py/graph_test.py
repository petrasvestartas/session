from .graph import Graph
from .vertex import Vertex


def test_graph_constructor():
    graph = Graph("my_graph")
    assert graph.name == "my_graph"
    assert graph.guid is not None


def test_graph_add_node():
    graph = Graph()
    result = graph.add_node("node1", "attribute_data")
    assert result == "node1"
    assert graph.has_node("node1")


def test_graph_add_edge():
    graph = Graph()
    result = graph.add_edge("node1", "node2", "edge_data")
    assert result == ("node1", "node2")
    assert graph.has_edge(("node1", "node2"))


def test_graph_has_node():
    graph = Graph()
    graph.add_node("node1")
    assert graph.has_node("node1")
    assert not graph.has_node("node2")


def test_graph_has_edge():
    graph = Graph()
    graph.add_edge("A", "B", "edge_attr")
    assert graph.has_edge(("A", "B"))
    assert not graph.has_edge(("C", "D"))


def test_graph_remove_node():
    graph = Graph()
    graph.add_node("node1")
    graph.remove_node("node1")
    assert not graph.has_node("node1")


def test_graph_remove_edge():
    graph = Graph()
    graph.add_edge("A", "B", "edge_attr")
    graph.remove_edge(("A", "B"))
    assert not graph.has_edge(("A", "B"))


def test_graph_get_vertices():
    graph = Graph()
    graph.add_node("node1", "node_data")
    vertices = graph.get_vertices()
    assert len(vertices) == 1
    assert isinstance(vertices[0], Vertex)
    assert vertices[0].name == "node1"


def test_graph_edges():
    graph = Graph()
    graph.add_edge("node1", "node2", "edge_data")
    edges = list(graph.get_edges())
    assert ("node1", "node2") in edges
    assert len(edges) == 1


def test_graph_neighbors():
    graph = Graph()
    graph.add_edge("A", "B", "edge1")
    graph.add_edge("A", "C", "edge2")
    neighbors = sorted(list(graph.neighbors("A")))
    assert neighbors == ["B", "C"]


def test_graph_number_of_vertices():
    graph = Graph()
    graph.add_node("node1")
    assert graph.number_of_vertices() == 1


def test_graph_number_of_edges():
    graph = Graph()
    graph.add_edge("node1", "node2")
    assert graph.number_of_edges() == 1


def test_graph_clear():
    graph = Graph()
    graph.add_node("node1")
    graph.clear()
    assert graph.number_of_vertices() == 0


def test_graph_node_attribute():
    graph = Graph()
    graph.add_node("node1", "initial_data")
    assert graph.node_attribute("node1") == "initial_data"
    graph.node_attribute("node1", "new_data")
    assert graph.node_attribute("node1") == "new_data"


def test_graph_edge_attribute():
    graph = Graph("test_graph")
    graph.add_edge("node1", "node2", "edge_data")
    assert graph.edge_attribute("node1", "node2") == "edge_data"
    graph.edge_attribute("node1", "node2", "new_data")
    assert graph.edge_attribute("node1", "node2") == "new_data"


def test_graph_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    graph = Graph("my_graph")
    graph.add_node("A", "vertex_A")
    graph.add_node("B", "vertex_B")
    graph.add_node("C", "vertex_C")
    graph.add_edge("A", "B", "edge_AB")
    graph.add_edge("B", "C", "edge_BC")

    path = Path(__file__).resolve().parents[2] / "test_graph.json"
    json_dump(graph, path)
    loaded = json_load(path)

    assert loaded.name == graph.name
    assert loaded.number_of_vertices() == graph.number_of_vertices()
    assert loaded.number_of_edges() == graph.number_of_edges()
