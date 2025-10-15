"""Tests for the encoders module."""

from pathlib import Path
from .encoders import (
    json_dump,
    json_dumps,
    json_load,
    json_loads,
    decode_node,
    GeometryEncoder,
    GeometryDecoder,
)
from .point import Point
from .vector import Vector
from .line import Line


def test_json_dump_and_load():
    """Test json_dump and json_load with file I/O."""
    point = Point(1.5, 2.5, 3.5)
    point.name = "test_point"

    filepath = Path(__file__).resolve().parents[2] / "test_encoders_point.json"
    json_dump(point, filepath)

    loaded = json_load(filepath)

    assert isinstance(loaded, Point)
    assert loaded.x == point.x
    assert loaded.y == point.y
    assert loaded.z == point.z
    assert loaded.name == point.name

    filepath.unlink()


def test_json_dumps_and_loads():
    """Test json_dumps and json_loads with string serialization."""
    vec = Vector(42.1, 84.2, 126.3)
    vec.name = "test_vector"

    json_str = json_dumps(vec)
    assert isinstance(json_str, str)
    assert "Vector" in json_str

    loaded = json_loads(json_str)

    assert isinstance(loaded, Vector)
    assert loaded.x == vec.x
    assert loaded.y == vec.y
    assert loaded.z == vec.z
    assert loaded.name == vec.name


def test_encode_collection():
    """Test encoding a collection of geometry objects."""
    points = [
        Point(1.0, 2.0, 3.0),
        Point(4.0, 5.0, 6.0),
        Point(7.0, 8.0, 9.0),
    ]

    json_str = json_dumps(points)
    assert isinstance(json_str, str)

    loaded = json_loads(json_str)
    assert isinstance(loaded, list)
    assert len(loaded) == 3
    assert all(isinstance(p, Point) for p in loaded)
    assert loaded[0].x == 1.0
    assert loaded[1].y == 5.0
    assert loaded[2].z == 9.0


def test_decode_node_primitives():
    """Test decode_node with primitive types."""
    assert decode_node(None) is None
    assert decode_node(42) == 42
    assert decode_node(3.14) == 3.14
    assert decode_node("hello") == "hello"
    assert decode_node(True) is True


def test_decode_node_list():
    """Test decode_node with lists."""
    data = [1, 2, 3]
    result = decode_node(data)
    assert result == [1, 2, 3]

    # List with geometry objects
    point_data = {
        "type": "Point",
        "x": 1.0,
        "y": 2.0,
        "z": 3.0,
        "guid": "test",
        "name": "p1",
    }
    data = [
        point_data,
        {"type": "Point", "x": 4.0, "y": 5.0, "z": 6.0, "guid": "test2", "name": "p2"},
    ]
    result = decode_node(data)
    assert len(result) == 2
    assert isinstance(result[0], Point)
    assert isinstance(result[1], Point)


def test_decode_node_dict():
    """Test decode_node with dictionaries."""
    # Plain dict
    data = {"a": 1, "b": 2}
    result = decode_node(data)
    assert result == {"a": 1, "b": 2}

    # Dict with type field (geometry object)
    data = {
        "type": "Vector",
        "x": 1.0,
        "y": 2.0,
        "z": 3.0,
        "guid": "test",
        "name": "v1",
    }
    result = decode_node(data)
    assert isinstance(result, Vector)
    assert result.x == 1.0


def test_nested_collections():
    """Test encoding and decoding nested collections."""
    lines = [
        Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
        Line(0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
    ]

    # Encode to JSON string
    json_str = json_dumps(lines)

    # Decode back
    loaded = json_loads(json_str)

    assert isinstance(loaded, list)
    assert len(loaded) == 2
    assert all(isinstance(line, Line) for line in loaded)


def test_roundtrip_with_file():
    """Test complete roundtrip with file I/O for collections."""
    vectors = [
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
    ]

    filepath = Path(__file__).resolve().parents[2] / "test_encoders_collection.json"
    json_dump(vectors, filepath)

    loaded = json_load(filepath)

    assert isinstance(loaded, list)
    assert len(loaded) == 3
    assert all(isinstance(v, Vector) for v in loaded)
    assert loaded[0].x == 1.0
    assert loaded[1].y == 1.0
    assert loaded[2].z == 1.0

    filepath.unlink()


def test_geometry_encoder():
    """Test GeometryEncoder directly."""
    import json

    point = Point(1.0, 2.0, 3.0)
    json_str = json.dumps(point, cls=GeometryEncoder)

    assert isinstance(json_str, str)
    assert "Point" in json_str


def test_geometry_decoder():
    """Test GeometryDecoder directly."""
    import json

    json_str = '{"type": "Point", "x": 1.0, "y": 2.0, "z": 3.0, "guid": "test", "name": "p1", "width": 1.0}'
    loaded = json.loads(json_str, cls=GeometryDecoder)

    assert isinstance(loaded, Point)
    assert loaded.x == 1.0


def test_pretty_vs_compact():
    """Test pretty vs compact JSON formatting."""
    point = Point(1.0, 2.0, 3.0)

    pretty = json_dumps(point, pretty=True)
    compact = json_dumps(point, pretty=False)

    assert len(pretty) > len(compact)
    assert "\n" in pretty
    assert "\n" not in compact

    # Both should deserialize correctly
    loaded_pretty = json_loads(pretty)
    loaded_compact = json_loads(compact)

    assert isinstance(loaded_pretty, Point)
    assert isinstance(loaded_compact, Point)
    assert loaded_pretty.x == 1.0
    assert loaded_compact.x == 1.0


def test_list_in_list_in_list():
    """Test nested lists (list in list in list)."""
    data = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
    json_str = json_dumps(data)
    loaded = json_loads(json_str)
    assert loaded == data
    assert loaded[0][0][0] == 1
    assert loaded[1][1][1] == 8


def test_dict_of_lists():
    """Test dictionary containing lists."""
    data = {
        "numbers": [1, 2, 3],
        "letters": ["a", "b", "c"],
        "points": [Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)],
    }
    json_str = json_dumps(data)
    loaded = json_loads(json_str)
    assert loaded["numbers"] == [1, 2, 3]
    assert loaded["letters"] == ["a", "b", "c"]
    assert len(loaded["points"]) == 2
    assert isinstance(loaded["points"][0], Point)
    assert loaded["points"][0].x == 1.0


def test_list_of_dict():
    """Test list containing dictionaries."""
    data = [
        {"name": "point1", "value": 10},
        {"name": "point2", "value": 20},
        {"geometry": Point(1.0, 2.0, 3.0)},
    ]
    json_str = json_dumps(data)
    loaded = json_loads(json_str)
    assert len(loaded) == 3
    assert loaded[0]["name"] == "point1"
    assert loaded[1]["value"] == 20
    assert isinstance(loaded[2]["geometry"], Point)
    assert loaded[2]["geometry"].z == 3.0


def test_dict_of_dicts():
    """Test nested dictionaries (dict of dicts)."""
    data = {
        "config": {"tolerance": 0.001, "scale": 1000},
        "geometry": {"point": Point(1.0, 2.0, 3.0), "vector": Vector(0.0, 0.0, 1.0)},
    }
    json_str = json_dumps(data)
    loaded = json_loads(json_str)
    assert loaded["config"]["tolerance"] == 0.001
    assert loaded["config"]["scale"] == 1000
    assert isinstance(loaded["geometry"]["point"], Point)
    assert isinstance(loaded["geometry"]["vector"], Vector)
    assert loaded["geometry"]["point"].x == 1.0
    assert loaded["geometry"]["vector"].z == 1.0
