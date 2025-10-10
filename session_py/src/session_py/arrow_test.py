import pytest
from session_py import Arrow, Line


def test_arrow_creation():
    """Test basic arrow creation."""
    line = Line(0.0, 0.0, 0.0, 0.0, 0.0, 10.0)
    arrow = Arrow(line, 1.0)

    assert arrow.radius == 1.0
    assert arrow.mesh.number_of_vertices() == 29
    assert arrow.mesh.number_of_faces() == 28
    assert arrow.name == "my_arrow"
    assert arrow.guid is not None


def test_arrow_json_serialization():
    """Test arrow JSON serialization."""
    line = Line(0.0, 0.0, 0.0, 5.0, 0.0, 0.0)
    arrow = Arrow(line, 2.0)

    data = arrow.to_json_data()
    assert data["type"] == "Arrow"
    assert data["radius"] == 2.0
    assert "mesh" in data
    assert "line" in data


def test_arrow_json_round_trip():
    """Test arrow JSON file I/O."""
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    arrow = Arrow(line, 0.5)

    filepath = "test_arrow.json"
    arrow.to_json(filepath)

    loaded = Arrow.from_json(filepath)
    assert loaded.radius == 0.5
    assert loaded.mesh.number_of_vertices() == 29
    assert loaded.mesh.number_of_faces() == 28


def test_arrow_mesh_colors():
    """Test that arrow mesh has color collections."""
    line = Line(0.0, 0.0, 0.0, 0.0, 0.0, 10.0)
    arrow = Arrow(line, 1.0)

    assert len(arrow.mesh.pointcolors) == 29
    assert len(arrow.mesh.facecolors) == 28
    assert len(arrow.mesh.linecolors) == 56
    assert len(arrow.mesh.widths) == 56
