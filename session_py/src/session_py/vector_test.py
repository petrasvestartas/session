import os
from .vector import Vector


def test_vector_constructor():
    """Test Vector constructor."""
    vector = Vector(1.0, 2.0, 3.0)
    assert vector.guid != ""
    assert vector.x == 1.0
    assert vector.y == 2.0
    assert vector.z == 3.0


def test_vector_equality():
    """Test Vector equality."""
    v1 = Vector(1.0, 2.0, 3.0)
    v2 = Vector(1.0, 2.0, 3.0)
    assert v1 == v2
    assert not (v1 != v2)

    v3 = Vector(1.0, 2.0, 3.0)
    v4 = Vector(1.1, 2.0, 3.0)
    assert not (v3 == v4)
    assert v3 != v4


def test_vector_to_json_data():
    """Test Vector to_json_data method."""
    vector = Vector(10.5, 20.7, 30.9)
    vector.name = "force_vector_X"
    data = vector.to_json_data()
    assert data["type"] == "Vector"
    assert data["name"] == "force_vector_X"
    assert data["x"] == 10.5
    assert data["y"] == 20.7
    assert data["z"] == 30.9
    assert "guid" in data


def test_vector_from_json_data():
    """Test Vector from_json_data method."""
    original_vector = Vector(45.1, 67.8, 89.2)
    data = original_vector.to_json_data()
    restored_vector = Vector.from_json_data(data)
    assert restored_vector.x == 45.1
    assert restored_vector.y == 67.8
    assert restored_vector.z == 89.2
    assert restored_vector.guid == original_vector.guid


def test_vector_to_json_from_json():
    """Test Vector file I/O with to_json and from_json."""
    original = Vector(100.25, 200.50, 300.75)
    filename = "test_vector.json"

    original.to_json(filename)
    loaded = Vector.from_json(filename)

    assert loaded.x == original.x
    assert loaded.y == original.y
    assert loaded.z == original.z
    assert loaded.name == original.name
    assert loaded.guid == original.guid
