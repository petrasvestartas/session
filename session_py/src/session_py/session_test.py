import os
from .session import Session
from .point import Point


def test_session_constructor():
    session = Session()
    assert session.name == "my_session"
    assert session.guid is not None
    assert session.objects is not None
    assert session.tree is not None
    assert session.graph is not None


def test_session_to_json_data():
    session = Session()
    point1 = Point(1.0, 2.0, 3.0)
    point2 = Point(4.0, 5.0, 6.0)
    session.add_point(point1)
    session.add_point(point2)
    session.add_edge(point1.guid, point2.guid, "connection")
    data = session.to_json_data()
    assert data["name"] == "my_session"
    assert "guid" in data
    assert len(data["objects"]["points"]) == 2
    assert len(data["graph"]["vertices"]) == 2
    assert len(data["graph"]["edges"]) == 1


def test_session_from_json_data():
    session = Session()
    point1 = Point(1.0, 2.0, 3.0)
    point2 = Point(4.0, 5.0, 6.0)
    session.add_point(point1)
    session.add_point(point2)
    session.add_edge(point1.guid, point2.guid, "connection")
    data = session.to_json_data()
    session2 = Session.from_json_data(data)
    assert session2.name == "my_session"
    assert len(session2.lookup) == 2
    assert len(session2.graph.get_vertices()) == 2


def test_session_to_json_from_json():
    session = Session()
    point1 = Point(1.0, 2.0, 3.0)
    point2 = Point(4.0, 5.0, 6.0)
    session.add_point(point1)
    session.add_point(point2)
    session.add_edge(point1.guid, point2.guid, "connection")
    filename = "test_session.json"

    session.to_json(filename)
    loaded_session = Session.from_json(filename)

    assert loaded_session.name == session.name
    assert len(loaded_session.lookup) == len(session.lookup)
    assert len(loaded_session.graph.get_vertices()) == len(session.graph.get_vertices())

    os.unlink(filename)


def test_session_add_point():
    session = Session()
    point = Point(1.0, 2.0, 3.0)
    session.add_point(point)

    assert len(session.objects.points) == 1
    assert point.guid in session.lookup
    assert session.graph.has_node(point.guid)


def test_session_add_edge():
    session = Session()
    point1 = Point(1.0, 2.0, 3.0)
    point2 = Point(4.0, 5.0, 6.0)
    session.add_point(point1)
    session.add_point(point2)
    session.add_edge(point1.guid, point2.guid, "connection")

    assert session.graph.has_edge((point1.guid, point2.guid))


def test_session_get_object():
    session = Session()
    point = Point(1.0, 2.0, 3.0)
    session.add_point(point)

    retrieved = session.get_object(point.guid)
    assert retrieved == point


def test_session_to_json_file():
    session = Session("test_session")
    point1 = Point(1.0, 2.0, 3.0)
    point2 = Point(4.0, 5.0, 6.0)
    session.add_point(point1)
    session.add_point(point2)
    session.add_edge(point1.guid, point2.guid, "test_connection")
    filename = "test_session.json"

    session.to_json(filename)
    loaded_session = Session.from_json(filename)

    assert loaded_session.name == session.name
    assert len(loaded_session.objects.points) == len(session.objects.points)
    assert (
        loaded_session.graph.number_of_vertices() == session.graph.number_of_vertices()
    )
    assert loaded_session.graph.number_of_edges() == session.graph.number_of_edges()
