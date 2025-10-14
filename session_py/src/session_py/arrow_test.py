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


def test_arrow_mesh_colors():
    """Test that arrow mesh has color collections."""
    line = Line(0.0, 0.0, 0.0, 0.0, 0.0, 10.0)
    arrow = Arrow(line, 1.0)

    assert len(arrow.mesh.pointcolors) == 29
    assert len(arrow.mesh.facecolors) == 28
    assert len(arrow.mesh.linecolors) == 56
    assert len(arrow.mesh.widths) == 56


def test_arrow_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    line = Line(0.0, 0.0, 0.0, 0.0, 0.0, 10.0)
    arrow = Arrow(line, 1.0)
    arrow.name = "test_arrow"

    path = Path(__file__).resolve().parents[2] / "test_arrow.json"
    json_dump(arrow, path)
    loaded = json_load(path)

    assert isinstance(loaded, Arrow)
    assert loaded.radius == arrow.radius
    assert loaded.line.z1 == arrow.line.z1
    assert loaded.name == arrow.name
